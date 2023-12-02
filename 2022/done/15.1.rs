use std::collections::{HashMap, VecDeque};
// import file libraries
use std::fs::File;
use std::io::Read;

fn main() {
    // open 1.txt
    let mut file = File::open("../data/15.txt").unwrap();
    // read file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let sensor_beacon_pairs: Vec<((i32, i32), (i32, i32))> = contents
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let sensor_x = parts[2]
                .replace("x=", "")
                .replace(",", "")
                .parse::<i32>()
                .unwrap();
            let sensor_y = parts[3]
                .replace("y=", "")
                .replace(":", "")
                .parse::<i32>()
                .unwrap();
            let beacon_x = parts[8]
                .replace("x=", "")
                .replace(",", "")
                .parse::<i32>()
                .unwrap();
            let beacon_y = parts[9].replace("y=", "").parse::<i32>().unwrap();
            ((sensor_x, sensor_y), (beacon_x, beacon_y))
        })
        .collect();
    let y = 2000000;
    let mut count = 0;
    for x in -20000000..20000000 {
        for (sensor, beacon) in sensor_beacon_pairs.iter() {
            if beacon.0 == x && beacon.1 == y {
                continue;
            }
            // get manhattan distance between sensor and beacon
            let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
            // get manhattan distance between sensor and (x,y)
            let distance2 = (sensor.0 - x).abs() + (sensor.1 - y).abs();
            if distance2 <= distance {
                count += 1;
                break;
            }
        }
    }
    // run this with --release (its slowish)
    println!("{}", count);
}
