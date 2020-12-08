use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::{HashMap, HashSet};

type Input = Vec<i32>;

#[aoc_generator(day8)]
pub fn gather_input(input: &str) -> Input {
    vec![]
}

#[aoc(day8, part1)]
pub fn part1(input: &Input) -> usize {
    0
}

#[aoc(day8, part2)]
pub fn part2(input: &Input) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    // tests!(8, 0, 0);
    test_examples!(
        (" " => 4),
        (" " => 32)
    );
}
