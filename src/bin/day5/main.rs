use std::collections::HashMap;
use advent_of_code_2024::read_puzzle_input;

fn check_invalid_line(line: String, map: &mut HashMap<usize, Vec<usize>>) -> usize {
    // Brute force Topological sort
    // Brute force O(n^3) my god

    let numbers: Vec<usize> = line.split(',').map(|v| v.parse::<usize>().unwrap()).collect();

    for i in (0..numbers.len()).rev() {
        let num = numbers[i];

        for j in 0..i{
            if map.entry(num).or_default().contains(&numbers[j]) {
                return 0
            }
        }
    }

    numbers.get(numbers.len() / 2).unwrap().clone()
}

fn reoder_invalid_line(line: String, map: &mut HashMap<usize, Vec<usize>>) -> usize {
    // Brute force Topological sort
    // Brute force O(n^3) my god

    let mut numbers: Vec<usize> = line.split(',').map(|v| v.parse::<usize>().unwrap()).collect();

    let mut i = numbers.len() - 1;
    let j = 0;

    loop {
        let num = numbers[i];
        let mut current_idx = i;

        for j in 0..i {
            if map.entry(num).or_default().contains(&numbers[j]) {
                // Out of order. Move that numbers behind the current one.
                let temp = numbers.remove(j);

                numbers.insert(current_idx, temp);
                current_idx -= 1;
            }
        }

        // If nothing changed. We increment
        if current_idx == i {
            if i == 0 {
                break;
            }

            i -= 1;
        }
    }

    numbers.get(numbers.len() / 2).unwrap().clone()
}

fn main() {
    let puzzle = read_puzzle_input("src/bin/day5/input.txt").unwrap();

    // Brute force: Store the numbers (values) that come after the key
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut part_one_total = 0;
    let mut part_two_total = 0;

    for line in puzzle.map_while(|l| l.ok()).filter(|l| l.len() > 0) {
        if line.contains("|") {
            let values: Vec<usize> = line.split("|").map(|v| v.parse::<usize>().unwrap()).collect();

            let v1 = values[0];
            let v2 = values[1];

            map.entry(v1).or_default().push(v2);
        } else {
            let value = check_invalid_line(line.clone(), &mut map);
            if value == 0 {
                let value_two = reoder_invalid_line(line, &mut map);

                part_two_total += value_two;
            }

            part_one_total += value;
        }
    }

    println!("Part one {part_one_total}");
    println!("Part one {part_two_total}");
}
