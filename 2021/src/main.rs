use glob::glob;
use std::process::Command;

fn main() {
    let mut file_names = glob("./src/solutions/*.rs")
        .expect("Failed to read glob pattern")
        .filter(|val| val.is_ok())
        .map(move |val| {
            val.unwrap()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>();
    file_names.sort();
    let last_file = &file_names.last().expect("Missing solution files");
    let run_file = format!("./src/solutions/{}.rs", last_file);
    let mut child = Command::new("rustc")
        .arg(run_file)
        .arg("--out-dir")
        .arg("build")
        .spawn()
        .expect("command failed to start");
    child.wait().expect("command failed to run");

    let mut child = Command::new("./build/".to_owned() + last_file)
        .spawn()
        .expect("command failed to start");
    child.wait().expect("command failed to run");
    println!("done");
}
