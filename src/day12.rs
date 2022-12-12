use std::str;
use std::collections::{HashSet, HashMap};
use anyhow::anyhow as err;

pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {
    let map: Map = data.parse()?;

    draw_map(&map);

    let path = build_path(&map, map.start(), map.end())
        .ok_or_else(|| err!("could not find path"))?;

    let result = rebuild_path(path, map.end());
    draw_path(&map, &result);
    println!("\x1b[{};1HDone", map.height + 1);

    let starts: Vec<_> = (0..map.height)
        .into_iter()
        .flat_map(|y| (0..map.width).into_iter().map(move |x| Point { x, y }))
        .filter(|p| map.h(p) == 0)
        .collect();
    println!("starting points: {}", starts.len());

    let mut min = usize::MAX;
    for (i, start) in starts.into_iter().enumerate() {
        if let Some(path) = build_path(&map, start, map.end()) {
            let path = rebuild_path(path, map.end());
            let steps = path.len();
            if steps < min {
                min = steps;
                draw_map(&map);
                draw_path(&map, &path);
            }
            println!("\x1b[{};1H Test #{}", map.height + 1, i);
        }
    }
    println!("\x1b[{};1H\nDone", map.height + 1);

    Ok((result.len() - 1, min - 1))
}

fn rebuild_path(pathes: HashMap<Point, Point>, goal: Point) -> Vec<Point> {
    let mut result = Vec::new();
    result.push(goal);
    let mut prev = goal;
    while let Some(&p) = pathes.get(&prev) {
        result.push(p);
        prev = p;
    }
    result
}

// reuse code from day 15 2021
fn pop_lowest(
    set: &HashSet<Point>, scores: &HashMap<Point, isize>,
) -> Point {
    set.iter()
        .min_by_key(|p| scores.get(p).expect("no score"))
        .expect("empty set")
        .clone()
}


fn build_path(map: &Map, start: Point, goal: Point) -> Option<HashMap<Point, Point>> {
    let mut todo = HashSet::new();
    todo.insert(start.clone());

    let mut f_scores = HashMap::new();
    f_scores.insert(start.clone(), 0isize);

    let mut g_scores = HashMap::new();
    g_scores.insert(start.clone(), 0isize);

    let mut path = HashMap::new();
    while !todo.is_empty() {
        let current = pop_lowest(&todo, &f_scores);
        todo.remove(&current);

        if current == goal {
            return Some(path)
        }
        // println!("checking neighbours of {:?}", current);
        // println!("todo: {:?}", todo);
        for n in current.allowed_neigbours(map) {
            // println!("   {:?}", n);
            let score = g_scores
                .get(&current)
                .map(|s| s + 1)
                .expect("current has score");
            // println!("   {:?} -> {}", n, score);
            if score < *g_scores.get(&n).unwrap_or(&isize::MAX) {
                path.insert(n, current.clone());
                g_scores.insert(n, score);
                f_scores.insert(n, score); // + h_score(&n, &goal));
                todo.insert(n);
            }
        }
        
        /*
        for y in 0..map.height {
            for x in 0..map.width {
                let p = Point { x, y };
                let c = (map.data[y * map.width + x] + b'a') as char;
                if path.contains_key(&p) {
                    print!("\x1b[{};{}H\x1b[1m{}\x1b[0m", y+1, x+1, c);
                }
            }
        }
        */
    }
    None
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    data: Vec<u8>,
    start: usize,
    end: usize,
}

impl str::FromStr for Map {
    type Err = anyhow::Error;
    fn from_str(val: &str) -> anyhow::Result<Self> {
        let width = val.find('\n').ok_or_else(|| err!("newline was expected"))?;
        let start = val.chars()
            .filter(|c| *c != '\n')
            .position(|c| c == 'S')
            .ok_or_else(|| err!("start not found"))?;
        let end = val.chars()
            .filter(|c| *c != '\n')
            .position(|c| c == 'E')
            .ok_or_else(|| err!("end not found"))?;
        let data = val.lines()
            .flat_map(|l| l.as_bytes())
            .map(|b| match b {
                b'a'..=b'z' => Ok(b - b'a'),
                b'S' => Ok(0),
                b'E' => Ok(b'z' - b'a'),
                _ => Err(err!("unexpected character")),
            }).collect::<anyhow::Result<Vec<_>>>()?;
        let height = data.len() / width;
        Ok(Map { width, height, data, start, end })
    }
}

impl Map {
    fn start(&self) -> Point {
        let x = self.start % self.width;
        let y = self.start / self.width;
        Point { x, y }
    }
    fn end(&self) -> Point {
        let x = self.end % self.width;
        let y = self.end / self.width;
        Point { x, y }
    }
    fn delta_height(&self, a: &Point, b: &Point) -> isize {
        let s1 = self.data[a.y * self.width + a.x] as isize;
        let s2 = self.data[b.y * self.width + b.x] as isize;
        s1 - s2
    }
    fn h(&self, p: &Point) -> u8 {
        self.data[p.y * self.width + p.x]
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn neighbours(&self, h: usize, w: usize) -> impl Iterator<Item=Point> {
        self.neighbours_opt(h, w).filter_map(|p| p)
    }
    fn neighbours_opt(&self, h: usize, w: usize) -> impl Iterator<Item=Option<Point>> {
        let x = self.x as isize;
        let y = self.y as isize;
        let h = h as isize;
        let w = w as isize;
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .into_iter()
            .map(move |(dx, dy)| (x + dx, y + dy))
            .map(move |(x, y)| {
                (x >= 0 && x < w && y >= 0 && y < h)
                .then(|| Point { x: x as usize, y: y as usize })
            })
    }
    fn allowed_neigbours<'a>(&self, map: &'a Map) -> impl Iterator<Item=Point> + 'a {
        let p1 = self.clone();
        self.neighbours(map.height, map.width)
            .filter(move |p2| map.delta_height(p2, &p1) <= 1)
    }
}

fn draw_map(map: &Map) {
    print!("\x1bc");
    let mut buf = String::with_capacity(map.height * map.width + map.height);
    for y in 0..map.height {
        for x in 0..map.width {
            let p = Point { x, y };
            let c = (map.h(&p) + b'a') as char;
            if map.start() == p {
                buf.push_str("\x1b[31mS\x1b[0m");
            } else if p == map.end() {
                buf.push_str("\x1b[31mE\x1b[0m");
            } else {
                buf.push_str(format!("\x1b[2m{}\x1b[0m", c).as_str());
            }
        }
        buf.push('\n');
    }
    print!("{}", buf);
}
fn draw_path<'a, I: IntoIterator<Item=&'a Point>>(map: &Map, result: I) {
    for p in result {
        let c = (map.h(p) + b'a') as char;
        print!("\x1b[{};{}H\x1b[31m{}\x1b[0m", p.y+1, p.x+1, c);
    }
}

#[cfg(test)]
mod test {
    use super::main;

    static DATA: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    #[test]
    fn solution() {
        let res = main(DATA).expect("invalid input");
        assert_eq!(res, (31, 29));
    }
}
