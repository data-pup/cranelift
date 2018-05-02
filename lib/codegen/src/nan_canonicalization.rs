//! A NaN-canonicalizing rewriting pass.

use cursor::{Cursor, FuncCursor};
use ir::{DataFlowGraph, Function, Inst, InstBuilder, InstructionData, Opcode, Value};
use ir::immediates::{Ieee32, Ieee64};
use ir::types;
use ir::types::Type;
// use ir::types::{F32, F64, Type};
use timing;

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

/// Given some instruction that could potentially return a nondeterministic
/// NaN value, determine if the operation is using 32-bit or 64-bit floating
/// point numbers, and return the corresponding NaN value.
/// FIXUP: Not sure if this is the correct prototype for this function.
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

/// Patch instructions that may result in a NaN result with operations to
/// identify and replace NaN's with a single canonical NaN value.
fn add_nan_canon_instrs(pos: &mut FuncCursor, inst: Inst) {
    // ----------------------------------------------------------------
    // Let x be the result to some floating point arithmetic operation.
    // Add the following instructions after `inst` : (Pseudo-code)
    // let is_nan = x != x;                          (fcmp)
    // let canonical_res = is_nan ? CANON_VALUE : x  (select)
    // ----------------------------------------------------------------
    // FIXUP: Verbose type annotations are for learning's sake.
    // ----------------------------------------------------------------

    // Original State:
    // ----------------------------------------------------------------
    // x = [floating point arithmetic instruction] <-pos
    // [next instruction]
    // ----------------------------------------------------------------

    // Select the operation's result, and move to the next instruction.
    // (FIXUP: Is this completely safe to unwrap? Is unwrapping even needed?)
    let inst_res: Value = pos.func.dfg.first_result(inst);
    let _next_inst: Inst = pos.next_inst().unwrap();

    // Insert a comparison to check if the result of the instruction was NaN,
    // Select a canonical value if NaN, otherwise select the original result.
    // FIXUP: How/Where to define the constant canonical value?
    let is_nan: Value = pos.ins().ffcmp(inst_res, inst_res);

    // Insert the canonical NaN constant value.
    match get_nan_type(&pos.func.dfg, inst) {
        types::F32 => {
            let canon_nan = Ieee32::with_bits(0b01111111100000000000000000000001);
            pos.ins().f32const(canon_nan);
        },
        types::F64 => {
            let canon_nan = Ieee64::with_bits(
                0b0111111111110000000000000000000000000000000000000000000000000001
            );
            pos.ins().f64const(canon_nan);
        }
        _ => unimplemented!() // Should this panic or throw some sort of Error?
    };

    // pos.insert_inst(canonical_nan_inst);
    let new_res: Value = pos.ins().select(is_nan, inst_res, inst_res);

    // Current State:
    // ----------------------------------------------------------------
    // x = [floating point arithmetic instruction]
    // is_nan = x != x
    // canonical_value =  is_nan ? TODO : x
    // [next instruction]                          <-pos
    // ----------------------------------------------------------------

    // Move backwards to the last instruction we inserted, so that we can
    // replace the results of the original instruction with aliases to the
    // results of the select instruction.
    let select_inst: Inst = pos.prev_inst().unwrap();

    // Replace the results of the original floating point arithmetic operation
    // with aliases to the results of the new instruction.
    // FIXUP: Comments for `replace_with_aliases` mention that `dest_inst` may
    // need to be removed from the graph. Does this apply in this case?

    // pos.func.dfg.replace_with_aliases(inst, select_inst);
    // pos.func.dfg.change_to_alias(inst_res, new_res);

    // Remove the original instruction after replacing the aliases.
    // pos.goto_inst(inst);
    // let _removed_inst: Inst = pos.remove_inst();
    // pos.goto_inst(select_inst);
}
