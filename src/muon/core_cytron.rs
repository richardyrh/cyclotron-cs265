use crate::base::{behavior::*, component::*, port::*, state::HasState};
use crate::base::mem::{MemRequest, MemResponse};

#[derive(Default)]
pub struct MuonState {
    pc: u64,
}

#[derive(Default)]
pub struct MuonCoreCytron {
    pub base: ComponentBase<MuonState>,
    pub imem_req: Port<OutputPort, MemRequest>,
    pub imem_resp: Port<InputPort, MemResponse>,
}

impl Resets for MuonCoreCytron {
    fn reset(&mut self) {
        self.base.state.pc = 0x80000000;
    }
}
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

impl Parameterizable for MuonCoreCytron {
    fn get_children(&mut self) -> Vec<Box<&mut dyn Parameterizable>> {
        std::vec![]
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
        if !self.imem_req.read::<8>(self.base.state.pc as usize) {
            panic!("imem_req port blocked!");
        }
        self.base.state.pc += 8;
    }

    fn decode(&mut self) {
        if let Some(resp) = self.imem_resp.get() {
            let data = resp.data.as_ref().unwrap();
            let hex_string: String = data.iter().map(|byte| format!("{:02X}", byte)).collect::<Vec<_>>().join(" ");

            println!("decode: got data={}", hex_string);
        }
    }
}
