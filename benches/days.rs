use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! make_day_bench {
    ($c:expr, $day:ident) => {
        let data = std::fs::read(format!("inputs/{}.txt", stringify!($day))).unwrap();
        $c.bench_function(&format!("{}_part1", stringify!($day)), |b| {
            b.iter(|| aoc2024::$day::part1(data.as_slice()))
        });
        $c.bench_function(&format!("{}_part2", stringify!($day)), |b| {
            b.iter(|| aoc2024::$day::part2(data.as_slice()))
        });
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    make_day_bench!(c, day1);
    make_day_bench!(c, day2);
    make_day_bench!(c, day3);
    make_day_bench!(c, day4);
    make_day_bench!(c, day5);
    make_day_bench!(c, day6);
    make_day_bench!(c, day7);
    make_day_bench!(c, day8);
    make_day_bench!(c, day9);
    make_day_bench!(c, day10);
    make_day_bench!(c, day11);
    make_day_bench!(c, day12);
    make_day_bench!(c, day13);
    make_day_bench!(c, day14);
    make_day_bench!(c, day15);
    make_day_bench!(c, day16);
    make_day_bench!(c, day17);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
