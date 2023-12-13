#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let answer = parser!(lines({
        "." => false,
        "#" => true
    }+))
    .parse(&txt)
    .unwrap();
    let grid = Array2D::from_rows(&answer).unwrap();
    let grid = Array2D::from_columns(
        &grid
            .columns_iter()
            .flat_map(|column| {
                let column: Vec<_> = column.cloned().collect();
                if column.iter().all(|&b| !b) {
                    vec![column.clone(), column.clone()]
                } else {
                    vec![column]
                }
            })
            .collect_vec(),
    )
    .unwrap();
    let grid = Array2D::from_rows(
        &grid
            .rows_iter()
            .flat_map(|row| {
                let row: Vec<_> = row.cloned().collect();
                if row.iter().all(|&b| !b) {
                    vec![row.clone(), row.clone()]
                } else {
                    vec![row]
                }
            })
            .collect_vec(),
    )
    .unwrap();

    // pretty print grid
    for row in grid.rows_iter() {
        for &b in row {
            print!("{}", if b { "#" } else { "." });
        }
        println!();
    }
    let galaxy_poses = grid
        .enumerate_row_major()
        .filter(|(_, &b)| b)
        .map(|(pos, _)| pos)
        .collect_vec();
    println!("{:?}", galaxy_poses.len());
    let mut ret: isize = 0;
    for i in 0..galaxy_poses.len() - 1 {
        let pos = galaxy_poses[i];
        for j in i + 1..galaxy_poses.len() {
            let other_pos = galaxy_poses[j];
            if pos > other_pos {
                continue;
            }
            let (x, y) = pos;
            let (other_x, other_y) = other_pos;
            let dist =
                (x as isize - other_x as isize).abs() + (y as isize - other_y as isize).abs();
            ret += dist;
        }
    }
    println!("{:?}", ret);
}
