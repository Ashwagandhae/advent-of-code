#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
use std::thread::current;

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let nums: Vec<usize> = parser!(char_of("0123456789")+).parse(&txt).unwrap();
    let mut files: Vec<Option<u64>> = nums
        .chunks(2)
        .enumerate()
        .flat_map(|(id, chunk)| {
            let (size, empty_size) = match chunk {
                [size, empty_size] => (size, empty_size),
                [size] => (size, &0),
                _ => panic!("what the sigma"),
            };
            [vec![Some(id as u64); *size], vec![None; *empty_size]].concat()
        })
        .collect();

    let (mut current_insert, _) = files
        .iter()
        .cloned()
        .find_position(Option::is_none)
        .unwrap();
    let (mut current_remove, _) = files
        .iter()
        .enumerate()
        .rev()
        .find(|(_, x)| x.is_some())
        .unwrap();
    'outer: loop {
        files.swap(dbg!(current_insert), dbg!(current_remove));
        while files[current_insert].is_some() {
            if current_insert >= current_remove {
                break 'outer;
            }
            current_insert += 1;
        }
        while files[current_remove].is_none() {
            if current_insert >= current_remove {
                break 'outer;
            }
            current_remove -= 1;
        }
    }
    let answer: u64 = files
        .into_iter()
        .take_while(Option::is_some)
        .map(Option::unwrap)
        .enumerate()
        .map(|(i, id)| i as u64 * id as u64)
        .sum();
    println!("{:?}", answer);
}
