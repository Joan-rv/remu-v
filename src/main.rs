use remu_v::{decode, execute, memory::Memory, state::State};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let Some(path) = std::env::args().nth(1) else {
        println!("Missing argument");
        return Err(Box::new(ArgError));
    };

    let mut memory = Memory::new();
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

#[derive(Debug)]
struct ArgError;
impl std::fmt::Display for ArgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Missing arguments")
    }
}
impl Error for ArgError {}
