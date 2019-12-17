use crate::endchannel::{Sender, Receiver};
use std::fs;
use std::iter::FromIterator;
use std::str;

const ADD: i64       = 1;
const MULTIPLY: i64  = 2;
const INPUT: i64     = 3;
const OUTPUT: i64    = 4;
const JMPIF: i64     = 5;
const JMPNIF: i64    = 6;
const LESS_THAN: i64 = 7;
const EQUALS: i64    = 8;
const HALT: i64      = 99;

const MODE_POSITION:  i64 = 0;
const MODE_IMMEDIATE: i64 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Computer {
    memory: Vec<i64>,
    position: usize,
    done: bool
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

        Computer{ memory, position, done: false }
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

    fn step(&mut self, input: &mut Receiver<i64>, output: &mut Sender<i64>) {
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
                let val = input.recv().expect("Failed to get input!");
                self.write(dest, val);
                self.position += 2;
            }
            OUTPUT => {
                let arg1 = self.read_arg(arg1mode, self.position + 1);
                output.send(arg1);
                self.position += 2;
            }
            JMPIF => {
                let arg1 = self.read_arg(arg1mode, self.position + 1);
                let arg2 = self.read_arg(arg2mode, self.position + 2);
                if arg1 != 0 {
                    self.position = arg2 as usize;
                } else {
                    self.position += 3;
                }
            }
            JMPNIF => {
                let arg1 = self.read_arg(arg1mode, self.position + 1);
                let arg2 = self.read_arg(arg2mode, self.position + 2);
                if arg1 == 0 {
                    self.position = arg2 as usize;
                } else {
                    self.position += 3;
                }
            }
            LESS_THAN => {
                let arg1 = self.read_arg(arg1mode, self.position + 1);
                let arg2 = self.read_arg(arg2mode, self.position + 2);
                let dest = self.read(self.position + 3) as usize;

                self.write(dest, if arg1 < arg2 { 1 } else { 0 });
                self.position += 4;
            }
            EQUALS => {
                let arg1 = self.read_arg(arg1mode, self.position + 1);
                let arg2 = self.read_arg(arg2mode, self.position + 2);
                let dest = self.read(self.position + 3) as usize;

                self.write(dest, if arg1 == arg2 { 1 } else { 0 });
                self.position += 4;
            }
            HALT => {
                output.conclude();
                self.done = true;
            }
            /* */
            unknown_pos =>
              panic!("Unknown instruction {}", unknown_pos)
        }
    }

    fn halted(&self) -> bool {
        self.done
    }

    pub fn run(&mut self, input: &mut Receiver<i64>, output: &mut Sender<i64>) {
        while !self.halted() {
            self.step(input, output);
        }
    }
}

#[cfg(test)]
use crate::endchannel::channel;

#[cfg(test)]
fn run_example(computer: Vec<i64>, inputs: Vec<i64>, targets: Vec<i64>) {
    let mut day5a = Computer{ memory: computer, position: 0, done: false };
    let (    mysend, mut corecv) = channel();
    let (mut cosend,     myrecv) = channel();
    for i in inputs.iter() {
        mysend.send(*i);
    }
    day5a.run(&mut corecv, &mut cosend);
    let outputs: Vec<i64> = myrecv.collect();
    assert_eq!(targets, outputs);
}

#[test]
fn test_examples() {
    let (mut deadsend, mut deadrecv) = channel();

    let mut example1 = Computer{ memory: vec![1,0,0,0,99], position: 0, done: false };
    let answer1  = Computer{ memory: vec![2,0,0,0,99], position: 4, done: false };
    example1.step(&mut deadrecv, &mut deadsend);
    assert_eq!(example1, answer1);

    let mut example2 = Computer{ memory: vec![2,3,0,3,99], position: 0, done: false };
    let answer2 = Computer{ memory: vec![2,3,0,6,99], position: 4, done: false };
    example2.step(&mut deadrecv, &mut deadsend);
    assert_eq!(example2, answer2);

    let mut example3 = Computer{ memory: vec![2,4,4,5,99,0], position: 0, done: false };
    let answer3 = Computer{ memory: vec![2,4,4,5,99,9801], position: 4, done: false };
    example3.step(&mut deadrecv, &mut deadsend);
    assert_eq!(example3, answer3);

    let mut example4 = Computer{ memory: vec![1,1,1,4,99,5,6,0,99], position: 0, done: false };
    let answer4 = Computer{ memory: vec![30,1,1,4,2,5,6,0,99], position: 8, done: true };
    example4.run(&mut deadrecv, &mut deadsend);
    assert_eq!(example4, answer4);
    assert!(example4.halted());

    let mut example5 = Computer{ memory: vec![1002,4,3,4,33], position: 0, done: false };
    let answer5 = Computer{ memory: vec![1002,4,3,4,99], position: 4, done: true };
    example5.run(&mut deadrecv, &mut deadsend);
    assert_eq!(example5, answer5);
    assert!(example5.halted());

    let mut example6 = Computer::from_string("1101,100,-1,4,0", 0);
    let answer6 = Computer{ memory: vec![1101,100,-1,4,99], position: 4, done: true };
    example6.run(&mut deadrecv, &mut deadsend);
    assert_eq!(example6, answer6);
    assert!(example6.halted());

    let mut day5a = Computer::load("inputs/day5", 0);
    let target = vec![0,0,0,0,0,0,0,0,0,7_259_358];
    let (    mysend, mut corecv) = channel();
    let (mut cosend,     myrecv) = channel();
    mysend.send(1);
    day5a.run(&mut corecv, &mut cosend);
    let outputs: Vec<i64> = myrecv.collect();
    assert_eq!(target, outputs);

    run_example(vec![3,9,8,9,10,9,4,9,99,-1,8], vec![8], vec![1]);
    run_example(vec![3,9,8,9,10,9,4,9,99,-1,8], vec![9], vec![0]);
    run_example(vec![3,9,7,9,10,9,4,9,99,-1,8], vec![4], vec![1]);
    run_example(vec![3,9,7,9,10,9,4,9,99,-1,8], vec![8], vec![0]);
    run_example(vec![3,9,7,9,10,9,4,9,99,-1,8], vec![9], vec![0]);
    run_example(vec![3,3,1108,-1,8,3,4,3,99],   vec![8], vec![1]);
    run_example(vec![3,3,1108,-1,8,3,4,3,99],   vec![9], vec![0]);
    run_example(vec![3,3,1107,-1,8,3,4,3,99],   vec![4], vec![1]);
    run_example(vec![3,3,1107,-1,8,3,4,3,99],   vec![8], vec![0]);
    run_example(vec![3,3,1107,-1,8,3,4,3,99],   vec![9], vec![0]);
    run_example(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                     1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                     999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99],
                vec![3], vec![999]);
    run_example(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                     1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                     999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99],
                vec![8], vec![1000]);
    run_example(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                     1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                     999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99],
                vec![192], vec![1001]);
}
