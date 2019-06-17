use std::env;
use std::error;
use std::io;
use std::mem;
use std::ptr;
use std::sync::atomic;

use structopt::StructOpt;

mod brush;
mod font;
mod term;
mod time;
mod view;

/// A digital clock for the terminal, inspired by tty-clock.
///
/// Defaults to 12-hour local time, no seconds, in the top left corner.
#[derive(Debug, StructOpt)]
#[structopt(name = "tock", about = "A digital clock for the terminal.")]
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
    color: brush::Color,

    /// Change the date format.
    ///
    /// Accepts a format string using [strftime][0] notation. Note
    /// that occurrences of the `%Z` specifier are naively replaced
    /// with the contents of the `TZ` environment variable, or the
    /// string "Local" if `TZ` is not set.
    ///
    /// [0]: https://docs.rs/chrono/0.4.6/chrono/format/strftime/index.html
    #[structopt(short = "f", long = "format", default_value = "%A, %B %m, %Y | %Z")]
    format: String,
}

/// Signal flag for interrupts.
static FINISH: atomic::AtomicBool = atomic::AtomicBool::new(false);

/// Signal flag for window size changes.
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
        // Initialize sigaction struct
        let mut action: libc::sigaction = mem::zeroed();
        action.sa_flags |= libc::SA_RESTART;
        test!(libc::sigemptyset(&mut action.sa_mask as _));

        // Copy with respective sigaction function pointers
        let finish = libc::sigaction { sa_sigaction: set_finish as _, .. action };
        let resize = libc::sigaction { sa_sigaction: set_resize as _, .. action };
        let null = ptr::null::<libc::sigaction>() as _;

        // Set signal handlers
        test!(libc::sigaction(libc::SIGINT, &finish, null));
        test!(libc::sigaction(libc::SIGTERM, &finish, null));
        test!(libc::sigaction(libc::SIGWINCH, &resize, null));
    }

    let args = Opt::from_args();
    let zone = env::var("TZ");
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut term = term::Term::new(&mut stdin, &mut stdout)?;
    let mut clock = view::Clock::new(
        args.x,
        args.y,
        args.w,
        args.h,
        zone.as_ref()
            .map(String::as_str)
            .unwrap_or("Local"),
        args.color,
        args.center,
        args.second,
        args.military,
        args.format,
    );

    // Draw immediately for responsiveness
    let mut size = term.size()?;
    clock.resize(size);
    clock.reset(&mut term)?;

    'main: while !FINISH.load(atomic::Ordering::Relaxed) {

        let mut dirty = false;

        if RESIZE.load(atomic::Ordering::Relaxed) {
            RESIZE.store(false, atomic::Ordering::Relaxed);
            dirty = true;
            size = term.size()?;
            clock.resize(size);
        }

        #[cfg(feature = "interactive")]
        while let Some(c) = term.poll() {
            match c {
            | 'q' | 'Q' | '\x1B' => break 'main,
            | 's' => {
                dirty = true;
                clock.toggle_second();
                clock.resize(size);
            }
            | 'm' => {
                dirty = true;
                clock.toggle_military();
                clock.resize(size);
            }
            | '0' ..= '7' => {
                dirty = true; 
                clock.set_color(brush::Color::C8(brush::C8(c as u8 - 48)));
            }
            | _ => (),
            }
        }

        if dirty { clock.reset(&mut term)?; }
        clock.sync();
        clock.update(&mut term)?;
    }

    Ok(())
}
