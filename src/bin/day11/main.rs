use std::collections::HashMap;
use advent_of_code_2024::read_puzzle_input;
use std::time::Instant;

fn count_digits(num: usize) -> usize {
    if num == 0 {
        return 1;
    }
    let mut count = 0;
    let mut n = num;
    while n != 0 {
        n /= 10;
        count += 1;
    }
    count
}

fn split_number(number: usize, num_digits: usize) -> (usize, usize) {
    let split_point = 10_usize.pow((num_digits / 2) as u32);
    (number / split_point, number % split_point)
}

#[memoize]
fn compute_cardinality(number: usize, depth: usize, iterations: usize) -> usize {
    if depth == iterations {
        return 1;
    }

    let num_digits = count_digits(number);

    let cardinality = if number == 0 {
        compute_cardinality(1, depth + 1, iterations)
    } else if num_digits % 2 == 0 {
        let (number_1, number_2) = split_number(number, num_digits);
        compute_cardinality(number_1, depth + 1, iterations) +
            compute_cardinality(number_2, depth + 1, iterations)
    } else {
        compute_cardinality(number * 2024, depth + 1, iterations)
    };

    cardinality
}

fn main() {
    let lines = read_puzzle_input("./src/bin/day11/input.txt").unwrap();
    let start = Instant::now();

    for line in lines.map_while(|l| l.ok()) {
        let numbers: Vec<usize> = line.split_ascii_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
        let cardinality: usize = numbers.into_iter().map(|number| compute_cardinality(number, 0, 75)).sum();

        println!("{:?}", cardinality);
    }

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
}
