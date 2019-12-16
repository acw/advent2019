use std::fs;
use std::iter::FromIterator;
use std::str;
use std::sync::mpsc::{Sender, Receiver};

const ADD: i64       = 1;
const MULTIPLY: i64  = 2;
const INPUT: i64     = 3;
const OUTPUT: i64    = 4;
const HALT: i64      = 99;

const MODE_POSITION:  i64 = 0;
const MODE_IMMEDIATE: i64 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Computer {
    memory: Vec<i64>,
    position: usize
}

impl Computer {
    pub fn load(path: &str, position: usize) -> Computer {
        let byte_buffer = fs::read(path).unwrap();
        let char_buffer = str::from_utf8(&byte_buffer).unwrap();
        Computer::from_string(&char_buffer, position)
    }

    fn from_string(char_buffer: &str, position: usize) -> Computer {
        let mut memory = vec![];
        let mut char_iter = char_buffer.chars().peekable();

        while char_iter.peek().is_some() {
            let next_iter = char_iter.by_ref().take_while(|x| *x != ',');
            let next_str = String::from_iter(next_iter);
            let next = i64::from_str_radix(&next_str.trim(), 10).unwrap();
            memory.push(next);
        }

        Computer{ memory, position }
    }

    pub fn show(&self) {
       for (idx, val) in self.memory.iter().enumerate() {
           println!("{:08}: {}", idx, val);
       }
       println!("POSITION: {}", self.position);
    }

    pub fn read(&self, idx: usize) -> i64 {
        self.memory[idx]
    }

    pub fn read_arg(&self, mode: i64, val: usize) -> i64 {
        match mode {
            MODE_POSITION  => self.read(self.read(val) as usize),
            MODE_IMMEDIATE => self.read(val),
            _ => panic!("Unknown argument mode: {}", mode)
        }
    }

    pub fn write(&mut self, idx: usize, val: i64) {
        self.memory[idx] = val;
    }

    fn step(&mut self, input: &Receiver<i64>, output: &Sender<i64>) {
        let next_instruction = self.read(self.position);
        let opcode = next_instruction % 100;
        let arg1mode = (next_instruction / 100) % 10;
        let arg2mode = (next_instruction / 1000) % 10;
        let _arg3mode = (next_instruction / 10000) % 10;

        match opcode {
            ADD => {
                let arg1 = self.read_arg(arg1mode, self.position + 1);
                let arg2 = self.read_arg(arg2mode, self.position + 2);
                let dest = self.read(self.position + 3) as usize;

                self.write(dest, arg1 + arg2);
                self.position += 4;
            }
            MULTIPLY => {
                let arg1 = self.read_arg(arg1mode, self.position + 1);
                let arg2 = self.read_arg(arg2mode, self.position + 2);
                let dest = self.read(self.position + 3) as usize;

                self.write(dest, arg1 * arg2);
                self.position += 4;
            }
            INPUT => {
                let dest = self.read(self.position + 1) as usize;
                let val = input.recv().expect("Failed to read input value from channel.");
                self.write(dest, val);
                self.position += 2;
            }
            OUTPUT => {
                let arg1 = self.read_arg(arg1mode, self.position + 1);
                output.send(arg1).expect("Send failed.");
                self.position += 2;
            }
            HALT => {}
            /* */
            unknown_pos =>
              panic!("Unknown instruction {}", unknown_pos)
        }
    }

    fn halted(&self) -> bool {
        self.read(self.position) == HALT
    }

    pub fn run(&mut self, input: &Receiver<i64>, output: &Sender<i64>) {
        while !self.halted() {
            self.step(input, output);
        }
    }
}

#[cfg(test)]
use std::sync::mpsc::channel;

#[test]
fn test_examples() {
    let (deadsend, deadrecv) = channel();

    let mut example1 = Computer{ memory: vec![1,0,0,0,99], position: 0 };
    let answer1  = Computer{ memory: vec![2,0,0,0,99], position: 4 };
    example1.step(&deadrecv, &deadsend);
    assert_eq!(example1, answer1);
    assert!(example1.halted());

    let mut example2 = Computer{ memory: vec![2,3,0,3,99], position: 0 };
    let answer2 = Computer{ memory: vec![2,3,0,6,99], position: 4 };
    example2.step(&deadrecv, &deadsend);
    assert_eq!(example2, answer2);
    assert!(example2.halted());

    let mut example3 = Computer{ memory: vec![2,4,4,5,99,0], position: 0 };
    let answer3 = Computer{ memory: vec![2,4,4,5,99,9801], position: 4 };
    example3.step(&deadrecv, &deadsend);
    assert_eq!(example3, answer3);
    assert!(example3.halted());

    let mut example4 = Computer{ memory: vec![1,1,1,4,99,5,6,0,99], position: 0 };
    let answer4 = Computer{ memory: vec![30,1,1,4,2,5,6,0,99], position: 8 };
    example4.run(&deadrecv, &deadsend);
    assert_eq!(example4, answer4);
    assert!(example4.halted());

    let mut example5 = Computer{ memory: vec![1002,4,3,4,33], position: 0 };
    let answer5 = Computer{ memory: vec![1002,4,3,4,99], position: 4 };
    example5.run(&deadrecv, &deadsend);
    assert_eq!(example5, answer5);

    let mut example6 = Computer::from_string("1101,100,-1,4,0", 0);
    let answer6 = Computer{ memory: vec![1101,100,-1,4,99], position: 4 };
    example6.run(&deadrecv, &deadsend);
    assert_eq!(example6, answer6);
}