// import things to open text file
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // open text file
    let file_path = "./data/1.txt";
    // loop through lines
    let mut data: Vec<u32> = Vec::new();
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file.txt"));

    for line in reader.lines() {
        data.push(line.unwrap().parse::<u32>().expect("invalid number"));
    }

    // sort data
    data.sort();

    for mid in &data {
        let mut low_index = 0;
        let mut high_index = data.len() - 1;
        let mut low = &data[low_index];
        let mut high = &data[high_index];
        while low < high {
            if mid + low + high > 2020 {
                high_index -= 1;
                high = &data[high_index];
            } else {
                low_index += 1;
                low = &data[low_index];
            }
            if mid + low + high == 2020 {
                println!("{} + {} + {} = 2020", mid, low, high);
                let result = mid * low * high;
                println!("{result}");
                return;
            }
        }
    }
    println!("failed")
}
