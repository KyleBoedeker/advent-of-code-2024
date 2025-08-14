use std::collections::HashSet;
use std::io::{self, BufRead};
use std::iter;

fn main() {
    let line: String = io::stdin().lock().lines().next().unwrap().unwrap();

    let mut puz: Vec<i32> = Vec::with_capacity(line.len() * 9);

    for (idx, c) in line.chars().enumerate() {
        let isfree: bool = (idx % 2) == 1;
        let size = c.to_digit(10).unwrap() as usize;
        let id = ((idx) / 2) as i32;
        if isfree {
            puz.extend(iter::repeat_n(-1, size));
        } else {
            puz.extend(iter::repeat_n(id, size));
        }
    }

    let mut front = 0;
    let mut rear = puz.len() - 1;
    loop {
        if front == rear {
            break;
        }
        // end of puzzle is "blank"
        if puz[rear] == -1 {
            rear -= 1;
            continue;
        }
        if puz[front] != -1 {
            front += 1;
            continue;
        }
        // else "rear" is now a real element
        // and the "front" is now blank
        puz[front] = puz[rear];
        puz[rear] = -1;
        front += 1;
    }

    let mut checksum: u64 = 0;
    for (idx, block) in puz.iter().enumerate() {
        if *block == -1 {
            continue;
        }
        checksum += (*block as u64) * (idx as u64);
    }
    for block in puz.iter() {
        if *block == -1 {
            print!(".");
        } else {
            print!("{}", block);
        }
    }
    println!();
    println!("{}", checksum)
}
