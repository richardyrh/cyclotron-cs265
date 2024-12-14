use std::path::Path;
use std::sync::Arc;
use crate::base::mem::*;
use crate::base::behavior::*;
use crate::muon::core_cytron::MuonCoreCytron;
use crate::muon::config::MuonConfig;
use crate::sim::elf::ElfBackedMem;

#[derive(Default, Clone)]
pub struct CyclotronTopConfig {
    pub timeout: u64,
    pub elf_path: String,
    pub muon_config: MuonConfig,
}

pub struct CyclotronTop {
    pub imem: ElfBackedMem,
    pub muon: MuonCoreCytron,
    pub config: Parameters<CyclotronTopConfig>,

    pub timeout: u64
}

impl CyclotronTop {
    pub fn new() -> CyclotronTop {
        let muon_core = MuonCoreCytron::new();
        let perfect_imem = ElfBackedMem::default();

        CyclotronTop {
            imem: perfect_imem,
            muon: muon_core,
            config: Default::default(),
            timeout: 1000
        }
    }
}

impl Parameterizable<CyclotronTopConfig> for CyclotronTop {
    fn conf(&self) -> &CyclotronTopConfig {
        self.config.c.get().unwrap()
    }

    fn init_conf(&mut self, c: CyclotronTopConfig) {
        self.muon.init_conf(c.muon_config.clone());
        self.imem.load_path(Path::new(&c.elf_path)).expect("elf not found");
        self.config.c.set(c).map_err(|_| "").unwrap();
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
