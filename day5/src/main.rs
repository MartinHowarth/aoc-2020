use utils;

fn bool_vec_to_int(vec: &Vec<bool>) -> usize {
    let base: usize = 2;
    vec.iter().rev().enumerate().fold(0, |acc, (count, &b)| {
        acc + (b as usize * base.pow(count as u32))
    })
}

fn get_row_num(line: &String) -> usize {
    let boolvec: Vec<bool> = line.chars().map(|ch| ch == 'B').collect();
    return bool_vec_to_int(&boolvec);
}

fn get_column_num(line: &String) -> usize {
    let boolvec: Vec<bool> = line.chars().map(|ch| ch == 'R').collect();
    return bool_vec_to_int(&boolvec);
}

fn get_seat_id(line: &String) -> usize {
    let row_str = line[..7].to_string();
    let column_str = line[7..].to_string();
    let row_num = get_row_num(&row_str);
    let col_num = get_column_num(&column_str);
    println!("{}-{}", row_num, col_num);
    return row_num * 8 + col_num;
}

fn part_1(lines: &Vec<String>) {
    println!(
        "{}",
        lines.iter().map(|line| get_seat_id(&line)).max().unwrap()
    )
}

fn part_2(lines: &Vec<String>) {
    let mut all_ids: Vec<usize> = lines.iter().map(|line| get_seat_id(&line)).collect();
    all_ids.sort();
    let mut last_id: usize = 0;
    for id in all_ids {
        if last_id == 0 {
            last_id = id;
            continue;
        }
        if last_id != id - 1 {
            println!("{} is missing.", id - 1);
        }
        last_id = id;
    }
}

fn main() {
    let filename = "day5/src/input.txt";
    let lines = utils::read_lines(filename);
    part_1(&lines);
    part_2(&lines);
}
