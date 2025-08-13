use regex::Regex;
use std::io::{self, BufRead};

fn concat_u64(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse::<u64>().unwrap()
}

// Exponential time algo since it's dirt simple and the input's not gonna get
// big enough to matter
fn is_calibratable(values: &Vec<u64>, calibration_target: u64) -> bool {
    let (a, b) = (values[0], values[1]);
    let mut sieve = vec![a * b, a + b, concat_u64(a, b)];

    // iterate over remaining elements in the calibration vector
    for b in values.iter().skip(2) {
        let mut new_sieve = vec![];
        for a in sieve.iter() {
            new_sieve.push(a + b);
            new_sieve.push(a * b);
            new_sieve.push(concat_u64(*a, *b));
        }
        // update the sieve and do it all again for the next value
        sieve = new_sieve;
    }

    sieve.contains(&calibration_target)
}

fn main() {
    let re = Regex::new(r"(\d+)").unwrap();

    let mut total_calibration_result = 0;

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        // get all nums as an array
        let mut nums: Vec<u64> = re
            .find_iter(&line)
            .filter_map(|m| m.as_str().parse().ok())
            .collect();

        // first element's the calibration target
        let calibration_target = nums.remove(0);

        if is_calibratable(&nums, calibration_target) {
            total_calibration_result += calibration_target;
        }
    }

    println!("total_calibration_result = {}", total_calibration_result);
}
