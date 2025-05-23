use std::vec::Vec;
use std::{
    fs::File,
    io::{BufReader, prelude::*},
    path::Path,
};

#[derive(Debug, Clone)]
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    pub fn load_file(
        &mut self,
        path: impl AsRef<Path>,
        offset: usize,
    ) -> Result<(), std::io::Error> {
        let program = BufReader::new(File::open(path)?);
        for (idx, byte) in program.bytes().enumerate() {
            self.data[offset + idx] = byte?;
        }
        Ok(())
    }

    pub fn lw(&self, addr: u32) -> u32 {
        let addr = addr as usize;
        debug_assert!(self.data.len() >= addr + 4);
        u32::from_le_bytes(self.data[addr..(addr + 4)].try_into().unwrap())
    }
}
