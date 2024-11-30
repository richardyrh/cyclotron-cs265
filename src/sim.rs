use std::collections::VecDeque;

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

    fn get(&mut self) -> Option<u64> {
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
    fn put(&mut self, data: u64) -> bool {
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

pub struct Sim {
    cycle: u64,
    counter: u64,
    pub imem_req: OutputPort,
    pub imem_resp: InputPort,
}

impl Sim {
    pub fn new() -> Sim {
        Sim {
            cycle: 0,
            counter: 0,
            imem_req: OutputPort::new(),
            imem_resp: InputPort::new(),
        }
    }

    pub fn tick(&mut self) {
        println!("tick! cycle={}", self.cycle);

        if self.imem_req.put(self.counter) {
            self.counter += 4;
        }

        // bogus logic that process responses
        match self.imem_resp.get() {
            Some(d) => {
                println!("rust: received response: data={}", d);
            }
            None => {}
        }

        self.cycle += 1;
    }
}
