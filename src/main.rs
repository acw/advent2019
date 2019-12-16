mod args;
mod fuel;
mod machine;
mod wiremap;

use crate::args::Command;
use crate::fuel::calculate_fuel;
use crate::wiremap::WireMap;
use std::cmp::{max,min};
use std::sync::mpsc::channel;

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

        Command::RunComputer(mut initial) => {
            let (my_sender,    their_receiver) = channel();
            let (their_sender, my_receiver)    = channel();
            println!("Initial Computer:");
            initial.show();
            println!("Running, with input 1.");
            my_sender.send(1).expect("Initial send failed");
            initial.run(&their_receiver, &their_sender);
            for val in my_receiver.iter() {
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
    }
 }
