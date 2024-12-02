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