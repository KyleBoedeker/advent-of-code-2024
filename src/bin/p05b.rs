use std::io::{self, BufRead};

fn fits_ordering(pages: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for (r1, r2) in rules {
        if let Some(idx1) = pages.iter().position(|v| v == r1) {
            if let Some(idx2) = pages.iter().position(|v| v == r2) {
                if idx1 > idx2 {
                    return false;
                }
            }
        }
    }
    return true;
}

// make a pass at sorting the vec (doesn't fix without calling it a bunch)
fn fix_ordering(pages: &mut Vec<i32>, rules: &Vec<(i32, i32)>) {
    for (r1, r2) in rules {
        if let Some(idx1) = pages.iter().position(|v| v == r1) {
            if let Some(idx2) = pages.iter().position(|v| v == r2) {
                if idx1 > idx2 {
                    let tmp = pages.remove(idx1);
                    pages.insert(idx2, tmp);
                }
            }
        }
    }
}

fn main() {
    let mut lines = io::stdin().lock().lines();

    let mut ordering_rules: Vec<(i32, i32)> = vec![];

    while let Some(Ok(line)) = lines.next() {
        if line == "" {
            break;
        }
        let (a, b) = line.split_once('|').unwrap();
        ordering_rules.push((a.parse().unwrap(), b.parse().unwrap()));
    }
    let mut total = 0;

    while let Some(Ok(line)) = lines.next() {
        let mut pages: Vec<i32> = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        if !fits_ordering(&pages, &ordering_rules) {
            // ooga-booga a few times till the page is definitely sorted (I hate sorting)
            for _ in 0..pages.len() {
                fix_ordering(&mut pages, &ordering_rules);
            }
            total += pages.iter().nth(pages.len() / 2).unwrap();
        }
    }

    println!("{}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering_works() {
        let rules = vec![(4, 5), (6, 7), (9, 10)];
        let pages = vec![9, 4, 5, 6, 10, 7];

        assert!(fits_ordering(&pages, &rules));

        // 4 and 5 are swapped
        let rules = vec![(4, 5), (6, 7), (9, 10)];
        let pages = vec![9, 5, 4, 6, 10, 7];

        assert!(!fits_ordering(&pages, &rules));
    }
}
