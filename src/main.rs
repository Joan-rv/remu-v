use remu_v::{decode, memory::Memory, state::State};
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
        let instruction = memory.lw(state.pc);
        state.pc += 4;
        println!("{:?}", decode::decode(instruction)?);
    }
}

#[derive(Debug)]
struct ArgError;
impl std::fmt::Display for ArgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Missing arguments")
    }
}
impl Error for ArgError {}
