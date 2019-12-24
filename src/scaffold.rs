use crate::machine::{Computer, RunResult};

struct ScaffoldMap {
    data: Vec<Tile>,
    width: usize,
    height: usize,
}

impl ScaffoldMap {
    fn new(intcode: &str) -> ScaffoldMap {
        let mut computer = Computer::load(intcode);
        let mut data = Vec::new();
        let mut width = 0;
        let mut height = 0;
        let mut got_width = false;

        loop {
            match computer.run() {
                RunResult::Halted(_) => {
                    height -= 1;
                    assert_eq!(height, (data.len() / width));
                    return ScaffoldMap{
                        data,
                        width, height
                    }
                }

                RunResult::Continue(next) =>
                    computer = next,

                RunResult::Input(_) =>
                    panic!("Don't know how to deal with input!"),

                RunResult::Output(o, next) => {
                    let c = o as u8 as char;

                    computer = next;

                    if c == '\n' {
                        height += 1;
                        got_width = true;
                        continue;
                    }
                    if !got_width {
                        width += 1;
                    }

                    data.push(Tile::from(c));
                }
            }
        }
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        assert!(x < self.width);
        assert!(y < self.height);
        self.data[ (y * self.width) + x ].clone()
    }

    fn is_cross(&self, x: usize, y: usize) -> bool {
        (x > 0) && (y > 0) && (x < (self.width - 1)) && (y < (self.height - 1)) &&
        (self.get(x, y) == Tile::Scaffold) &&
        (self.get(x - 1, y) == Tile::Scaffold) &&
        (self.get(x + 1, y) == Tile::Scaffold) &&
        (self.get(x, y - 1) == Tile::Scaffold) &&
        (self.get(x, y + 1) == Tile::Scaffold)
    }

    fn join_points(&self) -> Vec<(usize, usize)> {
        let mut res = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_cross(x, y) {
                    res.push((x, y));
                }
            }
        }

        res
    }

    fn print(&self) {
        println!("Scaffold is {} x {}", self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", char::from(self.get(x, y)));
            }
            println!();
        }
    }

    fn find_robot(&self) -> (Direction, usize, usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Tile::Robot(d) = self.get(x, y) {
                    return (d, x, y);
                }
            }
        }
        panic!("No robot found!!")
    }

    fn go(&self, dir: Direction, x: usize, y: usize) -> Option<(usize, usize)> {
        match dir {
            Direction::Right if x < (self.width - 1) =>
                Some((x + 1, y)),
            Direction::Left  if x > 0 =>
                Some((x - 1, y)),
            Direction::Up    if y > 0 =>
                Some((x, y - 1)),
            Direction::Down  if y < (self.height - 1) =>
                Some((x, y + 1)),
            _ =>
                None
        }
    }

    fn next_directions(&self, x: usize, y: usize, pointing: Direction) -> Option<Direction> {
        if (pointing != Direction::Down) && y > 0 && self.get(x, y - 1) == Tile::Scaffold {
            return Some(Direction::Up);
        }

        if (pointing != Direction::Left) && x < (self.width - 1) && self.get(x + 1, y) == Tile::Scaffold {
            return Some(Direction::Right);
        }

        if (pointing != Direction::Up) && y < (self.height - 1) && self.get(x, y + 1) == Tile::Scaffold {
            return Some(Direction::Down);
        }

        if (pointing != Direction::Right) && x > 0 && self.get(x - 1, y) == Tile::Scaffold {
            return Some(Direction::Left);
        }

        None
    }

    fn num_moves(&self, mut x: usize, mut y: usize, dir: Direction) -> usize {
        let mut count = 0;

        while let Some((newx, newy)) = self.go(dir, x, y) {
            if self.get(newx, newy) != Tile::Scaffold {
                break;
            } else {
                count += 1;
                x = newx;
                y = newy;
            }
        }

        count
    }

    fn trace_path(&self) -> Vec<Move> {
        let mut res = vec![];
        let (mut dir, mut x, mut y) = self.find_robot();

        while let Some(next_dir) = self.next_directions(x, y, dir) {
            println!("Now at ({}, {})", x, y);
            println!("next_dir: {:?}", next_dir);
            res.extend(dir.moves_to(next_dir));
            let moves = self.num_moves(x, y, next_dir);
            res.push(Move::Forward(moves));

            dir = next_dir;
            match dir {
                Direction::Up    => { x = x;         y = y - moves; }
                Direction::Down  => { x = x;         y = y + moves; }
                Direction::Left  => { x = x - moves; y = y;         }
                Direction::Right => { x = x + moves; y = y;         }
            }
            assert!(x < self.width);
            assert!(y < self.height);
        }

        res
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Move {
    Right,
    Left,
    Forward(usize),
}

impl Move {
    fn encode(&self) -> String {
        match self {
            Move::Right      => "R".to_string(),
            Move::Left       => "L".to_string(),
            Move::Forward(s) => s.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Path {
    moves: Vec<Move>
}

impl Path {
    fn new(moves: &[Move]) -> Option<Path> {
        let res = Path{ moves: Vec::from(moves) };

        if res.encode().len() > 20 {
            return None
        }

        Some(res)
    }

    fn encode(&self) -> String {
        let mut res = String::new();

        for mv in self.moves.iter() {
            if res.len() != 0 {
                res.push(',');
            }
            res.push_str(&mv.encode());
        }

        res
    }
}

fn generate_subparts(paths: &[Move]) -> Vec<Path> {
    let mut res = vec![];

    for i in 2..paths.len() {
        for piece in paths.windows(i) {
            if let Some(good) = Path::new(piece) {
                if !res.contains(&good) {
                    res.push(good);
                }
            }
        }
    }

    res
}

struct TripleIterator {
    i: usize,
    j: usize,
    k: usize,
    paths: Vec<Path>,
}

impl TripleIterator {
    fn new(paths: &[Path]) -> TripleIterator {
        TripleIterator {
            i: 0,
            j: 0,
            k: 0,
            paths: Vec::from(paths),
        }
    }
}

impl Iterator for TripleIterator {
    type Item = (Path, Path, Path);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.paths.len() {
            self.i = 0;
            self.j += 1;
        }

        if self.j == self.paths.len() {
            self.j = 0;
            self.k += 1;
        }

        if self.k >= self.paths.len() {
            return None;
        }

        let res = (self.paths[self.i].clone(),
                   self.paths[self.j].clone(),
                   self.paths[self.k].clone());
        self.i += 1;

        Some(res)
    }
}

#[test]
fn day17a() {
    let scaffold = ScaffoldMap::new("inputs/day17");
    scaffold.print();
    let joins = scaffold.join_points();
    println!("joins: {:?}", joins);
    let target: usize = 5620;
    assert_eq!(target, joins.iter().map(|(x, y)| x * y).sum());
}

#[derive(Debug)]
struct Answer {
    main: Vec<Trigger>,
    a: Path,
    b: Path,
    c: Path,
}

trait ToInput {
    fn to_input(&self) -> Vec<i64>;
}

#[derive(Debug)]
enum Trigger{ A, B, C }

impl ToInput for Trigger {
    fn to_input(&self) -> Vec<i64> {
        match self {
            Trigger::A => vec![65],
            Trigger::B => vec![66],
            Trigger::C => vec![67],
        }
    }
}

impl ToInput for Move {
    fn to_input(&self) -> Vec<i64> {
        match self {
            Move::Forward(x) => x.to_string().chars().map(|x| x as u8 as i64).collect(),
            Move::Left       => vec![76],
            Move::Right      => vec![82],
        }
    }
}

fn to_inputs<T: ToInput>(v: &[T]) -> Vec<i64> {
    let mut res = vec![];
    let mut viter = v.iter().peekable();

    while let Some(x) = viter.next() {
        res.extend(x.to_input());
        if viter.peek().is_some() {
            res.push(44);
        } else {
            res.push(10);
        }
    }

    res
}

impl Answer {
    fn to_path(&self) -> Vec<Move> {
        let mut res = vec![];

        for t in self.main.iter() {
            match t {
                Trigger::A => res.extend(&self.a.moves),
                Trigger::B => res.extend(&self.b.moves),
                Trigger::C => res.extend(&self.c.moves),
            }
        }

        res
    }

    fn to_inputs(&self) -> Vec<i64> {
        let mut res = vec![];

        println!("main: {:?}", self.main);
        println!("main': {:?}", to_inputs(&self.main));
        res.extend(to_inputs(&self.main));
        println!("a: {:?}", self.a);
        println!("a': {:?}", to_inputs(&self.a.moves));
        res.extend(to_inputs(&self.a.moves));
        println!("b: {:?}", self.b);
        println!("b': {:?}", to_inputs(&self.b.moves));
        res.extend(to_inputs(&self.b.moves));
        println!("c: {:?}", self.c);
        println!("c': {:?}", to_inputs(&self.c.moves));
        res.extend(to_inputs(&self.c.moves));

        res
    }
}

fn answers(full: &[Move], a: &Path, b: &Path, c: &Path) -> Vec<Answer> {
    let mut res = vec![];

    if full.len() == 0 {
        return vec![Answer{
            main: vec![],
            a: a.clone(),
            b: b.clone(),
            c: c.clone()
        }]
    }

    if full.starts_with(&a.moves) {
        for mut rest in answers(&full[a.moves.len()..], a, b, c).drain(0..) {
            rest.main.insert(0, Trigger::A);
            if rest.main.len() <= 20 {
                res.push(rest);
            }
        }
    }

    if full.starts_with(&b.moves) {
        for mut rest in answers(&full[b.moves.len()..], a, b, c).drain(0..) {
            rest.main.insert(0, Trigger::B);
            if rest.main.len() <= 20 {
                res.push(rest);
            }
        }
    }

    if full.starts_with(&c.moves) {
        for mut rest in answers(&full[c.moves.len()..], a, b, c).drain(0..) {
            rest.main.insert(0, Trigger::C);
            if rest.main.len() <= 20 {
                res.push(rest);
            }
        }
    }

    res
}

#[test]
fn day17b() {
    let scaffold = ScaffoldMap::new("inputs/day17");
    scaffold.print();
    let path = scaffold.trace_path();
    println!("path: {:?}", path);
    let parts = generate_subparts(&path);
    println!("# of possible parts: {}", parts.len());
    let triples = TripleIterator::new(&parts);
    for (a, b, c) in triples {
        for answer in answers(&path, &a, &b, &c).iter() {
            assert_eq!(answer.to_path(), path);
            let mut comp = Computer::load("inputs/day17");
            let mut inputs = answer.to_inputs();

            assert_eq!(comp.read(0), 1);
            comp.write(0, 2);
            inputs.push('n' as u8 as i64);
            inputs.push('\n' as u8 as i64);
            let results = comp.standard_run(&inputs);
            for x in results.iter() {
                if *x < 256 {
                    print!("{}", *x as u8 as char);
                } else {
                    assert_eq!(768115, *x);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Empty,
    Scaffold,
    Robot(Direction),
    Tumbler,
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '#' => Tile::Scaffold,
            '.' => Tile::Empty,
            '^' => Tile::Robot(Direction::Up),
            '>' => Tile::Robot(Direction::Right),
            'v' => Tile::Robot(Direction::Down),
            '<' => Tile::Robot(Direction::Left),
            'X' => Tile::Tumbler,
            _   => panic!("Unknown tile: {}", c),
        }
    }
}

impl From<Tile> for char {
    fn from(t: Tile) -> char {
        match t {
            Tile::Empty => '.',
            Tile::Scaffold => '#',
            Tile::Robot(Direction::Up) => '^',
            Tile::Robot(Direction::Right) => '>',
            Tile::Robot(Direction::Down) => 'v',
            Tile::Robot(Direction::Left) => '<',
            Tile::Tumbler => 'X',
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn moves_to(&self, newd: Direction) -> Vec<Move> {
        match (self, newd) {
            (Direction::Up   , Direction::Up   ) => vec![],
            (Direction::Up   , Direction::Down ) => vec![Move::Right, Move::Right],
            (Direction::Up   , Direction::Left ) => vec![Move::Left],
            (Direction::Up   , Direction::Right) => vec![Move::Right],
            (Direction::Down , Direction::Up   ) => vec![Move::Right, Move::Right],
            (Direction::Down , Direction::Down ) => vec![],
            (Direction::Down , Direction::Left ) => vec![Move::Right],
            (Direction::Down , Direction::Right) => vec![Move::Left],
            (Direction::Left , Direction::Up   ) => vec![Move::Right],
            (Direction::Left , Direction::Down ) => vec![Move::Left],
            (Direction::Left , Direction::Left ) => vec![],
            (Direction::Left , Direction::Right) => vec![Move::Right, Move::Right],
            (Direction::Right, Direction::Up   ) => vec![Move::Left],
            (Direction::Right, Direction::Down ) => vec![Move::Right],
            (Direction::Right, Direction::Left ) => vec![Move::Right, Move::Right],
            (Direction::Right, Direction::Right) => vec![],
        }
    }
}