use std::fs;
use std::str;

const ADD: u64       = 1;
const MULTIPLY: u64  = 2;
const HALT: u64      = 99;

#[derive(Clone, Debug, PartialEq)]
pub struct Computer {
    memory: Vec<u64>,
    position: usize
}

impl Computer {
    pub fn load(path: &str, position: usize) -> Computer {
        let mut memory = vec![];
        let byte_buffer = fs::read(path).unwrap();
        let char_buffer = str::from_utf8(&byte_buffer).unwrap();

        let mut current = 0;
        for c in char_buffer.chars() {
            match c {
                ',' => {
                    memory.push(current);
                    current = 0;
                }
                _ if c.is_digit(10) => {
                    let val = c.to_digit(10).unwrap() as u64;
                    current = (current * 10) + val;
                }
                _ if c.is_whitespace() => {
                }
                _ => {
                    panic!("Unrecognized character: '{}'", c);
                }
            }
        }
        memory.push(current);

        Computer{ memory, position }
    }

    pub fn show(&self) {
       for (idx, val) in self.memory.iter().enumerate() {
           println!("{:08}: {}", idx, val);
       }
       println!("POSITION: {}", self.position);
    }

    pub fn read(&self, idx: usize) -> u64 {
        self.memory[idx]
    }

    pub fn write(&mut self, idx: usize, val: u64) {
        self.memory[idx] = val;
    }

    fn step(&mut self) {
        match self.read(self.position) {
            ADD => {
                let arg1 = self.read(self.position + 1) as usize;
                let arg2 = self.read(self.position + 2) as usize;
                let dest = self.read(self.position + 3) as usize;

                self.write(dest, self.read(arg1) + self.read(arg2));
                self.position += 4;
            }
            MULTIPLY => {
                let arg1 = self.read(self.position + 1) as usize;
                let arg2 = self.read(self.position + 2) as usize;
                let dest = self.read(self.position + 3) as usize;

                self.write(dest, self.read(arg1) * self.read(arg2));
                self.position += 4;
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

    pub fn run(&mut self) {
        while !self.halted() {
            self.step();
        }
    }
}

#[test]
fn test_examples() {
    let mut example1 = Computer{ memory: vec![1,0,0,0,99], position: 0 };
    let answer1  = Computer{ memory: vec![2,0,0,0,99], position: 4 };
    example1.step();
    assert_eq!(example1, answer1);
    assert!(example1.halted());

    let mut example2 = Computer{ memory: vec![2,3,0,3,99], position: 0 };
    let answer2 = Computer{ memory: vec![2,3,0,6,99], position: 4 };
    example2.step();
    assert_eq!(example2, answer2);
    assert!(example2.halted());

    let mut example3 = Computer{ memory: vec![2,4,4,5,99,0], position: 0 };
    let answer3 = Computer{ memory: vec![2,4,4,5,99,9801], position: 4 };
    example3.step();
    assert_eq!(example3, answer3);
    assert!(example3.halted());

    let mut example4 = Computer{ memory: vec![1,1,1,4,99,5,6,0,99], position: 0 };
    let answer4 = Computer{ memory: vec![30,1,1,4,2,5,6,0,99], position: 8 };
    example4.run();
    assert_eq!(example4, answer4);
    assert!(example4.halted());
}