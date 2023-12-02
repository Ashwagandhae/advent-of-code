// import file libraries
use std::fs::File;
use std::io::BufRead;
// bufreader
use std::io::BufReader;

struct Double(usize, usize);
fn main() {
    println!(
        "{}",
        BufReader::new(File::open("../data/4.txt").unwrap())
            .lines()
            .filter(|line| line
                .as_ref()
                .unwrap()
                .split(',')
                .map(|part| {
                    match part
                        .split('-')
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()[..]
                    {
                        [a, b] => Double(a, b),
                        _ => panic!("invalid input"),
                    }
                })
                .collect::<Vec<Double>>()
                .chunks(2)
                .map(|chunk| match chunk {
                    [Double(start_1, end_1), Double(start_2, end_2)] => {
                        (start_1 <= start_2 && end_1 >= end_2)
                            || (start_2 <= start_1 && end_2 >= end_1)
                    }
                    _ => panic!("invalid input"),
                })
                .next()
                .unwrap())
            .count()
    );
}
