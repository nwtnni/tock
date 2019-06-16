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
pub const COLON: u16 = 0b0_000_010_000_010_000;

// ...
// ...
// ...
// ...
// ...
/// Bitmap ' ' character.
pub const SPACE: u16 = 0b0_000_000_000_000_000;

// .x.
// x.x
// xxx
// x.x
// x.x
/// Bitmap 'A' character.
pub const A: u16 = 0b0_010_101_111_101_101;

// xxx
// x.x
// xxx
// x..
// x..
/// Bitmap 'P' character.
pub const P: u16 = 0b0_111_101_111_100_100;

// x.x
// xxx
// x.x
// x.x
// x.x
/// Bitmap 'M' character.
pub const M: u16 = 0b0_101_111_101_101_101;

/// Bitmap digits from '0' - '9'.
pub const DIGIT: [u16; 10] = [
    // xxx
    // x.x
    // x.x
    // x.x
    // xxx
    0b0_111_101_101_101_111,

    // .x.
    // xx.
    // .x.
    // .x.
    // xxx
    0b0_010_110_010_010_111,

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
    // xxx
    0b0_111_101_111_001_111,
];
