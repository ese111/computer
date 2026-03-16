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

impl Bus<16> {
    pub fn from_u16(value: u16) -> Self {
        let mut bits = [Bit::Zero; 16];
        for i in 0..16 {
            if (value >> i) & 1 == 1 {
                bits[i] = Bit::One;
            }
        }
        Self(bits)
    }

    pub fn to_u16(&self) -> u16 {
        let mut value = 0u16;
        for i in 0..16 {
            if self.0[i] == Bit::One {
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
        let bus = Bus::from_u16(val);
        assert_eq!(bus.to_u16(), val);
    }
}
