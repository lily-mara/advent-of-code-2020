use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

type Input = Vec<Instruction>;

pub struct Instruction {
    direction: char,
    magnitude: i32,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    facing: char,
}

#[aoc_generator(day12)]
pub fn gather_input(input: &str) -> Input {
    let mut result = Vec::new();
    let re = Regex::new(r#"([A-Z])(\d+)"#).unwrap();

    for cap in re.captures_iter(input) {
        result.push(Instruction {
            direction: cap[1].chars().next().unwrap(),
            magnitude: cap[2].parse().unwrap(),
        });
    }

    result
}

impl Position {
    fn turn_r(&mut self, degrees: i32) {
        let turns = degrees / 90;

        for _ in 0..turns {
            self.facing = match self.facing {
                'N' => 'E',
                'E' => 'S',
                'S' => 'W',
                'W' => 'N',
                x => panic!("Unknown facing {}", x),
            };
        }
    }

    fn turn_l(&mut self, degrees: i32) {
        let turns = degrees / 90;

        for _ in 0..turns {
            self.facing = match self.facing {
                'N' => 'W',
                'W' => 'S',
                'S' => 'E',
                'E' => 'N',
                x => panic!("Unknown facing {}", x),
            };
        }
    }

    fn translate(&mut self, direction: char, magnitude: i32) {
        match direction {
            'N' => self.y += magnitude,
            'S' => self.y -= magnitude,
            'E' => self.x += magnitude,
            'W' => self.x -= magnitude,
            'R' => self.turn_r(magnitude),
            'L' => self.turn_l(magnitude),
            'F' => self.translate(self.facing, magnitude),
            x => panic!("Unknown direction {}", x),
        };
    }

    fn turn_r_rel(&mut self, degrees: i32) {
        let turns = degrees / 90;

        for _ in 0..turns {
            swap_neg(&mut self.x, &mut self.y);
        }
    }

    fn turn_l_rel(&mut self, degrees: i32) {
        let turns = degrees / 90;

        for _ in 0..turns {
            swap_neg(&mut self.y, &mut self.x);
        }
    }
}

fn swap_neg(i: &mut i32, j: &mut i32) {
    let tmp = -*i;
    *i = *j;
    *j = tmp;
}

// not 2282
#[aoc(day12, part1)]
pub fn part1(input: &Input) -> i32 {
    let mut position = Position {
        x: 0,
        y: 0,
        facing: 'E',
    };

    for instruction in input {
        position.translate(instruction.direction, instruction.magnitude);
    }

    position.x.abs() + position.y.abs()
}

// not 51780
// not 52383
// not 64549
// not 104123
#[aoc(day12, part2)]
pub fn part2(input: &Input) -> i32 {
    let mut ship = Position {
        x: 0,
        y: 0,
        facing: 'E',
    };
    let mut waypoint = Position {
        x: 10,
        y: 1,
        facing: 'E',
    };

    for instruction in input {
        match instruction.direction {
            'N' | 'S' | 'E' | 'W' => {
                waypoint.translate(instruction.direction, instruction.magnitude)
            }
            'R' => waypoint.turn_r_rel(instruction.magnitude),
            'L' => waypoint.turn_l_rel(instruction.magnitude),
            'F' => {
                ship.x += waypoint.x * instruction.magnitude;
                ship.y += waypoint.y * instruction.magnitude;
            }
            x => panic!("invalid direction {}", x),
        }
    }

    ship.x.abs() + ship.y.abs()
}

#[cfg(test)]
mod tests {
    tests!(12, 1424, 63447);
}
