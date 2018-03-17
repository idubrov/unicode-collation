use std::ops::{Index, IndexMut};
use std::u16;

/// Two stage lookup table for unicode code points.
pub struct MultiStage<T> {
    shift: u8,
    /// 1-based offset into blocks of size `1 << shift` stored in `low`.
    high: Vec<u16>,
    /// Blocks of size `1 << shift`
    low: Vec<T>,
}

impl<T: Default> MultiStage<T> {
    pub fn new(shift: u8) -> Self {
        assert!(shift < 16);
        Self {
            shift,
            high: Vec::new(),
            low: Vec::new(),
        }
    }

    /// Returns an entry for a given code point. If now entry was created before, a new
    /// entry is created. Returns mutable reference to the record.
    pub fn entry(&mut self, index: u32) -> &mut T {
        let (low_idx, high_idx) = self.indices(index);

        if self.high.len() <= high_idx {
            self.high.resize(high_idx + 1, u16::MAX);
        }

        if self.high[high_idx] == u16::MAX {
            // Reserve new block and get its index
            let len = self.low.len();
            let block_size = 1 << self.shift;
            self.low.reserve(len + block_size);
            for _ in 0..block_size {
                self.low.push(Default::default());
            }
            self.high[high_idx] = (len >> self.shift) as u16;
        }

        let block = self.high[high_idx] as usize;
        &mut self.low[(block << self.shift) + low_idx]
    }

    pub fn get(&self, index: u32) -> Option<&T> {
        let (low_idx, high_idx) = self.indices(index);

        self.high
            .get(high_idx)
            .map(|v| *v as usize)
            .and_then(|block| self.low.get((block << self.shift) + low_idx))
    }
}

impl<T> MultiStage<T> {
    fn indices(&self, index: u32) -> (usize, usize) {
        let high_idx = (index >> self.shift) as usize;
        let low_idx = (index & ((1 << self.shift) - 1)) as usize;
        (low_idx, high_idx)
    }
}

impl<T> Index<u32> for MultiStage<T> {
    type Output = T;

    fn index(&self, index: u32) -> &Self::Output {
        let (low_idx, high_idx) = self.indices(index);

        let block = self.high[high_idx] as usize;
        &self.low[(block << self.shift) + low_idx]
    }
}

impl<T> IndexMut<u32> for MultiStage<T> {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        let (low_idx, high_idx) = self.indices(index);

        let block = self.high[high_idx] as usize;
        &mut self.low[(block << self.shift) + low_idx]
    }
}
