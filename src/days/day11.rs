use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

fn parse_stones(reader: impl Read) -> HashMap<u64, usize> {
    let mut res = HashMap::new();
    BufReader::new(reader)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|value| value.parse::<u64>().unwrap())
        .fold(&mut res, |map, value| {
            map.entry(value)
                .and_modify(|count| *count += 1)
                .or_insert(1usize);
            map
        });
    res
}

fn push_stone(stones: &mut HashMap<u64, usize>, value: u64, previous_count: usize) {
    stones
        .entry(value)
        .and_modify(|count| *count += previous_count)
        .or_insert(previous_count);
}

fn evolve(stones: &mut HashMap<u64, usize>, blinks: usize) {
    for _ in 0..blinks {
        let mut buffer = HashMap::new();
        blink(stones, &mut buffer);
        std::mem::swap(stones, &mut buffer);
    }
}

fn blink(stones: &mut HashMap<u64, usize>, buffer: &mut HashMap<u64, usize>) {
    for (value, stone_count) in stones.iter() {
        match *value {
            0 => {
                push_stone(buffer, 1, *stone_count);
            }
            n => {
                let log = n.ilog10();
                if log % 2 == 1 {
                    let log = 10u64.pow(log.div_ceil(2));
                    let l = n / log;
                    let r = n % log;

                    push_stone(buffer, l, *stone_count);
                    push_stone(buffer, r, *stone_count);
                } else {
                    push_stone(buffer, n * 2024, *stone_count);
                }
            }
        }
    }
}

pub fn part1(reader: impl Read) -> usize {
    let mut stones = parse_stones(reader);
    evolve(&mut stones, 25);
    stones.into_values().sum()
}

pub fn part2(reader: impl Read) -> usize {
    let mut stones = parse_stones(reader);
    evolve(&mut stones, 75);
    stones.into_values().sum()
}
