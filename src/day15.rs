use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

use crate::util::parse_commas;

type Input = Vec<i32>;

#[aoc_generator(day15)]
pub fn gather_input(input: &str) -> Input {
    parse_commas(input)
}

#[aoc(day15, part1)]
pub fn part1(input: &Input) -> i32 {
    run(input, 2020)
}

fn run(input: &Input, turns: usize) -> i32 {
    let mut turn = input.len();
    let mut said: HashMap<_, _> = input
        .iter()
        .enumerate()
        .map(|(turn, num)| (*num, turn + 1))
        .collect();

    let mut last = input[input.len() - 1];

    while turn < turns {
        let say = match said.get(&last) {
            Some(last_said) => turn - *last_said,
            None => 0,
        };

        said.insert(last, turn);

        last = say as i32;

        turn += 1;
    }

    last
}

#[aoc(day15, part2)]
pub fn part2(input: &Input) -> i32 {
    run(input, 30000000)
}

#[cfg(test)]
mod tests {
    // This is commented out because part2 runs slowly
    // tests!(15, 1428, 3718541);
}
