use crate::dff::DFF;
use gates::bit::Bit;
use gates::bus::Bus;
use combinational::mux::mux16;

/// 16비트 레지스터: 16개의 DFF를 묶어 16비트 데이터를 저장합니다.
/// 'load' 신호에 따라 값을 새로 저장하거나 기존 값을 유지합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Register16 {
    dffs: [DFF; 16],
}

impl Register16 {
    pub const fn new() -> Self {
        Self {
            dffs: [DFF::new(); 16],
        }
    }

    /// 레지스터에 현재 저장되어 있는 16비트 데이터를 출력합니다.
    pub fn out(&self) -> Bus<16> {
        let mut bits = [Bit::Zero; 16];
        for i in 0..16 {
            bits[i] = self.dffs[i].out();
        }
        Bus::new(bits)
    }

    /// 레지스터의 다음 상태를 결정합니다.
    /// load가 1일 때만 'input' 값을 받아들이고, 0이면 현재 값을 유지합니다.
    pub fn update(&mut self, input: Bus<16>, load: Bit) {
        let current_out = self.out();
        // Mux를 사용하여 다음 상태 결정: if load { input } else { current_out }
        let next_val = mux16(current_out, input, load);
        
        for i in 0..16 {
            self.dffs[i].set_next(next_val.get(i));
        }
    }

    /// 클락 신호: 결정된 다음 상태로 전이합니다.
    pub fn tick(&mut self) {
        for i in 0..16 {
            self.dffs[i].tick();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates::bit::Bit::*;

    #[test]
    fn test_register16() {
        let mut reg = Register16::new();
        let val1 = Bus::from_u16(12345);
        let val2 = Bus::from_u16(54321);

        // 1. 초기값 확인
        assert_eq!(reg.out().to_u16(), 0);

        // 2. load=0인 경우: 업데이트를 시도해도 무시됨
        reg.update(val1, Zero);
        reg.tick();
        assert_eq!(reg.out().to_u16(), 0);

        // 3. load=1인 경우: 값이 저장됨
        reg.update(val1, One);
        reg.tick();
        assert_eq!(reg.out().to_u16(), 12345);

        // 4. 다시 load=0으로 바꾸고 다른 값을 넣어봄: 이전 값(12345) 유지
        reg.update(val2, Zero);
        reg.tick();
        assert_eq!(reg.out().to_u16(), 12345);
    }
}
