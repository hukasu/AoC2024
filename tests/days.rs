use aoc2024::coord::Coord;

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
    assert_eq!(aoc2024::day14::part1_for_testing(data.as_bytes()), 12);
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
}

#[test]
fn day18() {
    let data = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"#;
    assert_eq!(aoc2024::day18::part1_for_test(data.as_bytes()), 22);
    assert_eq!(
        aoc2024::day18::part2_for_test(data.as_bytes()),
        Coord::new(6, 1)
    );
}

#[test]
fn day19() {
    let data = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"#;
    assert_eq!(aoc2024::day19::part1(data.as_bytes()), 6);
    assert_eq!(aoc2024::day19::part2(data.as_bytes()), 16);
}

#[test]
fn day20() {
    let data = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;
    assert_eq!(aoc2024::day20::part1(data.as_bytes()), 0);
    assert_eq!(aoc2024::day20::part2(data.as_bytes()), 0);
}

#[test]
fn day21() {
    let data = r#"029A
980A
179A
456A
379A
"#;
    assert_eq!(aoc2024::day21::part1(data.as_bytes()), 126384);
    assert_eq!(aoc2024::day21::part2(data.as_bytes()), 154115708116294);
}

#[test]
fn day22() {
    let data = r#"1
10
100
2024
"#;
    assert_eq!(aoc2024::day22::part1(data.as_bytes()), 37327623);

    let data = r#"1
2
3
2024
"#;
    assert_eq!(aoc2024::day22::part2(data.as_bytes()), 23);
}

#[test]
fn day23() {
    let data = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"#;
    assert_eq!(aoc2024::day23::part1(data.as_bytes()), 7);
    assert_eq!(aoc2024::day23::part2(data.as_bytes()), "co,de,ka,ta");
}

#[test]
fn day24() {
    let data = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
"#;
    assert_eq!(aoc2024::day24::part1(data.as_bytes()), 4);

    let data = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"#;
    assert_eq!(aoc2024::day24::part1(data.as_bytes()), 2024);
}
