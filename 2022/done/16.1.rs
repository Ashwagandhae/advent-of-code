// import file libraries
use std::fs::File;
use std::io::Read;
// import set frm collections
use std::collections::HashMap;
use std::collections::HashSet;

struct Valve {
    rate: u32,
    children: Vec<String>,
}

fn best_sequence(
    valve_key: &String,
    valves: &HashMap<String, Valve>,
    // memo: &mut HashMap<(String, u32, Vec<String>), (u32, Vec<(u32, String)>)>,
    memo: &mut HashMap<(String, u32, Vec<String>), u32>,
    mut opened: HashSet<String>,
    max_opened: usize,
    minutes: u32,
    progress: &mut u32,
) -> u32 {
    // let mut path = vec![(minutes, valve_key.clone())];
    // print opened
    if minutes == 0 {
        return 0;
    }
    // // check if max opened
    // if opened.len() == max_opened {
    //     return 0;
    // }
    *progress += 1;
    if *progress % 1000000 == 0 {
        println!("progress: {}", progress);
    }

    // convert opened to sorted vec
    let mut opened_key: Vec<String> = opened.iter().map(|s| s.clone()).collect();
    opened_key.sort();
    let memo_key = &(valve_key.clone(), minutes.clone(), opened_key.clone());
    // check if has been memoized
    if memo.contains_key(&memo_key) {
        // path.append(&mut memo[memo_key].1.clone());
        // return (memo[memo_key].0, path);
        return memo[memo_key];
    }

    let valve = &valves[valve_key];

    let mut pressure = 0;
    let mut rates: Vec<u32> = Vec::new();
    // let mut child_paths: Vec<Vec<(u32, String)>> = Vec::new();
    for child in &valve.children {
        let child_pressure = best_sequence(
            child,
            valves,
            memo,
            opened.clone(),
            max_opened,
            minutes - 1,
            progress,
        );
        rates.push(child_pressure + pressure);
        // child_paths.push(child_path);
    }
    if valve.rate > 0 && !opened.contains(valve_key) {
        opened.insert(valve_key.clone());

        let minutes = minutes - 1;
        // path.push((minutes, valve_key.clone() + " OPEN"));
        if minutes == 0 {
            return 0;
        }
        pressure = valve.rate * (minutes - 1);
        for child in &valve.children {
            let child_pressure = best_sequence(
                child,
                valves,
                memo,
                opened.clone(),
                max_opened,
                minutes - 1,
                progress,
            );
            rates.push(child_pressure + pressure);
            // child_paths.push(child_path);
        }
    }
    // get index of max rate
    // let max_index = rates
    //     .iter()
    //     .enumerate()
    //     .max_by(|x, y| x.1.cmp(y.1))
    //     .unwrap()
    //     .0;
    let ret = rates.iter().max().unwrap().clone();
    // add child path to path
    // path.append(&mut child_paths[max_index]);

    memo.insert(memo_key.clone(), ret);
    // memo.insert(memo_key.clone(), (ret, path.clone()));
    // (ret, path.clone())
    ret
}
fn main() {
    // open 1.txt
    let mut file = File::open("../data/16.txt").unwrap();
    // read file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let valves: HashMap<String, Valve> = contents
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
            (name, Valve { rate, children })
        })
        .collect();

    let mut memo: HashMap<(String, u32, Vec<String>), u32> = HashMap::new();

    // print each key of memo

    let max_opened = valves.len();
    let mut progress: u32 = 0;
    let best = best_sequence(
        &"AA".to_string(),
        &valves,
        &mut memo,
        HashSet::new(),
        max_opened,
        31,
        &mut progress,
    );
    // run with --release
    println!("best: {}", best);
}
