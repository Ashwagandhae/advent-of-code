#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

#[derive(Debug, Clone)]
enum Cell {
    Empty(bool),
    Wall,
}
fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let lines = &parser!(lines(any_char *)).parse(&txt).unwrap();
    let mut grid = Array2D::from_rows(
        &lines
            .into_iter()
            .map(|line| {
                line.into_iter()
                    .map(|c| match c {
                        '#' => Cell::Wall,
                        '.' => Cell::Empty(false),
                        '^' => Cell::Empty(true),
                        _ => panic!("gimpossible"),
                    })
                    .collect_vec()
            })
            .collect_vec(),
    )
    .unwrap();
    let mut guard_pos = lines
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter().enumerate().find_map(|(j, c)| match c {
                '^' => Some((i, j)),
                _ => None,
            })
        })
        .unwrap();
    let mut guard_direction: (i32, i32) = (-1, 0);
    loop {
        // println!("new grid:");
        // for row in grid.rows_iter() {
        //     for element in row {
        //         print!(
        //             "{}",
        //             match element {
        //                 Cell::Empty(true) => "X",
        //                 Cell::Empty(false) => ".",
        //                 Cell::Wall => "#",
        //             }
        //         );
        //     }
        //     println!();
        // }
        let next_pos = (
            (guard_pos.0 as i32 + guard_direction.0) as usize,
            (guard_pos.1 as i32 + guard_direction.1) as usize,
        );
        match grid.get_mut(next_pos.0 as usize, next_pos.1 as usize) {
            None => break,
            Some(Cell::Wall) => {
                guard_direction = match guard_direction {
                    (0, -1) => (-1, 0),
                    (1, 0) => (0, -1),
                    (0, 1) => (1, 0),
                    (-1, 0) => (0, 1),
                    _ => panic!("gimpossible"),
                }
            }
            Some(Cell::Empty(filled)) => {
                guard_pos = next_pos;
                *filled = true;
            }
        }
    }
    let answer = grid
        .elements_row_major_iter()
        .filter(|x| matches!(x, Cell::Empty(true)))
        .count();
    println!("{:?}", answer);
}
