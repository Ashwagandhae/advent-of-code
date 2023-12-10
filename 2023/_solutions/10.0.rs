#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum P {
    UpDown,
    LeftRight,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    Start,
}

fn surrounding((x, y): (usize, usize), p: P) -> Vec<(usize, usize)> {
    use P::*;
    match p {
        Start => vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)],
        UpDown => vec![(x, y + 1), (x, y - 1)],
        LeftRight => vec![(x + 1, y), (x - 1, y)],
        UpRight => vec![(x + 1, y), (x, y - 1)],
        UpLeft => vec![(x - 1, y), (x, y - 1)],
        DownRight => vec![(x + 1, y), (x, y + 1)],
        DownLeft => vec![(x - 1, y), (x, y + 1)],
    }
}

fn connects(pos: (usize, usize), grid: &Vec<Vec<Option<P>>>) -> Vec<(usize, usize)> {
    let p = grid[pos.1][pos.0].unwrap();
    surrounding(pos, p)
        .into_iter()
        .filter_map(|(x, y)| {
            grid.get(y)
                .map(|row| row.get(x).copied())
                .flatten()
                .flatten()
                .map(|p| (p, (x, y)))
        })
        .filter(|(p, other_pos)| surrounding(*other_pos, *p).contains(&pos))
        .map(|(_, other_pos)| other_pos)
        .collect_vec()
}

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let grid = parser!(lines({
        "S" => Some(P::Start),
        "|" => Some(P::UpDown),
        "-" => Some(P::LeftRight),
        "L" => Some(P::UpRight),
        "J" => Some(P::UpLeft),
        "7" => Some(P::DownLeft),
        "F" => Some(P::DownRight),
        "." => None,
    }*))
    .parse(&txt)
    .unwrap();
    let mut start_pos = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == Some(P::Start) {
                start_pos = (x, y);
            }
        }
    }
    let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    visited[start_pos.1][start_pos.0] = true;
    let mut poses = HashSet::new();
    poses.insert(start_pos);
    let mut answer = 0;
    loop {
        for pos in poses.iter() {
            visited[pos.1][pos.0] = true;
        }
        poses = poses
            .into_iter()
            .flat_map(|pos| {
                dbg!(pos);
                dbg!(connects(pos, &grid))
            })
            .filter(|pos| !visited[pos.1][pos.0])
            .collect();

        answer += 1;

        if poses.len() == 1 {
            break;
        }
    }
    println!("{:?}", answer);
}
