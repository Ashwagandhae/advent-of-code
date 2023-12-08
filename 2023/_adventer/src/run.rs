use anyhow::Result;

use crate::resource::{get_example_input, get_example_output, get_input, get_output};
use crate::state::get_state;
use crate::styles::{accent, error, example, real, success, warning};
use crate::{language::Language, ProblemName, RunTarget};

use std::fs;

pub fn run_from_work(target: RunTarget) -> Result<()> {
    let state = get_state()?.work_state;
    let code = fs::read_to_string(state.language.work_code_path())?;
    run(state.problem, state.language, &code, target)
}

pub fn run_from_name(name: ProblemName, target: RunTarget) -> Result<()> {
    // find correct in _solutions
    let file = fs::read_dir("./_solutions")
        .unwrap()
        .filter_map(|entry| Some(entry.ok()?.path()))
        .find(|path| {
            path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with(&name.to_string())
        })
        .expect("file not found");

    let code = fs::read_to_string(file.clone()).unwrap();
    let extension = file.extension().unwrap().to_str().unwrap();
    let Some(lang) = Language::from_extension(extension) else {anyhow::bail!("invalid extension: {}", extension)};
    run(name, lang, &code, target)
}

fn run(name: ProblemName, lang: Language, code: &str, target: RunTarget) -> Result<()> {
    let example_run = match target {
        RunTarget::Example | RunTarget::All | RunTarget::ExampleInputChecked => Some(false),
        RunTarget::ExampleChecked | RunTarget::AllChecked | RunTarget::ExampleCheckedInput => {
            Some(true)
        }
        _ => None,
    };
    let input_run = match target {
        RunTarget::Input | RunTarget::All | RunTarget::ExampleCheckedInput => Some(false),
        RunTarget::InputChecked | RunTarget::AllChecked | RunTarget::ExampleInputChecked => {
            Some(true)
        }
        _ => None,
    };
    let mut success = true;
    if let Some(check) = example_run {
        success = run_and_check(
            example("example").to_string(),
            &name,
            &lang,
            check,
            || get_example_input(name),
            || get_example_output(name),
            code,
        )?;
    }
    let skip_run_input = matches!(
        target,
        RunTarget::ExampleChecked | RunTarget::AllChecked | RunTarget::ExampleCheckedInput
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
            run_and_check(
                real("real").to_string(),
                &name,
                &lang,
                check,
                || get_input(name.day),
                || get_output(name),
                code,
            )?;
        }
    }
    Ok(())
}

pub fn run_and_check(
    section: String,
    name: &ProblemName,
    lang: &Language,
    check: bool,
    get_input: impl FnOnce() -> Result<String>,
    get_output: impl FnOnce() -> Result<String>,
    code: &str,
) -> Result<bool> {
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
    let output = lang.run(&input, code)?;
    let success = if check {
        let correct_answer = get_output()?;
        let answer = get_answer_from_output(&output);
        if answer == correct_answer {
            println!("{} got {}", success("correct!"), success(answer));
            true
        } else {
            println!(
                "{} expected: {}, got: {}",
                error("wrong!"),
                warning(correct_answer),
                error(answer)
            );
            false
        }
    } else {
        true
    };
    Ok(success)
}

pub fn get_answer_from_output(output: &str) -> String {
    // split into lines and get last line
    output
        .clone()
        .split("\n")
        .filter(|line| !line.is_empty())
        .last()
        .unwrap_or(output)
        .to_string()
}

pub fn create_input_file(path: &str, input: &str) -> Result<()> {
    fs::write(format!("{}/{}", path, "input.txt"), input)?;
    Ok(())
}

use std::io::{BufRead, BufReader};
use std::process::Stdio;

// pub fn run_code(
//     code: &str,
//     file_path: &str,
//     command_program: &str,
//     command_args: &[&str],
//     command_path: &str,
// ) -> Result<String, RunCodeError> {
//     fs::write(file_path, code)?;
//     // Ok(run_command(
//     //     std::process::Command::new(command)
//     //         .current_dir(command_path)
//     //         .args(command_args),
//     // )
//     // .map(|output| output.trim().to_string())?)

//     let mut child = std::process::Command::new(command_program)
//         .current_dir(command_path)
//         .args(command_args)
//         .stdout(Stdio::piped())
//         // .stderr(Stdio::piped())
//         .spawn()?;

//     let child_stdout = child
//         .stdout
//         .take()
//         .expect("internal error, could not take stdout");
//     let child_stderr = child
//         .stderr
//         .take()
//         .expect("internal error, could not take stderr");

//     // let (stdout_tx, stdout_rx) = std::sync::mpsc::channel();
//     let (stderr_tx, stderr_rx) = std::sync::mpsc::channel();

//     // let stdout_thread = thread::spawn(move || {
//     //     let stdout_lines = BufReader::new(child_stdout).lines();
//     //     for line in stdout_lines {
//     //         let line = line.unwrap();
//     //         println!("{}", line);
//     //         stdout_tx.send(line).unwrap();
//     //     }
//     // });
//     let stdout_lines = BufReader::new(child_stdout).lines();
//     let mut stdout = String::new();
//     for line in stdout_lines {
//         let line = line.unwrap();
//         println!("line: {}", line);
//         stdout.push_str(&line);
//     }

//     let stderr_thread = thread::spawn(move || {
//         let stderr_lines = BufReader::new(child_stderr).lines();
//         for line in stderr_lines {
//             let line = line.unwrap();
//             eprintln!("{}", line);
//             stderr_tx.send(line).unwrap();
//         }
//     });

//     let status = child
//         .wait()
//         .expect("Internal error, failed to wait on child");

//     // stdout_thread.join().unwrap();
//     stderr_thread.join().unwrap();

//     // let stdout = stdout_rx.into_iter().collect::<Vec<String>>().join("");
//     let stderr = stderr_rx.into_iter().collect::<Vec<String>>().join("");

//     if status.success() {
//         Ok(stdout.trim().to_string())
//     } else {
//         Err(RunCodeError::CodeError(stderr.trim().to_string()))
//     }
// }

pub fn run_code(
    code: &str,
    file_path: &str,
    command_program: &str,
    command_args: &[&str],
    command_path: &str,
) -> Result<String> {
    fs::write(file_path, code)?;
    let mut child = std::process::Command::new("unbuffer")
        .arg(command_program)
        .args(command_args)
        .current_dir(command_path)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let child_stdout = child
        .stdout
        .take()
        .expect("internal error, could not take stdout");

    let stdout_lines = BufReader::new(child_stdout).lines();
    let mut stdout = String::new();
    for line in stdout_lines {
        let line = line.unwrap();
        println!("{}", line);
        stdout.push_str(format!("{}\n", line).as_str());
    }

    Ok(stdout.trim().to_string())
}
