use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Input = Problem;

pub struct Problem {
    earliest: u64,
    busses: Vec<Bus>,
}

enum Bus {
    Line(u64),
    Oos,
}

#[aoc_generator(day13)]
pub fn gather_input(input: &str) -> Input {
    let mut lines = input.lines();
    let earliest = lines.next().unwrap().parse().unwrap();

    let mut busses = Vec::new();

    for bus in lines.next().unwrap().split(',') {
        if bus == "x" {
            busses.push(Bus::Oos);
        } else {
            busses.push(Bus::Line(bus.parse().unwrap()))
        }
    }

    Problem { earliest, busses }
}

#[aoc(day13, part1)]
pub fn part1(input: &Input) -> u64 {
    let lines: HashSet<_> = input
        .busses
        .iter()
        .filter_map(|b| match b {
            Bus::Line(l) => Some(l),
            _ => None,
        })
        .collect();

    for i in input.earliest.. {
        for j in &lines {
            if i % *j == 0 {
                return (i - input.earliest) * *j;
            }
        }
    }

    panic!("no time found")
}

#[aoc(day13, part2)]
pub fn part2(input: &Input) -> u64 {
    let start = *input
        .busses
        .iter()
        .filter_map(|b| match b {
            Bus::Line(l) => Some(l),
            _ => None,
        })
        .nth(0)
        .unwrap() as u64;

    let busses_and_offsets: Vec<_> = input
        .busses
        .iter()
        .enumerate()
        .filter_map(|(i, b)| match b {
            Bus::Line(l) => Some((*l, i as u64)),
            _ => None,
        })
        .collect();

    let pb = indicatif::ProgressBar::new(u64::MAX);

    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar} {pos} {per_sec}")
            .progress_chars("##-"),
    );

    pb.enable_steady_tick(500);

    let mut t = start;

    'outer: loop {
        for (line_id, offset) in &busses_and_offsets {
            if ((t + *offset) % *line_id) != 0 {
                t += start;
                pb.inc(start);

                continue 'outer;
            }
        }

        return t;
    }
}

#[cfg(test)]
mod tests {
    // tests!(13, 3882, 0);
    // test_examples!(
    //     (" " => 4),
    //     ("3\n7,13,x,x,59,x,31,19" => 1068781)
    // );
}
