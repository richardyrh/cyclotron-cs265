
pub trait Ticks {
    fn tick_one(&mut self);
    fn tick(&mut self, cycles: u64) {
        for _ in 0..cycles {
            self.tick_one()
        }
    }
}

pub trait Stalls {
    fn is_stalled(&self) -> bool {
        false
    }
}

pub trait Resets {
    fn reset(&mut self) {}
}

pub trait Parameterizable {
    fn get_children(&mut self) -> Vec<Box<&mut dyn Parameterizable>> {
        vec![]
    }

    fn get_self_prefixes(&self) -> Vec<String> {
        vec![]
    }

    fn configure_self(&mut self, _prefix: &str, _config: &str) -> Result<(), String> {
        Ok(())
    }

    fn get_prefixes(&mut self) -> Vec<String> {
        [self.get_self_prefixes(),
        self.get_children().iter_mut().flat_map(|c| c.get_prefixes()).collect()].concat()
    }

    fn configure(&mut self, prefix: &str, config: &str) -> Result<(), String> {
        if self.get_prefixes().contains(&prefix.to_string()) {
            self.configure_self(prefix, config)?
        }
        self.get_children().iter_mut().try_for_each(|c| c.configure(prefix, config))
    }
}
