use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    io::Read,
};

pub fn part1(mut reader: impl Read) -> u64 {
    let mut data = Vec::with_capacity(10_000);
    reader.read_to_end(&mut data).unwrap();

    let (mut wires, operations) = parse(data.as_slice());

    wires.run_program(operations.as_slice());

    wires.z
}

pub fn part2(mut reader: impl Read) -> (String, usize) {
    let mut data = Vec::with_capacity(10_000);
    reader.read_to_end(&mut data).unwrap();

    let (_, operations) = parse(data.as_slice());

    let failures = find_failures(&operations);

    (
        failures
            .iter()
            .copied()
            .map(String::from_utf8_lossy)
            .collect::<Vec<_>>()
            .join(","),
        failures.len() / 2,
    )
}

fn parse(data: &[u8]) -> (Wires<'_>, Vec<Operation<'_>>) {
    let mut iter = data.split(|c| *c == b'\n');

    let mut wires = Wires::default();
    let mut operations = Vec::with_capacity(500);

    (&mut iter)
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let (l, r) = line.split_at(3);
            wires.set(l, r[2] - b'0');
        });

    iter.filter(|line| !line.is_empty()).for_each(|line| {
        let line = line.split(|c| *c == b' ').collect::<Vec<_>>();
        let [l, operator, r, _, store] = line.as_slice() else {
            unreachable!("Line always has 5 items.");
        };
        let operator = match operator {
            [b'A', b'N', b'D'] => Operator::And,
            [b'O', b'R'] => Operator::Or,
            [b'X', b'O', b'R'] => Operator::Xor,
            _ => unreachable!("Invalid operator"),
        };
        operations.push(Operation {
            l,
            operator,
            r,
            store,
        })
    });
    (wires, operations)
}

fn find_failures<'a>(operations: &[Operation<'a>]) -> BTreeSet<&'a [u8]> {
    let op_to_store = operations
        .iter()
        .map(|operation| {
            (
                (operation.l, operation.operator, operation.r),
                operation.store,
            )
        })
        .collect::<HashMap<_, _>>();
    let store_to_op = operations
        .iter()
        .map(|operation| {
            (
                operation.store,
                (operation.l, operation.operator, operation.r),
            )
        })
        .collect::<HashMap<_, _>>();

    let mut failures = BTreeSet::new();

    if !matches!(
        op_to_store
            .get(&(b"x00", Operator::Xor, b"y00"))
            .or_else(|| op_to_store.get(&(b"y00", Operator::Xor, b"x00")))
            .copied(),
        Some(b"z00")
    ) {
        failures.insert(b"z00".as_slice());
    }

    let mut carry_in = op_to_store
        .get(&(b"x00", Operator::And, b"y00"))
        .or_else(|| op_to_store.get(&(b"y00", Operator::Xor, b"x00")))
        .copied()
        .unwrap();
    let mut index = 1;

    loop {
        let x = [b'x', (index / 10) + b'0', (index % 10) + b'0'];
        let y = [b'y', (index / 10) + b'0', (index % 10) + b'0'];
        let z = [b'z', (index / 10) + b'0', (index % 10) + b'0'];

        // A operations `Xn XOR Yn` MUST exist,
        // if not, must have reached end of bits
        let Some(mut x_y_xor) = op_to_store
            .get(&(x.as_slice(), Operator::Xor, y.as_slice()))
            .or_else(|| op_to_store.get(&(y.as_slice(), Operator::Xor, x.as_slice())))
            .copied()
        else {
            break;
        };

        // A operations `Xn AND Yn` MUST exist,
        // if not, must have reached end of bits
        let Some(x_y_and) = op_to_store
            .get(&(x.as_slice(), Operator::And, y.as_slice()))
            .or_else(|| op_to_store.get(&(y.as_slice(), Operator::And, x.as_slice())))
            .copied()
        else {
            break;
        };

        // An operation `Zn = (Xn XOR Yn) XOR Cin` might exist
        if let Some(x_y_xor_cin_xor) = op_to_store
            .get(&(x_y_xor, Operator::Xor, carry_in))
            .or_else(|| op_to_store.get(&(carry_in, Operator::Xor, x_y_xor)))
            .copied()
        {
            // If an operation `(Xn XOR Yn) XOR Cin` exists but does not have `Zn` on the left side,
            // `Zn` has been swapped
            if x_y_xor_cin_xor != z.as_slice() {
                let (k, _) = store_to_op.get_key_value(z.as_slice()).unwrap();

                failures.insert(k);
                failures.insert(x_y_xor_cin_xor);
            }
        } else {
            // Operation that has `Zn` on the left side
            let inverse_z = store_to_op.get(z.as_slice()).unwrap();
            match (
                inverse_z.0 == x_y_xor,
                inverse_z.2 == x_y_xor,
                inverse_z.0 == carry_in,
                inverse_z.2 == carry_in,
            ) {
                (true, false, false, true) | (false, true, true, false) => {
                    unreachable!("Should've found an operation `(Xn XOR Yn) XOR Cin`")
                }
                (true, true, _, _) | (_, _, true, true) => {
                    unreachable!(
                        "A register can't be used twice. Operation {{ {} {:?} {} }}, l: {}, r: {} ",
                        String::from_utf8_lossy(inverse_z.0),
                        inverse_z.1,
                        String::from_utf8_lossy(inverse_z.2),
                        String::from_utf8_lossy(x_y_xor),
                        String::from_utf8_lossy(carry_in)
                    )
                }
                (true, _, true, _) | (_, true, _, true) => {
                    unreachable!("A register shouldn't match both sides. Operation {{ {} {:?} {} }}, l: {}, r: {} ",
                        String::from_utf8_lossy(inverse_z.0),
                        inverse_z.1,
                        String::from_utf8_lossy(inverse_z.2),
                        String::from_utf8_lossy(x_y_xor),
                        String::from_utf8_lossy(carry_in))
                }
                // An operation `Zn = `(Xn XOR Yn) XOR Cin` exists, but has wrong `Cin`
                (true, false, false, false) => {
                    failures.insert(carry_in);
                    carry_in = inverse_z.2;
                }
                (false, true, false, false) => {
                    failures.insert(carry_in);
                    carry_in = inverse_z.0;
                }
                // An operation `Zn = `(Xn XOR Yn) XOR Cin` exists, but has wrong `(Xn XOR Yn)`
                (false, false, true, false) => {
                    failures.insert(x_y_xor);
                    x_y_xor = inverse_z.2;
                }
                (false, false, false, true) => {
                    failures.insert(x_y_xor);
                    x_y_xor = inverse_z.0;
                }
                // An operation `Zn = `(Xn XOR Yn) XOR Cin` exists, but has wrong `(Xn XOR Yn)` and `Cin`
                (false, false, false, false) => {
                    unreachable!("Can't have both sides of the operation wrong.");
                }
            }
        };

        // A operations `(Xn XOR Yn) AND Cin` MUST exist
        let Some(x_y_xor_cin_and) = op_to_store
            .get(&(x_y_xor, Operator::And, carry_in))
            .or_else(|| op_to_store.get(&(carry_in, Operator::And, x_y_xor)))
            .copied()
        else {
            unreachable!("Should have fixed the inputs.");
        };

        // A operations `(Xn And Yn) OR ((Xn XOR Yn) AND Cin)` might exist
        let x_y_and_x_y_xor_cin_and_or_opt = op_to_store
            .get(&(x_y_and, Operator::Or, x_y_xor_cin_and))
            .or_else(|| op_to_store.get(&(x_y_xor_cin_and, Operator::Or, x_y_and)))
            .copied();
        let x_y_and_x_y_xor_cin_and_or =
            if let Some(x_y_and_x_y_xor_cin_and_or) = x_y_and_x_y_xor_cin_and_or_opt {
                // If it exists it is next `Cin`
                x_y_and_x_y_xor_cin_and_or
            } else {
                let z_next = [b'z', ((index + 1) / 10) + b'0', ((index + 1) % 10) + b'0'];
                // Operation that has `Zn+1` on the left side
                let inverse_z = store_to_op.get(z_next.as_slice()).unwrap();

                let mut res = None;

                let left = store_to_op.get(inverse_z.0).unwrap();
                match (
                    left.0 == x_y_and,
                    left.2 == x_y_and,
                    left.0 == x_y_xor_cin_and,
                    left.2 == x_y_xor_cin_and,
                ) {
                    (false, false, false, false) => (),
                    (true, true, _, _)
                    | (_, _, true, true)
                    | (true, _, true, _)
                    | (_, true, _, true) => {
                        unreachable!(
                            "Impossible case. Operation {{ {} {:?} {} }}, l: {}, r: {} ",
                            String::from_utf8_lossy(left.0),
                            left.1,
                            String::from_utf8_lossy(left.2),
                            String::from_utf8_lossy(x_y_and),
                            String::from_utf8_lossy(x_y_xor_cin_and)
                        )
                    }
                    (false, true, true, false) | (true, false, false, true) => {
                        unreachable!(
                            "Should've found on get. Operation {{ {} {:?} {} }}, l: {}, r: {} ",
                            String::from_utf8_lossy(left.0),
                            left.1,
                            String::from_utf8_lossy(left.2),
                            String::from_utf8_lossy(x_y_and),
                            String::from_utf8_lossy(x_y_xor_cin_and)
                        )
                    }
                    (true, false, false, false) => {
                        failures.insert(x_y_xor_cin_and);
                        res = Some(inverse_z.0);
                    }
                    (false, true, false, false) => {
                        failures.insert(x_y_xor_cin_and);
                        res = Some(inverse_z.0);
                    }
                    (false, false, true, false) => {
                        failures.insert(x_y_and);
                        res = Some(inverse_z.0);
                    }
                    (false, false, false, true) => {
                        failures.insert(x_y_and);
                        res = Some(inverse_z.0);
                    }
                }

                if res.is_none() {
                    let right = store_to_op.get(inverse_z.2).unwrap();
                    match (
                        right.0 == x_y_and,
                        right.2 == x_y_and,
                        right.0 == x_y_xor_cin_and,
                        right.2 == x_y_xor_cin_and,
                    ) {
                        (false, false, false, false)
                        | (true, true, _, _)
                        | (_, _, true, true)
                        | (true, _, true, _)
                        | (_, true, _, true) => {
                            unreachable!(
                                "Impossible case. Operation {{ {} {:?} {} }}, l: {}, r: {} ",
                                String::from_utf8_lossy(right.0),
                                right.1,
                                String::from_utf8_lossy(right.2),
                                String::from_utf8_lossy(x_y_and),
                                String::from_utf8_lossy(x_y_xor_cin_and)
                            )
                        }
                        (false, true, true, false) | (true, false, false, true) => {
                            unreachable!(
                                "Should've found on get. Operation {{ {} {:?} {} }}, l: {}, r: {} ",
                                String::from_utf8_lossy(right.0),
                                right.1,
                                String::from_utf8_lossy(right.2),
                                String::from_utf8_lossy(x_y_and),
                                String::from_utf8_lossy(x_y_xor_cin_and)
                            )
                        }
                        (true, false, false, false) => {
                            failures.insert(x_y_xor_cin_and);
                            res = Some(inverse_z.2);
                        }
                        (false, true, false, false) => {
                            failures.insert(x_y_xor_cin_and);
                            res = Some(inverse_z.2);
                        }
                        (false, false, true, false) => {
                            failures.insert(x_y_and);
                            res = Some(inverse_z.2);
                        }
                        (false, false, false, true) => {
                            failures.insert(x_y_and);
                            res = Some(inverse_z.2);
                        }
                    }
                }

                res.unwrap()
            };

        carry_in = x_y_and_x_y_xor_cin_and_or;

        index += 1;
    }

    let last_carry_in = &[b'z', (index / 10) + b'0', (index % 10) + b'0'];
    if carry_in != last_carry_in {
        failures.insert(carry_in);
    }

    failures
}

#[derive(Debug, Default, Clone)]
struct Wires<'a> {
    x: u64,
    y: u64,
    z: u64,
    intermediates: HashMap<&'a [u8], u8>,
}

impl<'a> Wires<'a> {
    fn run_program(&mut self, operations: &[Operation<'a>]) {
        let mut operations = VecDeque::from_iter(operations);
        while let Some(operator) = operations.pop_front() {
            if !self.execute(operator.l, operator.operator, operator.r, operator.store) {
                operations.push_back(operator);
            }
        }
    }

    fn set(&mut self, register: &'a [u8], bit: u8) {
        assert!((0..=1).contains(&bit));
        match register.split_at(1) {
            ([b'x'], suffix) if suffix[0].is_ascii_digit() && suffix[1].is_ascii_digit() => {
                let index = (suffix[0] - b'0') * 10 + (suffix[1] - b'0');
                let mask = 1 << index;
                self.x = (self.x & !mask) | (u64::from(bit) << index);
            }
            ([b'y'], suffix) if suffix[0].is_ascii_digit() && suffix[1].is_ascii_digit() => {
                let index = (suffix[0] - b'0') * 10 + (suffix[1] - b'0');
                let mask = 1 << index;
                self.y = (self.y & !mask) | (u64::from(bit) << index);
            }
            ([b'z'], suffix) if suffix[0].is_ascii_digit() && suffix[1].is_ascii_digit() => {
                let index = (suffix[0] - b'0') * 10 + (suffix[1] - b'0');
                let mask = 1 << index;
                self.z = (self.z & !mask) | (u64::from(bit) << index);
            }
            _ => {
                assert!(!self.intermediates.contains_key(register));
                self.intermediates.insert(register, bit);
            }
        }
    }

    fn get(&self, register: &[u8]) -> Option<u8> {
        match register.split_at(1) {
            ([b'x'], suffix) if suffix[0].is_ascii_digit() && suffix[1].is_ascii_digit() => {
                let index = (suffix[0] - b'0') * 10 + (suffix[1] - b'0');
                Some(u8::try_from((self.x >> index) & 1).unwrap())
            }
            ([b'y'], suffix) if suffix[0].is_ascii_digit() && suffix[1].is_ascii_digit() => {
                let index = (suffix[0] - b'0') * 10 + (suffix[1] - b'0');
                Some(u8::try_from((self.y >> index) & 1).unwrap())
            }
            ([b'z'], suffix) if suffix[0].is_ascii_digit() && suffix[1].is_ascii_digit() => {
                let index = (suffix[0] - b'0') * 10 + (suffix[1] - b'0');
                Some(u8::try_from((self.z >> index) & 1).unwrap())
            }
            _ => self.intermediates.get(register).copied(),
        }
    }

    fn execute(&mut self, l: &[u8], operator: Operator, r: &[u8], store: &'a [u8]) -> bool {
        if let (Some(l), Some(r)) = (self.get(l), self.get(r)) {
            assert!((0..=1).contains(&l));
            assert!((0..=1).contains(&r));
            self.set(store, operator.func()(l, r));
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
struct Operation<'a> {
    l: &'a [u8],
    operator: Operator,
    r: &'a [u8],
    store: &'a [u8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    And,
    Or,
    Xor,
}

impl Operator {
    fn func(&self) -> fn(u8, u8) -> u8 {
        match self {
            Self::And => std::ops::BitAnd::bitand,
            Self::Or => std::ops::BitOr::bitor,
            Self::Xor => std::ops::BitXor::bitxor,
        }
    }
}
