// An input port to the simulator
// TODO: having a timestamp requires this to eventually become a priority queue, to enable
// enqueueing entries N cycles in advance

use std::marker::PhantomData;
use std::sync::{Arc, OnceLock, RwLock};
use log::debug;

#[derive(Default)]
pub struct InputPort {}

#[derive(Default)]
pub struct OutputPort {}

#[derive(Default)]
pub struct Port<D, T> {
    valid: OnceLock<Arc<RwLock<bool>>>,
    // time: u64,
    data: T,
    direction: PhantomData<D>,
}

impl<D, T: Clone> Clone for Port<D, T> {
    fn clone(&self) -> Self {
        Self {
            valid: OnceLock::new(),
            data: self.data.clone(),
            direction: self.direction,
        }
    }
}

impl<D, T: Default> Port<D, T> {
    pub fn new() -> Port<D, T> {
        Port {
            valid: OnceLock::new(),
            data: T::default(),
            direction: PhantomData
        }
    }

    pub fn valid(&self) -> bool {
        self.valid.get().expect("port lock not set").read().expect("rw lock poisoned").clone()
    }
}

impl<OutputPort, T: Default> Port<OutputPort, T> {
    pub fn blocked(&self) -> bool {
        self.valid()
    }

    // returns true if port was ready and put succeeded.
    pub fn put(&mut self, data: T/*, time: u64*/) -> bool {
        if self.blocked() {
            return false;
        }
        // self.time = time;
        *self.valid.get().expect("lock not set").write().expect("lock poisoned") = true;
        self.data = data;
        true
    }
}

impl<InputPort, T: Default> Port<InputPort, T> {
    pub fn peek(&self) -> Option<&T> {
        self.valid().then_some(&self.data)
    }

    pub fn get(&mut self) -> Option<&T> {
        self.valid().then(|| {
            *self.valid.get().expect("lock not set").write().expect("lock poisoned") = false;
            &self.data
        })
    }
}

/// transfers data from an output port to an input port of the same type,
/// by giving them the same valid boolean
pub fn link<A, B, T: Default + Clone>
    (a: &mut Port<A, T>, b: &mut Port<B, T>) -> Arc<RwLock<bool>> {

    let lock = Arc::new(RwLock::new(false));
    a.valid.set(lock.clone()).expect("lock already set");
    b.valid.set(lock.clone()).expect("lock already set");
    lock
}

pub fn link_vec<A, B, T: Default + Clone>
    (a: &mut Vec<&mut Port<A, T>>, b: &mut Vec<&mut Port<B, T>>) -> Vec<Arc<RwLock<bool>>>{

    a.iter_mut().zip(b.iter_mut()).map(|(o, i)| link(o, i)).collect::<Vec<_>>()
}
