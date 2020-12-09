#![allow(dead_code)]

use std::fmt::Debug;
use std::str::FromStr;

pub fn parse_lines<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|l| match l.parse() {
            Ok(x) => x,
            Err(_) => panic!("Failed to parse {:?}", l),
        })
        .collect()
}

pub fn parse_commas<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

pub fn all_digits<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let re = regex::Regex::new("-?\\d+").unwrap();

    re.find_iter(input)
        .map(|x| x.as_str().parse().unwrap())
        .collect()
}

pub fn parse_lines_and_commas<T>(input: &str) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|line| line.split(',').map(|l| l.parse().unwrap()).collect())
        .collect()
}

pub fn lines_and_chars(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn parse_lines_and_split<T>(input: &str, split: &str) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|line| line.split(split).map(|l| l.parse().unwrap()).collect())
        .collect()
}
