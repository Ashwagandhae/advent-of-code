use std::collections::{HashMap, VecDeque};
// import file libraries
use std::fs::File;
use std::io::Read;

#[derive(Clone)]
struct Number {
    // hashmap
    mod_dict: HashMap<i128, i128>,
}
impl Number {
    fn new(val: i128) -> Number {
        let nums = vec![2, 3, 5, 7, 11, 13, 17, 19];
        let mut mod_dict = HashMap::new();
        for num in nums {
            mod_dict.insert(num, val % num);
        }
        Number { mod_dict }
    }
    fn is_divisible_by(&self, x: &i128) -> bool {
        self.mod_dict.get(x).unwrap() == &0
    }
    fn mult(&self, val: i128) -> Number {
        // loop thru dict
        let mut new_num = self.clone();
        for (key, value) in self.mod_dict.iter() {
            // multiply value by val
            new_num.mod_dict.insert(*key, (value * val) % key);
        }
        new_num
    }
    fn add(&self, val: i128) -> Number {
        // loop thru dict
        let mut new_num = self.clone();
        for (key, value) in self.mod_dict.iter() {
            // add value by val
            new_num.mod_dict.insert(*key, (value + val) % key);
        }
        new_num
    }
    fn square(&self) -> Number {
        // loop thru dict
        let mut new_num = self.clone();
        for (key, value) in self.mod_dict.iter() {
            // square value
            new_num.mod_dict.insert(*key, (value * value) % key);
        }
        new_num
    }
}
#[derive(Clone)]
enum Operation {
    Add(i128),
    Multiply(i128),
    Square,
}
impl Operation {
    fn apply(&self, x: &Number) -> Number {
        match self {
            Operation::Add(y) => x.add(*y),
            Operation::Multiply(y) => x.mult(*y),
            Operation::Square => x.square(),
        }
    }
}
#[derive(Clone)]
struct Test {
    denominator: i128,
    true_target: i128,
    false_target: i128,
}
impl Test {
    fn get_target_monkey(&self, x: &Number) -> i128 {
        match x.is_divisible_by(&self.denominator) {
            true => self.true_target,
            false => self.false_target,
        }
    }
}
#[derive(Clone)]
struct Monkey {
    items: VecDeque<Number>,
    operation: Operation,
    test: Test,
    inspect_count: i128,
}
impl Monkey {
    fn new(items: VecDeque<Number>, operation: Operation, test: Test) -> Monkey {
        Monkey {
            items,
            operation,
            test,
            inspect_count: 0,
        }
    }
    fn add_item(&mut self, item: Number) {
        self.items.push_back(item);
    }
    fn get_changes(&mut self) -> Vec<(i128, Number)> {
        self.items
            .iter()
            .map(|item| {
                let inspected_item = self.operation.apply(item);
                self.inspect_count += 1;
                let target_monkey = self.test.get_target_monkey(&inspected_item);
                (target_monkey, inspected_item)
            })
            .collect()
    }
}

fn main() {
    // open 1.txt
    let mut file = File::open("../data/11.txt").unwrap();
    // read file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut monkeys: Vec<Monkey> = contents
        .split("\n\n")
        .map(|block| {
            let mut lines = block.lines();
            let items: VecDeque<Number> = lines
                .nth(1)
                .unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i128>>()
                .iter()
                .map(|x| Number::new(*x))
                .collect();
            let operation = match lines
                .nth(0)
                .unwrap()
                .split("Operation: new = old ")
                .nth(1)
                .unwrap()
            {
                "* old" => Operation::Square,
                x if x.contains("+") => {
                    Operation::Add(x.split("+ ").nth(1).unwrap().parse().unwrap())
                }
                x if x.contains("*") => {
                    Operation::Multiply(x.split("* ").nth(1).unwrap().parse().unwrap())
                }
                _ => panic!("Unknown operation"),
            };
            let test_cond: i128 = lines
                .nth(0)
                .unwrap()
                .split("Test: divisible by ")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            let true_monkey: i128 = lines
                .nth(0)
                .unwrap()
                .split("If true: throw to monkey ")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            let false_monkey: i128 = lines
                .nth(0)
                .unwrap()
                .split("If false: throw to monkey ")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();

            Monkey::new(
                items,
                operation,
                Test {
                    denominator: test_cond,
                    true_target: true_monkey,
                    false_target: false_monkey,
                },
            )
        })
        .collect();
    for round in 0..10000 {
        println!("round {}", round);

        for j in 0..monkeys.len() {
            // print monkey items
            for (target_monkey, relief_item) in monkeys[j].get_changes() {
                monkeys[target_monkey as usize].add_item(relief_item);
            }
            monkeys[j].items.clear();
        }
        // print each monkey item in each round
        println!("{}", monkeys.len());
        for (i, monkey) in monkeys.iter().enumerate() {
            for item in monkey.items.iter() {
                // println!("Monkey {} has item {}", i, item);
            }
        }
    }
    println!("got thru");
    // sort monkeys by inspect_count (largest first)
    monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
    // print top two
    for monkey in monkeys.iter().take(2) {
        println!("Monkey {} ", monkey.inspect_count,);
    }
    // multiply top two
    println!(
        "Product of top two monkeys: {}",
        monkeys[0].inspect_count * monkeys[1].inspect_count
    );
}
