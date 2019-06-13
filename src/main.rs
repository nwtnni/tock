use std::error;
use std::io;
use std::sync;
use std::sync::atomic;

use structopt::StructOpt;

mod font;
mod term;
mod time;
mod view;

/// A tty-clock clone.
///
/// Displays a digital clock in the terminal.
/// Defaults to 12-hour local time, no seconds, in the top left corner.
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

    /// Display military (24-hour) time.
    #[structopt(short = "m", long = "military")]
    military: bool,
    
    /// Center the clock in the terminal. Overrides manual positioning.
    #[structopt(short = "c", long = "center")]
    center: bool,

    /// Change time zone.
    ///
    /// Refer to the [Time Zone Database][0] and its [repository][1]
    /// for official time zone names.
    ///
    /// [0]: http://www.iana.org/time-zones
    /// [1]: https://github.com/eggert/tz
    #[structopt(short = "z", long = "timezone")]
    zone: Option<chrono_tz::Tz>,
}

fn main() -> Result<(), Box<dyn error::Error>> {

    let args = Opt::from_args();
    let mut stdout = io::stdout();
    let mut term = term::Term::new(&mut stdout)?;

    let finish = sync::Arc::<atomic::AtomicBool>::default();
    let resize = sync::Arc::<atomic::AtomicBool>::default();

    signal_hook::flag::register(signal_hook::SIGINT, finish.clone())?;
    signal_hook::flag::register(signal_hook::SIGTERM, finish.clone())?;
    signal_hook::flag::register(signal_hook::SIGWINCH, resize.clone())?;

    let mut clock = view::Clock::start(
        args.x,
        args.y,
        args.w,
        args.h,
        args.zone,
        &mut term,
        args.second,
        args.military,
    )?;

    // Draw immediately for responsiveness
    let mut size = term::Term::size()?;
    if args.center { clock.center(size) }
    clock.reset()?;
    clock.draw()?;

    while !finish.load(atomic::Ordering::Relaxed) {
        if resize.load(atomic::Ordering::Relaxed) {
            resize.store(false, atomic::Ordering::Relaxed);
            size = term::Term::size()?;
            clock.reset()?;
            if args.center { clock.center(size) }
        }
        clock.sync();
        clock.draw()?;
    }

    Ok(())
}
