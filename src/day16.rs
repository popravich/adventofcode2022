use std::str;
use std::collections::{HashMap, HashSet};
use anyhow::anyhow as err;

use crate::bms;

pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {
    let mut valves = data
        .lines()
        .map(|l| l.parse())
        .collect::<anyhow::Result<Vec<Valve>>>()?;
    println!("");
    let graph = Graph::build(&valves);

    Ok((0, 0))
}

#[derive(Debug)]
struct Valve {
    label: String,
    rate: u32,
    next: Vec<String>,
}

impl str::FromStr for Valve {
    type Err = anyhow::Error;
    fn from_str(val: &str) -> anyhow::Result<Self> {
        let mut it = val.split(" ");
        let label = it.nth(1).ok_or_else(|| err!("label is expected"))?.to_string();
        let rate_str = it.nth(2).ok_or_else(|| err!("rate is expected"))?;
        let rate = rate_str[5..rate_str.len() - 1].parse()?;
        let next = it.skip(4).map(|word| word.trim_end_matches(',').to_string()).collect();
        Ok(Valve {
            label,
            rate,
            next,
        })
    }
}

#[derive(Debug)]
struct Graph {
    links: HashMap<String, Vec<String>>,
}

impl Graph {
    fn build(valves: &[Valve]) -> Self {
        let links = valves.iter().map(|v| (v.label.clone(), v.next.clone())).collect();
        Graph { links }
    }

    fn path(&self, from: &str, to: &str) -> Vec<String> {
        let path = bms::build_path(from, to, |node| {
            self.links.get(node).expect("valid node")
        });
        path.expect("path exists")
    }
    fn distance(&self, from: &str, to: &str) -> usize {
        self.path(from, to).len()
    }
}

#[cfg(test)]
mod test {
    use super::main;

    static DATA: &str = r#"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#;

    #[test]
    fn solution() {
        let res = main(DATA.trim()).expect("invalid input");
        assert_eq!(res, (1651, 0));

    }
}
