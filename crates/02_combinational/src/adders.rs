use gates::bit::Bit;
use gates::gates::{xor, and, or};
use gates::bus::Bus;

/// 반가산기: 두 비트를 더한다.
/// | a | b | sum | carry |
/// |---|---|-----|-------|
/// | 0 | 0 |  0  |   0   |
/// | 0 | 1 |  1  |   0   |
/// | 1 | 0 |  1  |   0   |
/// | 1 | 1 |  0  |   1   |
pub fn half_adder(a: Bit, b: Bit) -> (Bit, Bit) {
    (xor(a, b), and(a, b))
}

/// 전가산기: 두 비트와 이전의 올림수(carry-in)를 더한다.
pub fn full_adder(a: Bit, b: Bit, c: Bit) -> (Bit, Bit) {
    let (s1, c1) = half_adder(a, b);
    let (s2, c2) = half_adder(s1, c);
    (s2, or(c1, c2))
}

/// 16비트 리플 캐리 가산기: a + b
pub fn adder16(a: Bus<16>, b: Bus<16>) -> Bus<16> {
    let mut result = [Bit::Zero; 16];
    let mut carry = Bit::Zero;

    for i in 0..16 {
        let (sum, next_carry) = full_adder(a.get(i), b.get(i), carry);
        result[i] = sum;
        carry = next_carry;
    }

    Bus::new(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates::bit::Bit::*;

    #[test]
    fn test_half_adder() {
        assert_eq!(half_adder(Zero, Zero), (Zero, Zero));
        assert_eq!(half_adder(Zero, One), (One, Zero));
        assert_eq!(half_adder(One, Zero), (One, Zero));
        assert_eq!(half_adder(One, One), (Zero, One));
    }

    #[test]
    fn test_full_adder() {
        assert_eq!(full_adder(Zero, Zero, Zero), (Zero, Zero));
        assert_eq!(full_adder(One, One, Zero), (Zero, One));
        assert_eq!(full_adder(One, One, One), (One, One));
    }

    #[test]
    fn test_adder16() {
        let a = Bus::from_u16(1234);
        let b = Bus::from_u16(5678);
        let result = adder16(a, b);
        assert_eq!(result.to_u16(), 1234 + 5678);
    }

    #[test]
    fn test_adder16_overflow() {
        let a = Bus::from_u16(0xFFFF);
        let b = Bus::from_u16(1);
        let result = adder16(a, b);
        assert_eq!(result.to_u16(), 0); // Overflow
    }
}
