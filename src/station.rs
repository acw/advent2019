use std::fs;
use std::str;

struct StationMap {
    data: Vec<bool>,
    width: usize,
    height: usize,
}

impl StationMap {
    fn new(encoding: &str) -> StationMap {
        let mut data = vec![];
        let mut width = 0;
        let mut height = 0;

        for c in encoding.chars() {
            match c {
                '.' => data.push(false),
                '#' => data.push(true),
                '\n' if width == 0 => {
                    width = data.len();
                    height = 1;
                }
                '\n' => height += 1,
                _  => panic!("Unexpected character: {}", c),
            }
        }

        StationMap{ data, width, height }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.data[ (y * self.width) + x ]
    }

    fn count_visible(&self, from_x: usize, from_y: usize) -> usize {
        let mut count = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                if self.get(x, y) && ((x != from_x) || (y != from_y)) {
                    let diff_x = diff(x, from_x);
                    let diff_y = diff(y, from_y);
                    let gcd = gcd(diff_x, diff_y);
                    let rise = diff_y / gcd;
                    let run = diff_x / gcd;

                    let mut curx = from_x;
                    let mut cury = from_y;

                    let shadowed = loop {
                        let (newx, newy) = match (x > from_x, y > from_y) {
                            // we're going down and right
                            (true, true)  => (curx + run, cury + rise),
                            // we're going up and right
                            (true, false) => (curx + run, cury - rise),
                            // we're going down and left
                            (false, true) => (curx - run, cury + rise),
                            // we're going up and left
                            (false, false) => (curx - run, cury - rise),
                        };

                        if newx == x && newy == y {
                            break false;
                        }

                        if self.get(newx, newy) {
                            break true;
                        }

                        curx = newx;
                        cury = newy;
                    };

                    if !shadowed {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn place_station(&self) -> (usize, usize, usize) {
        let mut most_found = 0;
        let mut best_x = 0;
        let mut best_y = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) {
                    let mine = self.count_visible(x, y);
                    if mine > most_found {
                        best_x = x;
                        best_y = y;
                        most_found = mine;
                    }
                }
            }
        }

        (most_found, best_x, best_y)
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        let t = x % y;
        x = y;
        y = t;
    }
    x
}

#[test]
fn day10() {
    let example1 = StationMap::new(".#..#\n.....\n#####\n....#\n...##\n");
    assert_eq!((8, 3, 4), example1.place_station());
    let example2 = StationMap::new("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####\n");
    assert_eq!((33,5,8), example2.place_station());
    let example3 = StationMap::new("#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.\n");
    assert_eq!((35,1,2), example3.place_station());
    let example4 = StationMap::new(".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..\n");
    assert_eq!((41,6,3), example4.place_station());
    let example5 = StationMap::new(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##\n");
    assert_eq!((210,11,13), example5.place_station());
    let day10_contents = fs::read("inputs/day10").expect("Couldn't open day10 problem");
    let day10_str = str::from_utf8(&day10_contents).expect("Couldn't decode day10 problem");
    let day10a = StationMap::new(&day10_str);
    day10a.print();
    assert_eq!((334,23,20), day10a.place_station());
}