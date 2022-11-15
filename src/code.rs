pub const H3_1: Code = Code::Hamming(2);
pub const H7_4: Code = Code::Hamming(3);
pub const H15_11: Code = Code::Hamming(4);
pub const EH4_1: Code = Code::EHamming(3);
pub const EH8_4: Code = Code::EHamming(4);
pub const EH16_11: Code = Code::EHamming(5);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Code {
    Hamming(
        /// The number of parity bits
        u32,
    ),
    EHamming(
        /// The number of parity bits
        u32,
    ),
}

impl Code {
    #[must_use]
    pub fn from_block_size(b: u32) -> Option<Self> {
        if b < 3 {
            None
        } else if b.is_power_of_two() {
            Some(Self::EHamming(b.ilog2() + 1))
        } else if (b + 1).is_power_of_two() {
            Some(Self::Hamming((b + 1).ilog2()))
        } else {
            None
        }
    }

    #[must_use]
    pub fn block_bits(&self) -> u32 {
        match self {
            Self::Hamming(p) => 2u32.pow(*p) - 1,
            Self::EHamming(p) => 2u32.pow(p - 1),
        }
    }

    #[must_use]
    pub fn data_bits(&self) -> u32 {
        match self {
            Self::Hamming(p) => 2u32.pow(*p) - 1 - p,
            Self::EHamming(p) => 2u32.pow(p - 1) - p,
        }
    }

    #[must_use]
    pub fn parity_bits(&self) -> u32 {
        let (Self::Hamming(p) | Self::EHamming(p)) = self;
        *p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_block_bits() {
        assert_eq!(Code::from_block_bits(0), None);
        assert_eq!(Code::from_block_bits(1), None);
        assert_eq!(Code::from_block_bits(2), None);
        assert_eq!(Code::from_block_bits(3), Some(Code::Hamming(2)));
        assert_eq!(Code::from_block_bits(4), Some(Code::EHamming(3)));
        assert_eq!(Code::from_block_bits(5), None);
        assert_eq!(Code::from_block_bits(6), None);
        assert_eq!(Code::from_block_bits(7), Some(Code::Hamming(3)));
        assert_eq!(Code::from_block_bits(8), Some(Code::EHamming(4)));
    }

    #[test]
    fn block_bits() {
        assert_eq!(H3_1.block_bits(), 3);
        assert_eq!(H7_4.block_bits(), 7);
        assert_eq!(H15_11.block_bits(), 15);
        assert_eq!(EH4_1.block_bits(), 4);
        assert_eq!(EH8_4.block_bits(), 8);
        assert_eq!(EH16_11.block_bits(), 16);
    }

    #[test]
    fn data_bits() {
        assert_eq!(H3_1.data_bits(), 1);
        assert_eq!(H7_4.data_bits(), 4);
        assert_eq!(H15_11.data_bits(), 11);
        assert_eq!(EH4_1.data_bits(), 1);
        assert_eq!(EH8_4.data_bits(), 4);
        assert_eq!(EH16_11.data_bits(), 11);
    }

    #[test]
    fn parity_bits() {
        assert_eq!(H3_1.parity_bits(), 2);
        assert_eq!(H7_4.parity_bits(), 3);
        assert_eq!(H15_11.parity_bits(), 4);
        assert_eq!(EH4_1.parity_bits(), 3);
        assert_eq!(EH8_4.parity_bits(), 4);
        assert_eq!(EH16_11.parity_bits(), 5);
    }
}
