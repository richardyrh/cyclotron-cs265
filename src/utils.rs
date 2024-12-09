use num_traits::{PrimInt, Unsigned};

pub trait BitSlice {
    fn sel(&self, msb: usize, lsb: usize) -> Self;
}

pub trait BitSlice64 {
    fn sel64(&self, msb: usize, lsb: usize) -> Self;
}

impl BitSlice64 for u64 {
    fn sel64(&self, msb: usize, lsb: usize) -> u64 {
        assert!(msb >= lsb, "invalid bit slice");
        let mask: u64 = if msb - lsb >= 63 {
            0xffffffffffffffffu64
        } else {
            (1u64 << (msb - lsb + 1)) - 1u64
        };
        (*self >> lsb) & mask
    }
}

impl<T: PrimInt + Unsigned + TryFrom<u64>> BitSlice for T {
    fn sel(&self, msb: usize, lsb: usize) -> T {
        let self64: u64 = self.to_u64().unwrap();
        self64.sel64(msb, lsb).try_into().map_err(|_| "").unwrap()
    }
}