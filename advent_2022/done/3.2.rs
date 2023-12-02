// import file libraries
use std::fs::File;
use std::io::Read;

fn main() {
    // open 1.txt
    let mut file = File::open("../data/3.txt").unwrap();
    // read file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let alphabet: Vec<char> = String::from("abcdefghijklmnopqrstuvwxyz").chars().collect();
    let mut count = 0;
    let lines = contents.lines().collect::<Vec<&str>>();
    for i in 0..(lines.len() / 3) {
        let pack_1 = lines[i * 3];
        let pack_2 = lines[i * 3 + 1];
        let pack_3 = lines[i * 3 + 2];

        count += pack_1
            .chars()
            .filter(|&x| pack_2.contains(x))
            .filter(|&x| pack_3.contains(x))
            .map(|char_1| {
                // get letter index in alphabet
                let mut index = alphabet
                    .iter()
                    .position(|&r| r == char_1.to_lowercase().next().unwrap())
                    .unwrap()
                    + 1;
                // check if captial
                if char_1.is_uppercase() {
                    // add 26 to index to get captial letter index
                    index += 26;
                }
                println!("{} {}", char_1, index);
                index
            })
            // remove duplicates
            .collect::<std::collections::HashSet<usize>>()
            .iter()
            .next()
            .unwrap();
    }
    println!("{}", count);
}
