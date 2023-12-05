use anyhow::Result;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("command errored: {message}")]
    CommandError { message: String },
    #[error("command failed to execute: {0}")]
    CommandFailed(#[from] std::io::Error),
}

pub fn run_command(command: &mut Command) -> Result<()> {
    let output = command.output()?;
    if !output.status.success() {
        return Err(CommandError::CommandError {
            message: String::from_utf8(output.stderr).unwrap(),
        })?;
    }
    Ok(())
}
