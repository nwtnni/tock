use chrono::prelude::*;

// ...
// .x.
// ...
// .x.
// ...
pub const COLON: u16 = 0b0_000_010_000_010_000;

pub const DIGIT: [u16; 10] = [
    // xxx
    // x.x
    // x.x
    // x.x
    // xxx
    0b0_111_101_101_101_111,

    // ..x
    // ..x
    // ..x
    // ..x
    // ..x
    0b0_001_001_001_001_001,

    // xxx
    // ..x
    // xxx
    // x..
    // xxx
    0b0_111_001_111_100_111,

    // xxx
    // ..x
    // xxx
    // ..x
    // xxx
    0b0_111_001_111_001_111,

    // x.x
    // x.x
    // xxx
    // ..x
    // ..x
    0b0_101_101_111_001_001,

    // xxx
    // x..
    // xxx
    // ..x
    // xxx
    0b0_111_100_111_001_111,

    // xxx
    // x..
    // xxx
    // x.x
    // xxx
    0b0_111_100_111_101_111,

    // xxx
    // ..x
    // ..x
    // ..x
    // ..x
    0b0_111_001_001_001_001,

    // xxx
    // x.x
    // xxx
    // x.x
    // xxx
    0b0_111_101_111_101_111,

    // xxx
    // x.x
    // xxx
    // ..x
    // ..x
    0b0_111_101_111_001_001,
];

pub fn now() -> (Date, Time) {
    let now = chrono::Local::now();
    let date = Date::from(&now);
    let time = Time::from(&now);
    (date, time)
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Time([u16; 6]);

impl std::ops::Index<usize> for Time {
    type Output = u16;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
        | 0 => &self.0[0],
        | 1 => &self.0[1],
        | 2 => &COLON,
        | 3 => &self.0[2],
        | 4 => &self.0[3],
        | 5 => &COLON,
        | 6 => &self.0[4],
        | 7 => &self.0[5],
        | _ => unreachable!(),
        }
    }
}

impl std::ops::BitXor for Time {
    type Output = Time;
    fn bitxor(self, rhs: Time) -> Self::Output {
        let mut time = [0; 6];
        for i in 0..6 { time[i] = self.0[i] ^ rhs.0[i]; }
        Time(time)
    }
}

impl<Tz: TimeZone> From<&DateTime<Tz>> for Time {
    fn from(time: &DateTime<Tz>) -> Self {
        let h = time.hour() as usize;
        let m = time.minute() as usize;
        let s = time.second() as usize;
        Time([
             DIGIT[h / 10], DIGIT[h % 10],
             DIGIT[m / 10], DIGIT[m % 10],
             DIGIT[s / 10], DIGIT[s % 10],
        ])
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Date {
    pub y: i32,
    pub m: u8,
    pub d: u8,
}

impl<Tz: TimeZone> From<&DateTime<Tz>> for Date {
    fn from(date: &DateTime<Tz>) -> Date {
        Date {
            y: date.year(),
            m: date.month() as u8,
            d: date.day() as u8,
        }
    }
}
