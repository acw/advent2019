use std::cmp::{max,min};
use std::io::{Write,stdout};
use std::iter::FromIterator;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug,PartialEq)]
pub struct WireMap {
    map: Vec<Vec<WireState>>,
    turtle: (usize, usize),
    turtle_color: WireState,
}

impl WireMap {
    pub fn new() -> WireMap {
        WireMap{ map: vec![vec![WireState::Origin]],
                 turtle: (0,0),
                 turtle_color: WireState::Nothing }
    }

    fn origin(&self) -> (usize, usize) {
        for (y, line) in self.map.iter().enumerate() {
            for (x, pos) in line.iter().enumerate() {
                if *pos == WireState::Origin {
                    return (x, y);
                }
            }
        }
        panic!("No origin found?!");
    }

    pub fn joins(&self) -> Vec<(usize,usize)> {
        let mut res = Vec::new();

        for (y, line) in self.map.iter().enumerate() {
            for (x, pos) in line.iter().enumerate() {
                if *pos == WireState::BothWires {
                    res.push((x, y));
                }
            }
        }

        res
    }

    pub fn closest_intersection(&self) -> ((usize, usize), usize) {
        let mut best = usize::max_value();
        let mut best_point = (0, 0);
        let (ox, oy) = self.origin();

        for (jx, jy) in self.joins().iter() {
            let maxx = max(ox, *jx);
            let minx = min(ox, *jx);
            let maxy = max(oy, *jy);
            let miny = min(oy, *jy);
            let distance = (maxx - minx) + (maxy - miny);
            if distance < best {
                best = distance;
                best_point = (*jx, *jy);
            }
        }

        (best_point, best)
    }

    fn reset_turtle(&mut self, color: WireState) {
        self.turtle_color = color;
        self.turtle = self.origin();
    }

    fn advance_down(&mut self, mut amt: usize) {
        let (cur_x, mut cur_y) = self.turtle;
        let end_y = cur_y + amt;

        // make sure we're not going to fall off the end
        if end_y >= self.map.len() {
            let mut new_row = Vec::with_capacity(self.map[0].len());
            new_row.resize(self.map[0].len(), WireState::Nothing);
            self.map.resize(end_y + 1, new_row);
        }

        while amt > 0 {
            cur_y += 1;
            self.map[cur_y][cur_x] = self.map[cur_y][cur_x].merge(&self.turtle_color);
            amt -= 1;
        }

        self.turtle = (cur_x, cur_y);
    }

    fn advance_up(&mut self, mut amt: usize) {
        let (cur_x, mut cur_y) = self.turtle;

        while amt > cur_y {
            let mut new_row = Vec::with_capacity(self.map[0].len());
            new_row.resize(self.map[0].len(), WireState::Nothing);
            self.map.insert(0, new_row);
            cur_y += 1;
        }

        while amt > 0 {
            cur_y -= 1;
            self.map[cur_y][cur_x] = self.map[cur_y][cur_x].merge(&self.turtle_color);
            amt -= 1;
        }

        self.turtle = (cur_x, cur_y);
    }

    fn advance_right(&mut self, mut amt: usize) {
        let (mut cur_x, cur_y) = self.turtle;
        let end_x = cur_x + amt;

        // make sure we're not going to fall off the end
        if end_x >= self.map[0].len() {
            for row in self.map.iter_mut() {
                row.resize(end_x + 1, WireState::Nothing);
            }
        }

        while amt > 0 {
            cur_x += 1;
            self.map[cur_y][cur_x] = self.map[cur_y][cur_x].merge(&self.turtle_color);
            amt -= 1;
        }

        self.turtle = (cur_x, cur_y);
    }

    fn advance_left(&mut self, mut amt: usize) {
        let (mut cur_x, cur_y) = self.turtle;
        
        while amt > cur_x {
            for row in self.map.iter_mut() {
                row.insert(0, WireState::Nothing);
            }
            cur_x += 1;
        }

        while amt > 0 {
            cur_x -= 1;
            self.map[cur_y][cur_x] = self.map[cur_y][cur_x].merge(&self.turtle_color);
            amt -= 1;
        }

        self.turtle = (cur_x, cur_y);
    }

    pub fn add_wire(&mut self, wire: &Wire, num: usize) {
        let state = if num == 1 { WireState::Wire1 } else { WireState::Wire2 };
        self.reset_turtle(state);
        print!("Adding wire {}: ", num);
        for segment in wire.segments.iter() {
            print!("{} ", segment); let _ = stdout().flush();
            match segment.direction {
                Direction::Right => self.advance_right(segment.magnitude),
                Direction::Left  => self.advance_left(segment.magnitude),
                Direction::Down  => self.advance_down(segment.magnitude),
                Direction::Up    => self.advance_up(segment.magnitude),
            }
        }
        println!("DONE.");
    }

    pub fn steps_to(&self, wire: &Wire, target: (usize, usize)) -> usize {
        let mut steps = 0;
        let (mut cur_x, mut cur_y) = self.origin();

        for segment in wire.segments.iter() {
            let mut count = segment.magnitude;

            match segment.direction {
                Direction::Right => {
                    while count > 0 {
                        cur_x += 1; steps += 1; count -= 1;
                        if (cur_x, cur_y) == target {
                            return steps;
                        }
                    } 
                }
                Direction::Left => {
                    while count > 0 {
                        cur_x -= 1; steps += 1; count -= 1;
                        if (cur_x, cur_y) == target {
                            return steps;
                        }
                    } 
                }
                Direction::Down => {
                    while count > 0 {
                        cur_y += 1; steps += 1; count -= 1;
                        if (cur_x, cur_y) == target {
                            return steps;
                        }
                    } 
                }
                Direction::Up => {
                    while count > 0 {
                        cur_y -= 1; steps += 1; count -= 1;
                        if (cur_x, cur_y) == target {
                            return steps;
                        }
                    } 
                }
            }
        }

        panic!("Didn't find target on wire path?!");
    }
}

#[derive(Clone,Debug,PartialEq)]
pub enum WireState {
    Nothing,
    Origin,
    Wire1,
    Wire2,
    BothWires
}

impl WireState {
    fn merge(&self, other: &WireState) -> WireState {
        if self == &WireState::Nothing {
            return other.clone();
        }

        if self == &WireState::Origin {
            panic!("Ran back over Origin!");
        }

        if self == &WireState::BothWires {
            return self.clone();
        }

        if self == other {
            return self.clone();
        }

        WireState::BothWires
    }
}

#[derive(Debug,PartialEq)]
pub struct Wire {
    segments: Vec<Segment>
}

#[derive(Debug)]
pub enum WireParseError {
    SegmentParseError(SegmentParseError),
    NoSegmentsFound
}

impl From<SegmentParseError> for WireParseError {
    fn from(x: SegmentParseError) -> WireParseError {
        WireParseError::SegmentParseError(x)
    }
}

impl FromStr for Wire {
    type Err = WireParseError;

    fn from_str(s: &str) -> Result<Wire, Self::Err> {
        let mut segments = Vec::new();
        let mut chars = s.chars().peekable();

        while chars.peek().is_some() {
            let next_str = chars.by_ref().take_while(|c| *c != ',');
            let next = Segment::from_str(&String::from_iter(next_str))?;
            segments.push(next);
        }

        if segments.is_empty() {
            return Err(WireParseError::NoSegmentsFound);
        }

        Ok(Wire{ segments })
    }
}

#[derive(Debug,PartialEq)]
pub struct Segment {
    direction: Direction,
    magnitude: usize,
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.direction {
           Direction::Right => write!(f, "R{}", self.magnitude),
           Direction::Left  => write!(f, "L{}", self.magnitude),
           Direction::Up    => write!(f, "U{}", self.magnitude),
           Direction::Down  => write!(f, "D{}", self.magnitude),
        }
    }
}

#[derive(Debug)]
pub enum SegmentParseError {
    UnknownDirection(char),
    NumberParseError(ParseIntError),
    NoDirectionFound
}

impl From<ParseIntError> for SegmentParseError {
    fn from(x: ParseIntError) -> SegmentParseError {
        SegmentParseError::NumberParseError(x)
    }
}

impl FromStr for Segment {
    type Err = SegmentParseError;

    fn from_str(s: &str) -> Result<Segment,Self::Err> {
        let magnitude = usize::from_str_radix(&s[1..], 10)?;
        let direction = match s.chars().nth(0) {
            None      => return Err(SegmentParseError::NoDirectionFound),
            Some('R') => Direction::Right,
            Some('L') => Direction::Left,
            Some('U') => Direction::Up,
            Some('D') => Direction::Down,
            Some(d)   => return Err(SegmentParseError::UnknownDirection(d)),
        };

        Ok(Segment{ direction, magnitude })
    }
}

#[derive(Debug,PartialEq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[test]
fn segment_parsing() {
    let test1   = Segment::from_str("R75").unwrap();
    let answer1 = Segment{ direction: Direction::Right, magnitude: 75 };
    assert_eq!(test1, answer1);
    let test2   = Segment::from_str("U7").unwrap();
    let answer2 = Segment{ direction: Direction::Up,    magnitude: 7  };
    assert_eq!(test2, answer2);
    let test3   = Segment::from_str("D20").unwrap();
    let answer3 = Segment{ direction: Direction::Down,  magnitude: 20 };
    assert_eq!(test3, answer3);
    let test4   = Segment::from_str("L0").unwrap();
    let answer4 = Segment{ direction: Direction::Left,  magnitude: 0  };
    assert_eq!(test4, answer4);
}

#[test]
fn wire_parsing() {
    let input1  = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
    let test1   = Wire::from_str(input1).unwrap();
    let answer1 = Wire {
        segments: vec![ Segment{ direction: Direction::Right, magnitude: 75 },
                        Segment{ direction: Direction::Down,  magnitude: 30 },
                        Segment{ direction: Direction::Right, magnitude: 83 },
                        Segment{ direction: Direction::Up,    magnitude: 83 },
                        Segment{ direction: Direction::Left,  magnitude: 12 },
                        Segment{ direction: Direction::Down,  magnitude: 49 },
                        Segment{ direction: Direction::Right, magnitude: 71 },
                        Segment{ direction: Direction::Up,    magnitude: 7  },
                        Segment{ direction: Direction::Left,  magnitude: 72 },
                      ]
    };
    assert_eq!(test1, answer1);
}

#[test]
fn extend_down() {
    let mut base = WireMap::new();
    base.turtle_color = WireState::Wire1;
    base.advance_down(4);
    let target = WireMap {
        map: vec![vec![WireState::Origin],
                  vec![WireState::Wire1],
                  vec![WireState::Wire1],
                  vec![WireState::Wire1],
                  vec![WireState::Wire1],
                 ],
        turtle: (0, 4),
        turtle_color: WireState::Wire1,
    };
    assert_eq!(target, base);
}

#[test]
fn extend_up() {
    let mut base = WireMap::new();
    base.turtle_color = WireState::Wire1;
    base.advance_up(4);
    let target = WireMap {
        map: vec![vec![WireState::Wire1],
                  vec![WireState::Wire1],
                  vec![WireState::Wire1],
                  vec![WireState::Wire1],
                  vec![WireState::Origin],
                 ],
        turtle: (0, 0),
        turtle_color: WireState::Wire1,
    };
    assert_eq!(target, base);
}

#[test]
fn extend_right() {
    let mut base = WireMap::new();
    base.turtle_color = WireState::Wire1;
    base.advance_right(4);
    let target = WireMap {
        map: vec![vec![WireState::Origin,
                       WireState::Wire1,
                       WireState::Wire1,
                       WireState::Wire1,
                       WireState::Wire1,
                      ]
                 ],
        turtle: (4, 0),
        turtle_color: WireState::Wire1,
    };
    assert_eq!(target, base);
}

#[test]
fn extend_left() {
    let mut base = WireMap::new();
    base.turtle_color = WireState::Wire1;
    base.advance_left(4);
    let target = WireMap {
        map: vec![vec![WireState::Wire1,
                       WireState::Wire1,
                       WireState::Wire1,
                       WireState::Wire1,
                       WireState::Origin,
                      ]
                 ],
        turtle: (0, 0),
        turtle_color: WireState::Wire1,
    };
    assert_eq!(target, base);
}

#[cfg(test)]
const B: WireState = WireState::BothWires;
#[cfg(test)]
const N: WireState = WireState::Nothing;
#[cfg(test)]
const O: WireState = WireState::Origin;
#[cfg(test)]
const W1: WireState = WireState::Wire1;
#[cfg(test)]
const W2: WireState = WireState::Wire2;

#[test]
fn example_wires() {
    let mut base = WireMap::new();
    let wire1 = Wire{ segments: vec![
        Segment{ direction: Direction::Right, magnitude: 8 },
        Segment{ direction: Direction::Up,    magnitude: 5 },
        Segment{ direction: Direction::Left,  magnitude: 5 },
        Segment{ direction: Direction::Down,  magnitude: 3 },
    ] };
    base.add_wire(&wire1, 1);
    let target1 = WireMap {
        turtle: (3, 3),
        turtle_color: WireState::Wire1,
        map: vec![
            vec![N,  N,  N,  W1, W1, W1, W1, W1, W1],
            vec![N,  N,  N,  W1, N,  N,  N,  N,  W1],
            vec![N,  N,  N,  W1, N,  N,  N,  N,  W1],
            vec![N,  N,  N,  W1, N,  N,  N,  N,  W1],
            vec![N,  N,  N,  N,  N,  N,  N,  N,  W1],
            vec![O,  W1, W1, W1, W1, W1, W1, W1, W1],
        ]
    };
    assert_eq!(base, target1);
    let wire2 = Wire{ segments: vec![
        Segment{ direction: Direction::Up,    magnitude: 7 },
        Segment{ direction: Direction::Right, magnitude: 6 },
        Segment{ direction: Direction::Down,  magnitude: 4 },
        Segment{ direction: Direction::Left,  magnitude: 4 },
    ] };
    base.add_wire(&wire2, 2);
    let target2 = WireMap {
        turtle: (2, 4),
        turtle_color: WireState::Wire2,
        map: vec![
            vec![W2, W2, W2, W2, W2, W2, W2, N,  N ],
            vec![W2, N,  N,  N,  N,  N,  W2, N,  N ],
            vec![W2, N,  N,  W1, W1, W1, B,  W1, W1],
            vec![W2, N,  N,  W1, N,  N,  W2, N,  W1],
            vec![W2, N,  W2, B,  W2, W2, W2, N,  W1],
            vec![W2, N,  N,  W1, N,  N,  N,  N,  W1],
            vec![W2, N,  N,  N,  N,  N,  N,  N,  W1],
            vec![O,  W1, W1, W1, W1, W1, W1, W1, W1],
        ]
    };
    assert_eq!(base, target2);
    assert_eq!((0, 7), base.origin());
    assert_eq!(vec![(6,2),(3,4)], base.joins());
}

#[test]
fn example_distances() {
    let mut example1 = WireMap::new();
    let ex1wire1 = Wire{ segments: vec![
        Segment{ direction: Direction::Right, magnitude: 8 },
        Segment{ direction: Direction::Up,    magnitude: 5 },
        Segment{ direction: Direction::Left,  magnitude: 5 },
        Segment{ direction: Direction::Down,  magnitude: 3 },
    ] };
    example1.add_wire(&ex1wire1, 1);
    let ex1wire2 = Wire{ segments: vec![
        Segment{ direction: Direction::Up,    magnitude: 7 },
        Segment{ direction: Direction::Right, magnitude: 6 },
        Segment{ direction: Direction::Down,  magnitude: 4 },
        Segment{ direction: Direction::Left,  magnitude: 4 },
    ] };
    example1.add_wire(&ex1wire2, 2);
    let (ex1inter, ex1dist) = example1.closest_intersection();
    assert_eq!(6, ex1dist);
    assert_eq!(20, example1.steps_to(&ex1wire1, ex1inter));
    assert_eq!(20, example1.steps_to(&ex1wire2, ex1inter));

    let mut example2 = WireMap::new();
    let ex2wire1 = Wire{ segments: vec![
        Segment{ direction: Direction::Right, magnitude: 75 },
        Segment{ direction: Direction::Down,  magnitude: 30 },
        Segment{ direction: Direction::Right, magnitude: 83 },
        Segment{ direction: Direction::Up,    magnitude: 83 },
        Segment{ direction: Direction::Left,  magnitude: 12 },
        Segment{ direction: Direction::Down,  magnitude: 49 },
        Segment{ direction: Direction::Right, magnitude: 71 },
        Segment{ direction: Direction::Up,    magnitude: 7  },
        Segment{ direction: Direction::Left,  magnitude: 72 },
    ] };
    example2.add_wire(&ex2wire1, 1);
    let ex2wire2 = Wire{ segments: vec![
        Segment{ direction: Direction::Up,    magnitude: 62 },
        Segment{ direction: Direction::Right, magnitude: 66 },
        Segment{ direction: Direction::Up,    magnitude: 55 },
        Segment{ direction: Direction::Right, magnitude: 34 },
        Segment{ direction: Direction::Down,  magnitude: 71 },
        Segment{ direction: Direction::Right, magnitude: 55 },
        Segment{ direction: Direction::Down,  magnitude: 58 },
        Segment{ direction: Direction::Right, magnitude: 83 },
    ] };
    example2.add_wire(&ex2wire2, 2);
    assert_eq!(159, example2.closest_intersection().1);

    let mut example3 = WireMap::new();
    let ex3wire1 = Wire{ segments: vec![
        Segment{ direction: Direction::Right, magnitude: 98 },
        Segment{ direction: Direction::Up,    magnitude: 47 },
        Segment{ direction: Direction::Right, magnitude: 26 },
        Segment{ direction: Direction::Down,  magnitude: 63 },
        Segment{ direction: Direction::Right, magnitude: 33 },
        Segment{ direction: Direction::Up,    magnitude: 87 },
        Segment{ direction: Direction::Left,  magnitude: 62 },
        Segment{ direction: Direction::Down,  magnitude: 20 },
        Segment{ direction: Direction::Right, magnitude: 33 },
        Segment{ direction: Direction::Up,    magnitude: 53  },
        Segment{ direction: Direction::Right, magnitude: 51 },
    ] };
    example3.add_wire(&ex3wire1, 1);
    let ex3wire2 = Wire{ segments: vec![
        Segment{ direction: Direction::Up,    magnitude: 98 },
        Segment{ direction: Direction::Right, magnitude: 91 },
        Segment{ direction: Direction::Down,  magnitude: 20 },
        Segment{ direction: Direction::Right, magnitude: 16 },
        Segment{ direction: Direction::Down,  magnitude: 67 },
        Segment{ direction: Direction::Right, magnitude: 40 },
        Segment{ direction: Direction::Up,    magnitude: 7  },
        Segment{ direction: Direction::Right, magnitude: 15 },
        Segment{ direction: Direction::Up,    magnitude: 6  },
        Segment{ direction: Direction::Right, magnitude: 7  },
    ] };
    example3.add_wire(&ex3wire2, 2);
    assert_eq!(135, example3.closest_intersection().1);
}