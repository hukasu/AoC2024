use std::io::{BufRead, Read};

fn parse_input(data: &[u8]) -> impl Iterator<Item = (u64, Vec<u64>)> + '_ {
    data.lines().map(|res| {
        res.unwrap()
            .split_once(':')
            .map(|(res, operands)| {
                (
                    res.parse::<u64>().unwrap(),
                    operands
                        .split_terminator(' ')
                        .filter(|operand| !operand.is_empty())
                        .map(|operand| operand.parse::<u64>().unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .unwrap()
    })
}

fn is_valid_operation(result: u64, operands: &[u64], operators: &[fn(u64, u64) -> u64]) -> bool {
    let mut permutations = Permutations::new(operands.len() - 1, operators);
    permutations.any(|permutation: Vec<fn(u64, u64) -> u64>| {
        let mut permutation = permutation.into_iter();
        let operation = operands[1..].iter().fold(operands[0], |sum, operand| {
            permutation.next().unwrap()(sum, *operand)
        });
        operation == result
    })
}

pub fn part1(mut reader: impl Read) -> u64 {
    let mut data = Vec::with_capacity(100_000);
    reader.read_to_end(&mut data).unwrap();

    parse_input(&data)
        .filter_map(|(result, operands)| {
            if is_valid_operation(
                result,
                operands.as_slice(),
                &[std::ops::Add::add, std::ops::Mul::mul],
            ) {
                Some(result)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(mut reader: impl Read) -> u64 {
    let mut data = Vec::with_capacity(100_000);
    reader.read_to_end(&mut data).unwrap();

    parse_input(&data)
        .filter_map(|(result, operands)| {
            if is_valid_operation(
                result,
                operands.as_slice(),
                &[std::ops::Add::add, std::ops::Mul::mul, concatenate_u64],
            ) {
                Some(result)
            } else {
                None
            }
        })
        .sum()
}

fn concatenate_u64(lhs: u64, rhs: u64) -> u64 {
    let rhs_log = rhs.ilog10();
    lhs * 10u64.pow(rhs_log + 1) + rhs
}

struct Permutations<'a> {
    cur: Vec<usize>,
    operators: &'a [fn(u64, u64) -> u64],
    iterations: usize,
}

impl<'a> Permutations<'a> {
    fn new(len: usize, operators: &'a [fn(u64, u64) -> u64]) -> Self {
        Self {
            cur: vec![0; len],
            operators,
            iterations: 0,
        }
    }
}

impl Iterator for Permutations<'_> {
    type Item = Vec<fn(u64, u64) -> u64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iterations >= self.operators.len().pow(self.cur.len() as u32) {
            return None;
        }
        let mut to_mutate = 0;
        while let Some(mutate) = self.cur.get_mut(to_mutate) {
            if *mutate == self.operators.len() - 1 {
                *mutate = 0;
                to_mutate += 1;
            } else {
                *mutate += 1;
                to_mutate = usize::MAX;
            }
        }

        self.iterations += 1;
        Some(
            self.cur
                .iter()
                .map(|index| self.operators[*index])
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenate() {
        assert_eq!(concatenate_u64(1, 1), 11);
        assert_eq!(concatenate_u64(12, 1), 121);
        assert_eq!(concatenate_u64(1, 12), 112);
        assert_eq!(concatenate_u64(123, 1), 1231);
        assert_eq!(concatenate_u64(123, 456), 123456);
    }
}
