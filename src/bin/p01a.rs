use regex::Regex;
use std::io::{self, BufRead};

fn main() {
    // "\d+" captures one or more digit chars
    // "\D+" captures one or more non-digit chars
    let re = Regex::new(r"(\d+)\D+(\d+)").unwrap();

    let mut v1: Vec<i32> = vec![];
    let mut v2: Vec<i32> = vec![];

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        if let Some(captures) = re.captures(&line) {
            v1.push(captures[1].parse().unwrap());
            v2.push(captures[2].parse().unwrap());
        }
    }
    v1.sort();
    v2.sort();

    let mut total = 0;
    for (val1, val2) in v1.iter().zip(v2) {
        total += (val2 - val1).abs();
    }
    println!("total = {}", total);
}
