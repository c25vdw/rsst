use rsst::{App, RawInputMode};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _raw_input_mode = RawInputMode::new().unwrap();
    let mut app = App::new()?;

    app.start().await?;

    Ok(())
}
