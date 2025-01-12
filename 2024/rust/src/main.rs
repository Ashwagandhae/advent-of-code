#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use cached::proc_macro::cached;
use fraction::Fraction;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn min_tokens(button_a: (u64, u64), button_b: (u64, u64), prize: (u64, u64)) -> Option<u64> {
    dbg!("run");
    let frac = Fraction::from;
    let (a1, a2) = button_a;
    let (b1, b2) = button_b;
    let (r1, r2) = prize;

    // make sure integer solutions exist
    if !(r1 % gcd(a1, b1) == 0 && r2 % gcd(a2, b2) == 0) {
        return None;
    }
    dbg!("integer solution exists");

    let a1 = frac(a1);
    let a2 = frac(a2);
    let b1 = frac(b1);
    let b2 = frac(b2);
    let r1 = frac(r1);
    let r2 = frac(r2);
    // check if paralell
    if a1 * b2 - a2 * b1 == frac(0) {
        dbg!("it is paralell");
        // infinite solutions
        return if r1 * b2 == r2 * b1 {
            let b_intercept = r1 / b1;
            let b = b_intercept.ceil();
            let a = (r1 - b1 * b) / a1;
            (a * 3 + b).try_into().ok()
        } else {
            None
        };
    }
    let b = (r2 - (a2 / a1) * r1) / (frac(1) + (a2 * b1) / a1);
    let a = (r1 - b1 * b) / a1;
    if b.fract() == frac(0) && a.fract() == frac(0) {
        (a * 3 + b).try_into().ok()
    } else {
        None
    }
}

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let answer: u64 = parser!(sections(
     line("Button A: X+" u64 ", Y+" u64)
     line("Button B: X+" u64 ", Y+" u64)
     line("Prize: X=" u64 ", Y=" u64),
    ))
    .parse(&txt)
    .unwrap()
    .into_iter()
    // .map(|(a, b, prize)| (a, b, (prize.0 + 10000000000000, prize.1 + 10000000000000)))
    .filter_map(|(button_a, button_b, prize)| min_tokens(button_a, button_b, prize))
    .sum();
    println!("{:?}", answer);
}
