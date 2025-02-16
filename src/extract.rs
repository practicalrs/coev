use crate::{error::Error, Result};

pub async fn extract_json(text: &str) -> Result<String> {
    let text = text.to_string();
    let text = if text.starts_with("```json") && text.ends_with("```") {
        text.strip_prefix("```json")
            .ok_or(Error::Parsing)?
            .to_string()
            .strip_suffix("```")
            .ok_or(Error::Parsing)?
            .to_string()
    } else if text.starts_with("```") && text.ends_with("```") {
        text.strip_prefix("```")
            .ok_or(Error::Parsing)?
            .to_string()
            .strip_suffix("```")
            .ok_or(Error::Parsing)?
            .to_string()
    } else {
        text
    };

    Ok(text)
}

pub async fn extract_rust(text: &str) -> Result<String> {
    let text = text.to_string();
    let lines = text.split('\n').collect::<Vec<&str>>();
    let mut code = String::new();
    let mut collect = false;
    for line in lines {
        let line = line.to_string();
        if line.starts_with("```") && collect {
            collect = false;
        }
        if collect {
            code.push_str(&format!("{line}\n"));
        }
        if line.starts_with("```rust") && !collect {
            collect = true;
        }
    }

    Ok(code)
}
