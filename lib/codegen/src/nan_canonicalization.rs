//! A NaN-canonicalizing rewriting pass. For instructions that can potentially
//! result in a nondeterministic NaN value, insert operations to check for NaN,
//! and replace the result with a deterministic canonical NaN value if the
//! result of an instruction was in fact a NaN.

use cursor::{Cursor, FuncCursor};
use ir::{Function, Inst, InstBuilder, InstructionData, Opcode};
use ir::condcodes::FloatCC;
use ir::immediates::{Ieee32, Ieee64};
use ir::types;
use ir::types::Type;
use timing;

// Canonical 32-bit and 64-bit NaN values.
static CANON_32BIT_NAN: u32 = 0b01111111100000000000000000000001;
static CANON_64BIT_NAN: u64 = 0b0111111111110000000000000000000000000000000000000000000000000001;

/// Performs the NaN-canonicalization pass by identifying floating-point
/// arithmetic operations, and adding instructions to replace the result
/// with a canonical NaN value if the result of the operation was NaN.
pub fn do_nan_canonicalization(func: &mut Function) {
    let _tt = timing::canonicalize_nans();
    let mut pos = FuncCursor::new(func);
    while let Some(_ebb) = pos.next_ebb() {
        while let Some(inst) = pos.next_inst() {
            if is_fp_arith(&mut pos, inst) {
                add_nan_canon_instrs(&mut pos, inst);
            }
        }
    }
}

/// Returns true/false based on whether the instruction is a floating-point
/// arithmetic operation. This ignores operations like `fneg`, `fabs`, or
/// `fcopysign` that only operate on the sign bit of a floating point value.
fn is_fp_arith(pos: &mut FuncCursor, inst: Inst) -> bool {
    match pos.func.dfg[inst] {
        InstructionData::Unary { opcode, .. } => {
            opcode == Opcode::Ceil || opcode == Opcode::Floor || opcode == Opcode::Nearest ||
                opcode == Opcode::Sqrt || opcode == Opcode::Trunc
        }
        InstructionData::Binary { opcode, .. } => {
            opcode == Opcode::Fadd || opcode == Opcode::Fdiv || opcode == Opcode::Fmax ||
                opcode == Opcode::Fmin || opcode == Opcode::Fmul ||
                opcode == Opcode::Fsub
        }
        InstructionData::Ternary { opcode, .. } => opcode == Opcode::Fma,
        _ => false,
    }
}

/// Patch instructions that may result in a NaN result with operations to
/// identify and replace NaN's with a single canonical NaN value.
fn add_nan_canon_instrs(pos: &mut FuncCursor, inst: Inst) {
    // Select the instruction result, result type, step forward one instruction.
    let inst_res = pos.func.dfg.first_result(inst);
    let nan_type = pos.func.dfg.value_type(inst_res);
    let next_inst = pos.next_inst().expect("EBB missing terminator!");

    // Insert a comparison function, and a canonical NaN constant. Select
    // the constant value, and move forward to the next instruction.
    let is_nan = pos.ins().fcmp(FloatCC::NotEqual, inst_res, inst_res);
    insert_nan_const(pos, nan_type);
    let canon_nan_instr = pos.prev_inst().expect(
        "Could not find NaN constant definition!",
    );
    let canon_nan_val = pos.func.dfg.first_result(canon_nan_instr);

    // Jump to the succeeding instruction, add a select instruction that will
    // replace the result with a canonical value if it is NaN. Then step
    // backwards, so that the pass does not skip any instructions.
    pos.goto_inst(next_inst);
    pos.ins().select(is_nan, canon_nan_val, inst_res);
    pos.prev_inst();
}

/// Insert the canonical 32-bit or 64-bit NaN constant value at the current
/// position.
fn insert_nan_const(pos: &mut FuncCursor, nan_type: Type) {
    match nan_type {
        types::F32 => {
            let canon_nan = Ieee32::with_bits(CANON_32BIT_NAN);
            pos.ins().f32const(canon_nan);
        }
        types::F64 => {
            let canon_nan = Ieee64::with_bits(CANON_64BIT_NAN);
            pos.ins().f64const(canon_nan);
        }
        _ => {} // FIXUP: Should this panic or throw some sort of Error?
    }
}
