use std::collections::HashMap;
use advent_of_code_2024::read_puzzle_input;
use indicatif::{ProgressBar, ProgressStyle};

struct Registers {
    inst_ptr: usize,
    a: usize,
    b: usize,
    c: usize
}

impl Registers {
    fn new(a: usize) -> Self {
        Self {
            a,
            b: 0,
            c: 0,
            inst_ptr: 0
        }
    }

    fn reset(&mut self, a: usize) {
        self.inst_ptr = 0;

        self.a = a;
        self.b = 0;
        self.c = 0;
    }
}

fn perform_instruction(opcode: usize, operand:usize, register: &mut Registers, output: &mut Vec<usize>) {
    match opcode {
        0 => {
            // println!("adv: {operand} into A");
            register.a = register.a >> get_combo_operand_value(operand, register);
            register.inst_ptr += 2;
        }
        1 => {
            // println!("bxl: {operand} into B")
            register.b = register.b ^ operand;
            register.inst_ptr += 2
        }
        2 => {
            // println!("bst: {operand} into B");
            register.b = get_combo_operand_value(operand, register) & 7;
            register.inst_ptr += 2
        },
        3 => {
            if register.a == 0 {
                // println!("Ignore Jump: {operand}");
                register.inst_ptr += 2
            } else {
                // println!("Jump: {operand}");
                register.inst_ptr = operand;
            }
        }
        4 => {
            // println!("bxc: {operand}");
            register.b = register.b ^ register.c;
            register.inst_ptr += 2;
        }
        5 => {
            let combo_operand = get_combo_operand_value(operand, register);
            // println!("out: {operand} to combo {combo_operand}");
            output.push(combo_operand & 7);
            register.inst_ptr += 2;
        }
        6 => {
            let combo_operand = get_combo_operand_value(operand, register);
            // println!("bdv: {operand} to combo {combo_operand} to B");
            register.b = register.a >> combo_operand;
            register.inst_ptr += 2;
        }
        7 => {
            let combo_operand = get_combo_operand_value(operand, register);
            // println!("cdv: {operand} to combo {combo_operand} to C");

            register.c = register.a >> combo_operand;
            register.inst_ptr += 2;
        }
        _ => panic!("Unknown instruction val: {opcode}")
    }
}

fn get_combo_operand_value(operand: usize, registers: &Registers) -> usize {
    if operand < 4 {
        operand
    } else if operand == 4 {
        registers.a
    } else if operand == 5 {
        registers.b
    } else if operand == 6 {
        registers.c
    } else {
        panic!("Invalid combo operand: {operand}");
    }
}

fn brute_force(start: usize, end: usize, program: Vec<usize>) {
    let mut registers = Registers::new(0);

    let pb = ProgressBar::new((end-start) as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) ETA: {eta}").unwrap()
        .progress_chars("#>-"));

    for a_value in start..=end {
        registers.reset(a_value);

        let output = run_program(&program, &mut registers, true);
        // println!("A: {a_value} O: {:?}", output);

        if output == program {
            println!("Found at {a_value}");
            break;
        }

        pb.inc(1);
    }
}

fn run_program(program: &Vec<usize>, registers: &mut Registers, check_output: bool) -> Vec<usize> {
    let mut output = Vec::new();

    while registers.inst_ptr < program.len() {
        let opcode = program[registers.inst_ptr];
        let operand= program[registers.inst_ptr+1];

        // println!("Pointer: {:?} Opcode: {opcode}, operand: {operand}", program_ptr);
        // println!("Registers {:?}", registers);

        perform_instruction(opcode, operand, registers, &mut output);

        if check_output && output.len() > 0 && (program[output.len() - 1] != output[output.len()-1]) {
            break;
        }

        // println!("=======");
    }

    output
}

fn main() {
    let lines = read_puzzle_input("src/bin/day17/input.txt").unwrap();
    let mut registers: HashMap<String, usize> = HashMap::new();

    let lines: Vec<String> = lines.map_while(|v| v.ok()).collect();

    registers.insert("A".into(), lines[0].split_ascii_whitespace().last().unwrap().parse::<usize>().unwrap());
    registers.insert("B".into(), lines[1].split_ascii_whitespace().last().unwrap().parse::<usize>().unwrap());
    registers.insert("C".into(), lines[2].split_ascii_whitespace().last().unwrap().parse::<usize>().unwrap());

    let program: Vec<usize> = lines.last().unwrap().split_ascii_whitespace().last().unwrap().split(',').map(|v| v.parse().unwrap()).collect();

    let start= 2_i64.pow(45);
    let end = 2_i64.pow(48);

    brute_force(start as usize, end as usize, program);

    //println!("{:?}", registers);
    //println!("{:?}", output);
    //println!("{}", output.iter().join(","));
    //println!("{}", output.len());
}