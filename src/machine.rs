use std::collections::VecDeque;
use std::fs;
use std::iter::FromIterator;
use std::ops::Range;
use std::str;

const ADD: i64         = 1;
const MULTIPLY: i64    = 2;
const INPUT: i64       = 3;
const OUTPUT: i64      = 4;
const JMPIF: i64       = 5;
const JMPNIF: i64      = 6;
const LESS_THAN: i64   = 7;
const EQUALS: i64      = 8;
const ADJUST_BASE: i64 = 9;
const HALT: i64        = 99;

#[derive(Debug,PartialEq)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

impl From<i64> for Mode {
    fn from(x: i64) -> Mode {
        match x {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("Unknown mode value: {}", x),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Computer {
    memory: Vec<i64>,
    position: usize,
    relative_base: i64,
    done: bool
}

pub enum RunResult {
    Input(Box<dyn FnOnce(i64) -> Computer>),
    Output(i64, Computer),
    Continue(Computer),
    Halted(Computer),
}

impl Computer {
    pub fn load(path: &str) -> Computer {
        let byte_buffer = fs::read(path).unwrap();
        let char_buffer = str::from_utf8(&byte_buffer).unwrap();
        Computer::from_string(&char_buffer)
    }

    fn from_string(char_buffer: &str) -> Computer {
        let mut memory = vec![];
        let mut char_iter = char_buffer.chars().peekable();

        while char_iter.peek().is_some() {
            let next_iter = char_iter.by_ref().take_while(|x| *x != ',');
            let next_str = String::from_iter(next_iter);
            let next = i64::from_str_radix(&next_str.trim(), 10).unwrap();
            memory.push(next);
        }

        Computer{ memory, position: 0, relative_base: 0, done: false }
    }

    pub fn show(&self) {
       for (idx, val) in self.memory.iter().enumerate() {
           println!("{:08}: {}", idx, val);
       }
       println!("POSITION: {}", self.position);
    }

    pub fn read(&mut self, idx: usize) -> i64 {
        if idx >= self.memory.len() {
            self.memory.resize(idx + 1, 0);
        }
        self.memory[idx]
    }

    pub fn read_arg(&mut self, mode: Mode, val: usize) -> i64 {
        match mode {
            Mode::Position => {
                let ptr = self.read(val) as usize;
                self.read(ptr)
            }
            Mode::Immediate => self.read(val),
            Mode::Relative  => {
                let ptr = self.read(val) + self.relative_base;
                self.read(ptr as usize)
            }
        }
    }

    fn read_dest(&mut self, mode: Mode, val: usize) -> usize {
        assert_ne!(mode, Mode::Immediate);
        let mut base = self.read(val);
        if mode == Mode::Relative {
            base += self.relative_base;
        }
        assert!(base >= 0);
        base as usize
    }

    pub fn write(&mut self, idx: usize, val: i64) {
        if idx >= self.memory.len() {
            self.memory.resize(idx + 1, 0);
        }
        self.memory[idx] = val;
    }

    fn step(mut self) -> RunResult {
        let next_instruction = self.read(self.position);
        let opcode = next_instruction % 100;
        let arg1mode = Mode::from((next_instruction / 100) % 10);
        let arg2mode = Mode::from((next_instruction / 1000) % 10);
        let arg3mode = Mode::from((next_instruction / 10000) % 10);

        match opcode {
            ADD => {
                let arg1 = self.read_arg(arg1mode, self.position + 1);
                let arg2 = self.read_arg(arg2mode, self.position + 2);
                let dest = self.read_dest(arg3mode, self.position + 3) as usize;

                self.write(dest, arg1 + arg2);
                self.position += 4;
                RunResult::Continue(self)
            }
            MULTIPLY => {
                let arg1 = self.read_arg(arg1mode, self.position + 1);
                let arg2 = self.read_arg(arg2mode, self.position + 2);
                let dest = self.read_dest(arg3mode, self.position + 3) as usize;

                self.write(dest, arg1 * arg2);
                self.position += 4;
                RunResult::Continue(self)
            }
            INPUT => {
                let dest = self.read_dest(arg1mode, self.position + 1) as usize;
                self.position += 2;
                RunResult::Input(Box::new(move |x| {
                    self.write(dest, x);
                    self
                }))
            }
            OUTPUT => {
                let arg1 = self.read_arg(arg1mode, self.position + 1);
                self.position += 2;
                RunResult::Output(arg1, self)
            }
            JMPIF => {
                let arg1 = self.read_arg(arg1mode, self.position + 1);
                let arg2 = self.read_arg(arg2mode, self.position + 2);

                if arg1 != 0 {
                    self.position = arg2 as usize;
                } else {
                    self.position += 3;
                }
                RunResult::Continue(self)
            }
            JMPNIF => {
                let arg1 = self.read_arg(arg1mode, self.position + 1);
                let arg2 = self.read_arg(arg2mode, self.position + 2);

                if arg1 == 0 {
                    self.position = arg2 as usize;
                } else {
                    self.position += 3;
                }
                RunResult::Continue(self)
            }
            LESS_THAN => {
                let arg1 = self.read_arg(arg1mode, self.position + 1);
                let arg2 = self.read_arg(arg2mode, self.position + 2);
                let dest = self.read_dest(arg3mode, self.position + 3) as usize;

                self.write(dest, if arg1 < arg2 { 1 } else { 0 });
                self.position += 4;
                RunResult::Continue(self)
            }
            EQUALS => {
                let arg1 = self.read_arg(arg1mode, self.position + 1);
                let arg2 = self.read_arg(arg2mode, self.position + 2);
                let dest = self.read_dest(arg3mode,self.position + 3) as usize;

                self.write(dest, if arg1 == arg2 { 1 } else { 0 });
                self.position += 4;
                RunResult::Continue(self)
            }
            ADJUST_BASE => {
                let arg1 = self.read_arg(arg1mode, self.position + 1);

                self.relative_base += arg1;
                self.position += 2;
                RunResult::Continue(self)
            }
            HALT => {
                self.done = true;
                RunResult::Halted(self)
            }
            /* */
            unknown_pos =>
              panic!("Unknown instruction {}", unknown_pos)
        }
    }

    pub fn run(mut self) -> RunResult {
        loop {
            match self.step() {
                RunResult::Continue(next) =>
                    self = next,
                result =>
                    return result,
            }
        }
    }

    pub fn standard_run(mut self, inputs: &[i64]) -> Vec<i64> {
        let mut idx = 0;
        let mut res = vec![];

        loop {
            match self.step() {
                RunResult::Continue(next) =>
                    self = next,
                RunResult::Halted(_) =>
                    return res,
                RunResult::Input(c) if idx < inputs.len() => {
                    self = c(inputs[idx]);
                    idx += 1;
                }
                RunResult::Input(_) =>
                    panic!("Ran out of inputs in standard run."),
                RunResult::Output(x, next) => {
                    res.push(x);
                    self = next;
                }
            }
        }
    }

    pub fn serialize(&self, inputs: Vec<i64>) -> i64 {
        let mut previous_output = VecDeque::new();

        previous_output.push_back(0);
        for input in inputs.iter() {
            let mut my_machine = self.clone().prime(*input);
            let mut output = VecDeque::new();

            loop {
                match my_machine.run() {
                    RunResult::Continue(next) =>
                        my_machine = next,
                    RunResult::Halted(_) =>
                        break,
                    RunResult::Input(c) =>
                        match previous_output.pop_front() {
                            None =>
                                panic!("Serialized machine wanted input I didn't have!"),
                            Some(next_input) =>
                                my_machine = c(next_input),
                        },
                    RunResult::Output(o, next) => {
                        output.push_back(o);
                        my_machine = next;
                    }
                }
            }
            previous_output = output;
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

    pub fn prime(self, input: i64) -> Self {
        match self.run() {
            RunResult::Input(c) => c(input),
            _                   =>
                panic!("Priming failure: machine didn't ask for input first.")
        }
    }

    pub fn amplifier(&self, settings: Vec<i64>) -> i64 {
        assert_eq!(settings.len(), 5);

        let mut machine_a = self.clone().prime(settings[0]);
        let mut machine_b = self.clone().prime(settings[1]);
        let mut machine_c = self.clone().prime(settings[2]);
        let mut machine_d = self.clone().prime(settings[3]);
        let mut machine_e = self.clone().prime(settings[4]);
        let mut last_output = 0;

        loop {
            let aout = loop { match machine_a.run() {
                RunResult::Halted(_) => return last_output,
                RunResult::Output(o, next) => { machine_a = next; break o; }
                RunResult::Input(c) => machine_a = c(last_output),
                _ => panic!("Unexpted aout"),
            } };
            let bout = loop { match machine_b.run() {
                RunResult::Halted(_) => return last_output,
                RunResult::Output(o, next) => { machine_b = next; break o; }
                RunResult::Input(c) => machine_b = c(aout),
                _ => panic!("Unexpted aout"),
            } };
            let cout = loop { match machine_c.run() {
                RunResult::Halted(_) => return last_output,
                RunResult::Output(o, next) => { machine_c = next; break o; }
                RunResult::Input(c) => machine_c = c(bout),
                _ => panic!("Unexpted aout"),
            } };
            let dout = loop { match machine_d.run() {
                RunResult::Halted(_) => return last_output,
                RunResult::Output(o, next) => { machine_d = next; break o; }
                RunResult::Input(c) => machine_d = c(cout),
                _ => panic!("Unexpted aout"),
            } };
            let eout = loop { match machine_e.run() {
                RunResult::Halted(_) => return last_output,
                RunResult::Output(o, next) => { machine_e = next; break o; }
                RunResult::Input(c) => machine_e = c(dout),
                _ => panic!("Unexpted aout"),
            } };
            last_output = eout;
        }
    }
}

#[cfg(test)]
fn run_example(computer: Vec<i64>, inputs: &[i64], targets: &[i64]) {
    let day5a = Computer{ memory: computer, position: 0, relative_base: 0, done: false };
    run_computer(day5a, inputs, targets);
}

#[cfg(test)]
fn run_computer(computer: Computer, inputs: &[i64], targets: &[i64]) {
    let outputs = computer.standard_run(inputs);
    assert_eq!(&outputs, &targets);
}

#[test]
fn test_examples() {
    let example1 = Computer::from_string("1,0,0,0,99");
    let answer1  = Computer{ memory: vec![2,0,0,0,99], position: 4, relative_base: 0, done: false };
    match example1.step() {
        RunResult::Continue(result) => assert_eq!(answer1, result),
        _                           => assert!(false),
    }

    let example2 = Computer::from_string("2,3,0,3,99");
    let answer2 = Computer{ memory: vec![2,3,0,6,99], position: 4, relative_base: 0, done: false };
    match example2.step() {
        RunResult::Continue(result) => assert_eq!(answer2, result),
        _                           => assert!(false),
    }

    let example3 = Computer::from_string("2,4,4,5,99,0");
    let answer3 = Computer{ memory: vec![2,4,4,5,99,9801], position: 4, relative_base: 0, done: false };
    match example3.step() {
        RunResult::Continue(result) => assert_eq!(answer3, result),
        _                           => assert!(false),
    }

    let example4 = Computer::from_string("1,1,1,4,99,5,6,0,99");
    let answer4 = Computer{ memory: vec![30,1,1,4,2,5,6,0,99], position: 8, relative_base: 0, done: true };
    match example4.run() {
        RunResult::Halted(result) => assert_eq!(answer4, result),
        _                         => assert!(false),
    }

    let example5 = Computer::from_string("1002,4,3,4,33");
    let answer5 = Computer{ memory: vec![1002,4,3,4,99], position: 4, relative_base: 0, done: true };
    match example5.run() {
        RunResult::Halted(result) => assert_eq!(answer5, result),
        _                         => assert!(false),
    }

    let example6 = Computer::from_string("1101,100,-1,4,0");
    let answer6 = Computer{ memory: vec![1101,100,-1,4,99], position: 4, relative_base: 0, done: true };
    match example6.run() {
        RunResult::Halted(result) => assert_eq!(answer6, result),
        _                         => assert!(false),
    }

    let day5a = Computer::load("inputs/day5");
    let target = vec![0,0,0,0,0,0,0,0,0,7_259_358];
    let outputs = day5a.standard_run(&[1]);
    assert_eq!(target, outputs);

    run_example(vec![3,9,8,9,10,9,4,9,99,-1,8], &[8], &[1]);
    run_example(vec![3,9,8,9,10,9,4,9,99,-1,8], &[9], &[0]);
    run_example(vec![3,9,7,9,10,9,4,9,99,-1,8], &[4], &[1]);
    run_example(vec![3,9,7,9,10,9,4,9,99,-1,8], &[8], &[0]);
    run_example(vec![3,9,7,9,10,9,4,9,99,-1,8], &[9], &[0]);
    run_example(vec![3,3,1108,-1,8,3,4,3,99],   &[8], &[1]);
    run_example(vec![3,3,1108,-1,8,3,4,3,99],   &[9], &[0]);
    run_example(vec![3,3,1107,-1,8,3,4,3,99],   &[4], &[1]);
    run_example(vec![3,3,1107,-1,8,3,4,3,99],   &[8], &[0]);
    run_example(vec![3,3,1107,-1,8,3,4,3,99],   &[9], &[0]);
    run_example(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                     1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                     999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99],
                &[3], &[999]);
    run_example(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                     1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                     999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99],
                &[8], &[1000]);
    run_example(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                     1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                     999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99],
                &[192], &[1001]);

    let example7a = Computer::from_string("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    let result7a = example7a.serialize(vec![4,3,2,1,0]);
    assert_eq!(43210, result7a);
    let example7b = Computer::from_string("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
    let result7b = example7b.serialize(vec![0,1,2,3,4]);
    assert_eq!(54321, result7b);
    let example7c = Computer::from_string("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
    let target7c = 65210;
    let result7c = example7c.serialize(vec![1,0,4,3,2]);
    assert_eq!(target7c, result7c);
    let result7c2 = example7c.serialize(vec![1,0,4,3,2]);
    assert_eq!(target7c, result7c2);
    assert_eq!(result7c2, 65210);
    assert_eq!(example7c.find_best_signal(0..5, |x| example7c.serialize(x)).1, vec![1,0,4,3,2]);
    let day7a = Computer::load("inputs/day7");
    let (day7score, day7settings) = day7a.find_best_signal(0..5, |x| day7a.serialize(x));
    assert_eq!(day7score, 206580);
    assert_eq!(day7settings, vec![2,0,1,4,3]);

    let example7e = Computer::from_string("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
    assert_eq!(139629729, example7e.amplifier(vec![9,8,7,6,5]));
    let (example7es, example7et) = example7e.find_best_signal(5..10, |x| example7e.amplifier(x));
    assert_eq!(139629729, example7es);
    assert_eq!(vec![9,8,7,6,5], example7et);
    let example7f = Computer::from_string("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
    assert_eq!(18216, example7f.amplifier(vec![9,7,8,5,6]));
    let (example7fs, example7ft) = example7f.find_best_signal(5..10, |x| example7f.amplifier(x));
    assert_eq!(18216, example7fs);
    assert_eq!(vec![9,7,8,5,6], example7ft);

    run_example(vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99],
                &[],
                &[109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
    run_example(vec![1102,34915192,34915192,7,4,7,99,0],
                &[],
                &[1219070632396864]);
    run_example(vec![104,1125899906842624,99],
                &[],
                &[1125899906842624]);

    run_computer(Computer::load("inputs/day9"), &[1], &[3063082071]);
    run_computer(Computer::load("inputs/day9"), &[2], &[81348]);
}
