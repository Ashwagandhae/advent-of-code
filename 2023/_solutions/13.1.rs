#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn find_reflect_line(els: Vec<Vec<bool>>) -> Option<usize> {
    (0..els.len() - 1)
        .find(|&el| {
            let mut i = el;
            let mut j = el + 1;
            let mut smudge_fixed = false;
            let ret = loop {
                match (els.get(i), els.get(j)) {
                    (Some(a), Some(b)) => {
                        let diff = a.iter().zip(b.iter()).filter(|&(a, b)| a != b).count();
                        match diff {
                            0 => (),
                            1 => {
                                if smudge_fixed {
                                    break false;
                                }
                                smudge_fixed = true;
                            }
                            _ => break false,
                        }
                    }
                    (None, _) | (_, None) => break true,
                }
                i -= 1;
                j += 1;
            };
            ret && smudge_fixed
        })
        .map(|x| x + 1)
}
fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let answer = parser!(repeat_sep(repeat_sep({
        "#" => true,
        "." => false,
    }+, "\n"), "\n\n"))
    .parse(&txt)
    .unwrap();
    let answer: usize = answer
        .into_iter()
        .map(|x| Array2D::from_rows(&x).unwrap())
        .map(|grid| {
            let rows = grid.as_rows();
            let row = find_reflect_line(rows).unwrap_or(0);
            let cols = grid.as_columns();
            let col = find_reflect_line(cols).unwrap_or(0);
            col + row * 100
        })
        .sum();
    println!("{:?}", answer);
}
