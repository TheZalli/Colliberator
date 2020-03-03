use std::fmt;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// A basic color of the rainbow
pub enum BaseColor {
    Black,
    Grey,
    White,
    Red,
    Yellow,
    Green,
    Cyan,
    Blue,
    Magenta,
}

impl Default for BaseColor {
    fn default() -> Self {
        BaseColor::Black
    }
}

impl fmt::Display for BaseColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::BaseColor::*;

        write!(
            f,
            "{}",
            match *self {
                Black => "black",
                Grey => "grey",
                White => "white",
                Red => "red",
                Yellow => "yellow",
                Green => "green",
                Cyan => "cyan",
                Blue => "blue",
                Magenta => "magenta",
            }
        )
    }
}
