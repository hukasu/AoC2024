use std::{
    collections::{BTreeSet, VecDeque},
    io::{BufRead, BufReader, Read},
};

use crate::{coord::Coord, vec2d::Vec2d};

fn parse_input(reader: impl Read) -> impl Iterator<Item = Coord> {
    BufReader::with_capacity(10_000, reader)
        .lines()
        .map(|line| {
            line.unwrap()
                .split_once(',')
                .map(|(column, row)| Coord::new(row.parse().unwrap(), column.parse().unwrap()))
                .unwrap()
        })
}

pub fn part1(reader: impl Read) -> usize {
    part1_internal(reader, Coord::new(70, 70), 1024)
}

pub fn part1_for_test(reader: impl Read) -> usize {
    part1_internal(reader, Coord::new(6, 6), 12)
}

fn part1_internal(reader: impl Read, target: Coord, run_simulation_for: usize) -> usize {
    let obstacles = parse_input(reader);
    let maze = Maze {
        obstacles: obstacles.take(run_simulation_for).collect(),
        bounds: Coord::new(target.row + 1, target.column + 1),
    };
    maze.count_steps().unwrap()
}

pub fn part2(reader: impl Read) -> Coord {
    part2_internal(reader, Coord::new(70, 70))
}

pub fn part2_for_test(reader: impl Read) -> Coord {
    part2_internal(reader, Coord::new(6, 6))
}

fn part2_internal(reader: impl Read, target: Coord) -> Coord {
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
            bounds: Coord::new(target.row + 1, target.column + 1),
        };
        if maze.count_steps().is_some() {
            left = mid + 1;
        } else {
            right = mid;
        }
    };

    Coord::new(blockage.column, blockage.row)
}

struct Maze {
    obstacles: BTreeSet<Coord>,
    bounds: Coord,
}

impl Maze {
    fn count_steps(&self) -> Option<usize> {
        let mut queue = VecDeque::with_capacity(100);
        queue.push_back((Coord::new(0, 0), 0));

        let mut steps_to_coord_data = vec![usize::MAX; self.bounds.row * self.bounds.column];
        let mut steps_to_coord = Vec2d::new(
            &mut steps_to_coord_data,
            self.bounds.column,
            self.bounds.row,
        );

        for obstacle in &self.obstacles {
            steps_to_coord[*obstacle] = 0;
        }

        let mut steps_on_end = Vec::new();

        while let Some((coord, steps)) = queue.pop_front() {
            if coord == self.bounds - (1, 1) {
                steps_on_end.push(steps);
            } else if steps_to_coord[coord] > steps {
                steps_to_coord[coord] = steps;
                [
                    coord
                        .row
                        .checked_sub(1)
                        .map(|row| Coord::new(row, coord.column)),
                    coord
                        .row
                        .checked_add(1)
                        .filter(|row| *row < self.bounds.row)
                        .map(|row| Coord::new(row, coord.column)),
                    coord
                        .column
                        .checked_sub(1)
                        .map(|column| Coord::new(coord.row, column)),
                    coord
                        .column
                        .checked_add(1)
                        .filter(|column| *column < self.bounds.column)
                        .map(|column| Coord::new(coord.row, column)),
                ]
                .into_iter()
                .flatten()
                .for_each(|next_coord| queue.push_back((next_coord, steps + 1)));
            }
        }

        steps_on_end.into_iter().min()
    }
}
