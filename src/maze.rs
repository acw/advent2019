use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::str;

struct Maze {
    data: Vec<Tile>,
    keyset: HashSet<char>,
    width: usize,
    height: usize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Door(char),
    Key(char),
    Entrance,
}

impl Tile {
    fn is_key(&self) -> bool {
        match self {
            Tile::Key(_) => true,
            _            => false,
        }
    }
}

impl Maze {
    fn new(s: &str) -> Maze {
        let mut data = Vec::new();
        let mut keyset = HashSet::new();
        let mut width = 0;
        let mut height = 0;

        for line in s.trim().split('\n') {
            for c in line.chars() {
                match c {
                    '#' => data.push(Tile::Wall),
                    '.' => data.push(Tile::Empty),
                    '@' => data.push(Tile::Entrance),
                    kd if kd.is_ascii_lowercase() => {
                        data.push(Tile::Key(kd));
                        keyset.insert(kd);
                    }
                    kd if kd.is_ascii_uppercase() => data.push(Tile::Door(kd.to_ascii_lowercase())),
                    kd => panic!("Unrecognized character: {}", kd),
                }
                if height == 0 { width += 1 }
            }
            height += 1;
        }

        Maze{ data, keyset, width, height }
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        self.data[ (y * self.width) + x ]
    }

    fn origin(&self) -> (usize, usize) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.get(x, y) == Tile::Entrance {
                    return (x, y);
                }
            }
        }
        panic!("No origin found?!")
    }

    fn next_steps(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut initial_res = Vec::new();

        if x > 0                 { initial_res.push((x - 1, y)); }
        if y > 0                 { initial_res.push((x, y - 1)); }
        if x < (self.width  - 1) { initial_res.push((x + 1, y)); }
        if y < (self.height - 1) { initial_res.push((x, y + 1)); }

        let mut res = Vec::new();

        for (x, y) in initial_res {
            if self.get(x, y) != Tile::Wall {
                res.push((x, y));
            }
        }

        res
    }
}

#[derive(Clone)]
struct SearchState {
    collected_keys: HashSet<char>,
    path: Vec<(usize, usize)>,
}

impl SearchState {
    fn new(maze: &Maze) -> SearchState {
        SearchState {
            collected_keys: HashSet::new(),
            path: vec![maze.origin()],
        }
    }

    fn should_prune(&self, best_cases: &mut HashMap<(usize,usize),Vec<HashSet<char>>>) -> bool {
//         let mut history = vec![];
// 
//         for (x, y) in self.path.iter().rev() {
//             if maze.get(*x, *y).is_key() {
//                 break;
//             }
// 
//             if history.contains(&(x, y)) {
//                 return true;
//             }
// 
//             history.push((x, y));
//         }

        assert_ne!(self.path.len(), 0);
        let lastpos = self.path[self.path.len() - 1];

        match best_cases.get_mut(&lastpos) {
            None => {
                let _ = best_cases.insert(lastpos, vec![self.collected_keys.clone()]);
            }
            Some(seen) => {
                for previous in seen.iter_mut() {
                    if self.collected_keys.is_subset(previous) {
                        return true;
                    }

                    if previous.is_subset(&self.collected_keys) {
                        *previous = self.collected_keys.clone();
                        break;
                    }
                }
                seen.push(self.collected_keys.clone());
            }
        }

        false
    }
}

fn find_keys(maze: &Maze) -> Vec<(usize, usize)> {
    let initial_state = SearchState::new(maze);
    let mut queue = VecDeque::new();
    let mut best_states = HashMap::new();

    queue.push_back(initial_state);
    while let Some(state) = queue.pop_front() {
        // println!("path length {} [queue length {}, have {:?}, want {:?}]", state.path.len(), queue.len(), state.collected_keys, maze.keyset);
        assert_ne!(state.path.len(), 0);
        let (x, y) = state.path[state.path.len() - 1];
        let mut new_items = Vec::new();

        for (newx, newy) in maze.next_steps(x, y).drain(0..) {
            match maze.get(newx, newy) {
                Tile::Wall    => continue,
                Tile::Empty   => {
                    let mut newstate = state.clone();
                    newstate.path.push((newx, newy));
                    new_items.push(newstate);
                }
                Tile::Door(k) => {
                    if state.collected_keys.contains(&k) {
                        let mut newstate = state.clone();
                        newstate.path.push((newx, newy));
                        new_items.push(newstate);
                    }
                }
                Tile::Key(k) => {
                    let mut newstate = state.clone();
                    newstate.path.push((newx, newy));
                    newstate.collected_keys.insert(k);
                    if newstate.collected_keys == maze.keyset {
                        return newstate.path;
                    }
                    new_items.push(newstate);
                }
                Tile::Entrance => {
                    let mut newstate = state.clone();
                    newstate.path.push((newx, newy));
                    new_items.push(newstate);
                }
            }
        }

        for newstate in new_items.drain(0..) {
            if !newstate.should_prune(&mut best_states) {
                queue.push_back(newstate);
            }
        }
    }

    panic!("Gave up finding all the keys")
}

#[test]
fn example1() {
    let example1 = Maze::new("#########\n#b.A.@.a#\n#########\n");
    assert_eq!((5, 1), example1.origin());
    let target1 = vec![(5, 1), (6, 1), (7, 1), (6, 1), (5, 1), (4, 1), (3, 1), (2, 1), (1, 1)];
    assert_eq!(target1, find_keys(&example1));
}

#[test]
fn example2() {
    let example2 = Maze::new("########################\n#f.D.E.e.C.b.A.@.a.B.c.#\n######################.#\n#d.....................#\n########################");
    assert_eq!(86, find_keys(&example2).len() - 1);
}

#[test]
fn example3() {
    let example3 = Maze::new("########################\n#...............b.C.D.f#\n#.######################\n#.....@.a.B.c.d.A.e.F.g#\n########################");
    assert_eq!(132, find_keys(&example3).len() - 1);
}

#[test]
fn example4() {
    let example4 = Maze::new("#################\n#i.G..c...e..H.p#\n########.########\n#j.A..b...f..D.o#\n########@########\n#k.E..a...g..B.n#\n########.########\n#l.F..d...h..C.m#\n#################");
    assert_eq!(136, find_keys(&example4).len() - 1);
}

#[test]
fn example5() {
    let example5 = Maze::new("########################\n#@..............ac.GI.b#\n###d#e#f################\n###A#B#C################\n###g#h#i################\n########################");
    assert_eq!(81, find_keys(&example5).len() - 1);
}

#[test]
fn day18a() {
    let day18_contents = fs::read("inputs/day18").expect("Couldn't open day18 problem");
    let day18_str = str::from_utf8(&day18_contents).expect("Couldn't decode day18 problem");
    let maze = Maze::new(&day18_str);
    assert_eq!(6098, find_keys(&maze).len() - 1);
}