use std::sync::Arc;
use crate::base::{behavior::*, component::*, port::*};
use crate::base::behavior::Parameterizable;
use crate::base::mem::{MemRequest, MemResponse};
use crate::muon::config::MuonConfig;
use crate::muon::scheduler::Scheduler;
use crate::muon::warp::Warp;

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

        me.init_conf(Arc::new(*config));
        me
    }

    fn get_children(&mut self) -> Vec<&mut dyn ComponentBehaviors> {
        todo!()
    }
);

impl ComponentBehaviors for MuonCoreCytron {
    fn tick_one(&mut self) {
        self.scheduler.tick_one();

        self.scheduler.schedule.iter_mut()
            .zip(self.warps.iter_mut().map(|w| &mut w.schedule).collect::<Vec<_>>())
            .for_each(|(o, i)| link(o, i));

        self.warps.iter_mut().for_each(ComponentBehaviors::tick_one);
        
        self.scheduler.schedule_wb.iter_mut()
            .zip(self.warps.iter_mut().map(|w| &mut w.schedule_wb).collect::<Vec<_>>())
            .for_each(|(i, o)| link(o, i));
            
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
