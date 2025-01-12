use anyhow::{Context, Result};

use crate::resource::{
    get_example_input, get_example_output, get_input, get_output, submit_answer, SubmissionOutcome,
};
use crate::state::get_state;
use crate::styles::{accent, error, example, real, success, warning};
use crate::{language::Language, problem::Problem, RunTarget};

use std::fs;

pub fn run_from_work(target: RunTarget, quiet: bool) -> Result<()> {
    let state = get_state()?.work_state;
    let code = fs::read_to_string(state.language.work_code_path())?;
    run(state.problem, state.language, &code, target, quiet)
}

pub fn run_from_name(problem: Problem, target: RunTarget, quiet: bool) -> Result<()> {
    // find correct in _solutions
    let file = fs::read_dir("./_solutions")?
        .filter_map(|entry| Some(entry.ok()?.path()))
        .find(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(&problem.to_string()))
        })
        .context(format!("no solution file for {}", accent(problem)))?;

    let code = fs::read_to_string(file.clone())?;
    let extension = file
        .extension()
        .context(format!("no extension for {}", file.display()))?
        .to_str()
        .context(format!("invalid extension for {}", file.display()))?;

    let lang = Language::from_extension(extension).context(format!(
        "extension matches no known lang: {}",
        accent(extension)
    ))?;
    run(problem, lang, &code, target, quiet)
}

fn run(name: Problem, lang: Language, code: &str, target: RunTarget, quiet: bool) -> Result<()> {
    let example_run = match target {
        RunTarget::Example
        | RunTarget::All
        | RunTarget::AllThenSubmit
        | RunTarget::ExampleInputChecked => Some(false),
        RunTarget::ExampleChecked
        | RunTarget::AllChecked
        | RunTarget::ExampleCheckedInput
        | RunTarget::ExampleCheckedInputThenSubmit => Some(true),
        _ => None,
    };
    let input_run = match target {
        RunTarget::Input
        | RunTarget::InputThenSubmit
        | RunTarget::All
        | RunTarget::AllThenSubmit
        | RunTarget::ExampleCheckedInput
        | RunTarget::ExampleCheckedInputThenSubmit => Some(false),
        RunTarget::InputChecked | RunTarget::AllChecked | RunTarget::ExampleInputChecked => {
            Some(true)
        }
        _ => None,
    };
    let mut success = true;
    if let Some(check) = example_run {
        (success, _) = run_and_check(
            example("example").to_string(),
            &name,
            &lang,
            check,
            || get_example_input(name),
            || get_example_output(name),
            code,
            quiet,
        )?;
    }
    let skip_run_input = matches!(
        target,
        RunTarget::ExampleChecked
            | RunTarget::AllChecked
            | RunTarget::ExampleCheckedInput
            | RunTarget::ExampleCheckedInputThenSubmit
    ) && !success;
    if let Some(check) = input_run {
        if skip_run_input {
            println!(
                "{} {} on {}...",
                error("skipping"),
                accent(name),
                real("real")
            );
        } else {
            let (_, answer) = run_and_check(
                real("real").to_string(),
                &name,
                &lang,
                check,
                || get_input(name.day),
                || get_output(name),
                code,
                quiet,
            )?;
            if matches!(
                target,
                RunTarget::InputThenSubmit
                    | RunTarget::AllThenSubmit
                    | RunTarget::ExampleCheckedInputThenSubmit
            ) {
                if is_valid_answer(&answer) {
                    println!(
                        "{} {} for {}...",
                        warning("submitting"),
                        answer,
                        accent(name)
                    );
                    let outcome = submit_answer(name, &answer)?;
                    match outcome {
                        SubmissionOutcome::Correct(submitted) => {
                            println!(
                                "{} {}",
                                crate::styles::success("correct!"),
                                if submitted {
                                    warning("(already submitted)").to_string()
                                } else {
                                    "".into()
                                }
                            )
                        }
                        SubmissionOutcome::Incorrect(submitted) => {
                            println!(
                                "{} {}",
                                error("wrong!"),
                                if submitted {
                                    warning("(already submitted)").to_string()
                                } else {
                                    "".into()
                                }
                            )
                        }
                        SubmissionOutcome::Wait => {
                            println!("{}", warning("wait!"))
                        }
                        SubmissionOutcome::WrongLevel => {
                            println!("{}", warning("wrong level?"))
                        }
                    }
                } else {
                    println!(
                        "{} {}",
                        warning("skipping submission, invalid answer: {}"),
                        error(answer)
                    );
                }
            }
        }
    }
    Ok(())
}

pub fn is_valid_answer(answer: &str) -> bool {
    answer.chars().all(|c| c.is_ascii_digit())
}

#[allow(clippy::too_many_arguments)]
pub fn run_and_check(
    section: String,
    name: &Problem,
    lang: &Language,
    check: bool,
    get_input: impl FnOnce() -> Result<String>,
    get_output: impl FnOnce() -> Result<String>,
    code: &str,
    quiet: bool,
) -> Result<(bool, String)> {
    println!(
        "{} {} on {}...",
        if check {
            warning("checking").to_string()
        } else {
            "running".to_string()
        },
        accent(*name),
        section
    );
    let input = get_input()?;
    let output = lang.run(&input, code, quiet)?;
    let answer = get_answer_from_output(&output)?;
    let success = if check {
        let correct_answer = get_output()?;
        if answer == correct_answer {
            println!("{} got {}", success("correct!"), success(answer.clone()));
            (true, answer)
        } else {
            println!(
                "{} expected: {}, got: {}",
                error("wrong!"),
                warning(correct_answer),
                error(answer.clone())
            );
            (false, answer)
        }
    } else {
        (true, answer)
    };
    Ok(success)
}

use strip_ansi_escapes;
pub fn get_answer_from_output(output: &str) -> Result<String> {
    // split into lines and get last line
    let output = output
        .split('\n')
        .filter(|line| !line.is_empty())
        .last()
        .unwrap_or(output)
        .trim();
    let output = strip_ansi_escapes::strip(output);
    Ok(String::from_utf8(output)?)
}

pub fn create_input_file(path: &str, input: &str) -> Result<()> {
    fs::write(format!("{}/{}", path, "input.txt"), input)?;
    Ok(())
}

use std::io::{BufRead, BufReader};
use std::process::Stdio;

pub fn run_code(
    code: &str,
    file_path: &str,
    command_program: &str,
    command_args: &[&str],
    command_path: &str,
    quiet: bool,
) -> Result<String> {
    fs::write(file_path, code)?;
    let mut child = std::process::Command::new("unbuffer")
        .arg(command_program)
        .args(command_args)
        .current_dir(command_path)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let child_stdout = child
        .stdout
        .take()
        .expect("internal error, could not take stdout");

    let stdout_lines = BufReader::new(child_stdout).lines();
    let mut stdout = String::new();
    for line in stdout_lines {
        let line = line?;
        if !quiet {
            println!("{}", line);
        }
        stdout.push_str(format!("{}\n", line).as_str());
    }

    Ok(stdout.trim().to_string())
}
