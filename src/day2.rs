
pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {
    let mut part1: usize = 0;
    for line in data.lines() {
        let mut split = line.split(' ').take(2);
        let a = split.next()
            .map(|l| Hand::from_known(l))
            .ok_or(anyhow::anyhow!("Bad input line"))?;
        let b = split.next()
            .map(|l| Hand::from_guessed(l))
            .ok_or(anyhow::anyhow!("Bad input line"))?;
        part1 += b.score_over(&a);
        part1 += b.score();
    }

    let mut part2: usize = 0;
    for line in data.lines() {
        let mut split = line.split(' ').take(2);
        let a = split.next()
            .map(|l| Hand::from_known(l))
            .ok_or(anyhow::anyhow!("Bad input line"))?;
        let b = split.next()
            .map(|l| RoundResult::from_str(l))
            .ok_or(anyhow::anyhow!("Bad input line"))?;
        let hand = b.get_hand(&a);
        part2 += hand.score_over(&a);
        part2 += hand.score();
    }
    Ok((part1, part2))
}


#[derive(Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn score(&self) -> usize {
        use Hand::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn score_over(&self, other: &Hand) -> usize {
        use Hand::*;
        match (self, other) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => 6,
            (Rock, Rock) | (Scissors, Scissors) | (Paper, Paper) => 3,
            _ => 0,
        }
    }

    fn from_known(val: &str) -> Self {
        match val {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => unreachable!(),
        }
    }

    fn from_guessed(val: &str) -> Self {
        match val {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => unreachable!(),
        }
    }
}

enum RoundResult {
    Win,
    Lose,
    Draw,
}

impl RoundResult {
    fn from_str(val: &str) -> Self {
        use RoundResult::*;
        match val {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => unreachable!(),
        }
    }
    fn get_hand(&self, other: &Hand) -> Hand {
        use RoundResult::*;
        use Hand::*;
        match (self, other) {
            (Win, Rock) => Paper,
            (Win, Scissors) => Rock,
            (Win, Paper) => Scissors,
            (Lose, Rock) => Scissors,
            (Lose, Scissors) => Paper,
            (Lose, Paper) => Rock,
            x => *x.1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::main;

    static DATA: &str = concat!(
        "A Y\n",
        "B X\n",
        "C Z\n",
    );

    #[test]
    fn solution() {
        let (a, b) = main(DATA).expect("Invalid input");
        assert_eq!(a, 15);
        assert_eq!(b, 12);
    }
}
