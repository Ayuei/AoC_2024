use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;
use advent_of_code_2024::read_puzzle_input;

fn main() {
    let lines = read_puzzle_input("src/bin/day6/input.txt").unwrap();
    let mut map: HashSet<(isize, isize)> = HashSet::new();
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
                map.insert((x as isize, y as isize));
            } else {
                if current.0 != 0 {
                    println!("Start has been set multiple times?");
                }
                current = (x as isize, y as isize);
                direction = (0, -1);
            }
        }

        flag = true;
        y_bound += 1;
    }
    println!("X bound: {x_bound}, y bound: {y_bound}");

    let mut prev_position = current;
    positions.insert(current);

    while current.0 < x_bound && current.1 < y_bound && current.0 > -1 && current.1 > -1 {
        current = (current.0 + direction.0, current.1 + direction.1);
        println!("{:?}", current);

        if map.contains(&current) {
            println!("Changing direction");
            direction = (-1*direction.1, direction.0); // 90 degree rotation (y, -x)
            current = prev_position;
        } else {
            positions.insert(current);
        }

        prev_position = current;
    }

    println!("Part one: {}", positions.len() - 1);
}