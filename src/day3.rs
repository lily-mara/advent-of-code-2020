use crate::util::*;
use aoc_runner_derive::{aoc, aoc_generator};

pub struct Stuff {
    low: usize,
    high: usize,
    c: char,
    pass: String,
}

#[aoc_generator(day3)]
pub fn gather_input(input: &str) -> Vec<Vec<char>> {
    lines_and_chars(input)
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<Vec<char>>) -> i32 {
    let width = input[0].len();

    let mut trees = 0;
    let mut x = 0;

    for row in input {
        if row[x] == '#' {
            trees += 1;
        }
        x = (x + 3) % width;
    }

    trees
}

fn trees(input: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
    let width = input[0].len();

    let mut trees = 0;
    let mut x = 0;
    let mut y = 0;

    while y < input.len() {
        if input[y][x] == '#' {
            trees += 1;
        }
        x = (x + right) % width;
        y += down;
    }

    trees
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<Vec<char>>) -> usize {
    trees(input, 1, 1)
        * trees(input, 3, 1)
        * trees(input, 5, 1)
        * trees(input, 7, 1)
        * trees(input, 1, 2)
}

#[cfg(test)]
mod tests {
    tests!(3, 173, 4385176320);
}
