use std::cmp::Ordering;

use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i64 {
    let (mut list1, mut list2) = parse_input(input.as_bytes());
    list1.sort_unstable();
    list2.sort_unstable();
    list1
        .into_iter()
        .zip(list2.into_iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i64 {
    let (mut list1, mut list2) = parse_input(input.as_bytes());
    list1.sort_unstable();
    list2.sort_unstable();

    list1.reverse();
    list2.reverse();

    let mut n;
    let mut prev_score = 0;
    let mut prev_value = -1;
    let mut score = 0;

    loop {
        n = match list1.pop() {
            Some(l) => l,
            None => break score,
        };

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
}

fn parse_input(mut input: &[u8]) -> (Vec<i64>, Vec<i64>) {
    let size = input.len() / 14 + 1;
    let mut list1 = Vec::<i64>::with_capacity(size);
    let mut list2 = Vec::<i64>::with_capacity(size);

    loop {
        if input.is_empty() {
            break;
        }
        let l = unsafe { std::str::from_utf8_unchecked(&input[..5]) }
            .parse()
            .unwrap();
        let r = unsafe { std::str::from_utf8_unchecked(&input[8..13]) }
            .parse()
            .unwrap();

        list1.push(l);
        list2.push(r);

        if input.len() < 14 {
            break;
        }
        input = &input[14..];
    }

    (list1, list2)
}
