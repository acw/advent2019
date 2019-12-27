use std::fs;
use std::str;


const BASE_SEQUENCE: [i64; 4] = [0, 1, 0, -1];

struct SequenceIterator {
    i: usize,
    j: usize,
    jmax: usize,
}

impl SequenceIterator {
    fn new(digit: usize) -> SequenceIterator {
        SequenceIterator {
            i: 0,
            j: 0,
            jmax: digit,
        }
    }
}

impl Iterator for SequenceIterator {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        // advance first, because we wanted to skip the first digit.
        self.j += 1;
        if self.j == self.jmax {
            self.j = 0;
            self.i += 1;
            if self.i == BASE_SEQUENCE.len() {
                self.i = 0;
            }
        }

        Some(BASE_SEQUENCE[self.i] as i64)
    }
}

#[test]
fn sequence_check() {
    let seq1 = SequenceIterator::new(1);
    assert_eq!(vec![1, 0, -1, 0, 1, 0, -1, 0], seq1.take(8).collect::<Vec<i64>>());
    let seq2 = SequenceIterator::new(2);
    assert_eq!(vec![0, 1, 1, 0, 0, -1, -1, 0], seq2.take(8).collect::<Vec<i64>>());
    let seq4 = SequenceIterator::new(4);
    assert_eq!(vec![0, 0, 0, 1, 1, 1, 1, 0], seq4.take(8).collect::<Vec<i64>>());
}

struct NumStr {
    vals: Vec<u8>,
}

impl NumStr {
    fn new(s: &str) -> NumStr {
        let mut vals = Vec::with_capacity(s.len());

        for c in s.chars() {
            vals.push(c.to_digit(10).expect("Found non-digit?!") as u8);
        }

        NumStr{ vals }
    }

    fn apply_round(&mut self) {
        let orig = self.vals.clone();
        let mut i = 1;

        while i <= self.vals.len() {
            let mulseq = SequenceIterator::new(i);
            let mut total = 0;

            for (digit, mul) in orig.iter().zip(mulseq) {
                total += (*digit as i64) * mul;
            }

            self.vals[i-1] = (total.abs() % 10) as u8;

            i += 1;
        }
    }

    fn apply_rounds(&mut self, num: usize) {
        for _ in 0..num {
            self.apply_round();
        }
    }

    fn to_string(&self, chars: usize) -> String {
        let mut res = String::new();

        for d in self.vals.iter().take(chars) {
            let c = match d {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                _ => panic!("digit not a digit"),
            };
            res.push(c);
        }

        res
    }
}

#[test]
fn basic_apply() {
    let mut base = NumStr::new("12345678");
    base.apply_round();
    assert_eq!("48226158", &base.to_string(8));
    base.apply_round();
    assert_eq!("34040438", &base.to_string(8));
    base.apply_round();
    assert_eq!("03415518", &base.to_string(8));
    base.apply_round();
    assert_eq!("01029498", &base.to_string(8));
}

#[test]
fn examples() {
    let mut example1 = NumStr::new("80871224585914546619083218645595"); 
    example1.apply_rounds(100);
    assert_eq!("24176176", &example1.to_string(8));
    let mut example2 = NumStr::new("19617804207202209144916044189917"); 
    example2.apply_rounds(100);
    assert_eq!("73745418", &example2.to_string(8));
    let mut example3 = NumStr::new("69317163492948606335995924319873"); 
    example3.apply_rounds(100);
    assert_eq!("52432133", &example3.to_string(8));
}

#[test]
fn day16() {
    let day16_contents = fs::read("inputs/day16").expect("Couldn't open day14 problem");
    let day16_str = str::from_utf8(&day16_contents).expect("Couldn't decode day14 problem");
    let mut day16 = NumStr::new(&day16_str.trim());
    day16.apply_rounds(100);
    assert_eq!("63794407", &day16.to_string(8));
}