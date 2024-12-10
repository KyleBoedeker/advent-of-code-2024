use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut safe_count = 0;

    let mut lines = io::stdin().lock().lines();
    'report: while let Some(Ok(report)) = lines.next() {
        let levels: Vec<i32> = report.split(' ').map(|v| v.parse().unwrap()).collect();

        let increasing = levels[1] > levels[0];
        for x_xp1 in levels.windows(2) {
            let (x, xp1) = (x_xp1[0], x_xp1[1]);
            let delta = xp1 - x;

            // increasing/decreasing trend continues
            if increasing && delta < 0 || !increasing && delta > 0 {
                continue 'report;
            }

            // delta between elements is within bounds
            if !((1..=3).contains(&delta.abs())) {
                continue 'report;
            }
        }
        safe_count += 1;
    }
    println!("{} reports are safe", safe_count);
}
