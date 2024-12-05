use std::fs::File;
use std::io::{BufRead, BufReader};

fn check_valid_dampen(values: &Vec<isize>) -> bool {
    for i in 0..values.len() {
        if check_valid(&values.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, v)| v.clone()).collect()) {
            return true
        }
    }

    false
}

fn check_valid(values: &Vec<isize>) -> bool {
    // println!("Values: {:?}", values);
    let mut prev_val = &0;
    let mut prev_diff: Option<isize> = None;

    for (i, val) in values.iter().enumerate() {
        if i == 0 {
            prev_val = val;
            continue;
        }

        let current_diff = val - prev_val;

        if prev_val == val || !(1..4).contains(&current_diff.abs()) || prev_diff.map_or(false, |diff| diff.signum() != current_diff.signum()) {
            return false;
        }

        prev_diff = Some(current_diff);
        prev_val = val;
    }

    true
}

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());

    let mut part_one_valid = 0;
    let mut part_two_valid = 0;

    for line in reader.lines().map_while(|l| l.ok()) {
        let values: Vec<isize> = line.split_ascii_whitespace().map(|v| v.parse::<isize>().unwrap()).collect();

        let mut double_true = false;

        if check_valid(&values) {
            double_true = true;
            part_one_valid += 1;
        }

        if check_valid_dampen(&values) {
            part_two_valid += 1;
        } else {
            println!("{:?}", values);
        }

    }

    println!("Part one: {part_one_valid}");
    println!("Part two: {part_two_valid}");
}
