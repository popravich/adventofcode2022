use anyhow::anyhow as err;
use std::str;
use std::f64::consts;
use std::collections::HashSet;

pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {
    let mut visited_p1 = HashSet::new();
    let mut visited_p2 = HashSet::new();
    let mut head = Knot::default();
    let mut tail_p1 = Knot::default();
    let mut knot_2 = Knot::default();
    let mut knot_3 = Knot::default();
    let mut knot_4 = Knot::default();
    let mut knot_5 = Knot::default();
    let mut knot_6 = Knot::default();
    let mut knot_7 = Knot::default();
    let mut knot_8 = Knot::default();
    let mut tail_p2 = Knot::default();
    visited_p1.insert(tail_p1.pos);
    visited_p2.insert(tail_p2.pos);
    let input = data.lines()
        .map(|line| {
                line.parse::<Dt>()
                .map(|Dt(dir, time)| (0..time).map(move |_| dir))
                .expect("invalid row")  // ignore error and unwrap
        })
        .flatten();
    for direction in input {
        head.step(direction);
        visited_p1.insert(tail_p1.move_towards(&head));
        knot_2.move_towards(&tail_p1);
        knot_3.move_towards(&knot_2);
        knot_4.move_towards(&knot_3);
        knot_5.move_towards(&knot_4);
        knot_6.move_towards(&knot_5);
        knot_7.move_towards(&knot_6);
        knot_8.move_towards(&knot_7);
        visited_p2.insert(tail_p2.move_towards(&knot_8));
    }

    Ok((visited_p1.len(), visited_p2.len()))
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Position{
    x: i64,
    y: i64,
}

#[derive(Debug, Default, Copy, Clone)]
struct Knot {
    pos: Position,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    fn horizontal(dx: i64) -> Option<Self> {
        match dx {
            0 => None,
            i if i > 0 => Some(Direction::Right),
            _ => Some(Direction::Left),
        }
    }
    fn vertical(dy: i64) -> Option<Self> {
        match dy {
            0 => None,
            i if i > 0 => Some(Direction::Up),
            _ => Some(Direction::Down),
        }
    }
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
    fn step(&mut self, dir: Direction) {
        use Direction::*;
        let (x, y) = match dir {
            Up => (0, 1),
            Right => (1, 0),
            Down => (0, -1),
            Left => (-1, 0),
            UpRight => (1, 1),
            UpLeft => (-1, 1),
            DownRight => (1, -1),
            DownLeft => (-1, -1),
        };
        self.x += x;
        self.y += y;
    }

    fn move_to(&self, other: &Self) -> DirectionIterator {
        DirectionIterator { p0: *self, p1: *other }
    }
}

struct DirectionIterator {
    p0: Position,
    p1: Position,
}
impl Iterator for DirectionIterator {
    type Item = Direction;
    fn next(&mut self) -> Option<Self::Item> {
        let dx = self.p1.x - self.p0.x;
        let dy = self.p1.y - self.p0.y;
        if ((dy * dy + dx * dx) as f64).sqrt() <= consts::SQRT_2 {
            None
        } else {
            use Direction::*;
            let dir = match (Direction::horizontal(dx), Direction::vertical(dy)) {
                (None, None) => None,
                (None, Some(x)) => Some(x),
                (Some(x), None) => Some(x),
                (Some(Right), Some(Up)) => Some(UpRight),
                (Some(Left), Some(Up)) => Some(UpLeft),
                (Some(Right), Some(Down)) => Some(DownRight),
                (Some(Left), Some(Down)) => Some(DownLeft),
                _ => unreachable!()
            };
            if let Some(dir) = dir {
                self.p0.step(dir);
            }
            dir
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

    static DATA2: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

    #[test]
    fn solution1() {
        let (part1, _) = main(DATA).expect("invalid input");
        assert_eq!(part1, 13);
    }
    #[test]
    fn solution2() {
        let (_, part2) = main(DATA2).expect("invalid input");
        assert_eq!(part2, 36);
    }
}
