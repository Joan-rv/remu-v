use crate::{memory::Memory, state::State};
use std::io::{self, Write};
use thiserror::Error;

pub fn syscall(state: &State, memory: &Memory) -> Result<(), SyscallError> {
    let a7 = state.get(17);
    match a7 {
        64 => {
            // write
            let fd = state.get(10);
            let buf = state.get(11) as usize;
            let count = state.get(12) as usize;
            if fd == 1 {
                io::stdout().write_all(&memory.data[buf..buf + count]);
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
