use wiimote_rust::wiimote::Wiimote;

fn main() -> Result<(), String> {
    let mut remote = Wiimote::connect()?;

    println!("Accelerometer streaming started.");

    remote.read_loop();

    Ok(())
}
