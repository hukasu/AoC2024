use std::{
    io::{BufRead, BufReader, Read},
    ops::RangeInclusive,
};

fn parse_input(reader: impl Read, padding: isize, range: RangeInclusive<isize>) -> Vec<Machine> {
    let mut buf = BufReader::with_capacity(10_000, reader).lines();
    let mut machines = Vec::with_capacity(50);

    loop {
        let button_a = parse_button(
            &buf.next().expect("Should have button a.").unwrap(),
            "Button A",
            '+',
        );
        let button_b = parse_button(
            &buf.next().expect("Should have button a.").unwrap(),
            "Button B",
            '+',
        );
        let prize = parse_button(
            &buf.next().expect("Should have button a.").unwrap(),
            "Prize",
            '=',
        );
        machines.push(Machine {
            button_a,
            button_b,
            prize: (prize.0 + padding, prize.1 + padding),
            range: range.clone(),
        });
        if buf.next().is_none() {
            break;
        }
    }

    machines
}

fn parse_button(line: &str, header: &str, operator: char) -> (isize, isize) {
    line.split_once(':')
        .and_then(|(left, right)| {
            assert_eq!(left, header);

            right.split_once(',').map(|(x, y)| {
                (
                    x.trim()
                        .trim_start_matches(&format!("X{operator}"))
                        .parse::<isize>()
                        .unwrap(),
                    y.trim()
                        .trim_start_matches(&format!("Y{operator}"))
                        .parse::<isize>()
                        .unwrap(),
                )
            })
        })
        .unwrap()
}

pub fn part1(reader: impl Read) -> isize {
    let machines = parse_input(reader, 0, 0isize..=100);

    machines
        .into_iter()
        .flat_map(|machine| machine.find_cheapest_solution())
        .sum()
}

pub fn part2(reader: impl Read) -> isize {
    let machines = parse_input(reader, 10000000000000, 0isize..=10000000000000);

    machines
        .into_iter()
        .flat_map(|machine| machine.find_cheapest_solution())
        .sum()
}

#[derive(Debug)]
struct Machine {
    button_a: (isize, isize),
    button_b: (isize, isize),
    prize: (isize, isize),
    range: RangeInclusive<isize>,
}

impl Machine {
    fn find_cheapest_solution(&self) -> Option<isize> {
        let b = (self.prize.0 * self.button_a.1 - self.prize.1 * self.button_a.0)
            / (self.button_b.0 * self.button_a.1 - self.button_a.0 * self.button_b.1);
        let a = (self.prize.1 - b * self.button_b.1) / self.button_a.1;
        if self.prize.0 == a * self.button_a.0 + b * self.button_b.0
            && self.prize.1 == a * self.button_a.1 + b * self.button_b.1
            && self.range.contains(&a)
            && self.range.contains(&b)
        {
            Some(a * 3 + b)
        } else {
            None
        }
    }
}
