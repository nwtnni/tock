use std::error;
use std::io;
use std::mem;
use std::ptr;
use std::sync::atomic;

use structopt::StructOpt;

mod font;
mod term;
mod time;
mod view;

#[cfg(feature = "timezone")]
pub use chrono_tz as zone;

#[cfg(not(feature = "timezone"))]
mod zone;

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

    /// Change the color of the time.
    ///
    /// Accepts either a [single 8-bit number][0] or three
    /// comma-separated 8-bit numbers in R,G,B format. Does
    /// not check if your terminal supports the entire range of
    /// 8-bit or 24-bit colors.
    ///
    /// [0]: https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit
    #[structopt(short = "C", long = "color", default_value = "2")]
    color: term::Color,

    /// Change time zone.
    ///
    /// Refer to the [Time Zone Database][0] and its [repository][1]
    /// for official time zone names.
    ///
    /// [0]: http://www.iana.org/time-zones
    ///
    /// [1]: https://github.com/eggert/tz
    #[structopt(short = "z", long = "timezone")]
    zone: Option<zone::Tz>,
}

static FINISH: atomic::AtomicBool = atomic::AtomicBool::new(false);
static RESIZE: atomic::AtomicBool = atomic::AtomicBool::new(false);

extern "C" fn set_finish(_: libc::c_int) {
    FINISH.store(true, atomic::Ordering::Relaxed)
}

extern "C" fn set_resize(_: libc::c_int) {
    RESIZE.store(true, atomic::Ordering::Relaxed)
}

macro_rules! test {
    ($call:expr) => {
        if $call != 0 {
            return Err(Box::new(io::Error::last_os_error()))
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {

    unsafe {
        let mut action: libc::sigaction = mem::zeroed();
        action.sa_flags |= libc::SA_RESTART;
        test!(libc::sigemptyset(&mut action.sa_mask as _));

        let finish = libc::sigaction { sa_sigaction: set_finish as _, .. action };
        let resize = libc::sigaction { sa_sigaction: set_resize as _, .. action };

        test!(libc::sigaction(libc::SIGINT, &finish, ptr::null::<libc::sigaction>() as _));
        test!(libc::sigaction(libc::SIGTERM, &finish, ptr::null::<libc::sigaction>() as _));
        test!(libc::sigaction(libc::SIGWINCH, &resize, ptr::null::<libc::sigaction>() as _));
    }

    let args = Opt::from_args();
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut term = term::Term::new(&mut stdin, &mut stdout)?;
    let mut clock = view::Clock::start(
        args.x,
        args.y,
        args.w,
        args.h,
        args.zone,
        args.color,
        args.second,
        args.military,
    )?;

    // Draw immediately for responsiveness
    let mut size = term.size()?;
    if args.center { clock.center(size) }
    clock.reset(&mut term)?;
    clock.draw(&mut term)?;

    'main: while !FINISH.load(atomic::Ordering::Relaxed) {

        let mut dirty = false;

        if RESIZE.load(atomic::Ordering::Relaxed) {
            RESIZE.store(false, atomic::Ordering::Relaxed);
            size = term.size()?;
            dirty = true;
        }

        #[cfg(feature = "interactive")]
        while let Some(c) = term.poll() {
            match c {
            | 'q' | 'Q' | '\x1B' => break 'main,
            | 's' => { dirty = true; clock.toggle_second(); }
            | 'm' => { dirty = true; clock.toggle_military(); }
            | _ => (),
            }
        }

        clock.sync();

        if dirty {
            if args.center { clock.center(size) }
            clock.reset(&mut term)?;
        }

        clock.draw(&mut term)?;
    }

    Ok(())
}
