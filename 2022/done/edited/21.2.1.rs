// import file libraries
use std::fs::File;
use std::io::Read;
// import set frm collections
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
struct Rational {
    numerator: i128,
    denominator: i128,
}
fn gcd(x: i128, y: i128) -> i128 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

impl Rational {
    fn new(numerator: i128, denominator: i128) -> Self {
        if denominator == 0 {
            panic!("Zero is an invalid denominator!");
        }
        let mut numerator = numerator;
        let mut denominator = denominator;
        // first decrease numerator and denominator if they're too large
        while (numerator.abs() > 13043817825332782212) || (denominator.abs() > 13043817825332782212)
        {
            println!("bruh");
            numerator = numerator / 2;
            denominator = denominator / 2;
        }

        // Reduce to lowest terms by dividing by the greatest common
        // divisor.
        let gcd = gcd(numerator, denominator);
        Self {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        }
    }
    fn from_int(val: i128) -> Self {
        Self {
            numerator: val,
            denominator: 1,
        }
    }
}

impl Div for Rational {
    // The division of rational numbers is a closed operation.
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.numerator == 0 {
            panic!("Cannot divide by zero-valued `Rational`!");
        }

        let numerator = self.numerator * rhs.denominator;
        let denominator = self.denominator * rhs.numerator;
        Self::new(numerator, denominator)
    }
}
// mult
impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        Self::new(numerator, denominator)
    }
}
// add
impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.denominator + rhs.numerator * self.denominator;
        let denominator = self.denominator * rhs.denominator;
        Self::new(numerator, denominator)
    }
}
// sub
impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.denominator - rhs.numerator * self.denominator;
        let denominator = self.denominator * rhs.denominator;
        Self::new(numerator, denominator)
    }
}
#[derive(Debug, Clone)]
struct Expression {
    humn: Rational,
    num: Rational,
}
impl Expression {
    fn unit() -> Self {
        Self {
            humn: Rational::from_int(1),
            num: Rational::from_int(0),
        }
    }
    fn from_int(val: i128) -> Self {
        Self {
            humn: Rational::from_int(0),
            num: Rational::from_int(val),
        }
    }
}
impl Add for Expression {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            humn: self.humn + other.humn,
            num: self.num + other.num,
        }
    }
}

impl Sub for Expression {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            humn: self.humn - other.humn,
            num: self.num - other.num,
        }
    }
}

impl Mul for Expression {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let ret = if self.humn.numerator == 0 {
            Self {
                humn: other.humn * self.num,
                num: other.num * self.num,
            }
        } else {
            Self {
                humn: self.humn * other.num,
                num: self.num * other.num,
            }
        };

        ret
    }
}

impl Div for Expression {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let ret = if self.humn.numerator == 0 {
            Self {
                humn: other.humn / self.num,
                num: other.num / self.num,
            }
        } else {
            Self {
                humn: self.humn / other.num,
                num: self.num / other.num,
            }
        };

        ret
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}
enum Monkey {
    Shout(Expression),
    Math {
        monkey_1: String,
        monkey_2: String,
        operation: Operation,
    },
}
fn get_math_monkey(value: String) -> Monkey {
    let op = value.chars().nth(5).unwrap();
    let mut split = value.split(op);
    Monkey::Math {
        monkey_1: split.next().unwrap().trim().to_owned(),
        operation: match op {
            '+' => Operation::Add,
            '-' => Operation::Sub,
            '*' => Operation::Mul,
            '/' => Operation::Div,
            _ => panic!(),
        },
        monkey_2: split.next().unwrap().trim().to_owned(),
    }
}
fn get_monkey_val(monkey_name: &String, monkeys: &HashMap<String, Monkey>) -> Expression {
    match monkeys.get(monkey_name).unwrap() {
        Monkey::Shout(val) => val.clone(),
        Monkey::Math {
            monkey_1,
            monkey_2,
            operation,
        } => {
            let val_1 = get_monkey_val(monkey_1, monkeys);
            let val_2 = get_monkey_val(monkey_2, monkeys);

            let result = match operation {
                Operation::Add => val_1 + val_2,
                Operation::Sub => val_1 - val_2,
                Operation::Mul => val_1 * val_2,
                Operation::Div => val_1 / val_2,
            };
            result
        }
    }
}
fn main() {
    // open 1.txt
    let mut file = File::open("../data/21.txt").unwrap();
    // read file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let monkeys: HashMap<String, Monkey> = contents
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let monkey_name = parts.next().unwrap();
            if monkey_name == "humn" {
                return (monkey_name.to_owned(), Monkey::Shout(Expression::unit()));
            }
            let val = parts.next().unwrap();
            let monkey = val
                .parse::<i128>()
                .ok()
                .map(|num| Monkey::Shout(Expression::from_int(num)))
                .unwrap_or_else(|| get_math_monkey(val.to_owned()));
            (monkey_name.to_owned(), monkey)
        })
        .collect();
    let (monkey_1, monkey_2) = match &monkeys["root"] {
        Monkey::Math {
            monkey_1, monkey_2, ..
        } => (monkey_1, monkey_2),
        _ => panic!(),
    };
    let left_side = get_monkey_val(monkey_1, &monkeys);
    let right_side = get_monkey_val(monkey_2, &monkeys);
    // ax + b = cx + d
    // (a-c)x = d-b
    // x = (d-b)/(a-c)
    // humn1 = a, humn2 = c, num1 = b, num2 = d
    let humn = right_side.humn - left_side.humn;
    let num = left_side.num - right_side.num;
    println!("{:?}", left_side);
    println!("{:?}", right_side);
    let x = num / humn;
    println!("{:?}", x);
}
