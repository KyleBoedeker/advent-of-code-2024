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

fn sort_pages(a: &i32, b: &i32, rules: &Vec<(i32, i32)>) -> std::cmp::Ordering {
    for (r1, r2) in rules.iter() {
        if a == r1 && b == r2 {
            return std::cmp::Ordering::Greater;
        }
        if a == r2 && b == r1 {
            return std::cmp::Ordering::Less;
        }
    }
    return std::cmp::Ordering::Equal;
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
            pages.sort_by(|a, b| sort_pages(a, b, &ordering_rules));
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
        // 4 and 5 are swapped
        let mut pages_unsorted = vec![9, 5, 4, 6, 10, 7];

        assert!(fits_ordering(&pages, &rules));
        assert!(!fits_ordering(&pages_unsorted, &rules));
        pages_unsorted.sort_by(|a, b| sort_pages(a, b, &rules));
        assert_eq!(pages_unsorted, pages);
        assert!(fits_ordering(&pages_unsorted, &rules));
    }
}
