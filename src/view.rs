use std::io;

use chrono::Timelike;
use termion::clear;
use termion::color;
use termion::cursor;

use crate::font;
use crate::time;

const ON: color::Bg<&'static dyn color::Color> = color::Bg(&color::Blue);
const OFF: color::Bg<&'static dyn color::Color> = color::Bg(&color::Reset);

//  H       :   M       :   S
// ...|...|...|...|...|...|...|...
// ...|...|...|...|...|...|...|...
// ...|...|...|...|...|...|...|...
// ...|...|...|...|...|...|...|...
// ...|...|...|...|...|...|...|...
//
//           ....-..-..
//           Y    M  S
#[derive(Clone, Debug)]
pub struct Clock<W: io::Write> {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
    date: time::Date,
    time: time::Time,
    zone: Option<chrono_tz::Tz>,
    term: W,
    center: bool,
    second: bool,
    military: bool,
}

impl<W: io::Write> Clock<W> {

    pub fn start(
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        zone: Option<chrono_tz::Tz>,
        mut term: W,
        center: bool,
        second: bool,
        military: bool,
    ) -> io::Result<Self> {
        write!(term, "{}", cursor::Hide)?;
        Ok(Clock {
            x, y,
            w, h,
            date: time::Date::blank(),
            time: time::Time::blank(second, military),
            zone,
            term,
            center,
            second,
            military,
        })
    }

    pub fn reset(&mut self, (w, h): (u16, u16)) -> io::Result<()> {
        if self.center {
            self.x = w / 2 - self.width() / 2;
            self.y = h / 2 - self.height() / 2;
        }
        self.date = time::Date::blank();
        self.time = time::Time::blank(self.second, self.military);
        write!(self.term, "{}{}", OFF, clear::All)
    }

    /// Best effort real-time synchronization.
    pub fn sync(&self) {
        let start = chrono::Local::now().nanosecond() as u64;
        let delay = std::time::Duration::from_nanos(1_000_000_000 - start);
        std::thread::sleep(delay);
    }

    pub fn draw(&mut self) -> io::Result<()> {

        let (date, time) = time::now(&self.zone, self.second, self.military);
        let draw = self.time ^ time;

        for digit in 0..self.digits() {

            let dx = 1 + self.x + ((font::W + 1) * self.w * digit as u16);
            let dy = 1 + self.y;

            let mut mask = 0b1_000_000_000_000_000u16;

            for i in 0..15 {
                mask >>= 1; if draw[digit] & mask == 0 { continue }
                let color = if time[digit] & mask > 0 { ON } else { OFF };
                let width = self.w as usize;
                let x = i % font::W * self.w + dx;
                let y = i / font::W * self.h + dy;
                for j in 0..self.h {
                    let goto = cursor::Goto(x, y + j);
                    write!(self.term, "{}{}{:3$}", color, goto, " ", width)?;
                }
            }
        }

        if date != self.date {
            let date_x = 1 + self.x + self.width() / 2 - date.width() / 2;
            let date_y = 1 + self.y + self.height() + 1;
            let goto = cursor::Goto(date_x, date_y);
            write!(self.term, "{}{}{}", OFF, goto, date)?;
        }

        self.term.flush()?;
        self.date = date;
        self.time = time;
        Ok(())
    }

    fn digits(&self) -> usize {
        time::Time::width(self.second, self.military)
    }

    pub fn width(&self) -> u16 {
        (self.w * (font::W + 1)) * self.digits() as u16 - 1
    }

    pub fn height(&self) -> u16 {
        (self.h * font::H)
    }
}

impl<W: io::Write> Drop for Clock<W> {
    fn drop(&mut self) {
        write!(
            self.term,
            "{}{}{}{}",
            color::Bg(color::Reset),
            clear::All,
            cursor::Show,
            cursor::Goto(1, 1),
        ).ok();
    }
}
