use gates::bit::Bit;
use gates::bus::Bus;
use combinational::mux::{mux8way16, dmux8way};
use sequential::register::Register16;

/// RAM8: 8개의 16비트 레지스터가 들어있는 메모리 유닛.
/// 3비트 주소(address)를 사용합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RAM8 {
    registers: [Register16; 8],
}

impl RAM8 {
    pub const fn new() -> Self {
        Self {
            registers: [Register16::new(); 8],
        }
    }

    /// 주소에 해당하는 레지스터의 값을 읽습니다. (조합 회로 방식 - 즉시 리턴)
    pub fn out(&self, address: [Bit; 3]) -> Bus<16> {
        mux8way16(
            self.registers[0].out(),
            self.registers[1].out(),
            self.registers[2].out(),
            self.registers[3].out(),
            self.registers[4].out(),
            self.registers[5].out(),
            self.registers[6].out(),
            self.registers[7].out(),
            address
        )
    }

    /// 주소에 해당하는 레지스터에 값을 씁니다. (순차 회로 방식 - tick() 필요)
    pub fn update(&mut self, input: Bus<16>, load: Bit, address: [Bit; 3]) {
        // 주소에 따라 load 신호를 분배
        let (l0, l1, l2, l3, l4, l5, l6, l7) = dmux8way(load, address);
        
        self.registers[0].update(input, l0);
        self.registers[1].update(input, l1);
        self.registers[2].update(input, l2);
        self.registers[3].update(input, l3);
        self.registers[4].update(input, l4);
        self.registers[5].update(input, l5);
        self.registers[6].update(input, l6);
        self.registers[7].update(input, l7);
    }

    pub fn tick(&mut self) {
        for reg in &mut self.registers {
            reg.tick();
        }
    }
}

/// RAM64: 8개의 RAM8 유닛으로 구성된 메모리.
/// 6비트 주소(address)를 사용합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RAM64 {
    rams: [RAM8; 8],
}

impl RAM64 {
    pub const fn new() -> Self {
        Self {
            rams: [RAM8::new(); 8],
        }
    }

    pub fn out(&self, address: [Bit; 6]) -> Bus<16> {
        // 상위 3비트(address[3..6])로 RAM8을 선택
        // 하위 3비트(address[0..3])로 RAM8 내부 레지스터 선택
        let addr_high = [address[3], address[4], address[5]];
        let addr_low = [address[0], address[1], address[2]];

        mux8way16(
            self.rams[0].out(addr_low),
            self.rams[1].out(addr_low),
            self.rams[2].out(addr_low),
            self.rams[3].out(addr_low),
            self.rams[4].out(addr_low),
            self.rams[5].out(addr_low),
            self.rams[6].out(addr_low),
            self.rams[7].out(addr_low),
            addr_high
        )
    }

    pub fn update(&mut self, input: Bus<16>, load: Bit, address: [Bit; 6]) {
        let addr_high = [address[3], address[4], address[5]];
        let addr_low = [address[0], address[1], address[2]];
        
        let (l0, l1, l2, l3, l4, l5, l6, l7) = dmux8way(load, addr_high);
        
        self.rams[0].update(input, l0, addr_low);
        self.rams[1].update(input, l1, addr_low);
        self.rams[2].update(input, l2, addr_low);
        self.rams[3].update(input, l3, addr_low);
        self.rams[4].update(input, l4, addr_low);
        self.rams[5].update(input, l5, addr_low);
        self.rams[6].update(input, l6, addr_low);
        self.rams[7].update(input, l7, addr_low);
    }

    pub fn tick(&mut self) {
        for ram in &mut self.rams {
            ram.tick();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates::bit::Bit::*;

    #[test]
    fn test_ram8() {
        let mut ram = RAM8::new();
        let val = Bus::from_u16(42);
        let addr = [One, Zero, One]; // 주소 5 (binary 101)

        // 1. 초기값 0 확인
        assert_eq!(ram.out(addr).to_u16(), 0);

        // 2. 값 쓰기 시도 (load=1)
        ram.update(val, One, addr);
        ram.tick();
        assert_eq!(ram.out(addr).to_u16(), 42);

        // 3. 다른 주소는 여전히 0
        assert_eq!(ram.out([Zero, Zero, Zero]).to_u16(), 0);
    }

    #[test]
    fn test_ram64() {
        let mut ram = RAM64::new();
        let val = Bus::from_u16(12345);
        // 주소 35: 상위 100(4), 하위 011(3) -> 4*8 + 3 = 35
        let addr = [One, One, Zero, Zero, Zero, One]; 

        ram.update(val, One, addr);
        ram.tick();
        assert_eq!(ram.out(addr).to_u16(), 12345);
    }
}
