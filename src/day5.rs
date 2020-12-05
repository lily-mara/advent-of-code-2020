use crate::util::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Input = Vec<String>;

#[aoc_generator(day5)]
pub fn gather_input(input: &str) -> Input {
    parse_lines(input)
}

#[aoc(day5, part1)]
pub fn part1(input: &Input) -> i32 {
    input.iter().map(|s| seat_id(s)).max().unwrap()
}

fn seat_id(s: &str) -> i32 {
    let mut row_low = 0;
    let mut row_high = 127;

    let chars: Vec<_> = s.chars().collect();

    for c in &chars[..6] {
        if *c == 'F' {
            row_high = ((row_high - row_low) / 2) + row_low;
        } else {
            row_low = ((row_high - row_low) / 2) + row_low + 1;
        }
    }

    let row = if chars[6] == 'F' { row_low } else { row_high };

    let mut col_low = 0;
    let mut col_high = 7;

    for c in &chars[7..] {
        if *c == 'L' {
            col_high = ((col_high - col_low) / 2) + col_low;
        } else {
            col_low = ((col_high - col_low) / 2) + col_low + 1;
        }
    }

    let col = if chars[9] == 'L' { col_high } else { col_low };

    row * 8 + col
}

#[aoc(day5, part2)]
pub fn part2(input: &Input) -> i32 {
    let all_ids = input.iter().map(|s| seat_id(s)).collect::<HashSet<_>>();

    let min = *all_ids.iter().min().unwrap();
    let max = *all_ids.iter().max().unwrap();

    for i in min..max {
        if all_ids.contains(&i) {
            continue;
        }

        if all_ids.contains(&(i + 1)) && all_ids.contains(&(i - 1)) {
            return i;
        }
    }

    panic!("No matching seat found")
}

#[cfg(test)]
mod tests {
    tests!(5, 955, 569);
}
