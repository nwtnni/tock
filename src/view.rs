use std::io;
use std::fmt::Write;

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
    color: brush::Paint,
    center: bool,
    second: bool,
    military: bool,
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
    ) -> Self {
        Clock {
            x, y,
            w, h,
            date: time::Date::default(),
            time: time::Time::blank(second, military),
            zone,
            brush: brush::Brush::default(),
            color: brush::Paint { color, ground: brush::Ground::Back },
            center,
            second,
            military,
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
        self.color = brush::Paint { color, ground: brush::Ground::Back };
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

        let (date, time) = time::now(&self.zone, self.second, self.military);
        let draw = self.time ^ time;

        // Scan through each digit
        for digit in 0..self.digits() {

            let dx = self.x + ((font::W + 1) * self.w * digit as u16);
            let dy = self.y;

            // Scan through all bits in digit
            let mut mask = 0b1_000_000_000_000_000u16;

            for i in 0..15 {
                mask >>= 1;
                if draw[digit] & mask == 0 { continue }
                if time[digit] & mask > 0 {
                    self.brush.set(self.color)
                } else {
                    self.brush.reset()
                };
                let width = self.w as usize;
                let x = i % font::W * self.w + dx;
                let y = i / font::W * self.h + dy;
                for j in 0..self.h {
                    let goto = brush::Move(x, y + j);
                    write!(out, "{}{}{:3$}", self.brush, goto, " ", width)?;
                }
            }
        }

        // Only write date if it has changed
        if date != self.date { self.draw_date(&date, &mut out)?; }

        out.flush()?;
        self.date = date;
        self.time = time;
        Ok(())
    }

    /// Efficiently redraws the entire clock display.
    pub fn reset<W: io::Write>(&mut self, mut out: W) -> io::Result<()> {

        let (date, time) = time::now(self.zone, self.second, self.military);

        self.brush.reset();
        write!(out, "{}{}", self.brush, brush::CLEAR)?;

        // Scan through each row
        for y in 0..font::H {

            self.buffer.clear();

            // Scan through each digit
            for digit in 0..self.digits() {
                let width = self.w as usize;
                let mut mask = 1 << ((font::H - y) * font::W);
                for _ in 0..font::W {
                    mask >>= 1;
                    if time[digit] & mask > 0 {
                        self.brush.set(self.color)
                    } else {
                        self.brush.reset()
                    }
                    write!(&mut self.buffer, "{}{:2$}", self.brush, " ", width).unwrap();
                }
                self.brush.reset();
                write!(&mut self.buffer, "{}{:2$}", self.brush, " ", width).unwrap();
            }

            for i in 0..self.h {

                // Move to beginning of line
                let x = self.x;
                let y = self.y + y * self.h + i;
                write!(out, "{}{}", brush::Move(x, y), self.buffer)?;

            }
        }

        self.draw_date(&date, &mut out)?;
        out.flush()?;
        self.date = date;
        self.time = time;
        Ok(())
    }

    fn draw_date<W: io::Write>(&mut self, date: &time::Date, out: &mut W) -> io::Result<()> {
        let date_x = self.x + self.width() / 2 - date.width() / 2;
        let date_y = self.y + self.height() + 1;
        let goto = brush::Move(date_x, date_y);
        self.brush.reset();
        write!(out, "{}{}{}", self.brush, goto, date)
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
        (self.h * font::H)
    }
}
