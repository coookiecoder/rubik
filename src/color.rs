use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Blue,
    Green,
    Red,
    Orange,
    Yellow,
    White
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Color::Blue => "blue",
            Color::Green => "green",
            Color::Red => "red",
            Color::Orange => "orange",
            Color::Yellow => "yellow",
            Color::White => "white",
        };
        write!(f, "{}", name).expect("write failed");

        Ok(())
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Color::Blue => "Blue",
            Color::Green => "Green",
            Color::Red => "Red",
            Color::Orange => "Orange",
            Color::Yellow => "Yellow",
            Color::White => "White",
        };
        write!(f, "{}", name).expect("write failed");

        Ok(())
    }
}
