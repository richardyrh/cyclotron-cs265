use std::sync::Arc;
use crate::base::behavior::*;
use crate::base::component::{component_inner, ComponentBase, IsComponent};

pub struct QueueState<T, const N: usize> {
    pub storage: Vec<T>,
    size: usize,
    max_size: usize,
}

impl<T: Default, const N: usize> Default for QueueState<T, N> {
    fn default() -> Self {
        Self {
            storage: (0..N).map(|_| T::default()).collect(), // no need for Copy trait
            size: 0,
            max_size: N,
        }
    }
}

#[derive(Default)]
pub struct Queue<T, const N: usize> where T: Default {
    base: ComponentBase<QueueState<T, N>, ()>,
}

impl<T: Default, const N: usize> ComponentBehaviors for Queue<T, N> {
    fn tick_one(&mut self) {}
    fn reset(&mut self) {
        self.state().size = 0;
    }
}

impl<T: Default, const N: usize> IsComponent for Queue<T, N> {
    component_inner!(QueueState<T, N>, ());

    fn new(_: Arc<()>) -> Self {
        Queue::<T, N>::default()
    }
}

// TODO: add locks and stuff
impl<T: Default, const N: usize> Queue<T, N> {
    pub fn try_enq(&mut self, data: T) -> bool {
        let size = self.state().size;
        let max_size = self.state().max_size;
        if size >= max_size {
            return false;
        }
        self.state().storage[size] = data;
        true
    }

    pub fn try_deq(&mut self) -> Option<T> where T: Clone {
        let size = self.state().size;
        (size > 0).then(|| {
            self.state().size -= 1;
            self.state().storage[size].clone()
        })
    }

    pub fn resize(&mut self, size: usize) {
        self.state().max_size = size;
    }
}