use std::{
    cmp::Ordering,
    io::{BufRead, BufReader, Error, Read},
};

use crate::skip_at::SkipAt;

#[derive(Debug, Default)]
struct Safety {
    previous: u32,
    increasing: bool,
    decreasing: bool,
    stagnant: bool,
    max_change: u32,
}

fn test_safety(levels: impl Iterator<Item = u32>) -> bool {
    let safety = levels.fold(Safety::default(), |mut safety, level| {
        if safety.previous == 0 {
            safety.previous = level;
        } else {
            let cmp = safety.previous.cmp(&level);
            safety.increasing = safety.increasing || cmp == Ordering::Less;
            safety.decreasing = safety.decreasing || cmp == Ordering::Greater;
            safety.stagnant = safety.stagnant || cmp == Ordering::Equal;
            safety.max_change = safety.max_change.max(safety.previous.abs_diff(level));
            safety.previous = level;
        }
        safety
    });
    ((safety.increasing && !safety.decreasing) || (!safety.increasing && safety.decreasing))
        && !safety.stagnant
        && (1..=3).contains(&safety.max_change)
}

fn parse_reports(report: Result<String, Error>) -> impl Iterator<Item = u32> + Clone {
    report
        .unwrap()
        .split_ascii_whitespace()
        .map(|str| str.to_string())
        .collect::<Vec<_>>()
        .into_iter()
        .map(|level| level.parse::<u32>().unwrap())
}

pub fn part1(input: impl Read) -> usize {
    let buffer = BufReader::with_capacity(10000, input);

    let reports = buffer.lines();

    reports
        .map(parse_reports)
        .filter(|levels| test_safety(levels.clone()))
        .count()
}

pub fn part2(input: impl Read) -> usize {
    let buffer = BufReader::with_capacity(10000, input);

    let reports = buffer.lines();

    reports
        .map(parse_reports)
        .filter(|levels| {
            test_safety(levels.clone())
                || (0..levels.size_hint().1.unwrap())
                    .any(|index| test_safety(SkipAt::new(levels.clone(), index)))
        })
        .count()
}
