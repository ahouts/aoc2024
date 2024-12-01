use std::cmp::Ordering;

use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i64 {
    let (mut list1, mut list2) = parse_input(input.as_bytes());
    list1.sort_unstable();
    list2.sort_unstable();
    list1
        .into_iter()
        .zip(list2)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i64 {
    let (mut list1, mut list2) = parse_input(input.as_bytes());
    list1.sort_unstable_by(|a, b| a.cmp(b).reverse());
    list2.sort_unstable_by(|a, b| a.cmp(b).reverse());

    let mut prev_score = 0;
    let mut prev_value = -1;
    let mut score = 0;

    while let Some(n) = list1.pop() {
        if prev_value == n {
            score += prev_score;
            continue;
        }

        prev_value = n;
        let mut prev_count = 0;
        loop {
            if let Some(v) = list2.pop() {
                match v.cmp(&n) {
                    Ordering::Less => {
                        continue;
                    }
                    Ordering::Equal => {
                        prev_count += 1;
                    }
                    Ordering::Greater => {
                        list2.push(v);
                        break;
                    }
                }
            } else {
                break;
            }
        }

        prev_score = n * prev_count;
        score += prev_score;
    }

    score
}

fn parse_input(mut input: &[u8]) -> (Vec<i64>, Vec<i64>) {
    let size = input.len() / 14 + 1;
    let mut list1 = Vec::<i64>::with_capacity(size);
    let mut list2 = Vec::<i64>::with_capacity(size);

    loop {
        if input.is_empty() {
            break;
        }
        let l = parse_5_digit_base_10(input[..5].try_into().unwrap());
        let r = parse_5_digit_base_10(input[8..13].try_into().unwrap());

        list1.push(l);
        list2.push(r);

        if input.len() < 14 {
            break;
        }
        input = &input[14..];
    }

    (list1, list2)
}

fn parse_5_digit_base_10(input: [u8; 5]) -> i64 {
    let mut n: i64 = 0;
    for i in input {
        n = n * 10 + (i - b'0') as i64;
    }
    n
}
