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
    Shout(f64),
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
fn get_monkey_val(monkey_name: &String, monkeys: &HashMap<String, Monkey>) -> f64 {
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
            let mut val = parts.next().unwrap().to_string();
            if monkey_name == "root" {
                val = val.replace('+', "-");
            }
            let monkey = val
                .parse::<f64>()
                .ok()
                .map(Monkey::Shout)
                .unwrap_or_else(|| get_math_monkey(val));
            (monkey_name.to_owned(), monkey)
        })
        .collect();
    // see which direciton changing the humn value will change root value
    monkeys.insert("humn".into(), Monkey::Shout(0.0));
    let root_val_1 = get_monkey_val(&"root".to_string(), &monkeys);
    monkeys.insert("humn".into(), Monkey::Shout(1.0));
    let root_val_2 = get_monkey_val(&"root".to_string(), &monkeys);
    let increase_causes_increase = root_val_2 - root_val_1 > 0.0;
    // binary search for the humn value
    let mut lower_bound = u64::MIN;
    let mut upper_bound = u64::MAX;
    let answer = loop {
        let new_val = (lower_bound / 2) + (upper_bound / 2);
        println!("{}", new_val);
        monkeys.insert("humn".into(), Monkey::Shout(new_val as f64));
        let root_val = get_monkey_val(&"root".to_string(), &monkeys);
        println!("root_val : {}", root_val);
        if root_val == 0.0 {
            break new_val;
        } else if (root_val > 0.0) == increase_causes_increase {
            upper_bound = new_val;
        } else {
            lower_bound = new_val;
        }
    };
    println!("answer: {}", answer);
}
