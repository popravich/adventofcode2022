use anyhow::anyhow as err;
use std::str;

const T: usize = 24;

pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {
    let blueprints = data
        .lines()
        .map(|line| line.parse())
        .collect::<anyhow::Result<Vec<Blueprint>>>()?;
    println!("Data: {:?}", blueprints);

    Ok((0, 0))
}

#[derive(Debug)]
struct Blueprint {
    index: usize,
    ore_robot: Cost,
    clay_robot: Cost,
    obsidian_robot: Cost,
    geode_robot: Cost,
}


#[derive(Debug, Default)]
struct Cost {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

impl str::FromStr for Blueprint {
    type Err = anyhow::Error;
    fn from_str(val: &str) -> anyhow::Result<Self> {
        let (prefix, tail) = val.split_once(":").ok_or_else(|| err!("expected ':'"))?;
        let index = prefix["Blueprint ".len()..].parse()?;
        let robot_lines = tail
            .split(". ")
            .map(|w| w.trim_matches(&['.', ' '][..]))
            .collect::<Vec<_>>();
        let ore_robot = robot_lines[0]["Each ore robot costs ".len()..].parse()?;
        let clay_robot = robot_lines[1]["Each clay robot costs ".len()..].parse()?;
        let obsidian_robot = robot_lines[2]["Each obsidian robot costs ".len()..].parse()?;
        let geode_robot = robot_lines[3]["Each geode robot costs ".len()..].parse()?;

        Ok(Blueprint { index, ore_robot, clay_robot, obsidian_robot, geode_robot })
    }
}

impl str::FromStr for Cost {
    type Err = anyhow::Error;
    fn from_str(val: &str) -> anyhow::Result<Self> {
        let mut cost = Cost::default();
        for part in val.split(" and ") {
            let (x, typ) = part.split_once(" ").ok_or_else(|| err!("expected cost"))?;
            let x: usize = x.parse()?;
            match typ {
                "ore" => cost.ore = x,
                "clay" => cost.clay = x,
                "obsidian" => cost.obsidian = x,
                _ => return Err(err!("unexpected cost type: {}", part))
            }
        }
        Ok(cost)
    }
}

#[cfg(test)]
mod test {
    use super::main;

    static DATA: &str = concat!(
        "Blueprint 1:",
        " Each ore robot costs 4 ore. ",
        " Each clay robot costs 2 ore. ", 
        " Each obsidian robot costs 3 ore and 14 clay.",
        " Each geode robot costs 2 ore and 7 obsidian.",
        "\n",
        "Blueprint 2:",
        " Each ore robot costs 2 ore.",
        " Each clay robot costs 3 ore.",
        " Each obsidian robot costs 3 ore and 8 clay.",
        " Each geode robot costs 3 ore and 12 obsidian.",
    );

    #[test]
    fn solution() {
        let res = main(DATA).expect("invalid input");
        assert_eq!(res, (33, 0));
    }
}
