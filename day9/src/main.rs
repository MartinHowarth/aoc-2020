use std::iter;
use std::{collections::HashSet, slice::Split};
use utils;

fn number_is_valid(number: &i64, preamble: &Vec<i64>) -> bool {
    for &pre in preamble {
        let diff = number - pre;
        if diff != pre && diff > 0 {
            if preamble.contains(&diff) {
                return true;
            }
        }
    }
    false
}

fn parse_lines(lines: &Vec<String>) -> Vec<i64> {
    lines
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

fn part_1(lines: &Vec<String>) {
    let numbers = parse_lines(lines);
    for (index, number) in numbers.iter().enumerate() {
        if index < 25 {
            continue;
        }
        if !number_is_valid(number, &numbers[index - 25..index].to_vec()) {
            println!("{} @ {} is not valid", number, index);
            break;
        }
    }
}

fn part_2(lines: &Vec<String>) {
    let numbers = parse_lines(lines);
    let invalid_num: i64 = 26134589;
    let invalid_index: usize = 503;
    let mut sum: i64 = 0;
    let mut upper_index: usize = 0;
    for index in (0..invalid_index).rev() {
        upper_index = index;
        loop {
            let sum = numbers[index..=upper_index].iter().sum::<i64>();
            if sum == invalid_num {
                println!("{}-{}", index, upper_index);
                println!(
                    "min: {} + max: {} = {}",
                    numbers[index..=upper_index].iter().min().unwrap(),
                    numbers[index..=upper_index].iter().max().unwrap(),
                    numbers[index..=upper_index].iter().min().unwrap()
                        + numbers[index..=upper_index].iter().max().unwrap(),
                )
            } else if sum > invalid_num {
                break;
            }
            upper_index += 1
        }
    }
}

fn main() {
    let filename = "day9/src/input.txt";
    let lines = utils::read_lines(filename);
    part_1(&lines);
    part_2(&lines);
}
