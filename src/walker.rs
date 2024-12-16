use std::{collections::BTreeSet, sync::Mutex};

use crate::direction::Direction;

#[derive(Debug, Clone)]
pub struct Walker<'a> {
    lines: Vec<&'a [u8]>,
    line: &'a [u8],
    row: usize,
    column: usize,
    direction: Direction,
    extra_obstacles: BTreeSet<(usize, usize)>,
}

impl<'a> Walker<'a> {
    pub fn from_data(data: &'a [u8]) -> Option<Walker<'a>> {
        let func: fn(&u8) -> bool = |c| c == &b'\n';
        let lines = data.split(func).collect::<Vec<_>>();

        lines
            .iter()
            .enumerate()
            .find_map(|(i, line)| line.iter().position(|c| c == &b'^').map(|j| (*line, i, j)))
            .map(|(line, row, column)| Self {
                lines,
                line,
                row,
                column,
                direction: Direction::North,
                extra_obstacles: BTreeSet::new(),
            })
    }

    pub fn count_unique_steps(self) -> usize {
        self.map(|(_, i, j)| (i, j)).collect::<BTreeSet<_>>().len()
    }

    fn is_loop(&mut self) -> bool {
        let mut set = BTreeSet::new();
        set.insert((self.direction, self.row, self.column));
        self.any(|step| {
            let res = set.contains(&step);
            set.insert(step);
            res
        })
    }

    pub fn find_possible_loops2(self) -> usize {
        let start = (self.row, self.column);
        let original = self.clone();
        let mutex = Mutex::new(self);

        std::thread::scope(|scope| {
            let mut obstacles = BTreeSet::new();
            obstacles.insert(start);

            let (sender, receiver) = std::sync::mpsc::channel();

            for _ in 0..std::thread::available_parallelism().unwrap().get() {
                let sender_clone = sender.clone();
                let mutex_ref = &mutex;
                scope.spawn(|| {
                    let sender = sender_clone;

                    loop {
                        let opt_obstacle = mutex_ref
                            .lock()
                            .unwrap()
                            .next()
                            .map(|(_, row, column)| (row, column));
                        let Some(obstacle) = opt_obstacle else {
                            return;
                        };

                        let mut iter = original.clone();
                        iter.extra_obstacles = [obstacle].into_iter().collect();

                        if iter.is_loop() {
                            sender.send(obstacle).unwrap();
                        }
                    }
                });
            }

            std::mem::drop(sender);

            for obstacle in receiver {
                obstacles.insert(obstacle);
            }

            obstacles.len() - 1
        })
    }

    pub fn find_possible_loops(mut self) -> usize {
        let mut obstacles = BTreeSet::new();

        obstacles.insert((self.row, self.column));

        let clone = self.clone();

        for step in self.by_ref() {
            let obstacle = (step.1, step.2);

            let mut divert = clone.clone();
            divert.extra_obstacles = [obstacle].into_iter().collect();

            if divert.is_loop() {
                obstacles.insert(obstacle);
            }
        }

        obstacles.len() - 1
    }

    #[inline(always)]
    fn is_obstacle(&self, row: usize, column: usize) -> bool {
        let line = self.lines[row];
        line[column] == b'#' || self.extra_obstacles.contains(&(row, column))
    }
}

impl Iterator for Walker<'_> {
    type Item = (Direction, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.direction {
            Direction::North => {
                if let Some(row) = self.row.checked_sub(1) {
                    if self.is_obstacle(row, self.column) {
                        self.direction = self.direction.turn_right();
                        self.next()
                    } else {
                        self.row -= 1;
                        self.line = self.lines[row];
                        Some((self.direction, self.row, self.column))
                    }
                } else {
                    None
                }
            }
            Direction::East => {
                if self.line.get(self.column + 1).is_some() {
                    if self.is_obstacle(self.row, self.column + 1) {
                        self.direction = self.direction.turn_right();
                        self.next()
                    } else {
                        self.column += 1;
                        Some((self.direction, self.row, self.column))
                    }
                } else {
                    None
                }
            }
            Direction::South => {
                if self.lines.get(self.row + 1).is_some() {
                    if self.is_obstacle(self.row + 1, self.column) {
                        self.direction = self.direction.turn_right();
                        self.next()
                    } else {
                        self.row += 1;
                        self.line = self.lines[self.row];
                        Some((self.direction, self.row, self.column))
                    }
                } else {
                    None
                }
            }
            Direction::West => {
                if let Some(column) = self.column.checked_sub(1) {
                    if self.is_obstacle(self.row, column) {
                        self.direction = self.direction.turn_right();
                        self.next()
                    } else {
                        self.column -= 1;
                        Some((self.direction, self.row, self.column))
                    }
                } else {
                    None
                }
            }
        }
    }
}
