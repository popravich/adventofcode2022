use anyhow::anyhow as err;
use std::convert::TryFrom;
use std::cmp;

use crate::graph;


pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {
    let mut bounding_box: [isize; 4] = [500, 0, 500, 0];
    let mut tmp = Vec::new();
    for line in data.lines() {
        let coords: Vec<_> = line.split(" -> ").collect();
        for start_end in coords.windows(2).map(|slice| (slice[0], slice[1])) {
            let wall: Wall = start_end.try_into()?;
            bounding_box = [
                cmp::min(bounding_box[0], wall.x0),
                cmp::min(bounding_box[1], wall.y0),
                cmp::max(bounding_box[2], wall.x1),
                cmp::max(bounding_box[3], wall.y1),
            ];
            tmp.push(wall);
        }
    }
    bounding_box = [
        bounding_box[0] - 1,
        cmp::max(bounding_box[1] - 1, 0),
        bounding_box[2] + 2,
        bounding_box[3] + 2,
    ];
    let mut bounding_box2 = bounding_box;

    let mut map = Map::new(bounding_box);
    map.extend(&tmp);

    let w = (map.x1 - map.x0) as usize;
    let h = (map.y1 - map.y0) as usize;
    graph::draw_map(w, h, |offset: usize| {
        if map.buf[offset] == 0 {
            ('.', graph::SHADE)
        } else if map.buf[offset] == 1 {
            ('#', graph::COLOR_BLUE)
        } else {
            ('o', graph::HIGHLIGHT)
        }
    })?;
    let mut part1 = 0;
    loop {
        let start = Point { x: 500, y: 0 };
        match map.trace(start) {
            Some(stop) => {
                map.insert(&stop);
                graph::delay_draw_char(
                    (stop.x - map.x0) as usize,
                    stop.y as usize,
                    ('o', graph::HIGHLIGHT),
                )?;
            }
            None => break
        }
        part1 += 1;
    }
    bounding_box2[3] += 1;
    bounding_box2[0] = 500 - bounding_box2[3];
    bounding_box2[2] = 500 + bounding_box2[3];
    let mut map = Map::new(bounding_box2);
    map.extend(&tmp);
    // Add floor
    map.extend(&[Wall {
        x0: bounding_box2[0],
        y0: bounding_box2[3] - 1,
        x1: bounding_box2[2] - 1,
        y1: bounding_box2[3] - 1,
    }]);

    let w = (map.x1 - map.x0) as usize;
    let h = (map.y1 - map.y0) as usize;
    graph::draw_map(w, h, |offset: usize| {
        if map.buf[offset] == 0 {
            ('.', graph::SHADE)
        } else if map.buf[offset] == 1 {
            ('#', graph::COLOR_BLUE)
        } else {
            ('o', graph::HIGHLIGHT)
        }
    })?;
    let mut part2 = 0;
    loop {
        let start = Point { x: 500, y: 0 };
        match map.trace(start) {
            Some(stop) => {
                map.insert(&stop);
                graph::delay_draw_char(
                    (stop.x - map.x0) as usize,
                    stop.y as usize,
                    ( 'o', graph::HIGHLIGHT),
                )?;
            }
            None => break
        }
        part2 += 1;
    }
    graph::goto_line(h)?;
    Ok((part1, part2))
}

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Default)]
struct Map {
    x0: isize,
    y0: isize,
    x1: isize,
    y1: isize,
    buf: Vec<u8>,
}

impl Map {
    fn new(bounding_box: [isize; 4]) -> Self {
        let dx = (bounding_box[2] - bounding_box[0]) as usize;
        let dy = (bounding_box[3] - bounding_box[1]) as usize;
        let buf = [0].repeat(dx * dy);
        Map {
            x0: bounding_box[0],
            y0: bounding_box[1],
            x1: bounding_box[2],
            y1: bounding_box[3],
            buf,
        }
    }
    fn offset(&self, x: isize, y: isize) -> usize {
        let x = x - self.x0;
        let y = y - self.y0;
        let offset = y * (self.x1 - self.x0) + x;
        offset.try_into().expect("positive integer")
    }
    fn extend(&mut self, walls: &[Wall]) {
        for w in walls {
            if w.is_horizontal() {
                for x in w.x0..=w.x1 {
                    let offset = self.offset(x, w.y0);
                    self.buf[offset] = 1;
                }
            } else {
                assert!(w.is_vertical(), "expected vertical: {:?}", w);
                for y in w.y0..=w.y1 {
                    let offset = self.offset(w.x0, y);
                    self.buf[offset] = 1;
                }
            }
        }
    }
    fn insert(&mut self, point: &Point) {
        let offset = self.offset(point.x, point.y);
        self.buf[offset] = 2;
    }
    fn trace(&self, start: Point) -> Option<Point> {
        let mut todo = vec![start];
        loop {
            let p = todo.remove(0);
            let y = (p.y..self.y1)
                .into_iter()
                .take_while(|y| {
                    let offset = self.offset(p.x, *y);
                    self.buf[offset] == 0
                }).last();
            match y {
                None => return None,
                Some(y) if y == self.y1 - 1 => return None,
                Some(y) => {
                    let stop = Point { x: p.x, y };
                    if let Some(left) = self.slide_left(&stop) {
                        todo.push(left);
                    } else if let Some(right) = self.slide_right(&stop) {
                        todo.push(right);
                    } else {
                        return Some(stop)
                    }
                }
            }
        }
    }
    fn slide_left(&self, p: &Point) -> Option<Point> {
        let offset = self.offset(p.x - 1, p.y + 1);
        (self.buf[offset] == 0).then(|| Point {
            x: p.x - 1,
            y: p.y + 1,
        })
    }
    fn slide_right(&self, p: &Point) -> Option<Point> {
        let offset = self.offset(p.x + 1, p.y + 1);
        (self.buf[offset] == 0).then(|| Point {
            x: p.x + 1,
            y: p.y + 1,
        })

    }
}

#[derive(Debug, Clone)]
struct Wall {
    x0: isize,
    y0: isize,
    x1: isize,
    y1: isize,
}

impl Wall {
    fn is_horizontal(&self) -> bool {
        self.y0 == self.y1
    }
    fn is_vertical(&self) -> bool {
        self.x0 == self.x1
    }
}

impl TryFrom<(&str, &str)> for Wall {
    type Error = anyhow::Error;

    fn try_from((start, end): (&str, &str)) -> anyhow::Result<Self> {
        let (x0, y0) = start.split_once(",").ok_or_else(|| err!("comma expected"))?;
        let x0: isize = x0.parse()?;
        let y0: isize = y0.parse()?;
        let (x1, y1) = end.split_once(",").ok_or_else(|| err!("comma expected"))?;
        let x1: isize = x1.parse()?;
        let y1: isize = y1.parse()?;
        Ok(Wall {
            x0: cmp::min(x0, x1),
            x1: cmp::max(x0, x1),
            y0: cmp::min(y0, y1),
            y1: cmp::max(y0, y1),
        })
    }
}

#[cfg(test)]
mod test {
    use super::main;

    static DATA: &str = r#"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;

    #[test]
    fn solution() {
        let res = main(DATA.trim()).expect("invalid input");
        assert_eq!(res, (93, 0));
    }
}
