use std::collections::{HashMap, HashSet};
use std::process::Command;
use std::thread;
use std::time::Duration;
use itertools::Itertools;
use rand::Rng;
use termion::color;
use termion::event::Key::PageDown;
use advent_of_code_2024::read_puzzle_input;

struct Robot {
    x: isize,
    y: isize,
    v_x: isize,
    v_y: isize,

    width: isize,
    height: isize,
}

impl Robot {
    fn from_tuple(input: (isize, isize, isize, isize), width: isize, height: isize) -> Robot {
        Self {
            x: input.0,
            y: input.1,
            v_x: input.2,
            v_y: input.3,
            width,
            height,
        }
    }

    fn update_pos(&mut self) {
        // Modulus = ((a % b) + b) % b
        self.x = (((self.x + self.v_x) % self.width) + self.width) % self.width;
        self.y = (((self.y + self.v_y) % self.height) + self.height) % self.height;
    }

    fn get_quadrant(&self) -> Option<usize> {
        let middle_x = (self.width / 2);
        let middle_y = (self.height / 2);

        if self.x < middle_x && self.y < middle_y {
            Some(0) // Top-left
        } else if self.x > middle_x && self.y < middle_y {
            Some(1) // Top-right
        } else if self.x < middle_x && self.y > middle_y {
            Some(2) // Bottom-left
        } else if self.x > middle_x && self.y > middle_y {
            Some(3) // Bottom-right
        } else {
            None
        }
    }
}

fn simulate(robots: &mut Vec<Robot>, steps: usize, print: bool, stop_on_tree: bool) {
    for step in 1..steps+1 {
        for robot in robots.iter_mut() {
            robot.update_pos();
        }

        if print {
            if stop_on_tree {
                if check_possible_christmas_tree(robots) {
                    print_robots(robots);
                    println!("Christmas tree found {step}");
                    thread::sleep(Duration::from_secs(1));
                }
            } else {
                thread::sleep(Duration::from_millis(300)); // Adjust the duration as needed
            }
        }
    }
}

fn get_safety_factor(robots: &Vec<Robot>) -> usize {
    let mut counter: HashMap<usize, usize> = HashMap::new();

    robots.iter().filter_map(|r| r.get_quadrant()).for_each(|v| *counter.entry(v).or_insert(0) += 1);
    println!("{:?}", counter);
    println!("Sum: {}", counter.values().sum::<usize>());

    counter.into_values().product()
}

fn random_color() -> (color::Rgb, color::Rgb) {
    let mut rng = rand::thread_rng();
    let fg = color::Rgb(rng.gen(), rng.gen(), rng.gen());
    let bg = color::Rgb(rng.gen(), rng.gen(), rng.gen());
    (fg, bg)
}

fn calc_density(points: &HashSet<(isize, isize)>, x1: isize, x2: isize, y1: isize, y2: isize) -> usize {
    let density= points.iter()
        .filter(|p| p.0 > x1 && p.0 < x2 && p.0 > y1 && p.1 < y2)
        .count();

    density
}

fn check_possible_christmas_tree(robots: &Vec<Robot>) -> bool {
    let mut positions: HashSet<(isize, isize)> = HashSet::new();

    let height = robots.first().unwrap().height;
    let width = robots.first().unwrap().width;

    for robot in robots {
        positions.insert((robot.x, robot.y));
    }

    let box_height = height/4;
    let box_width = height/4;

    for y in 0..8 {
        for x in 0..8 {
            let x= x * box_width/2;
            let y= y * box_height/2;

            if y + box_height > height || x + box_width > width {
                continue
            }

            if calc_density(&positions, x, x+box_width, y, y+box_height) > 200 {
                println!("Found it");
                return true
            }
        }
    }

    false
}

fn print_robots(robots: &Vec<Robot>) {
    let mut positions: HashMap<(isize, isize), usize> = HashMap::new();

    let height = robots.first().unwrap().height;
    let width = robots.first().unwrap().width;

    for robot in robots {
        *positions.entry((robot.x, robot.y)).or_insert(0) += 1;
    }

    // Clear the terminal
    Command::new("clear").status().unwrap();

    for y in 0..height {
        for x in 0..width {
            if let Some(&count) = positions.get(&(x, y)) {
                let (fg, bg) = random_color();
                print!("{}{}{}{}{}", color::Fg(fg), color::Bg(bg), count, color::Fg(color::Reset), color::Bg(color::Reset));
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let lines = read_puzzle_input("./src/bin/day14/input.txt").unwrap();
    let mut robots: Vec<Robot> = Vec::new();

    for line in lines.map_while(|l| l.ok()) {
        let line = line.replace("p=", "").replace(" v=", ",");

        let (x, y, v_x, v_y) = line.split(",").map(|n| n.parse::<isize>().unwrap()).collect_tuple().unwrap();
        robots.push(Robot::from_tuple((x, y, v_x, v_y), 101, 103));
    }

    simulate(&mut robots, 30000, true, true);

    // print_robots(&robots);
    // println!("{}", get_safety_factor(&robots))
}
