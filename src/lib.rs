use clap::Parser;
use std::{error::Error, sync::Arc};

mod config;
mod error;
mod evolution;
mod extract;
mod ollama;
mod repo;
mod score;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Debug, Parser)]
#[command(about, author, long_about = None, version)]
pub struct Args {
    /// Repo directory
    #[arg(long, short)]
    pub dir: String,

    /// Ollama model
    #[arg(long, short)]
    pub model: String,

    /// Program theme
    #[arg(long, short)]
    pub theme: Option<String>,
}

pub async fn run() -> Result<()> {
    let args = Args::parse();
    let config = Arc::new(config::load(args)?);

    let source = repo::read_source(&config.dir).await?;

    evolution::evolve(config, &source).await?;

    Ok(())
}
