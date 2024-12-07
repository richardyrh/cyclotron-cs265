use super::{behavior::{Ticks, Stalls, Resets}, state::HasState};

pub struct ComponentBase<T> {
    pub cycle: u64,
    pub frequency: u64,
    pub state: T,
}

impl<T: Default> Default for ComponentBase<T> {
    fn default() -> Self {
        Self { cycle: 0, frequency: 500 << 20, state: T::default() }
    }
}

pub trait IsComponent<T: Default>: Default + Ticks + Stalls + Resets + HasState {
    fn get_base(&mut self) -> &mut ComponentBase<T>;

    fn reset(&mut self) {
        self.get_base().state = T::default()
    }
}
