use std::io::Read;

use crate::walker::Walker;

pub fn part1(mut reader: impl Read) -> usize {
    let mut data = Vec::with_capacity(100_000);
    reader.read_to_end(&mut data).unwrap();

    Walker::from_data(data.as_slice())
        .unwrap()
        .count_unique_steps()
}

pub fn part2(mut reader: impl Read) -> usize {
    let mut data = Vec::with_capacity(100_000);
    reader.read_to_end(&mut data).unwrap();

    Walker::from_data(data.as_slice())
        .unwrap()
        .find_possible_loops2()
}
