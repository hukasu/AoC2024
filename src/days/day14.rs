use std::{
    borrow::Borrow,
    cmp::Ordering,
    io::{BufRead, BufReader, Read},
};

pub fn part1_for_testing(reader: impl Read) -> isize {
    part1_internal(reader, (11, 7))
}

pub fn part1(reader: impl Read) -> isize {
    part1_internal(reader, (101, 103))
}

fn part1_internal(reader: impl Read, bounds: (i64, i64)) -> isize {
    let quadrants = parse_input(reader, bounds)
        .map(|mut robot| {
            robot.step(100);
            robot
        })
        .fold((0, 0, 0, 0), |a, b| count_robots_in_quadrant(a, b, bounds));

    quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
}

pub fn part2_for_testing(reader: impl Read) -> i64 {
    part2_internal(reader, (11, 7))
}

pub fn part2(reader: impl Read) -> i64 {
    part2_internal(reader, (101, 103))
}

fn part2_internal(reader: impl Read, bounds: (i64, i64)) -> i64 {
    let mut robots = parse_input(reader, bounds).collect::<Vec<_>>();
    (1..=(bounds.0 * bounds.1))
        .map(|i| {
            robots.iter_mut().for_each(|robot| robot.step(1));
            let quadrants = robots
                .iter()
                .fold((0, 0, 0, 0), |a, b| count_robots_in_quadrant(a, b, bounds));
            (
                i,
                quadrants
                    .0
                    .max(quadrants.1)
                    .max(quadrants.2)
                    .max(quadrants.3),
            )
        })
        .max_by_key(|(_, score)| *score)
        .map(|(i, _)| i)
        .unwrap()
}

fn parse_input(reader: impl Read, bounds: (i64, i64)) -> impl Iterator<Item = Robot> {
    BufReader::with_capacity(100_000, reader)
        .lines()
        .map(move |line| {
            line.unwrap()
                .split_once(' ')
                .map(|(left, right)| {
                    let position = left
                        .trim_start_matches("p=")
                        .split_once(',')
                        .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
                        .unwrap();
                    let velocity = right
                        .trim_start_matches("v=")
                        .split_once(',')
                        .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
                        .unwrap();
                    Robot {
                        position,
                        velocity,
                        bounds,
                    }
                })
                .unwrap()
        })
}

fn count_robots_in_quadrant(
    quadrants: (isize, isize, isize, isize),
    robot: impl Borrow<Robot>,
    bounds: (i64, i64),
) -> (isize, isize, isize, isize) {
    match (
        robot.borrow().position.0.cmp(&(bounds.0 / 2)),
        robot.borrow().position.1.cmp(&(bounds.1 / 2)),
    ) {
        (Ordering::Equal, _) | (_, Ordering::Equal) => quadrants,
        (Ordering::Less, Ordering::Less) => {
            (quadrants.0 + 1, quadrants.1, quadrants.2, quadrants.3)
        }
        (Ordering::Less, Ordering::Greater) => {
            (quadrants.0, quadrants.1 + 1, quadrants.2, quadrants.3)
        }
        (Ordering::Greater, Ordering::Less) => {
            (quadrants.0, quadrants.1, quadrants.2 + 1, quadrants.3)
        }
        (Ordering::Greater, Ordering::Greater) => {
            (quadrants.0, quadrants.1, quadrants.2, quadrants.3 + 1)
        }
    }
}

#[derive(Debug)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
    bounds: (i64, i64),
}

impl Robot {
    fn step(&mut self, steps: i64) {
        self.position.0 += self.velocity.0 * steps;
        self.position.0 = self.position.0.rem_euclid(self.bounds.0);
        self.position.1 += self.velocity.1 * steps;
        self.position.1 = self.position.1.rem_euclid(self.bounds.1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let mut robot = Robot {
            position: (2, 4),
            velocity: (2, -3),
            bounds: (11, 7),
        };
        robot.step(1);
        assert_eq!(robot.position, (4, 1));
        robot.step(1);
        assert_eq!(robot.position, (6, 5));
        robot.step(1);
        assert_eq!(robot.position, (8, 2));
        robot.step(1);
        assert_eq!(robot.position, (10, 6));
        robot.step(1);
        assert_eq!(robot.position, (1, 3));
    }
}
