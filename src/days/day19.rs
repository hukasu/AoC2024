use std::{collections::HashMap, fmt::Debug, io::Read, sync::Mutex};

pub fn part1(mut reader: impl Read) -> usize {
    let mut data = Vec::with_capacity(100_000);
    reader.read_to_end(&mut data).unwrap();

    let (towels, patterns) = parse_input(data.as_slice());

    match_patterns(&towels, patterns.as_slice())
}

pub fn part2(mut reader: impl Read) -> usize {
    let mut data = Vec::with_capacity(100_000);
    reader.read_to_end(&mut data).unwrap();

    let (towels, patterns) = parse_input(data.as_slice());

    count_patterns(&towels, patterns.as_slice())
}

fn parse_input(data: &[u8]) -> (Towels, Vec<&[u8]>) {
    let mut lines = data.split(|c| *c == b'\n');

    let towels = lines
        .next()
        .unwrap()
        .split(|c| *c == b',')
        .map(|slice| if slice[0] == b' ' { &slice[1..] } else { slice })
        .fold(Towels::default(), |mut towels, towel| {
            match towel[0] {
                b'b' => towels.black.push(towel),
                b'w' => towels.white.push(towel),
                b'r' => towels.red.push(towel),
                b'g' => towels.green.push(towel),
                b'u' => towels.blue.push(towel),
                _ => unreachable!("invalid towel stripe"),
            }
            towels
        });

    lines.next().unwrap();

    let patterns = lines.collect();

    (towels, patterns)
}

fn match_patterns(towels: &Towels, patterns: &[&[u8]]) -> usize {
    let iter = Mutex::new(patterns.iter().filter(|pattern| !pattern.is_empty()));

    std::thread::scope(|scope| {
        let (sender, receiver) = std::sync::mpsc::channel();

        for _ in 0..std::thread::available_parallelism().unwrap().get() {
            let sender_clone = sender.clone();
            let iter_ref = &iter;

            scope.spawn(move || {
                let sender = sender_clone;
                let iter = iter_ref;
                let mut cache = PatternCache::default();

                let mut pattern_opt = iter.lock().unwrap().next();
                loop {
                    let Some(pattern) = pattern_opt else {
                        break;
                    };
                    let possible = pattern_possible(pattern, towels, &mut cache);
                    if possible {
                        sender.send(()).unwrap();
                    }
                    pattern_opt = iter.lock().unwrap().next();
                }
            });
        }

        std::mem::drop(sender);

        receiver.into_iter().count()
    })
}

fn pattern_possible<'a>(
    pattern: &'a [u8],
    towels: &Towels<'a>,
    cache: &mut PatternCache<'a, bool>,
) -> bool {
    if pattern.is_empty() {
        return true;
    }
    let towels_with_stripe = match pattern[0] {
        b'b' => &towels.black,
        b'w' => &towels.white,
        b'r' => &towels.red,
        b'g' => &towels.green,
        b'u' => &towels.blue,
        _ => unreachable!("invalid towel stripe"),
    };
    towels_with_stripe
        .iter()
        .filter(|towel| pattern.starts_with(towel))
        .any(|towel| {
            if let Some(cached) = cache.get(&pattern[towel.len()..]) {
                cached
            } else if pattern_possible(&pattern[towel.len()..], towels, cache) {
                cache.insert(&pattern[towel.len()..], true);
                true
            } else {
                cache.insert(&pattern[towel.len()..], false);
                false
            }
        })
}

fn count_patterns(towels: &Towels, patterns: &[&[u8]]) -> usize {
    let iter = Mutex::new(patterns.iter().filter(|pattern| !pattern.is_empty()));

    std::thread::scope(|scope| {
        let (sender, receiver) = std::sync::mpsc::channel();

        for _ in 0..std::thread::available_parallelism().unwrap().get() {
            let sender_clone = sender.clone();
            let iter_ref = &iter;

            scope.spawn(move || {
                let sender = sender_clone;
                let iter = iter_ref;
                let mut cache = PatternCache::default();

                let mut pattern_opt = iter.lock().unwrap().next();
                loop {
                    let Some(pattern) = pattern_opt else {
                        break;
                    };
                    let possible = possible_patterns(pattern, towels, &mut cache);
                    sender.send(possible).unwrap();
                    pattern_opt = iter.lock().unwrap().next();
                }
            });
        }

        std::mem::drop(sender);

        receiver.into_iter().sum()
    })
}

fn possible_patterns<'a>(
    pattern: &'a [u8],
    towels: &Towels<'a>,
    cache: &mut PatternCache<'a, usize>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    let towels_with_stripe = match pattern[0] {
        b'b' => &towels.black,
        b'w' => &towels.white,
        b'r' => &towels.red,
        b'g' => &towels.green,
        b'u' => &towels.blue,
        _ => unreachable!("invalid towel stripe"),
    };
    towels_with_stripe
        .iter()
        .filter(|towel| pattern.starts_with(towel))
        .map(|towel| {
            if let Some(cached) = cache.get(&pattern[towel.len()..]) {
                cached
            } else {
                let count = possible_patterns(&pattern[towel.len()..], towels, cache);
                cache.insert(&pattern[towel.len()..], count);
                count
            }
        })
        .sum()
}

#[derive(Debug, Default)]
struct Towels<'a> {
    black: Vec<&'a [u8]>,
    white: Vec<&'a [u8]>,
    red: Vec<&'a [u8]>,
    green: Vec<&'a [u8]>,
    blue: Vec<&'a [u8]>,
}

#[derive(Debug, Default)]
struct PatternCache<'a, T> {
    cache: HashMap<&'a [u8], T>,
}

impl<'a, T: Copy + PartialEq + Debug> PatternCache<'a, T> {
    fn get(&self, key: &[u8]) -> Option<T> {
        self.cache.get(key).copied()
    }

    fn insert(&mut self, key: &'a [u8], value: T) {
        if let Some(exists) = self.cache.insert(key, value) {
            assert_eq!(
                exists, value,
                "In case there is an attempt to insert on the same key, value should be equals"
            );
        }
    }
}
