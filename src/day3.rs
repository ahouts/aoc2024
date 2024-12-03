use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut total: u64 = 0;
    for capture in re.captures_iter(input) {
        total += capture.get(1).unwrap().as_str().parse::<u64>().unwrap()
            * capture.get(2).unwrap().as_str().parse::<u64>().unwrap();
    }
    total
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|(don't)\(\)|(do)\(\)").unwrap();
    let mut total: u64 = 0;
    let mut enabled = true;
    for capture in re.captures_iter(input) {
        if let Some(c) = capture.get(1) {
            if enabled {
                total += c.as_str().parse::<u64>().unwrap()
                    * capture.get(2).unwrap().as_str().parse::<u64>().unwrap();
            }
        } else if capture.get(3).is_some() {
            enabled = false;
        } else if capture.get(4).is_some() {
            enabled = true;
        }
    }
    total
}
