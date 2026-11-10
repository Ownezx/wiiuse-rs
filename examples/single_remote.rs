use std::thread;
use std::time::{Duration, Instant};

use wiiuse_rs::{WiimoteButtons, WiimoteSet};

const MAX_WIIMOTES: usize = 1;
const FIND_TIMEOUT: Duration = Duration::from_secs(10);
const POLL_SLEEP: Duration = Duration::from_millis(2);
const ACCEL_PRINT_INTERVAL: Duration = Duration::from_millis(50); // 20 Hz

const BUTTONS: [(WiimoteButtons, &str); 11] = [
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

fn main() {
    println!("Searching for Wii Remote...");
    println!("Press 1 + 2 on the controller.");

    let mut wiimotes = match WiimoteSet::new(MAX_WIIMOTES) {
        Ok(wiimotes) => wiimotes,
        Err(error) => {
            eprintln!("Could not initialize wiiuse: {error}");
            return;
        }
    };

    let found = wiimotes.find(FIND_TIMEOUT);
    if found == 0 {
        eprintln!("No Wii Remote found.");
        return;
    }
    println!("Found {found} Wii Remote(s). Connecting...");

    let connected = wiimotes.connect();
    if connected == 0 {
        eprintln!("Could not connect to the Wii Remote.");
        return;
    }

    wiimotes.motion_sensing(0, true);
    println!("Connected.");
    println!("Press any button to print its name.");
    println!("Accelerometer output streams continuously at 20 Hz.");

    let mut next_accelerometer_print = Instant::now();

    loop {
        // Poll rapidly for responsive buttons; throttle only console output.
        wiimotes.poll();

        if let Some(state) = wiimotes.state(0) {
            for (button, label) in BUTTONS {
                if state.buttons.just_pressed.contains(button) {
                    println!("Pressed {label}");
                }
            }

            if Instant::now() >= next_accelerometer_print {
                let accel = state.accelerometer;
                println!(
                    "Accel: X={:+.3}g Y={:+.3}g Z={:+.3}g | raw=({}, {}, {})",
                    accel.gforce[0],
                    accel.gforce[1],
                    accel.gforce[2],
                    accel.raw[0],
                    accel.raw[1],
                    accel.raw[2],
                );
                next_accelerometer_print = Instant::now() + ACCEL_PRINT_INTERVAL;
            }
        }

        thread::sleep(POLL_SLEEP);
    }
}
