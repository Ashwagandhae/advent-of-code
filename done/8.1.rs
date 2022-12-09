// import file libraries
use std::fs::File;
use std::io::Read;

fn main() {
    // open 1.txt
    let mut file = File::open("../data/8.txt").unwrap();
    // read file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // convert into a 2d array
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<i32> = Vec::new();
        // convert each char to num
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap().try_into().unwrap());
        }
        grid.push(row);
    }
    // save grid dims
    let grid_width = grid[0].len();
    let grid_height = grid.len();
    println!("{}", grid_width);
    // create array of false of same size
    let mut visible: Vec<Vec<bool>> = Vec::new();
    for y in 0..grid_height {
        let mut row: Vec<bool> = Vec::new();
        for _ in 0..grid_width {
            row.push(false);
        }
        visible.push(row);
    }

    let mut max_row = vec![-1; grid_width];

    for y in 0..grid.len() {
        for (i, val) in grid[y].iter().enumerate() {
            if val > &max_row[i] {
                max_row[i] = *val;
                visible[y][i] = true;
            } else {
            }
        }
    }
    let mut max_row: Vec<i32> = vec![-1; grid_width];
    for y in 0..grid.len() {
        // reverse y
        let y = grid.len() - y - 1;
        for (i, val) in grid[y].iter().enumerate() {
            if val > &max_row[i] {
                max_row[i] = *val;
                visible[y][i] = true;
            } else {
            }
        }
    }

    let mut max_row = vec![-1; grid_height];
    for x in 0..grid[0].len() {
        for (i, val) in grid.iter().enumerate() {
            if val[x] > max_row[i] {
                max_row[i] = val[x];
                visible[i][x] = true;
            } else {
            }
        }
    }

    let mut max_row = vec![-1; grid_height];
    for x in 0..grid[0].len() {
        // reverse x
        let x = grid[0].len() - x - 1;
        for (i, val) in grid.iter().enumerate() {
            if val[x] > max_row[i] {
                max_row[i] = val[x];
                visible[i][x] = true;
            } else {
            }
        }
    }

    // count visible
    let mut count = 0;
    for y in 0..grid_height {
        for x in 0..grid_width {
            if visible[y][x] {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
