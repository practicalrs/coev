use crate::{error::Error, Result};
use std::{
    fs::{read_to_string, write},
    path::Path,
    process::Command,
};

pub async fn commit_cycle(cycle: u16, dir: &str) -> Result<()> {
    let message = format!("Cycle {cycle}");

    let commit_command = format!("cd {} && git add . && git commit -a -m \"{message}\"", dir);
    let _commit_command_result = Command::new("sh").arg("-c").arg(commit_command).output()?;

    Ok(())
}

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
