#[test]
fn day2() {
    let data = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;
    assert_eq!(aoc2024::day2::part1(data.as_bytes()), 2);
    assert_eq!(aoc2024::day2::part2(data.as_bytes()), 4);
}
