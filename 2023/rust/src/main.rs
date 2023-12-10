#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
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

fn surrounding((y, x): (usize, usize), p: P) -> Vec<(usize, usize)> {
    use P::*;
    let ret = match p {
        Start => vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)],
        UpDown => vec![(x, y + 1), (x, y - 1)],
        LeftRight => vec![(x + 1, y), (x - 1, y)],
        UpRight => vec![(x + 1, y), (x, y - 1)],
        UpLeft => vec![(x - 1, y), (x, y - 1)],
        DownRight => vec![(x + 1, y), (x, y + 1)],
        DownLeft => vec![(x - 1, y), (x, y + 1)],
    };
    ret.into_iter().map(|(x, y)| (y, x)).collect()
}

fn connects(pos: (usize, usize), grid: &Array2D<Option<P>>) -> Vec<(usize, usize)> {
    let p = grid[pos].unwrap();
    surrounding(pos, p)
        .into_iter()
        .filter_map(|(y, x)| grid.get(y, x).copied().flatten().map(|p| (p, (y, x))))
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
    let grid = Array2D::from_rows(&grid).unwrap();
    let start_pos = grid
        .enumerate_row_major()
        .find(|(_, p)| matches! {p, Some(P::Start)})
        .unwrap()
        .0;
    let mut visited = Array2D::filled_with(false, grid.num_rows(), grid.num_columns());
    visited[start_pos] = true;
    let mut poses = HashSet::new();
    poses.insert(start_pos);
    let mut all_pipe_poses = HashSet::new();
    all_pipe_poses.insert(start_pos);
    loop {
        for pos in poses.iter() {
            visited[*pos] = true;
        }
        poses = poses
            .into_iter()
            .flat_map(|pos| connects(pos, &grid))
            .filter(|pos| !visited[*pos])
            .collect();

        for pos in poses.iter() {
            all_pipe_poses.insert(*pos);
        }

        if poses.len() == 1 {
            break;
        }
    }
    for row in visited.rows_iter() {
        for cell in row {
            print!("{}", if *cell { "X" } else { " " });
        }
        println!();
    }
    let mut visited = Array2D::filled_with(false, grid.num_rows() + 1, grid.num_columns() + 1);
    let mut poses = HashSet::new();
    for y in 0..=grid.num_rows() {
        for pos in [(y, 0), (y, grid.num_columns())].iter() {
            if !(all_pipe_poses.contains(pos)) {
                poses.insert(*pos);
            }
        }
    }
    for x in 0..=grid.num_columns() {
        for pos in [(0, x), (grid.num_rows(), x)].iter() {
            if !(all_pipe_poses.contains(pos)) {
                poses.insert(*pos);
            }
        }
    }
    loop {
        for &pos in poses.iter() {
            visited[pos] = true;
        }
        // pretty print visited
        for row in visited.rows_iter() {
            for cell in row {
                print!("{}", if *cell { "X" } else { " " });
            }
            println!();
        }

        poses = poses
            .into_iter()
            .flat_map(|(y, x)| {
                [
                    ((x - 1, y - 1), (x, y - 1), (x, y - 1)),
                    ((x, y - 1), (x, y), (x + 1, y)),
                    ((x, y), (x - 1, y), (x, y + 1)),
                    ((x - 1, y), (x - 1, y - 1), (x - 1, y)),
                ]
                .into_iter()
                .map(|((x1, y1), (x2, y2), (x3, y3))| ((y1, x1), (y2, x2), (y3, x3)))
                .filter(|(p_1, p_2, _)| {
                    if all_pipe_poses.contains(p_1) && all_pipe_poses.contains(p_2) {
                        !connects(*p_1, &grid).contains(p_2)
                    } else {
                        true
                    }
                })
                .map(|(_, _, pos)| pos)
            })
            .filter(|pos| grid.get(pos.0, pos.1).copied().is_some())
            .filter(|&pos| !visited[pos])
            .collect();

        if poses.len() == 0 {
            break;
        }
    }

    let outside_area = (0..grid.num_rows())
        .cartesian_product(0..grid.num_columns())
        .filter(|pos| {
            [(0, 0), (0, 1), (1, 0), (1, 1)]
                .iter()
                .map(|(y, x)| (y + pos.0, x + pos.1))
                .all(|pos| visited[pos])
        })
        .count();
    let answer = grid.num_elements() - (dbg!(outside_area) + dbg!(all_pipe_poses.len()));
    println!("{:?}", answer);
}
