use std::{
    cmp::Ordering,
    io::{BufRead, BufReader, Error, Read},
};

#[derive(Debug, Default)]
struct Safety {
    previous: u32,
    increasing: u32,
    decreasing: u32,
    stagnant: u32,
    max_change: u32,
}

fn test_safety(levels: impl Iterator<Item = u32>, threshold: u32) -> bool {
    let safety = levels.fold(Safety::default(), |mut safety, level| {
        if safety.previous == 0 {
            safety.previous = level;
        } else {
            let cmp = safety.previous.cmp(&level);
            match cmp {
                Ordering::Greater => safety.decreasing += 1,
                Ordering::Equal => safety.stagnant += 1,
                Ordering::Less => safety.increasing += 1,
            }
            safety.max_change = safety.max_change.max(safety.previous.abs_diff(level));
            safety.previous = level;
        }
        safety
    });
    ((safety.increasing + safety.stagnant <= threshold)
        || (safety.decreasing + safety.stagnant <= threshold))
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
        .filter(|levels| test_safety(levels.clone(), 0))
        .count()
}

pub fn part2(input: impl Read) -> usize {
    let buffer = BufReader::with_capacity(10000, input);

    let reports = buffer.lines();

    reports
        .map(parse_reports)
        .filter(|levels| test_safety(levels.clone(), 1))
        .count()
}
