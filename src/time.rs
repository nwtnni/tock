use std::fmt;

use chrono::prelude::*;

use crate::font;

pub fn now(tz: &Option<chrono_tz::Tz>, second: bool, military: bool) -> (Date, Time) {
    if let &Some(tz) = tz {
        let dt = chrono::Utc::now().with_timezone(&tz);
        let date = Date::new(&dt, tz.name());
        let time = Time::new(&dt, second, military);
        (date, time)
    } else {
        let dt = chrono::Local::now();
        let date = Date::new(&dt, "Local");
        let time = Time::new(&dt, second, military);
        (date, time)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Time {
    S24([u16; 8]),
    S12([u16; 11]),
    M24([u16; 5]),
    M12([u16; 8]),
}

impl Time {
    pub fn blank(second: bool, military: bool) -> Self {
        match (second, military) {
        | (true, true) => Time::S24([0; 8]),
        | (true, false) => Time::S12([0; 11]),
        | (false, true) => Time::M24([0; 5]),
        | (false, false) => Time::M12([0; 8]),
        }
    }

    pub fn width(second: bool, military: bool) -> usize {
        match (second, military) {
        | (true, true) => 8,
        | (true, false) => 11,
        | (false, true) => 5,
        | (false, false) => 8,
        }
    }

    fn new<T: Timelike>(time: &T, second: bool, military: bool) -> Self {
        use font::*;
        let m = time.minute() as usize;
        match (second, military) {
        | (true, true) => {
            let h = time.hour() as usize;
            let s = time.second() as usize;
            Time::S24([
                DIGIT[h / 10], DIGIT[h % 10], COLON,
                DIGIT[m / 10], DIGIT[m % 10], COLON,
                DIGIT[s / 10], DIGIT[s % 10],
            ])
        }
        | (true, false) => {
            let (pm, h) = time.hour12();
            let h = h as usize;
            let s = time.second() as usize;
            Time::S12([
                DIGIT[h / 10], DIGIT[h % 10], COLON, 
                DIGIT[m / 10], DIGIT[m % 10], COLON,
                DIGIT[s / 10], DIGIT[s % 10], SPACE,
                if pm { P } else { A }, M,
            ])
        }
        | (false, true) => {
            let h = time.hour() as usize;
            Time::M24([
                DIGIT[h / 10], DIGIT[h % 10], COLON,
                DIGIT[m / 10], DIGIT[m % 10],
            ])
        }
        | (false, false) => {
            let (pm, h) = time.hour12();
            let h = h as usize;
            Time::M12([
                DIGIT[h / 10], DIGIT[h % 10], COLON,
                DIGIT[m / 10], DIGIT[m % 10], SPACE,
                if pm { P } else { A }, M,
            ])
        }
        }
    }
}

impl std::ops::Index<usize> for Time {
    type Output = u16;
    fn index(&self, idx: usize) -> &Self::Output {
        match self {
        | Time::S24(t) => &t[idx],
        | Time::S12(t) => &t[idx],
        | Time::M24(t) => &t[idx],
        | Time::M12(t) => &t[idx],
        }
    }
}

impl std::ops::BitXor for Time {
    type Output = Time;
    fn bitxor(self, rhs: Time) -> Self::Output {
        macro_rules! zip {
            ($time:ident, $l:expr, $r:expr, $len:expr) => { {
                let mut diff = [0; $len];
                for i in 0..$len { diff[i] = $l[i] ^ $r[i]; }
                $time(diff)
            } }
        }
        use Time::*;
        match (self, rhs) {
        | (S24(l), S24(r)) => { zip!(S24, l, r, 8) },
        | (S12(l), S12(r)) => { zip!(S12, l, r, 11) }
        | (M24(l), M24(r)) => { zip!(M24, l, r, 5) }
        | (M12(l), M12(r)) => { zip!(M12, l, r, 8) }
        | (_, _) => S12([font::FILL; 11]),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Date {
    pub y: i32,
    pub m: u8,
    pub d: u8,
    pub z: &'static str,
}

impl Date {
    pub fn blank() -> Self {
        Date { y: 0, m: 0, d: 0, z: "" }
    }

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
