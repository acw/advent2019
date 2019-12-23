use crate::endchannel::{Receiver, Sender, channel};
use crate::machine::Computer;
use std::collections::VecDeque;
use std::thread;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn apply(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::North => (x, y - 1),
            Direction::East  => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West  => (x - 1, y),
        }
    }

    fn random() -> Direction {
        match rand::random::<u8>() & 0x3 {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!("The world broke")
        }
    }
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
        let computer = Computer::load(f);
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Unknown,
    Empty,
    Wall,
    Oxygen,
}

struct Room {
    layout: Vec<Tile>,
    computer_input: Sender<i64>,
    computer_output: Receiver<i64>,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
}

impl Room {
    fn new(width: usize, height: usize, f: &str) -> Room {
        let (mut mysend, mut corecv) = channel();
        let (mut cosend, mut myrecv) = channel();
        let mut my_computer = Computer::load(f);
        let mut layout = Vec::with_capacity(width * height);

        layout.resize(width * height, Tile::Unknown);
        thread::spawn(move || my_computer.run(&mut corecv, &mut cosend));
        Room{
            layout,
            computer_input: mysend,
            computer_output: myrecv,
            width, height,
            x: width / 2,
            y: height / 2,
        }
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        self.layout[ (y * self.width) + x ]
    }

    fn set(&mut self, x: usize, y: usize, t: Tile) {
        self.layout[ (y * self.width) + x ] = t;
    }

    fn valid_nexts(&self, x: usize, y: usize) -> Vec<(Direction, usize, usize)>
    {
        let mut res = vec![];

        if y > 0                 { res.push((Direction::North, x, y - 1)); }
        if x > 0                 { res.push((Direction::West,  x - 1, y)); }
        if y < (self.height - 1) { res.push((Direction::South, x, y + 1)); }        
        if x < (self.width - 1)  { res.push((Direction::East,  x + 1, y)); }

        res
    }

    fn print(&self) {
        println!("\n\n\n\n\n\n");
        for y in 0..self.height {
            for x in 0..self.width {
                if self.x == x && self.y == y {
                    print!("R");
                    continue;
                }
                match self.get(x, y) {
                    Tile::Unknown => print!("?"),
                    Tile::Empty   => print!("."),
                    Tile::Oxygen  => print!("O"),
                    Tile::Wall    => print!("X"),
                }
            }
            println!();
        }
    }

    fn next_unknown(&self) -> Option<Direction> {
        let mut visited = vec![];
        let mut queue = VecDeque::new();

        // self.print();
        queue.extend(self.valid_nexts(self.x, self.y).iter());
        while let Some((dir, x, y)) = queue.pop_front() {
            // println!("Visiting ({:?}, {}, {})", dir, x, y);
            match self.get(x, y) {
                Tile::Unknown => return Some(dir),
                Tile::Wall    => continue,
                _             => {
                    for (_, newx, newy) in self.valid_nexts(x, y) {
                        if !visited.contains(&(newx, newy)) {
                            queue.push_back((dir, newx, newy));
                        }
                    }
                    visited.push((x, y));
                }
            }
        }

        None
    }

    fn step(&mut self, direction: Direction) -> bool {
        let (tx, ty) = direction.apply(self.x, self.y);

        if self.get(tx, ty) == Tile::Wall {
            return false;
        }

        self.computer_input.send(direction.encode());
        match self.computer_output.recv() {
            None => false,
            Some(resp) => {
                let response = MoveResult::new(resp);

                match response {
                    MoveResult::HitWall => {
                        self.set(tx, ty, Tile::Wall);
                        false
                    }
                    MoveResult::Done => {
                        self.set(tx, ty, Tile::Empty);
                        self.x = tx;
                        self.y = ty;
                        true
                    }
                    MoveResult::FoundSystem => {
                        self.set(tx, ty, Tile::Oxygen);
                        self.x = tx;
                        self.y = ty;
                        true
                    }
                }
            }
        }
    }

    fn map_room(&mut self) -> usize {
        let mut steps = 0;

        while let Some(next_step) = self.next_unknown() {
            //println!("Steps taken: {} [x {}, y {}, next {:?}]", steps, self.x, self.y, next_step);
            self.step(next_step);
            steps += 1;
        }

        steps
    }

    fn has_empty_space(&self) -> bool {
        for tile in self.layout.iter() {
            if tile == &Tile::Empty {
                return true;
            }
        }
        false
    }

    fn spread(&mut self) -> usize {
        let mut steps = 0;

        while self.has_empty_space() {
            let snapshot = self.layout.clone();

            for x in 0..self.width {
                for y in 0..self.height {
                    if snapshot[ (y * self.width) + x ] == Tile::Oxygen {
                        for (_, nx, ny) in self.valid_nexts(x, y).iter() {
                            if snapshot[ (*ny * self.width) + *nx ] == Tile::Empty {
                                self.set(*nx, *ny, Tile::Oxygen);
                            }
                        }
                    }
                }
            }
            steps += 1; 
            //self.print();
        }

        steps
    } 
}

#[test]
fn day15a() {
    let mut day15a = RepairSearch::new("inputs/day15");
    assert_eq!(298, day15a.run_search());
}

#[test]
fn day15b() {
    let mut day15b = Room::new(50, 50, "inputs/day15");
    assert!(day15b.next_unknown().is_some());
    assert_eq!(2452, day15b.map_room());
    assert_eq!(346, day15b.spread());
}
