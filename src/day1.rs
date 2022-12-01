pub fn main(data: &str) -> anyhow::Result<(usize, usize)>  {
    let mut cargo = Vec::new();
    let mut calories = 0usize;
    for line in data.lines() {
        if line.is_empty() {
            cargo.push(calories);
            calories = 0usize;
            continue
        }
        calories += line.parse::<usize>()?;
    }
    cargo.push(calories);

    cargo.sort_by_key(|c| -(*c as isize));

    let max_calories = cargo[0];
    let top_three = cargo[..3].iter().sum();
    Ok((max_calories, top_three))
}

#[cfg(test)]
mod test {
    use super::main;

    static DATA: &str = concat!(
        "1000\n",
        "2000\n",
        "3000\n",
        "\n",
        "4000\n",
        "\n",
        "5000\n",
        "6000\n",
        "\n",
        "7000\n",
        "8000\n",
        "9000\n",
        "\n",
        "10000\n",
    );

    #[test]
    fn solution() {
        let (a, b) = main(DATA).expect("invalid input");
        assert_eq!(a, 24000);
        assert_eq!(b, 45000);
    }

}
