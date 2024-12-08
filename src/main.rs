use std::error::Error;
use cyclotron::base::behavior::*;
use cyclotron::sim::top::CyclotronTop;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut cytron_top = CyclotronTop::new();

    // TODO: read a yaml and parse
    cytron_top.configure("sim.muon.kernel_path", "hello.elf")?;
    cytron_top.configure("sim.timeout", "1000")?;

    cytron_top.muon.reset();
    for c in 0..cytron_top.timeout {
        cytron_top.tick_one()
    }
    Ok(())
}
