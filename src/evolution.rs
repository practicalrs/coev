use crate::{config::Config, score, Result};
use std::sync::Arc;

pub async fn evolve(config: Arc<Config>, source: &str) -> Result<()> {
    println!("Step 1 - getting a score for the original program.");
    let score = score::evaluate(config, &source).await?;
    println!("score = {}", score);

    Ok(())
}
