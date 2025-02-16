use crate::{config::Config, extract, ollama::{self, Message}, Result};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct Score {
    documentation: u64,
    efficiency: u64,
    features: u64,
    maintainability: u64,
    readability: u64,
    robustness: u64,
    security: u64,
    test_coverage: u64,
}

pub async fn evaluate(config: Arc<Config>, source: &str) -> Result<u64> {
    let mut messages = vec![];

    let system_prompt = r#"You need to evaluate the given code. You need to score code from 0 to 255 in the following categories: documentation, efficiency, features, maintainability, readability, robustness, security, and test coverage. You respond in the following JSON format {"documentation": number, "efficiency": number, "features": number, "maintainability": number, "readability": number, "robustness": number, "security": number, "test_coverage": number }. Make sure that the response JSON is valid. Make sure to put the reply inside the ```json``` block."#;
    let message = Message {
        content: system_prompt.to_string(),
        role: "system".to_string(),
    };
    messages.push(message);

    let prompt = format!("Here is the code:\n\n{}", source);
    let message = Message {
        content: prompt,
        role: "user".to_string(),
    };
    messages.push(message);

    let response = ollama::request(config, messages).await?;
    let response = extract::extract_json(&response).await?;
    let score: Score = serde_json::from_str(&response)?;

    println!("response = {:?}", score);

    let overall = score.documentation * score.efficiency * score.features * score.maintainability * score.readability * score.robustness * score.security * score.test_coverage;

    Ok(overall)
}
