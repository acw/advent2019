use std::fs;
use std::str;

#[derive(Clone, Debug, PartialEq)]
struct BugMap {
    has_bug: Vec<bool>,
    width: usize,
    height: usize,
}

impl BugMap {
    fn new(s: &str) -> BugMap {
        let mut width = 0;
        let mut height = 0;
        let mut has_bug = Vec::new();

        println!("s length {}", s.len());
        for line in s.trim().split('\n') {
            println!("line: |{}|", line);
            for c in line.chars() {
                match c {
                    '.' => has_bug.push(false),
                    '#' => has_bug.push(true),
                    _   => panic!("Unexpected character in bug map"),
                }
                if height == 0 {
                    width += 1;
                }
            }
            height += 1;
        }

        BugMap{ has_bug, width, height }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.has_bug[ (y * self.width) + x ]
    }

    fn set(&mut self, x: usize, y: usize, v: bool) {
        self.has_bug[ (y * self.width) + x ] = v;
    }

    fn next(&self) -> BugMap {
        let mut result = self.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                let above = if y > 0 { self.get(x, y - 1) } else { false };
                let below = if y < (self.height - 1) { self.get(x, y + 1) } else { false };
                let left  = if x > 0 { self.get(x - 1, y) } else { false };
                let right = if x < (self.width - 1) { self.get(x + 1, y) } else {false };
                let bugs_nearby = count(above, below, left, right);

                if self.get(x, y) {
                    result.set(x, y, bugs_nearby == 1);
                } else {
                    result.set(x, y, (bugs_nearby >= 1) && (bugs_nearby <= 2));
                }

            }
        }

        result
    }

    fn biodiversity(&self) -> u128 {
        let mut result = 0;
        let mut two_power = 1;

        for v in self.has_bug.iter() {
            if *v {
                result += two_power;
            }
            two_power <<= 1;
        }

        result
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", if self.get(x, y) { '#' } else { '.' });
            }
            println!();
        }
    }
}

fn count(a: bool, b: bool, c: bool, d: bool) -> u8 {
    (a as u8) + (b as u8) + (c as u8) + (d as u8)
}

fn find_duplicate(mut cur: BugMap) -> BugMap {
    let mut steps = Vec::new();

    while !steps.contains(&cur) {
        steps.push(cur.clone());
        cur = cur.next();
    }

    cur
}

#[test]
fn example() {
    let map = BugMap::new("....#\n#..#.\n#..##\n..#..\n#....");
    let endpoint = find_duplicate(map);
    assert_eq!(2129920, endpoint.biodiversity());
}

#[test]
fn day24() {
    let contents = fs::read("inputs/day24").expect("Couldn't read day 24 file");
    let bugstr = str::from_utf8(&contents).expect("Couldn't read a string from day 24");
    let bugmap = BugMap::new(&bugstr);
    let endpoint = find_duplicate(bugmap);
    assert_eq!(32776479, endpoint.biodiversity());
}