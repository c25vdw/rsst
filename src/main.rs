use rsst::{App, RawInputMode};
use std::error::Error;
use std::io::{self, Read};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new()?;
    let _raw_input_mode = RawInputMode::new().unwrap();

    app.start()?;

    Ok(())
}
