use anyhow::anyhow as err;
use std::str;

pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {
    let grid: Grid = data.parse()?;
    let mut part1 = grid.perimeter();
    part1 += (1..grid.height() - 1)
        .flat_map(|row| (1..grid.width - 1).map(move |col| (row, col)))
        .filter(|(row, col)| grid.is_visible(*row, *col))
        .count();

    let part2 = (1..grid.height() - 1)
        .flat_map(|row| (1..grid.width - 1).map(move |col| (row, col)))
        .map(|(row, col)| grid.view_distance(row, col))
        .max()
        .ok_or_else(|| err!("empty iterator"))?;
    Ok((part1, part2))
}

#[derive(Debug)]
struct Grid {
    width: usize,
    data: Vec<u8>,
}

impl Grid {
    fn height(&self) -> usize {
        self.data.len() / self.width
    }
    fn perimeter(&self) -> usize {
        (self.width + self.height()) * 2 - 4
    }

    /// Creates two iterators of items to the left and to the right of specified cell,
    /// excluding the element itself.
    fn left_right_ray(&self, row: usize, col: usize)
        -> (impl Iterator<Item=u8> + '_, impl Iterator<Item=u8> + '_)
    {
        assert!(row < self.height());
        assert!(col < self.width);
        let start = row * self.width;
        let idx = start + col;
        let end = start + self.width;
        (
            self.data[start..idx].iter().rev().copied(),
            self.data[idx..end].iter().skip(1).copied(),
        )
    }
    /// Same as left_right_ray but up and down.
    fn up_down_ray(&self, row: usize, col: usize)
        -> (impl Iterator<Item=u8> + '_, impl Iterator<Item=u8> + '_)
    {
        assert!(row < self.height());
        assert!(col < self.width);
        let get_item = move |i| self.data[i * self.width + col];
        (
            (0..row).into_iter().rev().map(get_item),
            (row..self.height()).into_iter().skip(1).map(get_item)
        )
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        assert!(col < self.width);
        assert!(row < self.height());
        let idx = row * self.width + col;
        let val = self.data[idx];

        let (left, right) = self.left_right_ray(row, col);
        let (up, down) = self.up_down_ray(row, col);
        [left.max(), right.max(), up.max(), down.max()]
            .into_iter()
            .filter_map(|opt_max| opt_max)  // skip `None`s (edges)
            .any(|max| max < val)
    }

    fn view_distance(&self, row: usize, col: usize) -> usize {
        assert!(col < self.width);
        assert!(row < self.height());
        let idx = row * self.width + col;
        let val = self.data[idx];
        let (left, right) = self.left_right_ray(row, col);
        let (up, down) = self.up_down_ray(row, col);
        [
            left.count_while_inclusive(|i| i < val),
            right.count_while_inclusive(|i| i < val),
            up.count_while_inclusive(|i| i < val),
            down.count_while_inclusive(|i| i < val),
        ].into_iter()
        .product()
    }
}

impl str::FromStr for Grid {
    type Err = anyhow::Error;
    fn from_str(val: &str) -> anyhow::Result<Grid> {
        let mut data = Vec::new();
        let mut height: usize = 0;
        let mut width: usize = 0;
        for c in val.as_bytes() {
            match c {
                b'0'..=b'9' => data.push(c - b'0'),
                b'\n' => {
                    height += 1;
                    width = data.len() / height;
                    if data.len() % height != 0 {
                        return Err(err!("Unequal rows"))
                    }
                }
                _ => return Err(err!("Unexpected byte {}", c))
            }
        }
        Ok(Grid { width, data })
    }
}

/// Extend Iterator trait with extra method.
/// This is analogue to `.take_while(..).count()` but inclusive.
trait IteratorExt: Iterator + Sized {
    fn count_while_inclusive<F>(mut self, mut f: F) -> usize
    where
        F: FnMut(Self::Item) -> bool,
    {
        let mut x = 0usize;
        while let Some(item) = self.next() {
            x += 1;
            if !f(item) {
                break
            }
        }
        x
    }
}

impl<T> IteratorExt for T
where
    T: Iterator,
{}

#[cfg(test)]
mod test {
    use super::main;

    static DATA: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn solution() {
        let res = main(DATA).expect("invalid input");
        assert_eq!(res, (21, 8));
    }
}
