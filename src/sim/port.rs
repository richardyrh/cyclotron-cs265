// An input port to the simulator
// TODO: having a timestamp requires this to eventually become a priority queue, to enable
// enqueueing entries N cycles in advance
pub struct InputPort {
    valid: bool,
    ready: bool,
    time: u64,
    data: u64, // FIXME
}

impl InputPort {
    pub fn new() -> InputPort {
        InputPort {
            valid: false,
            ready: false,
            time: 0,
            data: 0,
        }
    }

    // returns true if port was ready and put succeeded.
    pub fn put(&mut self, data: u64, time: u64) -> bool {
        if !self.ready {
            return false;
        }
        self.data = data;
        self.time = time;
        self.valid = true;
        return true;
    }

    pub(super) fn get(&mut self) -> Option<u64> {
        // mark empty
        self.ready = true;
        match self.valid {
            true => Some(self.data),
            false => None,
        }
    }
}

pub struct OutputPort {
    valid: bool,
    ready: bool,
    time: u64,
    data: u64, // FIXME
}

impl OutputPort {
    pub fn new() -> OutputPort {
        OutputPort {
            valid: false,
            ready: false,
            time: 0,
            data: 0,
        }
    }

    // returns true if port was ready and put succeeded.
    pub(super) fn put(&mut self, data: u64) -> bool {
        if !self.ready {
            return false;
        }
        self.data = data;
        self.valid = true;
        return true;
    }

    pub fn get(&mut self) -> Option<u64> {
        // mark empty
        self.ready = true;
        match self.valid {
            true => Some(self.data),
            false => None,
        }
    }
}
