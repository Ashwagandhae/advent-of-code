use std::fs::{read_to_string, File};

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let answer: usize = txt
        .split("\n")
        .map(|line| {
            let (_, line) = line.split_once(": ").unwrap();
            let (winning, real) = line.split_once(" | ").unwrap();
            let winning = winning
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let real = real
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let mut score = 0;
            for r in real {
                if winning.contains(&r) {
                    if score == 0 {
                        score = 1;
                    } else {
                        score *= 2;
                    }
                }
            }
            score
        })
        .sum();
    println!("{}", answer);
}
