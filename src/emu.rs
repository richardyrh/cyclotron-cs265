#![feature(once_cell_get_mut)]

use crate::sim::Sim;
use std::sync::{OnceLock, RwLock};

struct Config {
    num_lanes: usize,
    wordsize: u64,
}

static CONFIG_CELL: OnceLock<Config> = OnceLock::new();

// A single-writer, multiple-reader mutex lock on the global singleton of Sim.
// A singleton is necessary because it's the only way to maintain context across independent DPI
// calls.  Could use RefCell instead, but RwLock covers the case where RTL is multi-threaded.
static CELL: RwLock<Option<Sim>> = RwLock::new(None);

struct ReqBundle {
    ready: bool,
    valid: bool,
    address: u64,
    size: u32,
}

struct RespBundle {
    ready: bool,
    valid: bool,
    size: u32,
}

#[no_mangle]
pub fn emulator_init_rs(num_lanes: i32) {
    CONFIG_CELL.get_or_init(|| Config {
        num_lanes: num_lanes as usize,
        wordsize: 4,
    });

    let mut sim = CELL.write().unwrap();
    if !sim.as_ref().is_none() {
        panic!("sim cell already initialized!");
    }
    *sim = Some(Sim::new());
}

#[no_mangle]
pub fn emulator_generate_rs(
    ptr_a_ready: *const u8,
    ptr_a_valid: *mut u8,
    ptr_a_address: *mut u64,
    _ptr_a_is_store: *mut u8,
    ptr_a_size: *mut u32,
    _ptr_a_data: *mut u64,
    ptr_d_ready: *mut u8,
    ptr_d_valid: *const u8,
    _ptr_d_is_store: *mut u8,
    ptr_d_size: *mut u32,
    inflight: u8,
    ptr_finished: *mut u8,
) {
    let conf = CONFIG_CELL.get().unwrap();

    let mut sim_guard = CELL.write().unwrap();
    let sim = match sim_guard.as_mut() {
        Some(s) => s,
        None => {
            panic!("sim cell not initialized!");
        }
    };
    sim.tick();

    let vec_a_ready = unsafe { std::slice::from_raw_parts(ptr_a_ready, conf.num_lanes) };
    let vec_a_valid = unsafe { std::slice::from_raw_parts_mut(ptr_a_valid, conf.num_lanes) };
    let vec_a_address = unsafe { std::slice::from_raw_parts_mut(ptr_a_address, conf.num_lanes) };
    let vec_a_size = unsafe { std::slice::from_raw_parts_mut(ptr_a_size, conf.num_lanes) };
    let vec_d_ready = unsafe { std::slice::from_raw_parts_mut(ptr_d_ready, conf.num_lanes) };
    let vec_d_valid = unsafe { std::slice::from_raw_parts(ptr_d_valid, conf.num_lanes) };
    let vec_d_size = unsafe { std::slice::from_raw_parts(ptr_d_size, conf.num_lanes) };
    let finished = unsafe { std::slice::from_raw_parts_mut(ptr_finished, 1) };

    // FIXME: work with 1 lane for now
    let mut resp_bundles = Vec::with_capacity(1);
    for i in 0..1 {
        resp_bundles.push(RespBundle {
            ready: (vec_a_ready[i] != 0),
            valid: (vec_d_valid[i] != 0),
            size: vec_d_size[i],
        });
    }

    push_imem_resp(sim, &resp_bundles[0]);

    // FIXME: check a_ready from mem side

    let mut req_bundles = Vec::with_capacity(1);
    match generate_imem_req(sim) {
        Some(bundle) => {
            req_bundles.push(bundle);
        },
        None => {}
    }

    req_bundles_to_vecs(
        &req_bundles,
        vec_a_valid,
        vec_a_address,
        vec_a_size,
        vec_d_ready,
    );
}

fn push_imem_resp(sim: &mut Sim, resp: &RespBundle) {
    if !resp.valid {
        return;
    }
    sim.push_imem_resp(resp.size as u64 /*bogus*/);
}

fn generate_imem_req(sim: &mut Sim) -> Option<ReqBundle> {
    let front = sim.pop_imem_resp();
    match front {
        Some(data) => Some(ReqBundle {
            valid: true,
            address: data,
            size: 4, // bogus
            ready: true,
        }),
        None => None,
    }
}

// unwrap arrays-of-structs to structs-of-arrays
fn req_bundles_to_vecs(
    bundles: &[ReqBundle],
    vec_a_valid: &mut [u8],
    vec_a_address: &mut [u64],
    vec_a_size: &mut [u32],
    vec_d_ready: &mut [u8],
) {
    for i in 0..bundles.len() {
        vec_a_valid[i] = if bundles[i].valid { 1 } else { 0 };
        vec_a_address[i] = bundles[i].address;
        vec_a_size[i] = bundles[i].size;
        vec_d_ready[i] = 1; // FIXME: bogus
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        // import_me()
    }
}
