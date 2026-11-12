//! Show individual Wii Remote and Nunchuk sensor feeds while their controls are held.
//!
//! Hold Wii Remote A for two seconds to display the Wii Remote accelerometer.
//! Hold Nunchuk Z for two seconds to display the Nunchuk accelerometer.
//! Hold Nunchuk C for two seconds to display the Nunchuk joystick.
//! Release a control to stop its corresponding feed.

use std::thread;
use std::time::{Duration, Instant};

use wiiuse_rs::{NunchukButtons, WiimoteButtons, WiimoteSet};

const MAX_WIIMOTES: usize = 1;
const FIND_TIMEOUT: Duration = Duration::from_secs(10);
const SENSOR_LONG_PRESS: Duration = Duration::from_secs(2);
const POLL_SLEEP: Duration = Duration::from_millis(2);
const SENSOR_PRINT_INTERVAL: Duration = Duration::from_millis(50); // 20 updates/second

const WIIMOTE_BUTTONS: [(WiimoteButtons, &str); 11] = [
    (WiimoteButtons::TWO, "2"),
    (WiimoteButtons::ONE, "1"),
    (WiimoteButtons::A, "A"),
    (WiimoteButtons::B, "B"),
    (WiimoteButtons::PLUS, "Plus"),
    (WiimoteButtons::MINUS, "Minus"),
    (WiimoteButtons::HOME, "Home"),
    (WiimoteButtons::UP, "Up"),
    (WiimoteButtons::DOWN, "Down"),
    (WiimoteButtons::LEFT, "Left"),
    (WiimoteButtons::RIGHT, "Right"),
];

const NUNCHUK_BUTTONS: [(NunchukButtons, &str); 2] = [
    (NunchukButtons::C, "Nunchuk C"),
    (NunchukButtons::Z, "Nunchuk Z"),
];

/// Starts a sensor display after a continuous long press, and stops it on release.
#[derive(Default)]
struct HoldToShow {
    pressed_since: Option<Instant>,
    active: bool,
}

impl HoldToShow {
    /// Update this control and return true when its display has just started.
    fn update(&mut self, is_down: bool, now: Instant, control_name: &str) -> bool {
        let was_active = self.active;

        if is_down {
            // Also handles the case where the first observed state is already held.
            let started = *self.pressed_since.get_or_insert(now);

            if !self.active && now.duration_since(started) >= SENSOR_LONG_PRESS {
                self.active = true;
                println!("{control_name} held for 2 seconds — display started.");
                println!("Release {control_name} to stop its display.");
            }
        } else {
            self.pressed_since = None;

            if self.active {
                self.active = false;
                println!("{control_name} released — display stopped.");
            }
        }

        !was_active && self.active
    }

    /// Stop a feed without describing a disconnected Nunchuk as a button release.
    fn cancel(&mut self) -> bool {
        let was_active = self.active;
        self.pressed_since = None;
        self.active = false;
        was_active
    }
}

fn main() {
    println!("Searching for Wii Remote...");
    println!("Press 1 + 2 on the controller.");

    // WiimoteSet owns the native device array and cleans it up automatically.
    let mut devices = match WiimoteSet::new(MAX_WIIMOTES) {
        Ok(devices) => devices,
        Err(error) => {
            eprintln!("Could not allocate Wii Remote state: {error}");
            return;
        }
    };

    let found = devices.find(FIND_TIMEOUT);

    if found == 0 {
        eprintln!("No Wii Remote found.");
        return;
    }

    println!("Found {found} Wii Remote(s). Connecting...");

    let connected = devices.connect();

    if connected == 0 {
        eprintln!("Could not connect to the Wii Remote.");
        return;
    }

    // Enable motion reporting for the Wii Remote and its attached Nunchuk.
    devices.motion_sensing(0, true);

    println!("Connected.");
    println!("Press any Wii Remote or Nunchuk button to print its name.");
    println!("Hold Wii Remote A for 2 seconds: show Wii Remote accelerometer.");
    println!("Hold Nunchuk Z for 2 seconds: show Nunchuk accelerometer.");
    println!("Hold Nunchuk C for 2 seconds: show Nunchuk joystick X/Y.");
    println!("Release each control to stop its corresponding display.");

    let mut wiimote_accel_feed = HoldToShow::default();
    let mut nunchuk_accel_feed = HoldToShow::default();
    let mut joystick_feed = HoldToShow::default();

    let mut nunchuk_was_connected = false;
    let mut next_sensor_print = Instant::now();

    loop {
        // Poll frequently to keep input latency low. Sensor output is throttled
        // separately so each active feed remains readable at 20 Hz.
        devices.poll();

        if let Some(state) = devices.state(0) {
            // Keep press notifications for all 11 buttons on the main Wii Remote.
            for (button, label) in WIIMOTE_BUTTONS {
                if state.buttons.just_pressed.contains(button) {
                    println!("Pressed {label}");
                }
            }

            let now = Instant::now();

            let wiimote_a_is_down = state.buttons.down.contains(WiimoteButtons::A)
                || state.buttons.held.contains(WiimoteButtons::A);

            let wiimote_accel_started =
                wiimote_accel_feed.update(wiimote_a_is_down, now, "Wii Remote A");

            let mut nunchuk_accel_started = false;
            let mut joystick_started = false;

            if let Some(nunchuk) = state.nunchuk.as_ref() {
                if !nunchuk_was_connected {
                    println!("Nunchuk connected.");
                    nunchuk_was_connected = true;
                }

                // Keep press notifications for both Nunchuk buttons.
                for (button, label) in NUNCHUK_BUTTONS {
                    if nunchuk.buttons.just_pressed.contains(button) {
                        println!("Pressed {label}");
                    }
                }

                let nunchuk_z_is_down = nunchuk.buttons.down.contains(NunchukButtons::Z)
                    || nunchuk.buttons.held.contains(NunchukButtons::Z);

                let nunchuk_c_is_down = nunchuk.buttons.down.contains(NunchukButtons::C)
                    || nunchuk.buttons.held.contains(NunchukButtons::C);

                nunchuk_accel_started =
                    nunchuk_accel_feed.update(nunchuk_z_is_down, now, "Nunchuk Z");

                joystick_started = joystick_feed.update(nunchuk_c_is_down, now, "Nunchuk C");
            } else {
                // Cancel Nunchuk feeds if the accessory is unplugged.
                if nunchuk_was_connected {
                    let nunchuk_feed_was_active =
                        nunchuk_accel_feed.cancel() | joystick_feed.cancel();

                    if nunchuk_feed_was_active {
                        println!("Nunchuk disconnected — Nunchuk sensor displays stopped.");
                    } else {
                        println!("Nunchuk disconnected.");
                    }
                } else {
                    nunchuk_accel_feed.cancel();
                    joystick_feed.cancel();
                }

                nunchuk_was_connected = false;
            }

            // Start the print timer immediately when any previously inactive feed starts.
            if wiimote_accel_started || nunchuk_accel_started || joystick_started {
                next_sensor_print = now;
            }

            if now >= next_sensor_print {
                // Wii Remote accelerometer, controlled by Wii Remote A.
                if wiimote_accel_feed.active {
                    let accel = &state.accelerometer;

                    println!(
                        "Wii Remote accel: X={:+.3}g Y={:+.3}g Z={:+.3}g | raw=({}, {}, {})",
                        accel.gforce[0],
                        accel.gforce[1],
                        accel.gforce[2],
                        accel.raw[0],
                        accel.raw[1],
                        accel.raw[2],
                    );
                }

                if let Some(nunchuk) = state.nunchuk.as_ref() {
                    // Nunchuk accelerometer, controlled by Nunchuk Z.
                    if nunchuk_accel_feed.active {
                        let accel = &nunchuk.accelerometer;

                        println!(
                            "Nunchuk accel: X={:+.3}g Y={:+.3}g Z={:+.3}g | raw=({}, {}, {})",
                            accel.gforce[0],
                            accel.gforce[1],
                            accel.gforce[2],
                            accel.raw[0],
                            accel.raw[1],
                            accel.raw[2],
                        );
                    }

                    // Nunchuk joystick, controlled by Nunchuk C.
                    if joystick_feed.active {
                        let joystick = &nunchuk.joystick;

                        println!(
                            "Nunchuk joystick: X={:+.3} Y={:+.3}",
                            joystick.x, joystick.y,
                        );
                    }
                }

                next_sensor_print = now + SENSOR_PRINT_INTERVAL;
            }
        }

        thread::sleep(POLL_SLEEP);
    }
}
