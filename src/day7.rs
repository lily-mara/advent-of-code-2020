use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::{HashMap, HashSet};

type Input = Vec<Rule>;

#[derive(Clone, Debug)]
pub struct Rule {
    outer: String,
    inner: Vec<(i32, String)>,
}

#[aoc_generator(day7)]
pub fn gather_input(input: &str) -> Input {
    let mut rules = Vec::new();

    let outer = Regex::new(r#"^(\w+ \w+) bags"#).unwrap();
    let inner = Regex::new(r#"(\d+) (\w+ \w+) bags?"#).unwrap();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let outer_bag = &outer.captures(line).unwrap()[1];
        let inner_bags = inner
            .captures_iter(line)
            .map(|c| (c[1].parse().unwrap(), c[2].into()))
            .collect();

        rules.push(Rule {
            outer: outer_bag.into(),
            inner: inner_bags,
        })
    }

    rules
}

// not 2
#[aoc(day7, part1)]
pub fn part1(input: &Input) -> usize {
    let mut reversed = HashMap::new();

    for rule in input {
        for (_, inner) in &rule.inner {
            reversed
                .entry(inner.into())
                .or_insert(HashSet::new())
                .insert(rule.outer.clone());
        }
    }

    let mut visited = HashSet::new();
    count_up("shiny gold", &reversed, &mut visited);

    visited.len() - 1
}

fn count_up(
    starting_at: &str,
    reversed: &HashMap<String, HashSet<String>>,
    visited: &mut HashSet<String>,
) {
    if !visited.insert(starting_at.into()) {
        return;
    }

    if let Some(parents) = reversed.get(starting_at) {
        for node in parents {
            count_up(node, reversed, visited);
        }
    }
}

#[aoc(day7, part2)]
pub fn part2(input: &Input) -> i32 {
    let mut sorted = HashMap::new();

    for rule in input {
        sorted.insert(rule.outer.clone(), rule.clone());
    }

    count_part_2("shiny gold", &sorted) - 1
}

fn count_part_2(starting_at: &str, sorted: &HashMap<String, Rule>) -> i32 {
    let mut count = 1;

    if let Some(rule) = sorted.get(starting_at) {
        for (child_cost, child) in &rule.inner {
            count += child_cost * count_part_2(&child, &sorted);
        }
    }

    count
}

#[cfg(test)]
mod tests {
    tests!(7, 229, 6683);
    test_examples!(
        ("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags." => 4),
        ("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags." => 32)
    );
}
