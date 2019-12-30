use std::collections::VecDeque;
use std::fs;
use std::str;

struct Maze<TileType> {
    data: Vec<TileType>,
    width: usize,
    height: usize,
}

#[derive(Clone, Debug, PartialEq)]
enum InputTile {
    Wall,
    Empty,
    Blank,
    Letter(char),
}

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Jump(String),
}

impl<'a> From<&'a InputTile> for Tile {
    fn from(x: &InputTile) -> Tile {
        match x {
            InputTile::Wall      => Tile::Wall,
            InputTile::Empty     => Tile::Empty,
            InputTile::Blank     => Tile::Wall,
            InputTile::Letter(_) => Tile::Wall,
        }
    }
}

impl<T: Clone> Maze<T> {
    fn get(&self, x: usize, y:usize) -> T {
        assert!(x < self.width);
        assert!(y < self.height);
        self.data[ (self.width * y) + x ].clone()
    }

    fn set(&mut self, x: usize, y: usize, v: T) {
        self.data[ (self.width * y) + x ] = v;
    }
}

impl Maze<InputTile> {
    fn new(f: &str) -> Maze<InputTile> {
        let contents = fs::read(f).expect("Couldn't open input donut maze");
        let strmap = str::from_utf8(&contents).expect("Couldn't turn donut maze into string");
        let mut data = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for line in strmap.trim_right().split('\n') {
            for c in line.chars() {
                match c {
                    ' ' => data.push(InputTile::Blank),
                    '.' => data.push(InputTile::Empty),
                    '#' => data.push(InputTile::Wall),
                    x if x.is_ascii_alphabetic() => data.push(InputTile::Letter(x)),
                    _ => panic!("Unknown character {}", c),
                }
                if height == 0 {
                    width += 1;
                }
            }
            height += 1;
        }

        while data.len() < (width * height) { data.push(InputTile::Blank); }

        Maze{ data, width, height }
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    InputTile::Blank     => print!(" "),
                    InputTile::Empty     => print!("."),
                    InputTile::Wall      => print!("#"),
                    InputTile::Letter(c) => print!("{}", c),
                }
            }
            println!();
        }
    }
}

impl Maze<Tile> {
    fn new(f: &str) -> Maze<Tile> {
        let inputmap: Maze<InputTile> = Maze::<InputTile>::new(f);
        let mut top = 0;
        let mut left = 0;
        let mut bottom = 0;
        let mut right = 0;

        // find the top left corner
        for y in 0..inputmap.height {
            for x in 0..inputmap.width {
                if top == 0 && inputmap.get(x, y) == InputTile::Wall {
                    top = y;
                    left = x;
                    break;
                }
            }
        }

        // find the bottom right corner
        for y in (0..inputmap.height).rev() {
            for x in (0..inputmap.width).rev() {
                if bottom == 0 && inputmap.get(x, y) == InputTile::Wall {
                    bottom = y;
                    right = x;
                    break;
                }
            }
        }

        let width = (right - left) + 1;
        let height = (bottom - top) + 1;
        let mut data = Vec::with_capacity(width * height);
        // Just copy the core bits over.
        for y in 0..height {
            for x in 0..width {
                data.push( Tile::from(&inputmap.get(x + left, y + top)) );
            }
        }

        // now we go back through and add the jump points.
        let mut res = Maze{ data, width, height };
        for y in 0..inputmap.height-1 {
            for x in 0..inputmap.width-1 {
                if let InputTile::Letter(c1) = inputmap.get(x, y) {
                    let mut s = String::new();

                    s.push(c1);
                    if let InputTile::Letter(c2) = inputmap.get(x + 1, y) {
                        s.push(c2);
                        if x + 2 < inputmap.width && inputmap.get(x + 2, y) == InputTile::Empty {
                            res.set( (x + 2) - left, y - top, Tile::Jump(s));
                        } else if x > 0 && inputmap.get(x - 1, y) == InputTile::Empty {
                            res.set( (x - 1) - left, y - top, Tile::Jump(s));
                        }
                    } else if let InputTile::Letter(c2) = inputmap.get(x, y + 1) {
                        s.push(c2);
                        if y + 2 < inputmap.height && inputmap.get(x, y + 2) == InputTile::Empty {
                            res.set( x - left, (y + 2) - top, Tile::Jump(s));
                        } else if y > 0 && inputmap.get(x, y - 1) == InputTile::Empty {
                            res.set( x - left, y - 1 - top, Tile::Jump(s));
                        }
                    }
                }
            }
        }
        
        res
    }

    fn origin(&self) -> (usize, usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) == Tile::Jump("AA".to_string()) {
                    return (x, y);
                }
            }
        }
        panic!("Couldn't find origin!");
    }

    fn jump_from(&self, sx: usize, sy: usize) -> Option<(usize, usize)> {
        if let Tile::Jump(label) = self.get(sx, sy) {
            for y in 0..self.height {
                for x in 0..self.width {
                    if (sx != x) || (sy != y) {
                        if let Tile::Jump(label2) = self.get(x, y) {
                            if label == label2 {
                                return Some((x, y));
                            }
                        }
                    }
                }
            }
        }

        None
    }

    fn next_moves(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut res = Vec::new();

        if x > 0 { res.push((x - 1, y)); }
        if y > 0 { res.push((x, y - 1)); }
        if x < (self.width - 1) { res.push((x + 1, y)); }
        if y < (self.height - 1) { res.push((x, y + 1)); }
        if let Some(target) = self.jump_from(x, y) { res.push(target); }

        res
    }

    fn find_path(&self) -> Vec<(usize, usize)> {
        let initial_path = vec![self.origin()];
        let mut queue = VecDeque::new();

        queue.push_back(initial_path);
        while let Some(mut cur) = queue.pop_front() {
            assert_ne!(cur.len(), 0);
            let (x, y) = cur[cur.len() - 1];
            let mut nexts = self.next_moves(x, y);
            for next in nexts.drain(0..) {
                let (nx, ny) = next;

                if let Tile::Jump(lbl) = self.get(nx, ny) {
                    if lbl == "ZZ".to_string() {
                        cur.push((nx, ny));
                        return cur;
                    }
                }

                if cur.contains(&next) {
                    continue;
                }

                if self.get(nx, ny) == Tile::Wall {
                    continue;
                }

                let mut newcopy = cur.clone();
                newcopy.push((nx, ny));
                queue.push_back(newcopy);
            }
        }
        panic!("No path found!")
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    Tile::Empty   => print!("."),
                    Tile::Wall    => print!("#"),
                    Tile::Jump(s) => print!("{}", s.chars().next().unwrap()),
                }
            }
            println!();
        }
    }
}

#[test]
fn example1() {
    let maze = Maze::<Tile>::new("inputs/day20_example1");
    assert_eq!(None, maze.jump_from(7, 0));
    assert_eq!(Some((0, 6)), maze.jump_from(7, 4));
    assert_eq!(Some((7, 4)), maze.jump_from(0, 6));
    assert_eq!(23, maze.find_path().len() - 1);
}

#[test]
fn example2() {
    let maze = Maze::<Tile>::new("inputs/day20_example2");
    let path = maze.find_path();
    assert_eq!(58, path.len() - 1);
}

#[test]
fn day20() {
    let maze = Maze::<Tile>::new("inputs/day20");
    let path = maze.find_path();
    assert_eq!(606, path.len() - 1);
}