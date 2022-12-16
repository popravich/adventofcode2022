use anyhow::anyhow as err;
use std::collections::HashSet;
use std::cmp;
use rayon::prelude::*;

use crate::day4::Range;

#[cfg(test)]
const PART1_ROW: i32 = 10;
#[cfg(test)]
const N_COLS: i32 = 20;

#[cfg(not(test))]
const PART1_ROW: i32 = 2_000_000;
#[cfg(not(test))]
const N_COLS: i32 = 4_000_000;

pub fn main(data: &str) -> anyhow::Result<(usize, u64)> {
    let mut sensors = Vec::with_capacity(data.lines().count());
    let mut beacons = Vec::with_capacity(data.lines().count());
    for line in data.lines() {
        if !line.starts_with("Sensor at x=") {
            return Err(err!("invalid line start"))
        }

        let (part1, part2) = line["Sensor at x=".len()..]
            .split_once(": closest beacon is at x=")
            .ok_or_else(|| err!("invalid line"))?;
        let (x1, y1) = part1.split_once(", y=").ok_or_else(|| err!(" y= expected"))?;
        let (x2, y2) = part2.split_once(", y=").ok_or_else(|| err!(" y= expected"))?;
        let x1: i32 = x1.parse()?;
        let y1: i32 = y1.parse()?;
        let x2: i32 = x2.parse()?;
        let y2: i32 = y2.parse()?;
        beacons.push((x2, y2));
        sensors.push(Sensor {
            x: x1,
            y: y1,
            r: (x2 - x1).abs() + (y2 - y1).abs(),
        });
    }

    let mut lines: HashSet<_> = sensors
        .iter()
        .filter_map(|s| s.intersects_horizontal(PART1_ROW))
        .flat_map(|(s, e)| s..=e)
        .collect();
    for (x, y) in beacons {
        if y == PART1_ROW && lines.contains(&x) {
            lines.remove(&x);
        }
    }
    let part1 = lines.len();

    let mut part2 = 0;
    let test_range = (0i32, N_COLS).into();
    for i in 0..N_COLS {
        // println!("{}", i);
        let mut v_segments: Vec<Range<_>> = sensors
            .par_iter()
            .filter_map(|s| s.intersects_vertical(i).map(|s| s.into()))
            .collect();
        sort_and_dedup_ranges(&mut v_segments);
        // skip ranges that cover searched range
        if v_segments[0].contains(&test_range) {
            continue
        }
        if let Some(y) = (0..N_COLS)
            .into_par_iter()
            .find_first(|y| v_segments.iter().all(|s| !s.contains_point(&y)))
        {
            part2 = i as u64 * 4_000_000u64 + y as u64;
        }
    }

    Ok((part1, part2))
}

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    r: i32,
}

impl Sensor {
    fn intersects_horizontal(&self, y: i32) -> Option<(i32, i32)> {
        (self.y - self.r <= y && y <= self.y + self.r)
        .then(|| {
            let dx = (self.r - (y - self.y).abs()).abs();
            (self.x - dx, self.x + dx)
        })
    }
    fn intersects_vertical(&self, x: i32) -> Option<(i32, i32)> {
        (self.x - self.r <= x && x <= self.x + self.r)
        .then(|| {
            let dy = (self.r - (x - self.x).abs()).abs();
            (self.y - dy, self.y + dy)
        })
    }
}

fn sort_and_dedup_ranges<T>(vec: &mut Vec<Range<T>>)
where
    T: Ord + Copy + std::fmt::Debug,
{
    let mut len = usize::MAX;
    vec.sort_by(|r1, r2| r1.start.cmp(&r2.start));
    loop {
        vec.dedup_by(|r1, r2| {
            if r1.overlaps_inclusive(r2) {
                r2.start = cmp::min(r1.start, r2.start);
                r2.end = cmp::max(r1.end, r2.end);
                true
            } else {
                false
            }
        });
        if len == vec.len() {
            break
        }
        len = vec.len();
    }
}

#[cfg(test)]
mod test {
    use super::main;
    static DATA: &str = r#"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;

    #[test]
    fn solution() {
        let res = main(DATA.trim()).expect("invalid input");
        assert_eq!(res, (26, 56000011));
    }
}
