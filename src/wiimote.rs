use std::ffi::c_void;
use std::ptr;
use std::thread;
use std::time::Duration;

use wiiuse_rs::*;

#[derive(Debug)]
pub struct Accelerometer {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Wiimote {
    devices: *mut c_void,
    count: i32,
}

impl Wiimote {
    pub fn connect() -> Result<Self, String> {
        unsafe {
            let devices = rust_wiiuse_init(1);

            if devices.is_null() {
                return Err("Unable to initialize wiiuse".into());
            }

            println!("Searching for Wii Remote...");
            println!("Press 1 + 2 on the controller.");

            let found = rust_wiiuse_find(devices, 1, 10);

            if found <= 0 {
                rust_wiiuse_cleanup(devices, 1);
                return Err("No Wii Remote found".into());
            }

            println!("Wii Remote found. Connecting...");

            let connected = rust_wiiuse_connect(devices, 1);

            if connected <= 0 {
                rust_wiiuse_cleanup(devices, 1);
                return Err("Connection failed".into());
            }

            rust_wiiuse_motion_sensing(devices, 0, 1);

            Ok(Self { devices, count: 1 })
        }
    }

    pub fn poll(&mut self) -> bool {
        unsafe { rust_wiiuse_poll(self.devices, self.count) > 0 }
    }

    pub fn accelerometer(&self) -> Accelerometer {
        unsafe {
            let mut x = 0.0;
            let mut y = 0.0;
            let mut z = 0.0;

            rust_wiiuse_get_gforce(self.devices, 0, &mut x, &mut y, &mut z);

            Accelerometer { x, y, z }
        }
    }

    pub fn read_loop(&mut self) {
        loop {
            self.poll();

            let accel = self.accelerometer();

            println!(
                "X: {:>7.3} | Y: {:>7.3} | Z: {:>7.3}",
                accel.x, accel.y, accel.z
            );

            thread::sleep(Duration::from_millis(20));
        }
    }
}

impl Drop for Wiimote {
    fn drop(&mut self) {
        unsafe {
            if !self.devices.is_null() {
                rust_wiiuse_cleanup(self.devices, self.count);

                self.devices = ptr::null_mut();
            }
        }
    }
}
