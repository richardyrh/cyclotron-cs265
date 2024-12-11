// An input port to the simulator
// TODO: having a timestamp requires this to eventually become a priority queue, to enable
// enqueueing entries N cycles in advance

use std::marker::PhantomData;
use log::debug;

#[derive(Default)]
pub struct InputPort {}

#[derive(Default)]
pub struct OutputPort {}

#[derive(Default)]
pub struct Port<D, T> {
    valid: bool,
    // time: u64,
    data: T, // FIXME
    direction: PhantomData<D>,
}

impl<D, T: Default> Port<D, T> {
    pub fn new() -> Port<D, T> {
        Port {
            valid: false,
            // time: 0,
            data: T::default(),
            direction: PhantomData
        }
    }

    // returns true if port was ready and put succeeded.
    pub fn put(&mut self, data: T/*, time: u64*/) -> bool {
        debug!("putting on port with valid {}", self.valid);
        if self.valid {
            return false;
        }
        self.data = data;
        // self.time = time;
        self.valid = true;
        true
    }

    pub fn get(&mut self) -> Option<&T> {
        debug!("getting from port with valid {}", self.valid);
        if self.valid {
            self.valid = false;
            Some(&self.data)
        } else {
            None
        }
    }
}
