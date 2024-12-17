use std::io::{BufRead, BufReader, Read};

pub fn part1(reader: impl Read) -> String {
    let mut vm = Vm::parse(reader);
    vm.execute()
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

pub fn part2(reader: impl Read) -> u64 {
    let mut vm = Vm::parse(reader);
    vm.debug_a()
}

#[derive(Debug, Clone)]
struct Vm {
    a: u64,
    b: u64,
    c: u64,
    pc: usize,
    program: Vec<(u8, u8)>,
}

impl Vm {
    fn parse(data: impl Read) -> Self {
        let mut buf = BufReader::new(data).lines();

        let a = buf
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches("Register A: ")
            .parse::<u64>()
            .unwrap();
        let b = buf
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches("Register B: ")
            .parse::<u64>()
            .unwrap();
        let c = buf
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches("Register C: ")
            .parse::<u64>()
            .unwrap();

        buf.next().unwrap().unwrap();

        let program = buf
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches("Program: ")
            .split(',')
            .map(|op| op.parse::<u8>().unwrap())
            .scan(None, |state, op| {
                if let Some(opcode) = state.take() {
                    Some(Some((opcode, op)))
                } else {
                    let _ = state.insert(op);
                    Some(None)
                }
            })
            .flatten()
            .collect();

        Self {
            a,
            b,
            c,
            pc: 0,
            program,
        }
    }

    fn execute(&mut self) -> Vec<u8> {
        let mut out = Vec::with_capacity(500);

        while let Some((opcode, operand)) = self.program.get(self.pc) {
            self.pc += 1;

            match opcode {
                0 => self.adv(*operand),
                1 => self.bxl(*operand),
                2 => self.bst(*operand),
                3 => self.jnz(*operand),
                4 => self.bxc(*operand),
                5 => out.push(self.out(*operand)),
                6 => self.bdv(*operand),
                7 => self.cdv(*operand),
                _ => unreachable!("invalid opcode"),
            }
        }

        out.into_iter()
            .map(u8::try_from)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }

    fn debug_a(&mut self) -> u64 {
        let expected = self
            .program
            .iter()
            .flat_map(|(opcode, operand)| [u64::from(*opcode), u64::from(*operand)])
            .collect::<Vec<_>>();

        let mut queue = Vec::with_capacity(500);
        queue.push(0);

        for i in (0..(self.program.len() * 2)).rev() {
            for a in std::mem::take(&mut queue) {
                for j in 0u64..8 {
                    let mut vm = self.clone();
                    let test = j << (i * 3);
                    vm.a = a | test;
                    let out = vm.execute();
                    if let Some(digit) = out.get(i).copied() {
                        if u64::from(digit) == expected[i] {
                            queue.push(a | test);
                        }
                    }
                }
            }
        }

        queue.into_iter().min().unwrap()
    }

    fn adv(&mut self, operand: u8) {
        self.a >>= self.get_combo_operand(operand);
    }

    fn bxl(&mut self, operand: u8) {
        self.b ^= self.get_literal_operand(operand);
    }

    fn bst(&mut self, operand: u8) {
        self.b = self.get_combo_operand(operand) % 8;
    }

    fn jnz(&mut self, operand: u8) {
        if self.a != 0 {
            self.pc = usize::try_from(self.get_literal_operand(operand)).unwrap();
        }
    }

    fn bxc(&mut self, _operand: u8) {
        self.b ^= self.c;
    }

    fn out(&mut self, operand: u8) -> u64 {
        self.get_combo_operand(operand) % 8
    }

    fn bdv(&mut self, operand: u8) {
        self.b = self.a >> self.get_combo_operand(operand);
    }

    fn cdv(&mut self, operand: u8) {
        self.c = self.a >> self.get_combo_operand(operand);
    }

    fn get_combo_operand(&self, operand: u8) -> u64 {
        match operand {
            literal @ 0..=3 => u64::from(literal),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!("invalid combo operand"),
        }
    }

    fn get_literal_operand(&self, operand: u8) -> u64 {
        u64::from(operand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut vm = Vm {
            a: 0,
            b: 0,
            c: 9,
            pc: 0,
            program: vec![(2, 6)],
        };

        vm.execute();

        assert_eq!(vm.b, 1);
    }

    #[test]
    fn example2() {
        let mut vm = Vm {
            a: 10,
            b: 0,
            c: 0,
            pc: 0,
            program: vec![(5, 0), (5, 1), (5, 4)],
        };

        assert_eq!(vm.execute(), vec![0, 1, 2]);
    }

    #[test]
    fn example3() {
        let mut vm = Vm {
            a: 2024,
            b: 0,
            c: 0,
            pc: 0,
            program: vec![(0, 1), (5, 4), (3, 0)],
        };

        assert_eq!(vm.execute(), vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(vm.a, 0);
    }

    #[test]
    fn example4() {
        let mut vm = Vm {
            a: 0,
            b: 29,
            c: 0,
            pc: 0,
            program: vec![(1, 7)],
        };

        vm.execute();

        assert_eq!(vm.b, 26);
    }

    #[test]
    fn example5() {
        let mut vm = Vm {
            a: 0,
            b: 2024,
            c: 43690,
            pc: 0,
            program: vec![(4, 0)],
        };

        vm.execute();

        assert_eq!(vm.b, 44354);
    }
}
