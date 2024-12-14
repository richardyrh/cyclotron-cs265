use crate::base::behavior::{Parameterizable, Resets, Stalls, Ticks};
use crate::base::component::{base_boilerplate, ComponentBase, IsComponent};
use crate::base::mem::{MemRequest, MemResponse};
use crate::base::port::{InputPort, OutputPort, Port};
use crate::base::state::HasState;
use crate::muon::core_cytron::MuonCoreCytron;
use crate::muon::config::MuonConfig;
use crate::muon::decode::{DecodeUnit, RegFile};
use crate::muon::execute::ExecuteUnit;

#[derive(Default)]
pub struct WarpState {
}

pub struct Warp {
    base: ComponentBase<WarpState, MuonConfig, MuonCoreCytron>,
    lanes: Vec<Lane>,

    pub imem_req: Port<OutputPort, MemRequest>,
    pub imem_resp: Port<InputPort, MemResponse>,
}

impl Ticks for Warp {
    fn tick_one(&mut self) {
        todo!()
    }
}

impl Stalls for Warp {}

impl Resets for Warp {
    fn reset(&mut self) {
        self.lanes.iter_mut().for_each(|lane| {
            lane.reg_file.reset();
            lane.execute_unit.reset();
        });
    }
}

impl HasState for Warp {}

impl IsComponent<WarpState, MuonConfig, MuonCoreCytron> for Warp {
    base_boilerplate!(WarpState, MuonConfig, MuonCoreCytron);
}

impl Warp {
    pub fn new() -> Warp {
        Warp {
            base: ComponentBase::default(),
            lanes: vec![],
            imem_req: Port::new(),
            imem_resp: Port::new()
        }
    }
}

pub struct Lane {
    pub reg_file: RegFile,
    pub decode_unit: DecodeUnit,
    pub execute_unit: ExecuteUnit,
}
