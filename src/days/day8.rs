use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
    iter::Enumerate,
    slice::{Iter, Split},
};

fn antinode_coords(
    mut coords: &[(usize, usize)],
    harmonics: usize,
    row_limit: usize,
    col_limit: usize,
) -> Vec<(usize, usize)> {
    let mut res = vec![];

    while let [tower, tail @ ..] = coords {
        for second_tower in tail {
            let row_diff = tower.0.abs_diff(second_tower.0);
            let col_diff = tower.1.abs_diff(second_tower.1);

            if tower.1 < second_tower.1 {
                res.extend(
                    std::iter::successors(Some(*tower), |prev| {
                        if let (Some(r), Some(c)) =
                            (prev.0.checked_sub(row_diff), prev.1.checked_sub(col_diff))
                        {
                            Some((r, c))
                        } else {
                            None
                        }
                    })
                    .skip(1)
                    .take(harmonics),
                );
                res.extend(
                    std::iter::successors(Some(*second_tower), |prev| {
                        Some((prev.0 + row_diff, prev.1 + col_diff))
                            .filter(|(r, c)| r <= &row_limit && c <= &col_limit)
                    })
                    .skip(1)
                    .take(harmonics),
                );
            } else {
                res.extend(
                    std::iter::successors(Some(*tower), |prev| {
                        if let Some(r) = prev.0.checked_sub(row_diff) {
                            Some((r, prev.1 + col_diff)).filter(|(_, c)| c <= &col_limit)
                        } else {
                            None
                        }
                    })
                    .skip(1)
                    .take(harmonics),
                );
                res.extend(
                    std::iter::successors(Some(*second_tower), |prev| {
                        if let Some(c) = prev.1.checked_sub(col_diff) {
                            Some((prev.0 + row_diff, c)).filter(|(r, _)| r <= &row_limit)
                        } else {
                            None
                        }
                    })
                    .skip(1)
                    .take(harmonics),
                );
            }
        }

        coords = &coords[1..];
    }

    res
}

fn count_antinodes(mut enumerate: Enumerate2d, harmonics: usize) -> usize {
    let mut antinodes = BTreeSet::new();

    let mut tower_coords = BTreeMap::new();
    (&mut enumerate)
        .filter(|(_, _, tower)| tower != &b'.')
        .for_each(|(row, col, tower)| {
            tower_coords
                .entry(tower)
                .and_modify(|coords: &mut Vec<(usize, usize)>| coords.push((row, col)))
                .or_insert_with(|| vec![(row, col)]);
        });

    for (_, similar) in tower_coords {
        for coords in
            antinode_coords(similar.as_slice(), harmonics, enumerate.row, enumerate.col).into_iter()
        {
            antinodes.insert(coords);
        }
        if harmonics > 1 && similar.len() > 1 {
            antinodes.extend(similar);
        }
    }

    antinodes.len()
}

pub fn part1(mut reader: impl Read) -> usize {
    let mut data = Vec::with_capacity(100_000);
    reader.read_to_end(&mut data).unwrap();

    count_antinodes(Enumerate2d::new(data.as_slice()), 1)
}

pub fn part2(mut reader: impl Read) -> usize {
    let mut data = Vec::with_capacity(100_000);
    reader.read_to_end(&mut data).unwrap();

    count_antinodes(Enumerate2d::new(data.as_slice()), usize::MAX)
}

type Enumerate2dIter<'a> = Enumerate<Split<'a, u8, fn(&u8) -> bool>>;

#[derive(Debug, Clone)]
struct Enumerate2d<'a> {
    iter: Enumerate2dIter<'a>,
    line: (usize, Enumerate<Iter<'a, u8>>),
    row: usize,
    col: usize,
}

impl<'a> Enumerate2d<'a> {
    fn new(data: &'a [u8]) -> Self {
        let f: fn(&u8) -> bool = |c| c == &b'\n';
        let mut iter = data.split(f).enumerate();
        let line = iter
            .next()
            .map(|(i, line)| (i, line.iter().enumerate()))
            .unwrap();
        let col = line.1.size_hint().1.unwrap();
        let row = iter.size_hint().1.unwrap() / col;
        Self {
            iter,
            line,
            row,
            col: col - 1,
        }
    }
}

impl Iterator for Enumerate2d<'_> {
    type Item = (usize, usize, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((col, val)) = self.line.1.next() {
            Some((self.line.0, col, *val))
        } else if let Some(new_line) = self.iter.next() {
            self.line = (new_line.0, new_line.1.iter().enumerate());
            self.next()
        } else {
            None
        }
    }
}
