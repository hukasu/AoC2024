use std::{
    collections::{BTreeMap, BTreeSet},
    io::{BufRead, BufReader, Read},
};

pub fn part1(reader: impl Read) -> u64 {
    let mut input = parse_input(reader);
    update_secrets(&mut input, 2000);
    input.into_iter().sum()
}

pub fn part2(reader: impl Read) -> u64 {
    let input = parse_input(reader);
    let time_series = time_series(&input, 2000);

    let unique_windows = BTreeSet::from_iter(
        time_series
            .iter()
            .flat_map(|time_series| time_series.price_changes_windows.keys()),
    );

    unique_windows
        .into_iter()
        .map(|windows| {
            time_series
                .iter()
                .flat_map(|time_series| time_series.price_changes_windows.get(windows))
                .sum()
        })
        .max()
        .unwrap()
}

fn parse_input(reader: impl Read) -> Vec<u64> {
    let buf = BufReader::with_capacity(10_000, reader);

    buf.lines()
        .map(|line| line.unwrap().parse::<u64>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn update_secrets(input: &mut [u64], updates: usize) {
    std::thread::scope(|scope| {
        let paralelism = std::thread::available_parallelism().unwrap().get();
        let input_len = input.len();
        for chunk in input.chunks_mut((input_len / paralelism) + 1) {
            scope.spawn(|| {
                for num in chunk {
                    for _ in 0..updates {
                        *num = process(*num);
                    }
                }
            });
        }
    });
}

#[derive(Debug, Default, Clone)]
struct TimeSeries {
    price_time_series: Vec<u64>,
    price_changes: Vec<i64>,
    price_changes_windows: BTreeMap<[i64; 4], u64>,
}

fn time_series(input: &[u64], updates: usize) -> Vec<TimeSeries> {
    let mut time_series = vec![TimeSeries::default(); input.len()];

    std::thread::scope(|scope| {
        let paralelism = std::thread::available_parallelism().unwrap().get();
        let input_len = input.len();

        let input_chunks = input.chunks((input_len / paralelism) + 1);
        let time_series_chunks = time_series.chunks_mut((input_len / paralelism) + 1);

        for (input_chunk, time_series_chunk) in input_chunks.zip(time_series_chunks) {
            scope.spawn(move || {
                for (num, time_series) in input_chunk.iter().zip(time_series_chunk.iter_mut()) {
                    let mut secret = *num;
                    let mut prev = None;
                    let mut windows = Vec::new();
                    for _ in 0..updates {
                        let price = secret % 10;
                        time_series.price_time_series.push(price);
                        if let Some(prev) = prev {
                            let price_change =
                                i64::try_from(price).unwrap() - i64::try_from(prev).unwrap();
                            time_series.price_changes.push(price_change);
                            windows.push(price_change);
                            if windows.len() > 4 {
                                windows.remove(0);
                            }
                        }
                        if windows.len() == 4 {
                            let window = windows.as_slice().try_into().unwrap();
                            time_series
                                .price_changes_windows
                                .entry(window)
                                .or_insert(price);
                        }

                        secret = process(secret);
                        prev.replace(price);
                    }
                }
            });
        }
    });

    time_series
}

fn process(secret: u64) -> u64 {
    let first_round = prune(mix(secret * 64, secret));
    let second_round = prune(mix(first_round / 32, first_round));
    prune(mix(second_round * 2048, second_round))
}

fn mix(value: u64, secret: u64) -> u64 {
    value ^ secret
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mix_test() {
        assert_eq!(mix(15, 42), 37);
    }

    #[test]
    fn prune_test() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn process_test() {
        let secret = process(123);
        assert_eq!(secret, 15887950);

        let secret = process(secret);
        assert_eq!(secret, 16495136);

        let secret = process(secret);
        assert_eq!(secret, 527345);

        let secret = process(secret);
        assert_eq!(secret, 704524);

        let secret = process(secret);
        assert_eq!(secret, 1553684);

        let secret = process(secret);
        assert_eq!(secret, 12683156);

        let secret = process(secret);
        assert_eq!(secret, 11100544);

        let secret = process(secret);
        assert_eq!(secret, 12249484);

        let secret = process(secret);
        assert_eq!(secret, 7753432);

        let secret = process(secret);
        assert_eq!(secret, 5908254);
    }

    #[test]
    fn time_series_test() {
        let input = [123];
        let time_series = time_series(input.as_slice(), 10);
        assert_eq!(
            time_series[0].price_time_series.as_slice(),
            &[3, 0, 6, 5, 4, 4, 6, 4, 4, 2]
        );
        assert_eq!(
            time_series[0].price_changes.as_slice(),
            &[-3, 6, -1, -1, 0, 2, -2, 0, -2]
        );
    }
}
