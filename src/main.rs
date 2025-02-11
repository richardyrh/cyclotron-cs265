extern crate lazy_static;

use std::env::args;
use std::sync::Arc;
use cyclotron::base::behavior::*;
use cyclotron::muon::config::MuonConfig;
use cyclotron::sim::top::{CyclotronTop, CyclotronTopConfig};

pub fn main() {
    env_logger::init();
    let argv: Vec<_> = args().collect();
    if argv.len() < 2 {
        println!("usage: {} <binary>", argv[0]);
        return;
    }

    let mut cytron_top = CyclotronTop::new(Arc::new(CyclotronTopConfig {
        timeout: 50000,
        elf_path: argv[1].clone(),
        muon_config: MuonConfig {
            num_lanes: 4,
            num_warps: 4,
            num_cores: 1,
            lane_config: Default::default(),
        },
    }));

    cytron_top.muon.reset();
    for _ in 0..cytron_top.timeout {
        cytron_top.tick_one()
    }
}
