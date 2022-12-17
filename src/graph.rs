use std::io::{self, Write};
use std::{time, thread};

pub struct Graphics;

pub type Modifier = &'static str;

pub const HIGHLIGHT: Modifier = "\x1b[1m";
pub const SHADE: Modifier = "\x1b[2m";
pub const COLOR_RED: Modifier = "\x1b[31m";
pub const COLOR_GREEN: Modifier = "\x1b[32m";
pub const COLOR_BLUE: Modifier = "\x1b[34m";

const LINES: usize = 160;

const CLR: &str = "\x1bc";

pub fn draw_map<F, D>(dx: usize, dy: usize, f: F) -> io::Result<()>
where
    F: Fn(usize) -> D,
    D: Drawable,
{
    let (dy, y_offset) = if dy <= LINES { (dy, 0) } else { (LINES, dy - LINES) };

    let mut stdout = io::stdout();
    stdout.write(CLR.as_bytes())?;
    for y in 0..dy {
        for x in 0..dx {
            let drawable = f((y + y_offset) * dx + x);
            write!(stdout, "{}{}\x1b[0m", drawable.modifier(), drawable.symbol())?;
        }
        stdout.write(b"\n")?;
    }
    stdout.flush()
}

pub fn goto_line(y: usize) -> io::Result<()> {
    write!(io::stdout(), "\x1b[{}H\n", y)
}

pub fn delay_draw_char<D>(x: usize, y: usize, drawable: D) -> io::Result<()>
where
    D: Drawable,
{
    //if y < 175 - LINES {
    //    return Ok(())
    //}
    // let y = y - (175 - LINES);
    let modifier = drawable.modifier();
    let symbol = drawable.symbol();
    thread::sleep(time::Duration::from_millis(1));
    write!(io::stdout(), "\x1b[{};{}H{}{}\x1b[0m",
        y + 1, x + 1, modifier, symbol)?;
    io::stdout().flush()
}

pub fn draw_path<I, F, D>(width: usize, path: I, f: F) -> io::Result<()>
where
    I: IntoIterator<Item=(usize, usize)>,
    F: Fn(usize) -> D,
    D: Drawable,
{
    let mut stdout = io::stdout();
    for (x, y) in path {
        let offset = y * width + x;
        let drawable = f(offset);
        let modifier = drawable.modifier();
        write!(stdout, "\x1b[{};{}H{}{}\x1b[0m",
            y + 1, x + 1, modifier, drawable.symbol())?;
        if drawable.flush() {
            stdout.flush()?;
        }
    }
    stdout.flush()
}


pub trait Drawable {
    fn symbol(&self) -> char;
    fn modifier(&self) -> Modifier;
    fn flush(&self) -> bool { false }
}

impl Drawable for char {
    fn symbol(&self) -> char { *self }
    fn modifier(&self) -> Modifier { "" }
}
impl Drawable for (char, Modifier) {
    fn symbol(&self) -> char { self.0 }
    fn modifier(&self) -> Modifier { self.1 }
}
impl Drawable for (char, bool) {
    fn symbol(&self) -> char { self.0 }
    fn modifier(&self) -> Modifier { "" }
    fn flush(&self) -> bool { self.1 }
}
impl Drawable for (char, Modifier, bool) {
    fn symbol(&self) -> char { self.0 }
    fn modifier(&self) -> Modifier { self.1 }
    fn flush(&self) -> bool { self.2 }
}
