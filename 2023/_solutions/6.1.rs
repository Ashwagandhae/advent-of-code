#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let answer = parser!("Time:" " "+ repeat_sep(string(digit+), " "+) "\n" "Distance:" " "+ repeat_sep(string(digit+), " "+) ).parse(&txt).unwrap();
    let (_, times, _, dists) = answer;
    let time = times.join("").parse::<usize>().unwrap();
    let best_dist = dists.join("").parse::<usize>().unwrap();
    let answer = (0..time)
        .map(|speed| {
            let dist = (time - speed) * speed;
            if dist > best_dist {
                1
            } else {
                0
            }
        })
        .sum::<usize>();
    println!("{:?}", answer);
}
