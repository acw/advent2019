mod args;
mod fuel;
mod machine;
mod wiremap;

use crate::args::Command;
use crate::fuel::calculate_fuel;
use crate::wiremap::WireMap;

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
            println!("Searching ...");
            for noun in 0..99 {
                for verb in 0..99 {
                    let mut proposed = initial.clone();
                    proposed.write(1, noun);
                    proposed.write(2, verb);
                    proposed.run();
                    if proposed.read(0) == 19690720 {
                        println!("Noun: {}", noun);
                        println!("Verb: {}", verb);
                        println!("ANSWER: {}", 100 * noun + verb);
                        break;
                    }
                }
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
    }
 }
