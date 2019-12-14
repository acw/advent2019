use clap::{App,Arg,SubCommand};
use std::fs;
use std::str;

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

fn calculate_base_fuel(mass: u64) -> u64 {
    let div3 = mass / 3;

    if div3 >= 2 {
        div3 - 2
    } else {
        0
    }
}

fn calculate_fuel(mass: u64) -> u64 {
    let mut res = calculate_base_fuel(mass);
    let mut last_round = res;

    loop {
        let new_round = calculate_base_fuel(last_round);
        if new_round == 0 {
            return res;
        } else {
            res += new_round;
            last_round = new_round;
        }
    }
}

struct Computer {
    memory: Vec<u64>,
    position: usize
}

impl Computer {
    fn load(path: &str, position: usize) -> Computer {
        let mut memory = vec![];
        let byte_buffer = fs::read(path).unwrap();
        let char_buffer = str::from_utf8(&byte_buffer).unwrap();

        let mut current = 0;
        for c in char_buffer.chars() {
            match c {
                ',' => {
                    memory.push(current);
                    current = 0;
                }
                _ if c.is_digit(10) => {
                    let val = c.to_digit(10).unwrap() as u64;
                    current = (current * 10) + val;
                }
                _ if c.is_whitespace() => {
                }
                _ => {
                    panic!("Unrecognized character: '{}'", c);
                }
            }
        }
        memory.push(current);

        Computer{ memory, position }
    }

    fn show(&self) {
       for (idx, val) in self.memory.iter().enumerate() {
           println!("{:08}: {}", idx, val);
       }
       println!("POSITION: {}", self.position);
    }
}

fn main() {
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
        let mut total = 0;

        match problem1.values_of("NUM") {
            None =>
                println!("ERROR: No values to compute fuel for!"),
            Some(masses) => {
                for mass_str in masses {
                    let mass = u64::from_str_radix(&mass_str, 10).unwrap();
                    let fuel = calculate_fuel(mass);
                    println!("Mass {}: {} fuel", mass, fuel);
                    total += fuel;
                }

                println!("TOTAL FUEL: {}", total);
                std::process::exit(0);
            }
        }
    }

    if let Some(problem2) = matches.subcommand_matches("compute") {
        let start_pos_str = problem2.value_of("START_POSITION").unwrap();
        let start_pos = usize::from_str_radix(&start_pos_str, 10).unwrap();
        let mut computer = Computer::load(problem2.value_of("COMPUTER").unwrap(), start_pos);
        println!("Initial Computer:");
        computer.show();
    }

    println!("Failed to run a reasonable command.");
}
