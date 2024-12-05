use std::{
    cmp::Ordering,
    collections::BTreeMap,
    io::{BufRead, BufReader, Read},
};

fn parse_page_order(reader: impl BufRead) -> BTreeMap<u32, Vec<u32>> {
    let mut map = BTreeMap::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line.trim().is_empty() {
            break;
        }

        let (l, r) = line
            .split_once('|')
            .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
            .unwrap();

        map.entry(l)
            .and_modify(|followers: &mut Vec<u32>| followers.push(r))
            .or_insert_with(|| vec![r]);
    }

    map
}

fn parse_updates(reader: impl BufRead) -> Vec<Vec<u32>> {
    let mut updates = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        updates.push(
            line.split_terminator(',')
                .map(|page| page.parse::<u32>().unwrap())
                .collect(),
        );
    }

    updates
}

fn update_in_order(pages: &[u32], page_order: &BTreeMap<u32, Vec<u32>>) -> bool {
    let [head, tail @ ..] = pages else {
        return true;
    };

    tail.iter().all(|r| {
        page_order
            .get(r)
            .filter(|followers| followers.contains(head))
            .is_none()
    }) && update_in_order(tail, page_order)
}

fn middle_page_of_valid_update(
    update: &[u32],
    page_order: &BTreeMap<u32, Vec<u32>>,
) -> Option<u32> {
    if update_in_order(update, page_order) {
        Some(update[update.len() / 2])
    } else {
        None
    }
}

fn middle_page_of_invalid_update(
    update: &mut [u32],
    page_order: &BTreeMap<u32, Vec<u32>>,
) -> Option<u32> {
    if update_in_order(update, page_order) {
        None
    } else {
        update.sort_by(|l, r| {
            if page_order
                .get(l)
                .filter(|followers| followers.contains(r))
                .is_some()
            {
                Ordering::Less
            } else if page_order
                .get(r)
                .filter(|followers| followers.contains(l))
                .is_some()
            {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        Some(update[update.len() / 2])
    }
}

pub fn part1(reader: impl Read) -> u32 {
    let mut buf = BufReader::with_capacity(100_000, reader);

    let page_order = parse_page_order(&mut buf);
    let updates = parse_updates(&mut buf);

    updates
        .iter()
        .filter_map(|update| middle_page_of_valid_update(update, &page_order))
        .sum()
}

pub fn part2(reader: impl Read) -> u32 {
    let mut buf = BufReader::with_capacity(100_000, reader);

    let page_order = parse_page_order(&mut buf);
    let mut updates = parse_updates(&mut buf);

    updates
        .iter_mut()
        .filter_map(|update| middle_page_of_invalid_update(update, &page_order))
        .sum()
}
