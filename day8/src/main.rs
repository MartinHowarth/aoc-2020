use std::collections::HashMap;

use utils;

struct Instruction {
    name: String,
    value: i32,
    call_count: usize,
}

struct Program {
    accumulator: i32,
    pointer: i32,
    instructions: Vec<Instruction>,
}

impl Program {
    fn perform_next_instruction(&mut self) -> bool {
        let mut instruction = self.instructions.get_mut(self.pointer as usize).unwrap();
        instruction.call_count += 1;
        if instruction.name == "acc" {
            self.accumulator += instruction.value;
            self.pointer += 1;
        } else if instruction.name == "jmp" {
            self.pointer += instruction.value;
        } else if instruction.name == "nop" {
            self.pointer += 1;
        }
        instruction.call_count > 1
    }

    fn run_part_1(&mut self) -> i32 {
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
                call_count: 0,
            })
            .collect(),
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
