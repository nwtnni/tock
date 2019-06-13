use std::io;

use termion::clear;
use termion::color;
use termion::cursor;
use termion::raw;

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
pub struct Clock {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
    date: time::Date,
    time: time::Time,
    second: bool,
}

impl Clock {
    pub fn new(x: u16, y: u16, w: u16, h: u16, second: bool) -> Self {
        Clock {
            x, y,
            w, h,
            date: time::Date::default(),
            time: time::Time::default(),
            second,
        }
    }

    pub fn center(&mut self, w: u16, h: u16) {
        self.x = w / 2 - self.width() / 2;
        self.y = h / 2 - self.height() / 2;
    }

    pub fn reset<W: io::Write>(&mut self, term: &mut raw::RawTerminal<W>) -> io::Result<()> {
        self.date = time::Date::default();
        self.time = time::Time::default();
        write!(term, "{}", clear::All)
    }

    pub fn tick<W: io::Write>(&mut self, term: &mut raw::RawTerminal<W>) -> io::Result<()> {

        let (date, time) = time::now();
        let draw = self.time ^ time;

        for digit in 0..self.digits() {

            let dx = self.x + 1 + ((font::DIGIT_W + 1) * self.w * digit as u16);
            let dy = self.y + 1;

            let mut mask = 0b1_000_000_000_000_000u16;

            for i in 0..15 {
                mask >>= 1; if draw[digit] & mask == 0 { continue }
                let color = if time[digit] & mask > 0 { ON } else { OFF };
                let width = self.w as usize;
                let x = i % font::DIGIT_W * self.w + dx;
                let y = i / font::DIGIT_W * self.h + dy;
                for j in 0..self.h {
                    let goto = cursor::Goto(x, y + j);
                    write!(term, "{}{}{:3$}", goto, color, " ", width)?;
                }
            }
        }

        let date_x = self.x + 1 + self.width() / 2 - 5;
        let date_y = self.y + 1 + self.height() + 2;

        write!(
            term,
            "{}{}{:4}-{:02}-{:02}",
            cursor::Goto(date_x, date_y),
            OFF,
            date.y,
            date.m,
            date.d,
        )?;

        term.flush()?;
        self.date = date;
        self.time = time;
        Ok(())
    }

    fn digits(&self) -> usize {
        if self.second { 8 } else { 5 }
    }

    pub fn width(&self) -> u16 {
        (self.w * (font::DIGIT_W + 1)) * self.digits() as u16 - 1
    }

    pub fn height(&self) -> u16 {
        (self.h * font::DIGIT_H)
    }
}
