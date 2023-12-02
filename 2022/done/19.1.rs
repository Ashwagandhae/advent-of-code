// import file libraries
use std::fs::File;
use std::io::Read;
// import set frm collections
use std::collections::HashMap;
use std::thread::{self, JoinHandle};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Resource {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
impl Resource {
    fn enough_for(&self, other: &Resource) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }
    fn subtract(&mut self, other: &Resource) {
        self.ore -= other.ore;
        self.clay -= other.clay;
        self.obsidian -= other.obsidian;
        self.geode -= other.geode;
    }
}
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Robot {
    cost: Resource,
    create: ResourceType,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Blueprint {
    robots: Vec<Robot>,
}
impl Default for Blueprint {
    fn default() -> Self {
        Self { robots: vec![] }
    }
}
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct State {
    resources: Resource,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    minutes: u32,
}
impl State {
    fn can_build_robot(&self, robot: &Robot) -> bool {
        self.resources.enough_for(&robot.cost)
    }
    fn update_resources(&mut self) {
        self.resources.ore += self.ore_robots;
        self.resources.clay += self.clay_robots;
        self.resources.obsidian += self.obsidian_robots;
        self.resources.geode += self.geode_robots;
    }
    fn build_robot(&mut self, robot: &Robot) {
        match robot.create {
            ResourceType::Ore => self.ore_robots += 1,
            ResourceType::Clay => self.clay_robots += 1,
            ResourceType::Obsidian => self.obsidian_robots += 1,
            ResourceType::Geode => self.geode_robots += 1,
        }
        self.resources.subtract(&robot.cost);
    }
    fn tick(&mut self) {
        self.update_resources();
        self.minutes -= 1;
    }
    fn is_done(&self) -> bool {
        self.minutes == 0
    }
    fn worst_possible_final_geodes(&self) -> u32 {
        // worst case scenario is that no more robots are built
        self.resources.geode + self.minutes * self.geode_robots
    }
    fn best_possible_final_geodes(&self) -> u32 {
        // best case scenario is that a geode robot is built every minute from now on
        let mut final_geodes = 0;
        let mut geode_robots = self.geode_robots;
        for _ in 0..self.minutes {
            geode_robots += 1;
            final_geodes += geode_robots;
        }
        self.resources.geode + final_geodes
    }
}

struct MaxGeodeFinder {
    blueprint: Blueprint,
    best_geodes: u32,
    // do not build more robots than needed to build another robot
    max_ore_robots: u32,
    max_clay_robots: u32,
    max_obsidian_robots: u32,
    visited: HashMap<State, u32>,
}
impl MaxGeodeFinder {
    fn new(blueprint: &Blueprint) -> Self {
        let max_ore_cost = blueprint
            .robots
            .iter()
            .map(|robot| robot.cost.ore)
            .max()
            .unwrap();
        let max_clay_cost = blueprint
            .robots
            .iter()
            .map(|robot| robot.cost.clay)
            .max()
            .unwrap();
        let max_obsidian_cost = blueprint
            .robots
            .iter()
            .map(|robot| robot.cost.obsidian)
            .max()
            .unwrap();
        Self {
            blueprint: blueprint.clone(),
            best_geodes: 0,
            max_ore_robots: max_ore_cost,
            max_clay_robots: max_clay_cost,
            max_obsidian_robots: max_obsidian_cost,
            visited: HashMap::new(),
        }
    }
    fn max_geodes(&mut self, state: State, could_have_built: Option<Vec<ResourceType>>) -> u32 {
        if state.is_done() {
            return state.resources.geode;
        }
        if self.visited.contains_key(&state) {
            return self.visited[&state];
        }
        if state.best_possible_final_geodes() <= self.best_geodes {
            return 0;
        }
        if state.worst_possible_final_geodes() > self.best_geodes {
            self.best_geodes = state.worst_possible_final_geodes();
            println!("best geodes: {}", self.best_geodes);
        }
        if state.ore_robots >= self.max_ore_robots
            && state.clay_robots >= self.max_clay_robots
            && state.obsidian_robots >= self.max_obsidian_robots
        {
            return 0;
        }
        let ret = self
            .blueprint
            .robots
            .clone()
            .iter()
            .filter(|robot| state.can_build_robot(robot))
            .filter(|robot| {
                // if we could have built a robot of this type, but decided not too, then dont build it
                could_have_built.clone().map_or(true, |could_have_built| {
                    !could_have_built.contains(&robot.create)
                })
            })
            .map(|robot| Some(robot))
            .chain(std::iter::once(None))
            .map(|robot| {
                let mut new_state = state.clone();
                new_state.tick();
                match robot {
                    Some(robot) => {
                        new_state.build_robot(&robot);
                        self.max_geodes(new_state, None)
                    }
                    None => match could_have_built.clone() {
                        Some(_) => self.max_geodes(new_state, could_have_built.clone()),
                        None => self.max_geodes(
                            new_state,
                            Some(
                                self.blueprint
                                    .robots
                                    .clone()
                                    .iter()
                                    .filter(|robot| state.can_build_robot(robot))
                                    .map(|robot| robot.create.clone())
                                    .collect(),
                            ),
                        ),
                    },
                }
            })
            .max()
            .unwrap();
        self.visited.insert(state, ret);
        ret
    }
}
fn max_geodes(blueprint: &Blueprint, minutes: u32) -> u32 {
    let max_geodes = MaxGeodeFinder::new(blueprint).max_geodes(
        State {
            resources: Resource {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            minutes,
        },
        None,
    );
    max_geodes
}
fn main() {
    // open 1.txt
    let mut file = File::open("../data/19.txt").unwrap();
    // read file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let handles: Vec<JoinHandle<u32>> = contents
        .lines()
        .map(|line| {
            let mut nums = line
                .split(" ")
                .map(|word| word.parse::<u32>())
                .filter(|word| word.is_ok())
                .map(|num| num.unwrap());
            // exclude first num
            let blueprint = Blueprint {
                robots: vec![
                    Robot {
                        cost: Resource {
                            ore: nums.next().unwrap(),
                            clay: 0,
                            obsidian: 0,
                            geode: 0,
                        },
                        create: ResourceType::Ore,
                    },
                    Robot {
                        cost: Resource {
                            ore: nums.next().unwrap(),
                            clay: 0,
                            obsidian: 0,
                            geode: 0,
                        },
                        create: ResourceType::Clay,
                    },
                    Robot {
                        cost: Resource {
                            ore: nums.next().unwrap(),
                            clay: nums.next().unwrap(),
                            obsidian: 0,
                            geode: 0,
                        },
                        create: ResourceType::Obsidian,
                    },
                    Robot {
                        cost: Resource {
                            ore: nums.next().unwrap(),
                            clay: 0,
                            obsidian: nums.next().unwrap(),
                            geode: 0,
                        },
                        create: ResourceType::Geode,
                    },
                ],
            };
            blueprint
        })
        .enumerate()
        .map(|(index, blueprint)| {
            println!("finding {index} max geodes");
            thread::spawn(move || max_geodes(&blueprint, 24))
        })
        .collect();
    let mut results: Vec<u32> = Vec::new();
    for handle in handles {
        results.push(handle.join().unwrap());
    }
    let sum_quality_levels: u32 = results
        .iter()
        .enumerate()
        .map(|(index, val)| val * (index + 1) as u32)
        .sum();

    // print all max geodes
    println!("{:?}", sum_quality_levels);
}
