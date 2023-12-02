use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let txt = read_to_string("../_inputs/2.txt").unwrap();
    println!(
        "{}",
        txt.split("\n")
            .map(|s| {
                let (_, content) = s.split_once(": ").unwrap();
                let mut min_cubes: HashMap<&str, i32> = HashMap::new();
                content.split("; ").for_each(|turn| {
                    turn.split(", ").for_each(|cube_info| {
                        let (cube_num, cube_name) = cube_info.split_once(" ").unwrap();
                        let cube_num: i32 = cube_num.parse().unwrap();
                        min_cubes
                            .entry(cube_name)
                            .and_modify(|e| *e = (*e).max(cube_num))
                            .or_insert(cube_num);
                    });
                });
                min_cubes.values().fold(1, |acc, &x| acc * x)
            })
            .sum::<i32>()
    );
}
