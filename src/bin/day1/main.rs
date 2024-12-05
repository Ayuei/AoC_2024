use std::collections::HashMap;
use std::iter;
use untitled2::read_puzzle_input;

fn main() {
    let lines =  read_puzzle_input("src/bin/day1/input.txt").unwrap();

    let mut l = Vec::new();
    let mut r = Vec::new();

    let mut r_map: HashMap<usize, usize> = HashMap::new();

    for line in lines.map_while(|l| l.ok()) {
        let items: Vec<usize> = line.split_ascii_whitespace().map(|v| v.parse::<usize>().unwrap()).collect();

        l.push(items.get(0).unwrap().clone());
        r.push(items.get(1).unwrap().clone());

        *r_map.entry(items.get(1).unwrap().clone()).or_insert(0) += 1;
    }

    l.sort();
    r.sort();

    // Part one
    let part_one: usize = iter::zip(l.iter(), r.iter()).map(|(left, right)| left.abs_diff(*right)).sum();

    // Part two
    let part_two: usize = l.iter().map(|v| r_map.get(v).unwrap_or(&0) * v).sum();

    println!("Part one: {part_one}");
    println!("Part two: {part_two}");
}