/**
 * Copyright (c) 2024, Lin Jiang (@Injng)
 *
 * Themes file that contains constants for colors.
 **/

use sdl2::pixels::Color;

// elementary colors
pub const BLACK: Color = Color::RGB(30, 30, 46);
pub const WHITE: Color = Color::RGB(205, 214, 244);
pub const ROSEWATER: Color = Color::RGB(245, 224, 220);
pub const FLAMINGO: Color = Color::RGB(242, 205, 205);
pub const PINK: Color = Color::RGB(245, 194, 231);
pub const MAUVE: Color = Color::RGB(203, 166, 247);
pub const RED: Color = Color::RGB(243, 139, 168);
pub const MAROON: Color = Color::RGB(235, 160, 172);
pub const PEACH: Color = Color::RGB(250, 179, 135);
pub const YELLOW: Color = Color::RGB(249, 226, 175);
pub const GREEN: Color = Color::RGB(166, 227, 161);
pub const TEAL: Color = Color::RGB(148, 226, 213);
pub const SKY: Color = Color::RGB(137, 220, 235);
pub const SAPPHIRE: Color = Color::RGB(116, 199, 236);
pub const BLUE: Color = Color::RGB(137, 180, 250);
pub const LAVENDER: Color = Color::RGB(180, 190, 254);

// use cases
pub const BACKGROUND: Color = BLACK;
pub const FOREGROUND: Color = WHITE;
pub const CELL: [Color; 14] = [LAVENDER, BLUE, SAPPHIRE, SKY, TEAL, GREEN, YELLOW, PEACH, MAROON, RED, MAUVE, PINK, FLAMINGO, ROSEWATER];

