use chrono::prelude::*;

use crate::font;

pub fn now() -> (Date, Time) {
    let now = chrono::Local::now();
    let date = Date::from(&now);
    let time = Time::from(&now);
    (date, time)
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Time([u16; 8]);

impl std::ops::Index<usize> for Time {
    type Output = u16;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl std::ops::BitXor for Time {
    type Output = Time;
    fn bitxor(self, rhs: Time) -> Self::Output {
        let mut time = [0; 8];
        for i in 0..8 { time[i] = self.0[i] ^ rhs.0[i]; }
        Time(time)
    }
}

impl<Tz: TimeZone> From<&DateTime<Tz>> for Time {
    fn from(time: &DateTime<Tz>) -> Self {
        let h = time.hour() as usize;
        let m = time.minute() as usize;
        let s = time.second() as usize;
        Time([
             font::DIGIT[h / 10],
             font::DIGIT[h % 10],
             font::COLON,
             font::DIGIT[m / 10],
             font::DIGIT[m % 10],
             font::COLON,
             font::DIGIT[s / 10],
             font::DIGIT[s % 10],
        ])
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
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
