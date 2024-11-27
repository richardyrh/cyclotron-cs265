use std::collections::VecDeque;

pub struct Sim {
    cycle: u64,
    imem_req: VecDeque<u64>,
    imem_resp: VecDeque<u64>,
}

impl Sim {
    pub fn new() -> Sim {
        Sim {
            cycle: 0,
            imem_req: VecDeque::new(),
            imem_resp: VecDeque::new(),
        }
    }

    pub fn tick(&mut self) {
        println!("tick! cycle={}", self.cycle);

        self.push_imem_req(self.cycle + 42);

        self.cycle += 1;
    }

    fn push_imem_req(&mut self, data: u64) {
        self.imem_req.push_back(data);
    }

    pub fn push_imem_resp(&mut self, data: u64) {
        self.imem_resp.push_back(data);
    }

    pub fn pop_imem_resp(&mut self) -> Option<u64> {
        self.imem_resp.pop_front()
    }

}
