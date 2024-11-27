use std::collections::VecDeque;

const MAX_QUEUE_LEN: usize = 4;

pub struct Sim {
    cycle: u64,
    counter: u64,
    imem_req: VecDeque<u64>,
    imem_resp: VecDeque<u64>,
}

impl Sim {
    pub fn new() -> Sim {
        Sim {
            cycle: 0,
            counter: 0,
            imem_req: VecDeque::new(),
            imem_resp: VecDeque::new(),
        }
    }

    pub fn tick(&mut self) {
        println!("tick! cycle={}", self.cycle);

        if self.push_imem_req(self.counter) {
            self.counter += 4;
        }

        // bogus logic that process responses
        match self.imem_resp.pop_front() {
            Some(d) => {
                println!("rust: received response: data={}", d);
            }
            None => {}
        }

        self.cycle += 1;
    }

    // returns false if the request queue was full
    fn push_imem_req(&mut self, data: u64) -> bool {
        if self.imem_req.len() >= MAX_QUEUE_LEN {
            assert!(
                self.imem_req.len() == MAX_QUEUE_LEN,
                "request queue overrun!"
            );
            return false;
        }
        self.imem_req.push_back(data);
        return true;
    }

    pub fn peek_imem_req(&mut self) -> Option<u64> {
        match self.imem_req.front() {
            Some(r) => Some(*r),
            None => None,
        }
    }

    pub fn pop_imem_req(&mut self) -> Option<u64> {
        self.imem_req.pop_front()
    }

    pub fn push_imem_resp(&mut self, data: u64) {
        if self.imem_resp.len() >= MAX_QUEUE_LEN {
            panic!("imem response queue full!");
        }
        self.imem_resp.push_back(data);
    }
}
