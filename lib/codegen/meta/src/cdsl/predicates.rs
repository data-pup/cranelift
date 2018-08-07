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

// DEVELOPMENT NOTES: These are the imports in the original Python code.
// from .formats import instruction_context
// from .formats import InstructionFormat, InstructionContext, FormatField  # noqa
// from .instructions import Instruction  # noqa
// from .settings import BoolSetting, SettingGroup  # noqa
// from .types import ValueType  # noqa
// from .typevar import TypeVar  # noqa
// PredContext = Union[SettingGroup, InstructionFormat,
// InstructionContext]
// PredLeaf = Union[BoolSetting, 'FieldPredicate', 'TypePredicate',
// 'CtrlTypePredicate']
// PredNode = Union[PredLeaf, 'Predicate']
// # A predicate key is a (recursive) tuple of primitive types that
// # uniquely describes a predicate. It is used for interning.
// PredKey = Tuple[Any, ...]
