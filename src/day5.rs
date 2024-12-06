use std::{
    cmp::Ordering,
    simd::{cmp::SimdPartialEq, simd_swizzle, u8x32, u8x64},
    str::FromStr,
};

use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let input = Input::from_str(input).unwrap();

    input
        .updates
        .iter()
        .filter(|(update, len)| is_update_valid(&update[0..(*len as usize)], &input.orderings))
        .map(|(update, len)| update[(*len / 2) as usize] as usize)
        .sum()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let input = Input::from_str(input).unwrap();

    input
        .updates
        .into_iter()
        .filter(|(update, len)| !is_update_valid(&update[0..(*len as usize)], &input.orderings))
        .map(|(mut update, len)| {
            let update = &mut update[0..(len as usize)];
            update.sort_unstable_by(|a, b| {
                if sort(&input.orderings)(a, b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            update[(len / 2) as usize] as usize
        })
        .sum()
}

fn is_update_valid(update: &[u8], orderings: &[u128; 100]) -> bool {
    update.is_sorted_by(sort(&orderings))
}

fn sort(orderings: &[u128; 100]) -> impl Fn(&u8, &u8) -> bool + '_ {
    |a, b| (orderings[*a as usize] & (1 << *b)) != 0
}

struct Input {
    orderings: [u128; 100],
    updates: Vec<([u8; 23], u8)>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.as_bytes();
        let mut orderings: [u128; 100] = [0; 100];
        let mut updates = Vec::with_capacity(256);

        let newline = u8x64::splat(b'\n');

        let mut curr = input.len();
        loop {
            curr -= size_of::<u8x64>();
            let d = u8x64::from_slice(&input[curr..]);
            let newlines = d.simd_eq(newline).to_bitmask();
            if (newlines & (newlines >> 1)) != 0 {
                break;
            }
            curr += 1;
        }

        let mut iter = curr..;
        let ordering_end = loop {
            let i = iter.next().unwrap();
            if input[i] == b'\n' && input[i + 1] == b'\n' {
                break i;
            }
        };

        let mut ordering_input = &input[0..(ordering_end + 1)];
        let mut updates_input = &input[(ordering_end + 2)..];

        ordering_input = simd_parse_10_to_99s_with_separators(ordering_input, |nums, _| {
            for [before, after] in nums.as_array().array_chunks::<2>().take(10) {
                orderings[*before as usize] |= 1 << *after;
            }
            4
        });

        for ordering in ordering_input.array_chunks::<6>() {
            let before = parse_10_to_99(ordering[0], ordering[1]);
            let after = parse_10_to_99(ordering[3], ordering[4]);
            orderings[before as usize] |= 1 << after;
        }

        let mut pages = [0; 23];
        let mut len = 0;
        updates_input = simd_parse_10_to_99s_with_separators(updates_input, |nums, sep| {
            for (n, s) in nums
                .as_array()
                .into_iter()
                .zip(sep.as_array().into_iter())
                .take(21)
            {
                pages[len] = *n;
                len += 1;
                if *s == b'\n' {
                    updates.push((pages, len as u8));
                    pages = [0; 23];
                    len = 0;
                }
            }

            1
        });

        let mut update_iter = updates_input.array_chunks::<3>();
        for update in &mut update_iter {
            pages[len] = parse_10_to_99(update[0], update[1]);
            len += 1;
            if update[2] == b'\n' {
                updates.push((pages, len as u8));
                pages = [0; 23];
                len = 0;
            }
        }

        let rem = update_iter.remainder();
        if !rem.is_empty() {
            pages[len] = parse_10_to_99(rem[0], rem[1]);
            len += 1;
            updates.push((pages, len as u8));
        }

        Ok(Input { orderings, updates })
    }
}

fn simd_parse_10_to_99s_with_separators(
    mut input: &[u8],
    mut accept: impl FnMut(u8x32, u8x32) -> usize,
) -> &[u8] {
    let zero_ascii = u8x64::splat(b'0');
    let ten = u8x32::splat(10);

    while let Some(chunk) = input.array_chunks::<64>().next() {
        let orig = u8x64::from_array(*chunk);
        let d = orig - zero_ascii;
        let tens = simd_swizzle!(
            d,
            [
                0, 3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45, 48, 51, 54, 57, 60, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        );
        let ones = simd_swizzle!(
            d,
            [
                1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34, 37, 40, 43, 46, 49, 52, 55, 58, 61, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
        let mut nums = tens * ten;
        nums += ones;

        let sep = simd_swizzle!(
            orig,
            [
                2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35, 38, 41, 44, 47, 50, 53, 56, 59, 62, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        );

        let unread = accept(nums, sep);
        input = &input[(64 - unread)..];
    }
    input
}

fn parse_10_to_99(a: u8, b: u8) -> u8 {
    10 * (a - b'0') + (b - b'0')
}
