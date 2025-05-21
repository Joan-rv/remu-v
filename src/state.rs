#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    pub regs: [u32; 32],
    pub pc: u32,
}

