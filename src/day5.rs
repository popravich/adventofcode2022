pub fn main(data: &str) -> anyhow::Result<(String, String)> {
    let (crates, moves) = data
        .split_once("\n\n")
        .ok_or(anyhow::anyhow!("\n\n was not found"))?;
    let mut lines = crates.lines().rev();
    let n = lines
        .next()
        .map(|l| l.chars().filter(|c| *c != ' ').count())
        .ok_or(anyhow::anyhow!("could not count number of stacks"))?;
    assert!(n < 10, "Assumption failed, found more than 9 columns");
    let mut stacks = vec![String::new(); n];

    let crate_indices = lines.flat_map(|line| {
        (0..n)
            .filter_map(|idx| line.chars().nth(idx * 4 + 1).map(|c| (idx, c)))
            .filter(|(_, c)| *c != ' ')
    });

    for (idx, c) in crate_indices {
        stacks[idx].push(c);
    }

    let mut stacks2 = stacks.clone();

    for line in moves.lines() {
        let (count, address) = &line[5..]
            .split_once(" from ")
            .ok_or(anyhow::anyhow!("expected ' from ' in move command"))?;
        let (src, dst) = address
            .split_once(" to ")
            .ok_or(anyhow::anyhow!("expected ' to ' in move command"))?;
        let count: usize = count.parse()?;
        let src: usize = src.parse::<usize>()? - 1;
        let dst: usize = dst.parse::<usize>()? - 1;

        for _ in 0..count {
            let c = stacks[src].pop().ok_or(anyhow::anyhow!("empty stack"))?;
            stacks[dst].push(c);
        }

        let x = stacks2[src].len() - count;
        let pack: String = stacks2[src].drain(x..).collect();
        stacks2[dst].push_str(&pack);
    }

    let part1: String = stacks.iter_mut().filter_map(|s| s.pop()).collect();
    let part2: String = stacks2.iter_mut().filter_map(|s| s.pop()).collect();

    Ok((part1, part2))
}

#[cfg(test)]
mod test {
    use super::main;

    static DATA: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn solution() {
        let res = main(DATA).expect("invalid input");
        assert_eq!(res, ("CMZ".to_string(), "MCD".to_string()));
    }
}
