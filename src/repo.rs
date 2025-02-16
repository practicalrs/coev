use crate::{error::Error, Result};
use std::{
    fs::{read_to_string, write},
    path::Path,
};

pub async fn read_source(dir: &str) -> Result<String> {
    let path = format!("{dir}/src/lib.rs");

    let exists = Path::new(&path);

    if exists.is_file() {
        let content = read_to_string(path)?;

        return Ok(content);
    }

    Err(Box::new(Error::SourceFileNotFound))
}

pub async fn write_source(dir: &str, source: &str) -> Result<()> {
    let path = format!("{dir}/src/lib.rs");

    let exists = Path::new(&path);

    if exists.is_file() {
        write(path, source)?;

        return Ok(());
    }

    Err(Box::new(Error::SourceFileNotFound))
}
