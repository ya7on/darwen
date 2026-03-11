/// Errors returned by relation construction and relational algebra operations.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// The tuple is malformed or does not match the expected heading.
    InvalidTuple,
    /// The attribute is missing, duplicated, or invalid for the operation.
    InvalidAttribute,
    /// The heading is missing or incompatible with the requested operation.
    InvalidHeading,
}
