use std::fmt::Write;

use chrono::prelude::*;

use crate::font;

/// Retrieves current date and time with provided formatting modifiers.
pub fn now(tz: &str, second: bool, military: bool) -> (Date, Time) {
    let dt = chrono::Local::now();
    let date = Date::new(&dt, tz);
    let time = Time::new(&dt, second, military);
    (date, time)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Date<'tz> {
    date: chrono::DateTime<Local>,
    zone: &'tz str,
}

impl<'tz> Date<'tz> {
    pub fn new(dt: &chrono::DateTime<chrono::Local>, tz: &'tz str) -> Self {
        Date { date: *dt, zone: tz }
    }

    // TODO: can we get rid of this heap allocation?
    pub fn format(&self, fmt: &str, buffer: &mut String) {
        write!(buffer, "{}", self.date.format(&fmt.replace("%Z", self.zone))).unwrap()
    }
}

impl<'tz> Default for Date<'tz> {
    fn default() -> Self {
        Date {
            date: chrono::DateTime::from_utc(
                chrono::NaiveDate::from_ymd(1, 1, 1).and_hms(1, 1, 1),
                chrono::offset::FixedOffset::east(0),
            ),
            zone: "",
        }
    }
}

/// Represents time as bitmap digits for ease of diffing and drawing.
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
        | (_, _) => unreachable!(),
        }
    }
}
