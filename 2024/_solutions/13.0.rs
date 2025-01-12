#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn min_tokens(button_a: (u64, u64), button_b: (u64, u64), prize: (u64, u64)) -> Option<u64> {
    (0..)
        .into_iter()
        .take_while(|a_presses| {
            button_a.0 * a_presses < prize.0 && button_a.1 * a_presses < prize.1
        })
        .filter_map(|a_presses| {
            let remaining_prize = (
                prize.0 - button_a.0 * a_presses,
                prize.1 - button_a.1 * a_presses,
            );
            if remaining_prize.0 % button_b.0 == 0 && remaining_prize.1 % button_b.1 == 0 {
                let b_presses = remaining_prize.0 / button_b.0;
                if b_presses == remaining_prize.1 / button_b.1 {
                    Some((a_presses, b_presses))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .map(|(a, b)| a * 3 + b)
        .min()
}

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let answer: u64 = parser!(sections(
     line("Button A: X+" u64 ", Y+" u64)
     line("Button B: X+" u64 ", Y+" u64)
     line("Prize: X=" u64 ", Y=" u64),
    ))
    .parse(&txt)
    .unwrap()
    .into_iter()
    .filter_map(|(button_a, button_b, prize)| min_tokens(button_a, button_b, prize))
    .sum();
    println!("{:?}", answer);
}
