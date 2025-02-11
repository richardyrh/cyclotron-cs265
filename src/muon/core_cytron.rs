use std::sync::Arc;
use log::info;
use crate::base::{behavior::*, component::*, port::*};
use crate::base::behavior::Parameterizable;
use crate::base::mem::{MemRequest, MemResponse};
use crate::muon::config::{LaneConfig, MuonConfig};
use crate::muon::scheduler::Scheduler;
use crate::muon::warp::Warp;
use crate::utils::fill;

#[derive(Default)]
pub struct MuonState {}

#[derive(Default)]
pub struct MuonCoreCytron {
    pub base: ComponentBase<MuonState, MuonConfig>,
    pub scheduler: Scheduler,
    pub warps: Vec<Warp>,
    pub imem_req: Vec<Port<OutputPort, MemRequest>>,
    pub imem_resp: Vec<Port<InputPort, MemResponse>>,
}

component!(MuonCoreCytron, MuonState, MuonConfig,
    fn new(config: Arc<MuonConfig>) -> MuonCoreCytron {
        let num_warps = config.num_warps;
        let mut me = MuonCoreCytron {
            base: Default::default(),
            scheduler: Scheduler::new(config.clone()),
            warps: (0..num_warps).map(|warp_id| Warp::new(Arc::new(MuonConfig {
                lane_config: LaneConfig {
                    warp_id,
                    ..config.lane_config
                },
                ..*config
            }))).collect(),
            imem_req: fill!(Port::new(), num_warps),
            imem_resp: fill!(Port::new(), num_warps),
        };

        let sched_out = &mut me.scheduler.schedule.iter_mut().collect();
        let sched_in = &mut me.warps.iter_mut().map(|w| &mut w.schedule).collect();
        link_vec(sched_out, sched_in);

        let sched_wb_in = &mut me.scheduler.schedule_wb.iter_mut().collect();
        let sched_wb_out = &mut me.warps.iter_mut().map(|w| &mut w.schedule_wb).collect();
        link_vec(sched_wb_out, sched_wb_in);

        let imem_req_warps = &mut me.warps.iter_mut().map(|w| &mut w.imem_req).collect();
        let imem_req_core = &mut me.imem_req.iter_mut().collect();
        link_vec(imem_req_warps, imem_req_core);

        let imem_resp_warps = &mut me.warps.iter_mut().map(|w| &mut w.imem_resp).collect();
        let imem_resp_core = &mut me.imem_resp.iter_mut().collect();
        link_vec(imem_resp_warps, imem_resp_core);

        info!("muon core {} instantiated!", config.lane_config.core_id);

        me.init_conf(config.clone());
        me
    }

    fn get_children(&mut self) -> Vec<&mut dyn ComponentBehaviors> {
        todo!()
    }
);

impl ComponentBehaviors for MuonCoreCytron {
    fn tick_one(&mut self) {
        self.scheduler.tick_one();
        if let Some(sched) = self.scheduler.schedule[0].peek() {
            info!("warp 0 schedule=0x{:08x}", sched.pc);
            assert_eq!(self.warps[0].schedule.peek().unwrap().pc, sched.pc);
        }

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
