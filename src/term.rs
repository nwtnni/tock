use std::io::{self, Read, Write};
use std::mem;

use crate::brush;

/// Non-canonical mode terminal.
#[allow(dead_code)]
pub struct Term<'main> {
    termios: libc::termios,
    stdin: io::StdinLock<'main>,
    stdout: io::StdoutLock<'main>,
    buffer: [u8; 1],
}

macro_rules! test {
    ($call:expr) => {
        if $call != 0 {
            return Err(io::Error::last_os_error());
        }
    };
}

impl<'main> Term<'main> {
    pub fn new(stdin: &'main mut io::Stdin, stdout: &'main mut io::Stdout) -> io::Result<Self> {
        let termios = unsafe {
            // Ensure that we have a tty device
            if libc::isatty(libc::STDIN_FILENO) != 1 || libc::isatty(libc::STDOUT_FILENO) != 1 {
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
        let stdin = stdin.lock();
        let mut stdout = stdout.lock();
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
            test!(libc::ioctl(
                libc::STDIN_FILENO,
                libc::TIOCGWINSZ,
                &mut size
            ));
            Ok((size.ws_col, size.ws_row))
        }
    }

    /// Non-blocking poll for user input.
    #[allow(dead_code)]
    pub fn poll(&mut self) -> Option<char> {
        match self.stdin.read_exact(&mut self.buffer) {
            Ok(_) => Some(self.buffer[0] as char),
            Err(_) => None,
        }
    }
}

impl<'main> io::Write for Term<'main> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

impl<'main> Drop for Term<'main> {
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
