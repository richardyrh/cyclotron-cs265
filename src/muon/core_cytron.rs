use crate::base::{behavior::*, component::*, port::*, state::HasState};

#[derive(Default)]
pub struct MuonState {
    pc: u64,
}

#[derive(Default)]
pub struct MuonCoreCytron {
    pub base: ComponentBase<MuonState>,
    pub imem_req: Port<OutputPort, u64>,
    pub imem_resp: Port<InputPort, u64>,
}

impl Resets for MuonCoreCytron {} // default impl is ok
impl HasState for MuonCoreCytron {} // default impl is ok
impl Stalls for MuonCoreCytron {} // default impl is ok (for now)

impl IsComponent<MuonState> for MuonCoreCytron {
    fn get_base(&mut self) -> &mut ComponentBase<MuonState> {
        &mut self.base
    }
}

impl Ticks for MuonCoreCytron {
    fn tick_one(&mut self) {
        println!("tick! cycle={}", self.base.cycle);

        self.decode();
        self.fetch();

        self.base.cycle += 1;
    }
}

impl MuonCoreCytron {
    pub fn new() -> MuonCoreCytron {
        MuonCoreCytron {
            base: ComponentBase::default(),
            imem_req: Port::new(),
            imem_resp: Port::new()
        }
    }

    pub fn time(&self) -> u64 {
        self.base.cycle
    }

    fn fetch(&mut self) {
        if !self.imem_req.put(self.base.state.pc) {
            panic!("imem_req port blocked!");
        }
        self.base.state.pc += 4; // FIXME: 32-bit hardcoded
    }

    fn decode(&mut self) {
        if let Some(data) = self.imem_resp.get() {
            println!("decode: got data={}", data);
        }
    }
}
