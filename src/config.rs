use crate::{error::Error, Args, Result};

#[derive(Clone, Debug)]
pub struct Config {
    pub dir: String,
    pub model: String,
    pub ollama_host: String,
    pub theme: Option<String>,
}

impl Config {
    pub fn new(dir: String, model: String, ollama_host: String, theme: Option<String>) -> Self {
        Self {
            dir,
            model,
            ollama_host,
            theme,
        }
    }
}

pub fn load(args: Args) -> Result<Config> {
    let dir = args.dir;
    let model = args.model;
    let ollama_host = if let Ok(val) = std::env::var("OLLAMA_HOST") {
        val
    } else {
        return Err(Box::new(Error::OllamaHostAddresMissing));
    };
    let theme = args.theme;

    let config = Config::new(dir, model, ollama_host, theme);

    Ok(config)
}
