use std::collections::HashMap;
use std::fs;
use std::io::empty;
use itertools::{Itertools};

type DiskMap = HashMap<usize, Vec<usize>>;

fn move_contiguous(mut disk_map: DiskMap) -> Vec<isize>{
    let mut expanded_arr: Vec<isize> = vec![];
    let mut c  = disk_map.len() - 1;

    for i in 0..disk_map.len() {
        let arr = disk_map.get(&i).unwrap();

        for _ in 0..arr.len() {
            expanded_arr.push(i as isize);
        }

        for _ in 0..(arr.capacity() - arr.len()) {
            expanded_arr.push(-1);
        }
    }

    // println!("{:?}", expanded_arr);

    while c > 0 {
        let file_block_size= disk_map.entry(c).or_default().len();

        // println!("KEY to move: {c} of size {}", file_block_size);

        let mut empty_block_size = 0;
        let mut empty_start: isize = -1;

        let mut e_c = 0;

        // Find empty position
        for entry in expanded_arr.iter() {
            if *entry == -1 {
                if empty_start == -1 {
                    empty_start = e_c as isize;
                }
                empty_block_size += 1;
            } else {
                empty_start = -1;
                empty_block_size = 0;
            }

            // println!("Empty size: {empty_block_size}");
            if empty_block_size == file_block_size {
                break
            }

            e_c += 1;
        }

        let mut c_idx = 0;

        // Find the position of the key
        for i in 0..expanded_arr.len() {
            if expanded_arr[i] == c as isize {
                c_idx = i;
                break;
            }
        }

        // println!("Idx: {c_idx}, key: {c}, empty_start: {empty_start}");
        // println!("{:?}", expanded_arr);

        // Skip condition.
        if (empty_start as usize) < c_idx {
            let mut file_block = disk_map.remove(&c).unwrap();
            // println!("Moving {c} of size {} to empty block at {empty_start}", file_block.len());

            for i in 0..expanded_arr.len() {
                if expanded_arr[i] == c as isize {
                    expanded_arr[i] = -1;
                }
            }

            for i in 0..file_block.len() {
                expanded_arr[(empty_start as usize) + i] = c as isize;
            }

            // println!("{:?}", expanded_arr);
        }

        c -= 1;
    }

    expanded_arr
}

fn move_individual(disk_map: &DiskMap) -> DiskMap {
    let mut disk_map = disk_map.clone();

    let mut c  = disk_map.len() - 1;
    let mut stop = false;

    while !stop {
        let mut empty_id = 0;

        // Find empty position
        for key in 0..disk_map.len() {
            let entry = disk_map.entry(key.clone()).or_default();

            if entry.len() < entry.capacity() {
                empty_id = key.clone();
                break;
            }
        }

        // Stopping condition.
        if c == empty_id {
            stop = true;
            break
        }

        disk_map.entry(empty_id).or_default().push(c);
        disk_map.entry(c).or_default().swap_remove(0);

        if disk_map.entry(c).or_default().len() == 0 {
            c -= 1;
        }
    }

    disk_map
}

fn compute_checksum(mut disk_map: DiskMap, debug: bool) -> usize {
    let mut i = 0;
    let mut checksum= 0;

    for k in 0..disk_map.len() {
        checksum += disk_map.entry(k).or_default().iter().fold(0, |acc, x| {
            let sum = acc + (x * i);
            if debug {
                print!("{x}");
            }
            i += 1;
            sum
        });
    }

    println!();

    checksum
}

fn main() {
    let puzzle = fs::read_to_string("./src/bin/day8/input.txt").unwrap();
    let mut disk_map: DiskMap = HashMap::new();
    let mut id = 0;

    let puzzle_chars: Vec<usize> = puzzle.chars().map(|v| v.to_string().parse::<usize>().unwrap()).collect();

    for chunk in &(0..puzzle.len()).chunks(2) {
        let chunks: (usize, usize) = match chunk.collect_tuple() {
            Some((a,b)) => (puzzle_chars.get(a).unwrap().clone(), puzzle_chars.get(b).unwrap().clone()),
            None => break,
        };

        disk_map.entry(id).or_insert(Vec::with_capacity(chunks.1 + chunks.0)).append(&mut vec![id; chunks.0]);

        id += 1;
    }

    let last = puzzle_chars.last().unwrap();

    disk_map.entry(id).or_insert(vec![id; *last]);

    println!("{:?}", disk_map);

    // let individual_disk_map = move_individual(&disk_map);
    let contiguous_disk_map = move_contiguous(disk_map);

    // println!("{:?}", disk_map);
    // println!("Checksum Part 1: {}", compute_checksum(individual_disk_map, false));
    println!("Checksum Part 2: {}", contiguous_disk_map.iter().map(|v| {
        if *v == -1 {
            &0
        } else {
            v
        }
    }).enumerate().fold(0, |acc, (i, v)| acc + *v as usize * i));
}
