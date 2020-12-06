use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

type Input = Vec<Group>;

pub struct Group {
    people: Vec<HashSet<char>>,
    all: HashMap<char, usize>,
}

#[aoc_generator(day6)]
pub fn gather_input(input: &str) -> Input {
    let mut group = Group {
        people: Vec::new(),
        all: HashMap::new(),
    };

    let mut out = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            out.push(group);
            group = Group {
                people: Vec::new(),
                all: HashMap::new(),
            };
            continue;
        }

        group.people.push(line.chars().collect());
        for c in line.chars() {
            *group.all.entry(c).or_insert(0) += 1;
        }
    }

    out.push(group);

    out
}

#[aoc(day6, part1)]
pub fn part1(input: &Input) -> usize {
    input.iter().map(|g| g.all.len()).sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .map(|g| {
            let people = g.people.len();

            g.all.iter().filter(|(_, c)| **c == people).count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    tests!(6, 6565, 3137);
}
