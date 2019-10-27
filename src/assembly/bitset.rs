pub struct BitSet {
    d: [u8; 128 * 3 / 8],
}

impl BitSet {
    fn calc_idx(i1: u8, i2: u8) -> (usize, usize) {
        assert!(i1 < 128);
        assert!(i2 < 3);
        let i = i2 as usize * 128 + i1 as usize;
        (i / 8, i % 8)
    }

    pub fn set_flag(&mut self, i1: u8, i2: u8) {
        let (i, j) = Self::calc_idx(i1, i2);
        self.d[i] |= 1 << j;
    }

    pub fn is_set(&self, i1: u8, i2: u8) -> bool {
        let (i, j) = Self::calc_idx(i1, i2);
        self.d[i] & 1 << j > 0
    }
}

impl Default for BitSet {
    fn default() -> BitSet {
        BitSet {
            d: [0u8; 128 * 3 / 8],
        }
    }
}
