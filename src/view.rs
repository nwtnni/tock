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
#[derive(Clone, Debug, Default)]
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

        for (dx, digit) in (0..8).map(|digit| (digit * 4 * self.size, digit as usize)) {

            let mut mask = 0b1_000_000_000_000_000u16;

            for (x, y) in (0..15).map(|index| (index % 3 + dx + self.x, index / 3 + self.y)) {
                mask >>= 1;
                if draw[digit] & mask == 0 { continue }
                let next = cursor::Goto(x, y);
                let color = if time[digit] & mask > 0 { ON } else { OFF };
                let width = self.size as usize;
                write!(term, "{}{}{:3$}", next, color, " ", width)?;
            }

        }

        self.date = date;
        self.time = time;
        Ok(())
    }
}
