// import things to open text file
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Path {
    x: usize,
    y: usize,
}
fn main() {
    // open text file
    let file_path = "./data/3.txt";
    // loop through lines
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file.txt"));

    let paths: Vec<Path> = vec![
        Path { x: 1, y: 1 },
        Path { x: 3, y: 1 },
        Path { x: 5, y: 1 },
        Path { x: 7, y: 1 },
        Path { x: 1, y: 2 },
    ];

    let mut result = 1;

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    for path in paths {
        let mut tree_count = 0;
        let mut x = 0;
        let mut i = 0;
        while i < lines.len() {
            let line = lines.get(i).unwrap();
            let real_x = x % line.len();
            if line
                .chars()
                .nth(real_x.try_into().unwrap())
                .expect("Out of bounds??????")
                == '#'
            {
                tree_count += 1;
            }

            x += path.x;
            i += path.y;
        }
        result *= tree_count;
    }

    println!("{result}")
}
