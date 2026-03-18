use gates::bit::Bit;
use gates::bus::Bus;
use gates::gates::{not, and, or};
use combinational::adders::adder16;
use combinational::mux::mux16;

/// 16비트 비트단위 AND 연산
fn and16(a: Bus<16>, b: Bus<16>) -> Bus<16> {
    let mut result = [Bit::Zero; 16];
    for i in 0..16 {
        result[i] = and(a.get(i), b.get(i));
    }
    Bus::new(result)
}

/// 16비트 비트단위 NOT 연산
fn not16(a: Bus<16>) -> Bus<16> {
    let mut result = [Bit::Zero; 16];
    for i in 0..16 {
        result[i] = not(a.get(i));
    }
    Bus::new(result)
}

/// ALU (Arithmetic Logic Unit)
/// 6개의 제어 신호에 따라 입력 x, y에 대해 산술/논리 연산을 수행합니다.
pub fn alu(
    x: Bus<16>, y: Bus<16>, 
    zx: Bit, nx: Bit, zy: Bit, ny: Bit, f: Bit, no: Bit
) -> (Bus<16>, Bit, Bit) {
    // 1. x 전처리
    // zx: x를 0으로 (mux(x, 0, zx))
    let x1 = mux16(x, Bus::from_u16(0), zx);
    // nx: x를 반전 (mux(x1, !x1, nx))
    let x2 = mux16(x1, not16(x1), nx);

    // 2. y 전처리
    // zy: y를 0으로
    let y1 = mux16(y, Bus::from_u16(0), zy);
    // ny: y를 반전
    let y2 = mux16(y1, not16(y1), ny);

    // 3. 연산 선택 (f)
    // f=1: x + y (adder), f=0: x & y (and)
    let out_f = mux16(and16(x2, y2), adder16(x2, y2), f);

    // 4. 후처리 (no)
    // no: 출력 반전
    let out = mux16(out_f, not16(out_f), no);

    // 5. 플래그 생성
    // NG (Negative): 최상위 비트(MSB, 15번 비트)가 1이면 음수
    let ng = out.get(15);
    
    // ZR (Zero): 모든 비트가 0이면 1
    // 모든 비트의 OR 결과가 0이어야 함
    let mut any_one = Bit::Zero;
    for i in 0..16 {
        any_one = or(any_one, out.get(i));
    }
    let zr = not(any_one);

    (out, zr, ng)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates::bit::Bit::*;

    #[test]
    fn test_alu_basic_ops() {
        let x = Bus::from_u16(10);
        let y = Bus::from_u16(20);

        // 1. 0 출력 (zx=1, nx=0, zy=1, ny=0, f=1, no=0) -> x+y=0
        let (out, zr, ng) = alu(x, y, One, Zero, One, Zero, One, Zero);
        assert_eq!(out.to_u16(), 0);
        assert_eq!(zr, One);
        assert_eq!(ng, Zero);

        // 2. 1 출력 (zx=1, nx=1, zy=1, ny=1, f=1, no=1) -> 1
        let (out, zr, ng) = alu(x, y, One, One, One, One, One, One);
        assert_eq!(out.to_u16(), 1);
        assert_eq!(zr, Zero);
        assert_eq!(ng, Zero);

        // 3. x + y (0, 0, 0, 0, 1, 0)
        let (out, _, _) = alu(x, y, Zero, Zero, Zero, Zero, One, Zero);
        assert_eq!(out.to_u16(), 30);

        // 4. x - y (y의 2의 보수를 더함 -> x + !y + 1) -> 10 - 20 = -10
        // (Hack 설계에서는 x-y는 복잡한 제어 조합으로 가능)
        // 010011 -> x-y 연산 조합
        let (out, zr, ng) = alu(x, y, Zero, One, Zero, Zero, One, One);
        assert_eq!(out.to_u16() as i16, -10);
        assert_eq!(zr, Zero);
        assert_eq!(ng, One);
    }
}
