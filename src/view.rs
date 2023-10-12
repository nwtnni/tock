use std::fmt::Write;
use std::io;

use chrono::Timelike;

use crate::brush;
use crate::font;
use crate::time;

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
#[derive(Clone, Debug)]
pub struct Clock<'tz> {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
    date: time::Date<'tz>,
    time: time::Time,
    zone: &'tz str,
    brush: brush::Brush,
    center: bool,
    second: bool,
    military: bool,
    format: String,
    buffer: String,
}

impl<'tz> Clock<'tz> {
    /// Create a new clock instance.
    pub fn new(
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        zone: &'tz str,
        color: brush::Color,
        center: bool,
        second: bool,
        military: bool,
        format: String,
    ) -> Self {
        Clock {
            x,
            y,
            w,
            h,
            date: time::Date::blank(),
            time: time::Time::blank(second, military),
            zone,
            brush: brush::Brush::new(color),
            center,
            second,
            military,
            format,
            buffer: String::new(),
        }
    }

    /// Toggle second display.
    pub fn toggle_second(&mut self) {
        self.second ^= true;
    }

    /// Toggle military (24H) time.
    pub fn toggle_military(&mut self) {
        self.military ^= true;
    }

    /// Set the color of the clock's time display.
    pub fn set_color(&mut self, color: brush::Color) {
        self.brush.dip(color)
    }

    /// Adjusts the clock's position to match the provided terminal dimensions.
    pub fn resize(&mut self, (w, h): (u16, u16)) {
        if self.center {
            self.x = w / 2 - self.width() / 2;
            self.y = h / 2 - self.height() / 2;
        }
    }

    /// Sleeps until approximately the next second boundary.
    pub fn sync(&self) {
        let start = chrono::Local::now().nanosecond() as u64;
        let delay = std::time::Duration::from_nanos(1_000_000_000 - start);
        std::thread::sleep(delay);
    }

    /// Draws the differences between the previous time and the next.
    pub fn update<W: io::Write>(&mut self, mut out: W) -> io::Result<()> {
        let (date, time) = time::now(self.zone, self.second, self.military);
        let draw = self.time ^ time;

        // Scan through each digit
        for digit in 0..self.digits() {
            // Skip digits with no difference
            if draw[digit] == 0 {
                continue;
            }

            let dx = self.x + ((font::W + 1) * self.w * digit as u16);
            let dy = self.y;

            // Scan through all bits in digit
            let mut mask = 0b1000_0000_0000_0000_u16;

            for i in 0..15 {
                mask >>= 1;

                // Skip bits with no difference
                if draw[digit] & mask == 0 {
                    continue;
                }

                // Write single row into buffer
                let x = i % font::W * self.w + dx;
                let y = i / font::W * self.h + dy;
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
    pub fn reset<W: io::Write>(&mut self, mut out: W) -> io::Result<()> {
        let (date, time) = time::now(self.zone, self.second, self.military);

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
            let x = self.x;
            let y = self.y + y * self.h;

            self.render_row_buffer(x, y, &mut out)?;
        }

        self.draw_date(&date, &mut out)?;
        out.flush()?;
        self.date = date;
        self.time = time;
        Ok(())
    }

    /// Draw the current date.
    fn draw_date<W: io::Write>(&mut self, date: &time::Date, out: &mut W) -> io::Result<()> {
        self.brush.raise();
        self.buffer.clear();
        date.format(&self.format, &mut self.buffer);
        let date_x = self.x + self.width() / 2 - self.buffer.len() as u16 / 2;
        let date_y = self.y + self.height() + 1;
        let goto = brush::Move(date_x, date_y);
        write!(out, "{}{}{}", self.brush, goto, self.buffer)
    }

    /// Write a row (with current color and width) of a font bit into the buffer.
    fn write_row_buffer(&mut self) {
        write!(
            &mut self.buffer,
            "{}{:2$}",
            self.brush, " ", self.w as usize
        )
        .expect("[INTERNAL ERROR]: writing into String failed");
    }

    /// Write a complete font bit to the screen.
    /// Expects a valid row to be in the buffer.
    fn render_row_buffer<W: io::Write>(&self, x: u16, y: u16, mut out: W) -> io::Result<()> {
        for i in 0..self.h {
            write!(out, "{}{}", brush::Move(x, y + i), self.buffer)?;
        }
        Ok(())
    }

    /// Get number of characters in current time format.
    fn digits(&self) -> usize {
        time::Time::width(self.second, self.military)
    }

    /// Get current clock width in characters.
    pub fn width(&self) -> u16 {
        (self.w * (font::W + 1)) * self.digits() as u16 - 1
    }

    /// Get current clock height in characters.
    pub fn height(&self) -> u16 {
        self.h * font::H
    }
}
