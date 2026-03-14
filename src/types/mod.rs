use std::fmt::Display;

pub mod heading;
pub mod predicate;
pub mod relation;
pub mod scalar;
pub mod tuple;

/// The name of an attribute in a heading or tuple.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttributeName(String);

impl From<&str> for AttributeName {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for AttributeName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for AttributeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
