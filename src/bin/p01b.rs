use regex::Regex;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    // "\d+" captures one or more digit chars
    // "\D+" captures one or more non-digit chars
    let re = Regex::new(r"(\d+)\D+(\d+)").unwrap();

    let mut v1: Vec<i32> = vec![];
    let mut h2: HashMap<i32, i32> = HashMap::new();

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        if let Some(captures) = re.captures(&line) {
            v1.push(captures[1].parse().unwrap());
            h2.entry(captures[2].parse().unwrap())
                .and_modify(|v| *v += 1).or_insert(1);
        }
    }

    let mut total = 0;
    for v in v1.iter() {
        if let Some(count) = h2.get(v) {
            total += v * count;
        }
    }
    println!("total = {}", total);
}
