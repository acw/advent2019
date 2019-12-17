use crate::endchannel::{Sender, Receiver, channel};
use std::fs;
use std::iter::FromIterator;
use std::ops::Range;
use std::str;
use std::thread;

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

    pub fn serialize(&self, inputs: Vec<i64>) -> i64 {
        let mut previous_output = vec![0];

        for input in inputs.iter() {
            let mut my_machine = self.clone();
            let (    mysend, mut corecv) = channel();
            let (mut cosend,     myrecv) = channel();
            mysend.send(*input);
            for i in previous_output.iter() {
                mysend.send(*i);
            }
            my_machine.run(&mut corecv, &mut cosend);
            previous_output = myrecv.collect();
        }

        assert_eq!(previous_output.len(), 1);
        previous_output[0]
    }

    pub fn find_best_signal<F>(&self, range: Range<i64>, f: F) -> (i64, Vec<i64>)
      where F: Fn(Vec<i64>) -> i64
    {
        let mut best_score = 0;
        let mut best_result = vec![];

        for a in range.clone() {
            for b in range.clone() {
                if b == a { continue; }
                for c in range.clone() {
                    if c == a || c == b { continue; }
                    for d in range.clone() {
                        if d == c || d == b || d == a { continue; }
                        for e in range.clone() {
                            if e == d || e == c || e == b || e == a { continue; }
                            let inputs = vec![a,b,c,d,e];
                            let result = f(inputs);
                            if result > best_score {
                                best_score = result;
                                best_result = vec![a,b,c,d,e];
                            }
                        }
                    }
                }
            }
        }

        (best_score, best_result)
    }

    pub fn amplifier(&self, settings: Vec<i64>) -> i64 {
        assert_eq!(settings.len(), 5);

        let mut machine_a = self.clone();
        let mut machine_b = self.clone();
        let mut machine_c = self.clone();
        let mut machine_d = self.clone();
        let mut machine_e = self.clone();

        let (    send_ha, mut recv_ha) = channel(); send_ha.send(settings[0]);
        let (mut send_ab, mut recv_ab) = channel(); send_ab.send(settings[1]);
        let (mut send_bc, mut recv_bc) = channel(); send_bc.send(settings[2]);
        let (mut send_cd, mut recv_cd) = channel(); send_cd.send(settings[3]);
        let (mut send_de, mut recv_de) = channel(); send_de.send(settings[4]);
        let (mut send_eh,     recv_eh) = channel();

        thread::spawn(move || { machine_a.run(&mut recv_ha, &mut send_ab) });
        thread::spawn(move || { machine_b.run(&mut recv_ab, &mut send_bc) });
        thread::spawn(move || { machine_c.run(&mut recv_bc, &mut send_cd) });
        thread::spawn(move || { machine_d.run(&mut recv_cd, &mut send_de) });
        thread::spawn(move || { machine_e.run(&mut recv_de, &mut send_eh) });

        send_ha.send(0); // kick it off

        let mut last_output = 0;

        for output in recv_eh {
            last_output = output;
            send_ha.send_ignore_error(output);
        }

        last_output
    }
}

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

    let example7a = Computer::from_string("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", 0);
    let result7a = example7a.serialize(vec![4,3,2,1,0]);
    assert_eq!(43210, result7a);
    let example7b = Computer::from_string("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0", 0);
    let result7b = example7b.serialize(vec![0,1,2,3,4]);
    assert_eq!(54321, result7b);
    let example7c = Computer::from_string("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", 0);
    let target7c = 65210;
    let result7c = example7c.serialize(vec![1,0,4,3,2]);
    assert_eq!(target7c, result7c);
    let result7c2 = example7c.serialize(vec![1,0,4,3,2]);
    assert_eq!(target7c, result7c2);
    assert_eq!(result7c2, 65210);
    assert_eq!(example7c.find_best_signal(0..5, |x| example7c.serialize(x)).1, vec![1,0,4,3,2]);
    let day7a = Computer::load("inputs/day7", 0);
    let (day7score, day7settings) = day7a.find_best_signal(0..5, |x| day7a.serialize(x));
    assert_eq!(day7score, 206580);
    assert_eq!(day7settings, vec![2,0,1,4,3]);

    let example7e = Computer::from_string("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5", 0);
    assert_eq!(139629729, example7e.amplifier(vec![9,8,7,6,5]));
    let (example7es, example7et) = example7e.find_best_signal(5..10, |x| example7e.amplifier(x));
    assert_eq!(139629729, example7es);
    assert_eq!(vec![9,8,7,6,5], example7et);
    let example7f = Computer::from_string("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10", 0);
    assert_eq!(18216, example7f.amplifier(vec![9,7,8,5,6]));
    let (example7fs, example7ft) = example7f.find_best_signal(5..10, |x| example7f.amplifier(x));
    assert_eq!(18216, example7fs);
    assert_eq!(vec![9,7,8,5,6], example7ft);
}
