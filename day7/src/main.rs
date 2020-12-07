use std::collections::HashMap;
use std::fmt;
use utils;

struct Bag {
    colour: String,
    contents: HashMap<String, usize>,
}

impl fmt::Display for Bag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.colour,
            self.contents
                .iter()
                .fold("".to_string(), |total, (name, count)| format!(
                    "{}{}",
                    total,
                    &format!("{}: {}, ", name, count)
                ))
        )
    }
}

impl Bag {
    fn from_line(line: &String) -> Bag {
        let mut split = line.split(" bags contain ");
        let colour = split.next().unwrap();
        let contents_str = split.next().unwrap();

        if contents_str.contains("no other bags") {
            return Bag {
                colour: colour.to_string(),
                contents: HashMap::new(),
            };
        }

        let contents: HashMap<String, usize> = contents_str
            .split(", ")
            .map(|bag_type| {
                bag_type
                    .replace(" bags", "")
                    .replace(" bag", "")
                    .replace(".", "")
            })
            .map(|bag_type| {
                (
                    bag_type[2..].to_string(),
                    bag_type
                        .chars()
                        .nth(0)
                        .unwrap()
                        .to_string()
                        .parse::<usize>()
                        .unwrap(),
                )
            })
            .collect();
        Bag {
            colour: colour.to_string(),
            contents: contents,
        }
    }

    fn could_contain(&self, colour: &String, all_bags: &HashMap<String, Bag>) -> bool {
        if self.contents.contains_key(colour) {
            return true;
        }
        for bag in self.contents.keys() {
            if all_bags.get(bag).unwrap().could_contain(colour, all_bags) {
                return true;
            }
        }
        false
    }

    fn total_contents(&self, all_bags: &HashMap<String, Bag>) -> usize {
        let mut count = 0;
        for (bag, number) in self.contents.iter() {
            count += number * (all_bags.get(bag).unwrap().total_contents(all_bags) + 1);
        }
        count
    }
}

fn part_1(lines: &Vec<String>) {
    let mut count = 0;
    let all_bags: HashMap<String, Bag> = lines
        .iter()
        .map(|line| Bag::from_line(line))
        .map(|bag| (bag.colour.clone(), bag))
        .collect();
    for line in lines {
        // println!("{}", line);
        // println!("{}", Bag::from_line(line));
        if Bag::from_line(line).could_contain(&"shiny gold".to_string(), &all_bags) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn part_2(lines: &Vec<String>) {
    let all_bags: HashMap<String, Bag> = lines
        .iter()
        .map(|line| Bag::from_line(line))
        .map(|bag| (bag.colour.clone(), bag))
        .collect();

    println!(
        "{}",
        all_bags
            .get("shiny gold")
            .unwrap()
            .total_contents(&all_bags)
    );
}

fn main() {
    let filename = "day7/src/input.txt";
    let lines = utils::read_lines(filename);
    part_1(&lines);
    part_2(&lines);
}
