use itertools::Itertools;
use advent_of_code_2024::read_puzzle_input;

#[derive(Copy, Clone)]
enum Operation {
    Multiply,
    Addition,
    Concat,
}

impl Operation {
    fn operate(self: Operation, num1: &usize, num2: &usize) -> usize {
        match self {
            Operation::Multiply => { num1 * num2 }
            Operation::Addition => { num1 + num2 }
            Operation::Concat=> { (*num1 as u64 * 10u64.pow(num2.ilog10() + 1) + *num2 as u64) as usize }
        }
    }
}

fn generate_sequences(symbols: &[Operation], length: usize, current: &mut Vec<Operation>, result: &mut Vec<Vec<Operation>>) {
    if current.len() == length {
        result.push(current.iter().map(|v| v.clone()).collect());
        return;
    }

    for &symbol in symbols {
        current.push(symbol);
        generate_sequences(symbols, length, current, result);
        current.pop();
    }
}

fn is_solvable(test_value: usize, values: Vec<usize>) -> bool {
    let mut cur = Vec::new();
    let mut sequences= Vec::new();

    generate_sequences(&[Operation::Addition, Operation::Multiply, Operation::Concat],
                       values.len() - 1,
                       &mut cur, &mut sequences);

    let mut solvable = false;

    for sequence in sequences {
        let total: usize = values.iter().skip(1).enumerate().fold(*values.get(0).unwrap(), |total, (i, v)| sequence.get(i).unwrap().operate(&total, v));

        if total == test_value {
            solvable = true;
            break;
        }
    }

    solvable
}

fn main() {
    let lines = read_puzzle_input("src/bin/day7/input.txt").unwrap();
    let mut part_one_total = 0;

    for line in lines.map_while(|l| l.ok()) {
        let mut values: Vec<usize> = line.replace(':', "").split_ascii_whitespace().into_iter().filter_map(|s| s.parse::<usize>().ok()).collect();
        let test_value = values.remove(0);

        if is_solvable(test_value, values) {
            part_one_total += test_value;
        } else {
        }
    }

    println!("{}", part_one_total);
}