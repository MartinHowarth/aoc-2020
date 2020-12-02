use recap::Recap;
use serde::Deserialize;
use utils;

#[derive(Debug, Clone, Deserialize, Recap)]
#[recap(regex = r"(?P<first>.*)-(?P<second>.*) (?P<char>[a-z]): (?P<password>.*)")]
struct DatabaseEntry {
    first: i32,
    second: i32,
    char: char,
    password: String,
}

fn parse_line(line: String) -> DatabaseEntry {
    let entry: DatabaseEntry = line.parse().ok().unwrap();
    entry
}

fn validate_entry_part_1(entry: DatabaseEntry) -> bool {
    (entry.first..=entry.second).contains(&(entry.password.matches(entry.char).count() as i32))
}

fn part_1(entries: Vec<DatabaseEntry>) {
    let valid = entries
        .iter()
        .map(|entry| validate_entry_part_1(entry.to_owned()))
        .filter(|is_valid| *is_valid);
    println!("{}", valid.count())
}

fn validate_entry_part_2(entry: DatabaseEntry) -> bool {
    (entry
        .password
        .chars()
        .nth((entry.first - 1) as usize)
        .unwrap()
        == entry.char)
        ^ (entry
            .password
            .chars()
            .nth((entry.second - 1) as usize)
            .unwrap()
            == entry.char)
}

fn part_2(entries: Vec<DatabaseEntry>) {
    let valid = entries
        .iter()
        .map(|entry| validate_entry_part_2(entry.to_owned()))
        .filter(|is_valid| *is_valid);
    println!("{}", valid.count())
}

fn main() {
    let filename = "day2/src/input.txt";
    let lines = utils::read_lines(filename);
    // println!("{}", lines.len());
    let entries: Vec<DatabaseEntry> = lines
        .iter()
        .map(|line| parse_line(line.to_string()))
        .collect();
    part_1(entries.clone());
    part_2(entries.clone());
}
