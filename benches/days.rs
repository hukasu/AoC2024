use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    ///////////////
    //// Day 1 ////
    ///////////////
    let data = std::fs::read("inputs/day1.txt").unwrap();
    c.bench_function("day1_part1", |b| {
        b.iter(|| aoc2024::day1::part1(data.as_slice()))
    });
    c.bench_function("day1_part2", |b| {
        b.iter(|| aoc2024::day1::part2(data.as_slice()))
    });

    ///////////////
    //// Day 2 ////
    ///////////////
    let data = std::fs::read("inputs/day2.txt").unwrap();
    c.bench_function("day2_part1", |b| {
        b.iter(|| aoc2024::day2::part1(data.as_slice()))
    });
    c.bench_function("day2_part2", |b| {
        b.iter(|| aoc2024::day2::part2(data.as_slice()))
    });

    ///////////////
    //// Day 3 ////
    ///////////////
    let data = std::fs::read("inputs/day3.txt").unwrap();
    c.bench_function("day3_part1", |b| {
        b.iter(|| aoc2024::day3::part1(data.as_slice()))
    });
    c.bench_function("day3_part2", |b| {
        b.iter(|| aoc2024::day3::part2(data.as_slice()))
    });

    ///////////////
    //// Day 4 ////
    ///////////////
    let data = std::fs::read("inputs/day4.txt").unwrap();
    c.bench_function("day4_part1", |b| {
        b.iter(|| aoc2024::day4::part1(data.as_slice()))
    });
    c.bench_function("day4_part2", |b| {
        b.iter(|| aoc2024::day4::part2(data.as_slice()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
