use std::sync::{Arc, RwLock};
use crate::base::{behavior::*, component::*, port::*};
use crate::base::mem::{MemRequest, MemResponse};
use crate::muon::config::MuonConfig;
use crate::muon::decode::*;
use crate::muon::execute::*;
use crate::muon::scheduler::Scheduler;
use crate::muon::warp::Warp;
use crate::sim::top::CyclotronTop;

#[derive(Default)]
pub struct MuonState {
    pub(crate) core_id: usize
}

#[derive(Default)]
pub struct MuonCoreCytron {
    pub base: ComponentBase<MuonState, MuonConfig>,
    pub scheduler: Scheduler,
    pub warps: Vec<Warp>,
    pub imem_req: Port<OutputPort, MemRequest>,
    pub imem_resp: Port<InputPort, MemResponse>,
}

component!(MuonCoreCytron, MuonState, MuonConfig,
    fn new(config: &MuonConfig) -> MuonCoreCytron {
        let mut me = MuonCoreCytron {
            base: Default::default(),
            scheduler: Default::default(),
            warps: vec![],
            imem_req: Default::default(),
            imem_resp: Default::default(),
        };

        let num_warps = config.num_warps;
        me.warps = (0..num_warps).map(|_| Warp::new(config)).collect();
        me.scheduler = Scheduler::new(config);

        me
    }

);

impl ComponentBehaviors for MuonCoreCytron {
    fn tick_one(&mut self) {
        self.scheduler.tick_one();
        // TODO: transfer schedule

        self.warps.iter_mut().for_each(ComponentBehaviors::tick_one);
        self.base.cycle += 1;
    }

    fn reset(&mut self) {
        self.scheduler.reset();
        self.warps.iter_mut().for_each(|w| w.reset());
    }
}

impl MuonCoreCytron {
    pub fn time(&self) -> u64 {
        self.base.cycle
    }
}
