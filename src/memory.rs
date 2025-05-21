use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufReader, prelude::*},
    path::Path,
};

#[derive(Debug, Clone)]
pub struct Memory {
    data: HashMap<u32, u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn load_file(&mut self, path: impl AsRef<Path>, offset: u32) -> Result<(), std::io::Error> {
        let program = BufReader::new(File::open(path)?);
        for (idx, byte) in program.bytes().enumerate() {
            self.data.insert(offset + idx as u32, byte?);
        }
        Ok(())
    }

    pub fn lw(&self, addr: u32) -> u32 {
        u32::from_le_bytes(
            [
                self.data.get(&addr),
                self.data.get(&(addr + 1)),
                self.data.get(&(addr + 2)),
                self.data.get(&(addr + 3)),
            ]
            .map(|x| *x.unwrap_or(&0)),
        )
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
