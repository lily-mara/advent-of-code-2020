use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, VecDeque};

use crate::util::parse_lines;

type Input = Vec<i32>;

#[aoc_generator(day10)]
pub fn gather_input(input: &str) -> Input {
    let mut input = parse_lines(input);

    input.sort();

    input.push(input[input.len() - 1] + 3);

    let mut input: VecDeque<_> = input.into();

    if input[0] != 0 {
        input.push_front(0);
    }

    input.into()
}

// not 1920
#[aoc(day10, part1)]
pub fn part1(input: &Input) -> usize {
    let mut last = 0;
    let mut diff_1 = 0;
    let mut diff_3 = 0;

    for i in input {
        let diff = i - last;

        if diff == 1 {
            diff_1 += 1;
        } else if diff == 3 {
            diff_3 += 1;
        } else if diff > 3 {
            panic!("diff too large - {} - {}  ={}", i, last, diff);
        }

        last = *i;
    }

    diff_1 * diff_3
}

fn diff_in_range(x: i32, y: i32) -> bool {
    let diff = y - x;

    1 <= diff && diff <= 3
}

#[aoc(day10, part2)]
pub fn part2(input: &Input) -> i64 {
    let mut cache = HashMap::new();
    rec2(&mut cache, input, 0, input[input.len() - 1])
}

fn rec2(cache: &mut HashMap<usize, i64>, input: &[i32], start: usize, end: i32) -> i64 {
    if input[start] == end {
        return 1;
    }

    if let Some(val) = cache.get(&start) {
        return *val;
    }

    let mut count = 0;
    for i in (start + 1)..(input.len()) {
        if !diff_in_range(input[start], input[i]) {
            break;
        }

        count += rec2(cache, input, i, end);
    }

    cache.insert(start, count);

    count
}

#[cfg(test)]
mod tests {
    tests!(10, 1984, 3543369523456);
}
