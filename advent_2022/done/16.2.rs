// import file libraries
use std::fs::File;
use std::io::Read;
// import set frm collections
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Valve {
    rate: u32,
    is_open: bool,
    children: Vec<usize>,
}
#[derive(Eq, Hash, PartialEq)]
struct MemoKey {
    valve_index: usize,
    valve_index_2: usize,
    minutes: u32,
    opened: Opened,
}
impl MemoKey {
    fn new(valve_index: usize, valve_index_2: usize, minutes: u32, opened: Opened) -> MemoKey {
        MemoKey {
            valve_index,
            valve_index_2,
            minutes,
            opened,
        }
    }
}

// use a u64 to represent the opened valves instead of a bool array
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Opened(u64);
impl Opened {
    fn new() -> Opened {
        Opened(0)
    }
    fn add(&mut self, index: usize) {
        self.0 |= 1 << index;
    }
    fn has(&self, index: usize) -> bool {
        self.0 & (1 << index) != 0
    }
}

struct BestSequenceFinder {
    valves: Vec<Valve>,
    memo: HashMap<MemoKey, u32>,
    max_opened: Opened,
    progress: u32,
    perfect_rate: u32,
    give_up_threshold: u32,
}
impl BestSequenceFinder {
    fn new(valves: Vec<Valve>, divider: usize) -> BestSequenceFinder {
        let mut max_opened = Opened::new();
        (divider..valves.len()).for_each(|index| {
            max_opened.add(index);
        });
        let perfect_rate = (0..divider).fold(0, |acc, index| acc + valves[index].rate);
        BestSequenceFinder {
            valves,
            memo: HashMap::new(),
            max_opened,
            progress: 0,
            perfect_rate,
            give_up_threshold: 1810,
        }
    }
    fn is_base_case(&mut self, minutes: u32, opened: &Opened) -> bool {
        minutes == 0 || *opened == self.max_opened
    }
    fn get_child_rates(
        &mut self,
        valve: &Valve,
        valve_2: &Valve,
        pressure: u32,
        total_pressure: u32,
        current_pressure_rate: u32,
        opened: &Opened,
        minutes: u32,
    ) -> Vec<u32> {
        let mut rates = Vec::new();
        for child in valve.children.iter() {
            for child_2 in valve_2.children.iter() {
                rates.push(
                    self.best_sequence_rec(
                        child.clone(),
                        child_2.clone(),
                        opened.clone(),
                        minutes - 1,
                        total_pressure,
                        current_pressure_rate,
                    ) + pressure,
                );
            }
        }
        rates
    }
    fn best_sequence_rec(
        &mut self,
        valve_index: usize,
        valve_index_2: usize,
        mut opened: Opened,
        minutes: u32,
        mut total_pressure: u32,
        mut current_pressure_rate: u32,
    ) -> u32 {
        // print opened
        if self.is_base_case(minutes, &opened) {
            return 0;
        }

        let memo_key = MemoKey::new(valve_index, valve_index_2, minutes, opened.clone());

        // check if has been memoized
        if self.memo.contains_key(&memo_key) {
            return self.memo[&memo_key];
        }
        self.progress(minutes, valve_index, valve_index_2);

        let valve = self.valves[valve_index].clone();
        let valve_2 = self.valves[valve_index_2].clone();

        let mut pressure = 0;
        if valve.is_open && !opened.has(valve_index) {
            pressure += valve.rate * (minutes - 1);
            current_pressure_rate += valve.rate;
            opened.add(valve_index);
        }
        if valve_2.is_open && !opened.has(valve_index_2) {
            pressure += valve_2.rate * (minutes - 1);
            current_pressure_rate += valve_2.rate;
            opened.add(valve_index_2);
        }
        total_pressure += pressure;

        // give up if we can't beat the give up threshold even with the perfect rate
        if total_pressure + (self.perfect_rate - current_pressure_rate) * (minutes - 1)
            < self.give_up_threshold
        {
            return 0;
        }

        let rates = self.get_child_rates(
            &valve,
            &valve_2,
            pressure,
            total_pressure,
            current_pressure_rate,
            &opened,
            minutes,
        );

        // get max rate
        let ret = rates.iter().max().unwrap().clone();

        self.memo.insert(memo_key, ret);
        ret
    }
    fn best_sequence(&mut self, start_valve: usize, minutes: u32) -> u32 {
        self.best_sequence_rec(start_valve, start_valve, Opened::new(), minutes + 1, 0, 0)
    }

    fn progress(&mut self, minutes: u32, valve_index: usize, valve_index_2: usize) {
        let indent_threshold = 24;
        if minutes > indent_threshold {
            let indent = (26 - indent_threshold + 1) - (minutes - indent_threshold);
            let mut s = String::new();
            for _ in 0..indent {
                s.push_str(" ");
            }
            println!("{}{}: {} and {}", s, minutes, valve_index, valve_index_2);
        }
    }
}
fn main() {
    // open 1.txt
    let mut file = File::open("../data/16.txt").unwrap();
    // read file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let valves: HashMap<String, (u32, Vec<String>)> = contents
        .lines()
        .map(|line| {
            let parts: Vec<String> = line.split("; ").map(|x| x.to_string()).collect();
            let first = parts[0].clone();
            let rate = parts[0]
                .split("rate=")
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let second = parts[1]
                .split("valve")
                .nth(1)
                .expect("couldnt do sedcond")
                .replace("s", "")
                .replace(" ", "");
            let name = first.split(" has").nth(0).unwrap().replace("Valve ", "");
            let children: Vec<String> = second.split(",").map(|x| x.trim().to_string()).collect();
            (name, (rate, children))
        })
        .collect();
    let key_index: HashMap<String, usize> = valves
        .keys()
        .enumerate()
        .map(|(index, key)| (key.clone(), index))
        .collect();
    // convert valve strings to indices
    let valves: HashMap<String, Valve> = valves
        .iter()
        .map(|(key, (rate, children))| {
            let children: Vec<usize> = children.iter().map(|child| key_index[child]).collect();
            (
                key.clone(),
                Valve {
                    rate: *rate,
                    is_open: false,
                    children,
                },
            )
        })
        .collect();
    // and make it a vector, using key_index because .values() is not deterministic
    let mut valves: Vec<Valve> = valves
        .iter()
        .map(|(key, valve)| (key_index[key], valve))
        .fold(
            vec![
                Valve {
                    rate: 0,
                    is_open: false,
                    children: vec![],
                };
                valves.len()
            ],
            |mut acc, (index, valve)| {
                acc[index] = valve.clone();
                acc
            },
        );
    // and add open nodes to each valve that isnt 0 (because 0 is useless)
    let len = valves.len();
    for i in 0..len {
        if valves[i].rate > 0 {
            valves.push(Valve {
                rate: valves[i].rate,
                is_open: true,
                children: valves[i].children.clone(),
            });
            let mut children = valves[i].children.clone();
            children.push(valves.len() - 1);
            valves[i] = Valve {
                rate: valves[i].rate,
                is_open: false,
                children,
            };
        }
    }

    // print valves

    let start_valve = key_index["AA"];
    let mut finder = BestSequenceFinder::new(valves, len);
    let best = finder.best_sequence(start_valve, 26);
    println!("best: {}", best);
}
