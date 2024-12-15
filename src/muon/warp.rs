use std::rc::Rc;
use std::sync::Arc;
use crate::base::behavior::*;
use crate::base::component::{component, ComponentBase, IsComponent};
use crate::base::mem::{MemRequest, MemResponse};
use crate::base::port::{InputPort, OutputPort, Port};
use crate::builtin::queue::Queue;
use crate::muon::config::MuonConfig;
use crate::muon::csr::CSRFile;
use crate::muon::decode::{DecodeUnit, RegFile};
use crate::muon::execute::ExecuteUnit;
use crate::muon::scheduler::ScheduleOut;
use crate::utils::BitSlice;

#[derive(Default)]
pub struct WarpState {
}

#[derive(Clone, Default)]
pub struct FetchMetadata {
    pub mask: u32,
    pub pc: u32,
}

impl From<&ScheduleOut> for FetchMetadata {
    fn from(value: &ScheduleOut) -> Self {
        Self { pc: value.pc, mask: value.mask }
    }
}

#[derive(Default)]
pub struct Warp {
    base: ComponentBase<WarpState, MuonConfig>,
    pub lanes: Vec<Lane>,

    pub fetch_queue: Queue<FetchMetadata, 4>,
    pub schedule: Port<InputPort, ScheduleOut>,

    pub imem_req: Port<OutputPort, MemRequest>,
    pub imem_resp: Port<InputPort, MemResponse>,
}

impl ComponentBehaviors for Warp {
    fn tick_one(&mut self) {
        { self.base().cycle += 1; }
        // fetch
        if let Some(schedule) = self.schedule.get() {
            if !self.imem_req.blocked() && self.fetch_queue.try_enq(schedule.into()) {
                assert!(self.imem_req.read::<8>(schedule.pc as usize));
            }
        }
        // decode, execute, writeback
        if let Some(resp) = self.imem_resp.get() {
            let metadata = self.fetch_queue.try_deq().expect("fetch queue empty");
            let inst_data: [u8; 8] = (*(resp.data.as_ref().unwrap().clone()))
                .try_into().expect("imem response is not 8 bytes");
            println!("cycle={}, pc={:08x}", self.base.cycle, metadata.pc);
            for lane_id in 0..self.conf().num_lanes {
                if !metadata.mask.bit(lane_id) { continue; }
                let rf = &self.lanes[lane_id].reg_file;
                let decoded = self.lanes[lane_id].decode_unit.decode(inst_data, metadata.pc, rf);
                let writeback = self.lanes[lane_id].execute_unit.execute(decoded);

                let rf_mut = &mut self.lanes[lane_id].reg_file;
                rf_mut.write_gpr(writeback.rd_addr, writeback.rd_data);
                /* if let Some(pc) = writeback.set_pc {
                    if pc == 0 {
                        println!("simulation has probably finished, main has returned");
                    }
                    self.base.state.pc = pc;
                } */
                todo!()
            }
        }
    }

    fn reset(&mut self) {
        self.lanes.iter_mut().for_each(|lane| {
            lane.reg_file.reset();
            lane.execute_unit.reset();
        });
    }
}

component!(Warp, WarpState, MuonConfig,
    fn get_param_children(&mut self) -> Vec<&mut dyn Parameterizable<ConfigType=MuonConfig>> {
        let execute_units: Vec<_> = self.lanes.iter_mut().map(|l| {
            &mut l.execute_unit as &mut dyn Parameterizable<ConfigType=Self::ConfigType>
        }).collect();

        execute_units
    }
    
    fn get_children(&mut self) -> Vec<&mut dyn ComponentBehaviors> {
        todo!()
    }

    fn new(config: &MuonConfig) -> Warp {
        let num_lanes = config.num_lanes;
        Warp {
            base: ComponentBase::default(),
            lanes: (0..num_lanes).map(|_| {
                Lane {
                    reg_file: RegFile::new(&()),
                    csr_file: Default::default(),
                    decode_unit: DecodeUnit,
                    execute_unit: Default::default(),
                }
            }).collect(),
            fetch_queue: Queue::default(),
            schedule: Default::default(),
            imem_req: Port::new(),
            imem_resp: Port::new()
        }
    }
);

pub struct Lane {
    pub reg_file: RegFile,
    pub csr_file: CSRFile,
    pub decode_unit: DecodeUnit,
    pub execute_unit: ExecuteUnit,
}
