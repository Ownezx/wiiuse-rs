use std::thread;
use std::time::{Duration, Instant};

use wiiuse_rs::{WiimoteButtons, WiimoteSet};

const MAX_WIIMOTES: i32 = 1;
const FIND_TIMEOUT_SECONDS: i32 = 10;
const LONG_PRESS: Duration = Duration::from_secs(1);
const POLL_SLEEP: Duration = Duration::from_millis(2);
const ACCEL_PRINT_INTERVAL: Duration = Duration::from_millis(50); // 20 lines/second

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

    // WiimoteSet owns the native device array and cleans it up automatically.
    let mut devices = match WiimoteSet::new(MAX_WIIMOTES as usize) {
        Ok(devices) => devices,
        Err(error) => {
            eprintln!("Could not allocate Wii Remote state: {error}");
            return;
        }
    };

    let found = devices.find(Duration::from_secs(FIND_TIMEOUT_SECONDS as u64));
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

    // Enable the Wii Remote accelerometer and IR sensor.
    devices.motion_sensing(0, true);
    devices.ir_sensing(0, true);

    println!("Connected.");
    println!("Press any button to print its name.");
    println!("Hold A for 1 second to start the accelerometer feed; release A to stop it.");
    println!("Hold B for 1 second to start the IR feed; release B to stop it.");

    let mut a_press_started: Option<Instant> = None;
    let mut accelerometer_feed_active = false;
    let mut b_press_started: Option<Instant> = None;
    let mut ir_feed_active = false;
    let mut next_accelerometer_print = Instant::now();
    let mut next_ir_print = Instant::now();

    loop {
        // Poll frequently to keep input latency low. The feeds themselves are
        // throttled separately so they print at a readable 20 Hz.
        devices.poll();

        if let Some(state) = devices.state(0) {
            // Print each button once, on its press transition.
            for (button, label) in BUTTONS {
                if state.buttons.just_pressed.contains(button) {
                    println!("Pressed {label}");
                }
            }

            // A-button long press controls the accelerometer feed.
            let a_is_down = state.buttons.down.contains(WiimoteButtons::A)
                || state.buttons.held.contains(WiimoteButtons::A);

            if state.buttons.just_pressed.contains(WiimoteButtons::A) {
                a_press_started = Some(Instant::now());
                accelerometer_feed_active = false;
            }

            if a_is_down {
                // Fallback in case the first state snapshot arrives after the
                // initial press-transition report.
                let started = *a_press_started.get_or_insert_with(Instant::now);

                if !accelerometer_feed_active && started.elapsed() >= LONG_PRESS {
                    accelerometer_feed_active = true;
                    next_accelerometer_print = Instant::now();
                    println!("A held for 1 second — accelerometer feed started.");
                    println!("Release A to stop the feed.");
                }
            } else if a_press_started.take().is_some() {
                if accelerometer_feed_active {
                    println!("A released — accelerometer feed stopped.");
                }
                accelerometer_feed_active = false;
            }

            // B-button long press controls the IR position feed.
            let b_is_down = state.buttons.down.contains(WiimoteButtons::B)
                || state.buttons.held.contains(WiimoteButtons::B);

            if state.buttons.just_pressed.contains(WiimoteButtons::B) {
                b_press_started = Some(Instant::now());
                ir_feed_active = false;
            }

            if b_is_down {
                let started = *b_press_started.get_or_insert_with(Instant::now);

                if !ir_feed_active && started.elapsed() >= LONG_PRESS {
                    ir_feed_active = true;
                    next_ir_print = Instant::now();
                    println!("B held for 1 second — IR feed started.");
                    println!("Release B to stop the feed.");
                }
            } else if b_press_started.take().is_some() {
                if ir_feed_active {
                    println!("B released — IR feed stopped.");
                }
                ir_feed_active = false;
            }

            // Print estimated IR position at 20 Hz while B's feed is active.
            if ir_feed_active && Instant::now() >= next_ir_print {
                match state.ir.pointer {
                    Some([x, y]) => {
                        println!("IR position: X={x}, Y={y}");
                    }
                    None => {
                        println!("IR position: unavailable");
                    }
                }

                next_ir_print = Instant::now() + ACCEL_PRINT_INTERVAL;
            }

            // Print accelerometer readings at 20 Hz while A's feed is active.
            if accelerometer_feed_active && Instant::now() >= next_accelerometer_print {
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

    // The loop intentionally runs until the process is stopped.
    // WiimoteSet handles native cleanup automatically when dropped.
}
