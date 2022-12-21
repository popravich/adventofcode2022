use std::str;
use std::collections::{VecDeque, HashSet};
use anyhow::anyhow as err;

pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {
    let cubes = data.trim().lines().map(|l| l.parse()).collect::<anyhow::Result<HashSet<Coord>>>()?;
    let mut sides = 0;

    let mut sides2 = 0;
    let min_x = cubes.iter().map(|c| c.x).min().ok_or_else(|| err!("empty set"))?;
    let max_x = cubes.iter().map(|c| c.x).max().ok_or_else(|| err!("empty set"))?;
    let min_y = cubes.iter().map(|c| c.y).min().ok_or_else(|| err!("empty set"))?;
    let max_y = cubes.iter().map(|c| c.y).max().ok_or_else(|| err!("empty set"))?;
    let min_z = cubes.iter().map(|c| c.z).min().ok_or_else(|| err!("empty set"))?;
    let max_z = cubes.iter().map(|c| c.z).max().ok_or_else(|| err!("empty set"))?;

    for cube in cubes.iter() {
        for c in cube.iter_neighbours().filter(|c| !cubes.contains(c)) {
            sides += 1;
            if search_out_of_bounds(c, &cubes, min_x, max_x, min_y, max_y, min_z, max_z) {
                sides2 += 1;
            }
        }
    }

    Ok((sides, sides2))
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Coord{
    x: isize,
    y: isize,
    z: isize,
}

impl Coord {
    fn iter_neighbours(&self) -> impl Iterator<Item=Coord> {
        let Coord { x, y , z } = *self;
        [
            Coord { x: x - 1, y, z },
            Coord { x: x + 1, y, z },
            Coord { x, y: y - 1, z },
            Coord { x, y: y + 1, z },
            Coord { x, y, z: z - 1 },
            Coord { x, y, z: z + 1 },
        ].into_iter()
    }
}

impl str::FromStr for Coord {
    type Err = anyhow::Error;
    fn from_str(val: &str) -> anyhow::Result<Self> {
        let (x, yz) = val.split_once(",").ok_or_else(|| err!("expected comma"))?;
        let (y, z) = yz.split_once(",").ok_or_else(|| err!("expected another comma"))?;
        Ok(Coord {
            x: x.parse()?,
            y: y.parse()?,
            z: z.parse()?,
        })
    }
}

fn search_out_of_bounds(
    cube: Coord,
    cubes: &HashSet<Coord>,
    min_x: isize, max_x: isize,
    min_y: isize, max_y: isize,
    min_z: isize, max_z: isize,
) -> bool {
    let mut seen = cubes.clone();
    let mut todo = VecDeque::new();
    todo.push_back(cube);
    while !todo.is_empty() {
        let c = todo.pop_front().expect("corrupted VecDeque");
        seen.insert(c);
        for n in c.iter_neighbours().filter(|c| !seen.contains(c)) {
            if n.x < min_x || n.x > max_x {
                return true
            } else if n.y < min_y || n.y > max_y {
                return true
            } else if n.z < min_z || n.z > max_z {
                return true
            }
            todo.push_front(n);
        }
    }
    false
}


#[cfg(test)]
mod test {
    use super::main;

    static DATA: &str = r#"
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
"#;
    #[test]
    fn solution() {
        let res = main(DATA).expect("invalid input");
        assert_eq!(res, (64, 58));
    }
}
