use std::{
    collections::{BTreeSet, VecDeque},
    io::Read,
};

fn find_trailheads<'a>(map: &'a [&[u8]]) -> impl Iterator<Item = (usize, usize)> + 'a {
    map.iter().enumerate().flat_map(|(i, line)| {
        line.iter()
            .enumerate()
            .filter(|(_, c)| c == &&b'0')
            .map(move |(j, _)| (i, j))
    })
}

fn find_trails(map: &[&[u8]]) -> BTreeSet<Vec<(usize, usize)>> {
    let mut queue = VecDeque::from_iter(find_trailheads(map).map(|coord| vec![coord]));
    let mut trails = BTreeSet::new();

    while let Some(trail) = queue.pop_front() {
        let tail = trail.last().unwrap();
        let height = u8::try_from(trail.len() - 1).unwrap();
        for next in [
            tail.0
                .checked_add(1)
                .filter(|i| *i < map.len())
                .map(|i| (i, tail.1)),
            tail.0.checked_sub(1).map(|i| (i, tail.1)),
            // Assumes square map
            tail.1
                .checked_add(1)
                .filter(|j| *j < map.len())
                .map(|j| (tail.0, j)),
            tail.1.checked_sub(1).map(|j| (tail.0, j)),
        ]
        .into_iter()
        .flatten()
        {
            let next_height = map[next.0][next.1] - b'0';
            if next_height == height + 1 {
                let mut next_trail = trail.clone();
                next_trail.push(next);
                if next_height == 9 {
                    trails.insert(next_trail);
                } else {
                    queue.push_back(next_trail);
                }
            }
        }
    }

    trails
}

pub fn part1(mut reader: impl Read) -> usize {
    let mut data = Vec::with_capacity(10_000);
    reader.read_to_end(&mut data).unwrap();

    let map = data.split(|c| c == &b'\n').collect::<Vec<_>>();

    find_trails(map.as_slice())
        .into_iter()
        .map(|trail| (trail[0], trail[9]))
        .collect::<BTreeSet<_>>()
        .len()
}

pub fn part2(mut reader: impl Read) -> usize {
    let mut data = Vec::with_capacity(10_000);
    reader.read_to_end(&mut data).unwrap();

    let map = data.split(|c| c == &b'\n').collect::<Vec<_>>();

    find_trails(map.as_slice()).len()
}
