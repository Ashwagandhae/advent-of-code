#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
use std::ops::Range;

fn normalize_range(range: Range<i64>) -> Range<i64> {
    if range.end < range.start {
        range.start..range.start
    } else {
        range
    }
}
fn is_valid_range(range: Range<i64>) -> bool {
    range.end > range.start
}
fn convert_range(
    range: Range<i64>,
    dest: i64,
    source: i64,
    len: i64,
) -> (Range<i64>, Range<i64>, Range<i64>) {
    let delta = dest - source;
    let overlap_start = range.start.max(source);
    let overlap_end = range.end.min(source + len);
    let left = normalize_range(range.start..overlap_start);
    let right = normalize_range(overlap_end..range.end);
    let overlap = normalize_range(overlap_start + delta..overlap_end + delta);
    (left, overlap, right)
}
fn combine_range(smaller_start: Range<i64>, larger_start: Range<i64>) -> Option<Range<i64>> {
    if smaller_start.end >= larger_start.start {
        Some(smaller_start.start..larger_start.end.max(smaller_start.end))
    } else {
        None
    }
}

fn convert_range_rec(map: &Vec<(i64, i64, i64)>, range: Range<i64>) -> Vec<Range<i64>> {
    if !is_valid_range(range.clone()) {
        return Vec::new();
    }
    let mut ranges = Vec::new();
    for (dest, source, len) in map {
        let (left, overlap, right) = convert_range(range.clone(), *dest, *source, *len);
        if is_valid_range(overlap.clone()) {
            ranges.push(overlap);
            ranges.append(&mut convert_range_rec(map, left));
            ranges.append(&mut convert_range_rec(map, right));
            return ranges;
        }
    }
    println!("rec: {:?} -> {:?}", range, ranges);
    return vec![range];
}
fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let (seeds, other) = txt.split_once("\n\n").unwrap();
    let seeds = parser!("seeds: " repeat_sep(i64 " " i64, " "))
        .parse(&seeds)
        .unwrap()
        .into_iter()
        .map(|(a, b)| a..b + a)
        .collect_vec();
    let other = parser!(repeat_sep(string(char_of("abcdefghijklmnopqrstuvwxyz-")+) " map:\n" repeat_sep((i64 " " i64 " " i64), "\n"), "\n\n"))
        .parse(&other)
        .unwrap();
    let mut min = i64::MAX;
    for seed in seeds.iter() {
        println!("seed {:?}", seed);
        let mut ranges = vec![seed.clone()];

        for (_, map) in &other {
            println!("{:?}", ranges);
            let mut new_ranges = ranges
                .iter()
                .flat_map(|range| convert_range_rec(map, range.clone()))
                .collect_vec();

            new_ranges.sort_by_key(|r| r.start);
            println!("unjoined: {:?}", new_ranges);
            new_ranges.dedup();
            let mut new_ranges2 = Vec::new();
            let mut current_add = new_ranges[0].clone();
            for i in 1..new_ranges.len() {
                if new_ranges[i].start == new_ranges[i].end {
                    continue;
                }
                let next = new_ranges[i].clone();
                if let Some(combined) = combine_range(current_add.clone(), next.clone()) {
                    current_add = combined;
                } else {
                    new_ranges2.push(current_add);
                    current_add = next;
                }
            }
            new_ranges2.push(current_add);
            ranges = new_ranges2;
        }
        if ranges[0].start < min {
            min = ranges[0].start;
        }
    }
    println!("{:?}", min);
}
