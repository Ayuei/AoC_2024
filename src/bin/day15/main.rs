use advent_of_code_2024::read_puzzle_input;

#[derive(Copy, Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn check_direction(self, coord: (usize, usize), board: &Vec<Vec<char>>) {
        let mut obstacle = false;
        let mut cur = coord;

        let x_bound: usize = board.first().unwrap().first().unwrap().len();
        let y_bound = board.first().unwrap().len();

        loop {
            let Some(c) = self.apply_direction(coord, x_bound, y_bound).unwrap() else {
                break;
            };

            if c == 'o' {
                obstacle = true;
            }

            cur = c;
        }
    }

    fn apply_direction(self, coord: (usize, usize), x_bound: usize, y_bound: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if coord.1 < y_bound - 1 {
                    Some((coord.0, coord.1 + 1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if coord.1 > 0 {
                    Some((coord.0, coord.1 - 1))
                } else {
                    None
                }
            }
            Direction::Left => {
                if coord.0 > 0 {
                    Some((coord.0 - 1, coord.1))
                } else {
                    None
                }
            }
            Direction::Right => {
                if coord.0 < x_bound - 1 {
                    Some((coord.0 + 1, coord.1))
                } else {
                    None
                }
            }
        }
    }
}

fn print_board(board: Vec<Vec<char>>) {
    for row in board {
        for c in row {
            print!("{c}");
        }
        println!();
    }
}

fn main() {
    let lines= read_puzzle_input("src/bin/day15/input.txt").unwrap();
    let mut second_half = false;

    let mut board: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<Direction>= Vec::new();

    for line in lines.map_while(|l| l.ok()) {
        if line.is_empty() {
            second_half = true;
        }

        if !second_half {
            board.append(line.chars().collect())
        } else {
            moves.append(lines.chars().collect())
        }
    }
}
