use std::collections::{HashMap, HashSet};

use utils;

struct Instruction {
    name: String,
    value: i32,
}

struct Program {
    accumulator: i32,
    pointer: i32,
    instructions: Vec<Instruction>,
    called_instructions: HashSet<i32>,
}

impl Program {
    fn perform_next_instruction_part_1(&mut self) -> bool {
        if self.called_instructions.contains(&self.pointer) {
            return true;
        }
        self.called_instructions.insert(self.pointer);

        let instruction = self.instructions.get(self.pointer as usize).unwrap();
        if instruction.name == "acc" {
            self.accumulator += instruction.value;
            self.pointer += 1;
        } else if instruction.name == "jmp" {
            self.pointer += instruction.value;
        } else if instruction.name == "nop" {
            self.pointer += 1;
        }
        false
    }

    fn run_part_1(&mut self) -> i32 {
        loop {
            if self.perform_next_instruction_part_1() {
                return self.accumulator;
            }
        }
    }

    fn reset(&mut self) {
        self.accumulator = 0;
        self.pointer = 0;
        self.called_instructions = HashSet::new();
    }

    fn perform_next_instruction_part_2(&mut self, corrupted_test: i32) -> bool {
        if self.called_instructions.contains(&self.pointer) {
            return true;
        }
        self.called_instructions.insert(self.pointer);

        let instruction = self.instructions.get(self.pointer as usize).unwrap();
        if instruction.name == "acc" {
            self.accumulator += instruction.value;
            self.pointer += 1;
        } else if instruction.name == "jmp" {
            if corrupted_test == self.pointer {
                // do "nop" instead
                self.pointer += 1;
            } else {
                self.pointer += instruction.value;
            }
        } else if instruction.name == "nop" {
            if corrupted_test == self.pointer {
                // do "jmp" instead
                self.pointer += instruction.value;
            } else {
                self.pointer += 1;
            }
        }
        false
    }

    fn run_part_2(&mut self) -> i32 {
        for corrupt_test in (0..self.instructions.len()) {
            loop {
                if self.perform_next_instruction_part_2(corrupt_test as i32) {
                    println!("{} is still infinite", corrupt_test);
                    self.reset();
                    break;
                }
                if self.pointer == self.instructions.len() as i32 {
                    println!("{} broke out!", corrupt_test);
                    return self.accumulator;
                }
            }
        }
        0
    }
}

fn parse_lines(lines: &Vec<String>) -> Program {
    Program {
        accumulator: 0,
        pointer: 0,
        instructions: lines
            .iter()
            .map(|line| Instruction {
                name: line.split_ascii_whitespace().next().unwrap().to_string(),
                value: line
                    .split_ascii_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap(),
            })
            .collect(),
        called_instructions: HashSet::new(),
    }
}

fn part_1(lines: &Vec<String>) {
    println!("Part 1: {}", parse_lines(lines).run_part_1())
}

fn part_2(lines: &Vec<String>) {
    println!("Part 2: {}", parse_lines(lines).run_part_2())
}

fn main() {
    let filename = "day8/src/input.txt";
    let lines = utils::read_lines(filename);
    part_1(&lines);
    part_2(&lines);
}
