use std::sync::Arc;
use log::{info};
use num_traits::FromPrimitive;
use crate::base::behavior::*;
use crate::base::component::{component, ComponentBase, IsComponent};
use crate::base::mem::HasMemory;
use crate::muon::config::MuonConfig;
use crate::muon::decode::DecodedInst;
use crate::muon::isa::{CSRType, InstAction, SFUType, ISA};
use crate::sim::top::GMEM;

pub struct Writeback {
    pub inst: DecodedInst,
    pub rd_addr: u8,
    pub rd_data: u32,
    pub set_pc: Option<u32>,
    pub sfu_type: Option<SFUType>,
    pub csr_type: Option<CSRType>,
}

// not deriving default here since if this changes in the future
// there needs to be a conscious adjustment
impl Default for Writeback {
    fn default() -> Self {
        Self {
            inst: Default::default(),
            rd_addr: 0,
            rd_data: 0,
            set_pc: None,
            sfu_type: None,
            csr_type: None,
        }
    }
}


#[derive(Default)]
pub struct ExecuteUnitState {}

#[derive(Default)]
pub struct ExecuteUnit {
    base: ComponentBase<ExecuteUnitState, MuonConfig>,
    // pub dmem_req: Port<OutputPort, MemRequest>,
    // pub dmem_resp: Port<InputPort, MemResponse>,
}

impl ComponentBehaviors for ExecuteUnit {
    fn tick_one(&mut self) {}

    fn reset(&mut self) {

    }
}

component!(ExecuteUnit, ExecuteUnitState, MuonConfig,
    fn new(_: Arc<MuonConfig>) -> Self {
        Default::default()
    }
);

impl ExecuteUnit {
    pub fn execute(&mut self, decoded: DecodedInst) -> Writeback {
        let isa = ISA::get_insts();
        info!("executing decoded instruction {}", decoded);
        let (alu_result, actions) = isa.iter().map(|inst_group| {
            inst_group.execute(&decoded)
        }).fold(None, |prev, curr| {
            assert!(prev.and(curr).is_none(), "multiple viable implementations for {}", &decoded);
            prev.or(curr)
        }).expect(&format!("unimplemented instruction {}", &decoded));

        let mut writeback = Writeback {
            inst: decoded,
            ..Writeback::default()
        };
        if (actions & InstAction::WRITE_REG) > 0 {
            writeback.rd_addr = decoded.rd;
            writeback.rd_data = alu_result;
        }
        if (actions & InstAction::MEM_LOAD) > 0 {
            let load_data_bytes = GMEM.write().expect("lock poisoned").read::<4>(
                alu_result as usize).expect("store failed");
            writeback.rd_addr = decoded.rd;
            writeback.rd_data = u32::from_le_bytes(*load_data_bytes);
        }
        if (actions & InstAction::MEM_STORE) > 0 {
            GMEM.write().expect("lock poisoned").write::<4>(
                alu_result as usize, Arc::new(decoded.rs2.to_le_bytes())).expect("store failed");
        }
        if (actions & InstAction::SET_REL_PC) > 0 {
            writeback.set_pc = (alu_result != 0).then(|| decoded.pc + alu_result);
        }
        if (actions & InstAction::SET_ABS_PC) > 0 {
            writeback.set_pc = Some(alu_result);
        }
        if (actions & InstAction::LINK) > 0 {
            writeback.rd_addr = decoded.rd;
            writeback.rd_data = decoded.pc + 8;
        }
        if (actions & InstAction::FENCE) > 0 {
            todo!();
        }
        if (actions & InstAction::SFU) > 0 {
            writeback.sfu_type = Some(SFUType::from_u32(alu_result).unwrap())
        }
        if (actions & InstAction::CSR) > 0 {
            writeback.csr_type = Some(CSRType::from_u32(alu_result).unwrap());
            writeback.rd_addr = decoded.rd;
            writeback.rd_data = decoded.imm32 as u32;
        }

        writeback
    }
}
