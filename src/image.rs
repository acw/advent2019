use bytecount::count;

const WHITE: char = ' ';
const BLACK: char = '\u{2588}';

#[derive(Debug)]
pub enum ImageParseError {
    NotEnoughData,
    IllegalCharacter(char),
}

#[derive(Debug,PartialEq)]
pub struct Image {
    layers: Vec<Layer>
}

impl Image {
    pub fn new(width: usize, height: usize, mut s: &str) -> Result<Image,ImageParseError> {
        let blocksize = height * width;
        let mut layers = vec![];

        s = s.trim();
        loop {
            if s.len() == 0 {
                return Ok(Image{ layers });
            }

            if s.len() < blocksize {
                println!("remaining: |{:?}|", s);
                return Err(ImageParseError::NotEnoughData);
            }

            let (start, end) = s.split_at(blocksize);
            layers.push(Layer::new(width, height, start)?);
            s = end;
        }
    }

    pub fn digits_for_layer(&self, layer: usize, digit: u8) -> usize {
        if layer >= self.layers.len() {
            return 0;
        }

        self.layers[layer].count_digit(digit)
    }

    pub fn digits_per_layer(&self, digit: u8) -> Vec<usize> {
        self.layers.iter().map(|l| l.count_digit(digit)).collect()
    }

    fn get_pixel(&self, x: usize, y: usize) -> u8 {
        for layer in self.layers.iter() {
            match layer.data[(layer.width * y) + x] {
                2 => continue,
                v => return v,
            }
        }
        panic!("Ran to the end of layers on get_pixel({},{})", x, y);
    }

    pub fn draw(&self) {
        assert!(self.layers.len() > 0);
        let width = self.layers[0].width;
        let height = self.layers[0].height;

        for y in 0..height {
            for x in 0..width {
                let c = match self.get_pixel(x, y) {
                    0 => BLACK,
                    1 => WHITE,
                    2 => panic!("Dropped to final transparent?!"),
                    v => panic!("Unexpected pixel value {}", v),
                };

                print!("{}", c);
            }
            println!("");
        }
    }
}

#[derive(Debug,PartialEq)]
struct Layer {
    width:  usize,
    height: usize,
    data:   Vec<u8>
}

impl Layer {
    pub fn new(width: usize, height: usize, s: &str) -> Result<Layer,ImageParseError> {
        let needed_bytes = height * width;
        let mut data = Vec::with_capacity(needed_bytes);

        if s.len() < needed_bytes {
            return Err(ImageParseError::NotEnoughData);
        }

        for c in s.chars().take(needed_bytes) {
            match c.to_digit(10) {
                None    => return Err(ImageParseError::IllegalCharacter(c)),
                Some(x) => data.push(x as u8),
            }
        }

        Ok(Layer{ height, width, data })
    }

    pub fn count_digit(&self, digit: u8) -> usize {
        count(&self.data, digit)
    }
}

#[test]
fn examples() {
    let example1 = "123456789012";
    let target1  = Image{ layers: vec![
                      Layer{ height: 2, width: 3, data: vec![1,2,3,4,5,6] },
                      Layer{ height: 2, width: 3, data: vec![7,8,9,0,1,2] },
                   ]};
    assert_eq!(target1, Image::new(3, 2, &example1).unwrap());
    assert_eq!(1, target1.layers[0].count_digit(1));
    assert_eq!(1, target1.layers[0].count_digit(4));
    assert_eq!(0, target1.layers[0].count_digit(9));
    assert_eq!(1, target1.digits_for_layer(0, 1));
    assert_eq!(vec![1,1], target1.digits_per_layer(1));
    assert_eq!(vec![0,1], target1.digits_per_layer(7));
}
