#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
use std::iter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Empty,
    Filled,
    Unknown,
}
fn rec(states: &[Spring], rows: &[usize]) -> usize {
    match (states.first(), rows.first()) {
        (Some(spring), Some(&row)) => {
            let mut ret = 0;
            if let Spring::Filled | Spring::Unknown = spring {
                let Some(row_springs) = states.get(..row) else { return 0 };
                if row_springs
                    .iter()
                    .all(|&s| s == Spring::Filled || s == Spring::Unknown)
                {
                    ret += match states.get(row) {
                        Some(Spring::Empty | Spring::Unknown) => {
                            rec(&states[row + 1..], &rows[1..])
                        }
                        None if rows.len() == 1 => 1,
                        _ => 0,
                    }
                }
            }
            if let Spring::Empty | Spring::Unknown = spring {
                ret += rec(&states[1..], rows);
            }
            ret
        }
        (None, Some(_)) => 0,
        (Some(_), None) => {
            if states.iter().any(|s| s == &Spring::Filled) {
                0
            } else {
                1
            }
        }
        (None, None) => 1,
    }
}

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let answer = parser!(lines({
        '#' => Spring::Filled,
        '.' => Spring::Empty,
        '?' => Spring::Unknown,
    }* " " repeat_sep(usize, ",")))
    .parse(&txt)
    .unwrap();
    let answer: usize = answer
        .into_iter()
        .map(|(states, rows)| {
            let states = (0..5)
                .map(|_| states.clone())
                .intersperse(vec![Spring::Unknown])
                .flatten()
                .collect_vec();
            let rows = (0..5).map(|_| rows.iter().copied()).flatten().collect_vec();
            (states, rows)
        })
        .map(|(states, rows)| rec(&states, &rows))
        .sum();
    println!("{:?}", answer);
}
