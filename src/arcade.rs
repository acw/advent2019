use crate::machine::{Computer, RunResult};
use terminal_graphics::{Colour, Display};
use std::collections::VecDeque;
use std::fmt;

pub struct Arcade {
    screen: Vec<Tile>,
    width: usize,
    height: usize,
    pub score: usize,
    logic: Computer,
    ball: (usize, usize),
    paddle: (usize, usize),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Empty            => write!(f, "Empty"),
            Tile::Wall             => write!(f, "Wall"),
            Tile::Block            => write!(f, "Block"),
            Tile::HorizontalPaddle => write!(f, "HorizontalPaddle"),
            Tile::Ball             => write!(f, "Ball"),
        }
    }
}

impl Tile {
    fn new(x: i64) -> Tile {
        match x {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => panic!("Unknown tile type: {}", x),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Tile::Empty            => ' ',
            Tile::Wall             => '█',
            Tile::Block            => '░',
            Tile::HorizontalPaddle => '=',
            Tile::Ball             => 'o',
        }
    }
}

impl Arcade {
    pub fn new(width: usize, height: usize, cheat: bool, logic_file: &str) -> Arcade {
        let mut logic = Computer::load(logic_file);

        if cheat { logic.write(0, 2); }
        let mut screen = Vec::with_capacity(width * height);
        screen.resize(width * height, Tile::Empty);
        Arcade {
            screen,
            width,
            height,
            logic,
            score: 0,
            ball: (0, 0),
            paddle: (0, 0),
        }
    }

    pub fn run<F: FnMut(&Arcade)>(mut self, mut redraw: F) -> Self {
        let mut output_buffer = vec![];
        let mut input_buffer = VecDeque::new();

        loop {
            match self.logic.run() {
                RunResult::Continue(next) =>
                    self.logic = next,
                RunResult::Halted(next) => {
                    self.logic = next;
                    return self;
                }
                RunResult::Output(x, next) => {
                    self.logic = next;
                    output_buffer.push(x);
                    if output_buffer.len() == 3 {
                        if output_buffer[0] == -1 && output_buffer[1] == 0 {
                            self.score = output_buffer[2] as usize;
                        } else {
                            let x = output_buffer[0] as usize;
                            let y = output_buffer[1] as usize;
                            let t = Tile::new(output_buffer[2]);
                            self.screen[ (y * self.width) + x ] = t;
                            if t == Tile::Ball {
                                self.ball = (x, y);
                                let (paddle_x, _) = self.paddle;

                                if paddle_x < x {
                                    input_buffer.push_back(Move::Right);
                                } else if paddle_x > x {
                                    input_buffer.push_back(Move::Left);
                                } else {
                                    input_buffer.push_back(Move::Neutral);
                                }
                            }
                            if t == Tile::HorizontalPaddle {
                                self.paddle = (x, y);
                            }
                        }
                        output_buffer = vec![];
                    }
                }
                RunResult::Input(c) => {
                    self.logic = c(input_buffer.pop_front().unwrap().encode());
                    redraw(&self);
                }
            }
        }
    }

    #[cfg(test)]
    fn count_blocks(&self) -> usize {
        let mut count = 0;

        for tile in self.screen.iter() {
            if tile == &Tile::Block {
                count += 1;
            }
        }

        count
    }

    pub fn draw(&self, display: &mut Display) {
        write_to_screen(display, 0, &format!("Score: {}", self.score));
        for row in 0..self.height {
            for col in 0..self.width {
                let c = self.screen[ (row * self.width) + col ].to_char();
                display.set_pixel(col as isize, (row + 2) as isize, c, Colour::White, Colour::Black);
            }
        }
        write_to_screen(display, 37, &format!("Paddle: {:?}", self.paddle));
        write_to_screen(display, 38, &format!("Ball: {:?}", self.ball));
    }
}

fn write_to_screen(display: &mut Display, row: isize, s: &str) {
    let mut col = 0;

    for c in s.chars() {
        display.set_pixel(col, row, c, Colour::White, Colour::Black);
        col += 1;
    }
}

pub enum Move {
    Left,
    Neutral,
    Right,
}

impl Move {
    fn encode(&self) -> i64 {
        match self {
            Move::Left    => -1,
            Move::Neutral => 0,
            Move::Right   => 1,
        }
    }
}

#[test]
fn day13() {
    let arcade1 = Arcade::new(38, 21, false, "inputs/day13");
    let result1 = arcade1.run(|_| {});
    assert_eq!(301, result1.count_blocks());

    let arcade2 = Arcade::new(38, 21, true, "inputs/day13");
    let result2 = arcade2.run(|_| {});
    assert_eq!(14096, result2.score);
}
