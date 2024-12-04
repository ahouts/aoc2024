use aoc_runner_derive::aoc;
use regex::bytes::Regex;

use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut total: u64 = 0;
    for capture in re.captures_iter(input.as_bytes()) {
        total += parse_number(capture.get(1).unwrap().as_bytes())
            * parse_number(capture.get(2).unwrap().as_bytes());
    }
    total
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|(don't)\(\)|(do)\(\)").unwrap();
    let mut total: u64 = 0;
    let mut enabled = true;
    for capture in re.captures_iter(input.as_bytes()) {
        if let Some(c) = capture.get(1) {
            if enabled {
                total +=
                    parse_number(c.as_bytes()) * parse_number(capture.get(2).unwrap().as_bytes());
            }
        } else if capture.get(3).is_some() {
            enabled = false;
        } else if capture.get(4).is_some() {
            enabled = true;
        }
    }
    total
}

fn parse_number(text: &[u8]) -> u64 {
    match text.len() {
        3 => 100 * (text[0] - b'0') as u64 + 10 * (text[1] - b'0') as u64 + (text[2] - b'0') as u64,
        2 => 10 * (text[0] - b'0') as u64 + (text[1] - b'0') as u64,
        1 => (text[0] - b'0') as u64,
        _ => {
            let mut total = 0;
            for c in text {
                total = total * 10 + (c - b'0') as u64;
            }
            total
        }
    }
}
