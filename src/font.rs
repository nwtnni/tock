//! All characters are stored in the lower 15 bits of u16 integers.
//!
//! After the most significant dummy bit, the bitmaps are layed
//! out as 5x3 grids, starting from the top left and running across
//! in rows until the least significant bit at the bottom right.

/// Height of a single character in bits.
pub const H: u16 = 5;

/// Width of a single character in bits.
pub const W: u16 = 3;

// ...
// .x.
// ...
// .x.
// ...
/// Bitmap ':' character.
pub const COLON: u16 = 0b0000_0100_0001_0000;

// ...
// ...
// ...
// ...
// ...
/// Bitmap ' ' character.
pub const SPACE: u16 = 0b0000_0000_0000_0000;

// .x.
// x.x
// xxx
// x.x
// x.x
/// Bitmap 'A' character.
pub const A: u16 = 0b0010_1011_1110_1101;

// xxx
// x.x
// xxx
// x..
// x..
/// Bitmap 'P' character.
pub const P: u16 = 0b0111_1011_1110_0100;

// x.x
// xxx
// x.x
// x.x
// x.x
/// Bitmap 'M' character.
pub const M: u16 = 0b0101_1111_0110_1101;

/// Bitmap digits from '0' - '9'.
pub const DIGIT: [u16; 10] = [
    // xxx
    // x.x
    // x.x
    // x.x
    // xxx
    0b0111_1011_0110_1111,
    // .x.
    // xx.
    // .x.
    // .x.
    // xxx
    0b0010_1100_1001_0111,
    // xxx
    // ..x
    // xxx
    // x..
    // xxx
    0b0111_0011_1110_0111,
    // xxx
    // ..x
    // xxx
    // ..x
    // xxx
    0b0111_0011_1100_1111,
    // x.x
    // x.x
    // xxx
    // ..x
    // ..x
    0b0101_1011_1100_1001,
    // xxx
    // x..
    // xxx
    // ..x
    // xxx
    0b0111_1001_1100_1111,
    // xxx
    // x..
    // xxx
    // x.x
    // xxx
    0b0111_1001_1110_1111,
    // xxx
    // ..x
    // ..x
    // ..x
    // ..x
    0b0111_0010_0100_1001,
    // xxx
    // x.x
    // xxx
    // x.x
    // xxx
    0b0111_1011_1110_1111,
    // xxx
    // x.x
    // xxx
    // ..x
    // xxx
    0b0111_1011_1100_1111,
];
