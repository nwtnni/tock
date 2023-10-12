use std::error;
use std::io;
use std::mem;
use std::ptr;
use std::sync::atomic;

use clap::Parser;
use view::Configuration;

mod brush;
mod font;
mod term;
mod time;
mod view;

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
            return Err(Box::new(io::Error::last_os_error()));
        }
    };
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let configuration = Configuration::parse();

    unsafe {
        // Initialize sigaction struct
        let mut action: libc::sigaction = mem::zeroed();
        action.sa_flags |= libc::SA_RESTART;
        test!(libc::sigemptyset(&mut action.sa_mask as _));

        // Copy with respective sigaction function pointers
        let finish = libc::sigaction {
            sa_sigaction: set_finish as _,
            ..action
        };
        let resize = libc::sigaction {
            sa_sigaction: set_resize as _,
            ..action
        };
        let null = ptr::null::<libc::sigaction>() as _;

        // Set signal handlers
        test!(libc::sigaction(libc::SIGINT, &finish, null));
        test!(libc::sigaction(libc::SIGTERM, &finish, null));
        test!(libc::sigaction(libc::SIGWINCH, &resize, null));
    }

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut term = term::Term::new(&mut stdin, &mut stdout)?;
    let mut clock = view::Clock::new(configuration);

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
                'q' | 'Q' | '\x1B' => break 'main,
                's' => {
                    dirty = true;
                    clock.toggle_second();
                    clock.resize(size);
                }
                'm' => {
                    dirty = true;
                    clock.toggle_military();
                    clock.resize(size);
                }
                '0'..='7' => {
                    dirty = true;
                    clock.set_color(brush::Color::C8(brush::C8(c as u8 - 48)));
                }
                _ => (),
            }
        }

        if dirty {
            clock.reset(&mut term)?;
        }
        clock.sync();
        clock.update(&mut term)?;
    }

    Ok(())
}
