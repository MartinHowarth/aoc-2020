use std::fmt;
use utils;

#[derive(Default, Clone, Copy)]
struct Passport {
    byr: bool, //Option<usize>,
    iyr: bool, //Option<usize>,
    eyr: bool, //Option<usize>,
    hgt: bool, //Option<usize>,
    hcl: bool, //Option<String>,
    ecl: bool, //Option<String>,
    pid: bool, //Option<String>,
    cid: bool, //Option<usize>,
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
            println!("{}", word);
            let field = word.split(':').nth(0).unwrap();
            // let value = word.split(':').nth(1).unwrap();
            match field {
                "byr" => self.byr = true,
                "iyr" => self.iyr = true,
                "eyr" => self.eyr = true,
                "hgt" => self.hgt = true,
                "hcl" => self.hcl = true,
                "ecl" => self.ecl = true,
                "pid" => self.pid = true,
                "cid" => self.cid = true,
                _ => panic!("unknown field"),
            }
        }
    }

    fn part1_valid(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid

        // [
        //     self.byr, //.is_some(),
        //     self.iyr, //.is_some(),
        //     self.eyr, //.is_some(),
        //     self.hgt, //.is_some(),
        //     self.hcl, //.is_some(),
        //     self.ecl, //.is_some(),
        //     self.pid, //.is_some(),
        //               // self.cid.is_some(),  // country ID missing is totally legit *wink*
        // ]
        // .iter()
        // .all(|a| *a)
    }
}

fn part_1(lines: &Vec<String>) {
    let mut count = 0;
    let mut passport: Passport = Passport::default();
    for line in lines {
        if line.is_empty() {
            println!("Finished passport parse: {}", passport);
            if passport.part1_valid() {
                count += 1;
            }
            passport = Passport::default();
        }
        passport.parse_line(&line);
    }
    println!("Finished passport parse: {}", passport);
    if passport.part1_valid() {
        count += 1;
    }
    println!("{}", count);
}

// fn part_2(lines: &Vec<String>) {}

fn main() {
    let filename = "day4/src/input.txt";
    let lines = utils::read_lines(filename);
    part_1(&lines);
    // part_2(&lines);
}
