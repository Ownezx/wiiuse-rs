//! Private native boundary. All raw pointers and unsafe calls stay in this module.

use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::os::raw::c_int;
use std::ptr::NonNull;

#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/// Owned copy of the fixed-layout C state snapshot. Contains no native pointers.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RawState {
    pub wiimote_buttons_down: u16,
    pub wiimote_buttons_held: u16,
    pub wiimote_buttons_released: u16,
    pub wiimote_accel_x: u16,
    pub wiimote_accel_y: u16,
    pub wiimote_accel_z: u16,
    pub wiimote_gforce_x: f32,
    pub wiimote_gforce_y: f32,
    pub wiimote_gforce_z: f32,
    pub has_nunchuk: u8,
    pub nunchuk_buttons_down: u8,
    pub nunchuk_buttons_held: u8,
    pub nunchuk_buttons_released: u8,
    pub nunchuk_accel_x: u16,
    pub nunchuk_accel_y: u16,
    pub nunchuk_accel_z: u16,
    pub nunchuk_gforce_x: f32,
    pub nunchuk_gforce_y: f32,
    pub nunchuk_gforce_z: f32,
}

/// Owns the opaque pointer returned by `rust_wiiuse_init`.
pub(crate) struct NativeDevices {
    devices: NonNull<c_void>,
    capacity: c_int,
}

impl NativeDevices {
    pub(crate) fn new(capacity: c_int) -> Option<Self> {
        if capacity <= 0 {
            return None;
        }

        // SAFETY: `capacity` is positive. A non-null return is an allocation
        // created by the shim and ownership is transferred to this value.
        let devices = NonNull::new(unsafe { bindings::rust_wiiuse_init(capacity) })?;
        Some(Self { devices, capacity })
    }

    pub(crate) fn find(&mut self, timeout_seconds: c_int) -> usize {
        // SAFETY: `devices` is the live array owned by this value, and the
        // native maximum is exactly the allocated capacity.
        let found = unsafe {
            bindings::rust_wiiuse_find(self.devices.as_ptr(), self.capacity, timeout_seconds)
        };
        usize::try_from(found)
            .unwrap_or(0)
            .min(self.capacity as usize)
    }

    pub(crate) fn connect(&mut self, found: usize) -> usize {
        let count = found.min(self.capacity as usize);
        if count == 0 {
            return 0;
        }

        // SAFETY: `count` is bounded by the allocated capacity and refers to
        // entries populated by the preceding discovery call.
        let connected =
            unsafe { bindings::rust_wiiuse_connect(self.devices.as_ptr(), count as c_int) };
        usize::try_from(connected).unwrap_or(0).min(count)
    }

    pub(crate) fn poll(&mut self, connected: usize) {
        let count = connected.min(self.capacity as usize);
        if count == 0 {
            return;
        }

        // SAFETY: pointer is live and count cannot exceed the allocated capacity.
        let _ = unsafe { bindings::rust_wiiuse_poll(self.devices.as_ptr(), count as c_int) };
    }

    pub(crate) fn motion_sensing(&mut self, index: usize, connected: usize, enabled: bool) {
        let count = connected.min(self.capacity as usize);
        if index >= count {
            return;
        }

        // SAFETY: index and count are checked against the allocated controller
        // array; the C shim repeats the index/count validation.
        unsafe {
            bindings::rust_wiiuse_motion_sensing(
                self.devices.as_ptr(),
                count as c_int,
                index as c_int,
                if enabled { 1 } else { 0 },
            );
        }
    }

    pub(crate) fn state(&self, index: usize, connected: usize) -> Option<RawState> {
        let count = connected.min(self.capacity as usize);
        if index >= count {
            return None;
        }

        let index = c_int::try_from(index).ok()?;
        let count = c_int::try_from(count).ok()?;
        let mut out = MaybeUninit::<bindings::rust_wiiuse_state_t>::uninit();

        // SAFETY: the device array is live; index is within count; count is
        // bounded by capacity; and out points to writable storage of the exact
        // snapshot type declared by the C shim header.
        let result = unsafe {
            bindings::rust_wiiuse_get_state(self.devices.as_ptr(), index, count, out.as_mut_ptr())
        };
        if result == 0 {
            return None;
        }

        // SAFETY: the C shim zeroes the whole snapshot and fills its fields
        // before returning nonzero.
        let out = unsafe { out.assume_init() };
        Some(RawState {
            wiimote_buttons_down: out.wiimote_buttons_down,
            wiimote_buttons_held: out.wiimote_buttons_held,
            wiimote_buttons_released: out.wiimote_buttons_released,
            wiimote_accel_x: out.wiimote_accel_x,
            wiimote_accel_y: out.wiimote_accel_y,
            wiimote_accel_z: out.wiimote_accel_z,
            wiimote_gforce_x: out.wiimote_gforce_x,
            wiimote_gforce_y: out.wiimote_gforce_y,
            wiimote_gforce_z: out.wiimote_gforce_z,
            has_nunchuk: out.has_nunchuk,
            nunchuk_buttons_down: out.nunchuk_buttons_down,
            nunchuk_buttons_held: out.nunchuk_buttons_held,
            nunchuk_buttons_released: out.nunchuk_buttons_released,
            nunchuk_accel_x: out.nunchuk_accel_x,
            nunchuk_accel_y: out.nunchuk_accel_y,
            nunchuk_accel_z: out.nunchuk_accel_z,
            nunchuk_gforce_x: out.nunchuk_gforce_x,
            nunchuk_gforce_y: out.nunchuk_gforce_y,
            nunchuk_gforce_z: out.nunchuk_gforce_z,
        })
    }
}

impl Drop for NativeDevices {
    fn drop(&mut self) {
        // SAFETY: this value uniquely owns the allocation returned by init;
        // cleanup is called exactly once with the original allocated capacity.
        unsafe { bindings::rust_wiiuse_cleanup(self.devices.as_ptr(), self.capacity) };
    }
}
