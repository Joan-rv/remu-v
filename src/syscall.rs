use crate::{memory::Memory, state::State};
use std::io::{self, Write};
use thiserror::Error;

pub fn syscall(state: &mut State, memory: &Memory) -> Result<(), SyscallError> {
    let a7 = state.get(17);
    match a7 {
        64 => {
            // write
            let fd = state.get(10);
            let buf = state.get(11) as usize;
            let count = state.get(12) as usize;
            if fd == 1 {
                _ = io::stdout().write_all(&memory.data[buf..buf + count]);
                state.set(10, count as u32); // return count
            } else {
                // TODO: better handling of errno
                state.set(10, 9u32.wrapping_neg()); // EBADF
            }
        }
        93 => {
            // exit
            let a0 = state.gets(10);
            std::process::exit(a0);
        }
        _ => return Err(SyscallError),
    };
    Ok(())
}

#[derive(Debug, Error)]
#[error("Invalid syscall")]
pub struct SyscallError;
