use crate::instruction::Instruction;
use crate::memory::Memory;
use crate::state::State;

pub fn execute(instruction: Instruction, state: &mut State, memory: &mut Memory) {
    match instruction {
        Instruction::Beq { imm, rs1, rs2 } => {
            if state.get(rs1) == state.get(rs2) {
                state.pc = state.pc.wrapping_add_signed(imm as i32 - 4);
            }
        }
        Instruction::Bne { imm, rs1, rs2 } => {
            if state.get(rs1) != state.get(rs2) {
                state.pc = state.pc.wrapping_add_signed(imm as i32 - 4);
            }
        }
        Instruction::Blt { imm, rs1, rs2 } => {
            if state.gets(rs1) < state.gets(rs2) {
                state.pc = state.pc.wrapping_add_signed(imm as i32 - 4);
            }
        }
        Instruction::Bge { imm, rs1, rs2 } => {
            if state.gets(rs1) >= state.gets(rs2) {
                state.pc = state.pc.wrapping_add_signed(imm as i32 - 4);
            }
        }
        Instruction::Bltu { imm, rs1, rs2 } => {
            if state.get(rs1) < state.get(rs2) {
                state.pc = state.pc.wrapping_add_signed(imm as i32 - 4);
            }
        }
        Instruction::Bgeu { imm, rs1, rs2 } => {
            if state.get(rs1) >= state.get(rs2) {
                state.pc = state.pc.wrapping_add_signed(imm as i32 - 4);
            }
        }
        Instruction::Lb { rd, rs1, imm } => {
            let addr = state.get(rs1).wrapping_add_signed(imm as i32);
            let v = memory.lb(addr) as u32;
            let v = (v << 24) >> 24;
            state.set(rd, v);
        }
        Instruction::Lh { rd, rs1, imm } => {
            let addr = state.get(rs1).wrapping_add_signed(imm as i32);
            let v = memory.lh(addr) as u32;
            let v = (v << 16) >> 16;
            state.set(rd, v);
        }
        Instruction::Lw { rd, rs1, imm } => {
            let addr = state.get(rs1).wrapping_add_signed(imm as i32);
            let v = memory.lw(addr);
            state.set(rd, v);
        }
        Instruction::Lbu { rd, rs1, imm } => {
            let addr = state.get(rs1).wrapping_add_signed(imm as i32);
            let v = memory.lb(addr) as u32;
            state.set(rd, v);
        }
        Instruction::Lhu { rd, rs1, imm } => {
            let addr = state.get(rs1).wrapping_add_signed(imm as i32);
            let v = memory.lh(addr) as u32;
            state.set(rd, v);
        }
        Instruction::Add { rd, rs1, rs2 } => {
            let v = state.get(rs1).wrapping_add(state.get(rs2));
            state.set(rd, v);
        }
        Instruction::Addi { rd, rs1, imm } => {
            let v = state.get(rs1) + imm as u32;
            state.set(rd, v);
        }
    }
}
