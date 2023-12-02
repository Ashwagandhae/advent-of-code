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
    for ruck in contents.lines() {
        let chars = ruck.chars().collect::<Vec<char>>();
        let len = chars.len();
        // split into two halves
        let compart_1 = &chars[0..len / 2];
        let compart_2 = &chars[len / 2..len];

        count += compart_1
            .iter()
            .filter(|&x| compart_2.contains(x))
            .map(|&x| x)
            // remove duplicates
            .collect::<std::collections::HashSet<char>>()
            .into_iter()
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
            .sum::<usize>();
    }
    println!("{}", count);
}
