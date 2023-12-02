// import file libraries
use std::fs::File;
use std::io::Read;
// import set frm collections
use std::collections::HashSet;

fn main() {
    // open 1.txt
    let mut file = File::open("../data/15.txt").unwrap();
    // read file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let sensor_beacon_pairs: Vec<((i64, i64), (i64, i64))> = contents
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let sensor_x = parts[2]
                .replace("x=", "")
                .replace(",", "")
                .parse::<i64>()
                .unwrap();
            let sensor_y = parts[3]
                .replace("y=", "")
                .replace(":", "")
                .parse::<i64>()
                .unwrap();
            let beacon_x = parts[8]
                .replace("x=", "")
                .replace(",", "")
                .parse::<i64>()
                .unwrap();
            let beacon_y = parts[9].replace("y=", "").parse::<i64>().unwrap();
            ((sensor_x, sensor_y), (beacon_x, beacon_y))
        })
        .collect();

    let mut possible_beacons: HashSet<(i64, i64)> = HashSet::new();
    // fill beacons (it must be on the edge of a sensor's shadow if there's only one possiblity)
    for (i, ((sensor_x, sensor_y), (beacon_x, beacon_y))) in sensor_beacon_pairs.iter().enumerate()
    {
        // get manhattan distance, + 1 for border
        let distance = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs() + 1;
        // fill grid
        for i in 0..distance {
            let x = sensor_x - i;
            let y = sensor_y - distance + i;
            let width = 1 + i * 2;
            // top triangle
            possible_beacons.insert((x, y));
            possible_beacons.insert((x + width, y));
            // bottom triangle
            let y = sensor_y + distance - i;
            possible_beacons.insert((x, y));
            possible_beacons.insert((x + width, y));
        }
        println!("done with sensor {}/{}", i, sensor_beacon_pairs.len());
    }

    let view = 4000000;
    let correct_beacons = possible_beacons
        .iter()
        .filter(|(x, y)| *x <= view && *x >= 0 && *y <= view && *y >= 0)
        .filter(|possible_beacon| {
            for (sensor, beacon) in sensor_beacon_pairs.iter() {
                if beacon.0 == possible_beacon.0 && beacon.1 == possible_beacon.1 {
                    return false;
                }
                // get manhattan distance between sensor and beacon
                let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
                // get manhattan distance between sensor and (x,y)
                let distance2 =
                    (sensor.0 - possible_beacon.0).abs() + (sensor.1 - possible_beacon.1).abs();
                if distance2 <= distance {
                    return false;
                }
            }
            true
        })
        .collect::<Vec<&(i64, i64)>>();
    // print hashset
    for i in correct_beacons.iter() {
        println!("{},{}", i.0, i.1);
    }
    let correct_beacon = &correct_beacons[0];
    // run this with --release (its slowww)
    let tuning_freq: i64 = correct_beacon.0 * 4000000 + correct_beacon.1;
    println!("tuning freq: {}", tuning_freq);
}
