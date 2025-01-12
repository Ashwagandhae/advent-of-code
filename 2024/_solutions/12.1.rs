#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Side {
    Top,
    Left,
}
fn get_sides(pos: (usize, usize)) -> [(usize, usize, Side, bool); 4] {
    [
        (pos.0, pos.1, Side::Top, true),       // top
        (pos.0, pos.1, Side::Left, true),      // left
        (pos.0 + 1, pos.1, Side::Top, false),  // bottom
        (pos.0, pos.1 + 1, Side::Left, false), // right
    ]
}

fn count_area_and_perimeter(
    pos: (usize, usize),
    grid: &Array2D<char>,
    visited: &mut Array2D<bool>,
) -> (usize, usize) {
    fn get_tiles(
        pos: (usize, usize),
        target: char,
        grid: &Array2D<char>,
        visited: &mut Array2D<bool>,
    ) -> Vec<(usize, usize)> {
        if !visited.get(pos.0, pos.1).is_some_and(|v| *v == false) {
            return Vec::new();
        }
        match grid.get(pos.0, pos.1) {
            Some(&c) if c == target => {
                let mut ret = vec![pos];
                visited[pos] = true;

                ret.extend(
                    [(0, 1), (0, -1), (1, 0), (-1, 0)]
                        .into_iter()
                        .filter_map(|delta| {
                            Some((
                                pos.0.checked_add_signed(delta.0)?,
                                pos.1.checked_add_signed(delta.1)?,
                            ))
                        })
                        .flat_map(|new_pos| get_tiles(new_pos, target, grid, visited)),
                );
                ret
            }
            _ => Vec::new(),
        }
    }
    let tiles = get_tiles(pos, grid[pos], grid, visited);
    let area = tiles.len();
    let fences: HashMap<(usize, usize, Side), bool> = tiles.into_iter().flat_map(get_sides).fold(
        HashMap::new(),
        |mut set: HashMap<(usize, usize, Side), bool>, tile| {
            let key = (tile.0, tile.1, tile.2);
            if set.contains_key(&key) {
                set.remove(&key);
            } else {
                set.insert(key, tile.3);
            }
            set
        },
    );
    let mut sides = 0;
    for i in 0..grid.num_rows() + 1 {
        let mut previous = None;
        for j in 0..grid.num_columns() + 1 {
            let tile_orient = fences.get(&(i, j, Side::Top));
            match (previous, tile_orient) {
                (Some(prev), Some(curr)) if prev != curr => sides += 1,
                (None, Some(_)) => sides += 1,
                _ => {}
            }
            previous = tile_orient;
        }
    }
    for j in 0..grid.num_columns() + 1 {
        let mut previous = None;
        for i in 0..grid.num_rows() + 1 {
            let tile_orient = fences.get(&(i, j, Side::Left));
            match (previous, tile_orient) {
                (Some(prev), Some(curr)) if prev != curr => sides += 1,
                (None, Some(_)) => sides += 1,
                _ => {}
            }
            previous = tile_orient;
        }
    }
    (area, sides)
}

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let grid = Array2D::from_rows(&parser!(lines(any_char+)).parse(&txt).unwrap()).unwrap();
    let mut visited = Array2D::filled_with(false, grid.num_rows(), grid.num_columns());

    let mut answer = 0;
    for i in 0..grid.num_rows() {
        for j in 0..grid.num_columns() {
            let (area, perimeter) = count_area_and_perimeter((i, j), &grid, &mut visited);
            answer += area * perimeter;
            if area != 0 {
                dbg!(area, perimeter);
            }
        }
    }
    println!("{:?}", answer);
}
