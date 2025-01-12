#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

const PRIMES: [i32; 25] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
];

fn all_pos_in_grid(
    delta: (i32, i32),
    start_pos: (i32, i32),
    rows: usize,
    cols: usize,
) -> Vec<(i32, i32)> {
    let mut ret = Vec::new();
    ret.push(start_pos);
    for dir in [-1, 1] {
        let delta = (delta.0 * dir, delta.1 * dir);
        let mut pos = (start_pos.0 + delta.0, start_pos.1 + delta.1);
        while pos.0 >= 0 && pos.1 >= 0 && pos.0 < rows as i32 && pos.1 < cols as i32 {
            ret.push(pos);
            pos = (pos.0 + delta.0, pos.1 + delta.1);
        }
    }
    ret
}

fn reduce_delta(mut delta: (i32, i32)) -> (i32, i32) {
    'outer: loop {
        for prime in PRIMES {
            if delta.0 % prime == 0 && delta.1 % prime == 0 {
                delta = (delta.0 / prime, delta.1 / prime);
                continue 'outer;
            }
        }
        return delta;
    }
}

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let grid = Array2D::from_rows(&parser!(lines(any_char+)).parse(&txt).unwrap()).unwrap();
    let antenna_groups: HashMap<char, Vec<(usize, usize)>> =
        grid.enumerate_row_major()
            .fold(HashMap::new(), |mut map, (pos, &c)| {
                if c == '.' {
                    return map;
                }
                map.entry(c)
                    .and_modify(|v| v.push(pos))
                    .or_insert_with(|| vec![pos]);
                map
            });
    let antinode_set: HashSet<(usize, usize)> = antenna_groups
        .into_iter()
        .flat_map(|(_, group)| {
            group
                .iter()
                .tuple_combinations()
                .flat_map(|(p1, p2)| {
                    let p1 = (p1.0 as i32, p1.1 as i32);
                    let p2 = (p2.0 as i32, p2.1 as i32);
                    let delta = (p1.0 - p2.0, p1.1 - p2.1);
                    all_pos_in_grid(reduce_delta(delta), p1, grid.num_rows(), grid.num_columns())
                })
                .filter(|(i, j)| {
                    *i >= 0
                        && *i < grid.num_rows() as i32
                        && *j >= 0
                        && *j < grid.num_columns() as i32
                })
                .map(|(i, j)| (i as usize, j as usize))
                .collect_vec()
        })
        .collect();
    for i in 0..grid.num_rows() {
        for j in 0..grid.num_columns() {
            if antinode_set.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    let answer = antinode_set.len();
    println!("{:?}", answer);
}
