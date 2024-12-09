use std::{collections::HashMap, fs, path::Path};
use std::sync::Arc;
use goblin::elf::Elf;
use crate::base::behavior::*;
use crate::base::component::{ComponentBase, IsComponent};
use crate::base::state::HasState;
use crate::base::mem::HasMemory;

#[derive(Default)]
pub struct ElfBackedMemState {
    pub sections: HashMap<(usize, usize), Vec<u8>>,
}

#[derive(Default)]
pub struct ElfBackedMem {
    pub base: ComponentBase<ElfBackedMemState>,
}

impl Parameterizable for ElfBackedMem {
    fn get_children(&mut self) -> Vec<Box<&mut dyn Parameterizable>> {
        vec![]
    }

    fn get_self_prefixes(&self) -> Vec<String> {
        vec!["sim.muon.kernel_path".to_string()]
    }

    fn configure_self(&mut self, prefix: &str, config: &str) -> Result<(), String> {
        match prefix {
            "sim.muon.kernel_path" => {
                self.load_path(Path::new(&config))
            },
            _ => Err(format!("unknown configuration item {}", config)),
        }
    }
}

impl IsComponent<ElfBackedMemState> for ElfBackedMem {
    fn get_base(&mut self) -> &mut ComponentBase<ElfBackedMemState> {
        &mut self.base
    }
}

impl Ticks for ElfBackedMem {
    fn tick_one(&mut self) {}
}

impl Stalls for ElfBackedMem {}
impl Resets for ElfBackedMem {}
impl HasState for ElfBackedMem {}

impl HasMemory for ElfBackedMem {
    fn read<const N: usize>(&mut self, addr: usize) -> Option<Arc<[u8; N]>> {
        self.base.state.sections.iter().fold(None, |prev, (range, data)| {
            prev.or(((addr >= range.0) && (addr + N <= range.1)).then(|| {
                Arc::new(data[(addr - range.0)..(addr - range.0 + N)].try_into().unwrap())
            }))
        })
    }

    fn write<const N: usize>(&mut self, _addr: usize, _data: Arc<[u8; N]>) -> Result<(), String> {
        Err("elf backed memory cannot be written to".to_string())
    }
}

impl ElfBackedMem {
    pub fn read_inst(&mut self, addr: usize) -> Option<u64> {
        let slice = self.read::<8>(addr)?;
        u64::from_le_bytes(*slice).into()
    }

    pub fn load_path(&mut self, path: &Path) -> Result<(), String> {
        let data = fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;
        let elf = Elf::parse(&data).map_err(|e| format!("Failed to parse ELF file: {}", e))?;

        self.base.state.sections = HashMap::new();

        // Iterate over the ELF sections
        for section in &elf.section_headers {
            let start = section.sh_addr;
            let size = section.sh_size;
            if size > 0 {
                let end = start + size;
                let range = (start as usize, end as usize);

                // Extract the section bytes
                let offset = section.sh_offset as usize;
                let size = section.sh_size as usize;
                if offset + size <= data.len() {
                    let bytes = data[offset..offset + size].to_vec();
                    self.base.state.sections.insert(range, bytes);
                } else {
                    return Err(format!(
                            "Invalid section bounds: offset {} size {}",
                            offset, size
                    ));
                }
            }
        }

        print!("{:?}", self.base.state.sections.keys());

        Ok(())
    }
}
