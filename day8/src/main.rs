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
    fn perform_next_instruction(&mut self) -> bool {
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
            if self.perform_next_instruction() {
                return self.accumulator;
            }
        }
    }

    fn run_part_2(&mut self) -> i32 {
        loop {
            if self.perform_next_instruction() {
                return self.accumulator;
            }
        }
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

fn part_2(lines: &Vec<String>) {}

fn main() {
    let filename = "day8/src/input.txt";
    let lines = utils::read_lines(filename);
    part_1(&lines);
    part_2(&lines);
}
