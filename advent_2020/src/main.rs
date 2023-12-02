// import things to open text file
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

struct Path {
    x: usize,
    y: usize,
}
fn main() {
    // open text file
    let file_path = "./data/4.txt";
    let mut reader = BufReader::new(File::open(file_path).expect("Cannot open file"));
    let mut text: String = String::new();
    reader.read_to_string(&mut text).expect("Cannot read file");

    let mut valid_count = 0;
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    // split by double newline
    for line in text.split("\n\n") {
        // split by whitespace
        let fields: Vec<&str> = line
            .split_whitespace()
            .map(|s| s.split(":").next().unwrap())
            .collect();
        let mut valid = true;
        for field in required_fields.iter() {
            if !fields.contains(field) {
                valid = false;
                break;
            }
        }
        if valid {
            valid_count += 1;
        }
    }
    println!("{}", valid_count);
}
