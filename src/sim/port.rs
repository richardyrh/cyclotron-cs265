// An input port to the simulator
pub struct InputPort {
    valid: bool,
    ready: bool,
    data: u64, // FIXME
}

impl InputPort {
    pub fn new() -> InputPort {
        InputPort {
            valid: false,
            ready: false,
            data: 0,
        }
    }

    // returns true if port was ready and put succeeded.
    pub fn put(&mut self, data: u64) -> bool {
        if !self.ready {
            return false;
        }
        self.data = data;
        self.valid = true;
        return true;
    }

    pub (super) fn get(&mut self) -> Option<u64> {
        // mark empty
        self.ready = true;
        match self.valid {
            true => Some(self.data),
            false => None
        }
    }
}

pub struct OutputPort {
    valid: bool,
    ready: bool,
    data: u64, // FIXME
}

impl OutputPort {
    pub fn new() -> OutputPort {
        OutputPort {
            valid: false,
            ready: false,
            data: 0,
        }
    }

    // returns true if port was ready and put succeeded.
    pub (super) fn put(&mut self, data: u64) -> bool {
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
            false => None
        }
    }
}
