use crate::state::State;
use thiserror::Error;

pub fn syscall(state: &State) -> Result<(), SyscallError> {
    let a7 = state.get(17);
    match a7 {
        93 => {
            let a0 = state.gets(10);
            std::process::exit(a0);
        }
        _ => Err(SyscallError),
    }
}

#[derive(Debug, Error)]
#[error("Invalid syscall")]
pub struct SyscallError;
