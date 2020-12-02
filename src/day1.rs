use crate::util::parse_lines;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn gather_input(input: &str) -> Vec<i32> {
    parse_lines(input)
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<i32>) -> i32 {
    for x in input {
        for y in input {
            if x + y == 2020 {
                return x * y;
            }
        }
    }

    panic!("No two numbers summed to 2020")
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<i32>) -> i32 {
    for x in input {
        for y in input {
            if x + y < 2020 {
                for z in input {
                    if x + y + z == 2020 {
                        return x * y * z;
                    }
                }
            }
        }
    }

    panic!("No three numbers summed to 2020")
}

#[cfg(test)]
mod tests {
    tests!(1, 793524, 61515678);
    test_examples!((
        "1721
979
366
299
675
1456
" =>
        514579
    ), ("1721
979
366
299
675
1456
" => 241861950));
}
