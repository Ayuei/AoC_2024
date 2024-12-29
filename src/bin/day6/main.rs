use std::collections::HashSet;
use advent_of_code_2024::read_puzzle_input;

fn check_cycle(starting_pos: (isize, isize), map: HashSet<(isize, isize)>, x_bound: isize,
               y_bound: isize) -> bool {
    // Store the unique positions traversed
    let mut positions = HashSet::new();
    let mut duplicates = 0;
    let mut direction: (isize, isize) = (0, -1);

    let mut current = starting_pos;
    let mut prev_position = starting_pos;
    positions.insert(current);
    // println!("{:?}", current);

    while current.0 < x_bound && current.1 < y_bound && current.0 > -1 && current.1 > -1 {
        current = (current.0 + direction.0, current.1 + direction.1);

        if map.contains(&(current.1, current.0)) {
            direction = (-1*direction.1, direction.0); // 90 degree rotation (y, -x)
            current = prev_position;
        } else {
            if positions.contains(&current) {
                duplicates += 1;
            }

            if positions.len() == duplicates {
                return true;
            }

            positions.insert(current);
        }

        prev_position = current;
    }

    false
}

fn main() {
    let lines = read_puzzle_input("src/bin/day6/input.txt").unwrap();
    let mut map: HashSet<(isize, isize)> = HashSet::new();
    let mut starting_position: (isize, isize) = (0, 0);
    let mut current: (isize, isize) = (0, 0);
    let mut direction: (isize, isize) = (0, 0);

    // Store the unique positions traversed
    let mut positions = HashSet::new();

    // Store the boundaries
    let mut y_bound = 0;

    // only consider x bound in first loop
    let mut flag = false;
    let mut x_bound = -1;

    for (y, line) in lines.map_while(|l| l.ok()).enumerate() {
        x_bound = line.chars().count() as isize;
        for (x, c) in line.chars().enumerate().filter(|(x, c)| c != &'.') {
            if c == '#' {
                map.insert((y as isize, x as isize));
            } else {
                if current.0 != 0 {
                    println!("Start has been set multiple times?");
                }
                starting_position = (x as isize, y as isize);
                current = starting_position;
                direction = (0, -1);
            }
        }

        flag = true;
        y_bound += 1;
    }
    // println!("X bound: {x_bound}, y bound: {y_bound}");

    let mut prev_position = current;
    positions.insert(current);
    // println!("{:?}", current);

    while current.0 < x_bound && current.1 < y_bound && current.0 > -1 && current.1 > -1 {
        current = (current.0 + direction.0, current.1 + direction.1);
        // println!("{:?}", current);

        if map.contains(&(current.1, current.0)) {
            // println!("Changing direction");
            direction = (-1*direction.1, direction.0); // 90 degree rotation (y, -x)
            current = prev_position;
        } else {
            positions.insert(current);
        }

        prev_position = current;
    }

    let mut obstructions = 0;
    let mut progress = 0;

    for position in positions.iter() {
        progress += 1;
        if position == &starting_position {
            continue
        }

        let mut new_map_obstacles = map.clone();
        new_map_obstacles.insert((position.1, position.0));

        if check_cycle(starting_position, new_map_obstacles, x_bound, y_bound) {
            obstructions += 1;
        }
    }

    // println!("{:?}", positions);

     for i in (0..x_bound) {
         for j in (0..y_bound) {
             let current = (j,i);
             if positions.contains(&current) {
                 print!("x");
             } else if map.contains(&(i,j)) {
                 print!("#");
             } else {
                 print!(".");
             }
         }
         println!();
     }

    positions.remove(&starting_position);
    println!("Part one: {}", positions.len());
    println!("Part two: {}", obstructions);
}