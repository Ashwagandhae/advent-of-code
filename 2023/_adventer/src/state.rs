use crate::problem::{ListParseable, Problem, ProblemDay};
use crate::{language::Language, problem::ProblemPart, styles::accent};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub work_state: WorkState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkState {
    pub problem: Problem,
    pub language: Language,
}
impl std::default::Default for State {
    fn default() -> Self {
        Self {
            work_state: WorkState {
                problem: Problem {
                    day: ProblemDay(1),
                    part: ProblemPart::Zero,
                },
                language: Language::Rust,
            },
        }
    }
}

pub fn modify_state(closure: impl FnOnce(&mut State)) -> Result<()> {
    let mut state = get_state()?;
    closure(&mut state);
    save_state_to_file(&state)?;
    Ok(())
}

pub fn set_work(problem: Option<Problem>, language: Option<Language>) -> Result<()> {
    modify_state(|state| {
        state.work_state = WorkState {
            problem: problem.unwrap_or(state.work_state.problem),
            language: language.unwrap_or(state.work_state.language),
        };
    })?;
    print_work_status()
}

pub fn init_work(problem: Option<Problem>, language: Option<Language>) -> Result<()> {
    set_work(problem, language)?;
    let state = get_state()?;
    println!(
        "init {} work for {}...",
        state.work_state.language.to_string(),
        accent(state.work_state.problem)
    );
    clear_work()?;
    open_work(true)?;
    Ok(())
}

use dialoguer;
pub fn save_work() -> Result<()> {
    let state = get_state()?.work_state;
    let filename = format!(
        "{}.{}",
        state.problem.to_string(),
        state.language.extension()
    );
    println!("saving {}", accent(&filename));
    let code = std::fs::read_to_string(state.language.work_code_path())?;
    let path = format!("./_solutions/{}", filename);
    // check if file exists
    if std::path::Path::new(&path).exists() {
        // take user input
        if !dialoguer::Confirm::new()
            .with_prompt(format!("{} already exists, overwrite?", filename))
            .interact()?
        {
            return Ok(());
        }
    }
    std::fs::write(path, code)?;
    Ok(())
}

pub fn save_and_next_work() -> Result<()> {
    save_work()?;
    modify_state(|state| state.work_state.problem = state.work_state.problem.next())?;
    print_work_status()
}

pub fn print_work_status() -> Result<()> {
    let state = get_state()?;
    println!(
        "working on {} in {}",
        accent(state.work_state.problem),
        state.work_state.language.to_string()
    );
    Ok(())
}

pub fn clear_work() -> Result<()> {
    let state = get_state()?;
    println!("clearing {} work", state.work_state.language.to_string());
    std::fs::write(
        state.work_state.language.work_code_path(),
        state.work_state.language.starter_file().0,
    )?;
    Ok(())
}

use crate::command;
pub fn open_work(at_pos: bool) -> Result<()> {
    let state = get_state()?;
    println!("opening {} work...", state.work_state.language.to_string());
    let (_, (line, col)) = state.work_state.language.starter_file();
    let target = format!(
        "{}{}",
        state.work_state.language.work_code_path(),
        if at_pos {
            format!(":{}:{}", line, col)
        } else {
            "".to_string()
        }
    );
    command::run_command(
        std::process::Command::new("code-insiders").args([".", "--goto", &target]),
    )?;
    Ok(())
}

pub fn get_state() -> Result<State> {
    // get state from file, if it exists and is valid
    let state = if let Some(state) = get_state_from_file()? {
        state
    } else {
        // otherwise, create a new state
        let state = State::default();
        // save state to file
        save_state_to_file(&state)?;
        state
    };
    Ok(state)
}
use std::fs;

fn get_state_from_file() -> Result<Option<State>> {
    let path = "./state.json";
    if !std::path::Path::new(path).exists() {
        return Ok(None);
    }
    let state = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&state).ok())
}

fn save_state_to_file(state: &State) -> Result<()> {
    let path = "./state.json";
    fs::write(path, serde_json::to_string_pretty(state)?)?;
    Ok(())
}
