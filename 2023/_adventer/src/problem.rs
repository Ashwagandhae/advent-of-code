use anyhow::{Context, Result};
use clap;
use serde::{Deserialize, Serialize};

use crate::styles::accent;

#[derive(Debug, Clone, Copy, clap::ValueEnum, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProblemPart {
    #[clap(alias = "0")]
    Zero,
    #[clap(alias = "1")]
    One,
}
impl ProblemPart {
    pub fn advent_display(&self) -> String {
        match self {
            ProblemPart::Zero => "1",
            ProblemPart::One => "2",
        }
        .to_string()
    }
}

impl std::fmt::Display for ProblemPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProblemPart::Zero => write!(f, "0"),
            ProblemPart::One => write!(f, "1"),
        }
    }
}

pub trait ListParseable: Eq + Copy {
    fn parse_single(name_list: &str) -> Result<Vec<Self>>;
    fn next(&self) -> Self;
    fn first() -> Self;
    fn last() -> Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProblemDay(pub u8);

impl ProblemDay {
    pub fn parse(name: &str) -> Result<Self> {
        Ok(Self(
            name.parse()
                .context(format!("invalid problem day: {}", name))?,
        ))
    }
}

impl ListParseable for ProblemDay {
    fn parse_single(name_list: &str) -> Result<Vec<ProblemDay>> {
        Ok(vec![ProblemDay::parse(name_list)?])
    }
    fn next(&self) -> Self {
        Self(self.0 + 1)
    }
    fn first() -> Self {
        Self(1)
    }
    fn last() -> Self {
        Self(25)
    }
}

impl std::fmt::Display for ProblemDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Problem {
    pub day: ProblemDay,
    pub part: ProblemPart,
}

impl Problem {
    pub fn parse(name: &str) -> Result<Self> {
        let (day, part) = name
            .split_once('.')
            .context(format!("invalid problem name: {}", name))?;
        Ok(Self {
            day: ProblemDay::parse(day)?,
            part: match part {
                "0" => ProblemPart::Zero,
                "1" => ProblemPart::One,
                _ => anyhow::bail!("invalid problem part: {}", part),
            },
        })
    }
}

impl From<Problem> for String {
    fn from(val: Problem) -> Self {
        val.to_string()
    }
}

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        accent(self.to_string()).fmt(f)
    }
}

impl ListParseable for Problem {
    fn parse_single(name_list: &str) -> Result<Vec<Problem>> {
        if name_list.contains('.') {
            Ok(vec![Problem::parse(name_list)?])
        } else {
            let day = ProblemDay::parse(name_list)?;
            Ok(vec![
                Problem {
                    day,
                    part: ProblemPart::Zero,
                },
                Problem {
                    day,
                    part: ProblemPart::One,
                },
            ])
        }
    }
    fn next(&self) -> Self {
        match self.part {
            ProblemPart::Zero => Self {
                day: self.day,
                part: ProblemPart::One,
            },
            ProblemPart::One => Self {
                day: self.day.next(),
                part: ProblemPart::Zero,
            },
        }
    }
    fn first() -> Self {
        Self {
            day: ProblemDay::first(),
            part: ProblemPart::Zero,
        }
    }
    fn last() -> Self {
        Self {
            day: ProblemDay::last(),
            part: ProblemPart::One,
        }
    }
}

pub fn parse_problem(s: &str) -> Result<Problem> {
    Problem::parse(s)
}

pub fn parse_list<T: ListParseable>(item_list: &str) -> Result<Vec<T>> {
    if item_list.contains(',') {
        Ok(item_list
            .split(',')
            .map(|s| T::parse_single(s))
            .collect::<Result<Vec<Vec<T>>>>()?
            .into_iter()
            .flatten()
            .collect())
    } else if let Some((start, end)) = item_list.split_once("..") {
        // let (start, end) = item_list.split_once("..")?;
        let start = match start {
            "" => T::first(),
            _ => *T::parse_single(start)?.first().unwrap(),
        };
        let end = match end {
            "" => T::last(),
            _ => *T::parse_single(end)?.last().unwrap(),
        };
        let mut list = vec![];
        let mut current = start;
        while current != end {
            list.push(current);
            current = current.next();
        }
        list.push(end);
        Ok(list)
    } else {
        T::parse_single(item_list)
    }
}
