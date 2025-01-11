use std::io::{self, BufRead};

fn is_report_safe(report: &Vec<i32>) -> bool {
    let increasing = report[1] > report[0];

    for x_xp1 in report.windows(2) {
        let (x, xp1) = (x_xp1[0], x_xp1[1]);
        let delta = xp1 - x;

        // increasing/decreasing trend continues
        if increasing && delta < 0 || !increasing && delta > 0 {
            return false;
        }

        // delta between elements is within bounds
        if !((1..=3).contains(&delta.abs())) {
            return false;
        }
    }
    true
}

fn main() {
    let mut safe_count = 0;

    let mut lines = io::stdin().lock().lines();
    'report: while let Some(Ok(report)) = lines.next() {
        let report: Vec<i32> = report.split(' ').map(|v| v.parse().unwrap()).collect();

        if is_report_safe(&report) {
            safe_count += 1;
            continue;
        }

        // Brute force (generate every set of reports with one element missing and test if it's safe)
        for idx in 0..report.len() {
            let tmp_report = report
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != idx)
                .map(|(_, v)| *v)
                .collect();
            if is_report_safe(&tmp_report) {
                safe_count += 1;
                continue 'report;
            }
        }
    }
    println!("{} reports are safe", safe_count);
}
