use std::collections::HashSet;

use chrono::prelude::*;

//  H   :   M   :   S
// ...|...|...|...|...
// ...|...|...|...|...
// ...|...|...|...|...
// ...|...|...|...|...
// ...|...|...|...|...
//
//     ....-..-..
//     Y    M  S
#[derive(Clone, Debug, Default)]
pub struct View {
    x: u16,
    y: u16,
    block: u8,
    dirty: HashSet<(u16, u16)>,

    hour: u8, 
    min: u8,
    sec: u8,
    year: i32,
    mon: u8,
    day: u8,
}

impl View {
    pub fn update(&mut self) {
        let now = chrono::Local::now();
        let hour = now.hour() as u8;
        let min = now.minute() as u8;
        let sec = now.second() as u8;
        let year = now.year();
        let mon = now.month() as u8;
        let day = now.day() as u8;
    }

}
