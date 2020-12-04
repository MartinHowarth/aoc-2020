use std::fmt;
use utils;

#[derive(Default, Clone)]
struct Passport {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: usize,
}

impl fmt::Display for Passport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "byr:{}, iyr:{}, eyr:{}, hgt:{}, hcl:{}, ecl:{}, pid:{}, cid:{}",
            self.byr, self.iyr, self.eyr, self.hgt, self.hcl, self.ecl, self.pid, self.cid
        )
    }
}

impl Passport {
    fn parse_line(&mut self, line: &String) {
        for word in line.split_ascii_whitespace().into_iter() {
            let field = word.split(':').nth(0).unwrap();
            let value = word.split(':').nth(1).unwrap();
            match field {
                "byr" => self.byr = value.parse::<usize>().unwrap(),
                "iyr" => self.iyr = value.parse::<usize>().unwrap(),
                "eyr" => self.eyr = value.parse::<usize>().unwrap(),
                "hgt" => self.hgt = value.parse::<String>().unwrap(),
                "hcl" => self.hcl = value.parse::<String>().unwrap(),
                "ecl" => self.ecl = value.parse::<String>().unwrap(),
                "pid" => self.pid = value.parse::<String>().unwrap(),
                "cid" => self.cid = value.parse::<usize>().unwrap(),
                _ => panic!("unknown field"),
            }
        }
    }

    fn part1_valid(&self) -> bool {
        self.byr != 0
            && self.iyr != 0
            && self.eyr != 0
            && !self.hgt.is_empty()
            && !self.hcl.is_empty()
            && !self.ecl.is_empty()
            && !self.pid.is_empty()
    }

    fn part2_hgt_valid(&self) -> bool {
        if self.hgt.contains("in") {
            return (59..=76).contains(&self.hgt.replace("in", "").parse::<usize>().unwrap());
        } else if self.hgt.contains("cm") {
            return (150..=193).contains(&self.hgt.replace("cm", "").parse::<usize>().unwrap());
        }
        false
    }

    fn part2_hcl_valid(&self) -> bool {
        self.hcl.len() == 7
            && self.hcl.chars().nth(0).unwrap() == '#'
            && self.hcl[1..]
                .chars()
                .all(|ch| "0123456789abcdef".contains(ch))
    }

    fn part2_ecl_valid(&self) -> bool {
        [
            "amb".to_string(),
            "blu".to_string(),
            "brn".to_string(),
            "gry".to_string(),
            "grn".to_string(),
            "hzl".to_string(),
            "oth".to_string(),
        ]
        .contains(&self.ecl)
    }

    fn part2_pid_valid(&self) -> bool {
        self.pid.len() == 9 && self.pid.parse::<usize>().is_ok()
    }

    fn part2_valid(&self) -> bool {
        (1920..=2002).contains(&self.byr)
            && (2010..=2020).contains(&self.iyr)
            && (2020..=2030).contains(&self.eyr)
            && self.part2_hgt_valid()
            && self.part2_hcl_valid()
            && self.part2_ecl_valid()
            && self.part2_pid_valid()
    }
}

fn discover_passports(lines: &Vec<String>) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();
    let mut passport: Passport = Passport::default();
    for line in lines {
        if line.is_empty() {
            println!("Finished passport parse: {}", passport);
            passports.push(passport);
            passport = Passport::default();
        }
        passport.parse_line(&line);
    }
    passports.push(passport);
    passports
}

fn part_1(lines: &Vec<String>) {
    let count: usize = discover_passports(lines)
        .iter()
        .filter(|pp| pp.part1_valid())
        .count();
    println!("{}", count);
}

fn part_2(lines: &Vec<String>) {
    let count: usize = discover_passports(lines)
        .iter()
        .filter(|pp| pp.part2_valid())
        .count();
    println!("{}", count);
}

fn main() {
    let filename = "day4/src/input.txt";
    let lines = utils::read_lines(filename);
    part_1(&lines);
    part_2(&lines);
}
