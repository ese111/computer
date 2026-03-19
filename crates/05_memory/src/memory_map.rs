use gates::bit::Bit;
use gates::bus::Bus;

/// MemoryMap: CPU가 바라보는 전체 메모리 주소 공간 (0x0000 ~ 0xFFFF).
/// 주소에 따라 실제 RAM, 화면(Screen), 키보드(Keyboard) 등으로 데이터를 전달합니다.
pub struct MemoryMap {
    // 실제 RAM: 16K (0 ~ 16383)
    pub ram: Vec<u16>,
    // 화면 버퍼 (예시): 8K (16384 ~ 24575)
    pub screen: Vec<u16>,
    // 키보드 상태 (예시): 1개 (24576)
    pub keyboard: u16,
}

impl MemoryMap {
    pub fn new() -> Self {
        Self {
            ram: vec![0u16; 16384],
            screen: vec![0u16; 8192],
            keyboard: 0,
        }
    }

    /// 특정 주소에서 값을 읽어옵니다. (라우팅)
    pub fn out(&self, address: Bus<16>) -> Bus<16> {
        let addr = address.to_u16();
        let value = if addr < 16384 {
            // 0 ~ 16383: RAM
            self.ram[addr as usize]
        } else if addr < 24576 {
            // 16384 ~ 24575: Screen
            self.screen[(addr - 16384) as usize]
        } else if addr == 24576 {
            // 24576: Keyboard
            self.keyboard
        } else {
            0
        };
        Bus::from_u16(value)
    }

    /// 특정 주소에 값을 씁니다. (라우팅)
    pub fn update(&mut self, input: Bus<16>, load: Bit, address: Bus<16>) {
        if load == Bit::Zero { return; }
        
        let addr = address.to_u16();
        let val = input.to_u16();

        if addr < 16384 {
            self.ram[addr as usize] = val;
        } else if addr < 24576 {
            self.screen[(addr - 16384) as usize] = val;
        } else if addr == 24576 {
            // 키보드는 보통 읽기 전용이지만, 예제 코드로 남겨둠
            self.keyboard = val;
        }
    }

    /// 클락 신호: 각 내부 장치들에 전파 (필요한 경우)
    pub fn tick(&mut self) {
        // 실제 하드웨어라면 여기서 화면을 갱신하거나 인터럽트를 처리합니다.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates::bit::Bit::*;

    #[test]
    fn test_memory_map() {
        let mut mem = MemoryMap::new();
        let addr_ram = Bus::from_u16(100);
        let addr_screen = Bus::from_u16(16384 + 10);
        
        // 1. RAM 쓰기 및 읽기
        mem.update(Bus::from_u16(555), One, addr_ram);
        assert_eq!(mem.out(addr_ram).to_u16(), 555);

        // 2. Screen 쓰기 및 읽기
        mem.update(Bus::from_u16(999), One, addr_screen);
        assert_eq!(mem.out(addr_screen).to_u16(), 999);

        // 3. 주소 영역 독립성 확인
        assert_ne!(mem.out(addr_ram).to_u16(), mem.out(addr_screen).to_u16());
    }
}
