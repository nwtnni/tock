use std::cell;
use std::fmt;
use std::str;

/// Clear the screen.
pub const CLEAR: &str = "\x1B[2J";

/// Switch to main screen buffer.
pub const MAIN: &str = "\x1B[?1049l";

/// Switch to alternate screen buffer.
pub const ALTERNATE: &str = "\x1B[?1049h";

/// Hide the cursor.
pub const HIDE: &str = "\x1B[?25l";

/// Show the cursor.
pub const SHOW: &str = "\x1B[?25h";

/// Reset the background color.
pub const RESET: Paint = Paint {
    color: Color::Reset,
    ground: Ground::Back,
};

/// Move the cursor to 0-indexed (x, y) terminal position.
#[derive(Copy, Clone, Debug, Default)]
pub struct Move(pub u16, pub u16);

impl fmt::Display for Move {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "\x1B[{};{}H", self.1 + 1, self.0 + 1)
    }
}

/// Edge-triggered paint: will only write
/// escape code when switching paint colors.
#[derive(Clone, Debug)]
pub struct Brush {
    paint: Paint,
    dried: cell::Cell<bool>,
    on: bool,
}

impl Brush {
    pub fn new(color: Color) -> Self {
        Brush {
            paint: Paint {
                color,
                ground: Ground::Back,
            },
            dried: cell::Cell::new(true),
            on: false,
        }
    }

    pub fn dip(&mut self, color: Color) {
        let old = self.paint;
        let new = Paint {
            color,
            ground: Ground::Back,
        };
        if self.on {
            self.dried.set(old == new && self.dried.get());
        }
        self.paint = new;
    }

    pub fn raise(&mut self) {
        self.set(false)
    }

    pub fn set(&mut self, on: bool) {
        self.dried.set(on == self.on && self.dried.get());
        self.on = on;
    }
}

impl fmt::Display for Brush {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self.dried.get() {
            return Ok(());
        }
        self.dried.set(true);
        write!(fmt, "{}", if self.on { self.paint } else { RESET })
    }
}

/// Change the terminal's writing color.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Paint {
    pub color: Color,
    pub ground: Ground,
}

impl fmt::Display for Paint {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let g = self.ground as u8;
        match self.color {
            Color::C8(c) => write!(fmt, "\x1B[{};5;{}m", g, c.0),
            Color::C24(c) => write!(fmt, "\x1B[{};2;{};{};{}m", g, c.r, c.g, c.b),
            Color::Reset => write!(fmt, "\x1B[{}m", g + 1),
        }
    }
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Ground {
    Fore = 38,
    Back = 48,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    C8(C8),
    C24(C24),
    Reset,
}

impl str::FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(c) = s.parse::<u8>() {
            return Ok(Color::C8(C8(c)));
        }

        let mut it = s.split(',');

        let (r, g, b) = match (it.next(), it.next(), it.next(), it.next()) {
            (Some(r), Some(g), Some(b), None) => (r, g, b),
            _ => return Err(format!("[USER ERROR]: invalid color specifier {}", s)),
        };

        match (r.parse::<u8>(), g.parse::<u8>(), b.parse::<u8>()) {
            (Ok(r), Ok(g), Ok(b)) => Ok(Color::C24(C24 { r, g, b })),
            _ => Err(format!("[USER ERROR]: invalid color specifier {}", s)),
        }
    }
}

/// 8-bit ANSI color.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct C8(pub u8);

/// 24-bit RGB color.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct C24 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
