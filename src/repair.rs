use crate::endchannel::{Receiver, Sender, channel};
use crate::machine::Computer;
use std::thread;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

const ALL_DIRECTIONS: [Direction; 4] = [Direction::North,
                                        Direction::South,
                                        Direction::East,
                                        Direction::West];

impl Direction {
    fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East  => Direction::West,
            Direction::South => Direction::North,
            Direction::West  => Direction::East,
        }
    }

    fn encode(&self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::East  => 4,
            Direction::South => 2,
            Direction::West  => 3,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Path {
    steps: Vec<Direction>
}

impl Path {
    fn new() -> Path {
        Path{ steps: vec![] }
    }

    fn reverse(&self) -> Path {
        let mut steps = Vec::with_capacity(self.steps.len());

        for step in self.steps.iter().rev() {
            steps.push(step.reverse());
        }

        Path{ steps }
    }

    fn extend(&self) -> Vec<Path> {
        let mut res = Vec::with_capacity(4);

        for dir in ALL_DIRECTIONS.iter() {
            let mut copy = self.steps.clone();
            copy.push(*dir);
            let potential = Path{ steps: copy };
            if !potential.loops() {
                res.push(potential);
            }
        }

        res
    }

    fn loops(&self) -> bool {
        let mut previous = vec![];
        let mut x: i64 = 0;
        let mut y: i64 = 0;

        for step in self.steps.iter() {
            match step {
                Direction::North => y -= 1,
                Direction::South => y += 1,
                Direction::East  => x += 1,
                Direction::West  => x -= 1,
            }

            if previous.contains(&(x, y)) {
                return true;
            }

            previous.push((x, y));
        }

        false
    }
}

struct RepairSearch {
    computer: Computer,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MoveResult {
    HitWall,
    Done,
    FoundSystem,
}

impl MoveResult {
    fn new(x: i64) -> MoveResult {
        match x {
            0 => MoveResult::HitWall,
            1 => MoveResult::Done,
            2 => MoveResult::FoundSystem,
            _ => panic!("Unknown move result!!"),
        }
    }
}

impl RepairSearch {
    fn new(f: &str) -> RepairSearch {
        let computer = Computer::load(f, 0);
        RepairSearch { computer }
    }

    fn try_path(&mut self, path: &Path) -> MoveResult {
        let (mut mysend, mut corecv) = channel();
        let (mut cosend, mut myrecv) = channel();
        let my_computer = self.computer.clone();
        let mut last_response = MoveResult::Done;

        thread::spawn(move || my_computer.clone().run(&mut corecv, &mut cosend));
        for step in path.steps.iter() {
            mysend.send(step.encode());
            match myrecv.recv() {
                None =>
                    return last_response,
                Some(response) => {
                    last_response = MoveResult::new(response);
                    if last_response == MoveResult::HitWall {
                        break
                    }
                }
            }
        }
        mysend.conclude();

        last_response
    }

    fn run_search(&mut self) -> usize {
        let mut horizon = Path::new().extend();

        loop {
            let mut new_horizon = vec![];

            assert_ne!(horizon.len(), 0);
            println!("{} items at length {}", horizon.len(), horizon[0].steps.len());
            for path in horizon.iter() {
                let result = self.try_path(path);

                match result {
                    MoveResult::HitWall => continue,
                    MoveResult::Done    => new_horizon.append(&mut path.extend()),
                    MoveResult::FoundSystem => return path.steps.len(),
                }
            }

            horizon = new_horizon;
        }
    }
}

#[test]
fn day15() {
    let mut day15a = RepairSearch::new("inputs/day15");
    assert_eq!(298, day15a.run_search());
}
