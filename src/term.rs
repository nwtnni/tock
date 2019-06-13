use std::io;
use std::mem;

pub struct Term<'main> {
    ios: libc::termios,
    out: io::StdoutLock<'main>,
}

macro_rules! test {
    ($call:expr) => {
        if $call != 0 { return Err(io::Error::last_os_error()) }
    }
}

impl<'main> Term<'main> {
    pub fn new(stdout: &'main mut io::Stdout) -> io::Result<Self> {
        unsafe {
            if libc::isatty(libc::STDIN_FILENO) != 1 {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "[USER ERROR]: expected stdin to be a TTY",
                ))
            }

            // Hold onto stdout lock
            let out = stdout.lock();

            // Retrieve previous termios settings
            let mut ios: libc::termios = mem::zeroed();
            test!(libc::tcgetattr(libc::STDIN_FILENO, &mut ios));

            // Change to canonical mode
            let mut set = ios.clone();
            set.c_lflag &= !(libc::ICANON | libc::ECHO);
            set.c_cc[libc::VMIN] = 0;
            set.c_cc[libc::VTIME] = 0;
            test!(libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &set));

            Ok(Term { ios, out })
        }
    }

    pub fn size() -> io::Result<(u16, u16)> {
        unsafe {
            let mut size: libc::winsize = mem::zeroed();
            test!(libc::ioctl(libc::STDIN_FILENO, libc::TIOCGWINSZ.into(), &mut size));
            Ok((size.ws_col as u16, size.ws_row as u16))
        }
    }
}

impl<'main> io::Write for Term<'main> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.out.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.out.flush()
    }
}

impl<'main> Drop for Term<'main> {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &self.ios);
        }
    }
}
