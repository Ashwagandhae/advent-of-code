#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

#[derive(PartialEq, Eq, Ord, PartialOrd, Hash)]
// enum HandType {
//     Useless,
//     One,
//     Two,
//     Three,
//     Full,
//     Four,
//     Five,
// }
enum HandType {
    Five,
    Four,
    Full,
    Three,
    Two,
    One,
    Useless,
}

fn get_hand_type(hand: Vec<char>) -> HandType {
    let mut map: HashMap<char, usize> = HashMap::new();
    for c in hand {
        *map.entry(c).or_insert(0) += 1;
    }
    let mut counts: Vec<usize> = map.values().cloned().collect::<Vec<usize>>();
    counts.sort();
    if counts == [5] {
        HandType::Five
    } else if counts == [1, 4] {
        HandType::Four
    } else if counts == [2, 3] {
        HandType::Full
    } else if counts == [1, 1, 3] {
        HandType::Three
    } else if counts == [1, 2, 2] {
        HandType::Two
    } else if counts == [1, 1, 1, 2] {
        HandType::One
    } else {
        HandType::Useless
    }
}

const CARD_ORD: &str = "AKQJT98765432";

pub fn get_card_ord(card: char) -> usize {
    CARD_ORD.find(card).unwrap()
}

use std::cmp::Ordering;

pub fn second_ord(hand_1: Vec<char>, hand_2: Vec<char>) -> std::cmp::Ordering {
    for (card_1, card_2) in hand_1.iter().zip(hand_2.iter()) {
        let ord_1 = get_card_ord(*card_1);
        let ord_2 = get_card_ord(*card_2);
        if ord_1 != ord_2 {
            return ord_1.cmp(&ord_2);
        }
    }
    std::cmp::Ordering::Equal
}
fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let mut answer = parser!(lines(alnum+ " " usize)).parse(&txt).unwrap();

    answer.sort_by(|(hand_1, _), (hand_2, _)| {
        let hand_type_1 = get_hand_type(hand_1.clone());
        let hand_type_2 = get_hand_type(hand_2.clone());
        if hand_type_1 != hand_type_2 {
            return hand_type_1.cmp(&hand_type_2);
        }
        second_ord(hand_1.clone(), hand_2.clone())
    });
    let answer = answer
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1))
        .sum::<usize>();
    println!("{:?}", answer);
}
