use anyhow::anyhow as err;

pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {
    let part1 = find_unique_charset(data.as_bytes(), 4)? + 4;
    let part2 = find_unique_charset(data.as_bytes(), 14)? + 14;
    Ok((part1, part2))
}

fn find_unique_charset(input: &[u8], size: usize) -> anyhow::Result<usize> {
    input
    .windows(size)
    .enumerate()
    .skip_while(|(_, window)| {
        window
        .iter()
        .fold(0u32, |mask, byte| {
            mask | 1 << byte.checked_sub(b'a').expect("invalid char, not a-z")
        })
        .count_ones() as usize != size
    })
    .map(|(idx, _)| idx)
    .next()
    .ok_or(err!("no unique chars found"))
}

#[cfg(test)]
mod test {
    use super::main;

    static DATA: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn solution() {
        let res = main(DATA).expect("invalid input");
        assert_eq!(res, (7, 19));
    }
}
