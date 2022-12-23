use std::str;
use std::collections::{HashSet, HashMap};
use anyhow::anyhow as err;

pub fn main(data: &str) -> anyhow::Result<(i64, i64)> {
    let n = data.trim().lines().count();
    let mut yelled: HashSet<String> = HashSet::with_capacity(n);
    let mut gin: HashMap<String, HashSet<String>> = HashMap::new();
    let mut monkeys: HashMap<String, Yell> = HashMap::with_capacity(n);

    for line in data.trim().lines() {
        let (name, yell) = line.split_once(": ")
            .ok_or_else(|| err!("unexpected line: {}", line))?;
        let yell: Yell = yell.parse()?;
        // println!("Monkey '{}' yells {:?}", name, yell);
        if let Some([monkey1, monkey2]) = yell.waits() {
            gin.entry(monkey1.to_string())
                .or_default()
                .insert(name.to_string());
            gin.entry(monkey2.to_string())
                .or_default()
                .insert(name.to_string());
        }
        monkeys.insert(name.to_string(), yell);
    }

    loop {
        let yelling = monkeys
            .iter()
            .filter(|(n, y)| y.is_yelling() & !yelled.contains(n.as_str()))
            .map(|(n, y)| (n.clone(), y.clone()))
            .collect::<Vec<(String, Yell)>>();
        if yelling.is_empty() {
            break
        }
        for (name, yell) in yelling {
            for to in gin.remove(&name).iter().flatten() {
                let monkey = monkeys.get_mut(to).ok_or_else(|| err!("unknown monkey"))?;
                monkey.hear(yell.number(), &name);
            }
            yelled.insert(name);
        }
    }
    let root = monkeys.get("root").ok_or_else(|| err!("monkey is missing"))?;
    assert!(root.is_yelling());
    Ok((root.number(), 0))
}

#[derive(Debug, Clone)]
enum Yell {
    Add(Arg, Arg),
    Sub(Arg, Arg),
    Mult(Arg, Arg),
    Div(Arg, Arg),
    Number(i64),
}

#[derive(Debug, Clone)]
enum Arg {
    Wait(String),
    Ready(i64),
}

impl Yell {
    fn waits(&self) -> Option<[&str; 2]> {
        use Yell::*;
        use Arg::*;
        match self {
            Number(_) => None,
            Add(Wait(a), Wait(b)) => Some([&a, &b]),
            Sub(Wait(a), Wait(b)) => Some([&a, &b]),
            Mult(Wait(a), Wait(b)) => Some([&a, &b]),
            Div(Wait(a), Wait(b)) => Some([&a, &b]),
            _ => unreachable!("waits() must be called before yelling"),
        }
    }

    fn is_yelling(&self) -> bool {
        matches!(self, Yell::Number(_))
    }
    fn number(&self) -> i64 {
        match self {
            Yell::Number(n) => *n,
            _ => unreachable!("must have checked if monkey is_yelling()"),
        }
    }

    fn hear(&mut self, number: i64, from: &str) {
        use Yell::*;
        use Arg::*;
        let next = match self.clone() {
            Add(Wait(name), b) if name == from => Add(Ready(number), b),
            Sub(Wait(name), b) if name == from => Sub(Ready(number), b),
            Mult(Wait(name), b) if name == from => Mult(Ready(number), b),
            Div(Wait(name), b) if name == from => Div(Ready(number), b),
            Number(_) => unreachable!("monkey already heard enough"),
            x => x, // check in next match section
        };
        let next = match next {
            Add(a, Wait(name)) if name == from => Add(a, Ready(number)),
            Sub(a, Wait(name)) if name == from => Sub(a, Ready(number)),
            Mult(a, Wait(name)) if name == from => Mult(a, Ready(number)),
            Div(a, Wait(name)) if name == from => Div(a, Ready(number)),
            Number(_) => unreachable!("monkey already heard enough"),
            x => x,
        };
        *self = match next {
            Add(Ready(a), Ready(b)) => Number(a + b),
            Sub(Ready(a), Ready(b)) => Number(a - b),
            Mult(Ready(a), Ready(b)) => Number(a * b),
            Div(Ready(a), Ready(b)) => Number(a / b),
            x => x,
        };
    }
}

impl str::FromStr for Yell {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> anyhow::Result<Self> {
        match value.trim().parse::<i64>() {
            Ok(x) => Ok(Yell::Number(x)),
            Err(_) => {
                let (arg1, tail) = value.split_once(" ").ok_or_else(|| err!("space not found"))?;
                let (op, arg2) = tail.split_once(" ").ok_or_else(|| err!("space not found"))?;
                let arg1 = Arg::Wait(arg1.to_string());
                let arg2 = Arg::Wait(arg2.to_string());
                match op {
                    "+" => Ok(Yell::Add(arg1, arg2)),
                    "-" => Ok(Yell::Sub(arg1, arg2)),
                    "*" => Ok(Yell::Mult(arg1, arg2)),
                    "/" => Ok(Yell::Div(arg1, arg2)),
                    _ => Err(err!("unexpected line {}", value)),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::main;

    static DATA: &str = r#"
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#;

    #[test]
    fn solution() {
        let res = main(DATA).expect("invalid input");
        assert_eq!(res, (152, 0));
    }
}
