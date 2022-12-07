pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {
    let part1 = data
        .lines()
        .filter_map(|line| {
            let n = line.len() / 2;
            let (left, right) = line.split_at(n);
            left.chars().take(n).filter(|c| right.contains(*c)).next()
        })
        .collect::<String>();

    let mut lines = data.lines();
    let mut part2 = 0;
    while let Some(first) = lines.next() {
        let second = lines.next().ok_or(anyhow::anyhow!("invalid input"))?;
        let third = lines.next().ok_or(anyhow::anyhow!("invalid input"))?;
        let c = first
            .chars()
            .filter(|c| second.contains(*c))
            .filter(|c| third.contains(*c))
            .next()
            .ok_or(anyhow::anyhow!("No common items found"))?;
        part2 += char_score(c) as usize;
    }

    Ok((score(&part1), part2))
}

fn score(s: &str) -> usize {
    s.chars().map(|c| char_score(c) as usize).sum()
}

fn char_score(c: char) -> u8 {
    match c as u8 {
        b'a'..=b'z' => (c as u8) - b'a' + 1,
        b'A'..=b'Z' => (c as u8) - b'A' + 27,
        _ => unreachable!("Unexpected char: {}", c),
    }
}

#[cfg(test)]
mod test {
    use super::main;

    static DATA: &str = concat!(
        "vJrwpWtwJgWrhcsFMMfFFhFp\n",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n",
        "PmmdzqPrVvPwwTWBwg\n",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n",
        "ttgJtRGJQctTZtZT\n",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    );

    #[test]
    fn solution() {
        let res = main(DATA).expect("Invalid input");
        assert_eq!(res, (157, 70));
    }
}
