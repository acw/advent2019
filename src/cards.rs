use std::fs::read;
use std::str::{FromStr, from_utf8};

enum Shuffle {
    DealNew,
    Deal(usize),
    Cut(usize),
    CutBottom(usize),
}

impl Shuffle {
    fn new(s: &str) -> Shuffle {
        if s.starts_with("deal with increment ") {
            let amt = usize::from_str(&s[19..].trim()).expect("Couldn't parse deal with number");
            return Shuffle::Deal(amt);
        }

        if s.starts_with("cut -") {
            let amt = usize::from_str(&s[5..].trim()).expect("Couldn't parse cut with negative number");
            return Shuffle::CutBottom(amt);
        }

        if s.starts_with("cut ") {
            let amt = usize::from_str(&s[4..].trim()).expect("Couldn't parse cut with number");
            return Shuffle::Cut(amt);
        }

        if s.starts_with("deal into new stack") {
            return Shuffle::DealNew;
        }

        panic!("Couldn't parse shuffle mechanism")
    }

    fn apply(&self, mut deck: Vec<u32>) -> Vec<u32> {
        match self {
            Shuffle::DealNew => {
                deck.reverse(); 
                deck
            }
            Shuffle::Deal(amt) => {
                let mut res = deck.clone();
                let len = deck.len();
                let mut out = 0;

                for i in 0..len {
                    res[out] = deck[i];
                    out = (out + amt) % len;
                }

                res
            }
            Shuffle::Cut(place) => {
                let len = deck.len();
                let mut result = Vec::with_capacity(len);
                let mut i = *place;

                loop {
                    result.push(deck[i]);
                    i = (i + 1) % len;
                    if i == *place {
                        return result;
                    }
                }
            }
            Shuffle::CutBottom(place) => {
                let len = deck.len();
                let mut result = Vec::with_capacity(len);
                let mut i = len - *place;

                loop {
                    result.push(deck[i]);
                    i = (i + 1) % len;
                    if i == (len - *place) {
                        return result;
                    }
                }
            }
        }
    }
}

struct ShuffleOrder {
    order: Vec<Shuffle>
}

impl ShuffleOrder {
    fn from_file(s: &str) -> ShuffleOrder {
        let raw = read(s).expect("Couldn't read file.");
        let strs = from_utf8(&raw).expect("Couldn't get string data");
        ShuffleOrder::new(&strs)
    }

    fn new(strs: &str) -> ShuffleOrder {
        let mut order = Vec::new();

        for line in strs.trim().split('\n') {
            order.push( Shuffle::new(line) );
        }

        ShuffleOrder{ order }
    }

    fn shuffle(&self, initial: &[u32]) -> Vec<u32> {
        let mut cur = Vec::from(initial);

        for shuffle in self.order.iter() {
            cur = shuffle.apply(cur);
        }

        cur
    }
}

#[test]
fn base() {
    let deck1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0], Shuffle::DealNew.apply(deck1));

    let deck2 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2], Shuffle::Cut(3).apply(deck2));

    let deck3 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5], Shuffle::CutBottom(4).apply(deck3));

    let deck4 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3], Shuffle::Deal(3).apply(deck4));
}

#[test]
fn example1() {
    let shuffles = ShuffleOrder::new("deal with increment 7\ndeal into new stack\ndeal into new stack");
    let done = shuffles.shuffle(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7], done);
}

#[test]
fn example2() {
    let shuffles = ShuffleOrder::new("cut 6\ndeal with increment 7\ndeal into new stack");
    let done = shuffles.shuffle(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6], done);

}

#[test]
fn example3() {
    let shuffles = ShuffleOrder::new("deal with increment 7\ndeal with increment 9\ncut -2");
    let done = shuffles.shuffle(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9], done);

}

#[test]
fn example4() {
    let shuffles = ShuffleOrder::new("deal into new stack\ncut -2\ndeal with increment 7\ncut 8\ncut -4\ndeal with increment 7\ncut 3\ndeal with increment 9\ndeal with increment 3\ncut -1");
    let done = shuffles.shuffle(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6], done);
}

fn find_card(deck: &[u32], v: u32) -> usize {
    for (idx, value) in deck.iter().enumerate() {
        if *value == v {
            return idx;
        }
    }
    panic!("Couldn't find card {}", v)
}

#[test]
fn day22a() {
    let shuffles = ShuffleOrder::from_file("inputs/day22");
    let deck: Vec<u32> = (0..10007).collect();
    let result = shuffles.shuffle(&deck);
    assert_eq!(2939, find_card(&result, 2019));
}