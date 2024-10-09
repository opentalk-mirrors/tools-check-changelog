use std::{
    fs,
    io::{self, Read as _},
    path::Path,
};

use anyhow::Context as _;

pub fn read_input(source: &Path) -> anyhow::Result<String> {
    if source.to_string_lossy() == "-" {
        read_from_stdin()
    } else {
        read_from_file(source)
    }
}

fn read_from_stdin() -> anyhow::Result<String> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin
        .read_to_string(&mut buffer)
        .context("Failed to read stdin")?;
    log::debug!("Read input from stdin");

    Ok(buffer)
}

fn read_from_file(source: &Path) -> anyhow::Result<String> {
    // read task from stdin
    let mut buffer = String::new();
    let mut stdin = fs::File::open(source)
        .with_context(|| format!("Failed to open file {}", source.to_string_lossy()))?;
    stdin
        .read_to_string(&mut buffer)
        .with_context(|| format!("Failed to read file {}", source.to_string_lossy()))?;
    log::debug!("Read input from file");

    Ok(buffer)
}
