use rsst::{App, RawInputMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _raw_input_mode = RawInputMode::new().unwrap();
    let mut app = App::new()?;

    app.start()?;

    Ok(())
}
