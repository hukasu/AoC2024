use std::io::Read;

enum StateMachine {
    Start,
    ReadM,
    ReadU,
    ReadL,
    ReadLParen(u8),
    ReadingDigits1,
    ReadComma,
    ReadingDigits2,
    ReadD,
    ReadO,
    ReadN,
    ReadQuote,
    ReadT,
}

fn collect_all_mul(string: &str, disable_conditionals: bool) -> u32 {
    let mut chars = string.chars();
    let mut muls = Vec::with_capacity(1000);

    let mut state_machine = StateMachine::Start;

    let mut lhs = 0;
    let mut rhs = 0;

    let mut enabled = true;

    loop {
        let Some(c) = chars.next() else {
            break;
        };

        state_machine = match (state_machine, c) {
            (StateMachine::Start, 'm') => StateMachine::ReadM,
            (StateMachine::ReadM, 'u') => StateMachine::ReadU,
            (StateMachine::ReadU, 'l') => StateMachine::ReadL,
            (StateMachine::ReadL, '(') => StateMachine::ReadLParen(0),
            (StateMachine::ReadLParen(0) | StateMachine::ReadingDigits1, '0'..='9') => {
                lhs *= 10;
                lhs += c.to_digit(10).unwrap();
                StateMachine::ReadingDigits1
            }
            (StateMachine::ReadingDigits1, ',') => StateMachine::ReadComma,
            (StateMachine::ReadComma | StateMachine::ReadingDigits2, '0'..='9') => {
                rhs *= 10;
                rhs += c.to_digit(10).unwrap();
                StateMachine::ReadingDigits2
            }
            (StateMachine::ReadingDigits2, ')') => {
                muls.push(lhs * rhs * enabled as u32);
                lhs = 0;
                rhs = 0;
                StateMachine::Start
            }
            (StateMachine::Start, 'd') => StateMachine::ReadD,
            (StateMachine::ReadD, 'o') => StateMachine::ReadO,
            (StateMachine::ReadO, '(') => StateMachine::ReadLParen(1),
            (StateMachine::ReadO, 'n') => StateMachine::ReadN,
            (StateMachine::ReadN, '\'') => StateMachine::ReadQuote,
            (StateMachine::ReadQuote, 't') => StateMachine::ReadT,
            (StateMachine::ReadT, '(') => StateMachine::ReadLParen(2),
            (StateMachine::ReadLParen(1), ')') => {
                enabled = true;
                StateMachine::Start
            }
            (StateMachine::ReadLParen(2), ')') => {
                enabled = disable_conditionals;
                StateMachine::Start
            }
            _ => {
                lhs = 0;
                rhs = 0;
                StateMachine::Start
            }
        }
    }

    muls.iter().sum()
}

pub fn part1(mut reader: impl Read) -> u32 {
    let mut program = String::new();
    reader.read_to_string(&mut program).unwrap();

    collect_all_mul(program.as_str(), true)
}

pub fn part2(mut reader: impl Read) -> u32 {
    let mut program = String::new();
    reader.read_to_string(&mut program).unwrap();

    collect_all_mul(program.as_str(), false)
}
