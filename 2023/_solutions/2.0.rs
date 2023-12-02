use std::fs::{read_to_string, File};

fn main() {
    let txt = read_to_string("../_inputs/2.txt").unwrap();
    println!(
        "{}",
        txt.split("\n")
            .map(|s| {
                let (game_name, content) = s.split_once(": ").unwrap();
                let game_num: i32 = game_name.split_once(" ").unwrap().1.parse().unwrap();
                let possible = content.split("; ").all(|turn| {
                    turn.split(", ").all(|cube_info| {
                        let (cube_num, cube_name) = cube_info.split_once(" ").unwrap();
                        let cube_num: i32 = cube_num.parse().unwrap();
                        match cube_name {
                            "red" if cube_num <= 12 => true,
                            "green" if cube_num <= 13 => true,
                            "blue" if cube_num <= 14 => true,
                            _ => false,
                        }
                    })
                });
                if possible {
                    game_num
                } else {
                    0
                }
            })
            .sum::<i32>()
    );
}
