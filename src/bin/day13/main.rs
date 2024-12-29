use std::cmp::min;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use itertools::Itertools;
use regex::Regex;

fn simultaneous_equations(a: (isize, isize), b: (isize, isize), amount: (isize, isize)) -> usize {
    // Equation 1
    // a.0 * x + b.0 * y = amount.0
    // x = (amount.0 - b.0 * y) / a.0

    // Equation 2
    // a.1 * x + b.1 * y = amount.1
    // Sub X
    // a.1 * (amount.0 - b.0 * y) / a.0 + b.1 * y = amount.1
    // Solve for Y
    // (a.1 * amount.0 - a.1 * b.0 * y) / a.0 + b.1 * y = amount.1
    //  - a.1 * b.0 * y + a.0 * b.1 * y = amount.1 * a.0
    // y ( a.0 * b.1 - a.1 * b.0 ) = a.0 * amount.1 - a.1 * amount.0
    // y = a.0 * amount.1 - a.1 * amount.0 / ( a.0 * b.1 - a.1 * b.0 )
    // y = (94 * 5400 - 34 * 8400) / (94 * 67 - 34 * 22)

    let y = (a.0 * amount.1 - a.1 * amount.0) as f64 / ( a.0 * b.1 - a.1 * b.0 ) as f64;

    // Solve for x.
    // x = (amount.0 - b.0 * y) / a.0

    let x = (amount.0 as f64 - b.0 as f64 * y) / a.0 as f64;

    if x.fract() != 0.0 || y.fract() != 0.0 || y.is_sign_negative() || x.is_sign_negative() {
        return 0
    }

    println!("X: {x}, Y: {y}");

    // Apply the weights to x and y
    let num_coins = 3.0 * x + y;

    num_coins as usize
}

fn bottom_up_dp(a: (usize, usize), b: (usize, usize), amount: (usize, usize)) -> usize {
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();

    // Base case
    map.insert((0, 0), 0);

    for dir in &[(a, 3), (b, 1)] {
        let x = dir.0.0;
        let y = dir.0.1;
        let w = dir.1;

        for i in x..=amount.0 {
            for j in y..=amount.1 {
                if let Some(&prev) = map.get(&(i - x, j - y)) {
                    let current = map.get(&(i, j)).cloned().unwrap_or(usize::MAX);
                    map.insert((i, j), min(current, prev + w));
                }
                // Early termination if we reach the target amount
                if i == amount.0 && j == amount.1 && map.get(&(i, j)).unwrap_or(&usize::MAX) != &usize::MAX {
                    return *map.get(&(i, j)).unwrap();
                }
            }
        }
    }

    *map.get(&amount).unwrap_or(&0)
}

fn bottom_up_dp_fast(a: (usize, usize), b: (usize, usize), amount: (usize, usize)) -> usize {
    let mut dp = vec![vec![usize::MAX; amount.1 + 1]; amount.0 + 1];
    dp[0][0] = 0;

    for dir in &[(a, 3), (b, 1)] {
        let x = dir.0.0;
        let y = dir.0.1;
        let w = dir.1;

        for i in x..=amount.0 {
            for j in y..=amount.1 {
                if i >= x && j >= y && dp[i - x][j - y] != usize::MAX {
                    dp[i][j] = min(dp[i][j], dp[i - x][j - y] + w);
                }
            }
        }
    }

    let amount = dp[amount.0][amount.1];

    if amount == usize::MAX {
        0
    } else {
        amount
    }
}

fn main() {
    let mut haystack = String::new();

    fs::File::open("src/bin/day13/input.txt").unwrap().read_to_string(&mut haystack).unwrap();
    let re = Regex::new(r"(\d+)").unwrap();

    let mut part_one_total = 0;
    let mut part_two_total = 0;

    for num in &re.find_iter(&haystack).map(|v| v.as_str().parse::<isize>().unwrap()).chunks(6) {
        let (a_x, a_y, b_x, b_y, prize_x, prize_y) = num.collect_tuple().unwrap();
        part_one_total += simultaneous_equations((a_x, a_y), (b_x, b_y), (prize_x, prize_y));
        part_two_total += simultaneous_equations((a_x, a_y), (b_x, b_y), (prize_x+10000000000000, prize_y+10000000000000));
    }

    println!("Number of coins: {part_one_total}");
    println!("Part B: Number of coins: {part_two_total}");
}
