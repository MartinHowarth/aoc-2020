use reduce::Reduce;
use utils;

fn parse_line(line: &String) -> Vec<bool> {
    line.chars().map(|c| c == '#').collect()
}

fn tree_at_position(terrain: Vec<bool>, position: usize) -> bool {
    terrain[position % terrain.len()]
}

fn find_trees_on_slope(lines: &Vec<String>, right: usize, down: usize) -> usize {
    let mut count = 0;
    let mut position = 0;
    for (i, line) in (&lines).iter().enumerate() {
        if i % down != 0 {
            continue;
        }
        if tree_at_position(parse_line(line), position) {
            count += 1;
        }
        position += right;
    }
    count
}

fn part_1(lines: &Vec<String>) {
    println!("{}", find_trees_on_slope(&lines, 3, 1))
}

fn part_2(lines: &Vec<String>) {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let tree_hits = slopes
        .iter()
        .map(|pair| find_trees_on_slope(&lines, pair.0, pair.1));
    println!("{}", tree_hits.reduce(|a, b| a * b).unwrap())
}

fn main() {
    let filename = "src/input.txt";
    let lines = utils::read_lines(filename);
    part_1(&lines);
    part_2(&lines);
}
