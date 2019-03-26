use std::fmt;

pub enum Dice {
    D4(u8),
    D6(u8),
    D8(u8),
    D10(u8),
    D12(u8),
    D20(u8),
    D100(u8),
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Dice::*;
        match self {
            D4(x) => write!(f, "{}d4", x),
            D6(x) => write!(f, "{}d6", x),
            D8(x) => write!(f, "{}d8", x),
            D10(x) => write!(f, "{}d10", x),
            D12(x) => write!(f, "{}d12", x),
            D20(x) => write!(f, "{}d20", x),
            D100(x) => write!(f, "{}d100", x),
        }
    }
}
