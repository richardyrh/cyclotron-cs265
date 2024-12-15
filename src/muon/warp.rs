use crate::base::behavior::*;
use crate::base::component::{component, ComponentBase, IsComponent};
use crate::base::mem::{MemRequest, MemResponse};
use crate::base::port::{InputPort, OutputPort, Port};
use crate::builtin::queue::Queue;
use crate::muon::config::{LaneConfig, MuonConfig};
use crate::muon::csr::CSRFile;
use crate::muon::decode::{DecodeUnit, DecodedInst, RegFile};
use crate::muon::execute::{ExecuteUnit, Writeback};
use crate::muon::isa::SFUType;
use crate::muon::scheduler::ScheduleOut;
use crate::utils::BitSlice;

#[derive(Default)]
pub struct WarpState {
    pub stalled: bool,
}

#[derive(Clone, Default)]
pub struct FetchMetadata {
    pub mask: u32,
    pub pc: u32,
    pub active_warps: u32,
    pub end_stall: bool,
}

impl From<&ScheduleOut> for FetchMetadata {
    fn from(value: &ScheduleOut) -> Self {
        Self {
            pc: value.pc,
            mask: value.mask,
            active_warps: value.active_warps,
            end_stall: value.end_stall
        }
    }
}

#[derive(Default, Clone)]
pub struct ScheduleWriteback {
    pub insts: Vec<DecodedInst>,
    pub branch: Option<u32>,
    pub sfu: Option<SFUType>,
}

#[derive(Default)]
pub struct Warp {
    base: ComponentBase<WarpState, MuonConfig>,
    pub lanes: Vec<Lane>,

    pub fetch_queue: Queue<FetchMetadata, 4>,
    pub schedule: Port<InputPort, ScheduleOut>,

    pub imem_req: Port<OutputPort, MemRequest>,
    pub imem_resp: Port<InputPort, MemResponse>,

    pub schedule_wb: Port<OutputPort, ScheduleWriteback>,
}

impl ComponentBehaviors for Warp {
    fn tick_one(&mut self) {
        { self.base().cycle += 1; }
        // fetch
        if let Some(schedule) = self.schedule.get() {
            if schedule.end_stall || !self.base.state.stalled {
                if !self.imem_req.blocked() && self.fetch_queue.try_enq(schedule.into()) {
                    assert!(self.imem_req.read::<8>(schedule.pc as usize));
                }
            }
        }
        // decode, execute, writeback
        if let Some(resp) = self.imem_resp.get() {
            let metadata = self.fetch_queue.try_deq().expect("fetch queue empty");

            self.base.state.stalled &= !metadata.end_stall;
            if self.base.state.stalled {
                println!("stalling pc 0x{:08x}", metadata.pc);
                return;
            }

            // decode, execute, write back to register file
            let inst_data: [u8; 8] = (*(resp.data.as_ref().unwrap().clone()))
                .try_into().expect("imem response is not 8 bytes");
            println!("cycle={}, pc={:08x}", self.base.cycle, metadata.pc);
            let writebacks: Vec<_> = (0..self.conf().num_lanes).map(|lane_id| {
                self.lanes[lane_id].csr_file.emu_access(0xcc3, metadata.active_warps);
                self.lanes[lane_id].csr_file.emu_access(0xcc4, metadata.mask)
                ;
                if !metadata.mask.bit(lane_id) {
                    return Writeback::default();
                }

                let rf = &self.lanes[lane_id].reg_file;
                let decoded = self.lanes[lane_id].decode_unit.decode(inst_data, metadata.pc, rf);
                let writeback = self.lanes[lane_id].execute_unit.execute(decoded.clone());

                let rf_mut = &mut self.lanes[lane_id].reg_file;
                rf_mut.write_gpr(writeback.rd_addr, writeback.rd_data);
                writeback
            }).collect();

            // update the scheduler
            writebacks[0].set_pc.map(|pc| {
                if pc == 0 {
                    println!("simulation has probably finished, main has returned");
                    std::process::exit(0);
                }
                self.state().stalled = true;
                ScheduleWriteback {
                    insts: writebacks.iter().map(|w| w.inst).collect(),
                    branch: Some(pc),
                    sfu: None,
                }
            }).or(writebacks[0].sfu_type.map(|sfu| {
                self.state().stalled = true;
                ScheduleWriteback {
                    insts: writebacks.iter().map(|w| w.inst).collect(),
                    branch: None,
                    sfu: Some(sfu),
                }
            })).map(|wb| {
                assert!(self.schedule_wb.put(wb));
            });
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
            lanes: (0..num_lanes).map(|lane_id| {
                let lane_config = MuonConfig {
                    num_lanes: config.num_lanes,
                    num_warps: config.num_warps,
                    num_cores: config.num_cores,
                    lane_config: LaneConfig {
                        lane_id,
                        ..config.lane_config
                    }
                };
                Lane {
                    reg_file: RegFile::new(&()),
                    csr_file: CSRFile::new(&lane_config),
                    decode_unit: DecodeUnit,
                    execute_unit: ExecuteUnit::new(&lane_config),
                }
            }).collect(),
            fetch_queue: Queue::new(&()),
            schedule: Port::new(),
            imem_req: Port::new(),
            imem_resp: Port::new(),
            schedule_wb: Port::new(),
        }
    }
);

pub struct Lane {
    pub reg_file: RegFile,
    pub csr_file: CSRFile,
    pub decode_unit: DecodeUnit,
    pub execute_unit: ExecuteUnit,
}
