use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bit {
    Zero,
    One,
}

impl From<bool> for Bit {
    fn from(b: bool) -> Self {
        if b { Bit::One } else { Bit::Zero }
    }
}

impl From<Bit> for bool {
    fn from(b: Bit) -> Self {
        match b {
            Bit::Zero => false,
            Bit::One => true,
        }
    }
}

impl From<u8> for Bit {
    fn from(v: u8) -> Self {
        if v == 0 { Bit::Zero } else { Bit::One }
    }
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Bit::Zero => write!(f, "0"),
            Bit::One => write!(f, "1"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_from_bool() {
        assert_eq!(Bit::from(false), Bit::Zero);
        assert_eq!(Bit::from(true), Bit::One);
    }

    #[test]
    fn bit_from_u8() {
        assert_eq!(Bit::from(0u8), Bit::Zero);
        assert_eq!(Bit::from(1u8), Bit::One);
        assert_eq!(Bit::from(42u8), Bit::One);
    }

    #[test]
    fn bit_display() {
        assert_eq!(format!("{}", Bit::Zero), "0");
        assert_eq!(format!("{}", Bit::One), "1");
    }
}
