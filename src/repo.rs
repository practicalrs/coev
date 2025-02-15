use crate::{error::Error, Result};
use std::{fs::read_to_string, path::Path};

pub async fn read_source(dir: &str) -> Result<String> {
    let path = format!("{dir}/src/lib.rs");

    let exists = Path::new(&path);

    if exists.is_file() {
        let content = read_to_string(path)?;

        return Ok(content);
    }

    Err(Box::new(Error::SourceFileNotFound))
}
