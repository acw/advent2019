mod args;
mod fuel;
mod machine;

use crate::args::Command;
use crate::fuel::calculate_fuel;
use crate::machine::{Computer};
use std::fs;

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
    }
 }
