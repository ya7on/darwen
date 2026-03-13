use std::fmt::Display;

/// Enumerates the scalar types supported by the engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarType {
    /// Boolean values.
    Boolean,
    /// Signed 64-bit integers.
    Integer,
    /// Owned UTF-8 strings.
    String,
}

impl Display for ScalarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScalarType::Boolean => write!(f, "BOOLEAN"),
            ScalarType::Integer => write!(f, "INTEGER"),
            ScalarType::String => write!(f, "STRING"),
        }
    }
}

/// Stores a typed scalar value used in tuples and predicates.
///
/// # Example
///
/// ```rust
/// use darwen::prelude::Scalar;
///
/// let value = Scalar::Integer(42);
///
/// assert_eq!(value.to_string(), "INTEGER(42)");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scalar {
    /// A boolean scalar value.
    Boolean(bool),
    /// A signed 64-bit integer scalar value.
    Integer(i64),
    /// A string scalar value.
    String(String),
}

impl Scalar {
    /// Returns the scalar type of the value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{Scalar, ScalarType};
    ///
    /// let value = Scalar::String("hello".into());
    ///
    /// assert_eq!(value.ty(), ScalarType::String);
    /// ```
    #[must_use]
    pub fn ty(&self) -> ScalarType {
        match self {
            Scalar::Boolean(_) => ScalarType::Boolean,
            Scalar::Integer(_) => ScalarType::Integer,
            Scalar::String(_) => ScalarType::String,
        }
    }
}

impl From<bool> for Scalar {
    fn from(value: bool) -> Self {
        Scalar::Boolean(value)
    }
}

impl From<i64> for Scalar {
    fn from(value: i64) -> Self {
        Scalar::Integer(value)
    }
}

impl From<String> for Scalar {
    fn from(value: String) -> Self {
        Scalar::String(value)
    }
}

impl From<&str> for Scalar {
    fn from(value: &str) -> Self {
        Scalar::String(value.to_string())
    }
}

impl Display for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scalar::Boolean(value) => write!(f, "BOOLEAN({value})"),
            Scalar::Integer(value) => write!(f, "INTEGER({value})"),
            Scalar::String(value) => write!(f, "STRING({value})"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ty() {
        let scalar = Scalar::Boolean(true);
        assert_eq!(scalar.ty(), ScalarType::Boolean);
        let scalar = Scalar::Integer(42);
        assert_eq!(scalar.ty(), ScalarType::Integer);
    }

    #[test]
    fn test_scalar_variants_are_not_equal() {
        assert_ne!(Scalar::Boolean(true), Scalar::Integer(1));
    }
}
