#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    regs: [u32; 32],
    pub pc: u32,
}

impl State {
    pub fn get(&mut self, r: u8) -> u32 {
        self.regs[r as usize]
    }
    pub fn set(&mut self, r: u8, v: u32) {
        if r > 0 {
            self.regs[r as usize] = v;
        }
    }
}
