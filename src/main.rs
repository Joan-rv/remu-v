use remu_v::{decode, execute, memory::Memory, state::State};
use std::error::Error;
use thiserror::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let Some(path) = std::env::args().nth(1) else {
        println!("Missing argument");
        return Err(Box::new(ArgError));
    };

    let mut memory = Memory::new(1024);
    memory.load_file(path, 0)?;

    let mut state = State::default();

    loop {
        let word = memory.lw(state.pc);
        state.pc += 4;
        let instruction = match decode(word) {
            Ok(i) => i,
            Err(e) => {
                println!("Failed decoding: {}", e);
                break;
            }
        };
        execute(instruction, &mut state);
    }
    println!("End state: {:?}", state);
    Ok(())
}

#[derive(Error, Debug)]
#[error("Missing arguments")]
struct ArgError;
