use anyhow::Result;

use crate::resource::{get_example_input_output, get_input};
use crate::state::get_state;
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
    let (run_example, run_input) = match target {
        RunTarget::Example => (true, false),
        RunTarget::Input => (false, true),
        RunTarget::All | RunTarget::AllConditional => (true, true),
    };
    let mut success = true;
    if run_example {
        println!("running {} on example...", name.to_string());
        let (input, output) = get_example_input_output(name)?;
        success = run_and_check(&lang, Some(&output), &input, code)?;
    }
    let skip_run_input = matches!(target, RunTarget::AllConditional) && !success;
    if run_input {
        if skip_run_input {
            println!("skipping {} on input...", name.to_string());
        } else {
            println!("running {} on input...", name.to_string());
            let input = get_input(name.day)?;
            run_and_check(&lang, None, &input, code)?;
        }
    }
    Ok(())
}

pub fn run_and_check(
    lang: &Language,
    correct_answer: Option<&str>,
    input: &str,
    code: &str,
) -> Result<bool> {
    let output = lang.run(input, code)?;
    let success = if let Some(correct_answer) = correct_answer {
        let answer = get_answer_from_output(&output);
        if answer == correct_answer {
            println!("correct! got {}", answer);
            true
        } else {
            println!("incorrect! expected: {}, got: {}", correct_answer, answer);
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
