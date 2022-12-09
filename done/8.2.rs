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

    let mut max_tree_view = 0;
    // find best tree
    for y in 0..grid_height {
        for x in 0..grid_width {
            let curr = grid[y][x];
            // get up view from current pos
            let mut view_dist = 1;
            let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
            println!("at point {} {}", x, y);
            for (delta_x, delta_y) in dirs {
                let (mut move_y, mut move_x): (i32, i32) =
                    (y.try_into().unwrap(), x.try_into().unwrap());

                let mut count: i32 = 0;
                println!("going dir {} {}", delta_x, delta_y);
                loop {
                    move_y += delta_y;
                    move_x += delta_x;
                    if move_y < 0
                        || move_y >= grid_height as i32
                        || move_x < 0
                        || move_x >= grid_width as i32
                    {
                        break;
                    }
                    count += 1;
                    let check = grid[move_y as usize][move_x as usize];
                    println!("({} {}) = {}", move_x, move_y, check);
                    if check >= curr {
                        break;
                    }
                }
                println!("{}", count);
                view_dist *= count;
            }
            if view_dist > max_tree_view {
                println!("finished, {}", view_dist);

                max_tree_view = view_dist;
            }
        }
    }
    println!("{}", max_tree_view);
}
