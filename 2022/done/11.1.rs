use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
// import file libraries
use std::fs::File;
use std::io::Read;

#[derive(Clone)]
enum Operation {
    Add(i32),
    Multiply(i32),
    Square,
}
impl Operation {
    fn apply(&self, x: &i32) -> i32 {
        match self {
            Operation::Add(y) => x + y,
            Operation::Multiply(y) => x * y,
            Operation::Square => x * x,
        }
    }
}
#[derive(Clone)]
struct Test {
    denominator: i32,
    true_target: i32,
    false_target: i32,
}
impl Test {
    fn get_target_monkey(&self, x: &i32) -> i32 {
        match x % self.denominator {
            0 => self.true_target,
            _ => self.false_target,
        }
    }
}
#[derive(Clone)]
struct Monkey {
    items: VecDeque<i32>,
    operation: Operation,
    test: Test,
    inspect_count: i32,
}
impl Monkey {
    fn new(items: VecDeque<i32>, operation: Operation, test: Test) -> Monkey {
        Monkey {
            items,
            operation,
            test,
            inspect_count: 0,
        }
    }
    fn add_item(&mut self, item: i32) {
        self.items.push_back(item);
    }
    fn get_changes(&mut self) -> Vec<(i32, i32)> {
        self.items
            .iter()
            .map(|item| {
                let inspected_item = self.operation.apply(&item);
                self.inspect_count += 1;
                let relief_item = inspected_item / 3;
                let target_monkey = self.test.get_target_monkey(&relief_item);
                (target_monkey, relief_item)
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
            let items: VecDeque<i32> = lines
                .nth(1)
                .unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|x| x.parse().unwrap())
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
            let test_cond: i32 = lines
                .nth(0)
                .unwrap()
                .split("Test: divisible by ")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            let true_monkey: i32 = lines
                .nth(0)
                .unwrap()
                .split("If true: throw to monkey ")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            let false_monkey: i32 = lines
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
    for round in 0..20 {
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
                println!("Monkey {} has item {}", i, item);
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
