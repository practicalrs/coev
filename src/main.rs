#![forbid(unsafe_code)]

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let result = coev::run();

    if let Err(e) = result.await {
        eprint!("Error: {:?}", e);
    }

    Ok(())
}
