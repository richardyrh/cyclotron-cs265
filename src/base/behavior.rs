use std::sync::{Arc, OnceLock};

pub trait ComponentBehaviors {
    fn tick_one(&mut self);
    fn tick(&mut self, cycles: u64) {
        for _ in 0..cycles {
            self.tick_one()
        }
    }
    fn is_stalled(&self) -> bool {
        false
    }
    fn reset(&mut self) {}
}

pub trait Parameterizable {
    type ConfigType;

    fn conf(&self) -> &Self::ConfigType;
    fn init_conf(&mut self, conf: Arc<Self::ConfigType>);
}

pub trait FullComponentBehaviors: ComponentBehaviors + Parameterizable {}

#[derive(Default)]
pub struct Parameters<C> {
    pub c: OnceLock<C>,
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