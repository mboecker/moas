pub struct BitSet {
    d: Vec<u8>,
    set_bit: usize,
}

impl BitSet {
    fn calc_idx(i: usize) -> (usize, usize) {
        (i / 8, i % 8)
    }

    pub fn set_flag(&mut self, i: usize) {
        self.set_bit += !self.is_set(i) as usize;
        let (i, j) = Self::calc_idx(i);
        self.d[i] |= 1 << j;
    }

    pub fn unset_flag(&mut self, i: usize) {
        self.set_bit -= self.is_set(i) as usize;
        let (i, j) = Self::calc_idx(i);
        self.d[i] &= !(1 << j);
    }

    pub fn is_set(&self, i: usize) -> bool {
        let (i, j) = Self::calc_idx(i);
        self.d[i] & 1 << j > 0
    }

    pub fn empty(n: usize) -> BitSet {
        BitSet {
            d: std::iter::repeat(0x00).take(n / 8 + 1).collect(),
            set_bit: 0,
        }
    }

    pub fn full(n: usize) -> BitSet {
        BitSet {
            d: std::iter::repeat(0xFF).take(n / 8 + 1).collect(),
            set_bit: n,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.set_bit == 0
    }
}

#[test]
fn bitset_empty() {
    let mut bs = BitSet::empty(10);

    assert!(bs.is_empty());

    for i in 0..10 {
        assert!(!bs.is_set(i));
    }

    for i in 0..10 {
        bs.set_flag(i);
        for b in &bs.d {
            println!("{:b}", b);
        }
        println!();
        for j in 0..10 {
            if j <= i {
                assert!(bs.is_set(j));
            } else {
                assert!(!bs.is_set(j));
            }
        }
    }
}

#[test]
fn bitset_full() {
    let mut bs = BitSet::full(10);

    for i in 0..10 {
        assert!(bs.is_set(i));
    }

    for i in 0..10 {
        bs.unset_flag(i);
        for j in 0..10 {
            if j > i {
                assert!(bs.is_set(j));
            } else {
                assert!(!bs.is_set(j));
            }
        }
    }
}

#[test]
fn bitset_seq_empty() {
    for i in 0..10 {
        let mut bs = BitSet::empty(10);
        bs.set_flag(i);

        for j in 0..10 {
            if j == i {
                assert!(bs.is_set(j));
            } else {
                assert!(!bs.is_set(j));
            }
        }
    }
}

#[test]
fn bitset_seq_empty_combinations() {
    for i in 0..10 {
        let mut bs = BitSet::empty(10);
        bs.set_flag(i);

        for k in 0..10 {
            if k != i {
                bs.set_flag(k);

                for j in 0..10 {
                    if j == i || j == k {
                        assert!(bs.is_set(j));
                    } else {
                        assert!(!bs.is_set(j));
                    }
                }

                bs.unset_flag(k);
            }
        }
    }
}

#[test]
fn bitset_seq_full() {
    for i in 0..10 {
        let mut bs = BitSet::full(10);
        bs.unset_flag(i);

        for j in 0..10 {
            if j != i {
                assert!(bs.is_set(j));
            } else {
                assert!(!bs.is_set(j));
            }
        }
    }
}

#[test]
fn bitset_seq_full_combinations() {
    for i in 0..10 {
        let mut bs = BitSet::full(10);
        bs.unset_flag(i);

        for k in 0..10 {
            if k != i {
                bs.unset_flag(k);

                for j in 0..10 {
                    if j == i || j == k {
                        assert!(!bs.is_set(j));
                    } else {
                        assert!(bs.is_set(j));
                    }
                }

                bs.set_flag(k);
            }
        }
    }
}

#[test]
fn bitset_full_empty() {
    let mut bs = BitSet::full(10);
    for j in 0..10 {
        bs.unset_flag(j);
    }
    assert!(bs.is_empty());
}
