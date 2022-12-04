use std::str;

pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in data.lines() {
        let (range1, range2) = line.split_once(',').ok_or(anyhow::anyhow!("invalid input"))?;
        let range1: Range = range1.parse()?;
        let range2: Range = range2.parse()?;
        if range1.contains(&range2) || range2.contains(&range1) {
            part1 += 1;
        }
        if range1.overlaps(&range2) {
            part2 += 1;
        }

    }
    Ok((part1, part2))
}

struct Range {
    start: u64,
    end: u64,
}

impl str::FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(val: &str) -> anyhow::Result<Self> {
        let (start, end) = val.split_once('-').ok_or(anyhow::anyhow!("expected '-'"))?;
        let start: u64 = start.parse()?;
        let end: u64 = end.parse()?;
        Ok(Range { start, end })
    }
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        !(self.end < other.start || self.start > other.end)
    }
}

#[cfg(test)]
mod test {
    use super::main;

    static DATA: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn solution() {
        let res = main(DATA).expect("invalid input");
        assert_eq!(res, (2, 4));
    }
}
