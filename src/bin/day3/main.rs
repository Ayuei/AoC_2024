use itertools::Itertools;
use untitled2::read_puzzle_input;
use regex::Regex;

fn part_one(line: String) -> i32 {
    let pattern = Regex::new(r"mul\((\d+?),(\d+?)\)").unwrap();
    let mut total = 0;

    let captures = pattern.captures_iter(&line);

    for capture in captures {
        let num1 = capture.get(1).map_or("", |m| m.as_str());
        let num2 = capture.get(2).map_or("", |m| m.as_str());

        total += (num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap());
    }

    total
}

fn part_two(line: String) -> i32 {
    let pattern = Regex::new(r"(do\(\))|(don't\(\))|mul\((\d+?),(\d+?)\)").unwrap();
    let mut total = 0;

    let captures = pattern.captures_iter(&line);
    let mut negated = false;

    for capture in captures {
        if let (Some(num1), Some(num2)) = (capture.get(3), capture.get(4)) {
            if !negated {
                println!("Multiplied");
                total += (num1.as_str().parse::<i32>().unwrap() * num2.as_str().parse::<i32>().unwrap());
            } else {
                println!("Skip");
            }
        } else if let Some(v) = capture.get(2) {
            println!("Negate {:?}", v);
            negated = true;
        } else if let Some(v) = capture.get(1) {
            println!("Do {:?}", v);
            negated = false;
        }
    }

    total
}

fn main() {
    let lines =  read_puzzle_input("src/bin/day3/input.txt").unwrap();

    let mut part_one_total = 0;
    let mut part_two_total = 0;

    for line in lines.map_while(|v| v.ok()) {
        part_one_total += part_one(line.clone());
        part_two_total += part_two(line);
    }

    println!("{}", part_one_total);
    println!("{}", part_two_total);
}
