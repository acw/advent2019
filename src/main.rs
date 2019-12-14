use clap::{App,Arg,SubCommand};

fn is_number(s: String) -> Result<(), String> {
    match u64::from_str_radix(&s, 10) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
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
                                             .validator(is_number)))
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

    println!("Failed to run a reasonable command.");
}
