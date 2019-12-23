use crate::endchannel::{Receiver, Sender, channel};
use crate::machine::Computer;
use terminal_graphics::{Colour, Display};
use std::fmt;
use std::thread;

pub struct Arcade {
    screen: Vec<Tile>,
    width: usize,
    height: usize,
    pub score: usize,
    joystick_port: Sender<i64>,
    update_port: Receiver<Update>,
    ball: (usize, usize),
    paddle: (usize, usize),
}

pub fn auto_move(arcade: &Arcade) -> Move {
    let (ball_x, _) = arcade.ball;
    let (paddle_x, _) = arcade.paddle;

    if paddle_x < ball_x {
        return Move::Right;
    }

    if paddle_x > ball_x {
        return Move::Left;
    }

    Move::Neutral
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

enum Update {
    Score(usize),
    Ball(usize, usize),
    Paddle(usize, usize),
    Draw(usize, usize, Tile),
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
        let (    mysend, mut corecv) = channel();
        let (mut cosend, mut myrecv) = channel();
        let (mut upsend,     uprecv) = channel();

        if cheat { logic.write(0, 2); }
        thread::spawn(move || logic.run(&mut corecv, &mut cosend));

        let mut screen = Vec::with_capacity(width * height);
        screen.resize(width * height, Tile::Empty);

        thread::spawn(move || {
            while let Some(first) = myrecv.recv() {
                let second = myrecv.recv().expect("Didn't get second?!");
                let third  = myrecv.recv().expect("Didn't get third?!");

                if first == -1 && second == 0 {
                    upsend.send_ignore_error(Update::Score(third as usize));
                } else {
                    let x = first as usize;
                    let y = second as usize;
                    let t = Tile::new(third);
                    upsend.send_ignore_error(Update::Draw(x, y, t));
                    if t == Tile::Ball             {
                        upsend.send_ignore_error(Update::Ball(x, y));
                    }
                    if t == Tile::HorizontalPaddle {
                        upsend.send_ignore_error(Update::Paddle(x, y));
                    }
                }
            }
            upsend.conclude();
        });

        Arcade {
            screen,
            width,
            height,
            joystick_port: mysend,
            update_port: uprecv,
            score: 0,
            ball: (0, 0),
            paddle: (0, 0),
        }
    }

    fn count_blocks(&self) -> usize {
        let mut count = 0;

        for tile in self.screen.iter() {
            if tile == &Tile::Block {
                count += 1;
            }
        }

        count
    }

    pub fn process_update<F>(&mut self, next_move: F) -> bool
         where F: Fn(&Arcade) -> Move
    {
        let next = self.update_port.recv();

        if let Some(ref update) = next {
            match update {
                &Update::Score(s) => self.score = s,
                &Update::Draw(x, y, t) => self.screen[ (y * self.width) + x ] = t,
                &Update::Ball(x, y) => {
                    self.ball = (x, y);
                    let next = next_move(&self);
                    self.paddle_move(next);
                }
                &Update::Paddle(x, y) => {
                    self.paddle = (x, y);
                //    let next = next_move(&self);
                //    self.paddle_move(next);
                }
            }
        }

        next.is_some()
    }

    pub fn paddle_move(&mut self, motion: Move) {
        match motion {
            Move::Left    => self.joystick_port.send_ignore_error(-1),
            Move::Neutral => self.joystick_port.send_ignore_error(0),
            Move::Right   => self.joystick_port.send_ignore_error(1),
        }
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

#[test]
fn day13() {
    let mut arcade1 = Arcade::new(38, 21, false, "inputs/day13");
    while arcade1.process_update(auto_move) { }
    assert_eq!(301, arcade1.count_blocks());

    let mut arcade2 = Arcade::new(38, 21, true, "inputs/day13");
    while arcade2.process_update(auto_move) { }
    assert_eq!(14096, arcade2.score);
}
