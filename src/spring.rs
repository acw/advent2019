use crate::machine::{Computer, RunResult};
use std::collections::VecDeque;

struct Jumper {
    result: i64
}

impl Jumper {
    fn new(file: &str, program: Program, command: &str) -> Jumper {
        let mut computer = Computer::load(file);
        let mut encoding = program.encode();

        for c in command.chars() {
            encoding.push_back(c as u8 as i64);

        }
        encoding.push_back('\n' as u8 as i64);
        let result = loop {
            match computer.run() {
                RunResult::Continue(next) => computer = next,
                RunResult::Halted(_) => panic!("Machine halted?!"),
                RunResult::Output(o, next) => {
                    if o > 128 {
                        break o;
                    }
                    print!("{}", o as u8 as char);
                    computer = next;
                }
                RunResult::Input(function) => {
                    match encoding.pop_front() {
                        None =>
                            panic!("Ran out of input?!"),
                        Some(x) =>
                            computer = function(x),
                    }
                }
            }
        };


        Jumper{ result }
    }
}

#[test]
fn day21() {
    let jumper1 = Jumper::new("inputs/day21", Program::step1(), "WALK");
    assert_eq!(19357761, jumper1.result);
    let jumper2 = Jumper::new("inputs/day21", Program::step2(), "RUN");
    assert_eq!(1142249706, jumper2.result);
}

struct Program {
    parts: Vec<Instruction>
}

impl Program {
    fn step1() -> Program {
        Program {
            parts: vec![ Instruction::Not(Register::OneAway,   Register::Temp), // Temp = One Away is Hole
                         Instruction::Not(Register::TwoAway,   Register::Jump), // Temp = Two Away is Hole
                         Instruction::Or( Register::Jump,      Register::Temp), // Temp = Either One/Two Away is Hole
                         Instruction::Not(Register::ThreeAway, Register::Jump),
                         Instruction::Or( Register::Jump,      Register::Temp),
                         Instruction::Not(Register::OneAway,   Register::Jump), // Jump = !one
                         Instruction::And(Register::OneAway,   Register::Jump), // Jump = false
                         Instruction::Or( Register::FourAway,  Register::Jump), // Jump = Three away is not hole
                         Instruction::And(Register::Temp,      Register::Jump), // Jump = Landing is not hole, one/two is
                       ]
        }
    }

    fn step2() -> Program {
        Program {
            parts: vec![
                // if any of the next three are empty
                Instruction::Not(Register::OneAway,   Register::Jump),
                Instruction::Not(Register::TwoAway,   Register::Temp),
                Instruction::Or( Register::Temp,      Register::Jump),
                Instruction::Not(Register::ThreeAway, Register::Temp),
                Instruction::Or( Register::Temp,      Register::Jump),
                // and if we're not going to land on a blank
                Instruction::Not(Register::FourAway,  Register::Temp),
                Instruction::And(Register::FourAway,  Register::Temp),
                Instruction::Or( Register::FourAway,  Register::Temp),
                Instruction::And(Register::Temp,      Register::Jump),
                // and the thing after that or four ahead of that isn't blank
                Instruction::Not(Register::FourAway,  Register::Temp),
                Instruction::And(Register::FourAway,  Register::Temp),
                Instruction::Or( Register::EightAway, Register::Temp),
                Instruction::Or( Register::FiveAway,  Register::Temp),
                Instruction::And(Register::Temp,      Register::Jump),
            ]
        }
    }

    fn encode(&self) -> VecDeque<i64> {
        let mut res = VecDeque::new();

        for part in self.parts.iter() {
            res.extend(part.encode());
        }

        res
    }
}


enum Instruction {
    And(Register, Register),
    Or(Register, Register),
    Not(Register, Register)
}

impl Instruction {
    fn encode(&self) -> Vec<i64> {
        let (mut res, x, y) = match self {
            Instruction::And(x, y) => (vec!['A' as u8 as i64,
                                            'N' as u8 as i64,
                                            'D' as u8 as i64], x, y),
            Instruction::Or(x, y)  => (vec!['O' as u8 as i64,
                                            'R' as u8 as i64], x, y),
            Instruction::Not(x, y) => (vec!['N' as u8 as i64,
                                            'O' as u8 as i64,
                                            'T' as u8 as i64], x, y),
        };

        res.push(' ' as u8 as i64);
        res.push(x.encode());
        res.push(' ' as u8 as i64);
        res.push(y.encode());
        res.push('\n' as u8 as i64);

        res
    }
}

#[allow(dead_code)]
enum Register {
    Temp,
    OneAway,
    TwoAway,
    ThreeAway,
    FourAway,
    FiveAway,
    SixAway,
    SevenAway,
    EightAway,
    NineAway,
    Jump,
}

impl Register {
    fn encode(&self) -> i64 {
        match self {
            Register::Temp => 'T' as u8 as i64,
            Register::OneAway => 'A' as u8 as i64,
            Register::TwoAway => 'B' as u8 as i64,
            Register::ThreeAway => 'C' as u8 as i64,
            Register::FourAway => 'D' as u8 as i64,
            Register::FiveAway => 'E' as u8 as i64,
            Register::SixAway => 'F' as u8 as i64,
            Register::SevenAway => 'G' as u8 as i64,
            Register::EightAway => 'H' as u8 as i64,
            Register::NineAway => 'I' as u8 as i64,
            Register::Jump => 'J' as u8 as i64,
        }
    }
}