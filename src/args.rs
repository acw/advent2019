use clap::{App,Arg,SubCommand};
use crate::machine::Computer;
use crate::wiremap::{Wire};
use std::fs;
use std::iter::FromIterator;
use std::str;
use std::str::FromStr;

pub enum Command {
    ComputeFuel(Vec<u64>),
    RunComputer(Computer),
    WireMap(Vec<Wire>),
    PasswordCrack(u32, u32),
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
                           .subcommand(SubCommand::with_name("wiremap")
                                        .about("compute the given wire map")
                                        .arg(Arg::with_name("MAP")
                                                 .index(1)
                                                 .help("The wiremap to run.")
                                                 .required(true)
                                                 .validator(is_file))
                                        )
                           .subcommand(SubCommand::with_name("crack")
                                        .about("crack a code in the given range")
                                        .arg(Arg::with_name("START")
                                                 .index(1)
                                                 .help("The starting number.")
                                                 .required(true)
                                                 .validator(is_number))
                                        .arg(Arg::with_name("END")
                                                 .index(2)
                                                 .help("The ending number")
                                                 .required(true)
                                                 .validator(is_number))
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
            let computer = Computer::load(problem2.value_of("COMPUTER").unwrap(), start_pos);
            return Command::RunComputer(computer);
        }

        if let Some(problem3) = matches.subcommand_matches("wiremap") {
            let file_contents = fs::read(problem3.value_of("MAP").unwrap()).unwrap();
            let str_contents = str::from_utf8(&file_contents).unwrap();
            let mut contents_iter = str_contents.chars().peekable();
            let mut resvec = Vec::new();

            while contents_iter.peek().is_some() {
                let nextline = contents_iter.by_ref().take_while(|x| *x != '\n');
                let nextstr = String::from_iter(nextline);
                let next = Wire::from_str(&nextstr).unwrap();
                resvec.push(next);
            }

            return Command::WireMap(resvec);
        }

        if let Some(problem4) = matches.subcommand_matches("crack") {
            let start_str = problem4.value_of("START").unwrap();
            let end_str = problem4.value_of("END").unwrap();
            let start = u32::from_str_radix(&start_str, 10).unwrap();
            let end = u32::from_str_radix(&end_str, 10).unwrap();

            return Command::PasswordCrack(start, end);
        }
    
        panic!("Failed to run a reasonable command.");
    }
}