use std::simd::{
    cmp::{SimdPartialEq, SimdPartialOrd},
    u8x64,
};

use aoc_runner_derive::aoc;

const MAX_LINE_LENGTH: usize = 13;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    partn::<Part1NextOp>(input)
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Op {
    Add,
    Mul,
    Cons,
}

trait NextOp {
    fn next(prev: Op) -> Option<Op>;
}

struct Part1NextOp;
impl NextOp for Part1NextOp {
    fn next(op: Op) -> Option<Op> {
        match op {
            Op::Add => Some(Op::Mul),
            Op::Mul => None,
            Op::Cons => None,
        }
    }
}

struct Part2NextOp;
impl NextOp for Part2NextOp {
    fn next(op: Op) -> Option<Op> {
        match op {
            Op::Add => Some(Op::Mul),
            Op::Mul => Some(Op::Cons),
            Op::Cons => None,
        }
    }
}

struct Ops<'a>(&'a mut [Op]);

impl<'a> Ops<'a> {
    fn next<NO: NextOp>(&mut self) -> Option<usize> {
        let s = &mut self.0;
        let mut invalidated = 1;
        for i in (0..s.len()).rev() {
            if let Some(next) = NO::next(s[i]) {
                s[i] = next;
                return Some(invalidated);
            }
            s[i] = Op::Add;
            invalidated += 1;
        }
        None
    }
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    partn::<Part2NextOp>(input)
}

fn partn<NO: NextOp>(input: &str) -> u64 {
    let mut total = 0;
    iter_lines(input, |nums, nums_digits, nums_count| {
        let target = nums[0];
        let vars = &nums[2..nums_count];
        let digits = &nums_digits[2..nums_count];
        let mut ops = [Op::Add; MAX_LINE_LENGTH];
        let mut ops = Ops(&mut ops[2..nums_count]);

        let mut cache = [0; MAX_LINE_LENGTH];
        let mut valid = 0;

        loop {
            for i in valid..vars.len() {
                let prev = if i == 0 { nums[1] } else { cache[i - 1] };
                match ops.0[i] {
                    Op::Add => cache[i] = prev + vars[i],
                    Op::Mul => cache[i] = prev * vars[i],
                    Op::Cons => {
                        cache[i] = prev * 10u64.pow(digits[i] as u32);
                        cache[i] += vars[i];
                    }
                }
            }
            valid = vars.len();

            if cache[vars.len() - 1] == target {
                total += target;
                return;
            }

            if let Some(invalidated) = ops.next::<NO>() {
                valid -= invalidated;
            } else {
                break;
            }
        }
    });
    total
}

fn iter_lines(
    input: &str,
    mut handle_line: impl FnMut([u64; MAX_LINE_LENGTH], [u8; MAX_LINE_LENGTH], usize),
) {
    let mut input = input.as_bytes();

    let newline = u8x64::splat(b'\n');
    let zero_ascii = u8x64::splat(b'0');
    let nine_ascii = u8x64::splat(b'9');

    while let Some(chunk) = input.array_chunks::<64>().next() {
        let line = u8x64::from_array(*chunk);
        let newline_mask = line.simd_eq(newline);
        let newline_index = newline_mask.first_set().unwrap();
        let digit_mask = line.simd_ge(zero_ascii) & line.simd_le(nine_ascii);

        let values = line - zero_ascii;

        let mut nums = [0; MAX_LINE_LENGTH];
        let mut num_count = 0;
        let mut nums_digits = [0; MAX_LINE_LENGTH];
        let mut digit_count = 0;
        let mut is_first = true;
        for (i, is_digit) in digit_mask
            .to_array()
            .into_iter()
            .enumerate()
            .take(newline_index + 1)
        {
            if is_digit {
                nums[num_count] *= 10;
                nums[num_count] += values[i] as u64;
                digit_count += 1;
            } else if is_first {
                is_first = false;
            } else {
                nums_digits[num_count] = digit_count;
                digit_count = 0;
                num_count += 1;
            }
        }

        handle_line(nums, nums_digits, num_count);

        input = &input[(newline_index + 1)..];
    }

    let input = unsafe { std::str::from_utf8_unchecked(input) };
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut nums = [0; MAX_LINE_LENGTH];
        let mut nums_count = 0;
        let mut nums_digits = [0; MAX_LINE_LENGTH];
        for part in line.split(' ') {
            let part = part.trim_end_matches(':');
            nums[nums_count] *= 10;
            nums[nums_count] += part.parse::<u64>().unwrap();
            nums_digits[nums_count] = part.len() as u8;
            nums_count += 1;
        }
        handle_line(nums, nums_digits, nums_count);
    }
}
