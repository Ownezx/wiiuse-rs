//! Safe Rust API for the native `wiiuse` library.
//!
//! Native pointers and FFI calls are private to `ffi`. Applications use the
//! owning [`WiimoteSet`] type and receive ordinary Rust-owned state snapshots.

#![deny(unsafe_code)]

#[allow(unsafe_code)]
mod ffi;
mod state;

use std::time::Duration;

pub use state::{
    Accelerometer, ButtonState, Joystick, NunchukButtons, NunchukState, WiimoteButtons,
    WiimoteState,
};

/// Errors returned when allocating a Wii Remote set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InitError {
    /// At least one controller slot must be allocated.
    ZeroCapacity,
    /// The requested capacity cannot be represented by the native C API.
    CapacityTooLarge,
    /// The native `wiiuse_init` call could not allocate the controller array.
    AllocationFailed,
}

impl std::fmt::Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroCapacity => write!(f, "controller capacity must be greater than zero"),
            Self::CapacityTooLarge => {
                write!(f, "controller capacity exceeds the native C API limit")
            }
            Self::AllocationFailed => write!(f, "wiiuse could not allocate controller state"),
        }
    }
}

impl std::error::Error for InitError {}

/// Owns the native controller array and releases it automatically on drop.
///
/// The type deliberately exposes no native pointers. Operations that mutate
/// controller state require `&mut self`, preventing safe Rust callers from
/// polling, discovering, connecting, or configuring the same set concurrently.
pub struct WiimoteSet {
    native: ffi::NativeDevices,
    capacity: usize,
    found: usize,
    connected: usize,
}

impl std::fmt::Debug for WiimoteSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WiimoteSet")
            .field("capacity", &self.capacity)
            .field("found", &self.found)
            .field("connected", &self.connected)
            .finish()
    }
}

impl WiimoteSet {
    /// Allocates storage for up to `capacity` Wii Remotes.
    ///
    /// The native allocation is released automatically when this value is dropped.
    pub fn new(capacity: usize) -> Result<Self, InitError> {
        if capacity == 0 {
            return Err(InitError::ZeroCapacity);
        }

        let native_capacity = i32::try_from(capacity).map_err(|_| InitError::CapacityTooLarge)?;
        let native = ffi::NativeDevices::new(native_capacity).ok_or(InitError::AllocationFailed)?;

        Ok(Self {
            native,
            capacity,
            found: 0,
            connected: 0,
        })
    }

    /// Maximum number of controller slots allocated in this set.
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Number of controllers found by the most recent [`find`](Self::find).
    pub const fn found(&self) -> usize {
        self.found
    }

    /// Number of controllers connected by the most recent [`connect`](Self::connect).
    pub const fn connected(&self) -> usize {
        self.connected
    }

    /// Searches for nearby Wii Remotes for up to `timeout`.
    ///
    /// The timeout is rounded up to a whole second because that is the unit
    /// accepted by the native wiiuse API. The result never exceeds `capacity`.
    pub fn find(&mut self, timeout: Duration) -> usize {
        let seconds = timeout
            .as_secs()
            .saturating_add(if timeout.subsec_nanos() != 0 { 1 } else { 0 })
            .min(i32::MAX as u64) as i32;

        self.found = self.native.find(seconds).min(self.capacity);
        self.connected = 0;
        self.found
    }

    /// Connects to the controllers found by the most recent [`find`](Self::find).
    ///
    /// Returns the number of successfully connected controllers.
    pub fn connect(&mut self) -> usize {
        self.connected = if self.found == 0 {
            0
        } else {
            self.native.connect(self.found).min(self.found)
        };
        self.connected
    }

    /// Polls connected controllers for new input reports.
    ///
    /// Call this regularly before reading controller state.
    pub fn poll(&mut self) {
        if self.connected > 0 {
            self.native.poll(self.connected);
        }
    }

    /// Enables or disables the Wii Remote accelerometer for a connected slot.
    ///
    /// Returns `false` when `index` is not a connected controller slot.
    pub fn motion_sensing(&mut self, index: usize, enabled: bool) -> bool {
        if index >= self.connected {
            return false;
        }
        self.native.motion_sensing(index, self.connected, enabled);
        true
    }

    /// Returns an owned snapshot of the latest state for a connected slot.
    ///
    /// Call this after [`poll`](Self::poll). Returns `None` if the index is
    /// outside the connected range or the native device is absent.
    pub fn state(&self, index: usize) -> Option<WiimoteState> {
        if index >= self.connected {
            return None;
        }
        self.native
            .state(index, self.connected)
            .map(WiimoteState::from_raw)
    }
}
