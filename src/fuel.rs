fn calculate_base_fuel(mass: u64) -> u64 {
    let div3 = mass / 3;

    if div3 >= 2 {
        div3 - 2
    } else {
        0
    }
}

pub fn calculate_fuel(mass: u64) -> u64 {
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

