use crate::{error::Error, Result};

pub async fn extract_json(text: &str) -> Result<String> {
    let text = text.to_string();
    let text = if text.starts_with("```json")
        && text.ends_with("```")
    {
        text
            .strip_prefix("```json")
            .ok_or(Error::Parsing)?
            .to_string()
            .strip_suffix("```")
            .ok_or(Error::Parsing)?
            .to_string()
    } else if text.starts_with("```")
        && text.ends_with("```")
    {
        text
            .strip_prefix("```")
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
