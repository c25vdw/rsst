use anyhow::Result;
use rsst::{run, RawInputMode};

pub fn main() -> Result<()> {
    let _raw_input_mode = RawInputMode::new().unwrap();
    if let Err(e) = run() {
        drop(_raw_input_mode);
        eprintln!("main error: {:?}", e);
    }
    Ok(())
}
