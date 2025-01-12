#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn count_stone_expansion(stone: u64, count: u8, memo: &mut HashMap<(u64, u8), u64>) -> u64 {
    if memo.contains_key(&(stone, count)) {
        return memo[&(stone, count)];
    }
    let ret = 'f: {
        if count == 0 {
            break 'f 1;
        }
        let count = count - 1;
        if stone == 0 {
            break 'f count_stone_expansion(1, count, memo);
        }
        let string = stone.to_string();
        if string.len() % 2 == 0 {
            let stone_left = string[..string.len() / 2].parse().unwrap();
            let stone_right = string[string.len() / 2..].parse().unwrap();
            break 'f count_stone_expansion(stone_left, count, memo)
                + count_stone_expansion(stone_right, count, memo);
        }
        count_stone_expansion(stone * 2024, count, memo)
    };
    memo.insert((stone, count), ret);
    ret
}
fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let stones = parser!(repeat_sep(u64, " ")).parse(&txt).unwrap();

    let answer: u64 = stones
        .into_iter()
        .map(|stone| count_stone_expansion(stone, 75, &mut HashMap::new()))
        .sum();
    println!("{:?}", answer);
}
