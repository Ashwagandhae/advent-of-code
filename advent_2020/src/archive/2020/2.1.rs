// import things to open text file
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // open text file
    let file_path = "./data/2.txt";
    // loop through lines
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file.txt"));

    let mut valid_count = 0;
    for line in reader.lines() {
        let s = line.unwrap();
        let split = s.split(" ").collect::<Vec<&str>>();
        let min_max = split[0].split("-").collect::<Vec<&str>>();
        let min = min_max[0].parse::<usize>().unwrap();
        let max = min_max[1].parse::<usize>().unwrap();
        let letter = split[1].chars().nth(0).unwrap();
        let password = split[2].to_string();
        // count amount of char in password
        let count = password.matches(letter).count();
        if min <= count && count <= max {
            println!("{} is valid", password);
            valid_count += 1;
        }
    }

    println!("{valid_count}")
}
