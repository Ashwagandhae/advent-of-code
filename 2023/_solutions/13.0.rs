#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

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
            let row = (0..grid.num_rows() - 1)
                .find(|&row| {
                    let mut i = row;
                    let mut j = row + 1;
                    loop {
                        match (rows.get(i), rows.get(j)) {
                            (Some(a), Some(b)) => {
                                if a != b {
                                    return false;
                                }
                            }
                            (None, _) | (_, None) => return true,
                        }
                        i -= 1;
                        j += 1;
                    }
                })
                .map(|x| x + 1)
                .unwrap_or(0);
            let cols = grid.as_columns();
            let col = (0..grid.num_columns() - 1)
                .find(|&col| {
                    let mut i = col;
                    let mut j = col + 1;
                    loop {
                        match (cols.get(i), cols.get(j)) {
                            (Some(a), Some(b)) => {
                                if a != b {
                                    return false;
                                }
                            }
                            (None, _) | (_, None) => return true,
                        }
                        i -= 1;
                        j += 1;
                    }
                })
                .map(|x| x + 1)
                .unwrap_or(0);
            col + row * 100
        })
        .sum();
    println!("{:?}", answer);
}
