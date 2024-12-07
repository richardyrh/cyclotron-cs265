// An input port to the simulator
// TODO: having a timestamp requires this to eventually become a priority queue, to enable
// enqueueing entries N cycles in advance
use std::{mem::{self, MaybeUninit}, ptr::copy_nonoverlapping, slice};

pub trait HasState: Sized + Default {
    fn set_state(&mut self, state: Self) {
        *self = state
    }

    fn inject_state(&mut self, buf: &[u8]) -> Result<(), String> {
        if buf.len() != mem::size_of::<Self>() {
            return Err(format!(
                    "Incorrect size: expected {} bytes, got {} bytes",
                    mem::size_of::<Self>(),
                    buf.len()
            ));
        }

        // Create an uninitialized T
        let mut value = MaybeUninit::<Self>::uninit();

        unsafe {
            // Copy the bytes into the uninitialized memory
            copy_nonoverlapping(buf.as_ptr(), value.as_mut_ptr().cast::<u8>(), mem::size_of::<Self>());
            // Assume initialized: This is only safe if T can hold any bit pattern (e.g., no invalid enums)
            *self = value.assume_init();
        }
        Ok(())
    }

    fn dump_state(&self) -> Vec<u8> {
        unsafe {
            // Create a slice of bytes from the reference to self
            let ptr = self as *const Self as *const u8;
            let slice = slice::from_raw_parts(ptr, mem::size_of::<Self>());
            slice.to_vec()
        }
    }
}

#[derive(Default)]
pub struct NullState {}
