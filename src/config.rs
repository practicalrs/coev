use crate::{error::Error, Args, Result};

#[derive(Clone, Debug)]
pub struct Config {
    pub cycles: u16,
    pub dir: String,
    pub model: String,
    pub ollama_host: String,
    pub theme: Option<String>,
}

impl Config {
    pub fn new(
        cycles: u16,
        dir: String,
        model: String,
        ollama_host: String,
        theme: Option<String>,
    ) -> Self {
        Self {
            cycles,
            dir,
            model,
            ollama_host,
            theme,
        }
    }
}

pub fn load(args: Args) -> Result<Config> {
    let cycles = args.cycles.unwrap_or(1);
    let dir = args.dir;
    let model = args.model;
    let ollama_host = if let Ok(val) = std::env::var("OLLAMA_HOST") {
        val
    } else {
        return Err(Box::new(Error::OllamaHostAddresMissing));
    };
    let theme = args.theme;

    let config = Config::new(cycles, dir, model, ollama_host, theme);

    Ok(config)
}
