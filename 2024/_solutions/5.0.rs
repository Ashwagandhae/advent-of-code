#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
fn get_mid(arr: Vec<u32>) -> u32 {
    arr[arr.len() / 2]
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Order {
    Before,
    DontCare,
    After,
}

fn build_ordered_arr(arr: Vec<u32>, rules: &HashSet<(u32, u32)>) -> Vec<u32> {
    fn add_to_arr(arr: &mut Vec<u32>, rules: &HashSet<(u32, u32)>, x: u32) {
        arr.push(x);
        arr.sort_by_key(|&y| {
            if rules.contains(&(y, x)) {
                Order::Before
            } else if rules.contains(&(x, y)) {
                Order::After
            } else {
                Order::DontCare
            }
        })
    }
    let mut res = Vec::new();
    for x in arr {
        add_to_arr(&mut res, rules, x);
    }
    res
}
fn main() {
    let s = read_to_string("./input.txt").unwrap();
    let (txt1, txt2) = s.split_once("\n\n").unwrap();
    let mut rules = parser!(lines(u32 "|" u32)).parse(&txt1).unwrap();
    let orders = parser!(lines(repeat_sep(u32, ","))).parse(&txt2).unwrap();
    rules.sort();

    let mut arr: Vec<u32> = orders.iter().flatten().cloned().collect();
    arr.sort();
    arr.dedup();

    let rules_set: HashSet<_> = rules.into_iter().collect();

    let ordered_arr = build_ordered_arr(arr, &rules_set);

    let ordering_map: HashMap<u32, usize> = ordered_arr
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (*x, i + 1))
        .collect();
    dbg!(&ordering_map);
    let answer: u32 = orders
        .into_iter()
        .filter(|order| {
            for i in 0..(order.len() - 1) {
                for j in (i + 1)..order.len() {
                    if rules_set.contains(&(order[j], order[i])) {
                        return false;
                    }
                }
            }
            true
        })
        .map(get_mid)
        .sum();

    println!("{:?}", answer);
}
