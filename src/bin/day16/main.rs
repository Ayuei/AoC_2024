use std::cmp::{Ordering, PartialEq, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use advent_of_code_2024::read_puzzle_input;


#[derive(PartialEq, Copy, Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn get_update(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1)
        }
    }
}

fn manhattan_distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

#[derive(Clone)]
struct Path {
    pub heuristic: usize,
    pub pos: (usize, usize),
    pub cost: usize,
    pub path: Vec<(usize, usize)>,
    pub visited: HashSet<(usize, usize)>,
    pub path_direction: Vec<Direction>,
    pub direction: Direction,
}

impl Path {
    fn new(heuristic: usize, pos: (usize, usize), cost: usize,
           path: Vec<(usize, usize)>, visited: HashSet<(usize, usize)>,
           path_direction: Vec<Direction>, direction: Direction) -> Path {

        Path {
            heuristic, pos, cost,
            path, visited,
            path_direction, direction
        }
    }

    fn get_cost(&self, new_dir: Direction) -> usize {
        let v1 = self.direction.get_update();
        let v2 = new_dir.get_update();

        if v1.0.abs_diff(v2.0) == 0 || v1.1.abs_diff(v2.1) == 0 {
            1
        } else {
            1001
        }
    }
}

impl Eq for Path {}

impl PartialEq<Self> for Path {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic.eq(&other.heuristic)
    }
}

impl PartialOrd<Self> for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.heuristic.partial_cmp(&other.heuristic)
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heuristic.cmp(&other.heuristic)
    }
}


// Find all the best paths
fn a_star_flood_fill(maze: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Vec<Path> {
    let height = maze.len();
    let width = maze.first().unwrap().len();

    let mut pr: BinaryHeap<Reverse<Path>> = BinaryHeap::new();
    let mut best_cost = usize::MAX;
    let mut best_paths = Vec::new();

    // Keep track of the best cost to reach a certain point
    let mut best_cost_for_p: HashMap<(usize, usize), usize> = HashMap::new();

    pr.push(Reverse(Path::new(0 + manhattan_distance(start, end), start, 0,
                              vec![start], HashSet::new(),
                              vec![Direction::Right], Direction::Right)));

    while !pr.is_empty() {
        let mut p = pr.pop().unwrap().0;

        if p.cost < *best_cost_for_p.entry(p.pos).or_insert(usize::MAX) {
            best_cost_for_p.insert(p.pos, p.cost + 1001);
        } else {
            continue
        }

        if p.pos == end {
            if best_cost == usize::MAX {
                best_cost = p.cost;
            }

            if p.cost == best_cost {
                best_paths.push(p.clone());
            } else {
                break
            }
        }

        for direction in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            match p.direction {
                Direction::Up => {
                    if direction == Direction::Down {
                        continue;
                    }
                }
                Direction::Down => {
                    if direction == Direction::Up {
                        continue;
                    }
                }
                Direction::Left => {
                    if direction == Direction::Right {
                        continue;
                    }
                }
                Direction::Right => {
                    if direction == Direction::Left {
                        continue;
                    }
                }
            }

            let (dy, dx) = direction.get_update();

            let row= p.pos.0 as isize + dy;
            let col= p.pos.1 as isize + dx;

            if row < height as isize && row > 0 && col > 0 && col < width as isize
                && !p.visited.contains(&(row as usize, col as usize)) {

                let row = row as usize;
                let col = col as usize;

                let c = maze[row][col];

                if c == '#' {
                    // Skip wall
                    continue;
                }

                let cost = p.get_cost(direction);

                let mut visited = p.visited.clone();
                visited.insert((row, col));

                let mut path: Vec<(usize, usize)> = p.path.iter().cloned().collect();
                path.push((row, col));

                let mut directions_path: Vec<Direction> = p.path_direction.iter().cloned().collect();
                directions_path.push(direction);

                pr.push(Reverse(Path::new(
                    p.cost + cost + manhattan_distance((row, col), end),
                    (row, col),
                    p.cost + cost,
                    path,
                    visited,
                    directions_path,
                    direction
                )));
            }
        }
    }

    best_paths
}

fn print_path(maze: &Vec<Vec<char>>, p: &Path) {
    println!("Path length: {}", p.path.len());
    println!("Path: {:?}", p.path_direction);

    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            let c = maze[row][col];

            if c == 'S' || c == 'E' {
                print!("{}", maze[row][col]);
            } else if p.path.contains(&(row, col)) {
                let index = p.path.iter().position(|x| x == &(row, col)).unwrap();
                let dir = p.path_direction[index];

                let dir_c = match dir {
                    Direction::Up => '^',
                    Direction::Down => 'v',
                    Direction::Left => '<',
                    Direction::Right => '>',
                };

                print!("{}", dir_c);
            } else {
                print!("{}", maze[row][col]);
            }
        }

        println!();
    }
}

fn print_union(maze: &Vec<Vec<char>>, union: HashSet<(usize, usize)>) {
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            let c = maze[row][col];

            if union.contains(&(row, col)) {
                print!("O");
            } else {
                print!("{c}");
            }
        }

        println!();
    }
}

fn path_union(paths: Vec<Path>) -> HashSet<(usize, usize)> {
    let mut union: HashSet<(usize, usize)> = HashSet::new();

    for path in paths {
        for p in path.path {
            union.insert(p);
        }
    }

    union
}

fn main() {
    let lines= read_puzzle_input("src/bin/day16/input.txt").unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in lines.map_while(|l| l.ok()).enumerate() {
        map.push(line.chars().collect());

        let start_position = line.chars().position(|p| p == 'S');
        let end_position = line.chars().position(|p| p == 'E');

        if let Some(x) = start_position {
            start = (x,y);
        }

        if let Some(x) = end_position {
            end = (x,y);
        }
    }

    let p = a_star_flood_fill(&map, end, start);

    //print_path(&map, p.first().unwrap());
    println!("{:?}", p.first().unwrap().cost);
    println!("{}", p.len());

    let union = path_union(p);
    print_union(&map, union.clone());

    println!("Part two: {}", union.len());
}