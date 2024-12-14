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
