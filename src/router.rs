use crate::machine::{Computer, RunResult};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::ops::Range;

struct ComputerState {
    next: Box<dyn FnOnce(i64) -> Computer>,
    input_queue: VecDeque<i64>,
}

impl ComputerState {
    fn new(mut c: Computer, address: i64) -> ComputerState {
        let mut sent_address = false;

        loop {
            match c.run() {
                RunResult::Continue(next) =>
                    c = next,
                RunResult::Halted(_) =>
                    panic!("Computer halted right away!"),
                RunResult::Output(_, _) =>
                    panic!("Computer sent output right away!"),
                RunResult::Input(f) if sent_address =>
                    return ComputerState {
                        next: f,
                        input_queue: VecDeque::new(),
                    },
                RunResult::Input(f) => {
                    c = f(address);
                    sent_address = true;
                }
            }
        }
    }

    fn run(mut self, output: &mut VecDeque<i64>) -> Option<Self> {
        let mut c = match self.input_queue.pop_front() {
            None => (self.next)(-1),
            Some(x) => (self.next)(x),
        };

        loop {
            match c.run() {
                RunResult::Continue(next) =>
                    c = next,
                RunResult::Halted(_) =>
                    return None,
                RunResult::Output(o, next) => {
                    output.push_back(o);
                    c = next;
                } 
                RunResult::Input(f) =>
                    match self.input_queue.pop_front() {
                        None => {
                            self.next = f;
                            return Some(self);
                        }
                        Some(x) =>
                            c = f(x),
                    }
            }
        }
    }
}

struct Router {
    map: HashMap<usize, ComputerState>
}

impl Router {
    fn new(file: &str, nodes: Range<usize>) -> Router {
        let mut map = HashMap::new();
        let c = Computer::load(file);

        for i in nodes {
            map.insert(i, ComputerState::new(c.clone(), i as i64));
        }

        Router{ map }
    }

    fn step(mut self) -> Self {
        let mut outputs = VecDeque::new();
        let mut res = HashMap::new();

        for (key, val) in self.map.drain() {
            match val.run(&mut outputs) {
                None            => {},
                Some(new_state) => {
                    let _ = res.insert(key, new_state);
                }
            }
        }

        for (dest, x, y) in outputs.drain(0..).tuples() {
            match res.get_mut(&(dest as usize)) {
                None =>
                    panic!("Unknown destination {} (x {}, y {})", dest, x, y),
                Some(state) => {
                    state.input_queue.push_back(x);
                    state.input_queue.push_back(y);
                }
            }
        }

        Router{ map: res }
    }

    fn run(mut self) -> Self {
        while self.map.len() > 0 {
            self = self.step();
        }
        self
    }
}

#[test]
fn day23a() {
    let router = Router::new("inputs/day23", 0..50);
    router.run();
}