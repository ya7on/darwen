use crate::{AttributeName, ScalarType};

/// Errors returned by relation construction and relational algebra operations.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// A tuple has a different number of attributes than the heading requires.
    InvalidWidth {
        /// The number of attributes required by the heading.
        expected: usize,
        /// The number of attributes actually present in the tuple.
        actual: usize,
    },
    /// An operation referenced an attribute that does not exist.
    AttributeNotFound {
        /// The missing attribute name.
        name: AttributeName,
    },
    /// Two scalar types were expected to match, but they differ.
    ScalarTypeMismatch {
        /// The left-hand scalar type.
        lhs: ScalarType,
        /// The right-hand scalar type.
        rhs: ScalarType,
    },
    /// An operation attempted to create or reuse a duplicate attribute name.
    AttributeAlreadyExists {
        /// The duplicate attribute name.
        name: AttributeName,
    },
    /// An ordered comparison was requested for non-comparable scalar types.
    NonComparableTypes {
        /// The left-hand scalar type.
        lhs: ScalarType,
        /// The right-hand scalar type.
        rhs: ScalarType,
    },
    /// A relation operation requiring identical headings received different headings.
    HeadingMismatch,
    /// A relation builder was finalized without a heading.
    HeadingMissing,
    /// A rename mapping is structurally invalid.
    InvalidRenameMapping {
        /// The duplicated source attribute name in the rename mapping.
        name: AttributeName,
    },
}
