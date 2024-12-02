#![feature(iter_map_windows)]

use std::io::stdin;

pub fn test() -> String {
    String::from("Hello World!")
}

fn day_one_input() -> (Vec<u64>, Vec<u64>) {
    let (mut l1, mut l2) = (Vec::<u64>::new(), Vec::<u64>::new());

    for line in stdin().lines().map(Result::unwrap) {
        
        let mut line_iter = line.split_whitespace();
        l1.push(line_iter.next().unwrap().parse().unwrap());
        l2.push(line_iter.next().unwrap().parse().unwrap());
    }

    (l1, l2)
}

pub fn day_one_first() -> u64 {
    let (mut l1, mut l2) = day_one_input();

    l1.sort();
    l2.sort();

    std::iter::zip(l1, l2)
        .map(|(l, r)| u64::abs_diff(l, r))
        .sum()
}

pub fn day_one_second() -> u64 {
    let (l1, l2) = day_one_input();

    l1
        .iter()
        .map(|l| l * l2.iter()
            .filter(|r| *l == **r)
            .count() as u64
        )
        .sum()
}

pub fn day_two_first() -> i64 {
    let mut count = 0;

    for line in stdin().lines().map(Result::unwrap) {
        let diffs: Vec<_> = line
            .split_whitespace()
            .map(str::parse::<i64>)
            .map(Result::unwrap)
            .map_windows(|[l, r]| l - r)
            .collect();

        if diffs.iter().all(|diff| (-3..0).contains(diff))
            || diffs.iter().all(|diff| (1..=3).contains(diff)) {
            count += 1;
        }
    }

    count
}

pub fn day_two_second() -> i64 {
    let mut count = 0;

    fn evaluate<I: Iterator<Item = i64> + Clone>(iter: &I) -> bool {
        iter.clone().all(|diff| (-3..0).contains(&diff))
            || iter.clone().all(|diff| (1..=3).contains(&diff))
    }

    'outer: for line in stdin().lines().map(Result::unwrap) {
        let diffs: Vec<_> = line
            .split_whitespace()
            .map(str::parse::<i64>)
            .map(Result::unwrap)
            .map_windows(|[l, r]| l - r)
            .collect();

        let len = diffs.iter().count();
        let increasing: Vec<_> = diffs.iter().map(|diff| (-3..0).contains(diff)).collect();
        let decreasing: Vec<_> = diffs.iter().map(|diff| (1..=3).contains(diff)).collect();

        let count_inc = usize::abs_diff(len, increasing.iter().filter(|b| **b).count());
        let count_dec = usize::abs_diff(len, decreasing.iter().filter(|b| **b).count());

        if count_inc == 0 || count_dec == 0 {
            count += 1;
        } else {
            for idx in 0..=diffs.len() {
                let mut copy = diffs.clone();

                if idx == copy.len() { // !! Horrible Hack Alert !!
                    copy.remove(0); // I should have worked on the input vec instead of diffs
                } else if idx != copy.len() - 1 {
                    copy[idx+1] += copy[idx];
                    copy.remove(idx);
                } else {
                    copy.remove(idx);
                }

                if evaluate(&copy.iter().cloned()) {
                    count += 1;
                    continue 'outer;
                }
            }
        }
    }

    count
}