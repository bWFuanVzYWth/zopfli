use libc::{c_int, size_t, c_double};

pub const ZOPFLI_NUM_LL: size_t = 288;
pub const ZOPFLI_NUM_D: size_t = 32;

pub const ZOPFLI_WINDOW_SIZE: size_t = 32768;
pub const ZOPFLI_WINDOW_MASK: size_t = 32767; // ZOPFLI_WINDOW_SIZE - 1
pub const ZOPFLI_MAX_MATCH: size_t = 258;
pub const ZOPFLI_MIN_MATCH: size_t = 3;
pub const ZOPFLI_CACHE_LENGTH: size_t = 8;
pub const ZOPFLI_MAX_CHAIN_HITS: size_t = 8192;
pub const ZOPFLI_LARGE_FLOAT: c_double = 1E30;

const LENGTH_SYMBOL_TABLE: [c_int; 259] = [
    0, 0, 0,
    257, 258, 259, 260, 261, 262, 263, 264,
    265, 265, 266, 266, 267, 267, 268, 268,
    269, 269, 269, 269, 270, 270, 270, 270,
    271, 271, 271, 271, 272, 272, 272, 272,
    273, 273, 273, 273, 273, 273, 273, 273,
    274, 274, 274, 274, 274, 274, 274, 274,
    275, 275, 275, 275, 275, 275, 275, 275,
    276, 276, 276, 276, 276, 276, 276, 276,
    277, 277, 277, 277, 277, 277, 277, 277,
    277, 277, 277, 277, 277, 277, 277, 277,
    278, 278, 278, 278, 278, 278, 278, 278,
    278, 278, 278, 278, 278, 278, 278, 278,
    279, 279, 279, 279, 279, 279, 279, 279,
    279, 279, 279, 279, 279, 279, 279, 279,
    280, 280, 280, 280, 280, 280, 280, 280,
    280, 280, 280, 280, 280, 280, 280, 280,
    281, 281, 281, 281, 281, 281, 281, 281,
    281, 281, 281, 281, 281, 281, 281, 281,
    281, 281, 281, 281, 281, 281, 281, 281,
    281, 281, 281, 281, 281, 281, 281, 281,
    282, 282, 282, 282, 282, 282, 282, 282,
    282, 282, 282, 282, 282, 282, 282, 282,
    282, 282, 282, 282, 282, 282, 282, 282,
    282, 282, 282, 282, 282, 282, 282, 282,
    283, 283, 283, 283, 283, 283, 283, 283,
    283, 283, 283, 283, 283, 283, 283, 283,
    283, 283, 283, 283, 283, 283, 283, 283,
    283, 283, 283, 283, 283, 283, 283, 283,
    284, 284, 284, 284, 284, 284, 284, 284,
    284, 284, 284, 284, 284, 284, 284, 284,
    284, 284, 284, 284, 284, 284, 284, 284,
    284, 284, 284, 284, 284, 284, 284, 285,
];

/// Gets the symbol for the given length, cfr. the DEFLATE spec.
/// Returns symbol in range [257-285] (inclusive).
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn ZopfliGetLengthSymbol(length: c_int) -> c_int {
    LENGTH_SYMBOL_TABLE[length as usize]
}

/// Gets the symbol for the given dist, cfr. the DEFLATE spec.
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn ZopfliGetDistSymbol(dist: c_int) -> c_int {
    match dist {
        0...4 => dist - 1,
        5...6 => 4,
        7...8 => 5,
        9...12 => 6,
        13...16 => 7,
        17...24 => 8,
        25...32 => 9,
        33...48 => 10,
        49...64 => 11,
        65...96 => 12,
        97...128 => 13,
        129...192 => 14,
        193...256 => 15,
        257...384 => 16,
        385...512 => 17,
        513...768 => 18,
        769...1024 => 19,
        1025...1536 => 20,
        1537...2048 => 21,
        2049...3072 => 22,
        3073...4096 => 23,
        4097...6144 => 24,
        6145...8192 => 25,
        8193...12288 => 26,
        12289...16384 => 27,
        16385...24576 => 28,
        _ => 29,
    }
}

const LENGTH_EXTRA_BITS: [c_int; 259] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0
];

/// Gets the amount of extra bits for the given length, cfr. the DEFLATE spec.
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn ZopfliGetLengthExtraBits(l: c_int) -> c_int {
    LENGTH_EXTRA_BITS[l as usize]
}

/// Gets the amount of extra bits for the given dist, cfr. the DEFLATE spec.
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn ZopfliGetDistExtraBits(dist: c_int) -> c_int {
    if dist < 5 {
        0
    } else {
        match dist {
            5...8 => 1,
            9...16 => 2,
            17...32 => 3,
            33...64 => 4,
            65...128 => 5,
            129...256 => 6,
            257...512 => 7,
            513...1024 => 8,
            1025...2048 => 9,
            2049...4096 => 10,
            4097...8192 => 11,
            8193...16384 => 12,
            _ => 13,
        }
    }
}

/// Gets value of the extra bits for the given dist, cfr. the DEFLATE spec.
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn ZopfliGetDistExtraBitsValue(dist: c_int) -> c_int {
    if dist < 5 {
        0
    } else {
        match dist {
            5...8 => (dist - 5) & 1,
            9...16 => (dist - 9) & 3,
            17...32 => (dist - 17) & 7,
            33...64 => (dist - 33) & 15,
            65...128 => (dist - 65) & 31,
            129...256 => (dist - 129) & 63,
            257...512 => (dist - 257) & 127,
            513...1024 => (dist - 513) & 255,
            1025...2048 => (dist - 1025) & 511,
            2049...4096 => (dist - 2049) & 1023,
            4097...8192 => (dist - 4097) & 2047,
            8193...16384 => (dist - 8193) & 4095,
            _ => (dist - 16385) & 8191,
        }
    }
}

const LENGTH_EXTRA_BITS_VALUE: [c_int; 259] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 0,
    1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5,
    6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6,
    7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
    13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2,
    3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
    29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
    18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6,
    7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 0
];

/// Gets value of the extra bits for the given length, cfr. the DEFLATE spec.
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn ZopfliGetLengthExtraBitsValue(l: c_int) -> c_int {
    LENGTH_EXTRA_BITS_VALUE[l as usize]
}

const LENGTH_SYMBOL_EXTRA_BITS_TABLE: [c_int; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2,
    3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0
];

/// Gets the amount of extra bits for the given length symbol.
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn ZopfliGetLengthSymbolExtraBits(s: c_int) -> c_int {
    LENGTH_SYMBOL_EXTRA_BITS_TABLE[s as usize - 257]
}

const DIST_SYMBOL_EXTRA_BITS_TABLE: [c_int; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8,
    9, 9, 10, 10, 11, 11, 12, 12, 13, 13
];

/// Gets the amount of extra bits for the given distance symbol.
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn ZopfliGetDistSymbolExtraBits(s: c_int) -> c_int {
    DIST_SYMBOL_EXTRA_BITS_TABLE[s as usize]
}
