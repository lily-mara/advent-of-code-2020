use crate::util::*;
use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<Vec<char>>;

#[aoc_generator(day11)]
pub fn gather_input(input: &str) -> Input {
    let v: Vec<String> = parse_lines(input);

    v.iter().map(|v| v.chars().collect()).collect()
}

#[aoc(day11, part1)]
pub fn part1(input: &Input) -> usize {
    let (mut count, mut seats) = run(input);
    let mut last_count = count + 1;

    while count != last_count {
        last_count = count;
        let result = run(&seats);
        count = result.0;
        seats = result.1;
    }

    count
}

fn run(old_seats: &Input) -> (usize, Input) {
    let mut new_seats = old_seats.clone();

    let mut count = 0;

    for i in 0..old_seats.len() {
        for j in 0..(old_seats[i].len()) {
            let adjacent = count_occupied_adjacent(old_seats, i, j);
            let seat = &mut new_seats[i][j];

            if *seat == 'L' && adjacent == 0 {
                *seat = '#';
            } else if *seat == '#' && adjacent >= 4 {
                *seat = 'L';
            }

            if *seat == '#' {
                count += 1;
            }
        }
    }

    (count, new_seats)
}

fn count_occupied_adjacent(seats: &Input, i: usize, j: usize) -> usize {
    let mut count = 0;

    let diffs: &[(isize, isize)] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (i_d, j_d) in diffs {
        let i = i as isize + i_d;
        let j = j as isize + j_d;

        if 0 <= i && i < seats.len() as isize {
            if 0 <= j && j < seats[i as usize].len() as isize {
                if seats[i as usize][j as usize] == '#' {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_occupied_sightline(seats: &Input, start_i: usize, start_j: usize) -> usize {
    let mut count = 0;

    let diffs: &[(isize, isize)] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (i_d, j_d) in diffs {
        let mut i = start_i as isize + *i_d;
        let mut j = start_j as isize + *j_d;

        while 0 <= i && i < seats.len() as isize && 0 <= j && j < seats[i as usize].len() as isize {
            if seats[i as usize][j as usize] == '#' {
                count += 1;
                break;
            } else if seats[i as usize][j as usize] == 'L' {
                break;
            }

            j += j_d;
            i += i_d;
        }
    }

    count
}

fn run2(old_seats: &Input) -> (usize, Input) {
    let mut new_seats = old_seats.clone();
    let mut count = 0;

    for i in 0..old_seats.len() {
        for j in 0..(old_seats[i].len()) {
            let adjacent = count_occupied_sightline(old_seats, i, j);
            let seat = &mut new_seats[i][j];

            if *seat == 'L' && adjacent == 0 {
                *seat = '#';
            } else if *seat == '#' && adjacent >= 5 {
                *seat = 'L';
            }

            if *seat == '#' {
                count += 1;
            }
        }
    }

    (count, new_seats)
}

#[aoc(day11, part2)]
pub fn part2(input: &Input) -> usize {
    let (mut count, mut seats) = run2(input);
    let mut last_count = count + 1;

    while count != last_count {
        last_count = count;
        let result = run2(&seats);
        count = result.0;
        seats = result.1;
    }

    count
}

#[cfg(test)]
mod tests {
    tests!(11, 2338, 2134);
}
