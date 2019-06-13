use std::fmt;
use std::io;
use std::mem;

/// Clear the screen.
pub const CLEAR: &'static str = "\x1B[2J";

/// Hide the cursor.
pub const HIDE: &'static str = "\x1B[?25l";

/// Show the cursor.
pub const SHOW: &'static str = "\x1B[?25h";

/// Non-canonical mode terminal.
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

/// 0-indexed (x, y) terminal position.
#[derive(Copy, Clone, Debug, Default)]
pub struct Move(pub u16, pub u16);

impl fmt::Display for Move {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "\x1B[{};{}H", self.1 + 1, self.0 + 1)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Paint {
    pub color: Color,
    pub ground: Ground,
}

impl fmt::Display for Paint {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let g = self.ground as u8;
        match self.color {
        | Color::ANSI(c) => write!(fmt, "\x1B[{};5;{}m", g, c as u8),
        | Color::C256(c) => write!(fmt, "\x1B[{};5;{}m", g, c.0),
        | Color::CRGB(c) => write!(fmt, "\x1B[{};2;{};{};{}m", g, c.r, c.g, c.b),
        | Color::Reset => write!(fmt, "\x1B[{}m", g + 1),
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Ground {
    Fore = 38,
    Back = 48,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    ANSI(ANSI),
    C256(C256),
    CRGB(CRGB),
    Reset,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ANSI {
    Black         = 00,
    Red           = 01,
    Green         = 02,
    Yellow        = 03,
    Blue          = 04,
    Magenta       = 05,
    Cyan          = 06,
    White         = 07,
    BrightBlack   = 08,
    BrightRed     = 09,
    BrightGreen   = 10,
    BrightYellow  = 11,
    BrightBlue    = 12,
    BrightMagenta = 13,
    BrightCyan    = 14,
    BrightWhite   = 15,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct C256(pub u8);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CRGB {
    pub r: u8,    
    pub g: u8,
    pub b: u8,
}
