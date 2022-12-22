use anyhow::anyhow as err;

pub fn main(data: &str) -> anyhow::Result<(i64, i64)> {
    let mut values = Vec::with_capacity(data.trim().lines().count());
    for (index, line) in data.trim().lines().enumerate() {
        let value = line.parse()?;
        values.push(N { value, index });
    }

    let mut part1 = values.clone();
    shuffle_values(&mut part1)?;
    let it = part1.iter().cycle().skip_while(|v| v.value != 0);
    let a1 = it.clone().nth(1000).ok_or_else(|| err!("corrupted iterator"))?.value;
    let b1 = it.clone().nth(2000).ok_or_else(|| err!("corrupted iterator"))?.value;
    let c1 = it.clone().nth(3000).ok_or_else(|| err!("corrupted iterator"))?.value;

    let decr_key: i64 = 811589153;

    let mut part2 = values
        .into_iter()
        .map(|N{value, index}| N { index, value: decr_key * value})
        .collect();
    for _ in 0..10 {
        shuffle_values(&mut part2)?;
    }
    let it = part2.iter().cycle().skip_while(|v| v.value != 0);
    let a2 = it.clone().nth(1000).ok_or_else(|| err!("corrupted iterator"))?.value;
    let b2 = it.clone().nth(2000).ok_or_else(|| err!("corrupted iterator"))?.value;
    let c2 = it.clone().nth(3000).ok_or_else(|| err!("corrupted iterator"))?.value;

    Ok((a1 + b1 + c1, a2 + b2 + c2))
}

#[derive(Debug, Clone, Copy)]
struct N {
    value: i64,
    index: usize,
}

fn shuffle_values(values: &mut Vec<N>) -> anyhow::Result<()> {
    for i in 0..values.len() {
        let (from, &item) = values
            .iter()
            .enumerate()
            .find(|(_, v)| v.index == i)
            .ok_or_else(|| err!("not found {}", i))?;
        if item.value == 0 {
            continue;
        }
        let value = item.value;
        values.remove(from);
        if value < 0 {
            let r = value.abs() as usize % values.len();
            values.rotate_right(r);
        } else {
            let r = value as usize % values.len();
            values.rotate_left(r);
        }
        values.insert(from, item);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::main;
    static DATA: &str = r#"
1
2
-3
3
-2
0
4"#;

    #[test]
    fn solution() {
        let res = main(DATA).expect("invalid input");
        assert_eq!(res, (3, 1623178306));
    }
}
