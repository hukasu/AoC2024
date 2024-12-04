use std::io::Read;

const PATTERN: &str = "XMAS";
const INVERSE_PATTERN: &str = "SAMX";
const XPATTERN: &str = "MAS";
const XINVERSE_PATTERN: &str = "SAM";

fn test_horizontal(word_search: &[&[u8]], pattern: &[u8]) -> usize {
    word_search
        .iter()
        .map(|line| line.windows(4).filter(|chunk| chunk == &pattern).count())
        .sum()
}

fn test_vertical(word_search: &[&[u8]], pattern: &[u8]) -> usize {
    word_search
        .windows(4)
        .map(|window| {
            let [a, b, c, d] = window else {
                panic!("Windows did not have size of 4 (1)");
            };

            a.iter()
                .zip(b.iter())
                .zip(c.iter().zip(d.iter()))
                .filter(|((a, b), (c, d))| [**a, **b, **c, **d] == pattern)
                .count()
        })
        .sum()
}

fn test_diagonal_right(word_search: &[&[u8]], pattern: &[u8]) -> usize {
    word_search
        .windows(4)
        .map(|window| {
            let [a, b, c, d] = window else {
                panic!("Windows did not have size of 4 (2)");
            };

            a.iter()
                .zip(b.iter().skip(1))
                .zip(c.iter().skip(2).zip(d.iter().skip(3)))
                .filter(|((a, b), (c, d))| [**a, **b, **c, **d] == pattern)
                .count()
        })
        .sum()
}

fn test_diagonal_left(word_search: &[&[u8]], pattern: &[u8]) -> usize {
    word_search
        .windows(4)
        .map(|window| {
            let [a, b, c, d] = window else {
                panic!("Windows did not have size of 4 (3)");
            };

            a.iter()
                .skip(3)
                .zip(b.iter().skip(2))
                .zip(c.iter().skip(1).zip(d.iter()))
                .filter(|((a, b), (c, d))| [**a, **b, **c, **d] == pattern)
                .count()
        })
        .sum()
}

#[allow(non_snake_case)]
fn test_X(word_search: &[&[u8]], pattern_left: &[u8], pattern_right: &[u8]) -> usize {
    word_search
        .windows(3)
        .map(|window| {
            let [a, b, c] = window else {
                panic!("Windows did not have size of 3");
            };
            let left = a
                .iter()
                .zip(b.iter().skip(1))
                .zip(c.iter().skip(2))
                .map(|((a, b), c)| [*a, *b, *c]);
            let right = a
                .iter()
                .skip(2)
                .zip(b.iter().skip(1))
                .zip(c.iter())
                .map(|((a, b), c)| [*a, *b, *c]);

            left.zip(right)
                .filter(|(left, right)| left == pattern_left && right == pattern_right)
                .count()
        })
        .sum()
}

pub fn part1(mut reader: impl Read) -> usize {
    let mut buf = Vec::with_capacity(20_000);
    reader.read_to_end(&mut buf).unwrap();

    let word_search = buf.split(|byte| byte == &b'\n').collect::<Vec<_>>();
    let word_search_ref = word_search.as_slice();

    std::thread::scope(|scope| {
        let handles = [
            scope.spawn(|| test_horizontal(word_search_ref, PATTERN.as_bytes())),
            scope.spawn(|| test_horizontal(word_search_ref, INVERSE_PATTERN.as_bytes())),
            scope.spawn(|| test_vertical(word_search_ref, PATTERN.as_bytes())),
            scope.spawn(|| test_vertical(word_search_ref, INVERSE_PATTERN.as_bytes())),
            scope.spawn(|| test_diagonal_right(word_search_ref, PATTERN.as_bytes())),
            scope.spawn(|| test_diagonal_right(word_search_ref, INVERSE_PATTERN.as_bytes())),
            scope.spawn(|| test_diagonal_left(word_search_ref, PATTERN.as_bytes())),
            scope.spawn(|| test_diagonal_left(word_search_ref, INVERSE_PATTERN.as_bytes())),
        ];

        handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .sum()
    })
}

pub fn part2(mut reader: impl Read) -> usize {
    let mut buf = Vec::with_capacity(20_000);
    reader.read_to_end(&mut buf).unwrap();

    let word_search = buf.split(|byte| byte == &b'\n').collect::<Vec<_>>();
    let word_search_ref = word_search.as_slice();

    std::thread::scope(|scope| {
        let handles = [
            scope.spawn(|| test_X(word_search_ref, XPATTERN.as_bytes(), XPATTERN.as_bytes())),
            scope.spawn(|| {
                test_X(
                    word_search_ref,
                    XPATTERN.as_bytes(),
                    XINVERSE_PATTERN.as_bytes(),
                )
            }),
            scope.spawn(|| {
                test_X(
                    word_search_ref,
                    XINVERSE_PATTERN.as_bytes(),
                    XPATTERN.as_bytes(),
                )
            }),
            scope.spawn(|| {
                test_X(
                    word_search_ref,
                    XINVERSE_PATTERN.as_bytes(),
                    XINVERSE_PATTERN.as_bytes(),
                )
            }),
        ];

        handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .sum()
    })
}
