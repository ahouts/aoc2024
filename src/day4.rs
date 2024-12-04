use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let (input, width, height) = parse_input(input);
    let index = |x: usize, y: usize| y * width + x;

    let mut count = 0;

    let mut handle_value = |value: &[u8]| match value {
        [b'X', b'M', b'A', b'S'] | [b'S', b'A', b'M', b'X'] => count += 1,
        _ => {}
    };

    // horizontal
    for i in 0..(input.len() - 4) {
        handle_value(&input[i..(i + 4)]);
    }

    // vertical
    for y in 0..(height - 3) {
        for x in 0..width {
            handle_value(&[
                input[index(x, y)],
                input[index(x, y + 1)],
                input[index(x, y + 2)],
                input[index(x, y + 3)],
            ]);
        }
    }

    // diagonal 1
    for y in 0..(height - 3) {
        for x in 0..(width - 3) {
            handle_value(&[
                input[index(x, y)],
                input[index(x + 1, y + 1)],
                input[index(x + 2, y + 2)],
                input[index(x + 3, y + 3)],
            ]);
        }
    }

    for y in 0..(height - 3) {
        for x in 3..width {
            handle_value(&[
                input[index(x, y)],
                input[index(x - 1, y + 1)],
                input[index(x - 2, y + 2)],
                input[index(x - 3, y + 3)],
            ]);
        }
    }

    count
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u64 {
    let (input, width, height) = parse_input(input);
    let index = |x: usize, y: usize| y * width + x;

    let mut count = 0;

    for y in 0..(height - 2) {
        for x in 0..(width - 2) {
            if input[index(x + 1, y + 1)] != b'A' {
                continue;
            }
            match [input[index(x, y)], input[index(x + 2, y + 2)]] {
                [b'M', b'S'] | [b'S', b'M'] => {
                    match [input[index(x, y + 2)], input[index(x + 2, y)]] {
                        [b'M', b'S'] | [b'S', b'M'] => count += 1,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    count
}

fn parse_input(input: &str) -> (Vec<u8>, usize, usize) {
    let mut input = input.as_bytes().to_vec();
    input.push(b'\n');
    let width = input
        .iter()
        .copied()
        .enumerate()
        .find(|(_, c)| *c == b'\n')
        .map(|(i, _)| i)
        .unwrap()
        + 1;
    let height = (input.len() + 1) / width;
    (input, width, height)
}
