use std::io::{self, BufRead};

use regex::Regex;

fn change_stones(stones: &mut Vec<u64>, new_stones: &mut Vec<u64>) {
    for stone in stones {
        // rule 1: replace 0 with 1
        if *stone == 0 {
            new_stones.push(1);
            continue;
        }
        // rule 2: split stones in half with even # of digits
        let stringy_stone = stone.to_string();
        if stringy_stone.len() % 2 == 0 {
            let (left, right) = stringy_stone.split_at(stringy_stone.len() / 2);
            new_stones.push(left.parse().unwrap());
            new_stones.push(right.parse().unwrap());
            continue;
        }
        // rule 3: multiply by 2024
        new_stones.push(*stone * 2024);
    }
}

fn main() {
    // read stdin
    let mut buffer = String::new();
    {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_line(&mut buffer).unwrap();
    }

    // Parse stdin as a list of numbers
    let re = Regex::new(r"(\d+)").unwrap();
    let mut stones: Vec<u64> = re
        .captures_iter(&buffer)
        .map(|v| v.get(0).unwrap().as_str().parse().unwrap())
        .collect();

    let mut new_stones = Vec::new();

    let num_blinks = 25;
    for _ in 0..num_blinks {
        // update stones into new_stones
        change_stones(&mut stones, &mut new_stones);
        // reset stones (no effect on allocated vec size)
        stones.clear();
        // swap such that stones has the changed copy and new_stones is empty
        std::mem::swap(&mut stones, &mut new_stones);
    }

    println!(
        "# of stones after blinking {} times: {}",
        num_blinks,
        stones.len()
    );
}
