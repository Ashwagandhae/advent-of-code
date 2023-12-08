#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

const PRIMES: [usize; 100] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541,
];

pub fn least_common_multiple(numbers: Vec<usize>) -> usize {
    let mut factors = HashMap::new();
    for number in numbers {
        let mut number = number;
        for prime in PRIMES.iter() {
            if number == 1 {
                break;
            }
            let mut count = 0;
            while number % prime == 0 {
                number /= prime;
                count += 1;
            }
            if let Some(old) = factors.get_mut(prime) {
                if *old < count {
                    *old = count;
                }
            } else {
                factors.insert(*prime, count);
            }
        }
    }
    factors
        .iter()
        .map(|(prime, count)| prime.pow(*count as u32))
        .product()
}
fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let (instructs, body) = txt.split_once("\n\n").unwrap();
    let answer = parser!(lines(string(alnum*) " = (" string(alnum*) ", " string(alnum*) ")"))
        .parse(&body)
        .unwrap();
    let map: HashMap<String, (String, String)> = answer
        .iter()
        .map(|(a, b, c)| (a.clone(), (b.clone(), c.clone())))
        .collect();
    let keys = map
        .clone()
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|a| a.clone())
        .collect_vec();

    let distances = keys
        .iter()
        .map(|key| {
            let mut i = 0;

            let mut key = key.clone();
            loop {
                if key.ends_with("Z") {
                    break;
                }
                let index = i % instructs.len();
                let dir = instructs.chars().nth(index).unwrap();
                let next = map[&key].clone();
                key = if dir == 'R' {
                    next.1.clone()
                } else {
                    next.0.clone()
                };
                i += 1;
            }
            i
        })
        .collect_vec();
    let a = least_common_multiple(distances);
    // loop {
    //     if keys.iter().all(|x| x.ends_with("Z")) {
    //         break;
    //     }
    //     let index = i % instructs.len();
    //     let dir = instructs.chars().nth(index).unwrap();
    //     // for key in &mut keys {
    //     //     let next = map[&key.clone()].clone();
    //     //     **key = if dir == 'R' {
    //     //         next.1.clone()
    //     //     } else {
    //     //         next.0.clone()
    //     //     };
    //     //     i += 1;
    //     // }
    //     for i in 0..keys.len() {
    //         let key = &mut keys[i];
    //         let next = map[&key.clone()].clone();
    //         keys[i] = if dir == 'R' {
    //             next.1.clone()
    //         } else {
    //             next.0.clone()
    //         };
    //     }
    //     i += 1;
    //     println!("{}", i);
    // }
    println!("{:?}", a);
}
