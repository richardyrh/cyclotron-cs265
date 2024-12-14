use std::sync::OnceLock;

pub trait Ticks {
    fn tick_one(&mut self);
    fn tick(&mut self, cycles: u64) {
        for _ in 0..cycles {
            self.tick_one()
        }
    }
}

pub trait Stalls {
    fn is_stalled(&self) -> bool {
        false
    }
}

pub trait Resets {
    fn reset(&mut self) {}
}

#[derive(Default)]
pub struct Parameters<T> {
    pub c: OnceLock<T>,
}

pub trait IsParameters {}
impl<T> IsParameters for Parameters<T> {}

pub trait Parameterizable<T> {
    fn conf(&self) -> &T;
    fn init_conf(&mut self, c: T);
}

/*
impl<T> Parameterizable<T> for Parameters<T> {
    fn conf(&self) -> &T {
        self.v.get().expect("trying to get configuration before initialization")
    }

    fn init_conf(&mut self, c: T) {
        self.v.get_or_init(|| c);
    }
}
*/