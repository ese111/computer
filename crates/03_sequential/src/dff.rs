use gates::bit::Bit;

/// D Flip-Flop: 1비트 정보를 저장하는 최소 단위.
/// 클락(tick)이 발생할 때만 상태가 업데이트됩니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DFF {
    current: Bit,
    next: Bit,
}

impl DFF {
    pub const fn new() -> Self {
        Self {
            current: Bit::Zero,
            next: Bit::Zero,
        }
    }

    /// 현재 저장되어 출력되고 있는 값을 반환합니다.
    pub fn out(&self) -> Bit {
        self.current
    }

    /// 다음 클락에 저장될 값을 예약합니다.
    pub fn set_next(&mut self, bit: Bit) {
        self.next = bit;
    }

    /// 클락 신호: 예약된 'next' 값을 'current'로 옮깁니다.
    /// 실제 하드웨어의 클락 엣지(Edge) 동작을 시뮬레이션합니다.
    pub fn tick(&mut self) {
        self.current = self.next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates::bit::Bit::*;

    #[test]
    fn test_dff_logic() {
        let mut dff = DFF::new();
        
        // 1. 초기 상태는 0
        assert_eq!(dff.out(), Zero);

        // 2. 다음 값을 1로 설정해도 출력은 여전히 0 (기억 유지)
        dff.set_next(One);
        assert_eq!(dff.out(), Zero);

        // 3. tick()을 호출해야 비로소 1로 업데이트됨
        dff.tick();
        assert_eq!(dff.out(), One);

        // 4. 다시 0으로 설정해도 tick() 전에는 1 유지
        dff.set_next(Zero);
        assert_eq!(dff.out(), One);
        dff.tick();
        assert_eq!(dff.out(), Zero);
    }
}
