
pub trait Ticks {
    fn tick_one(&mut self);
    fn tick(&mut self, cycles: u64) {
        for _ in 0..cycles {
            self.tick_one()
        }
    }
}

pub trait Stalls {
    fn is_stalled(&mut self) -> bool {
        false
    }
}

pub trait Resets {
    fn reset(&mut self) {}
}
