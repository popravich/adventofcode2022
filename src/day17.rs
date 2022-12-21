use anyhow::anyhow as err;
use std::time;

const PART1_SHAPES: usize = 2022;
const PART2_SHAPES: usize = 1_000_000_000_000;

pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {
    let moves: Vec<_> = data.trim().chars().map(|c| match c {
        '<' => Ok(Shift::Left),
        '>' => Ok(Shift::Right),
        _ => Err(err!("invalid character {}", c)),
    }).collect::<anyhow::Result<Vec<_>>>()?;

    let shapes = vec![
        Shape::Minus,
        Shape::Plus,
        Shape::LShape,
        Shape::Line,
        Shape::Square,
    ];

    println!("Moves: {}", moves.len());
    println!("Shapes: {}", shapes.len());

    let mut moves_iter = moves.iter().cycle();
    let mut field1 = Field::new();
    for shape in shapes.iter().cycle().take(PART1_SHAPES) {
        loop {
            let shift = moves_iter.next().ok_or_else(|| err!("endless iter"))?;
            field1.shift_to(*shift, shape); 
            if !field1.fall_down(shape) {
                break
            }
        }
        field1.record(shape);
        // println!("{}", PrettyBits(&field.field));
    }
    println!("{}", PrettyBits(&field1.field));

    let mut moves_iter = moves.iter().cycle();
    let mut field2 = Field::new();
    for shape in shapes.iter().cycle().take(PART2_SHAPES) {
        loop {
            let shift = moves_iter.next().ok_or_else(|| err!("endless iter"))?;
            field2.shift_to(*shift, shape); 
            if !field2.fall_down(shape) {
                break
            }
        }
        field2.record(shape);
    }

    Ok((field1.total_height(), field2.height()))
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Minus,
    Plus,
    LShape,
    Line,
    Square,
}

impl Shape {
    fn width(&self) -> usize {
        use Shape::*;
        match self {
            Minus => 4,
            Plus => 3,
            LShape => 3,
            Line => 1,
            Square => 2,
        }
    }
    fn bit_shape(&self, x: usize) -> u32 {
        // shapes are bottom-up 0-byte is bottom n-th byte is top
        use Shape::*;
        let bits = match self {
            Minus => 0b01111000_00000000_00000000_00000000,
            Plus => 0b00100000_01110000_00100000_00000000,
            LShape => 0b01110000_00010000_00010000_00000000,
            Line => 0b01000000_01000000_01000000_01000000,
            Square => 0b01100000_01100000_00000000_00000000,
        };
        bits >> x
    }
}

#[derive(Debug, Clone, Copy)]
enum Shift {
    Left,
    Right,
}

#[derive(Debug)]
struct Field {
    x: usize,
    y: usize,
    total_height: usize,
    field: Vec<u8>,
}

impl Field {
    fn new() -> Field {
        Field {
            x: 2,
            y: 3,
            total_height: 0,
            field: [0].repeat(3 + 4),
        }
    }
    fn height(&self) -> usize {
        self.field.iter().take_while(|&&x| x > 0).count()
    }
    fn total_height(&self) -> usize {
        self.total_height + self.height()
    }
    fn record(&mut self, shape: &Shape) {
        if self.field.len() - self.y < 4 {
            self.field.extend([0u8; 4]);
        }
        let bytes = shape.bit_shape(self.x).to_be_bytes();
        for (a, b) in self.field.iter_mut().skip(self.y).zip(&bytes) {
            *a |= b;
        }
        if let Some(idx) = self.field.iter().position(|&byte| byte == 0b0111_1111_u8) {
            self.total_height += idx + 1;
            self.field.drain(..=idx);
        }
        self.x = 2;
        self.y = self.height() + 3;
    }
    fn shift_to(&mut self, shift: Shift, shape: &Shape) {
        let w = shape.width();
        let x = match shift {
            Shift::Left if self.x == 0 => return,
            Shift::Left => self.x - 1,
            Shift::Right if self.x == 7 - w => return,
            Shift::Right => self.x + 1,
        };
        if collision(self.get_field(self.y), shape.bit_shape(x)) {
            return
        }
        self.x = x;
    }
    fn fall_down(&mut self, shape: &Shape) -> bool {
        let y = match self.y.checked_sub(1) {
            Some(y) if y > self.height() => y,
            Some(y) => {
                let field = self.get_field(y);
                if collision(field, shape.bit_shape(self.x)) {
                    return false
                }
                y
            }
            None => return false
        };
        self.y = y;
        true
    }

    fn get_field(&self, y: usize) -> u32 {
        let mut field = [0u8; 4];
        if y < self.field.len() {
            for (i, byte) in self.field[y..].iter().take(4).enumerate() {
                field[i] = *byte;
            }
        }
        u32::from_be_bytes(field)
    }
}

fn collision(field: u32, shape: u32) -> bool {
    field & shape > 0
}


struct PrettyBits<'a>(&'a [u8]);
use std::fmt;

impl<'a> fmt::Display for PrettyBits<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for word in self.0.iter().rev() {
            let s = format!("|{:07b}|", word).replace("0", ".").replace("1", "#");
            write!(f, "{}\n", s)?;
        }
        write!(f, "'-------'")
    }
}

#[cfg(test)]
mod test {
    use super::main;
    static DATA: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

    #[test]
    fn solution() {
        let res = main(DATA).expect("invalid input");
        assert_eq!(res, (3068, 1514285714288));
    }
}
