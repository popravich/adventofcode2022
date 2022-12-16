use std::str::Chars;
use std::cmp::Ordering;
use anyhow::anyhow as err;

pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {
    let mut part1 = 0;
    let mut part2_data = Vec::new();
    for (idx, pair) in data.trim().split("\n\n").enumerate() {
        let (left, right) = pair.split_once("\n").ok_or_else(|| err!("expected a pair"))?;
        let l: Vec<Packet> = <Chars<'_> as Into<Tokenizer<'_>>>::into(left.chars()).collect();
        let r: Vec<Packet> = <Chars<'_> as Into<Tokenizer<'_>>>::into(right.chars()).collect();
        if l <= r {
            part1 += idx + 1;
        }
        part2_data.push(l);
        part2_data.push(r);
    }
    let marker_a = vec![Packet::List(vec![Packet::Value(2)])];
    let marker_b = vec![Packet::List(vec![Packet::Value(6)])];
    part2_data.push(marker_a.clone());
    part2_data.push(marker_b.clone());

    part2_data.sort();
    let part2 = part2_data
        .iter()
        .position(|x| *x == marker_a)
        .ok_or_else(|| err!("marker not found"))? + 1;
    let part2 = part2 * (part2_data
        .iter()
        .position(|x| *x == marker_b)
        .ok_or_else(|| err!("marker not found"))? + 1);
    Ok((part1, part2))
}

fn lists_compare<'a, I1, I2>(left: I1, right: I2) -> Option<Ordering>
where
    I1: IntoIterator<Item=&'a Packet>,
    I2: IntoIterator<Item=&'a Packet>,
{
    let mut l = left.into_iter();
    let mut r = right.into_iter();
    loop {
        match (l.next(), r.next()) {
            (None, None) => return Some(Ordering::Equal),
            (Some(_), None) => return Some(Ordering::Greater),
            (None, Some(_)) => return Some(Ordering::Less),
            (Some(a), Some(b))=> {
                match a.partial_cmp(b)? {
                    o @ (Ordering::Less | Ordering::Greater) => return Some(o),
                    Ordering::Equal => continue
                }
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        use Packet::*;

        match (self, other) {
            (Value(l), Value(r)) => l.partial_cmp(r),
            (List(l), List(r)) => lists_compare(l, r),
            (lv @ Value(_), List(r)) => lists_compare([lv], r),
            (List(l), rv @ Value(_)) => lists_compare(l, [rv]),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Ord)]
enum Packet {
    Value(u32),
    List(Vec<Packet>),
}

impl FromIterator<Token> for Vec<Packet> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item=Token>
    {
        let mut result = Vec::new();
        let mut it = iter.into_iter();
        let mut stack = Vec::new();
        while let Some(token) = it.next() {
            match token {
                Token::Value(v) => result.push(Packet::Value(v)),
                Token::ListStart => {
                    stack.push(result);
                    result = Vec::new();
                }
                Token::ListEnd => {
                    let mut tmp = stack.pop().expect("invalid lists stack");
                    tmp.push(Packet::List(result));
                    result = tmp;
                }
            }
        }
        result
    }
}

// Parsing stuff...

#[derive(Debug)]
enum Token {
    Value(u32),
    ListStart,
    ListEnd,
}
struct Tokenizer<'a> {
    chars: Chars<'a>
}

impl<'a> From<Chars<'a>> for Tokenizer<'a> {
    fn from(chars: Chars<'a>) -> Tokenizer<'a> {
        Tokenizer { chars }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let c = self.chars.next()?;
            match c {
                '[' => return Some(Token::ListStart),
                ']' => return Some(Token::ListEnd),
                ',' => continue,
                '0'..='9' => {
                    let mut v: u32 = (c as u8 - b'0') as u32;
                    let mut x = self.chars.clone();
                    loop {
                        match x.next() {
                            Some(c @ '0'..='9') => {
                                v = v * 10 + (c as u8 - b'0') as u32;
                                self.chars.next();
                            }
                            _ => break
                        }
                    }
                    return Some(Token::Value(v))
                }
                _ => unreachable!("Unexpected char {}", c),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::main;

    static DATA: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    #[test]
    fn solution() {
        let res = main(DATA).expect("invalid input");
        assert_eq!(res, (13, 140));
    }
}
