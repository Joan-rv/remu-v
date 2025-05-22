use crate::instruction::Instruction;
use crate::state::State;

pub fn execute(instruction: Instruction, state: &mut State) {
    match instruction {
        Instruction::Add { rd, rs1, rs2 } => {
            state.regs[rd as usize] = state.regs[rs1 as usize] + state.regs[rs2 as usize]
        }
    }
}
