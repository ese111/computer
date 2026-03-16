use gates::bit::Bit;
use gates::gates::{not, and};

/// 3-to-8 Decoder: 3비트 입력을 받아 8개의 출력 비트 중 하나만 1로 설정한다.
pub fn decoder3to8(in_bits: [Bit; 3]) -> [Bit; 8] {
    let s0 = in_bits[0];
    let s1 = in_bits[1];
    let s2 = in_bits[2];

    let ns0 = not(s0);
    let ns1 = not(s1);
    let ns2 = not(s2);

    [
        // 000
        and(and(ns2, ns1), ns0),
        // 001
        and(and(ns2, ns1), s0),
        // 010
        and(and(ns2, s1), ns0),
        // 011
        and(and(ns2, s1), s0),
        // 100
        and(and(s2, ns1), ns0),
        // 101
        and(and(s2, ns1), s0),
        // 110
        and(and(s2, s1), ns0),
        // 111
        and(and(s2, s1), s0),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates::bit::Bit::*;

    #[test]
    fn test_decoder3to8() {
        assert_eq!(decoder3to8([Zero, Zero, Zero])[0], One);
        assert_eq!(decoder3to8([One, One, One])[7], One);
        assert_eq!(decoder3to8([Zero, One, Zero])[2], One);
    }
}
