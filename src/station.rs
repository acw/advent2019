use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::ops::Rem;
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

    fn marvinize(&mut self, from_x: usize, from_y: usize) -> Vec<(usize, usize)> {
        /* step 1, find all the data points by rise/run */
        let mut mapmap = HashMap::new();

        for x in 0..self.width {
            for y in 0..self.height {
                if x == from_x && y == from_y {
                    continue;
                }

                if self.get(x, y) {
                    let deltax = (x as isize) - (from_x as isize);
                    let deltay = (y as isize) - (from_y as isize);
                    let gcd = gcd(deltax.abs(), deltay.abs());
                    let rise = deltay / gcd;
                    let run = deltax / gcd;
                    let key = (rise, run);
                    let deltax2 = (deltax.abs() * deltax.abs()) as f64;
                    let deltay2 = (deltay.abs() * deltay.abs()) as f64;
                    let distance = (deltax2 + deltay2).sqrt();

                    match mapmap.get_mut(&key) {
                        None => {
                            mapmap.insert( (rise, run), vec![(distance, x, y)] );
                        }
                        Some(existing) =>
                            existing.push((distance, x, y)),
                    }
                }
            }
        }

        let mut all_keys: Vec<(isize, isize)> = mapmap.keys().map(|(a,b)| (*a, *b)).collect();
        all_keys.sort_by(compare_slopes);

        let mut res = vec![];
        while !mapmap.is_empty() {
            for key in all_keys.iter() {
                if let Some(curvec) = mapmap.get_mut(&key) {
                    println!("Current slope: {:?}", key);
                    let (x, y) = remove_lowest(curvec);
                    res.push((x, y));
                    if curvec.len() == 0 {
                        mapmap.remove(&key);
                    }
                }
            }
        }

        res
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

fn gcd<T: Copy + Default + PartialEq + Rem<Output=T>>(mut x: T, mut y: T) -> T {
    while y != T::default() {
        let t = x % y;
        x = y;
        y = t;
    }
    x
}

fn quadrant(slope: &(isize, isize)) -> u8 {
    let (rise, run) = slope;

    if run >= &0 && rise <  &0 {
        return 1;
    }

    if run >  &0 && rise >= &0 {
        return 2;
    }

    if run <= &0 && rise >  &0 {
        return 3;
    }

    4
}

fn compare_slopes(a: &(isize, isize), b: &(isize, isize)) -> Ordering {
    let aquad = quadrant(a);
    let bquad = quadrant(b);

    if aquad < bquad {
        return Ordering::Less;
    }

    if aquad > bquad {
        return Ordering::Greater;
    }

    let (arise, arun) = a;
    let (brise, brun) = b;

    match (*arun == 0, *brun == 0) {
        (false, false) => {}
        (false, true)  if [1,3].contains(&aquad) => return Ordering::Greater,
        (false, true)                            => return Ordering::Less,
        (true,  false) if [1,3].contains(&aquad) => return Ordering::Less,
        (true,  false)                           => return Ordering::Greater,
        (true,  true)  => return Ordering::Equal,
    }

    let aslope = arise.abs() as f64 / arun.abs() as f64;
    let bslope = brise.abs() as f64 / brun.abs() as f64;

    if [1,3].contains(&aquad) {
        aslope.partial_cmp(&bslope).unwrap_or(Ordering::Equal).reverse()
    } else {
        aslope.partial_cmp(&bslope).unwrap_or(Ordering::Equal)
    }
}

fn remove_lowest(v: &mut Vec<(f64, usize, usize)>) -> (usize, usize) {
    assert!(v.len() > 0);

    let (dist0, x0, y0) = v[0];
    let mut found_idx = 0;
    let mut lowest = dist0;
    let mut retval = (x0, y0);

    println!("Remove lowest: {:?}", v);
    for (idx, (distance, x, y)) in v.iter().enumerate().skip(1) {
        if distance <= &lowest {
            found_idx = idx;
            lowest = *distance;
            retval = (*x, *y);
        }
    }
    v.remove(found_idx);
    println!("Chose {:?}", retval);

    retval
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
    let mut example6 = StationMap::new(".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....#...###..\n..#.#.....#....##\n");
    let (_, ex6x, ex6y) = example6.place_station();
    assert_eq!((8, 3), (ex6x, ex6y));
    let example6_destroyed = example6.marvinize(8, 3);
    assert_eq!((8,  1), example6_destroyed[0]);
    assert_eq!((9,  0), example6_destroyed[1]);
    assert_eq!((9,  1), example6_destroyed[2]);
    assert_eq!((10, 0), example6_destroyed[3]);
    assert_eq!((9,  2), example6_destroyed[4]);
    assert_eq!((11, 1), example6_destroyed[5]);
    assert_eq!((12, 1), example6_destroyed[6]);
    assert_eq!((11, 2), example6_destroyed[7]);
    assert_eq!((15, 1), example6_destroyed[8]);
    assert_eq!((12, 2), example6_destroyed[9]);
    assert_eq!((13, 2), example6_destroyed[10]);
    assert_eq!((14, 2), example6_destroyed[11]);
    assert_eq!((15, 2), example6_destroyed[12]);
    assert_eq!((12, 3), example6_destroyed[13]);
    assert_eq!((16, 4), example6_destroyed[14]);
    assert_eq!((15, 4), example6_destroyed[15]);
    assert_eq!((10, 4), example6_destroyed[16]);
    assert_eq!((4,  4), example6_destroyed[17]);
    assert_eq!((2,  4), example6_destroyed[18]);
    assert_eq!((2,  3), example6_destroyed[19]);
    assert_eq!((0,  2), example6_destroyed[20]);
    assert_eq!((1,  2), example6_destroyed[21]);
    assert_eq!((0,  1), example6_destroyed[22]);
    assert_eq!((1,  1), example6_destroyed[23]);
    assert_eq!((5,  2), example6_destroyed[24]);
    assert_eq!((1,  0), example6_destroyed[25]);
    assert_eq!((5,  1), example6_destroyed[26]);
    assert_eq!((6,  1), example6_destroyed[27]);
    assert_eq!((6,  0), example6_destroyed[28]);
    assert_eq!((7,  0), example6_destroyed[29]);
    assert_eq!((8,  0), example6_destroyed[30]);
    assert_eq!((10, 1), example6_destroyed[31]);
    assert_eq!((14, 0), example6_destroyed[32]);
    assert_eq!((16, 1), example6_destroyed[33]);
    assert_eq!((13, 3), example6_destroyed[34]);
    assert_eq!((14, 3), example6_destroyed[35]);

    let day10_contents = fs::read("inputs/day10").expect("Couldn't open day10 problem");
    let day10_str = str::from_utf8(&day10_contents).expect("Couldn't decode day10 problem");
    let mut day10 = StationMap::new(&day10_str);
    day10.print();
    let (count, x, y) = day10.place_station();
    assert_eq!((334,23,20), (count, x, y));
    let order = day10.marvinize(x, y);
    assert!(order.len() > 200);
    assert_eq!((11, 19), order[199]);
}