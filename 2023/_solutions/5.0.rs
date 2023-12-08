#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let (seeds, other) = txt.split_once("\n\n").unwrap();
    let mut seeds = parser!("seeds: " repeat_sep(usize, " "))
        .parse(&seeds)
        .unwrap();
    let other = parser!(repeat_sep(string(char_of("abcdefghijklmnopqrstuvwxyz-")+) " map:\n" repeat_sep((usize " " usize " " usize), "\n"), "\n\n"))
        .parse(&other)
        .unwrap();
    for (_, map) in other {
        println!("{:?}", seeds);
        for seed in seeds.iter_mut() {
            for (dest, source, len) in &map {
                if (*source..*source + *len).contains(seed) {
                    let source_diff = *seed - source;
                    *seed = dest + source_diff;
                    break;
                }
            }
        }
    }
    println!("{:?}", seeds.iter().min().unwrap());
}
