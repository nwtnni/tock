use std::fmt;

use chrono::prelude::*;

use crate::font;

pub fn now(tz: &Option<chrono_tz::Tz>) -> (Date, Time) {
    if let &Some(tz) = tz {
        let dt = chrono::Utc::now().with_timezone(&tz);
        let date = Date::new(&dt, tz.name());
        let time = Time::new(&dt);
        (date, time)
    } else {
        let dt = chrono::Local::now();
        let date = Date::new(&dt, "Local");
        let time = Time::new(&dt);
        (date, time)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Time([u16; 8]);

impl Time {
    fn new<T: Timelike>(time: &T) -> Self {
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Date {
    pub y: i32,
    pub m: u8,
    pub d: u8,
    pub z: &'static str,
}

impl Date {
    fn new<D: Datelike>(date: &D, zone: &'static str) -> Date {
        Date {
            y: date.year(),
            m: date.month() as u8,
            d: date.day() as u8,
            z: zone,
        }
    }

    pub fn width(&self) -> u16 {
        4 + 1 + 2 + 1 + 2 + 3 + self.z.len() as u16
    }
}

impl fmt::Display for Date {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:4}-{:02}-{:02} | {}", self.y, self.m, self.d, self.z)
    }
}
