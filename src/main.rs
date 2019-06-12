use std::error;
use std::io;
use std::io::Write;
use std::alloc;

use termion::clear;
use termion::color;
use termion::cursor;
use termion::raw::IntoRawMode;

mod time;
mod view;

#[global_allocator]
static A: alloc::System = alloc::System;

fn main() -> Result<(), Box<dyn error::Error>> {
    let sleep = std::time::Duration::from_secs(1);

    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut clock = view::Clock::default();

    stdout.activate_raw_mode()?;

    write!(&mut stdout, "{}{}", clear::All, cursor::Hide)?;

    for _ in 0..10 {
        clock.tick(&mut stdout)?;
        std::thread::sleep(sleep);
    }

    write!(
        &mut stdout,
        "{}{}{}{}",
        color::Bg(color::Reset),
        clear::All,
        cursor::Show,
        cursor::Goto(1, 1),
    )?;

    Ok(())
}
