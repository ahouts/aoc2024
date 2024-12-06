use seq_macro::seq;
use std::{
    cmp::Ordering,
    ops::{Index, IndexMut},
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
            update.sort_unstable_by(|a, b| input.orderings[(*a, *b)]);
            update[(len / 2) as usize] as usize
        })
        .sum()
}

fn is_update_valid(update: &[u8], orderings: &Orderings) -> bool {
    update.is_sorted_by(|a, b| orderings[(*a, *b)] == Ordering::Less)
}

struct Orderings([Ordering; Self::SIZE]);

impl Orderings {
    const SIZE: usize = 0b1100011_1100011;

    fn offset(&self, a: u8, b: u8) -> usize {
        ((a as usize) << 7) | (b as usize)
    }
}

impl Default for Orderings {
    fn default() -> Self {
        Self([Ordering::Equal; Self::SIZE])
    }
}

impl Index<(u8, u8)> for Orderings {
    type Output = Ordering;

    fn index(&self, (a, b): (u8, u8)) -> &Self::Output {
        &self.0[self.offset(a, b)]
    }
}

impl IndexMut<(u8, u8)> for Orderings {
    fn index_mut(&mut self, (a, b): (u8, u8)) -> &mut Self::Output {
        &mut self.0[self.offset(a, b)]
    }
}

struct Input {
    orderings: Orderings,
    updates: Vec<([u8; 23], u8)>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.as_bytes();
        let mut orderings = Orderings::default();
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
                orderings[(*before, *after)] = Ordering::Less;
                orderings[(*after, *before)] = Ordering::Greater;
            }
            4
        });

        for ordering in ordering_input.array_chunks::<6>() {
            let before = parse_10_to_99(ordering[0], ordering[1]);
            let after = parse_10_to_99(ordering[3], ordering[4]);
            orderings[(after, before)] = Ordering::Greater;
            orderings[(before, after)] = Ordering::Less;
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

macro_rules! swizzle_x64_radix_3_with_offset {
    ( $data:expr, $offset:expr ) => {
        simd_swizzle!($data, seq!(N in 0..32 {
            [
                #(
                    if N < 21 { N * 3 + $offset } else { 0 },
                )*
            ]
        }))
    };
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
        let tens = swizzle_x64_radix_3_with_offset!(d, 0);
        let ones = swizzle_x64_radix_3_with_offset!(d, 1);
        let mut nums = tens * ten;
        nums += ones;

        let sep = swizzle_x64_radix_3_with_offset!(orig, 2);

        let unread = accept(nums, sep);
        input = &input[(64 - unread)..];
    }
    input
}

fn parse_10_to_99(a: u8, b: u8) -> u8 {
    10 * (a - b'0') + (b - b'0')
}
