use crate::machine::{Computer, RunResult};
use image::{ImageBuffer, Rgb};

struct HullGrid {
    data: ImageBuffer<Rgb<u8>, Vec<u8>>,
    computer: Computer,
    robot_x: u32,
    robot_y: u32,
    robot_dir: Direction,
}

impl HullGrid {
    fn new(width: u32, height: u32, computer_path: &str) -> HullGrid {
        let computer = Computer::load(computer_path);

        assert!(width & 1 == 1);
        assert!(height & 1 == 1);
        HullGrid {
            data: ImageBuffer::new(width, height),
            computer,
            robot_x: width / 2,
            robot_y: height / 2,
            robot_dir: Direction::Up,
        }
    }

    fn is_white(&self) -> bool {
        let cur = self.data.get_pixel(self.robot_x, self.robot_y);
        cur == &Rgb([0xff,0xff,0xff])
    }

    fn set_black(&mut self) {
        self.data.put_pixel(self.robot_x, self.robot_y, Rgb([0x00,0x00,0x00]));
    }

    fn set_white(&mut self) {
        self.data.put_pixel(self.robot_x, self.robot_y, Rgb([0xff,0xff,0xff]));
    }

    fn step(&mut self) {
        match self.robot_dir {
            Direction::Up    => self.robot_y -= 1,
            Direction::Down  => self.robot_y += 1,
            Direction::Right => self.robot_x += 1,
            Direction::Left  => self.robot_x -= 1,
        }
    }

    fn paint_next(mut self, output: Option<&str>) -> Option<Self> {
        let mut new_color: Option<i64> = None;

        loop {
            let color = if self.is_white() { 1 } else { 0 };

            match self.computer.run() {
                RunResult::Continue(next) =>
                    self.computer = next,
                RunResult::Halted(next) => {
                    self.computer = next;
                    if let Some(fname) = output {
                        self.render(fname);
                    }
                    return None;
                }
                RunResult::Input(c) =>
                    self.computer = c(color),
                RunResult::Output(o, next) if new_color.is_none() => {
                    new_color = Some(o);
                    self.computer = next;
                }
                RunResult::Output(rotation, next) => {
                    self.computer = next;

                    if new_color.unwrap() == 0 {
                        self.set_black()
                    } else {
                        self.set_white()
                    }

                    self.robot_dir = if rotation == 0 {
                        self.robot_dir.rotate_right()
                    } else {
                        self.robot_dir.rotate_left()
                    };

                    self.step();
                    return Some(self);
                }
            }
        }
    }

    fn paint_hull(mut self, output: Option<&str>) -> usize {
        let mut points = vec![];

        while let Some(next) = self.paint_next(output) {
            self = next;
            let cur = (self.robot_x, self.robot_y);
            if !points.contains(&cur) {
                points.push(cur);
            }
        }

        points.len()
    }

    fn render(&self, file: &str) {
        match self.data.save(file) {
            Err(e) =>
                println!("Error saving file: {}", e),
            Ok(_) =>
                {}
        }
    }
}

#[derive(Debug,PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            Direction::Up    => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down  => Direction::Left,
            Direction::Left  => Direction::Up,
        }
    }

    fn rotate_left(&self) -> Direction {
        match self {
            Direction::Up    => Direction::Left,

            Direction::Right => Direction::Up,
            Direction::Down  => Direction::Right,
            Direction::Left  => Direction::Down,
        }
    }
}

#[test]
fn rotations_work() {
    assert_eq!(Direction::Up, Direction::Up.rotate_right().rotate_left());
    assert_eq!(Direction::Up, Direction::Up.rotate_left().rotate_right());
    assert_eq!(Direction::Right, Direction::Right.rotate_right().rotate_left());
    assert_eq!(Direction::Right, Direction::Right.rotate_left().rotate_right());
    assert_eq!(Direction::Down, Direction::Down.rotate_right().rotate_left());
    assert_eq!(Direction::Down, Direction::Down.rotate_left().rotate_right());
    assert_eq!(Direction::Left, Direction::Left.rotate_right().rotate_left());
    assert_eq!(Direction::Left, Direction::Left.rotate_left().rotate_right());
    /* */
    assert_eq!(Direction::Left, Direction::Left.rotate_right().rotate_right().rotate_right().rotate_right());
    assert_eq!(Direction::Right, Direction::Right.rotate_right().rotate_right().rotate_right().rotate_right());
    assert_eq!(Direction::Down, Direction::Down.rotate_right().rotate_right().rotate_right().rotate_right());
    assert_eq!(Direction::Up, Direction::Up.rotate_right().rotate_right().rotate_right().rotate_right());
    /* */
    assert_eq!(Direction::Left, Direction::Left.rotate_left().rotate_left().rotate_left().rotate_left());
    assert_eq!(Direction::Right, Direction::Right.rotate_left().rotate_left().rotate_left().rotate_left());
    assert_eq!(Direction::Down, Direction::Down.rotate_left().rotate_left().rotate_left().rotate_left());
    assert_eq!(Direction::Up, Direction::Up.rotate_left().rotate_left().rotate_left().rotate_left());
}

#[test]
fn day11() {
    let day1a = HullGrid::new(1001, 1001, "inputs/day11");
    assert_eq!(2373, day1a.paint_hull(None));
    let mut day1b = HullGrid::new(1001, 1001, "inputs/day11");
    day1b.set_white();
    assert_eq!(249, day1b.paint_hull(None));
}