use clap::Parser;
use std::error::Error;

mod config;
mod error;
mod repo;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Debug, Parser)]
#[command(about, author, long_about = None, version)]
pub struct Args {
    /// Repo directory
    #[arg(long, short)]
    pub dir: String,

    /// Program theme
    #[arg(long, short)]
    pub theme: Option<String>,
}

pub async fn run() -> Result<()> {
    let args = Args::parse();

    let config = config::load(args)?;

    let source = repo::read_source(&config.dir).await?;
    println!("test {:?}", source);
    Ok(())
}
