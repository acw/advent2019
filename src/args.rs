use clap::{App,Arg,SubCommand};
use crate::machine::Computer;
use std::fs;

pub enum Command {
    ComputeFuel(Vec<u64>),
    RunComputer(Computer),
}

fn is_number(s: String) -> Result<(), String> {
    match u64::from_str_radix(&s, 10) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

fn is_file(s: String) -> Result<(), String> {
    match fs::metadata(&s) {
        Err(e) => Err(e.to_string()),
        Ok(md) if md.is_file() => Ok(()),
        _      => Err(format!("{} is not a file.", s))
    }
}

impl Command {
    pub fn get() -> Command {
        let matches = App::new("My Advent of Code Thing")
                          .version("1.0")
                          .author("Adam Wick <awick@uhsure.com>")
                          .about("Runs advent of code programs")
                          .subcommand(SubCommand::with_name("fuel")
                                        .about("runs the fuel computation from day1")
                                        .arg(Arg::with_name("NUM")
                                                 .help("The mass of the ship")
                                                 .multiple(true)
                                                 .validator(is_number))
                                        )
                          .subcommand(SubCommand::with_name("compute")
                                        .about("run the given computer")
                                        .arg(Arg::with_name("START_POSITION")
                                                 .short("p")
                                                 .long("start-position")
                                                 .help("The starting position to execute from.")
                                                 .default_value("0")
                                                 .validator(is_number))
                                        .arg(Arg::with_name("COMPUTER")
                                                 .index(1)
                                                 .help("The computer to run.")
                                                 .required(true)
                                                 .validator(is_file))
                                        )
                          .get_matches();
    
        if let Some(problem1) = matches.subcommand_matches("fuel") {
            match problem1.values_of("NUM") {
                None =>
                    println!("ERROR: No values to compute fuel for!"),
                Some(masses) => {
                    let args = masses.map(|x| u64::from_str_radix(x, 10).unwrap()).collect();
                    return Command::ComputeFuel(args);
               }
            }
        }
    
        if let Some(problem2) = matches.subcommand_matches("compute") {
            let start_pos_str = problem2.value_of("START_POSITION").unwrap();
            let start_pos = usize::from_str_radix(&start_pos_str, 10).unwrap();
            let mut computer = Computer::load(problem2.value_of("COMPUTER").unwrap(), start_pos);
            return Command::RunComputer(computer);
        }
    
        panic!("Failed to run a reasonable command.");
    }
}