use crate::bit::Bit;

/// NAND: 유일한 원시 게이트. 모든 다른 게이트는 이것으로 만든다.
/// | a | b | nand |
/// |---|---|------|
/// | 0 | 0 |  1   |
/// | 0 | 1 |  1   |
/// | 1 | 0 |  1   |
/// | 1 | 1 |  0   |
pub fn nand(a: Bit, b: Bit) -> Bit {
    match (a, b) {
        (Bit::One, Bit::One) => Bit::Zero,
        _ => Bit::One,
    }
}

/// NOT: 입력을 반전. nand(a, a)로 구현.
pub fn not(a: Bit) -> Bit {
    nand(a, a)
}

/// AND: 둘 다 1일 때만 1. nand의 결과를 반전.
pub fn and(a: Bit, b: Bit) -> Bit {
    not(nand(a, b))
}

/// OR: 하나라도 1이면 1. 드모르간 법칙 적용.
pub fn or(a: Bit, b: Bit) -> Bit {
    nand(not(a), not(b))
}

/// NOR: 둘 다 0일 때만 1.
pub fn nor(a: Bit, b: Bit) -> Bit {
    not(or(a, b))
}

/// XOR: 두 입력이 다를 때만 1.
pub fn xor(a: Bit, b: Bit) -> Bit {
    let nand_ab = nand(a, b);
    nand(nand(a, nand_ab), nand(b, nand_ab))
}

#[cfg(test)]
mod tests {
    use super::*;
    use Bit::*;

    #[test]
    fn test_nand() {
        assert_eq!(nand(Zero, Zero), One);
        assert_eq!(nand(Zero, One), One);
        assert_eq!(nand(One, Zero), One);
        assert_eq!(nand(One, One), Zero);
    }

    #[test]
    fn test_not() {
        assert_eq!(not(Zero), One);
        assert_eq!(not(One), Zero);
    }

    #[test]
    fn test_and() {
        assert_eq!(and(Zero, Zero), Zero);
        assert_eq!(and(Zero, One), Zero);
        assert_eq!(and(One, Zero), Zero);
        assert_eq!(and(One, One), One);
    }

    #[test]
    fn test_or() {
        assert_eq!(or(Zero, Zero), Zero);
        assert_eq!(or(Zero, One), One);
        assert_eq!(or(One, Zero), One);
        assert_eq!(or(One, One), One);
    }

    #[test]
    fn test_nor() {
        assert_eq!(nor(Zero, Zero), One);
        assert_eq!(nor(Zero, One), Zero);
        assert_eq!(nor(One, Zero), Zero);
        assert_eq!(nor(One, One), Zero);
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor(Zero, Zero), Zero);
        assert_eq!(xor(Zero, One), One);
        assert_eq!(xor(One, Zero), One);
        assert_eq!(xor(One, One), Zero);
    }
}
