use crate::machine::{Computer, RunResult};

const GATHER_STEPS: [&'static str; 34] = [
    "south",
    "take monolith",
    "east",
    "take asterisk",
    "west",
    "north",
    "west",
    "take coin",
    "north",
    "east",
    "take astronaut ice cream",
    "west",
    "south",
    "east",
    "north",
    "north",
    "take mutex",
    "west",
    "take astrolabe",
    "west",
    "take dehydrated water",
    "west",
    "take wreath",
    "east",
    "south",
    "east",
    "north",
    "drop astronaut ice cream",
    "drop wreath",
    "drop coin",
    "drop dehydrated water",
    "drop asterisk",
    "drop astrolabe",
    "drop mutex",
];

const THINGS: [&'static str; 8] = [
    "astronaut ice cream",
    "wreath",
    "coin",
    "dehydrated water",
    "asterisk",
    "astrolabe",
    "mutex",
    "monolith",
];

fn combine_commands(cmds: &[&str]) -> String {
    let mut res = String::new();

    for cmd in cmds.iter() {
        res.push_str(cmd);
        res.push_str("\n");
    }

    res
}

fn select_things(c: u16) -> String {
    let mut res = String::new();

    for bit in 0..8 {
        if (c >> bit) & 0x1 == 1 {
            res.push_str("take ");
            res.push_str(THINGS[bit]);
            res.push_str("\n");
        }
    }

    res
}

fn run_computer(mut comp: Computer, buffer: &mut String) -> (Box<dyn FnOnce(i64) -> Computer>, String) {
    let mut outbuf = String::new();

    loop {
        match comp.run() {
            RunResult::Continue(comp2) => comp = comp2,
            RunResult::Halted(_) => panic!("Machine halted in run_computer: {}", outbuf),
            RunResult::Output(c, comp2) => {
                outbuf.push(c as u8 as char);
                comp = comp2;
            }
            RunResult::Input(f) => {
                if buffer.len() == 0 {
                    return (f, outbuf);
                }

                let c = buffer.remove(0);
                comp = f(c as u8 as i64);
            } 
        }
    }
}

fn gather_everything(comp: Computer) -> Computer {
    let mut gather_buffer = combine_commands(&GATHER_STEPS);
    let (res, outb) = run_computer(comp, &mut gather_buffer);
    println!("{}", outb);
    res('\n' as u8 as i64)
}

fn combination_works(comp: Computer, code: u16) -> bool {
    let mut get_buffer = select_things(code);
    let (next, _) = run_computer(comp, &mut get_buffer);
    let mut north = "nv\nnorth\n".to_string();
    let (_after, outbuf) = run_computer(next('i' as u8 as i64), &mut north);
    println!("outbuf: {}", outbuf);
    !outbuf.contains("heavier") && !outbuf.contains("lighter")
}

pub fn find_santa(base_computer: Computer) {
    let at_checkpoint = gather_everything(base_computer);

    for code in 1..256 {
        println!("------------------------------------------");
        println!("Trying code: {}", code);
        if combination_works(at_checkpoint.clone(), code) {
            println!("Combination {} worked.", code);
            break;
        }
    }
}