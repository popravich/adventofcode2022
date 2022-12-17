use std::str;
use std::collections::{HashSet, HashMap, VecDeque};
use std::io;
use std::{thread, time};

use anyhow::anyhow as err;
use crate::graph;

pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {
    let map: Map = data.parse()?;

    let t0 = time::Instant::now();
    bms(&map, map.start(), map.end(), false)
        .ok_or_else(|| err!("could not find path"))?;
    let dt1 = t0.elapsed();
    let t0 = time::Instant::now();
    build_path(&map, map.start(), map.end(), false)
        .ok_or_else(|| err!("could not find path"))?;
    let dt2 = t0.elapsed();

    draw_map(&map)?;

    let path = bms(&map, map.start(), map.end(), true)
    // let path = build_path(&map, map.start(), map.end(), false)
        .ok_or_else(|| err!("could not find path"))?;

    let result1 = rebuild_path(path, map.end());
    draw_path(&map, &result1, graph::COLOR_RED, false)?;

    thread::sleep(time::Duration::from_secs(5));

    draw_map(&map)?;
    let path = build_path(&map, map.start(), map.end(), true)
        .ok_or_else(|| err!("could not find path"))?;

    let result = rebuild_path(path, map.end());
    draw_path(&map, &result, graph::COLOR_GREEN, false)?;

    thread::sleep(time::Duration::from_secs(2));

    draw_map(&map)?;
    draw_path(&map, &result1, graph::COLOR_RED, true)?;
    draw_path(&map, &result, graph::COLOR_GREEN, true)?;
    println!("\x1b[{};1HDone", map.height + 1);

    println!("Bms: RED:   took: {:?}", dt1);
    println!("A*:  GREEN: took: {:?}", dt2);

    let starts: Vec<_> = (0..map.height)
        .into_iter()
        .flat_map(|y| (0..map.width).into_iter().map(move |x| Point { x, y }))
        .filter(|p| map.h(p) == 0)
        .collect();

    let mut min = usize::MAX;
    for start in starts.into_iter() {
        if let Some(path) = bms(&map, start, map.end(), false) {
            let path = rebuild_path(path, map.end());
            let steps = path.len();
            if steps < min {
                min = steps;
            }
        }
    }

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

fn bms(map: &Map, start: Point, goal: Point, draw: bool) -> Option<HashMap<Point, Point>> {
    let mut result = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut seen = HashSet::new();
    seen.insert(start);
    while let Some(p) = queue.pop_front() {
        if p == goal {
            return Some(result)
        }
        for n in p.allowed_neigbours(map) {
            if seen.contains(&n) {
                continue
            }
            seen.insert(n);
            result.insert(n, p);
            queue.push_back(n);
            if draw {
                graph::delay_draw_char(
                    n.x, n.y, ((map.h(&n) + b'a') as char, graph::HIGHLIGHT)).ok();
            }
        }
    }
    None
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

fn build_path(map: &Map, start: Point, goal: Point, draw: bool) -> Option<HashMap<Point, Point>> {
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
        for n in current.allowed_neigbours(map) {
            let score = g_scores
                .get(&current)
                .map(|s| s + 1)
                .expect("current has score");
            if score < *g_scores.get(&n).unwrap_or(&isize::MAX) {
                path.insert(n, current.clone());
                g_scores.insert(n, score);
                f_scores.insert(n, score); // + h_score(&n, &goal));
                todo.insert(n);
                if draw {
                    graph::delay_draw_char(
                        n.x, n.y, ((map.h(&n) + b'a') as char, graph::HIGHLIGHT)).ok();
                }
            }
        }
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

// Drawing stuf...

fn draw_map(map: &Map) -> io::Result<()> {
    graph::draw_map(map.width, map.height, |offset| {
        let c = (map.data[offset] + b'a') as char;
        let s = map.start();
        let e = map.end();
        if offset == s.y * map.width + s.x {
            ('S', graph::COLOR_RED)
        } else if offset == e.y * map.width + e.x {
            ('E', graph::COLOR_RED)
        } else {
            (c, graph::SHADE)
        }
    })
}

fn draw_path(map: &Map, path: &[Point], modifier: graph::Modifier, slow: bool) -> io::Result<()> {
    graph::draw_path(map.width, path.iter().map(|p| (p.x, p.y)), move |offset| {
        if slow {
            thread::sleep(time::Duration::from_millis(10));
        }
        let c = (map.data[offset] + b'a') as char;
        (c, modifier, slow)
    })
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
