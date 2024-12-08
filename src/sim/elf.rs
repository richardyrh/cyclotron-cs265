use std::{collections::HashMap, fs, path::Path};

use goblin::elf::Elf;

use crate::base::parameterizable::Parameterizable;

struct ElfReader {
    sections: HashMap<(usize, usize), Vec<u8>>,
}

impl Parameterizable<String> for ElfReader {
    type Error = String;

    fn get_prefixes() -> Vec<String> {
        vec!["sim.muon.kernel_path".to_string()]
    }

    fn configure(&mut self, prefix: &str, config: String) -> Result<(), Self::Error> {
        match prefix {
            "sim.muon.kernel_path" => {
                self.load_path(Path::new(&config))
            },
            _ => Err(format!("unknown configuration item {}", config)),
        }
    }
}

impl ElfReader {
    fn read(&self, addr: usize) -> Option<u64> {
        let &slice = self.read_slice::<8>(addr)?;
        u64::from_le_bytes(slice).into()
    }

    fn read_slice<const N: usize>(&self, addr: usize) -> Option<&[u8; N]> {
        self.sections.iter().fold(None, |prev, (range, data)| {
            prev.or(((addr >= range.0) && (addr + N <= range.1)).then(|| {
                data[(addr - range.0)..(addr - range.0 + N)].try_into().unwrap()
            }))
        })
    }

    pub fn load_path(&mut self, path: &Path) -> Result<(), String> {
        let data = fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;
        let elf = Elf::parse(&data).map_err(|e| format!("Failed to parse ELF file: {}", e))?;

        let mut section_map = HashMap::new();

        // Iterate over the ELF sections
        for section in &elf.section_headers {
            let start = section.sh_addr;
            let size = section.sh_size;
            if size > 0 {
                let end = start + size;
                let range = (start, end);

                // Extract the section bytes
                let offset = section.sh_offset as usize;
                let size = section.sh_size as usize;
                if offset + size <= data.len() {
                    let bytes = data[offset..offset + size].to_vec();
                    section_map.insert(range, bytes);
                } else {
                    return Err(format!(
                            "Invalid section bounds: offset {} size {}",
                            offset, size
                    ));
                }
            }
        }

        Ok(())
    }
}
