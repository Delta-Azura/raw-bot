use crate::compare::compare;
use anyhow::{Result, Context};
use std::fs;

pub fn building() -> Result<()> {
    let packages: Vec<String> = compare().context("Failed to compare")?;
    let configuration = fs::read_to_string("/etc/raw-bot/building").context("Shell script doesn't exist")?;
    if configuration.is_empty() {
        anyhow::bail!("The environment setup instructions are empty")
    }
    return Ok(());
}