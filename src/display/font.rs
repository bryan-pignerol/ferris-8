const CHAR_WIDTH: usize = 4;
const CHAR_HEIGHT: usize = 5;

const NUMBER_1: [u8; 5] = [
    0b0010, //   X
    0b0110, //  XX
    0b1010, // X X
    0b0010, //   X
    0b1111, // XXXX
];

const NUMBER_2: [u8; 5] = [
    0b0110, //  XX
    0b1001, // X  X
    0b0010, //   X
    0b0100, //  X
    0b1111, // XXXX
];

const NUMBER_3: [u8; 5] = [
    0b0110, //  XX
    0b1001, // X  X
    0b0010, //   X
    0b1001, // X  X
    0b0110, //  XX
];

const NUMBER_4: [u8; 5] = [
    0b1010, // X X
    0b1010, // X X
    0b1110, // XXX
    0b0010, //   X
    0b0010, //   X
];

const NUMBER_5: [u8; 5] = [
    0b1111, // XXXX
    0b1000, // X
    0b1110, // XXX
    0b0001, //    X
    0b1110, // XXX
];

const NUMBER_6: [u8; 5] = [
    0b0111, //  XXX
    0b1000, // X
    0b1110, // XXX
    0b1001, // X  X
    0b0110, //  XX
];

const NUMBER_7: [u8; 5] = [
    0b1111, // XXXX
    0b0001, //    X
    0b0010, //   X
    0b0100, //  X
    0b1000, // X
];

const NUMBER_8: [u8; 5] = [
    0b0110, //  XX
    0b1001, // X  X
    0b0110, //  XX
    0b1001, // X  X
    0b0110, //  XX
];

const NUMBER_9: [u8; 5] = [
    0b0110, //  XX
    0b1001, // X  X
    0b0111, //  XXX
    0b0001, //    X
    0b0110, //  XX
];

const NUMBER_0: [u8; 5] = [
    0b0110, //  XX
    0b1001, // X  X
    0b1001, // X  X
    0b1001, // X  X
    0b0110, //  XX
];


pub struct Font {
    char_width: usize,
    char_height: usize
}
