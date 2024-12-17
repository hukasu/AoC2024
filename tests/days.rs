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

#[test]
fn day7() {
    let data = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
    assert_eq!(aoc2024::day7::part1(data.as_bytes()), 3749);
    assert_eq!(aoc2024::day7::part2(data.as_bytes()), 11387);
}

#[test]
fn day8() {
    let data = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
    assert_eq!(aoc2024::day8::part1(data.as_bytes()), 14);
    assert_eq!(aoc2024::day8::part2(data.as_bytes()), 34);
}

#[test]
fn day9() {
    let data = r#"2333133121414131402"#;
    assert_eq!(aoc2024::day9::part1(data.as_bytes()), 1928);
    assert_eq!(aoc2024::day9::part2(data.as_bytes()), 2858);
}

#[test]
fn day10() {
    let data = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
    assert_eq!(aoc2024::day10::part1(data.as_bytes()), 36);
    assert_eq!(aoc2024::day10::part2(data.as_bytes()), 81);
}

#[test]
fn day11() {
    let data = r#"125 17"#;
    assert_eq!(aoc2024::day11::part1(data.as_bytes()), 55312);
}

#[test]
fn day12() {
    let data = r#"AAAA
BBCD
BBCC
EEEC"#;
    assert_eq!(aoc2024::day12::part1(data.as_bytes()), 140);
    assert_eq!(aoc2024::day12::part2(data.as_bytes()), 80);

    let data = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;
    assert_eq!(aoc2024::day12::part1(data.as_bytes()), 772);
    assert_eq!(aoc2024::day12::part2(data.as_bytes()), 436);

    let data = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;
    assert_eq!(aoc2024::day12::part2(data.as_bytes()), 236);

    let data = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;
    assert_eq!(aoc2024::day12::part2(data.as_bytes()), 368);

    let data = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
    assert_eq!(aoc2024::day12::part1(data.as_bytes()), 1930);
    assert_eq!(aoc2024::day12::part2(data.as_bytes()), 1206);
}

#[test]
fn day13() {
    let data = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;
    assert_eq!(aoc2024::day13::part1(data.as_bytes()), 480);
}

#[test]
fn day14() {
    aoc2024::day14::BOUNDS.set((11, 7)).unwrap();

    let data = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;
    assert_eq!(aoc2024::day14::part1(data.as_bytes()), 12);
}

#[test]
fn day15() {
    let data = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;
    assert_eq!(aoc2024::day15::part1(data.as_bytes()), 2028);

    let data = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
    assert_eq!(aoc2024::day15::part1(data.as_bytes()), 10092);
    assert_eq!(aoc2024::day15::part2(data.as_bytes()), 9021);
}

#[test]
fn day16() {
    let data = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
    assert_eq!(aoc2024::day16::part1(data.as_bytes()), 7036);
    assert_eq!(aoc2024::day16::part2(data.as_bytes()), 45);

    let data = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;
    assert_eq!(aoc2024::day16::part1(data.as_bytes()), 11048);
    assert_eq!(aoc2024::day16::part2(data.as_bytes()), 64);
}

#[test]
fn day17() {
    let data = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"#;
    assert_eq!(
        aoc2024::day17::part1(data.as_bytes()),
        "4,6,3,5,6,3,5,2,1,0"
    );

    let data = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"#;
    assert_eq!(aoc2024::day17::part2(data.as_bytes()), 117440);

    assert_eq!(
        aoc2024::day17::part1(std::fs::File::open("inputs/day17.txt").unwrap()),
        "2,1,0,4,6,2,4,2,0"
    );
    assert_eq!(
        aoc2024::day17::part2(std::fs::File::open("inputs/day17.txt").unwrap()),
        109685330781408
    );
}
