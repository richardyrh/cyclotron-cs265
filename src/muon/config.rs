use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MuonConfig {
    pub num_lanes: usize,
    pub num_warps: usize,
    pub num_cores: usize,
}

impl Default for MuonConfig {
    fn default() -> Self {
        Self {
            num_lanes: 4,
            num_warps: 1,
            num_cores: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LaneConfig {
    pub lane_id: usize,
    pub warp_id: usize,
    pub core_id: usize,

    pub num_lanes: usize,
    pub num_warps: usize,
    pub num_cores: usize,
}

impl Default for LaneConfig {
    fn default() -> Self {
        Self {
            lane_id: 0,
            warp_id: 0,
            core_id: 0,
            num_lanes: 0,
            num_warps: 0,
            num_cores: 0,
        }
    }
}
