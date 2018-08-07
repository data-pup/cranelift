//! Cranelift predicates that consider `Function` fields.

// DEVELOPMENT NOTE: These are the original imports in the Python code.
// from cdsl.predicates import FieldPredicate
// from .formats import UnaryGlobalValue, InstructionFormat
// from cdsl.formats import InstructionFormat, FormatField

// DEVELOPMENT NOTE: These all inherit from FieldPredicate in the original code.

/// An instruction predicate that checks the referenced function is colocated.
struct _IsColocatedFunc;

/// An instruction predicate that checks the referenced data object is
/// colocated.
struct _IsColocatedData;

/// TODO: Document this predicate?
struct _LengthEquals;
