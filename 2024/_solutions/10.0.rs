#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
fn get_trailhead_endings(
    (i, j): (usize, usize),
    grid: &Array2D<usize>,
    expected_height: usize,
) -> Vec<(usize, usize)> {
    match grid.get(i, j) {
        Some(&height) if height == expected_height => {
            if height == 9 {
                vec![(i, j)]
            } else {
                [(1, 0), (-1, 0), (0, 1), (0, -1)]
                    .into_iter()
                    .filter_map(|(di, dj)| {
                        Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?))
                    })
                    .flat_map(|new_pos| get_trailhead_endings(new_pos, grid, expected_height + 1))
                    .collect()
            }
        }
        _ => Vec::new(),
    }
}
fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let grid =
        Array2D::from_rows(&parser!(lines(char_of("0123456789")+)).parse(&txt).unwrap()).unwrap();
    let answer: usize = grid
        .enumerate_row_major()
        .map(|(pos, _)| get_trailhead_endings(pos, &grid, 0))
        .map(|ending| ending.into_iter().collect::<HashSet<_>>().len())
        .sum();
    println!("{:?}", answer);
}
