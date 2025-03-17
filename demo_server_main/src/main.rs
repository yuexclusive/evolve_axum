// use demo_server_main;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    demo_server_main::start_server().await?;
    Ok(())
}
