#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let answer = parser!("Time:" " "+ repeat_sep(usize, " "+) "\n" "Distance:" " "+ repeat_sep(usize, " "+) ).parse(&txt).unwrap();
    let (_, times, _, dists) = answer;
    let answer = times
        .into_iter()
        .zip(dists.into_iter())
        .map(|(time, best_dist)| {
            (0..time)
                .map(|speed| {
                    let dist = (time - speed) * speed;
                    if dist > best_dist {
                        1
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .fold(1, |acc, x| acc * x);
    println!("{:?}", answer);
}
