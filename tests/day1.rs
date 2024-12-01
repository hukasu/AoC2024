#[test]
fn day1() {
    let data = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;
    assert_eq!(aoc2024::day1_part1(data.as_bytes()), 11);
    assert_eq!(aoc2024::day1_part2(data.as_bytes()), 31);
}
