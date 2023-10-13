use std::io;
use std::io::IsTerminal as _;
use std::io::Read as _;
use std::io::Write;
use std::mem;

use crate::brush;

/// Non-canonical mode terminal.
pub struct Term {
    termios: libc::termios,
    stdin: io::StdinLock<'static>,
    stdout: io::StdoutLock<'static>,
    buffer: [u8; 1],
}

macro_rules! test {
    ($call:expr) => {
        if $call != 0 {
            return Err(io::Error::last_os_error());
        }
    };
}

impl Term {
    pub fn new() -> io::Result<Self> {
        let termios = unsafe {
            // Ensure that we have a tty device
            if !io::stdout().is_terminal() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "[USER ERROR]: not a TTY",
                ));
            }

            // Get current settings
            let mut termios: libc::termios = mem::zeroed();
            test!(libc::tcgetattr(libc::STDIN_FILENO, &mut termios));

            // Change to non-canonical mode
            let mut set = termios;
            set.c_lflag &= !(libc::ICANON | libc::ECHO);
            set.c_cc[libc::VMIN] = 0;
            set.c_cc[libc::VTIME] = 0;
            test!(libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &set));

            // Save for restoring later
            termios
        };

        // Hold onto locks
        let stdin = io::stdin().lock();
        let mut stdout = io::stdout().lock();
        write!(stdout, "{}{}", brush::ALTERNATE, brush::HIDE)?;
        Ok(Term {
            termios,
            stdin,
            stdout,
            buffer: [0],
        })
    }

    /// Get the terminal width and height.
    pub fn size(&self) -> io::Result<(u16, u16)> {
        unsafe {
            let mut size: libc::winsize = mem::zeroed();
            test!(libc::ioctl(libc::STDIN_FILENO, libc::TIOCGWINSZ, &mut size));
            Ok((size.ws_col, size.ws_row))
        }
    }

    /// Non-blocking poll for user input.
    pub fn poll(&mut self) -> Option<char> {
        match self.stdin.read_exact(&mut self.buffer) {
            Ok(_) => Some(self.buffer[0] as char),
            Err(_) => None,
        }
    }
}

impl Write for Term {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

impl Drop for Term {
    /// Restore initial termios settings and clear the screen.
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &self.termios);
            write!(
                self.stdout,
                "{}{}{}{}",
                brush::RESET,
                brush::SHOW,
                brush::Move::default(),
                brush::MAIN,
            )
            .ok();
        }
    }
}
