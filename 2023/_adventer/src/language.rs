use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum)]
pub enum Language {
    #[clap(alias = "rs")]
    Rust,
    #[clap(alias = "py")]
    Python,
}

impl Language {
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension {
            "rs" => Some(Self::Rust),
            "py" => Some(Self::Python),
            _ => None,
        }
    }
    pub fn to_string(&self) -> &str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
        }
    }
    pub fn extension(&self) -> &str {
        match self {
            Self::Rust => "rs",
            Self::Python => "py",
        }
    }

    pub fn run(&self, input: &str, code: &str) -> Result<String> {
        match self {
            Self::Rust => run_rust(input, code),
            Self::Python => run_python(input, code),
        }
    }

    pub fn work_code_path(&self) -> String {
        match self {
            Self::Rust => "./rust/src/main.rs".to_string(),
            Self::Python => "./python/main.py".to_string(),
        }
    }

    pub fn starter_file(&self) -> (&str, (u32, u32)) {
        match self {
            Self::Rust => (
                r#"#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let answer = parser!(lines(i32)).parse(&txt).unwrap();
    println!("{:?}", answer);
}"#,
                (9, 36),
            ),
            Self::Python => (
                r#"import math, random, time
import numpy as np
s = ""
with open("./input.txt", "r") as f:
    s = f.read()
answer = 0
print(answer)"#,
                (6, 11),
            ),
        }
    }
}

use crate::run::{create_input_file, run_code};
use anyhow::Result;
fn run_rust(input: &str, code: &str) -> Result<String> {
    create_input_file("./rust", &input)?;
    Ok(run_code(
        code,
        "./rust/src/main.rs",
        "cargo",
        &["run", "--quiet", "--release"],
        "./rust",
    )?)
}

fn run_python(input: &str, code: &str) -> Result<String> {
    create_input_file("./python", &input)?;
    Ok(run_code(
        code,
        "./python/main.py",
        "python3",
        &["main.py"],
        "./python",
    )?)
}
