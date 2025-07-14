use clap::Parser;
use remu_v::{decode, execute, memory::Memory, state::State};
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 1024)]
    memory_size: usize,
    program_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut memory = Memory::new(args.memory_size);
    memory.load_file(args.program_path, 0)?;

    let mut state = State::default();

    loop {
        let word = memory.lw(state.pc);
        state.pc += 4;
        let instruction = decode(word)?;
        execute(instruction, &mut state, &mut memory)?;
    }
}
