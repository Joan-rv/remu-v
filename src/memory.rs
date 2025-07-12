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

    pub fn lb(&self, addr: u32) -> i8 {
        let addr = addr as usize;
        i8::from_le_bytes(self.data[addr..(addr + 1)].try_into().unwrap())
    }
    pub fn lh(&self, addr: u32) -> i16 {
        let addr = addr as usize;
        i16::from_le_bytes(self.data[addr..(addr + 2)].try_into().unwrap())
    }
    pub fn lw(&self, addr: u32) -> u32 {
        let addr = addr as usize;
        u32::from_le_bytes(self.data[addr..(addr + 4)].try_into().unwrap())
    }
    pub fn lbu(&self, addr: u32) -> u8 {
        let addr = addr as usize;
        u8::from_le_bytes(self.data[addr..(addr + 1)].try_into().unwrap())
    }
    pub fn lhu(&self, addr: u32) -> u16 {
        let addr = addr as usize;
        u16::from_le_bytes(self.data[addr..(addr + 2)].try_into().unwrap())
    }
    pub fn sb(&mut self, addr: u32, value: u8) {
        let addr = addr as usize;
        self.data[addr] = value;
    }
    pub fn sh(&mut self, addr: u32, value: u16) {
        let addr = addr as usize;
        self.data[addr..addr + 2].copy_from_slice(&value.to_le_bytes());
    }
    pub fn sw(&mut self, addr: u32, value: u32) {
        let addr = addr as usize;
        self.data[addr..addr + 4].copy_from_slice(&value.to_le_bytes());
    }
}
