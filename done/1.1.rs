// import file libraries
use std::fs::File;
use std::io::Read;

fn main() {
    // open 1.txt
    let mut file = File::open("../data/1.txt").unwrap();
    // read file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut max = 0;
    for elf in contents.split("\n\n") {
        let mut current = 0;
        for cal in elf.lines() {
            current += cal.parse::<i32>().unwrap();
        }
        if current > max {
            max = current;
        }
    }
    println!("{}", max);
}
