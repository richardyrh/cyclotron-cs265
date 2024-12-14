use crate::base::behavior::{Parameterizable, Resets, Stalls, Ticks};
use crate::base::component::{base_boilerplate, ComponentBase, IsComponent};
use crate::base::port::{InputPort, OutputPort, Port};
use crate::base::state::HasState;
use crate::muon::config::MuonConfig;
use crate::muon::core_cytron::MuonCoreCytron;
use crate::muon::csr::CSRFile;
use crate::utils::{BitSlice};

#[derive(Default)]
pub struct SchedulerState {
    active_warps: u32,
    thread_masks: Vec<u32>,
    pc: Vec<u32>,
    ipdom_stack: Vec<u32>,
}

#[derive(Default)]
pub struct ScheduleOut {
    pc: u32,
    mask: u32,
}

// instantiated per core
#[derive(Default)]
pub struct Scheduler {
    base: ComponentBase<SchedulerState, MuonConfig, MuonCoreCytron>,
    csr_file: Vec<Vec<Box<CSRFile>>>, // TODO: make this a port
    schedule: Vec<Port<OutputPort, ScheduleOut>>,
}

impl Ticks for Scheduler {
    fn tick_one(&mut self) {
        // schedulable warps
        // TODO: need to ensure active_warps = 1 => thread_mask > 0
        self.schedule.iter_mut().enumerate().for_each(|(wid, port)| {
            let ready = self.base.state.active_warps.bit(wid) && !port.blocked();
            if ready {
                let &pc = &self.base.state.pc[wid];
                port.put(ScheduleOut {
                    pc, mask: self.base.state.thread_masks[wid]
                });
                *(&mut self.base.state.pc[wid]) += 8;
            }
        });
    }
}

impl Stalls for Scheduler {}

impl Resets for Scheduler {
    fn reset(&mut self) {
        let tmask = ((1u64 << self.conf().num_lanes) - 1u64) as u32;
        self.base.state.thread_masks = [tmask].repeat(self.conf().num_lanes);
        self.base.state.active_warps = 1;
        self.update_csr();
    }
}

impl HasState for Scheduler {}

impl IsComponent<SchedulerState, MuonConfig, MuonCoreCytron> for Scheduler {
    base_boilerplate!(SchedulerState, MuonConfig, MuonCoreCytron);
}

impl Scheduler {
    pub fn set_csr_refs(&mut self, csr_files: Vec<Vec<Box<CSRFile>>>) {
        self.csr_file = csr_files;
    }

    pub fn tmc(&mut self, wid: usize, tmask: u32) {
        self.base.state.thread_masks[wid] = tmask;
        if (tmask == 0) {
            self.base.state.active_warps.mut_bit(wid, false);
        }
        self.update_csr();
    }

    pub fn wspawn(&mut self, wid: usize, curr_pc: u32, num_warps: usize) {
        let start_pc = curr_pc + 8;
        for i in 0..num_warps {
            if !self.base.state.active_warps.bit(i) {
                self.base.state.pc[i] = start_pc;
            }
            self.base.state.active_warps.mut_bit(wid, true);
        }
        self.update_csr();
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

    pub fn update_csr(&mut self) {
        if self.csr_file.is_empty() {
            panic!("scheduler csr file references are not set");
        }
        for warp_id in 0..self.conf().num_warps {
            let tmask = *(&self.state().thread_masks[warp_id]);

            let core_id = self.parent_ref().base.state.core_id;
            let active_warps = self.base.state.active_warps;
            let num_lanes = self.conf().num_lanes;
            let num_warps = self.conf().num_warps;
            let num_cores = self.conf().num_cores;
            let mhartid_base = core_id * num_warps + warp_id * num_lanes;

            self.csr_file[warp_id].iter_mut().enumerate().for_each(|(lane_id, csr)| {
                csr.emu_access(0xcc0, lane_id as u32);
                csr.emu_access(0xcc1, warp_id as u32);
                csr.emu_access(0xcc2, core_id as u32);
                csr.emu_access(0xcc3, active_warps);
                csr.emu_access(0xcc4, tmask);
                csr.emu_access(0xfc0, num_lanes as u32);
                csr.emu_access(0xfc1, num_warps as u32);
                csr.emu_access(0xfc2, num_cores as u32);
                csr.emu_access(0xf14, (mhartid_base + lane_id) as u32);
            })
        }
    }
}