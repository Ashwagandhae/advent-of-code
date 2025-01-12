#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
fn update(stones: Vec<u64>) -> Vec<u64> {
    stones
        .into_iter()
        .flat_map(|stone| {
            if stone == 0 {
                return vec![1];
            }
            let string = stone.to_string();
            if string.len() % 2 == 0 {
                return vec![
                    string[..string.len() / 2].parse().unwrap(),
                    string[string.len() / 2..].parse().unwrap(),
                ];
            }
            return vec![stone * 2024];
        })
        .collect()
}
fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let mut stones = parser!(repeat_sep(u64, " ")).parse(&txt).unwrap();
    for _ in 0..75 {
        stones = update(stones);
    }
    let answer = stones.len();
    println!("{:?}", answer);
}
