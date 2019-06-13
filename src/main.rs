use std::error;
use std::io;
use std::io::Write;

use structopt::StructOpt;
use termion::clear;
use termion::color;
use termion::cursor;
use termion::raw::IntoRawMode;

mod font;
mod time;
mod view;

/// A tty-clock clone.
///
/// Displays a digital clock in the terminal.
#[derive(Debug, StructOpt)]
#[structopt(name = "tock", about = "A tty-clock clone.")]
struct Opt {
    /// Horizontal 0-indexed position of top-left corner.
    #[structopt(short = "x", long = "x", default_value = "1")]
    x: u16,

    /// Vertical 0-indexed position of top-left corner.
    #[structopt(short = "y", long = "y", default_value = "1")]
    y: u16,

    /// Font width in characters per tile.
    #[structopt(short = "w", long = "width", default_value = "2")]
    w: u16,

    /// Font height in characters per tile.
    #[structopt(short = "h", long = "height", default_value = "1")]
    h: u16,

    /// Display seconds.
    #[structopt(short = "s", long = "seconds")]
    second: bool,
    
    /// Center the clock in the terminal. Overrides manual positioning.
    #[structopt(short = "c", long = "center")]
    center: bool,
}

fn main() -> Result<(), Box<dyn error::Error>> {

    let args = Opt::from_args();
    let sleep = std::time::Duration::from_secs(1);

    let stdout = io::stdout().into_raw_mode()?;
    stdout.activate_raw_mode()?;

    let mut clock = view::Clock::start(
        args.x,
        args.y,
        args.w,
        args.h,
        stdout.lock(),
        args.second
    )?;

    let (mut w, mut h) = termion::terminal_size()?;
    if args.center { clock.center(w, h); }

    for _ in 0..5 {
        let (new_w, new_h) = termion::terminal_size()?;
        if w != new_w || h != new_h {
            clock.reset()?;
            clock.center(new_w, new_h);
            w = new_w; 
            h = new_h;
        }
        clock.tick()?;
        std::thread::sleep(sleep);
    }

    Ok(())
}