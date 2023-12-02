// import file libraries
use std::fs::File;
use std::io::Read;
// import set frm collections
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Valve {
    rate: u32,
    children: Vec<usize>,
}
#[derive(Eq, Hash, PartialEq)]
struct MemoKey {
    valve_index: usize,
    minutes: u32,
    opened: Opened,
}
impl MemoKey {
    fn new(valve_index: usize, minutes: u32, opened: Opened) -> MemoKey {
        MemoKey {
            valve_index,
            minutes,
            opened,
        }
    }
}
enum Task {
    Open,
    Go,
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
}
impl BestSequenceFinder {
    fn new(valves: Vec<Valve>) -> BestSequenceFinder {
        let mut max_opened = Opened::new();
        (0..valves.len()).for_each(|index| {
            max_opened.add(index);
        });
        BestSequenceFinder {
            valves,
            memo: HashMap::new(),
            max_opened,
            progress: 0,
        }
    }
    fn is_base_case(&mut self, minutes: u32, opened: &Opened) -> bool {
        minutes == 0 || *opened == self.max_opened
    }
    fn get_child_rates(
        &mut self,
        valve: &Valve,
        pressure: u32,
        opened: &Opened,
        minutes: u32,
    ) -> Vec<u32> {
        valve
            .children
            .iter()
            .map(|child| {
                self.best_sequence_rec(child.clone(), Task::Go, opened.clone(), minutes - 1)
                    + pressure
            })
            .collect()
    }
    fn best_sequence_rec(
        &mut self,
        valve_index: usize,
        task: Task,
        mut opened: Opened,
        minutes: u32,
    ) -> u32 {
        self.progress();
        // print opened
        if self.is_base_case(minutes, &opened) {
            return 0;
        }

        let memo_key = MemoKey::new(valve_index, minutes, opened.clone());

        // check if has been memoized
        if self.memo.contains_key(&memo_key) {
            return self.memo[&memo_key];
        }

        let valve = self.valves[valve_index].clone();

        let mut rates: Vec<u32> = Vec::new();

        let mut pressure = 0;

        match task {
            Task::Open => {
                // open valve_1
                if valve.rate > 0 && !opened.has(valve_index) {
                    opened.add(valve_index);
                    let minutes = minutes - 1;
                    if self.is_base_case(minutes, &opened) {
                        return 0;
                    }
                    pressure = valve.rate * (minutes - 1);
                    rates.extend(self.get_child_rates(&valve, pressure, &opened, minutes));
                } else {
                    return 0;
                }
            }
            Task::Go => {
                // no open
                rates.extend(self.get_child_rates(&valve, pressure, &opened, minutes));
                rates.push(self.best_sequence_rec(valve_index, Task::Open, opened, minutes));
            }
        }

        // get max rate
        let ret = rates.iter().max().unwrap().clone();

        self.memo.insert(memo_key, ret);
        ret
    }
    fn best_sequence(&mut self, start_valve: usize, minutes: u32) -> u32 {
        self.best_sequence_rec(start_valve, Task::Go, Opened::new(), minutes + 1)
    }
    fn progress(&mut self) {
        self.progress += 1;
        if self.progress % 1000 == 0 {
            println!("progress: {}", self.progress);
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
                    children,
                },
            )
        })
        .collect();
    // and make it a vector, using key_index because .values() is not deterministic
    let valves: Vec<Valve> = valves
        .iter()
        .map(|(key, valve)| (key_index[key], valve))
        .fold(
            vec![
                Valve {
                    rate: 0,
                    children: vec![],
                };
                valves.len()
            ],
            |mut acc, (index, valve)| {
                acc[index] = valve.clone();
                acc
            },
        );

    // print each key of memo

    let start_valve = key_index["AA"];
    let mut finder = BestSequenceFinder::new(valves);
    let best = finder.best_sequence(start_valve, 30);
    println!("best: {}", best);
}
