#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn concat(num1: i64, num2: i64) -> i64 {
    let mut ret = num1.to_string();
    ret.push_str(&num2.to_string());
    ret.parse::<i64>().unwrap()
}

fn check_equation_possible(answer: i64, nums: &[i64], curr: i64) -> bool {
    // if nums.len() == 0 && answer == 0 {
    //     return true;
    // }
    // return check_equation_possible(answer, nums)
    match nums {
        [] => answer == curr,
        [head, tail @ ..] => {
            check_equation_possible(answer, tail, curr + head)
                || check_equation_possible(answer, tail, curr * head)
                || check_equation_possible(answer, tail, concat(*head, curr))
        }
    }
}

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let answer = parser!(lines(i64 ": " repeat_sep(i64, " ")))
        .parse(&txt)
        .unwrap()
        .into_iter()
        .filter(|(answer, nums)| check_equation_possible(*answer, &nums, 0))
        .map(|(answer, _)| answer)
        .sum::<i64>();
    println!("{:?}", answer);
}
