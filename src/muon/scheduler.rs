use crate::base::behavior::*;
use crate::base::component::{component, ComponentBase, IsComponent};
use crate::base::port::{InputPort, OutputPort, Port};
use crate::muon::config::MuonConfig;
use crate::utils::{BitSlice};

#[derive(Default)]
pub struct SchedulerState {
    active_warps: u32,
    thread_masks: Vec<u32>,
    pc: Vec<u32>,
    ipdom_stack: Vec<u32>,
}

#[derive(Default, Clone)]
pub struct ScheduleOut {
    pub pc: u32,
    pub mask: u32,
    pub active_warps: u32,
}

// instantiated per core
#[derive(Default)]
pub struct Scheduler {
    base: ComponentBase<SchedulerState, MuonConfig>,
    pub schedule: Vec<Port<OutputPort, ScheduleOut>>,
}

impl ComponentBehaviors for Scheduler {
    fn tick_one(&mut self) {
        // schedulable warps
        // TODO: need to ensure active_warps = 1 => thread_mask > 0
        self.schedule.iter_mut().enumerate().for_each(|(wid, port)| {
            let ready = self.base.state.active_warps.bit(wid) && !port.blocked();
            if ready {
                let &pc = &self.base.state.pc[wid];
                port.put(ScheduleOut {
                    pc,
                    mask: self.base.state.thread_masks[wid],
                    active_warps: self.base.state.active_warps,
                });
                *(&mut self.base.state.pc[wid]) += 8;
            }
        });
    }

    fn reset(&'_ mut self) {
        let num_lanes = (&self.conf().num_lanes).clone();
        let tmask = ((1u64 << num_lanes) - 1u64) as u32;
        self.state().thread_masks = [tmask].repeat(num_lanes);
        // self.base.state.active_warps = 1;
        // self.update_csr();
    }
}

component!(Scheduler, SchedulerState, MuonConfig,
    fn new(config: &MuonConfig) -> Scheduler {
        let num_warps = config.num_warps;
        Scheduler {
            base: Default::default(),
            schedule: vec![Default::default(); num_warps],
        }
    }
);

impl Scheduler {

    pub fn tmc(&mut self, wid: usize, tmask: u32) {
        self.base.state.thread_masks[wid] = tmask;
        if (tmask == 0) {
            self.base.state.active_warps.mut_bit(wid, false);
        }
    }

    pub fn wspawn(&mut self, wid: usize, curr_pc: u32, num_warps: usize) {
        let start_pc = curr_pc + 8;
        for i in 0..num_warps {
            if !self.base.state.active_warps.bit(i) {
                self.base.state.pc[i] = start_pc;
            }
            self.base.state.active_warps.mut_bit(wid, true);
        }
    }

    pub fn split(&mut self, wid: usize, then_mask: u32) -> u32 {
        todo!();
    }

    pub fn join(&mut self, wid: usize, divergent: bool) {
        todo!();
    }

    pub fn branch(&mut self, wid: usize, target_u32: u32) {
        self.base.state.pc[wid] = target_u32;
    }
}