use std::str;
use anyhow::anyhow as err;

pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {
    let (map, commands) = data.split_once("\n\n").ok_or_else(|| err!("bad input"))?;

    let map: Map = map.parse()?;
    let Commands(commands) = commands.parse()?;
    let mut position = map.start();
    println!("Map: {}x{}", map.width, map.height);
    for cmd in commands {
        match cmd {
            Command::Forward(x) => position.forward(&map, x),
            Command::TurnRight => position.turn_right(),
            Command::TurnLeft => position.turn_left(),
        }
    }

    let part1 = position.number();

    Ok((part1, 0))
}

#[derive(Debug, Clone, Copy)]
enum Point {
    Skip,
    Open,
    Wall,
}

#[derive(Debug)]
enum Command {
    Forward(usize),
    TurnRight,
    TurnLeft,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug)]
struct Position {
    row: usize,
    col: usize,
    dir: Direction,
}

#[derive(Debug, Clone)]
struct Map {
    width: usize,
    height: usize,
    data: Vec<Vec<Point>>,
}

impl Map {
    fn start(&self) -> Position {
        let dir = Direction::Right;
        let row = 0;
        let col = self.data[row].iter().position(|p| matches!(p, Point::Open))
            .expect("no open tiles on first row");
        Position { row, col, dir }
    }
    fn iter_direction(&self, x: usize, y: usize, dir: Direction)
        -> Box<dyn Iterator<Item=(&'_ Point, usize, usize)> + '_>
    {
        use Direction::*;
        match dir {
            Right => {
                let row = &self.data[y];
                let n = row.len();
                let row = row[x..].iter().chain(&row[..x]);
                Box::new(
                    row
                    .enumerate()
                    .map(move |(i, p)| (p, (x + i) % n))
                    .cycle()
                    .filter(|(p, _)| !matches!(p, Point::Skip))
                    .map(move |(p, x)| (p, x, y))
                )
            },
            Left => {
                let row = &self.data[y];
                let n = row.len();
                let row = row[x..].iter().chain(&row[..x]);
                Box::new(
                    row
                    .enumerate()
                    .map(move |(i, p)| (p, (x + i) % n))
                    .cycle()
                    .step_by(n - 1) // reverse iterator start at current pos
                    .filter(|(p, _)| !matches!(p, Point::Skip))
                    .map(move |(p, x)| (p, x, y))
                )
            }
            Up => {
                let col = &self.data;
                let col: Vec<_> = col[y..].iter().chain(&col[0..y]).collect();
                let n = self.height;
                Box::new(
                    col
                    .into_iter()
                    .enumerate()
                    .map(move |(i, p)| (p, (y + i) % n))
                    .cycle()
                    .step_by(n - 1)
                    .filter_map(move |(row, y)| row.get(x).map(move |p| (p, x, y)))
                    .filter(|(p, _, _)| !matches!(p, Point::Skip))
                )
            }
            Down => {
                let col = &self.data;
                let col = col[y..].iter().chain(&col[0..y]);
                let n = self.height;
                Box::new(
                    col
                    .enumerate()
                    .map(move |(i, p)| (p, (y + i) % n))
                    .cycle()
                    .filter_map(move |(row, y)| row.get(x).map(move |p| (p, x, y)))
                    .filter(|(p, _, _)| !matches!(p, Point::Skip))
                )
            }
        }
    }
}

impl Position {
    fn forward(&mut self, map: &Map, steps: usize) {
        if let Some((_, col, row)) = map
            .iter_direction(self.col, self.row, self.dir)
            .take_while(|(p, _, _)| !matches!(p, Point::Wall))
            .take(steps + 1)
            .last()
        {
            self.row = row;
            self.col = col;
        }
        //println!("New row/col: {}/{}", row, col);
    }

    fn turn_right(&mut self) {
        use Direction::*;
        self.dir = match self.dir {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
    fn turn_left(&mut self) {
        use Direction::*;
        self.dir = match self.dir {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }

    fn number(&self) -> usize {
        1000 * (self.row + 1) + 4 * (self.col + 1) + (self.dir as usize)
    }
}


// just a wrapper to parse Vec<Command>;
struct Commands(Vec<Command>);

impl str::FromStr for Commands {
    type Err = anyhow::Error;
    fn from_str(val: &str) -> anyhow::Result<Self> {
        let mut out = Vec::new();
        let mut x = None;
        for c in val.trim().chars() {
            match c {
                '0'..='9' => {
                    let i = (c as u8 - b'0') as usize;
                    if let Some(x) = x.as_mut() {
                        *x = *x * 10 + i;
                    } else {
                        x = Some(i);
                    };
                }
                'R' => {
                    match x.take() {
                        Some(0) | None => (),
                        Some(x) => out.push(Command::Forward(x)),
                    }
                    out.push(Command::TurnRight);
                }
                'L' => {
                    match x.take() {
                        Some(0) | None => (),
                        Some(x) => out.push(Command::Forward(x)),
                    }
                    out.push(Command::TurnLeft);
                }
                _ => return Err(err!("unexpected char: {:?}", c))
            }
        }
        if let Some(x) = x.take() {
            out.push(Command::Forward(x));
        }
        Ok(Commands(out))
    }
}

impl str::FromStr for Map {
    type Err = anyhow::Error;
    fn from_str(val: &str) -> anyhow::Result<Self> {
        let height = val.lines().count();
        let width = val.lines().map(|l| l.len()).max().ok_or_else(|| err!("no lines"))?;
        let mut data = Vec::with_capacity(height);
        for line in val.lines() {
            let row: Vec<_> = line.chars().map(|c| match c {
                ' ' => Ok(Point::Skip),
                '#' => Ok(Point::Wall),
                '.' => Ok(Point::Open),
                _ => Err(err!("unexpected char '{:?}'", c)),
            }).collect::<anyhow::Result<Vec<_>>>()?;
            data.push(row);
        }
        Ok(Map { width, height, data })
    }
}

#[cfg(test)]
mod test {
    use super::main;
    use super::*;

    static DATA: &str = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
"#;

    #[test]
    fn solution() {
        let res = main(DATA).expect("invalid input");
        assert_eq!(res, (6032, 0));
    }

    #[test]
    fn map() {
        let map: Map = concat!(
            "  ..\n",
            "....\n",
        ).parse().expect("invalid input");

        let mut start = map.start();
        assert_eq!(start.row, 0);
        assert_eq!(start.col, 2);

        // move up & down
        start.turn_left();  // up
        start.forward(&map, 1);
        assert_eq!(start.row, 1);
        assert_eq!(start.col, 2);
        start.turn_right(); // right
        start.turn_right(); // down
        start.forward(&map, 1);
        assert_eq!(start.row, 0);
        assert_eq!(start.col, 2);

        start.forward(&map, 1);
        assert_eq!(start.row, 1);
        assert_eq!(start.col, 2);

        start.turn_left();
        start.forward(&map, 1);
        assert_eq!(start.row, 1);
        assert_eq!(start.col, 3);

        start.turn_right();
        start.turn_right();
        start.forward(&map, 1);
        assert_eq!(start.row, 1);
        assert_eq!(start.col, 2);

    }
}
