use std::collections::HashMap;
use std::fs;
use std::str;

#[derive(Clone, Debug, PartialEq)]
struct Reaction<'a> {
    amt: u64,
    produces: &'a str,
    consumes: Vec<(u64, &'a str)>,
}

fn parse_compound<'a>(s: &'a str) -> (u64, &'a str) {
    let mut bits = s.split(' ');
    let amtstr = bits.next().expect("Missing amount");
    let kind = bits.next().expect("Missing kind");

    (amtstr.trim().parse().expect("Missed a number?"), kind)
}

impl<'a> Reaction<'a> {
    fn new(s: &'a str) -> Reaction<'a> {
        let mut bits = s.split(" => ");
        let cparts = bits.next().expect("Missing consumption parts");
        let ppart = bits.next().expect("Missing production part");
        let mut consumes = Vec::new();
        let (amt, produces) = parse_compound(ppart);

        for constr in cparts.split(", ") {
            consumes.push(parse_compound(constr));
        }

        Reaction{ amt, produces, consumes }
    }
}

fn apply_reaction<'a>(amt: u64, chemical: &'a str, reactions: &[Reaction<'a>]) -> (Vec<(u64, &'a str)>, u64, &'a str) {
    let mut res = Vec::new();
    let mut extra_amt = 0;

    for reaction in reactions {
        if reaction.produces == chemical {
            let multiplier = if reaction.amt > amt { 1 } else { (amt + (reaction.amt - 1)) / reaction.amt };
            extra_amt = (reaction.amt * multiplier) - amt;
            for (camt, cchem) in reaction.consumes.iter() {
                res.push((*camt * multiplier, *cchem));
            }
        }
    }
    assert_ne!(0, res.len());

    (res, extra_amt, chemical)
}

fn compute_ore<'a>(reactions: &[Reaction<'a>]) -> u64 {
    let mut goal: Vec<(u64, &str)> = vec![(1, "FUEL")];
    let mut extras: HashMap<&str,u64> = HashMap::new();

    while !goal.iter().all(|(_, chem)| chem == &"ORE") {
        let mut new_goal = Vec::new();

        println!("Working on goal: {:?}", goal);
        for (mut amt, item) in goal.iter() {
            if item == &"ORE" {
                new_goal.push((amt, "ORE"));
            } else {
                let extra_amt = extras.get_mut(item);
                println!("I have {:?} extra {}", extra_amt, item);

                if let Some(amt_extra) = extra_amt {
                    if *amt_extra >= amt {
                        *amt_extra = *amt_extra - amt;
                        continue;
                    } else {
                        amt -= *amt_extra;
                        extras.remove(item);
                    }
                }

                let (mut newbits, amt_extra, chem) = apply_reaction(amt, item, reactions);
                let newamt = match extras.get(chem) {
                    None => amt_extra,
                    Some(v) => *v + amt_extra,
                };
                extras.insert(chem, newamt);
                new_goal.append(&mut newbits);
            }
        }
        println!("Converted that to {:?}", new_goal);
        goal = new_goal;
    }

    goal.iter()
        .fold(0, |acc, (x, _)| acc + x)
}

#[test]
fn examples() {
    let rule1 = Reaction::new("10 ORE => 10 A");
    let rule2 = Reaction::new("1 ORE => 1 B");
    let rule3 = Reaction::new("7 A, 1 B => 1 C");
    let rule4 = Reaction::new("7 A, 1 C => 1 D");
    let rule5 = Reaction::new("7 A, 1 D => 1 E");
    let rule6 = Reaction::new("7 A, 1 E => 1 FUEL");

    assert_eq!(rule1, Reaction{ amt: 10, produces: "A", consumes: vec![(10, "ORE")] });
    assert_eq!(rule6, Reaction{ amt: 1,  produces: "FUEL", consumes: vec![(7, "A"), (1, "E")]});
    assert_eq!(31, compute_ore(&[rule1, rule2, rule3, rule4, rule5, rule6]));

    let rules2 = [Reaction::new("9 ORE => 2 A"),
                  Reaction::new("8 ORE => 3 B"),
                  Reaction::new("7 ORE => 5 C"),
                  Reaction::new("3 A, 4 B => 1 AB"),
                  Reaction::new("5 B, 7 C => 1 BC"),
                  Reaction::new("4 C, 1 A => 1 CA"),
                  Reaction::new("2 AB, 3 BC, 4 CA => 1 FUEL")];
    assert_eq!(165, compute_ore(&rules2));

    let rules3 = [Reaction::new("157 ORE => 5 NZVS"),
                  Reaction::new("165 ORE => 6 DCFZ"),
                  Reaction::new("44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL"),
                  Reaction::new("12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ"),
                  Reaction::new("179 ORE => 7 PSHF"),
                  Reaction::new("177 ORE => 5 HKGWZ"),
                  Reaction::new("7 DCFZ, 7 PSHF => 2 XJWVT"),
                  Reaction::new("165 ORE => 2 GPVTF"),
                  Reaction::new("3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT")];
    assert_eq!(13312, compute_ore(&rules3));

    let rules4 = [Reaction::new("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG"),
                  Reaction::new("17 NVRVD, 3 JNWZP => 8 VPVL"),
                  Reaction::new("53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL"),
                  Reaction::new("22 VJHF, 37 MNCFX => 5 FWMGM"),
                  Reaction::new("139 ORE => 4 NVRVD"),
                  Reaction::new("144 ORE => 7 JNWZP"),
                  Reaction::new("5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC"),
                  Reaction::new("5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV"),
                  Reaction::new("145 ORE => 6 MNCFX"),
                  Reaction::new("1 NVRVD => 8 CXFTF"),
                  Reaction::new("1 VJHF, 6 MNCFX => 4 RFSQX"),
                  Reaction::new("176 ORE => 6 VJHF")];
    assert_eq!(180697, compute_ore(&rules4));

    let rules5 = [Reaction::new("171 ORE => 8 CNZTR"),
                  Reaction::new("7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL"),
                  Reaction::new("114 ORE => 4 BHXH"),
                  Reaction::new("14 VRPVC => 6 BMBT"),
                  Reaction::new("6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL"),
                  Reaction::new("6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT"),
                  Reaction::new("15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW"),
                  Reaction::new("13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW"),
                  Reaction::new("5 BMBT => 4 WPTQ"),
                  Reaction::new("189 ORE => 9 KTJDG"),
                  Reaction::new("1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP"),
                  Reaction::new("12 VRPVC, 27 CNZTR => 2 XDBXC"),
                  Reaction::new("15 KTJDG, 12 BHXH => 5 XCVML"),
                  Reaction::new("3 BHXH, 2 VRPVC => 7 MZWV"),
                  Reaction::new("121 ORE => 7 VRPVC"),
                  Reaction::new("7 XCVML => 6 RJRHP"),
                  Reaction::new("5 BHXH, 4 VRPVC => 5 LTCX")];
    assert_eq!(2210736, compute_ore(&rules5));
}

#[test]
fn day14() {
    let day14_contents = fs::read("inputs/day14").expect("Couldn't open day14 problem");
    let day14_str = str::from_utf8(&day14_contents).expect("Couldn't decode day14 problem");
    let reactions: Vec<Reaction> = day14_str.trim().split('\n').map(Reaction::new).collect();
    assert_eq!(319014, compute_ore(&reactions));
}