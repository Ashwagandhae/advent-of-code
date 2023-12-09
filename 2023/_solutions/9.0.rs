#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn rec(vec: Vec<i32>) -> i32 {
    println!("{:?}", vec);

    if vec.iter().all(|&x| x == 0) {
        return 0;
    }
    let child = vec
        .iter()
        .tuple_windows()
        .map(|(&x, &y)| y - x)
        .collect_vec();
    let final_diff = rec(child.clone());
    return vec.last().unwrap() + final_diff;
}

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let answer = parser!(lines(repeat_sep(i32, " "))).parse(&txt).unwrap();
    let answer = answer.into_iter().map(|x| rec(x)).sum::<i32>();
    println!("{:?}", answer);
}
