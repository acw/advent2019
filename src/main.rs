mod arcade;
mod args;
mod fuel;
mod image;
mod machine;
mod orbits;
#[cfg(test)]
mod repair;
#[cfg(test)]
mod robot;
mod scaffold;
mod wiremap;

use crate::args::Command;
use crate::fuel::calculate_fuel;
use crate::orbits::Object;
use crate::wiremap::WireMap;
use std::cmp::{max,min};
use terminal_graphics::Display;

fn main() {
    match Command::get() {
        Command::ComputeFuel(masses) => {
            let mut total = 0;

            for mass in masses {
                let fuel = calculate_fuel(mass);
                println!("Mass {}: {} fuel", mass, fuel);
                total += fuel;
            }

            println!("TOTAL FUEL: {}", total);
        }

        Command::RunComputer(initial) => {
            println!("Initial Computer:");
            initial.show();
            println!("Running, with input 5.");
            let results = initial.standard_run(&[5]);
            for val in results.iter() {
                println!("Received value: {}", val);
            }
        }

        Command::WireMap(wires) => {
            let mut wiremap = WireMap::new();

            for (num, wire) in wires.iter().enumerate() {
                wiremap.add_wire(wire, num);
            }

            let (target, distance) = wiremap.closest_intersection();
            println!("Closest intersection: ({}, {}) [distance {}]",
                     target.0, target.1, distance);

            let mut best_total_steps = usize::max_value();

            for (target_num, join_point) in wiremap.joins().iter().enumerate() {
                let mut total_steps = 0;

                for (num, wire) in wires.iter().enumerate() {
                    let wire_steps = wiremap.steps_to(wire, *join_point);
                    println!("Wire #{} takes {} steps to target #{}.", num, wire_steps, target_num);
                    total_steps += wire_steps;
                }

                if total_steps < best_total_steps {
                    best_total_steps = total_steps;
                }
            }

            println!("Total steps taken: {}", best_total_steps);
        }

        Command::PasswordCrack(start, end) => {
            let mut count = 0;
            let     first = max(start, 100000);
            let     last  = min(end,   999999);

            for cur in first..last {
                let d0 = cur % 10;
                let d1 = (cur / 10) % 10;
                let d2 = (cur / 100) % 10;
                let d3 = (cur / 1000) % 10;
                let d4 = (cur / 10000) % 10;
                let d5 = (cur / 100000) % 10;

                if d5 > d4 { continue; }
                if d4 > d3 { continue; }
                if d3 > d2 { continue; }
                if d2 > d1 { continue; }
                if d1 > d0 { continue; }

                let mut got_double = false;
                if d0 == d1 { got_double |= !(               d1 == d2 ); }
                if d1 == d2 { got_double |= !((d0 == d1) || (d2 == d3)); }
                if d2 == d3 { got_double |= !((d1 == d2) || (d3 == d4)); }
                if d3 == d4 { got_double |= !((d2 == d3) || (d4 == d5)); }
                if d4 == d5 { got_double |= !( d3 == d4               ); }

                if got_double {
                    count += 1;
                    println!("{} is a possibility [total {}]", cur, count);
                }
            }

            // 353 is wrong (low)
            println!("Successful digits: {}", count);
        }

        Command::Orbits(uom) => {
            println!("Got orbits:");
            uom.show();
            println!("Base map has {} orbits.", uom.num_orbits());
            match uom.find_path(&Object::new("YOU"), &Object::new("SAN")) {
                None =>
                    println!("There is no path from you to Santa. :("),
                Some(path) => {
                    print!("The path from you to Santa is: ");
                    let mut path_iter = path.iter().peekable();

                    while let Some(x) = path_iter.next() {
                        print!("{}", x);
                        if path_iter.peek().is_some() {
                            print!(" => ");
                        }
                    }
                    println!("");
                    println!("  ... so the number of transfers needed is {}", path.len() - 3);
                }
            }
        }

        Command::Amplify(computer) => {
            let (amount_a, settings_a) = computer.find_best_signal(0..5, |x| computer.serialize(x));
            println!("Best signal without loopback is {} @ {:?}", amount_a, settings_a);
            let (amount_b, settings_b) = computer.find_best_signal(5..10, |x| computer.amplifier(x));
            println!("Best signal with loopback is {} @ {:?}", amount_b, settings_b);
        }

        Command::Image(image) => {
            let zero_byte_counts = image.digits_per_layer(0);
            let mut lowest_score = usize::max_value();
            let mut lowest_idx   = 0;

            for (idx, layer_count) in zero_byte_counts.iter().enumerate() {
                if layer_count < &lowest_score {
                    lowest_score = *layer_count;
                    lowest_idx = idx;
                }
            }

            println!("Fewest number of zeros at layer {} [{} zeros]", lowest_idx, lowest_score);
            let one_digits = image.digits_for_layer(lowest_idx, 1);
            println!("Layer {} has {} one digits.", lowest_idx, one_digits);
            let two_digits = image.digits_for_layer(lowest_idx, 2);
            println!("Layer {} has {} two digits.", lowest_idx, two_digits);
            println!("Multiplied together is {}", one_digits * two_digits);
            image.draw();
        }

        Command::Arcade(arcade) => {
            let mut screen = Display::new(40, 40);

            screen.clear();
            let result = arcade.run(move |a| {
                a.draw(&mut screen);
                screen.print();
            });
            println!("Final score: {}", result.score);
        }
    }
 }
