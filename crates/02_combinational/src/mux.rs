use gates::bit::Bit;
use gates::gates::{not, and, or};
use gates::bus::Bus;

/// Multiplexor: sel이 0이면 a, 1이면 b를 반환.
/// (a & !sel) | (b & sel)
pub fn mux(a: Bit, b: Bit, sel: Bit) -> Bit {
    or(and(a, not(sel)), and(b, sel))
}

/// Demultiplexor: sel에 따라 입력을 a 또는 b로 분배.
/// a = in & !sel, b = in & sel
pub fn dmux(in_bit: Bit, sel: Bit) -> (Bit, Bit) {
    (and(in_bit, not(sel)), and(in_bit, sel))
}

/// 16비트 Mux
pub fn mux16(a: Bus<16>, b: Bus<16>, sel: Bit) -> Bus<16> {
    let mut result = [Bit::Zero; 16];
    for i in 0..16 {
        result[i] = mux(a.get(i), b.get(i), sel);
    }
    Bus::new(result)
}

/// 4-way 16비트 Mux
pub fn mux4way16(a: Bus<16>, b: Bus<16>, c: Bus<16>, d: Bus<16>, sel: [Bit; 2]) -> Bus<16> {
    let mux_ab = mux16(a, b, sel[0]);
    let mux_cd = mux16(c, d, sel[0]);
    mux16(mux_ab, mux_cd, sel[1])
}

/// 8-way 16비트 Mux
pub fn mux8way16(
    a: Bus<16>, b: Bus<16>, c: Bus<16>, d: Bus<16>,
    e: Bus<16>, f: Bus<16>, g: Bus<16>, h: Bus<16>,
    sel: [Bit; 3]
) -> Bus<16> {
    let mux_abcd = mux4way16(a, b, c, d, [sel[0], sel[1]]);
    let mux_efgh = mux4way16(e, f, g, h, [sel[0], sel[1]]);
    mux16(mux_abcd, mux_efgh, sel[2])
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates::bit::Bit::*;

    #[test]
    fn test_mux() {
        assert_eq!(mux(Zero, One, Zero), Zero);
        assert_eq!(mux(Zero, One, One), One);
    }

    #[test]
    fn test_dmux() {
        assert_eq!(dmux(One, Zero), (One, Zero));
        assert_eq!(dmux(One, One), (Zero, One));
    }

    #[test]
    fn test_mux4way16() {
        let a = Bus::from_u16(1);
        let b = Bus::from_u16(2);
        let c = Bus::from_u16(3);
        let d = Bus::from_u16(4);
        
        assert_eq!(mux4way16(a, b, c, d, [Zero, Zero]).to_u16(), 1);
        assert_eq!(mux4way16(a, b, c, d, [One, Zero]).to_u16(), 2);
        assert_eq!(mux4way16(a, b, c, d, [Zero, One]).to_u16(), 3);
        assert_eq!(mux4way16(a, b, c, d, [One, One]).to_u16(), 4);
    }
}
