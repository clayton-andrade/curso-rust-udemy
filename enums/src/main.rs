#![allow(dead_code)]

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CmykColor { cyan: u8, magenta: u8, yellow: u8, black: u8 },
}

fn main() {
    let c = Color::CmykColor { cyan: 29, magenta: 123, yellow: 5, black: 200 };
    match c {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::RgbColor(0, 0, 0) | Color::CmykColor { cyan: _, magenta: _, yellow: _, black: 255 } => println!("black"),
        Color::RgbColor(255, 255, 255) => println!("white"),
        Color::RgbColor(r, g, b) => println!("RgbColor({}, {}, {})", r, g, b),
        Color::CmykColor { cyan: c, magenta: m, yellow: y, black: b } => println!("CmykColor({}, {}, {}, {})", c, m, y, b),
    }
}
