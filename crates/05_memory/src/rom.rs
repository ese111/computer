use gates::bus::Bus;

/// ROM: 프로그램 명령어를 담고 있는 읽기 전용 메모리.
/// CPU의 Program Counter(PC)가 가리키는 주소의 명령어를 즉시 출력합니다.
#[derive(Debug, Clone, Default)]
pub struct ROM {
    data: Vec<u16>,
}

impl ROM {
    /// 주어진 프로그램 데이터를 사용하여 ROM을 생성합니다.
    pub fn new(program: Vec<u16>) -> Self {
        Self { data: program }
    }

    /// 특정 주소의 16비트 명령어를 읽습니다. (조합 회로 방식)
    /// 주소 범위를 벗어나면 0(NOP)을 반환합니다.
    pub fn out(&self, address: Bus<16>) -> Bus<16> {
        let addr = address.to_u16() as usize;
        if addr < self.data.len() {
            Bus::from_u16(self.data[addr])
        } else {
            Bus::from_u16(0) // 0은 보통 NOP(No Operation) 명령어
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rom() {
        let program = vec![0x1234, 0xABCD, 0x5678];
        let rom = ROM::new(program);
        
        assert_eq!(rom.out(Bus::from_u16(0)).to_u16(), 0x1234);
        assert_eq!(rom.out(Bus::from_u16(1)).to_u16(), 0xABCD);
        assert_eq!(rom.out(Bus::from_u16(99)).to_u16(), 0); // Out of bounds
    }
}
