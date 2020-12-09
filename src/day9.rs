use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::parse_lines;

type Input = Vec<i64>;

#[aoc_generator(day9)]
pub fn gather_input(input: &str) -> Input {
    parse_lines(input)
}

#[aoc(day9, part1)]
pub fn part1(input: &Input) -> i64 {
    input[find_bad_num_idx(input)]
}

fn find_bad_num_idx(input: &Input) -> usize {
    const PREAMBLE_SIZE: usize = 25;

    'outer: for idx in PREAMBLE_SIZE..(input.len()) {
        for i in (idx - PREAMBLE_SIZE)..(idx) {
            for j in (idx - PREAMBLE_SIZE)..(idx) {
                if i == j {
                    continue;
                }

                if input[i] + input[j] == input[idx] {
                    continue 'outer;
                }
            }
        }

        return idx;
    }

    panic!("no numbers matching criteria found")
}

#[aoc(day9, part2)]
pub fn part2(input: &Input) -> i64 {
    let idx = find_bad_num_idx(input);
    let val = input[idx];

    'outer: for low in 0..input.len() {
        for high in (low + 1)..input.len() {
            let mut sum = 0;
            let mut min = input[low];
            let mut max = input[low];

            for num in &input[low..high] {
                sum += num;
                if *num > max {
                    max = *num;
                }
                if *num < min {
                    min = *num;
                }
            }

            if sum == val {
                return min + max;
            } else if sum > val {
                continue 'outer;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    tests!(9, 29221323, 4389369);
}
