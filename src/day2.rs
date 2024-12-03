use std::simd::{
    cmp::{SimdPartialEq, SimdPartialOrd},
    i8x64,
    num::{SimdInt as _, SimdUint},
    u8x8, Mask,
};

use aoc_runner_derive::aoc;
use genawaiter::stack::{let_gen_using, Co};

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u64 {
    let (data, num_levels) = parse_input(input);

    let_gen_using!(masks, |co| gen_num_safe_lines_masks(
        data.as_slice(),
        num_levels.as_slice(),
        co
    ));

    let mut result = 0;
    for mask in masks {
        result += mask.count_ones() as u64;
    }

    result
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u64 {
    let (data, mut num_levels) = parse_input(input);

    let one = u8x8::splat(1);
    let chunks = data.len() / size_of::<i8x64>();
    for c in 0..chunks {
        let l = c * size_of::<u8x8>();
        let range = l..(l + size_of::<u8x8>());
        let levels = u8x8::from_slice(&num_levels[range.clone()]);
        num_levels[range].copy_from_slice(&levels.saturating_sub(one).to_array()[..]);
    }

    let mut all_masks = Vec::with_capacity(1024 * 1024);

    for i in 0..8 {
        let mut data = data.clone();

        if i != 7 {
            let mut line_num = 0;
            for chunk in data.array_chunks_mut::<8>() {
                if i as u8 >= num_levels[line_num] {
                    line_num += 1;
                    continue;
                }
                chunk.copy_within((i + 1).., i);
                chunk[7] = 0;
                line_num += 1;
            }
        }

        let_gen_using!(masks, |co| gen_num_safe_lines_masks(
            data.as_slice(),
            num_levels.as_slice(),
            co
        ));

        if i == 0 {
            for mask in masks {
                all_masks.push(mask);
            }
        } else {
            let mut j = 0;
            for mask in masks {
                all_masks[j] |= mask;
                j += 1;
            }
        }
    }

    let mut result = 0;
    for mask in all_masks {
        result += mask.count_ones() as u64;
    }

    result
}

async fn gen_num_safe_lines_masks<'a>(data: &'a [i8], num_levels: &'a [u8], co: Co<'a, u64>) {
    let earlier = &data[..(data.len() - 1)];
    let later = &data[1..];

    let chunks = later.len() / size_of::<i8x64>();

    let max_threshold = i8x64::splat(4);
    let positive = i8x64::splat(1);
    let flat = i8x64::splat(0);
    let negative = i8x64::splat(-1);
    let one = u8x8::splat(1);
    let zero = u8x8::splat(0);

    for c in 0..chunks {
        let i = c * size_of::<i8x64>();
        let range = i..(i + size_of::<i8x64>());

        let earlier_c = i8x64::from_slice(&earlier[range.clone()]);
        let later_c = i8x64::from_slice(&later[range.clone()]);

        let l = c * size_of::<u8x8>();
        let mut line_levels = [0; 8];
        line_levels.copy_from_slice(&num_levels[l..(l + size_of::<u8x8>())]);
        for level in line_levels.iter_mut() {
            *level = [
                0b11111111, 0b00000000, 0b00000001, 0b00000011, 0b00000111, 0b00001111, 0b00011111,
                0b00111111, 0b01111111,
            ][(*level) as usize];
        }
        let line_mask = u8x8::from_array(line_levels);

        let delta = later_c.saturating_sub(earlier_c);

        let delta_signs = delta.signum();

        let is_flat = delta_mask_to_line_bitset(delta_signs.simd_eq(flat));
        let is_increasing =
            (delta_mask_to_line_bitset(delta_signs.simd_eq(positive)) & line_mask).simd_ne(zero);
        let is_decreasing =
            (delta_mask_to_line_bitset(delta_signs.simd_eq(negative)) & line_mask).simd_ne(zero);

        let increasing_and_decreasing = is_increasing & is_decreasing;
        let mode_fails = is_flat | increasing_and_decreasing.select(one, zero);

        let unsigned_delta = delta.saturating_abs();
        let over_threshold = unsigned_delta.simd_ge(max_threshold);
        let lines_over_threshold = delta_mask_to_line_bitset(over_threshold);

        co.yield_(
            ((lines_over_threshold | mode_fails) & line_mask)
                .simd_eq(zero)
                .to_bitmask(),
        )
        .await;
    }
}

fn delta_mask_to_line_bitset(delta_mask: Mask<i8, 64>) -> u8x8 {
    u8x8::from_array(delta_mask.to_bitmask().to_ne_bytes())
}

fn parse_input(input: &str) -> (Vec<i8>, Vec<u8>) {
    let mut input = input.as_bytes();

    let mut data = Vec::<i8>::with_capacity(4 * 1024 * 1024);
    let mut num_levels = Vec::<u8>::with_capacity(1024 * 1024);

    let mut levels = 0;
    loop {
        match input.first() {
            f @ (Some(b'\n') | None) => {
                num_levels.push(levels);
                levels = 0;
                while (data.len() % size_of::<u8x8>()) != 0 {
                    data.push(0);
                }
                if f.is_some() {
                    input = &input[1..];
                } else {
                    break;
                }
            }
            Some(b' ') => {
                input = &input[1..];
            }
            Some(f) => {
                input = &input[1..];
                levels += 1;
                match input.first() {
                    Some(b' ') | Some(b'\n') | None => {
                        data.push((f - b'0') as i8);
                    }
                    Some(s) => {
                        data.push((10 * (f - b'0') + (s - b'0')) as i8);
                        input = &input[1..];
                    }
                }
            }
        }
    }

    while (data.len() % size_of::<i8x64>()) != 0 {
        data.push(0);
    }

    data.push(0);

    while num_levels.len() < data.len() / size_of::<u8x8>() {
        num_levels.push(0);
    }

    (data, num_levels)
}
