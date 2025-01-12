#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::identity;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
enum Cell {
    Empty(bool),
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_pos(self, (i, j): (usize, usize)) -> Option<(usize, usize)> {
        let (di, dj): (fn(usize) -> Option<usize>, fn(usize) -> Option<usize>) = match self {
            Direction::Up => (|x| x.checked_sub(1), Some),
            Direction::Down => (|x| x.checked_add(1), Some),
            Direction::Left => (Some, |x| x.checked_sub(1)),
            Direction::Right => (Some, |x| x.checked_add(1)),
        };
        Some((di(i)?, dj(j)?))
    }

    fn rotate_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn does_loop(
    grid: &Array2D<Cell>,
    mut pos: (usize, usize),
    mut dir: Direction,
    new_wall: (usize, usize),
) -> bool {
    let mut visited: HashSet<((usize, usize), Direction)> = HashSet::new();
    while let Some((next_pos, next)) = dir
        .move_pos(pos)
        .and_then(|p @ (r, c)| Some((p, grid.get(r, c)?)))
    {
        if visited.contains(&(pos, dir)) {
            return true;
        }
        visited.insert((pos, dir));
        (pos, dir) = match next {
            Cell::Wall => (pos, dir.rotate_right()),
            Cell::Empty(_) if next_pos == new_wall => (pos, dir.rotate_right()),
            Cell::Empty(_) => (next_pos, dir),
        }
    }
    false
}

fn count_loopers(grid: &Array2D<Cell>, pos: (usize, usize), dir: Direction) -> Vec<(usize, usize)> {
    fn count_loopers_rec(
        grid: &Array2D<Cell>,
        pos: (usize, usize),
        dir: Direction,
        memo: &mut HashMap<((usize, usize), Direction), Vec<(usize, usize)>>,
    ) -> Vec<(usize, usize)> {
        if let Some(ret) = memo.get(&(pos, dir)) {
            return ret.clone();
        }
        let Some(next_pos) = dir.move_pos(pos) else {
            return Vec::new();
        };
        let Some(next) = grid.get(next_pos.0, next_pos.1) else {
            return Vec::new();
        };
        let ret = match next {
            Cell::Wall => count_loopers_rec(grid, pos, dir.rotate_right(), memo),
            Cell::Empty(_) => [
                count_loopers_rec(grid, next_pos, dir, memo),
                if does_loop(grid, pos, dir, next_pos) {
                    vec![next_pos]
                } else {
                    Vec::new()
                },
            ]
            .concat(),
        };
        memo.insert((pos, dir), ret.clone());
        ret
    }
    count_loopers_rec(grid, pos, dir, &mut HashMap::new())
}
fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let lines = &parser!(lines({
        '#' => Cell::Wall,
        '.' => Cell::Empty(false),
        '^' => Cell::Empty(true)
    } *))
    .parse(&txt)
    .unwrap();
    let grid = Array2D::from_rows(&lines).unwrap();
    let guard_pos = grid
        .enumerate_row_major()
        .find(|(_, e)| matches!(e, Cell::Empty(true)))
        .map(|(p, _)| p)
        .unwrap();
    let mut poses = count_loopers(&grid, guard_pos, Direction::Up);
    poses.sort();
    poses.dedup();
    // check poses just in case
    poses = poses
        .into_iter()
        .filter(|&wall_pos| does_loop(&grid, guard_pos, Direction::Up, wall_pos))
        .collect_vec();
    let answer = poses.len();
    println!("{:?}", answer);
}
