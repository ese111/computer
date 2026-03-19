use crate::bit::Bit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bus<const N: usize>([Bit; N]);

impl<const N: usize> Bus<N> {
    pub const fn new(bits: [Bit; N]) -> Self {
        Self(bits)
    }

    pub fn get(&self, index: usize) -> Bit {
        self.0[index]
    }

    pub fn set(&mut self, index: usize, bit: Bit) {
        self.0[index] = bit;
    }

    pub fn bits(&self) -> &[Bit; N] {
        &self.0
    }
}

impl From<u16> for Bus<16> {
    fn from(value: u16) -> Self {
        let mut bits = [Bit::Zero; 16];
        for i in 0..16 {
            if (value >> i) & 1 == 1 {
                bits[i] = Bit::One;
            }
        }
        Self(bits)
    }
}

impl From<Bus<16>> for u16 {
    fn from(bus: Bus<16>) -> Self {
        let mut value = 0u16;
        for i in 0..16 {
            if bus.0[i] == Bit::One {
                value |= 1 << i;
            }
        }
        value
    }
}

impl<const N: usize> Default for Bus<N> {
    fn default() -> Self {
        Self([Bit::Zero; N])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bus_u16() {
        let val = 0b1010_1100_1111_0000u16;
        let bus = Bus::from(val);
        assert_eq!(u16::from(bus), val);
    }
}
