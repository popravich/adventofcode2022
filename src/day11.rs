use anyhow::anyhow as err;

use std::str::FromStr;

pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {
    let mut it = data.lines();
    let mut monkeys = Vec::new();
    while let Some(l) = it.next() {
        if l.is_empty() {
            continue
        }
        let MonkeyLine(n) = l.parse()?;
        let StartingLine(items) = it.next().ok_or_else(|| err!("more line expected"))?.parse()?;
        let OperationLine(operation) = it.next().ok_or_else(|| err!("more line expected"))?.parse()?;
        let TestLine(divisible_by) = it.next().ok_or_else(|| err!("more line expected"))?.parse()?;
        let TrueLine(on_true) = it.next().ok_or_else(|| err!("more line expected"))?.parse()?;
        let FalseLine(on_false) = it.next().ok_or_else(|| err!("more line expected"))?.parse()?;

        assert!(n == monkeys.len());
        monkeys.push(Monkey { items, operation, divisible_by, on_true, on_false });
    }

    let mut monkeys2 = monkeys.clone();
    let mut counts2 = [0].repeat(monkeys.len());

    let mut counts = [0].repeat(monkeys.len());
    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            while let Some(n) = monkeys[idx].items.pop() {
                counts[idx] += 1;
                let (i, x) = monkeys[idx].inspect(n);
                monkeys[i].items.push(x);
            }
        }
    }
    counts.sort();
    let part1 = counts.iter().rev().take(2).product();

    let lcm = monkeys2.iter().map(|m| m.divisible_by).product();
    for _ in 0..10_000 {
        for idx in 0..monkeys2.len() {
            while !monkeys2[idx].items.is_empty() {
                let n = monkeys2[idx].items.remove(0);
                counts2[idx] += 1;
                let (i, x) = monkeys2[idx].inspect_v2(n, lcm);
                monkeys2[i].items.push(x);
            }
        }
    }
    counts2.sort();
    let part2 = counts2.iter().rev().take(2).product();
    Ok((part1, part2))
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    divisible_by: i64,
    on_true: usize,
    on_false: usize,
}
impl Monkey {
    fn inspect(&self, item: i64) -> (usize, i64) {
        let out = self.operation.apply(item);
        let out = out / 3;
        let idx = if out % self.divisible_by == 0 {
            self.on_true
        } else {
            self.on_false
        };
        (idx, out)
    }

    fn inspect_v2(&self, item: i64, lcm: i64) -> (usize, i64) {
        let out = self.operation.apply(item);
        let idx = if out % self.divisible_by == 0 {
            self.on_true
        } else {
            self.on_false
        };
        (idx, out % lcm)
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Arg),
    Mult(Arg),
}
impl Operation {
    fn apply(&self, item: i64) -> i64 {
        use Operation::*;
        use Arg::*;
        match self {
            Add(Old) => item + item,
            Add(Val(x)) => item + x,
            Mult(Old) => item * item,
            Mult(Val(x)) => item * x,
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum Arg {
    Old,
    Val(i64),
}

struct MonkeyLine(usize);
struct StartingLine(Vec<i64>);
struct OperationLine(Operation);
struct TestLine(i64);
struct TrueLine(usize);
struct FalseLine(usize);

impl FromStr for MonkeyLine {
    type Err = anyhow::Error;
    fn from_str(val: &str) -> anyhow::Result<Self> {
        assert!(val.starts_with("Monkey "));
        let idx = val["Monkey ".len()..val.len() - 1].parse()?;
        Ok(MonkeyLine(idx))
    }
}
impl FromStr for StartingLine {
    type Err = anyhow::Error;
    fn from_str(val: &str) -> anyhow::Result<Self> {
        assert!(val.starts_with("  Starting items: "));
        let items = val["  Starting items: ".len()..]
            .split(',')
            .map(|s| s.trim().parse())
            .collect::<Result<Vec<i64>, std::num::ParseIntError>>()?;
        Ok(StartingLine(items))
    }
}
impl FromStr for OperationLine {
    type Err = anyhow::Error;
    fn from_str(val: &str) -> anyhow::Result<Self> {
        assert!(val.starts_with("  Operation: new = old "));
        let (op, arg) = val["  Operation: new = old ".len()..]
            .split_once(" ")
            .ok_or_else(|| err!("space expected"))?;
        let arg = match arg {
            "old" => Arg::Old,
            x => Arg::Val(x.parse()?),
        };
        let op = match op {
            "*" => Operation::Mult(arg),
            "+" => Operation::Add(arg),
            x => return Err(err!("unknown operation {}", x))
        };
        Ok(OperationLine(op))
    }
}
impl FromStr for TestLine {
    type Err = anyhow::Error;
    fn from_str(val: &str) -> anyhow::Result<Self> {
        assert!(val.starts_with("  Test: divisible by "));
        let i = val["  Test: divisible by ".len()..].parse()?;
        Ok(TestLine(i))
    }
}
impl FromStr for TrueLine {
    type Err = anyhow::Error;
    fn from_str(val: &str) -> anyhow::Result<Self> {
        assert!(val.starts_with("    If true: throw to monkey "));
        let i = val["    If true: throw to monkey ".len()..].parse()?;
        Ok(TrueLine(i))
    }
}
impl FromStr for FalseLine {
    type Err = anyhow::Error;
    fn from_str(val: &str) -> anyhow::Result<Self> {
        assert!(val.starts_with("    If false: throw to monkey "));
        let i = val["    If false: throw to monkey ".len()..].parse()?;
        Ok(FalseLine(i))
    }
}

#[cfg(test)]
mod test {
    use super::main;

    static DATA: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

    #[test]
    fn solution() {
        let res = main(DATA).expect("invalid input");
        assert_eq!(res, (10605, 2713310158));
    }
}
