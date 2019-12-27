use std::fmt;
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq)]
struct Body {
    x: i64,
    y: i64,
    z: i64,
    x_vel: i64,
    y_vel: i64,
    z_vel: i64,
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>",
               self.x, self.y, self.z, self.x_vel, self.y_vel, self.z_vel)
    }
}

impl Body {
    fn new(x: i64, y: i64, z: i64) -> Body {
        Body {
            x, y, z,
            x_vel: 0, y_vel: 0, z_vel: 0,
        }
    }

    fn apply_velocity(&mut self) {
        self.x += self.x_vel;
        self.y += self.y_vel;
        self.z += self.z_vel;
    }

    fn apply_gravity(&mut self, others: &[Body]) {
        for other in others.iter() {
            self.x_vel += velocity_adjustment(self.x, other.x);
            self.y_vel += velocity_adjustment(self.y, other.y);
            self.z_vel += velocity_adjustment(self.z, other.z);
        }
    }

    fn potential_energy(&self) -> u64 {
        self.x.abs() as u64 +
        self.y.abs() as u64 +
        self.z.abs() as u64
    }

    fn kinetic_energy(&self) -> u64 {
        self.x_vel.abs() as u64 +
        self.y_vel.abs() as u64 +
        self.z_vel.abs() as u64
    }

    fn total_energy(&self) -> u64 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn time_step(bodies: &mut [Body]) {
    let mut copy_bodies = Vec::new();
    
    copy_bodies.extend(bodies.iter().map(|x| x.clone()));
    for body in bodies.iter_mut() {
        body.apply_gravity(&copy_bodies);
    }
    for body in bodies.iter_mut() {
        body.apply_velocity();
    }
}

fn velocity_adjustment(v1: i64, v2: i64) -> i64 {
    match v1.cmp(&v2) {
        Ordering::Greater => -1,
        Ordering::Equal   => 0,
        Ordering::Less    => 1,
    }
}

#[test]
fn example1() {
    let mut example_bodies = [ Body::new(-1, 0, 2),
                               Body::new(2, -10, -7),
                               Body::new(4, -8, 8),
                               Body::new(3, 5, -1),
                             ];
    time_step(&mut example_bodies);
    assert_eq!(example_bodies, [
        Body{ x: 2, y:-1, z: 1, x_vel: 3, y_vel:-1, z_vel:-1 },
        Body{ x: 3, y:-7, z:-4, x_vel: 1, y_vel: 3, z_vel: 3 },
        Body{ x: 1, y:-7, z: 5, x_vel:-3, y_vel: 1, z_vel:-3 },
        Body{ x: 2, y: 2, z: 0, x_vel:-1, y_vel:-3, z_vel: 1 }, ]);

    time_step(&mut example_bodies);
    assert_eq!(example_bodies, [
        Body{ x: 5, y:-3, z:-1, x_vel: 3, y_vel:-2, z_vel:-2 },
        Body{ x: 1, y:-2, z: 2, x_vel:-2, y_vel: 5, z_vel: 6 },
        Body{ x: 1, y:-4, z:-1, x_vel: 0, y_vel: 3, z_vel:-6 },
        Body{ x: 1, y:-4, z: 2, x_vel:-1, y_vel:-6, z_vel: 2 }, ]);

    time_step(&mut example_bodies);
    assert_eq!(example_bodies, [
        Body{ x: 5, y:-6, z:-1, x_vel: 0, y_vel:-3, z_vel: 0 },
        Body{ x: 0, y: 0, z: 6, x_vel:-1, y_vel: 2, z_vel: 4 },
        Body{ x: 2, y: 1, z:-5, x_vel: 1, y_vel: 5, z_vel:-4 },
        Body{ x: 1, y:-8, z: 2, x_vel: 0, y_vel:-4, z_vel: 0 }, ]);

    time_step(&mut example_bodies);
    assert_eq!(example_bodies, [
        Body{ x: 2, y:-8, z: 0, x_vel:-3, y_vel:-2, z_vel: 1 },
        Body{ x: 2, y: 1, z: 7, x_vel: 2, y_vel: 1, z_vel: 1 },
        Body{ x: 2, y: 3, z:-6, x_vel: 0, y_vel: 2, z_vel:-1 },
        Body{ x: 2, y:-9, z: 1, x_vel: 1, y_vel:-1, z_vel:-1 }, ]);

    time_step(&mut example_bodies);
    assert_eq!(example_bodies, [
        Body{ x:-1, y:-9, z: 2, x_vel:-3, y_vel:-1, z_vel: 2 },
        Body{ x: 4, y: 1, z: 5, x_vel: 2, y_vel: 0, z_vel:-2 },
        Body{ x: 2, y: 2, z:-4, x_vel: 0, y_vel:-1, z_vel: 2 },
        Body{ x: 3, y:-7, z:-1, x_vel: 1, y_vel: 2, z_vel:-2 }, ]);

    time_step(&mut example_bodies);
    assert_eq!(example_bodies, [
        Body{ x:-1, y:-7, z: 3, x_vel: 0, y_vel: 2, z_vel: 1 },
        Body{ x: 3, y: 0, z: 0, x_vel:-1, y_vel:-1, z_vel:-5 },
        Body{ x: 3, y:-2, z: 1, x_vel: 1, y_vel:-4, z_vel: 5 },
        Body{ x: 3, y:-4, z:-2, x_vel: 0, y_vel: 3, z_vel:-1 }, ]);

    time_step(&mut example_bodies);
    assert_eq!(example_bodies, [
        Body{ x: 2, y:-2, z: 1, x_vel: 3, y_vel: 5, z_vel:-2 },
        Body{ x: 1, y:-4, z:-4, x_vel:-2, y_vel:-4, z_vel:-4 },
        Body{ x: 3, y:-7, z: 5, x_vel: 0, y_vel:-5, z_vel: 4 },
        Body{ x: 2, y: 0, z: 0, x_vel:-1, y_vel: 4, z_vel: 2 }, ]);

    time_step(&mut example_bodies);
    assert_eq!(example_bodies, [
        Body{ x: 5, y: 2, z:-2, x_vel: 3, y_vel: 4, z_vel:-3 },
        Body{ x: 2, y:-7, z:-5, x_vel: 1, y_vel:-3, z_vel:-1 },
        Body{ x: 0, y:-9, z: 6, x_vel:-3, y_vel:-2, z_vel: 1 },
        Body{ x: 1, y: 1, z: 3, x_vel:-1, y_vel: 1, z_vel: 3 }, ]);

    time_step(&mut example_bodies);
    assert_eq!(example_bodies, [
        Body{ x: 5, y: 3, z:-4, x_vel: 0, y_vel: 1, z_vel:-2 },
        Body{ x: 2, y:-9, z:-3, x_vel: 0, y_vel:-2, z_vel: 2 },
        Body{ x: 0, y:-8, z: 4, x_vel: 0, y_vel: 1, z_vel:-2 },
        Body{ x: 1, y: 1, z: 5, x_vel: 0, y_vel: 0, z_vel: 2 } ]);

    time_step(&mut example_bodies);
    assert_eq!(example_bodies, [
        Body{ x: 2, y: 1, z:-3, x_vel:-3, y_vel:-2, z_vel: 1 },
        Body{ x: 1, y:-8, z: 0, x_vel:-1, y_vel: 1, z_vel: 3 },
        Body{ x: 3, y:-6, z: 1, x_vel: 3, y_vel: 2, z_vel:-3 },
        Body{ x: 2, y: 0, z: 4, x_vel: 1, y_vel:-1, z_vel:-1 } ]);

    assert_eq!(36, example_bodies[0].total_energy());
    assert_eq!(45, example_bodies[1].total_energy());
    assert_eq!(80, example_bodies[2].total_energy());
    assert_eq!(18, example_bodies[3].total_energy());
}

#[test]
fn example2() {
    let mut example_bodies = [ Body::new(-8, -10, 0),
                               Body::new(5,  5,   10),
                               Body::new(2,  -7,  3),
                               Body::new(9,  -8,  -3), ];

    for _ in 0..100 {
        time_step(&mut example_bodies);
    }

    assert_eq!(1940, example_bodies.iter()
                                   .map(|x| x.total_energy())
                                   .fold(0, |acc,x| acc + x));
}

#[test]
fn day12() {
    let mut example_bodies = [ Body::new(3,  -6, 6),
                               Body::new(10, 7,  -9),
                               Body::new(-3, -7, 9),
                               Body::new(-8, 0,  4) ];

    for _ in 0..1000 {
        time_step(&mut example_bodies);
    }

    assert_eq!(6849, example_bodies.iter()
                                   .map(|x| x.total_energy())
                                   .fold(0, |acc,x| acc + x));
}