use crate::machine::Computer;

struct TractorMap {
    base_computer: Computer,
    data: Vec<bool>,
    width: usize,
    affected_size: usize,
}

impl TractorMap {
    fn new(file: &str, side_size: usize) -> TractorMap {
        let     base_computer = Computer::load(file);
        let mut result = Vec::with_capacity(side_size * side_size);
        let mut affected = 0;

        result.resize(side_size * side_size, false);
        for x in 0..side_size {
            for y in 0..side_size {
                let computer = base_computer.clone();
                let results = computer.standard_run(&[x as i64, y as i64]);
                assert_eq!(results.len(), 1);
                match results[0] {
                    0 => {},
                    1 => {
                        affected += 1;
                        let idx = (y * side_size) + x;
                        result[idx] = true;
                    }
                    _ => panic!("Weird value {}", x),
                }
            }
        }

        TractorMap {
            base_computer,
            data: result,
            width: side_size,
            affected_size: affected,
        }
    }

    fn ask(&self, x: usize, y: usize) -> bool {
        let computer = self.base_computer.clone();
        let mut results = computer.standard_run(&[x as i64, y as i64]);
        match results.pop() {
            None => panic!("Uh-oh, computer broken!"),
            Some(0) => false,
            Some(1) => true,
            Some(x) => panic!("Weird value {}", x),
        }
    }

    fn find_block(&self, start: (usize, usize), side_size: usize) -> Option<(usize, usize)> {
        let mut xl = start.0;
        let mut yl = start.1;

        loop {
            let mut reset_x = 0;

            loop {
                if self.ask(xl, yl) {
                    if reset_x == 0 {
                        reset_x = xl;
                    }

                    let xr = xl + side_size - 1;
                    let yr = yl + side_size - 1;

                    if self.ask(xl, yr) && self.ask(xr, yl) && self.ask(xr, yr) {
                        return Some((xl, yl));
                    }

                    xl += 1;
                } else {
                    if reset_x != 0 {
                        break;
                    } else {
                        xl += 1;
                    }
                }
            }

            xl = reset_x;
            yl += 1;
        }
    }

    fn print(&self) {
        for y in 0..self.width {
            for x in 0..self.width {
                // if ((x >= 20) && (x <= 22)) && ((y >= 16) && (y <= 18)) {
                //     assert!(self.data[ (y * self.width) + x ]);
                //     print!("X");
                //     continue;
                // }
                if self.data[ (y * self.width) + x ] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

#[test]
fn day19a() {
    let tmap = TractorMap::new("inputs/day19", 50);
    assert_eq!(223, tmap.affected_size);
    tmap.print();
    assert_eq!(Some((20, 16)), tmap.find_block((0, 6), 3));
}

#[test]
fn day19b() {
    let tmap = TractorMap::new("inputs/day19", 50);
    let (x, y) = tmap.find_block((10, 16), 100).expect("a solution");
    assert_eq!(948, x);
    assert_eq!(761, y);
    assert_eq!(9480761, (x * 10000) + y);
}