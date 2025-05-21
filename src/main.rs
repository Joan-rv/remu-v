use remu_v::decode::decode;
use std::error::Error;
use std::io::{BufReader, prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let Some(path) = std::env::args().nth(1) else {
        println!("Missing argument");
        return Err(Box::new(ArgError));
    };
    let program = BufReader::new(std::fs::File::open(path)?);
    let memory = program.bytes().collect::<Result<Vec<u8>, _>>()?;

    println!(
        "{:?}",
        decode(u32::from_le_bytes(memory[0..4].try_into().unwrap()))?
    );
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
