// import file libraries
use std::fs::File;
use std::io::Read;
// import set frm collections
use std::collections::HashMap;

enum Operation {
    Add,
    Sub,
    Mult,
    Div,
}
enum Monkey {
    Shout(u64),
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
            '*' => Operation::Mult,
            '/' => Operation::Div,
            _ => panic!(),
        },
        monkey_2: split.next().unwrap().trim().to_owned(),
    }
}
fn get_monkey_val(monkey_name: &String, monkeys: &HashMap<String, Monkey>) -> u64 {
    match monkeys.get(monkey_name).unwrap() {
        Monkey::Shout(val) => *val,
        Monkey::Math {
            monkey_1,
            monkey_2,
            operation,
        } => {
            let val_1 = get_monkey_val(monkey_1, monkeys);
            let val_2 = get_monkey_val(monkey_2, monkeys);
            match operation {
                Operation::Add => val_1 + val_2,
                Operation::Sub => val_1 - val_2,
                Operation::Mult => val_1 * val_2,
                Operation::Div => val_1 / val_2,
            }
        }
    }
}
fn main() {
    // open 1.txt
    let mut file = File::open("../data/21.txt").unwrap();
    // read file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut monkeys: HashMap<String, Monkey> = contents
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let monkey_name = parts.next().unwrap();
            let val = parts.next().unwrap();
            let monkey = val
                .parse::<u64>()
                .ok()
                .map(|num| Monkey::Shout(num))
                .unwrap_or_else(|| get_math_monkey(val.to_owned()));
            (monkey_name.to_owned(), monkey)
        })
        .collect();
    println!("{:?}", get_monkey_val(&"root".to_string(), &monkeys))
}
