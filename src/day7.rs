use std::str;
use std::collections::BTreeMap;
use anyhow::anyhow as err;


const PART1_THRESHOLD: usize = 100_000;
const TOTAL_SPACE: usize = 70_000_000;
const NEED_SPACE: usize = 30_000_000;

pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {

    let mut cur_path = Path::new();
    let mut tree = BTreeMap::new();
    let mut stdout = data.lines().map(Line::from_str);
    while let Some(line) = stdout.next() {
        let line = line?;
        match line {
            Line::Input(Command::Cd(Cd::Root)) => {
                cur_path.root();
            }
            Line::Input(Command::Cd(Cd::Up)) => {
                cur_path.pop();
            }
            Line::Input(Command::Cd(Cd::Down(name))) => {
                cur_path.push(name);
            }
            Line::Output(LsOutput::File { size, .. }) => {
                for abs_path in cur_path.pathes_to_root() {
                    tree.entry(abs_path)
                        .and_modify(|total| *total += size)
                        .or_insert(size);
                }
            }
            _ => {}
        }
    };

    let mut part1 = 0;
    for size in tree.values() {
        if *size <= PART1_THRESHOLD {
            part1 += size;
        }
    }

    let used = tree.get("/").ok_or(err!("missing root"))?;
    let free = TOTAL_SPACE - used;
    let need = NEED_SPACE - free;

    let mut min = TOTAL_SPACE;
    for &size in tree.values() {
        if size >= need && size < min {
            min = size;
        }
    }
    Ok((part1, min))
}

#[derive(Debug)]
enum Line<'a> {
    Input(Command<'a>),
    Output(LsOutput<'a>),
}

#[derive(Debug)]
enum Command<'a> {
    Cd(Cd<'a>),
    Ls,
}

#[derive(Debug)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
enum LsOutput<'a> {
    Dir {
        name: &'a str,
    },
    File {
        size: usize,
        name: &'a str,
    }
}

#[derive(Debug)]
struct Path<'a>(Vec<&'a str>);

impl<'a> Line<'a> {
    fn from_str(val: &'a str) -> anyhow::Result<Line<'a>> {
        let res = if val.starts_with("$ ") {
            Line::Input(Command::from_str(&val[2..])?)
        } else {
            Line::Output(LsOutput::from_str(val)?)
        };
        Ok(res)
    }
}

impl<'a> Command<'a> {
    fn from_str(val: &'a str ) -> anyhow::Result<Command<'a>>
    {
        let result = if val.starts_with("cd ") {
            Command::Cd(Cd::from_str(&val[3..]))
        } else if val == "ls" {
            Command::Ls
        } else {
            return Err(err!("unexpected command {}", val))
        };
        Ok(result)
    }
}

impl<'a> Cd<'a> {
    fn from_str(val: &'a str) -> Cd<'a> {
        match val {
            "/" => Cd::Root,
            ".." => Cd::Up,
            x => Cd::Down(x),
        }
    }
}

impl<'a> LsOutput<'a> {
    fn from_str(val: &'a str) -> anyhow::Result<LsOutput<'a>> {
        let res = if val.starts_with("dir ") {
            let name = &val[4..];
            LsOutput::Dir { name }
        } else {
            let (size, name) = val.split_once(' ').ok_or(err!("space expected"))?;
            let size = size.parse()?;
            LsOutput::File { size, name }
        };
        Ok(res)
    }
}

impl<'a> Path<'a> {
    fn new() -> Self {
        Path(Vec::new())
    }
    fn push(&mut self, sub: &'a str) {
        self.0.push(sub)
    }
    fn root(&mut self) {
        self.0.clear();
        self.0.push("");
    }

    fn pop(&mut self) {
        self.0.pop();
    }

    fn pathes_to_root(&'a self) -> impl Iterator<Item=String> + 'a {
        (0..self.0.len())
            .rev()
            .map(|n| self.0[0..self.0.len() - n].join("/"))
            .map(|path| if path.is_empty() {
                "/".to_string()
            } else {
                path
            })
    }
}


#[cfg(test)]
mod test {
    use super::main;

    static DATA: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn solution() {
        let res = main(DATA).expect("invalid input");
        assert_eq!(res, (95437, 24933642));
    }
}