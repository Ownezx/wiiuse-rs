fn main() {
    if let Err(error) = wiimote_osc::run() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}
