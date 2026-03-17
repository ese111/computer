use crate::register::Register16;
use gates::bit::Bit;
use gates::bus::Bus;
use combinational::adders::adder16;
use combinational::mux::mux16;

/// Program Counter (PC): 다음에 실행할 명령어의 주소를 저장하는 16비트 레지스터입니다.
/// 기능: Reset(초기화), Load(점프), Inc(1 증가)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PC {
    register: Register16,
}

impl PC {
    pub const fn new() -> Self {
        Self {
            register: Register16::new(),
        }
    }

    /// 현재 저장된 주소를 반환합니다.
    pub fn out(&self) -> Bus<16> {
        self.register.out()
    }

    /// PC의 동작을 결정합니다. (우선순위: Reset > Load > Inc)
    /// - reset: PC를 0으로 설정
    /// - load: 입력받은 'input' 주소로 점프
    /// - inc: 현재 주소를 1 증가
    pub fn update(&mut self, input: Bus<16>, load: Bit, inc: Bit, reset: Bit) {
        let current_out = self.out();
        
        // 1. Increment 로직: current + 1
        let incremented = adder16(current_out, Bus::from_u16(1));
        
        // 2. 우선순위에 따른 선택 로직 (Mux의 체인)
        // (1) inc가 켜져 있으면 증가시킨 값을 선택, 아니면 현재 값 유지
        let mut next_val = mux16(current_out, incremented, inc);
        // (2) load가 켜져 있으면 외부 입력(input) 값을 선택
        next_val = mux16(next_val, input, load);
        // (3) reset이 켜져 있으면 0을 선택 (가장 높은 우선순위)
        next_val = mux16(next_val, Bus::from_u16(0), reset);

        // 결정된 최종 값을 레지스터에 저장 (무조건 저장해야 하므로 load=1)
        self.register.update(next_val, Bit::One);
    }

    /// 클락 신호: 상태를 업데이트합니다.
    pub fn tick(&mut self) {
        self.register.tick();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates::bit::Bit::*;

    #[test]
    fn test_pc_logic() {
        let mut pc = PC::new();
        let target_addr = Bus::from_u16(100);

        // 1. 초기값 0 확인
        assert_eq!(pc.out().to_u16(), 0);

        // 2. Increment: 1 증가
        pc.update(target_addr, Zero, One, Zero);
        pc.tick();
        assert_eq!(pc.out().to_u16(), 1);

        // 3. 다시 Increment: 2
        pc.update(target_addr, Zero, One, Zero);
        pc.tick();
        assert_eq!(pc.out().to_u16(), 2);

        // 4. Load (Jump): 100으로 점프
        pc.update(target_addr, One, Zero, Zero);
        pc.tick();
        assert_eq!(pc.out().to_u16(), 100);

        // 5. Jump된 상태에서 Increment: 101
        pc.update(target_addr, Zero, One, Zero);
        pc.tick();
        assert_eq!(pc.out().to_u16(), 101);

        // 6. Reset: 0으로 초기화
        pc.update(target_addr, One, One, One); // 모든 신호가 켜져도 Reset이 이김
        pc.tick();
        assert_eq!(pc.out().to_u16(), 0);
    }
}
