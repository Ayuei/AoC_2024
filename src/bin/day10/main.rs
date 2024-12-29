use std::collections::HashSet;
use itertools::Itertools;
use advent_of_code_2024::read_puzzle_input;

#[derive(Copy, Debug, Clone)]
enum Operation {
    Up,
    Down,
    Left,
    Right,
}

impl Operation {
    fn apply_direction(self, coord: (usize, usize), x_bound: usize, y_bound: usize) -> Option<(usize, usize)> {
        match self {
            Operation::Up => {
                if coord.1 < y_bound - 1 {
                    Some((coord.0, coord.1 + 1))
                } else {
                    None
                }
            }
            Operation::Down => {
                if coord.1 > 0 {
                    Some((coord.0, coord.1 - 1))
                } else {
                    None
                }
            }
            Operation::Left => {
                if coord.0 > 0 {
                    Some((coord.0 - 1, coord.1))
                } else {
                    None
                }
            }
            Operation::Right => {
                if coord.0 < x_bound - 1 {
                    Some((coord.0 + 1, coord.1))
                } else {
                    None
                }
            }
        }
    }
}

fn check_valid_slope(map: &Vec<Vec<usize>>, a: (usize, usize), b: (usize, usize)) -> bool {
     map[b.1][b.0] - map[a.1][a.0] == 1
}

fn generate_possible_path(coord: (usize, usize), map: &Vec<Vec<usize>>, symbols: &[Operation], result: &mut Vec<(usize, usize)>) {
    let x_bound = map.first().unwrap().len();
    let y_bound = map.len();

    if map[coord.1][coord.0] == 9 {
        result.push(coord);
        return;
    }

    for &symbol in symbols {
        let Some(new_coord) = symbol.apply_direction(coord, x_bound, y_bound) else {
            // println!("Out of bounds");
            // Early exit
            continue
        };

        if !check_valid_slope(map, coord, new_coord) {
            // println!("Invalid slope");
            // Early exit
            continue
        }

        generate_possible_path(new_coord, map, symbols, result);
    }
}

fn tailhead_rating(map: &Vec<Vec<usize>>, start: (usize, usize)) -> usize {
    let mut reachable_coordinates= Vec::new();

    generate_possible_path(start, map, &[Operation::Up, Operation::Down, Operation::Left, Operation::Right],
                           &mut reachable_coordinates);

    reachable_coordinates.len()
}

fn tailhead_score(map: &Vec<Vec<usize>>, start: (usize, usize)) -> usize {
    let mut reachable_coordinates= Vec::new();

    generate_possible_path(start, map, &[Operation::Up, Operation::Down, Operation::Left, Operation::Right],
                           &mut reachable_coordinates);

    let unique: Vec<&(usize, usize)> = reachable_coordinates.iter().unique().collect();
    unique.len()
}

fn main() {
    let lines = read_puzzle_input("src/bin/day10/input.txt").unwrap();
    let mut map: Vec<Vec<usize>> = Vec::new();
    let mut starting_coords = Vec::new();

    for (y, line) in lines.map_while(|l| l.ok()).enumerate() {
        let row: Vec<usize> = line.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect();
        let mut starts: Vec<(usize, usize)> = row.iter().enumerate().filter(|(i, c)| **c == 0).map(|(x, c)| (x, y)).collect();

        starting_coords.append(&mut starts);
        map.push(row);
    }

    let mut part_one_total = 0;
    let mut part_two_total = 0;

    for starting_coord in starting_coords.iter() {
        // Technically doing the same thing twice. But it's ok, same time complexity.
        let score = tailhead_score(&map, *starting_coord);
        let rating = tailhead_rating(&map, *starting_coord);

        part_one_total += score;
        part_two_total += rating;
    }

    println!("Tailhead score: {part_one_total}");
    println!("Tailhead rating: {part_two_total}");
}
