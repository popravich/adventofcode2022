pub fn main(data: &str) -> anyhow::Result<(i64, String)> {
    let part1 = data.lines()
        .into_elves_cpu_cmd_iterator()
        .enumerate()
        .map(|(i, x)| (i+1, x))
        .skip(19)
        .step_by(40)
        .map(|(tick, x)| (tick as i64) * x)
        .sum();

    let mut prev_x = 1;
    let mut sprite = 7u64; // initialy sprite is vissible;
    let mask: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
    let mut screen = ['.'; 6*41];
    for i in 0..6 {
        screen[i * 41 + 40] = '\n';
    }
    for (idx, x) in data.lines().into_elves_cpu_cmd_iterator().enumerate() {
        if prev_x != x {
            if x > 0 {
                sprite = (7u64 << x) >> 1;
            } else {
                sprite = (7u64 >> x.abs()) >> 1;
            }
            prev_x = x;
        }
        let pixel = mask & sprite;
        // println!("CRT: {} {} {} {}", row, col, x, is_lit);

        let (row, col) = (idx / 40, idx % 40);
        let is_lit = pixel & (1 << col) > 0;
        if is_lit {
            screen[row * 41 + col] = '#';
        }
    }

    let part2 = screen.iter().collect::<String>();
    println!("---------");
    print!("{}", part2);
    println!("---------");

    Ok((part1, part2))
}

struct ElvesCpuCommands<'a, I>
where
    I: Iterator<Item=&'a str>,
{
    lines: &'a mut I,
    reg_x: i64,
    next_cmd_ticks: Option<u8>,
    next_cmd_value: Option<i64>,
}

impl<'a, I> ElvesCpuCommands<'a, I>
where
    I: Iterator<Item=&'a str>,
{
    fn maybe_pull_command(&mut self) -> Option<()> {
        if self.next_cmd_ticks.is_none() { 
            let cmd = self.lines.next()?;
            // println!("line: {}", cmd);
            if cmd.starts_with("addx ") {
                let next_val: i64 = cmd[5..].parse().expect("invalid input");
                self.next_cmd_value.replace(next_val);
                self.next_cmd_ticks.replace(2);
            } else if cmd == "noop" {
                self.next_cmd_value.take();
                self.next_cmd_ticks.replace(1);
            } else {
                unreachable!("Unknown command: {}", cmd)
            }
        }
        Some(())
    }
    fn start_tick(&mut self) {
        self.next_cmd_ticks = if let Some(t) = self.next_cmd_ticks {
            Some(t - 1)
        } else {
            unreachable!("");
        }
    }

    fn finish_tick(&mut self) {
        if let Some(0) = self.next_cmd_ticks {
            self.reg_x += self.next_cmd_value.take().unwrap_or(0);
            self.next_cmd_ticks.take();
        }
    }
}

//

impl<'a, I> Iterator for ElvesCpuCommands<'a, I>
where
    I: Iterator<Item=&'a str>,
{
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        self.finish_tick();
        self.maybe_pull_command()?;
        self.start_tick();
        Some(self.reg_x)
    }
}

trait IteratorExt<'a>: Iterator<Item=&'a str>
where
    Self: Sized,
{
    fn into_elves_cpu_cmd_iterator(&'a mut self) -> ElvesCpuCommands<'a, Self> {
        ElvesCpuCommands { lines: self, reg_x: 1, next_cmd_ticks: None, next_cmd_value: None }
    }
}

impl<'a, T> IteratorExt<'a> for T
where
    T: Iterator<Item=&'a str>
{}


#[cfg(test)]
mod test {
    use super::main;

    static DATA: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    #[test]
    fn solution() {
        let res = main(DATA).expect("invalid input");
        let part2 = r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#.to_string();
        assert_eq!(res, (13140, part2));
    }
}

