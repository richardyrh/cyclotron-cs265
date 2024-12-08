use std::sync::Arc;
use crate::base::mem::*;
use crate::base::behavior::*;
use crate::muon::core_cytron::MuonCoreCytron;
use crate::sim::elf::ElfBackedMem;

pub struct CyclotronTop {
    pub imem: ElfBackedMem,
    pub muon: MuonCoreCytron,

    pub timeout: u64
}

impl CyclotronTop {
    pub fn new() -> CyclotronTop {
        let muon_core = MuonCoreCytron::new();
        let perfect_imem = ElfBackedMem::default();

        CyclotronTop {
            imem: perfect_imem,
            muon: muon_core,
            timeout: 0
        }
    }
}

impl Ticks for CyclotronTop {
    fn tick_one(&mut self) {
        if let Some(req) = self.muon.imem_req.get() {
            assert_eq!(req.size, 8, "imem read request is not 8 bytes");
            let inst = self.imem.read_inst(req.address)
                .expect(&format!("invalid pc: 0x{:x}", req.address));
            let succ = self.muon.imem_resp.put(MemResponse {
                op:MemRespOp::Ack,
                data: Some(Arc::new(inst.to_le_bytes())),
            });
            assert!(succ, "muon asserted fetch pressure, not implemented");
        }
        self.imem.tick_one(); // useless now
        self.muon.tick_one();
    }
}

impl Parameterizable for CyclotronTop {
    fn get_children(&mut self) -> Vec<Box<&mut dyn Parameterizable>> {
        vec![Box::new(&mut self.imem), Box::new(&mut self.muon)]
    }

    fn get_self_prefixes(&self) -> Vec<String> {
        vec!["sim.timeout".to_string()]
    }

    fn configure_self(&mut self, prefix: &str, config: &str) -> Result<(), String> {
        match prefix {
            "sim.timeout" => { self.timeout = config.parse().map_err(|_| "bad timeout value")? }
            _ => {}
        }
        Ok(())
    }
}
