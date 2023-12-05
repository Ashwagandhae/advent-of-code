use std::fs::{read_to_string, File};

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let cards: Vec<usize> = txt
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
                    score += 1;
                }
            }
            score
        })
        .collect();
    let mut card_counts = vec![1; cards.len()];
    for i in 0..cards.len() {
        println!("{} {:?}", i, card_counts);
        for j in 0..cards[i] {
            card_counts[i + j + 1] += 1 * card_counts[i];
        }
    }

    let answer: usize = card_counts.iter().sum();

    println!("{}", answer);
}
