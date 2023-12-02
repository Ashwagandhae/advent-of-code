// import file libraries
use std::fs::File;
use std::io::Read;

fn main() {
    // open 1.txt
    let mut file = File::open("../data/4.txt").unwrap();
    // read file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let count = contents
        .lines()
        .filter(|line| {
            // split by comma
            let mut parts = line.split(',');
            // get first part
            let first = parts
                .next()
                .unwrap()
                .split('-')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            // get second part
            let second = parts
                .next()
                .unwrap()
                .split('-')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            (first[0] <= second[0] && first[1] >= second[1])
                || (second[0] <= first[0] && second[1] >= first[1])
        })
        .count();
    println!("{}", count);
}
