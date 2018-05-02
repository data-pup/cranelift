//! A NaN-canonicalizing rewriting pass.

use cursor::{Cursor, FuncCursor};
use ir::{DataFlowGraph, Function, Inst, InstBuilder, InstructionData, Opcode, Value};
use ir::condcodes::FloatCC;
use ir::immediates::{Ieee32, Ieee64};
use ir::types;
use ir::types::Type;
// use ir::types::{F32, F64, Type};
use timing;

static CANON_32BIT_NAN: u32 = 0b01111111100000000000000000000001;
static CANON_64BIT_NAN: u64 =
    0b0111111111110000000000000000000000000000000000000000000000000001;

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
    // FIXUP: Should `ceil`, `floor`, `trunc`, `nearest`, and
    // immediate constants be considered as well?
    match pos.func.dfg[inst] {
        InstructionData::Unary { opcode, .. } => {
            opcode == Opcode::Sqrt
        },
        InstructionData::Binary { opcode, .. } => {
            opcode == Opcode::Fadd
            || opcode == Opcode::Fsub
            || opcode == Opcode::Fmul
            || opcode == Opcode::Fdiv
            || opcode == Opcode::Fmin
            || opcode == Opcode::Fmax
        },
        InstructionData::Ternary { opcode, .. } => {
            opcode == Opcode::Fma
        },
        _ => false,
    }
}

/// Patch instructions that may result in a NaN result with operations to
/// identify and replace NaN's with a single canonical NaN value.
fn add_nan_canon_instrs(pos: &mut FuncCursor, inst: Inst) {
    // Select the operation's result, and move to the next instruction.
    // (FIXUP: Is this completely safe to unwrap? Is unwrapping even needed?)
    let inst_res: Value = pos.func.dfg.first_result(inst);
    let next_inst: Inst = pos.next_inst().unwrap();
    let is_nan: Value = pos.ins().fcmp(FloatCC::NotEqual, inst_res, inst_res);

    insert_nan_const(pos, inst);
    let canon_nan_instr: Inst = pos.prev_inst().unwrap();
    let canon_nan_res: Value = pos.func.dfg.first_result(canon_nan_instr);

    pos.goto_inst(next_inst);
    pos.ins().select(is_nan, canon_nan_res, inst_res);
}

/// Insert the canonical 32-bit or 64-bit NaN constant value at the current
/// position.
fn insert_nan_const(pos: &mut FuncCursor, inst: Inst) {
    match get_nan_type(&pos.func.dfg, inst) {
        types::F32 => {
            let canon_nan = Ieee32::with_bits(CANON_32BIT_NAN);
            pos.ins().f32const(canon_nan);
        },
        types::F64 => {
            let canon_nan = Ieee64::with_bits(CANON_64BIT_NAN);
            pos.ins().f64const(canon_nan);
        },
        _ => unimplemented!() // FIXUP: Should this panic or throw some sort of Error?
    }
}

/// Given some instruction that could potentially return a nondeterministic
/// NaN value, determine if the operation is using 32-bit or 64-bit floating
/// point numbers, and return the corresponding NaN value.
fn get_nan_type(dfg: &DataFlowGraph, inst: Inst) -> Type {
    let inst_data: &InstructionData = &dfg[inst];
    match *inst_data {
        InstructionData::Unary { arg, .. } => {
            dfg.value_type(arg)
        },
        InstructionData::Binary { args, .. } => {
            let lhs_operand = args[0];
            dfg.value_type(lhs_operand)
        },
        InstructionData::Ternary { args, .. } => {
            let lhs_operand = args[0];
            dfg.value_type(lhs_operand)
        },
        _ => unimplemented!(), // FIXUP: What would I do in this case? Error?
    }
}
