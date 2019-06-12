use std::io;

use termion::color;
use termion::cursor;
use termion::raw;

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
}

impl Clock {
    pub fn tick<W: io::Write>(&mut self, term: &mut raw::RawTerminal<W>) -> io::Result<()> {

        let (date, time) = time::now();
        let draw = self.time ^ time;

        for digit in 0..8 {

            let dx = self.x + 1 + 4 * self.w * digit as u16;
            let dy = self.y + 1;

            let mut mask = 0b1_000_000_000_000_000u16;

            for i in 0..15 {
                mask >>= 1; if draw[digit] & mask == 0 { continue }
                let color = if time[digit] & mask > 0 { ON } else { OFF };
                let width = self.w as usize;
                let x = i % 3 * self.w + dx;
                let y = i / 3 * self.h + dy;
                for j in 0..self.h {
                    let shift = cursor::Goto(x, y + j);
                    write!(term, "{}{}{:3$}", shift, color, " ", width)?;
                }
            }
        }

        let w = (self.w * 4 << 3) - 1;
        let h = self.h * 5;

        let x = w / 2 - (4 + 1 + 2 + 1 + 2) / 2 + self.x;
        let y = h + self.h * 2 + self.y;

        write!(
            term,
            "{}{}{:4}-{:02}-{:02}",
            cursor::Goto(x, y),
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
}

impl Default for Clock {
    fn default() -> Self {
        Clock {
            x: 1,
            y: 1,
            w: 2,
            h: 1,
            date: time::Date::default(),
            time: time::Time::default(),
        }
    }
}
