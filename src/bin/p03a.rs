use regex::Regex;
use std::io::{self, BufRead};

fn main() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut total: u32 = 0;

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        for (_, [x, y]) in re.captures_iter(&line).map(|c| c.extract()) {
            total += x.parse::<u32>().unwrap() * y.parse::<u32>().unwrap();
        }
    }
    println!("total is: {}", total);
}
