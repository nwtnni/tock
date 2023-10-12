use std::env;
use std::fmt::Write as _;
use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

use chrono::Local;
use chrono::Timelike as _;
use clap::Parser;

use crate::brush;
use crate::brush::Brush;
use crate::brush::Color;
use crate::font;
use crate::time;
use crate::time::Date;
use crate::time::Time;

/// A digital clock for the terminal, inspired by tty-clock.
///
/// Defaults to 12-hour local time, no seconds, in the top left corner.
#[derive(Parser, Debug)]
#[clap(name = "tock", about = "A digital clock for the terminal.")]
pub struct Configuration {
    /// Horizontal 0-indexed position of top-left corner.
    #[clap(short, long, default_value_t = 0)]
    x: u16,

    /// Vertical 0-indexed position of top-left corner.
    #[clap(short, long, default_value_t = 0)]
    y: u16,

    /// Font width in characters per tile.
    #[clap(short = 'W', long, default_value_t = 2)]
    width: u16,

    /// Font height in characters per tile.
    #[clap(short = 'H', long, default_value_t = 1)]
    height: u16,

    /// Display seconds.
    #[clap(short, long)]
    second: bool,

    /// Display military (24-hour) time.
    #[clap(short, long)]
    military: bool,

    /// Center the clock in the terminal. Overrides manual positioning.
    #[clap(short, long)]
    center: bool,

    /// Change the color of the time.
    ///
    /// Accepts either a [single 8-bit number][0] or three
    /// comma-separated 8-bit numbers in R,G,B format. Does
    /// not check if your terminal supports the entire range of
    /// 8-bit or 24-bit colors.
    ///
    /// [0]: https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit
    #[clap(short = 'C', long, default_value = "2")]
    color: Color,

    /// Change the date format.
    ///
    /// Accepts a format string using [strftime][0] notation. Note
    /// that occurrences of the `%Z` specifier are naively replaced
    /// with the contents of the `TZ` environment variable, or the
    /// string "Local" if `TZ` is not set.
    ///
    /// [0]: https://docs.rs/chrono/0.4.6/chrono/format/strftime/index.html
    #[clap(short, long, default_value = "%F | %Z")]
    format: String,
}

//  H       :   M       :   S
// ...|...|...|...|...|...|...|...
// ...|...|...|...|...|...|...|...
// ...|...|...|...|...|...|...|...
// ...|...|...|...|...|...|...|...
// ...|...|...|...|...|...|...|...
//
//           ....-..-..
//           Y    M  S
/// Represents a digital clock.
#[derive(Debug)]
pub struct Clock {
    configuration: Configuration,
    date: Date,
    time: Time,
    brush: Brush,
    buffer: String,
}

impl Clock {
    /// Create a new clock instance.
    pub fn new(mut configuration: Configuration) -> Self {
        let zone = env::var("TZ").unwrap_or_else(|_| String::from("Local"));
        configuration.format = configuration.format.replace("%Z", &zone);

        Clock {
            date: Date::blank(),
            time: Time::blank(configuration.second, configuration.military),
            brush: Brush::new(configuration.color),
            buffer: String::new(),
            configuration,
        }
    }

    /// Toggle second display.
    pub fn toggle_second(&mut self) {
        self.configuration.second ^= true;
    }

    /// Toggle military (24H) time.
    pub fn toggle_military(&mut self) {
        self.configuration.military ^= true;
    }

    /// Set the color of the clock's time display.
    pub fn set_color(&mut self, color: Color) {
        self.brush.dip(color)
    }

    /// Adjusts the clock's position to match the provided terminal dimensions.
    pub fn resize(&mut self, (w, h): (u16, u16)) {
        if self.configuration.center {
            self.configuration.x = w / 2 - self.width() / 2;
            self.configuration.y = h / 2 - self.height() / 2;
        }
    }

    /// Sleeps until approximately the next second boundary.
    pub fn sync(&self) {
        let start = Local::now().nanosecond() as u64;
        let delay = Duration::from_nanos(1_000_000_000 - start);
        thread::sleep(delay);
    }

    /// Draws the differences between the previous time and the next.
    pub fn update<W: Write>(&mut self, mut out: W) -> io::Result<()> {
        let (date, time) = time::now(self.configuration.second, self.configuration.military);
        let draw = self.time ^ time;

        // Scan through each digit
        for digit in 0..self.digits() {
            // Skip digits with no difference
            if draw[digit] == 0 {
                continue;
            }

            let dx =
                self.configuration.x + ((font::W + 1) * self.configuration.width * digit as u16);
            let dy = self.configuration.y;

            // Scan through all bits in digit
            let mut mask = 0b1000_0000_0000_0000_u16;

            for i in 0..15 {
                mask >>= 1;

                // Skip bits with no difference
                if draw[digit] & mask == 0 {
                    continue;
                }

                // Write single row into buffer
                let x = i % font::W * self.configuration.width + dx;
                let y = i / font::W * self.configuration.height + dy;
                self.brush.set(time[digit] & mask > 0);
                self.buffer.clear();
                self.write_row_buffer();
                self.render_row_buffer(x, y, &mut out)?;
            }
        }

        // Only write date if it has changed
        if date != self.date {
            self.draw_date(&date, &mut out)?;
        }

        out.flush()?;
        self.date = date;
        self.time = time;
        Ok(())
    }

    /// Efficiently redraws the entire clock display.
    pub fn reset<W: Write>(&mut self, mut out: W) -> io::Result<()> {
        let (date, time) = time::now(self.configuration.second, self.configuration.military);

        self.brush.raise();
        write!(out, "{}{}", self.brush, brush::CLEAR)?;

        // Scan through each row
        for y in 0..font::H {
            self.buffer.clear();

            // Scan through each digit
            for digit in 0..self.digits() {
                let mut mask = 1 << ((font::H - y) * font::W);
                for _ in 0..font::W {
                    mask >>= 1;
                    self.brush.set(time[digit] & mask > 0);
                    self.write_row_buffer();
                }
                self.brush.raise();
                self.write_row_buffer();
            }

            // Move to beginning of line
            let x = self.configuration.x;
            let y = self.configuration.y + y * self.configuration.height;

            self.render_row_buffer(x, y, &mut out)?;
        }

        self.draw_date(&date, &mut out)?;
        out.flush()?;
        self.date = date;
        self.time = time;
        Ok(())
    }

    /// Draw the current date.
    fn draw_date<W: Write>(&mut self, date: &Date, out: &mut W) -> io::Result<()> {
        self.brush.raise();
        self.buffer.clear();
        date.format(&self.configuration.format, &mut self.buffer);
        let date_x = self.configuration.x + self.width() / 2 - self.buffer.len() as u16 / 2;
        let date_y = self.configuration.y + self.height() + 1;
        let goto = brush::Move(date_x, date_y);
        write!(out, "{}{}{}", self.brush, goto, self.buffer)
    }

    /// Write a row (with current color and width) of a font bit into the buffer.
    fn write_row_buffer(&mut self) {
        write!(
            &mut self.buffer,
            "{}{:2$}",
            self.brush, " ", self.configuration.width as usize
        )
        .expect("[INTERNAL ERROR]: writing into String failed");
    }

    /// Write a complete font bit to the screen.
    /// Expects a valid row to be in the buffer.
    fn render_row_buffer<W: Write>(&self, x: u16, y: u16, mut out: W) -> io::Result<()> {
        for i in 0..self.configuration.height {
            write!(out, "{}{}", brush::Move(x, y + i), self.buffer)?;
        }
        Ok(())
    }

    /// Get number of characters in current time format.
    fn digits(&self) -> usize {
        Time::width(self.configuration.second, self.configuration.military)
    }

    /// Get current clock width in characters.
    pub fn width(&self) -> u16 {
        (self.configuration.width * (font::W + 1)) * self.digits() as u16 - 1
    }

    /// Get current clock height in characters.
    pub fn height(&self) -> u16 {
        self.configuration.height * font::H
    }
}
