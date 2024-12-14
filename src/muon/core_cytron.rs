use crate::base::{behavior::*, component::*, port::*, state::HasState};
use crate::base::mem::{MemRequest, MemResponse};
use crate::muon::config::MuonConfig;
use crate::muon::decode::*;
use crate::muon::execute::*;
use crate::muon::scheduler::Scheduler;
use crate::muon::warp::Warp;

#[derive(Default)]
pub struct MuonState {
    pub(crate) core_id: usize
}

pub struct MuonCoreCytron {
    pub base: ComponentBase<MuonState, MuonConfig, ()>,
    pub scheduler: Scheduler,
    pub warps: Vec<Warp>,
    pub imem_req: Port<OutputPort, MemRequest>,
    pub imem_resp: Port<InputPort, MemResponse>,
}

impl Resets for MuonCoreCytron {
    fn reset(&mut self) {
        self.scheduler.reset();
        self.warps.iter_mut().for_each(|w| w.reset());
    }
}
impl HasState for MuonCoreCytron {} // default impl is ok
impl Stalls for MuonCoreCytron {} // default impl is ok (for now)

impl IsComponent<MuonState, MuonConfig, ()> for MuonCoreCytron {
    base_boilerplate!(MuonState, MuonConfig, ());
}

impl Parameterizable<MuonConfig> for MuonCoreCytron {
    fn conf(&self) -> &MuonConfig {
        self.base_ref().config.c.get_or_init(|| MuonConfig::default())
    }

    fn init_conf(&mut self, c: MuonConfig) {
        self.base().config.c.set(c).unwrap()
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

impl MuonCoreCytron {
    pub fn new() -> MuonCoreCytron {
        MuonCoreCytron {
            base: ComponentBase::default(),
            scheduler: Scheduler::default(),
            warps: vec![],
            imem_req: Default::default(),
            imem_resp: Default::default(),
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
