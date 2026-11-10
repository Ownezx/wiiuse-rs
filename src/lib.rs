pub mod wiimote;

use wiimote::Wiimote;

pub fn run() -> Result<(), String> {
    let mut remote = Wiimote::connect()?;

    println!("Connected!");
    println!("Reading accelerometer values. Press Ctrl+C to stop.\n");

    remote.read_loop();

    Ok(())
}
