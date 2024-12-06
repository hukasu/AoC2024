#[test]
fn day1() {
    let data = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;
    assert_eq!(aoc2024::day1::part1(data.as_bytes()), 11);
    assert_eq!(aoc2024::day1::part2(data.as_bytes()), 31);
}

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

#[test]
fn day3() {
    let data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(aoc2024::day3::part1(data.as_bytes()), 161);
    let data = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(aoc2024::day3::part2(data.as_bytes()), 48);
}

#[test]
fn day4() {
    let data = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
    assert_eq!(aoc2024::day4::part1(data.as_bytes()), 18);
    assert_eq!(aoc2024::day4::part2(data.as_bytes()), 9);
}

#[test]
fn day5() {
    let data = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
    assert_eq!(aoc2024::day5::part1(data.as_bytes()), 143);
    assert_eq!(aoc2024::day5::part2(data.as_bytes()), 123);
}

#[test]
fn day6() {
    let data = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
    assert_eq!(aoc2024::day6::part1(data.as_bytes()), 41);
    assert_eq!(aoc2024::day6::part2(data.as_bytes()), 6);
}
