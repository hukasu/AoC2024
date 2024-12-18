use std::{
    collections::{BTreeSet, VecDeque},
    io::{BufRead, BufReader, Read},
};

use crate::vec2d::Vec2d;

fn parse_input(reader: impl Read) -> impl Iterator<Item = (usize, usize)> {
    BufReader::with_capacity(10_000, reader)
        .lines()
        .map(|line| {
            line.unwrap()
                .split_once(',')
                .map(|(column, row)| {
                    (
                        row.parse::<usize>().unwrap(),
                        column.parse::<usize>().unwrap(),
                    )
                })
                .unwrap()
        })
}

pub fn part1(reader: impl Read) -> usize {
    part1_internal(reader, (70, 70), 1024)
}

pub fn part1_for_test(reader: impl Read) -> usize {
    part1_internal(reader, (6, 6), 12)
}

fn part1_internal(reader: impl Read, target: (usize, usize), run_simulation_for: usize) -> usize {
    let obstacles = parse_input(reader);
    let maze = Maze {
        obstacles: obstacles.take(run_simulation_for).collect(),
        bounds: (target.0 + 1, target.1 + 1),
    };
    maze.count_steps().unwrap()
}

pub fn part2(reader: impl Read) -> (usize, usize) {
    part2_internal(reader, (70, 70))
}

pub fn part2_for_test(reader: impl Read) -> (usize, usize) {
    part2_internal(reader, (6, 6))
}

fn part2_internal(reader: impl Read, target: (usize, usize)) -> (usize, usize) {
    let obstacles = parse_input(reader).collect::<Vec<_>>();

    let mut left = 0;
    let mut right = obstacles.len();

    let blockage = loop {
        if left == right {
            break obstacles[left];
        }
        let mid = left + (right - left) / 2;
        let maze = Maze {
            obstacles: obstacles.iter().copied().take(mid + 1).collect(),
            bounds: (target.0 + 1, target.1 + 1),
        };
        if maze.count_steps().is_some() {
            left = mid + 1;
        } else {
            right = mid;
        }
    };

    (blockage.1, blockage.0)
}

struct Maze {
    obstacles: BTreeSet<(usize, usize)>,
    bounds: (usize, usize),
}

impl Maze {
    fn count_steps(&self) -> Option<usize> {
        let mut queue = VecDeque::with_capacity(100);
        queue.push_back(((0, 0), 0));

        let mut steps_to_coord_data = vec![usize::MAX; self.bounds.0 * self.bounds.1];
        let mut steps_to_coord = Vec2d::new(&mut steps_to_coord_data, self.bounds.1, self.bounds.0);

        for obstacle in &self.obstacles {
            steps_to_coord[*obstacle] = 0;
        }

        let mut steps_on_end = Vec::new();

        while let Some((coord, steps)) = queue.pop_front() {
            if coord == (self.bounds.0 - 1, self.bounds.1 - 1) {
                steps_on_end.push(steps);
            } else if steps_to_coord[coord] > steps {
                steps_to_coord[coord] = steps;
                [
                    coord.0.checked_sub(1).map(|row| (row, coord.1)),
                    coord
                        .0
                        .checked_add(1)
                        .filter(|row| *row < self.bounds.0)
                        .map(|row| (row, coord.1)),
                    coord.1.checked_sub(1).map(|column| (coord.0, column)),
                    coord
                        .1
                        .checked_add(1)
                        .filter(|column| *column < self.bounds.1)
                        .map(|column| (coord.0, column)),
                ]
                .into_iter()
                .flatten()
                .for_each(|next_coord| queue.push_back((next_coord, steps + 1)));
            }
        }

        steps_on_end.into_iter().min()
    }
}
