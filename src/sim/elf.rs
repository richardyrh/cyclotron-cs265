use std::collections::HashMap;

use crate::base::parameterizable::Parameterizable;


struct ElfReader {
    sections: HashMap<u32, Box<[u8]>>,
}

impl Parameterizable<String> for ElfReader {
    type Error = String;

    fn get_prefix() -> String {
        "sim.muon.kernel_path".to_string()
    }

    fn configure(&mut self, config: String) -> Result<(), Self::Error> {
        todo!() // TODO: read the elf
    }
}

impl ElfReader {
    fn read(addr: u32) -> u64 {
        0
    }
}
