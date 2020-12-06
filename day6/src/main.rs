use std::collections::HashSet;
use utils;

fn part_1(lines: &Vec<String>) {
    let mut total = 0;
    for group in utils::group_lines_split_by_empty_line(&lines) {
        let mut set = HashSet::<char>::new();
        set.extend(group.chars());
        total += set.len();
    }
    println!("{}", total);
}

pub fn group_lines_split_by_empty_line_and_count(
    lines: &Vec<String>,
) -> impl Iterator<Item = (usize, String)> + '_ {
    lines
        .split(|line| line == "")
        .map(|slice| (slice.len(), slice.join("")))
}

fn part_2(lines: &Vec<String>) {
    let mut total = 0;
    for (num_members, group) in group_lines_split_by_empty_line_and_count(&lines) {
        let mut set = HashSet::<char>::new();
        set.extend(group.chars());
        for letter in set {
            if group.matches(letter).count() == num_members {
                total += 1;
            }
        }
    }
    println!("{}", total);
}

fn main() {
    let filename = "day6/src/input.txt";
    let lines = utils::read_lines(filename);
    part_1(&lines);
    part_2(&lines);
}
