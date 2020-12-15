use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Instruction {
    Value { index: usize, value: u64 },
    Mask { ones: u64, zeroes: u64, floats: u64 },
}

type Input = Vec<Instruction>;

#[aoc_generator(day14)]
pub fn gather_input(input: &str) -> Input {
    let mut out = Vec::new();

    let re = Regex::new(r#"mem\[(\d+)\] = (\d+)"#).unwrap();

    for line in input.lines() {
        if line.starts_with("mask = ") {
            let mut ones = 0u64;
            let mut zeroes = u64::MAX;
            let mut floats = 0;

            for c in line[6..].chars() {
                if c == '1' {
                    ones |= 1;
                }

                if c != '0' {
                    zeroes |= 1;
                }

                if c == 'X' {
                    floats |= 1;
                }

                ones <<= 1;
                zeroes <<= 1;
                floats <<= 1;
            }

            ones >>= 1;
            zeroes >>= 1;
            floats >>= 1;

            out.push(Instruction::Mask {
                ones,
                zeroes,
                floats,
            });
        } else {
            let caps = re.captures(line).unwrap();

            let index = caps[1].parse().unwrap();
            let value = caps[2].parse().unwrap();

            out.push(Instruction::Value { index, value });
        }
    }

    out
}

// not 8596549542340
#[aoc(day14, part1)]
pub fn part1(input: &Input) -> u64 {
    let mut memory = HashMap::new();
    let mut m_ones = 0u64;
    let mut m_zeroes = u64::MAX;

    for instruction in input {
        // eprintln!(
        //     "Instruction: {:?}\n  ones: {:064b}\nzeroes: {:064b}",
        //     instruction, m_ones, m_zeroes
        // );
        match instruction {
            Instruction::Value { index, value } => {
                memory.insert(index, (*value & m_zeroes) | m_ones);
            }
            Instruction::Mask { zeroes, ones, .. } => {
                m_zeroes = *zeroes;
                m_ones = *ones;
            }
        }
    }

    dbg!(&memory);

    memory.values().sum()
}

#[aoc(day14, part2)]
pub fn part2(_: &Input) -> u64 {
    /*
    let mut memory = HashMap::new();
    let mut m_ones = 0u64;
    let mut m_zeroes = u64::MAX;
    let mut m_floats = 0u64;

    for instruction in input {
        // eprintln!(
        //     "Instruction: {:?}\n  ones: {:064b}\nzeroes: {:064b}",
        //     instruction, m_ones, m_zeroes
        // );
        match instruction {
            Instruction::Value { index, value } => {
                let mut index_float = *index;

                memory.insert(*index, *value);
            }
            Instruction::Mask {
                zeroes,
                ones,
                floats,
            } => {
                m_zeroes = *zeroes;
                m_ones = *ones;
                m_floats = *floats;
            }
        }
    }

    dbg!(&memory);

    memory.values().sum()
    */
    0
}

#[cfg(test)]
mod tests {
    // tests!(14, 0, 0);
    //     test_examples!(
    //         ("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    // mem[8] = 11
    // mem[7] = 101
    // mem[8] = 0" => 165),
    //         (" " => 32)
    //     );
}
