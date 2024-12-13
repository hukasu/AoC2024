use std::{
    cell::Cell,
    collections::HashMap,
    io::Read,
    ops::{Deref, DerefMut},
    rc::Rc,
};

fn parse_data(data: &[u8]) -> Vec<&[u8]> {
    data.split(|c| c == &b'\n').collect()
}

fn calculate_area_perimeter(data: &[&[u8]]) -> Vec<(u64, u64)> {
    let mut merge_insert = MergeInsert::default();

    for convolution in Convolution::new(data) {
        let ([[_, t, _], [l, c, r], [_, b, _]], line, col) = convolution;

        let cur_area = 1u64;
        let cur_perimeter = (4 - [t, r, b, l].into_iter().filter(|crop| *crop == c).count()) as u64;

        match (c == l, c == t) {
            (true, true) => {
                merge_insert.insert((line, col), (cur_area, cur_perimeter));
                merge_insert.merge((line, col), (line, col - 1));
                merge_insert.merge((line, col), (line - 1, col));
            }
            (true, false) => {
                merge_insert.insert((line, col), (cur_area, cur_perimeter));
                merge_insert.merge((line, col), (line, col - 1));
            }
            (false, true) => {
                merge_insert.insert((line, col), (cur_area, cur_perimeter));
                merge_insert.merge((line, col), (line - 1, col));
            }
            (false, false) => {
                merge_insert.insert((line, col), (cur_area, cur_perimeter));
            }
        }
    }

    merge_insert
        .drain()
        .filter_map(|(_, v)| match v {
            MergeInsertNode::Root(v) => Some(v.get()),
            _ => None,
        })
        .collect()
}

fn calculate_sides(data: &[&[u8]]) -> Vec<(u64, u64)> {
    let mut merge_insert = MergeInsert::default();

    for (convolution, line, col) in Convolution::new(data) {
        let c = convolution[1][1];
        match convolution.map(|line| line.map(|col| col == c)) {
            // Unreacheable
            [_, [_, false, _], _] => unreachable!("Center is always equal to center."),
            // No adjacents
            [[_, false, _], [false, true, false], [_, false, _]] => {
                merge_insert.insert((line, col), (1, 4));
            }
            // Four way adjacents
            [[_, true, _], [true, true, true], [_, true, _]] => {
                merge_insert.insert((line, col), (1, 0));
                merge_insert.merge((line, col), (line - 1, col));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // XAX
            // XAX
            // XXX
            [[false, true, false], [false, true, false], [_, false, _]] => {
                merge_insert.insert((line, col), (1, 1));
                merge_insert.merge((line, col), (line - 1, col));
            }
            // AAX
            // XAX
            // XXX
            [[true, true, false], [false, true, false], [_, false, _]] => {
                merge_insert.insert((line, col), (1, 2));
                merge_insert.merge((line, col), (line - 1, col));
            }
            // XAA
            // XAX
            // XXX
            [[false, true, true], [false, true, false], [_, false, _]] => {
                merge_insert.insert((line, col), (1, 2));
                merge_insert.merge((line, col), (line - 1, col));
            }
            // AAA
            // XAX
            // XXX
            [[true, true, true], [false, true, false], [_, false, _]] => {
                merge_insert.insert((line, col), (1, 3));
                merge_insert.merge((line, col), (line - 1, col));
            }
            // XXX
            // AAX
            // XXX
            [[false, false, _], [true, true, false], [false, false, _]] => {
                merge_insert.insert((line, col), (1, 1));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // AXX
            // AAX
            // XXX
            [[true, false, _], [true, true, false], [false, false, _]] => {
                merge_insert.insert((line, col), (1, 2));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // XXX
            // AAX
            // AXX
            [[false, false, _], [true, true, false], [true, false, _]] => {
                merge_insert.insert((line, col), (1, 2));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // AXX
            // AAX
            // AXX
            [[true, false, _], [true, true, false], [true, false, _]] => {
                merge_insert.insert((line, col), (1, 3));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // XXX
            // XAX
            // XAX
            [[_, false, _], [false, true, false], [_, true, _]] => {
                merge_insert.insert((line, col), (1, 3));
            }
            // XXX
            // XAA
            // XXX
            [[_, false, _], [false, true, true], [_, false, _]] => {
                merge_insert.insert((line, col), (1, 3));
            }
            // XAX
            // AAX
            // XXX
            [[_, true, false], [true, true, false], [false, false, _]] => {
                merge_insert.insert((line, col), (1, 0));
                merge_insert.merge((line, col), (line - 1, col));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // XAA
            // AAX
            // XXX
            [[_, true, true], [true, true, false], [false, false, _]] => {
                merge_insert.insert((line, col), (1, 1));
                merge_insert.merge((line, col), (line - 1, col));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // XAX
            // AAX
            // AXX
            [[_, true, false], [true, true, false], [true, false, _]] => {
                merge_insert.insert((line, col), (1, 1));
                merge_insert.merge((line, col), (line - 1, col));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // XAA
            // AAX
            // AXX
            [[_, true, true], [true, true, false], [true, false, _]] => {
                merge_insert.insert((line, col), (1, 2));
                merge_insert.merge((line, col), (line - 1, col));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // XAX
            // XAX
            // XAX
            [[false, true, false], [false, true, false], [_, true, _]] => {
                merge_insert.insert((line, col), (1, 0));
                merge_insert.merge((line, col), (line - 1, col));
            }
            // AAX
            // XAX
            // XAX
            [[true, true, false], [false, true, false], [_, true, _]] => {
                merge_insert.insert((line, col), (1, 1));
                merge_insert.merge((line, col), (line - 1, col));
            }
            // XAA
            // XAX
            // XAX
            [[false, true, true], [false, true, false], [_, true, _]] => {
                merge_insert.insert((line, col), (1, 1));
                merge_insert.merge((line, col), (line - 1, col));
            }
            // AAA
            // XAX
            // XAX
            [[true, true, true], [false, true, false], [_, true, _]] => {
                merge_insert.insert((line, col), (1, 2));
                merge_insert.merge((line, col), (line - 1, col));
            }
            // XAX
            // XAA
            // XXX
            [[false, true, _], [false, true, true], [_, false, _]] => {
                merge_insert.insert((line, col), (1, 1));
                merge_insert.merge((line, col), (line - 1, col));
            }
            // AAX
            // XAA
            // XXX
            [[true, true, _], [false, true, true], [_, false, _]] => {
                merge_insert.insert((line, col), (1, 2));
                merge_insert.merge((line, col), (line - 1, col));
            }
            // XXX
            // AAX
            // XAX
            [[false, false, _], [true, true, false], [_, true, _]] => {
                merge_insert.insert((line, col), (1, 1));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // AXX
            // AAX
            // XAX
            [[true, false, _], [true, true, false], [_, true, _]] => {
                merge_insert.insert((line, col), (1, 2));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // XXX
            // AAA
            // XXX
            [[false, false, _], [true, true, true], [false, false, _]] => {
                merge_insert.insert((line, col), (1, 0));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // AXX
            // AAA
            // XXX
            [[true, false, _], [true, true, true], [false, false, _]] => {
                merge_insert.insert((line, col), (1, 1));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // XXX
            // AAA
            // AXX
            [[false, false, _], [true, true, true], [true, false, _]] => {
                merge_insert.insert((line, col), (1, 1));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // AXX
            // AAA
            // AXX
            [[true, false, _], [true, true, true], [true, false, _]] => {
                merge_insert.insert((line, col), (1, 2));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // XXX
            // XAA
            // XAX
            [[_, false, _], [false, true, true], [_, true, _]] => {
                merge_insert.insert((line, col), (1, 2));
            }
            // XAX
            // AAX
            // XAX
            [[_, true, false], [true, true, false], [_, true, _]] => {
                merge_insert.insert((line, col), (1, 0));
                merge_insert.merge((line, col), (line - 1, col));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // XAA
            // AAX
            // XAX
            [[_, true, true], [true, true, false], [_, true, _]] => {
                merge_insert.insert((line, col), (1, 1));
                merge_insert.merge((line, col), (line - 1, col));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // XAX
            // AAA
            // XXX
            [[_, true, _], [true, true, true], [false, false, _]] => {
                merge_insert.insert((line, col), (1, 0));
                merge_insert.merge((line, col), (line - 1, col));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // XAX
            // AAA
            // AXX
            [[_, true, _], [true, true, true], [true, false, _]] => {
                merge_insert.insert((line, col), (1, 1));
                merge_insert.merge((line, col), (line - 1, col));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // XAX
            // XAA
            // XAX
            [[false, true, _], [false, true, true], [_, true, _]] => {
                merge_insert.insert((line, col), (1, 0));
                merge_insert.merge((line, col), (line - 1, col));
            }
            // AAX
            // XAA
            // XAX
            [[true, true, _], [false, true, true], [_, true, _]] => {
                merge_insert.insert((line, col), (1, 1));
                merge_insert.merge((line, col), (line - 1, col));
            }
            // XXX
            // AAA
            // XAX
            [[false, false, _], [true, true, true], [_, true, _]] => {
                merge_insert.insert((line, col), (1, 0));
                merge_insert.merge((line, col), (line, col - 1));
            }
            // AXX
            // AAA
            // XAX
            [[true, false, _], [true, true, true], [_, true, _]] => {
                merge_insert.insert((line, col), (1, 1));
                merge_insert.merge((line, col), (line, col - 1));
            }
        };
    }

    merge_insert
        .drain()
        .filter_map(|(_, v)| match v {
            MergeInsertNode::Root(v) => Some(v.get()),
            _ => None,
        })
        .collect()
}

pub fn part1(mut reader: impl Read) -> u64 {
    let mut data = Vec::with_capacity(10_000);
    reader.read_to_end(&mut data).unwrap();
    let data = parse_data(data.as_slice());

    calculate_area_perimeter(data.as_slice())
        .into_iter()
        .map(|(area, perimeter)| area * perimeter)
        .sum()
}

pub fn part2(mut reader: impl Read) -> u64 {
    let mut data = Vec::with_capacity(10_000);
    reader.read_to_end(&mut data).unwrap();
    let data = parse_data(data.as_slice());

    calculate_sides(data.as_slice())
        .into_iter()
        .map(|(area, perimeter)| area * perimeter)
        .sum()
}

struct Convolution<'a> {
    data: &'a [&'a [u8]],
    line: usize,
    column: usize,
}

impl<'a> Convolution<'a> {
    fn new(data: &'a [&[u8]]) -> Convolution<'a> {
        Self {
            data,
            line: 0,
            column: 0,
        }
    }
}

impl Iterator for Convolution<'_> {
    type Item = ([[u8; 3]; 3], usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.line >= self.data.len() {
            None
        } else {
            let next = [
                [
                    self.line
                        .checked_sub(1)
                        .and_then(|line| self.data.get(line))
                        .and_then(|line| {
                            self.column
                                .checked_sub(1)
                                .and_then(|column| line.get(column))
                        })
                        .copied()
                        .unwrap_or(u8::MAX),
                    self.line
                        .checked_sub(1)
                        .and_then(|line| self.data.get(line))
                        .and_then(|line| line.get(self.column))
                        .copied()
                        .unwrap_or(u8::MAX),
                    self.line
                        .checked_sub(1)
                        .and_then(|line| self.data.get(line))
                        .and_then(|line| line.get(self.column + 1))
                        .copied()
                        .unwrap_or(u8::MAX),
                ],
                [
                    self.data
                        .get(self.line)
                        .and_then(|line| {
                            self.column
                                .checked_sub(1)
                                .and_then(|column| line.get(column))
                        })
                        .copied()
                        .unwrap_or(u8::MAX),
                    self.data
                        .get(self.line)
                        .and_then(|line| line.get(self.column))
                        .copied()
                        .unwrap_or(u8::MAX),
                    self.data
                        .get(self.line)
                        .and_then(|line| line.get(self.column + 1))
                        .copied()
                        .unwrap_or(u8::MAX),
                ],
                [
                    self.data
                        .get(self.line + 1)
                        .and_then(|line| {
                            self.column
                                .checked_sub(1)
                                .and_then(|column| line.get(column))
                        })
                        .copied()
                        .unwrap_or(u8::MAX),
                    self.data
                        .get(self.line + 1)
                        .and_then(|line| line.get(self.column))
                        .copied()
                        .unwrap_or(u8::MAX),
                    self.data
                        .get(self.line + 1)
                        .and_then(|line| line.get(self.column + 1))
                        .copied()
                        .unwrap_or(u8::MAX),
                ],
            ];
            let line = self.line;
            let column = self.column;
            self.column += 1;
            if self.column >= self.data[0].len() {
                self.column = 0;
                self.line += 1;
            }
            Some((next, line, column))
        }
    }
}

#[derive(Debug, Clone)]
enum MergeInsertNode {
    Root(Rc<Cell<(u64, u64)>>),
    Indirection(usize, usize),
}

#[derive(Debug, Default)]
struct MergeInsert {
    data: HashMap<(usize, usize), MergeInsertNode>,
}

impl MergeInsert {
    fn insert(&mut self, index: (usize, usize), value: (u64, u64)) {
        self.data
            .insert(index, MergeInsertNode::Root(Rc::new(Cell::new(value))));
    }

    fn merge(&mut self, li: (usize, usize), ri: (usize, usize)) {
        let Some(l) = self.data.get(&li).cloned() else {
            unreachable!("Should never be called with invalid l.")
        };
        let Some(r) = self.data.get(&ri).cloned() else {
            unreachable!("Should never be called with invalid r.")
        };

        match (l, r) {
            (MergeInsertNode::Root(l), MergeInsertNode::Root(r)) => {
                if !std::ptr::addr_eq(l.as_ptr(), r.as_ptr()) {
                    let r_old = r.get();
                    let l_old = l.get();
                    r.set((r_old.0 + l_old.0, r_old.1 + l_old.1));
                    self.data
                        .insert(li, MergeInsertNode::Indirection(ri.0, ri.1));
                }
            }
            (MergeInsertNode::Root(_), MergeInsertNode::Indirection(line, col)) => {
                self.merge(li, (line, col));
            }
            (MergeInsertNode::Indirection(line, col), MergeInsertNode::Root(_)) => {
                self.merge((line, col), ri);
            }
            (
                MergeInsertNode::Indirection(lline, lcol),
                MergeInsertNode::Indirection(rline, rcol),
            ) => self.merge((lline, lcol), (rline, rcol)),
        }
    }
}

impl Deref for MergeInsert {
    type Target = HashMap<(usize, usize), MergeInsertNode>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for MergeInsert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
