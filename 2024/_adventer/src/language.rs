use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum)]
pub enum Language {
    #[clap(alias = "rs")]
    Rust,
    #[clap(alias = "py")]
    Python,
    #[clap(alias = "ts")]
    TypeScript,
}

impl Language {
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension {
            "rs" => Some(Self::Rust),
            "py" => Some(Self::Python),
            "ts" => Some(Self::TypeScript),
            _ => None,
        }
    }
    pub fn to_string(&self) -> &str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            Self::TypeScript => "typescript",
        }
    }
    pub fn extension(&self) -> &str {
        match self {
            Self::Rust => "rs",
            Self::Python => "py",
            Self::TypeScript => "ts",
        }
    }

    pub fn run(&self, input: &str, code: &str, quiet: bool) -> Result<String> {
        match self {
            Self::Rust => {
                create_input_file("./rust", input)?;
                Ok(run_code(
                    code,
                    "./rust/src/main.rs",
                    "cargo",
                    &["run", "--quiet", "--release"],
                    "./rust",
                    quiet,
                )?)
            }
            Self::Python => {
                create_input_file("./python", input)?;
                Ok(run_code(
                    code,
                    "./python/main.py",
                    "python3",
                    &["main.py"],
                    "./python",
                    quiet,
                )?)
            }
            Self::TypeScript => {
                create_input_file("./typescript", input)?;
                Ok(run_code(
                    code,
                    "./typescript/index.ts",
                    "bun",
                    &["run", "index.ts"],
                    "./typescript",
                    quiet,
                )?)
            }
        }
    }

    pub fn work_code_path(&self) -> String {
        match self {
            Self::Rust => "./rust/src/main.rs".to_string(),
            Self::Python => "./python/main.py".to_string(),
            Self::TypeScript => "./typescript/index.ts".to_string(),
        }
    }

    pub fn starter_file(&self) -> (&str, (u32, u32)) {
        match self {
            Self::Rust => (
                r#"#![allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use array2d::Array2D;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn main() {
    let txt = read_to_string("./input.txt").unwrap();
    let answer = parser!(lines(i32)).parse(&txt).unwrap();
    println!("{:?}", answer);
}"#,
                (10, 36),
            ),
            Self::Python => (
                r#"import math, random, time
import numpy as np
s = ""
with open("./input.txt", "r") as f:
    s = f.read()
answer = 0
print(answer)"#,
                (6, 12),
            ),
            Self::TypeScript => (
                r#"let text = await Bun.file("./input.txt").text();
let answer = 0;
console.log(answer);"#,
                (2, 15),
            ),
        }
    }
}

use crate::run::{create_input_file, run_code};
use anyhow::Result;
