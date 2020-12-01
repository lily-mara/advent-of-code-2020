use crate::util::parse_lines;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn gather_input(input: &str) -> Vec<i32> {
    parse_lines(input)
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<i32>) -> i32 {
    0
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<i32>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    // tests!(1, 3408471, 5109803);
}
