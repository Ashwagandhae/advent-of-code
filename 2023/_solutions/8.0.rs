#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let (instructs, body) = txt.split_once("\n\n").unwrap();
    let answer = parser!(lines(string(alpha*) " = (" string(alpha*) ", " string(alpha*) ")"))
        .parse(&body)
        .unwrap();
    let map: HashMap<String, (String, String)> = answer
        .iter()
        .map(|(a, b, c)| (a.clone(), (b.clone(), c.clone())))
        .collect();
    let mut i = 0;
    let mut key = "AAA".to_string();
    loop {
        if key == "ZZZ" {
            break;
        }
        let index = i % instructs.len();
        let dir = instructs.chars().nth(index).unwrap();
        let next = map[&key].clone();
        key = if dir == 'R' {
            next.1.clone()
        } else {
            next.0.clone()
        };
        i += 1;
    }
    println!("{:?}", i);
}
