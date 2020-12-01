use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("==== Part 1 ====");
    part1();
    println!("==== Part 2 ====");
    part2();
}

fn part1() {
    // let contents = fs::read_to_string("src/input.txt".to_string()).expect("Failed to read file.");
    let numbers: Vec<i32> = read_numbers("src/input.txt");
    let target = 2020;
    for outer in &numbers {
        for inner in &numbers {
            // TODO: Could not repeat yourself here, because we get the same answers in reverse as well currently.
            if outer + inner == target {
                println!("{}+{}={}", outer, inner, target);
                let mult = outer * inner;
                println!("{}*{}={}", outer, inner, mult);
            }
        }
    }
}

fn part2() {
    // let contents = fs::read_to_string("src/input.txt".to_string()).expect("Failed to read file.");
    let numbers: Vec<i32> = read_numbers("src/input.txt");
    let target = 2020;
    for outer in &numbers {
        for inner in &numbers {
            for extra_inner in &numbers {
                // TODO: Could not repeat yourself here, because we get the same answers in reverse as well currently.
                if outer + inner + extra_inner == target {
                    println!("{}+{}+{}={}", outer, inner, extra_inner, target);
                    let mult = outer * inner * extra_inner;
                    println!("{}*{}*{}={}", outer, inner, extra_inner, mult);
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_numbers<P>(filename: P) -> Vec<i32>
where
    P: AsRef<Path>,
{
    let mut result: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(number_str) = line {
                let number: i32 = number_str.parse().unwrap();
                result.push(number);
            }
        }
    }
    result
}
