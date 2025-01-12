#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

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
                    let diff = (p1.0 - p2.0, p1.1 - p2.1);
                    [
                        (p1.0 + diff.0, p1.1 + diff.1),
                        (p2.0 - diff.0, p2.1 - diff.1),
                    ]
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
    // for i in 0..grid.num_rows() {
    //     println!();
    //     for j in 0..grid.num_columns() {
    //         if antinode_set.contains(&('0', (i, j))) || antinode_set.contains(&('A', (i, j))) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    // }
    let answer = antinode_set.len();
    println!("{:?}", answer);
}
