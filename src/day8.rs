use crate::asm::{Kind, Program, Termination};
use aoc_runner_derive::{aoc, aoc_generator};

type Input = Program;

#[aoc_generator(day8)]
pub fn gather_input(input: &str) -> Input {
    Program::from_str(input)
}

#[aoc(day8, part1)]
pub fn part1(input: &Input) -> i32 {
    let mut input = input.clone();

    if let Termination::Looping(val) = input.run() {
        return val;
    }

    panic!("Program did not terminate via looping")
}

#[aoc(day8, part2)]
pub fn part2(input: &Input) -> i32 {
    let indices = input
        .instructions
        .iter()
        .enumerate()
        .filter_map(|(idx, i)| match i.kind {
            Kind::Nop | Kind::Jmp => Some(idx),
            _ => None,
        });

    for idx in indices {
        let mut input = input.clone();
        let new_kind = match input.instructions[idx].kind {
            Kind::Jmp => Kind::Nop,
            Kind::Nop => Kind::Jmp,
            _ => panic!("Should not be possible"),
        };

        input.instructions[idx].kind = new_kind;

        if let Termination::Normal(result) = input.run() {
            return result;
        }
    }

    panic!("No program found which causes termination via normal exit")
}

#[cfg(test)]
mod tests {
    tests!(8, 1489, 1539);
}
