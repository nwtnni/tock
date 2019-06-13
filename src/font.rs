/// Height of a single character in bits.
pub const H: u16 = 5;

/// Width of a single character in bits.
pub const W: u16 = 3;

// ...
// .x.
// ...
// .x.
// ...
/// Bitmap colon character.
pub const COLON: u16 = 0b0_000_010_000_010_000;

// .x.
// x.x
// xxx
// x.x
// x.x
/// Bitmap A character.
pub const A: u16 = 0b0_010_101_111_101_101;

// xx.
// x.x
// xx.
// x..
// x..
/// Bitmap P character.
pub const P: u16 = 0b0_110_101_110_100_100;

// x.x
// xxx
// x.x
// x.x
// x.x
/// Bitmap M character.
pub const M: u16 = 0b0_101_111_101_101_101;

/// Bitmap digit font. Uses lower 15 bits to store each 5x3 digit.
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
