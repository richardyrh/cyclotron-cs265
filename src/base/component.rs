use std::sync::OnceLock;
use crate::base::behavior::{Parameterizable, Parameters};
use super::{behavior::{Ticks, Stalls, Resets}, state::HasState};

pub struct ComponentBase<T, C, P> {
    pub cycle: u64,
    pub frequency: u64,
    pub state: T,
    pub config: Parameters<C>,
    pub parent: OnceLock<Box<P>>,
}

impl<T: Default, C: Default, P> Default for ComponentBase<T, C, P> {
    fn default() -> Self {
        Self {
            cycle: 0,
            frequency: 500 << 20,
            state: T::default(),
            config: Parameters::default(),
            parent: OnceLock::new(),
        }
    }
}

pub trait IsComponent<T: 'static, C: 'static, P: 'static>: Ticks + Stalls + Resets + HasState {
    fn base(&mut self) -> &mut ComponentBase<T, C, P>;
    fn base_ref(&self) -> &ComponentBase<T, C, P>;

    fn parent(&mut self) -> &Box<P> {
        self.base().parent.get().expect("no parent configured")
    }
    fn parent_ref(&self) -> &Box<P> {
        self.base_ref().parent.get().expect("no parent configured")
    }

    fn state(&mut self) -> &mut T {
        &mut self.base().state
    }

    fn init(&mut self, conf: C, parent: Box<P>) {
        IsComponent::<T, C, P>::init_conf(self, conf);
        self.base().parent.set(parent).map_err(|_| "parent already set").unwrap();
    }

    fn conf(&self) -> &C where P: Parameterizable<C> {
        self.base_ref().config.c.get().or(
            Some(self.base_ref().parent.get().expect("cannot get config").conf())).unwrap()
    }

    fn init_conf(&mut self, conf: C) {
        self.base().config.c.set(conf).map_err(|_| "config already set").unwrap();
    }
}

macro_rules! base_boilerplate {
    ($T:ty, $C:ty, $P:ty) => {
        fn base(&mut self) -> &mut ComponentBase<$T, $C, $P> {
            &mut self.base
        }

        fn base_ref(&self) -> &ComponentBase<$T, $C, $P> {
            &self.base
        }
    };
}
pub(crate) use base_boilerplate;
