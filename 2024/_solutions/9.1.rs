#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
use std::thread::current;

#[derive(Debug, Clone, Copy)]
struct File {
    id: u32,
    size: u8,
    empty_size: u8,
}

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let nums: Vec<usize> = parser!(char_of("0123456789")+).parse(&txt).unwrap();
    let mut files: Vec<File> = nums
        .chunks(2)
        .enumerate()
        .map(|(id, chunk)| {
            let (size, empty_size) = match chunk {
                [size, empty_size] => (size, empty_size),
                [size] => (size, &0),
                _ => panic!("what the sigma"),
            };
            File {
                id: id as u32,
                size: *size as u8,
                empty_size: *empty_size as u8,
            }
        })
        .collect();

    let max_id = files.iter().map(|f| f.id).max().unwrap();
    for id in (0..=max_id).rev() {
        let (i, file) = files
            .iter()
            .cloned()
            .enumerate()
            .find(|(_, f)| f.id == id)
            .unwrap();
        for j in 0..i {
            if files[j].empty_size >= file.size {
                files.remove(i);
                files
                    .get_mut(i - 1)
                    .map(|f| f.empty_size += file.size + file.empty_size);
                let old_empty_size = files[j].empty_size;
                files[j].empty_size = 0;
                files.insert(
                    j + 1,
                    File {
                        id: file.id,
                        size: file.size,
                        empty_size: old_empty_size - file.size,
                    },
                );
                break;
            }
        }
    }
    let mut answer: u64 = 0;
    let mut index = 0;
    for file in files {
        for _ in 0..file.size {
            answer += index * file.id as u64;
            index += 1;
        }
        index += file.empty_size as u64;
    }
    println!("{:?}", answer);
}
