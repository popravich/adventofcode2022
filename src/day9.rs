use anyhow::anyhow as err;
use std::str;
use std::f64::consts;
use std::collections::HashSet;

pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut head = Knot::default();
    let mut tail = Knot::default();
    visited.insert(tail.pos);
    let input = data.lines()
        .map(|line| {
                line.parse::<Dt>()
                .map(|Dt(dir, time)| (0..time).map(move |_| dir))
                .expect("invalid row")  // ignore error and unwrap
        })
        .flatten();
    for direction in input {
        head.step(direction);
        visited.insert(tail.move_towards(&head));
    }

    Ok((visited.len(), 0))
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Position{
    x: i64,
    y: i64,
}

#[derive(Debug, Default)]
struct Knot {
    pos: Position,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Dt(Direction, u8);

impl str::FromStr for Dt {
    type Err = anyhow::Error;
    fn from_str(val: &str) -> anyhow::Result<Self> {
        use Direction::*;
        let (d, t) = val.split_once(' ').ok_or_else(|| err!("no space in line {}", val))?;
        let d = match d {
            "U" => Up,
            "R" => Right,
            "D" => Down,
            "L" => Left,
            x => return Err(err!("invalid direction {}", x))
        };
        let t = t.parse()?;
        Ok(Dt(d, t))
    }
}

impl Knot {
    fn step(&mut self, dir: Direction) {
        self.pos.step(dir)
    }

    fn move_towards(&mut self, other: &Self) -> Position {
        self.pos.move_to(&other.pos).for_each(|dir| self.step(dir));
        self.pos
    }
}

impl Position {
    fn move_to(&self, other: &Self) -> DirectionIterator {
        DirectionIterator { x0: self.x, y0: self.y, x1: other.x, y1: other.y }
    }

    fn step(&mut self, dir: Direction) {
        use Direction::*;
        match dir {
            Up => self.y += 1,
            Right => self.x += 1,
            Down => self.y -= 1,
            Left => self.x -= 1,
        }
    }
}

struct DirectionIterator {
    x0: i64,
    y0: i64,
    x1: i64,
    y1: i64,
}
impl Iterator for DirectionIterator {
    type Item = Direction;
    fn next(&mut self) -> Option<Self::Item> {
        let dx = self.x1 - self.x0;
        let dy = self.y1 - self.y0;
        if ((dy * dy + dx * dx) as f64).sqrt() <= consts::SQRT_2 {
            None
        } else if dx.abs() > dy.abs() {
            self.move_dy(dy).or_else(|| self.move_dx(dx))
        } else if dy.abs() > dx.abs() {
            self.move_dx(dx).or_else(|| self.move_dy(dy))
        } else {
            unreachable!("dx == dx: {} {}", dx, dy);
        }
    }
}
impl DirectionIterator {
    fn move_dx(&mut self, dx: i64) -> Option<Direction> {
        match dx {
            0 => None,
            x if x > 0 => {
                self.x0 += 1;
                Some(Direction::Right)
            }
            _ => {
                self.x0 -= 1;
                Some(Direction::Left)
            }
        }
    }
    fn move_dy(&mut self, dy: i64) -> Option<Direction> {
        match dy {
            0 => None,
            y if y > 0 => {
                self.y0 += 1;
                Some(Direction::Up)
            }
            _ => {
                self.y0 -= 1;
                Some(Direction::Down)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::main;

    static DATA: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    #[test]
    fn solution() {
        let res = main(DATA).expect("invalid input");
        assert_eq!(res, (13, 0));
    }
}
