extern crate sdl2;

use sdl2::pixels::Color;

pub const COLOR_BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

pub const COLOR_DARK_BLUE: Color = Color {
    r: 29,
    g: 43,
    b: 83,
    a: 255,
};

pub const COLOR_DARK_PURPLE: Color = Color {
    r: 126,
    g: 37,
    b: 83,
    a: 255,
};

pub const COLOR_DARK_GREEN: Color = Color {
    r: 0,
    g: 135,
    b: 81,
    a: 255,
};

pub const COLOR_BROWN: Color = Color {
    r: 171,
    g: 82,
    b: 54,
    a: 255,
};

pub const COLOR_DARK_GRAY: Color = Color {
    r: 95,
    g: 87,
    b: 79,
    a: 255,
};

pub const COLOR_LIGHT_GRAY: Color = Color {
    r: 194,
    g: 195,
    b: 199,
    a: 255,
};

pub const COLOR_WHITE: Color = Color {
    r: 255,
    g: 241,
    b: 232,
    a: 255,
};

pub const COLOR_RED: Color = Color {
    r: 255,
    g: 0,
    b: 77,
    a: 255,
};

pub const COLOR_ORANGE: Color = Color {
    r: 255,
    g: 163,
    b: 0,
    a: 255,
};

pub const COLOR_YELLOW: Color = Color {
    r: 255,
    g: 236,
    b: 39,
    a: 255,
};

pub const COLOR_GREEN: Color = Color {
    r: 0,
    g: 228,
    b: 54,
    a: 255,
};

pub const COLOR_BLUE: Color = Color {
    r: 41,
    g: 173,
    b: 255,
    a: 255,
};

pub const COLOR_INDIGO: Color = Color {
    r: 131,
    g: 118,
    b: 156,
    a: 255,
};

pub const COLOR_PINK: Color = Color {
    r: 255,
    g: 119,
    b: 168,
    a: 255,
};

pub const COLOR_PEACH: Color = Color {
    r: 255,
    g: 204,
    b: 170,
    a: 255,
};

pub const COLOR: [Color; 16] = [
    COLOR_BLACK,
    COLOR_DARK_BLUE,
    COLOR_DARK_PURPLE,
    COLOR_DARK_GREEN,
    COLOR_BROWN,
    COLOR_DARK_GRAY,
    COLOR_LIGHT_GRAY,
    COLOR_WHITE,
    COLOR_RED,
    COLOR_ORANGE,
    COLOR_YELLOW,
    COLOR_GREEN,
    COLOR_BLUE,
    COLOR_INDIGO,
    COLOR_PINK,
    COLOR_PEACH,
];
