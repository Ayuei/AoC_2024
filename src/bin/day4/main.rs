use untitled2::read_puzzle_input;

fn search_candidate_x(input: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let y_boundary = input.len();
    let x_boundary = input[0].len();

    let word = vec!['X', 'M', 'A', 'S'];

    // Search 8 directions
    let mut directions = vec![0; 8];

    for i in 1..4 {
        // Right -> -> ->
        if x + i < x_boundary && input[y][x + i] == *word.get(i).unwrap() {
            directions[0] += 1;
        }

        // Left <- <- <-
        if x >= i && input[y][x - i] == *word.get(i).unwrap() {
            directions[1] += 1;
        }

        // Down
        if y + i < y_boundary && input[y + i][x] == *word.get(i).unwrap() {
            directions[2] += 1;
        }

        // Up
        if y >= i && input[y - i][x] == *word.get(i).unwrap() {
            directions[3] += 1;
        }

        // Down-Right
        if x + i < x_boundary && y + i < y_boundary && input[y + i][x + i] == *word.get(i).unwrap() {
            directions[4] += 1;
        }

        // Down-Left
        if x >= i && y + i < y_boundary && input[y + i][x - i] == *word.get(i).unwrap() {
            directions[5] += 1;
        }

        // Up-Right
        if x + i < x_boundary && y >= i && input[y - i][x + i] == *word.get(i).unwrap() {
            directions[6] += 1;
        }

        // Up-Left
        if x >= i && y >= i && input[y - i][x - i] == *word.get(i).unwrap() {
            directions[7] += 1;
        }
    }

    directions.into_iter().filter(|v| *v == 3).count()
}

fn search_candidate_a(input: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let y_boundary = input.len();
    let x_boundary = input[0].len();
    let mut count = 0;
    let i = 1;

    // Each diagonal must have an M and an S
    if x + i < x_boundary && y + i < y_boundary && x >= i && y >= i {
        let down_right = input[y + i][x + i];
        let up_left = input[y - i][x - i];

        if down_right != up_left && (down_right == 'M' || down_right == 'S') && (up_left == 'M' || up_left == 'S') {
            count += 1 ;
        }
    }

    if x >= i && y + i < y_boundary && x + i < x_boundary && y >= i {
        let down_left = input[y + i][x - i];
        let up_right = input[y - i][x + i];

        if down_left != up_right && (down_left == 'M' || down_left == 'S') && (up_right == 'M' || up_right == 'S') {
            count += 1 ;
        }
    }

    (count == 2) as usize
}

fn main() {
    let mut puzzle : Vec<Vec<char>>= Vec::new();

    for line in read_puzzle_input("src/bin/day4/input.txt").unwrap() {
        puzzle.push(line.unwrap().chars().collect());
    }

    let mut part_one_total = 0;
    let mut part_two_total = 0;

    for y in 0..puzzle.len() {
        for x in 0..puzzle[y].len() {
            if 'X' == puzzle[y][x] {
                part_one_total += search_candidate_x(&puzzle, x, y);
            }

            if 'A' == puzzle[y][x] {
                part_two_total += search_candidate_a(&puzzle, x, y);
            }
        }
    }

    println!("Part one {}", part_one_total);
    println!("Part two {}", part_two_total);
}
