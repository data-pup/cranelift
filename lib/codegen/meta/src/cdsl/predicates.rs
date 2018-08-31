//! Cranelift predicates.
//!
//! A *predicate* is a function that computes a boolean result. The inputs to the
//! function determine the kind of predicate:
//!
//! - An *ISA predicate* is evaluated on the current ISA settings together with the
//!   shared settings defined in the :py:mod:`settings` module. Once a target ISA
//!   has been configured, the value of all ISA predicates is known.
//!
//! - An *Instruction predicate* is evaluated on an instruction instance, so it can
//!   inspect all the immediate fields and type variables of the instruction.
//!   Instruction predicates can be evaluated before register allocation, so they
//!   can not depend on specific register assignments to the value operands or
//!   outputs.
//!
//! Predicates can also be computed from other predicates using the `And`, `Or`,
//! and `Not` combinators defined in this module.
//!
//! All predicates have a *context* which determines where they can be evaluated.
//! For an ISA predicate, the context is the ISA settings group. For an instruction
//! predicate, the context is the instruction format.

/// Superclass for all computed predicates.
///
/// Leaf predicates can have other types, such as `Setting`.
///
/// :param parts: Tuple of components in the predicate expression.
pub enum _Predicate {
    /// Computed predicate that is true if all parts are true.
    And,
    /// Computed predicate that is true if any parts are true.
    Or,
    /// Computed predicate that is true if its single part is false.
    Not,
}

/// An instruction predicate that performs a test on a single `FormatField`.
///
/// :param field: The `FormatField` to be tested.
/// :param function: Boolean predicate function to call.
/// :param args: Additional arguments for the predicate function.
pub enum _FieldPredicate {
    IsEqual,
    IsZero32BitFloat,
    IsZero64BitFloat,
    IsSignedInt,
    IsUnsignedInt,
}
