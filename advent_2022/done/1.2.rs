// import file libraries
use std::fs::File;
use std::io::Read;

fn main() {
    // open 1.txt
    let mut file = File::open("../data/1.txt").unwrap();
    // read file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut cals: Vec<i32> = Vec::new();
    for elf in contents.split("\n\n") {
        let mut current = 0;
        for cal in elf.lines() {
            current += cal.parse::<i32>().unwrap();
        }
        cals.push(current);
    }
    // sort cals reverse
    cals.sort();
    cals.reverse();
    println!("{}", cals[0] + cals[1] + cals[2]);
}
