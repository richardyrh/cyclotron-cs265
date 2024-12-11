use crate::base::{behavior::*, component::*, port::*, state::HasState};
use crate::base::mem::{MemRequest, MemResponse};
use crate::muon::decode::*;
use crate::muon::execute::*;

#[derive(Default)]
pub struct MuonState {
    pc: u32,
}

pub struct MuonCoreCytron {
    pub base: ComponentBase<MuonState>,
    pub reg_file: RegFile,
    pub decode_unit: DecodeUnit,
    pub execute_unit: ExecuteUnit,

    pub imem_req: Port<OutputPort, MemRequest>,
    pub imem_resp: Port<InputPort, MemResponse>,
}

impl Resets for MuonCoreCytron {
    fn reset(&mut self) {
        self.base.state.pc = 0x80000000;
        self.reg_file.reset();
        self.execute_unit.reset();
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
        self.fetch();
        if let Some(decoded) = self.decode() {
            println!("cycle={}, pc={:08x}", self.base.cycle, self.base.state.pc);
            let writeback = self.execute(decoded);
            self.writeback(writeback);
        }

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
            reg_file: RegFile::default(),
            decode_unit: DecodeUnit,
            execute_unit: ExecuteUnit::default(),
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

    fn decode(&mut self) -> Option<DecodedInst> {
        self.imem_resp.get().map(|resp| {
            let inst_data: [u8; 8] = (*(resp.data.as_ref().unwrap().clone()))
                .try_into().expect("imem response is not 8 bytes");
            self.decode_unit.decode(inst_data, self.base.state.pc, &self.reg_file)
        })
    }

    fn execute(&mut self, decoded_inst: DecodedInst) -> Writeback {
        self.execute_unit.execute(decoded_inst)
    }

    fn writeback(&mut self, writeback: Writeback) {
        self.reg_file.write_gpr(writeback.rd_addr, writeback.rd_data);
        if let Some(pc) = writeback.set_pc {
            if pc == 0 {
                println!("simulation has probably finished, main has returned");
            }
            self.base.state.pc = pc;
        }
    }
}
