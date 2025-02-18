use crate::{
    config::Config,
    extract,
    ollama::{self, Message},
    Result,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct Score {
    documentation: u8,
    efficiency: u8,
    features: u8,
    maintainability: u8,
    readability: u8,
    robustness: u8,
    security: u8,
    test_coverage: u8,
}

pub async fn evaluate(config: Arc<Config>, source: &str, feature_added: bool) -> Result<u16> {
    let mut messages = vec![];

    let system_prompt = r#"You need to evaluate the given code. You need to score code from 1 to 10 in the following categories: documentation, efficiency, features, maintainability, readability, robustness, security, and test coverage. Make sure you have a correct score range. Don't explain, just provide scores. You respond in the following JSON format {"documentation": number, "efficiency": number, "features": number, "maintainability": number, "readability": number, "robustness": number, "security": number, "test_coverage": number }. Make sure that the response JSON is valid. Make sure to put the reply inside the ```json``` block."#;
    let message = Message {
        content: system_prompt.to_string(),
        role: "system".to_string(),
    };
    messages.push(message);

    if feature_added {
        let prompt = format!("The following code contains a new feature that improves the code. Make sure this new feature makes sense and take it into account while calculating scores.");
        let message = Message {
            content: prompt,
            role: "user".to_string(),
        };
        messages.push(message);
    }

    let prompt = format!("Here is the code:\n\n{}", source);
    let message = Message {
        content: prompt,
        role: "user".to_string(),
    };
    messages.push(message);

    let response = ollama::request(config, messages).await?;
    println!("response {:?}", response);
    let response = extract::extract_json(&response).await?;
    println!("response {:?}", response);
    let score: Score = serde_json::from_str(&response)?;
    println!("score {:?}", score);

    let overall = score.documentation
        + score.efficiency
        + score.features
        + score.maintainability
        + score.readability
        + score.robustness
        + score.security
        + score.test_coverage;

    Ok(overall.into())
}
