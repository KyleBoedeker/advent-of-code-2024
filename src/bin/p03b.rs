use regex::Regex;
use std::io;

fn main() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut total = 0;
    let mut instruction_enabled = true;

    let stdin = io::read_to_string(io::stdin()).unwrap();
    for m in re.captures_iter(&stdin) {
        if m[0].starts_with("don't") {
            instruction_enabled = false;
        } else if m[0].starts_with("do") {
            instruction_enabled = true;
        } else if instruction_enabled {
            total += m[1].parse::<u32>().unwrap() * m[2].parse::<u32>().unwrap();
        }
    }
    println!("total is: {}", total);
}
