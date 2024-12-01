use std::{
    collections::BTreeMap,
    io::{BufRead, BufReader, Read},
    ops::AddAssign,
};

pub fn day1_part1(reader: impl Read) -> u32 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let buffer = BufReader::new(reader);

    for line in buffer.lines() {
        let line = line.expect("Line should be Ok");

        let split = line.split_ascii_whitespace().collect::<Vec<_>>();

        match split.as_slice() {
            [lhs, rhs] => {
                left.push(lhs.parse::<i32>().expect("Could not parse lhs"));
                right.push(rhs.parse::<i32>().expect("Could not parse rhs"));
            }
            _ => panic!("Splitting the line did not yield 2 values"),
        }
    }

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(lhs, rhs)| lhs.abs_diff(*rhs))
        .sum()
}

pub fn day1_part2(reader: impl Read) -> i32 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let buffer = BufReader::new(reader);

    for line in buffer.lines() {
        let line = line.expect("Line should be Ok");

        let split = line.split_ascii_whitespace().collect::<Vec<_>>();

        match split.as_slice() {
            [lhs, rhs] => {
                left.push(lhs.parse::<i32>().expect("Could not parse lhs"));
                right.push(rhs.parse::<i32>().expect("Could not parse rhs"));
            }
            _ => panic!("Splitting the line did not yield 2 values"),
        }
    }

    let mut map = BTreeMap::new();
    for rhs in right.iter() {
        map.entry(*rhs)
            .and_modify(|old: &mut i32| old.add_assign(1))
            .or_insert(1);
    }

    left.iter()
        .map(|lhs| lhs * map.get(lhs).unwrap_or(&0))
        .sum()
}
