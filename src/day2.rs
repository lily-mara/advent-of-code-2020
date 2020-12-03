use aoc_runner_derive::{aoc, aoc_generator};

pub struct Stuff {
    low: usize,
    high: usize,
    c: char,
    pass: String,
}

#[aoc_generator(day2)]
pub fn gather_input(input: &str) -> Vec<Stuff> {
    let re = regex::Regex::new(r#"(\d+)-(\d+) (\w): (\w+)"#).unwrap();
    re.captures_iter(input)
        .map(|cap| Stuff {
            low: cap[1].parse().unwrap(),
            high: cap[2].parse().unwrap(),
            c: cap[3].chars().next().unwrap(),
            pass: cap[4].to_string(),
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<Stuff>) -> i32 {
    let mut valid = 0;

    for s in input {
        let count = s.pass.chars().filter(|c| *c == s.c).count();
        if s.low <= count && count <= s.high {
            valid += 1;
        }
    }

    valid
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Stuff>) -> i32 {
    let mut valid = 0;

    for s in input {
        let chars: Vec<char> = s.pass.chars().collect();
        if (chars[s.low - 1] == s.c) ^ (chars[s.high - 1] == s.c) {
            valid += 1;
        }
    }

    valid
}

#[cfg(test)]
mod tests {
    tests!(2, 586, 352);
}
