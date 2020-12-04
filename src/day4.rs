use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

type Input = Vec<HashMap<String, String>>;

// not 103
#[aoc_generator(day4)]
pub fn gather_input(input: &str) -> Input {
    let mut v = Vec::new();
    let mut batch = HashMap::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            v.push(batch);
            batch = HashMap::new();
            continue;
        }

        for pair in line.split(' ') {
            let k: Vec<_> = pair.split(':').collect();

            batch.insert(k[0].into(), k[1].into());
        }
    }

    v.push(batch);

    v
}

#[aoc(day4, part1)]
pub fn part1(input: &Input) -> i32 {
    let mut valid = 0;

    let cols = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let cols: HashSet<String> = cols.into_iter().map(|x| x.into()).collect();

    for passport in input {
        let mut keys: HashSet<String> = passport.keys().map(|x| x.into()).collect();
        keys.remove("cid");

        if cols == keys {
            valid += 1;
        }
    }

    valid
}

macro_rules! t_con {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(_) => {
                continue;
            }
        }
    };
}

#[aoc(day4, part2)]
pub fn part2(input: &Input) -> usize {
    let mut valid = 0;

    let cols = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let cols: HashSet<String> = cols.into_iter().map(|x| x.into()).collect();

    let eyes = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    let eyes: HashSet<String> = eyes.into_iter().map(|x| x.into()).collect();

    for passport in input {
        let mut keys: HashSet<String> = passport.keys().map(|x| x.into()).collect();
        keys.remove("cid");

        if cols != keys {
            continue;
        }

        let byr: i32 = t_con!(passport["byr"].parse());
        if byr < 1920 || byr > 2002 {
            continue;
        }

        let iyr: i32 = t_con!(passport["iyr"].parse());
        if iyr < 2010 || iyr > 2020 {
            continue;
        }

        let eyr: i32 = t_con!(passport["eyr"].parse());
        if eyr < 2020 || eyr > 2030 {
            continue;
        }

        let hgt = &passport["hgt"];
        let hgt_num: i32 = t_con!(hgt[..(hgt.len() - 2)].parse());
        if hgt.ends_with("cm") {
            if hgt_num < 150 || hgt_num > 193 {
                continue;
            }
        } else if hgt.ends_with("in") {
            if hgt_num < 59 || hgt_num > 76 {
                continue;
            }
        } else {
            continue;
        }

        let r = regex::Regex::new("^#[0-9a-f]{6}$").unwrap();
        if !r.is_match(passport["hcl"].as_str()) {
            continue;
        }

        let ecl = &passport["ecl"];
        if !eyes.contains(ecl) {
            continue;
        }

        let r = regex::Regex::new("^[0-9]{9}$").unwrap();
        if !r.is_match(passport["pid"].as_str()) {
            continue;
        }

        valid += 1;
    }

    valid
}

#[cfg(test)]
mod tests {
    tests!(4, 200, 116);
}
