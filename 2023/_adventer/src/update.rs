use anyhow::Result;
use self_replace;

use std::process::Command;

use crate::command::run_command;

pub fn update() -> Result<()> {
    println!("building...");
    run_command(
        Command::new("cargo")
            .current_dir("./_adventer")
            .args(["build", "--release"]),
    )?;
    println!("replacing...");
    let new_binary = "./_adventer/target/release/_adventer";
    self_replace::self_replace(&new_binary)?;
    Ok(())
}
