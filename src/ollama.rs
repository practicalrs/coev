use crate::{config::Config, error::Error, Result};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::Duration;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    pub content: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct OllamaRequest {
    pub messages: Vec<Message>,
    pub model: String,
    pub stream: bool,
}

#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    //    created_at: String,
    //    done: bool,
    //    eval_count: i64,
    //    eval_duration: i64,
    //    load_duration: i64,
    message: Message,
    //    model: String,
    //    prompt_eval_count: i64,
    //    prompt_eval_duration: i64,
    //    total_duration: i64,
}

pub async fn request(config: Arc<Config>, messages: Vec<Message>) -> Result<String> {
    let ollama_request = OllamaRequest {
        messages,
        model: config.model.clone(),
        stream: false,
    };

    let url = format!("{}/api/chat", config.ollama_host);

    let response = reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(60))
        .build()?
        .post(url)
        .json(&ollama_request)
        .send()
        .await;

    match response {
        Err(_) => {
            return Err(Box::new(Error::OllamaRequestProblem));
        }
        Ok(response) => {
            if response.status() == StatusCode::CREATED || response.status() == StatusCode::OK {
                let response_text = response.text().await?;

                let ollama_response: OllamaResponse = serde_json::from_str(&response_text)?;

                return Ok(ollama_response.message.content);
            }
        }
    }

    Err(Box::new(Error::OllamaRequestProblem))
}
