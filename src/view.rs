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
    size: u16,
    date: time::Date,
    time: time::Time,
}

impl Clock {
    pub fn tick<W: io::Write>(&mut self, term: &mut raw::RawTerminal<W>) -> io::Result<()> {

        let (date, time) = time::now();
        let draw = self.time ^ time;

        for digit in 0..8 {

            let dx = self.x + 1 + 4 * self.size * digit as u16;
            let dy = self.y + 1;

            let mut mask = 0b1_000_000_000_000_000u16;

            for i in 0..15 {
                mask >>= 1; if draw[digit] & mask == 0 { continue }
                let shift = cursor::Goto(i % 3 * self.size + dx, i / 3 + dy);
                let color = if time[digit] & mask > 0 { ON } else { OFF };
                let width = self.size as usize;
                write!(term, "{}{}{:3$}", shift, color, " ", width)?;
            }
        }

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
            size: 5,
            date: time::Date::default(),
            time: time::Time::default(),
        }
    }
}
