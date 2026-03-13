use std::{collections::BTreeMap, fmt::Display};

use crate::{
    error::Error,
    types::{scalar::Scalar, AttributeName},
};

/// Builds a [`Tuple`] from `name = value` pairs.
///
/// # Example
///
/// ```rust
/// use darwen::{tuple, prelude::TupleBuilder};
///
/// let tuple = tuple!(id = 1, name = "Monica")?;
///
/// assert_eq!(tuple.arity(), 2);
/// # Ok::<(), darwen::prelude::Error>(())
/// ```
#[macro_export]
macro_rules! tuple {
    ($($key:ident = $value:expr),* $(,)?) => {
        TupleBuilder::new()
            $(
                .with_value(stringify!($key), $value)
            )*
            .build()
    };
}

/// Builds a [`Tuple`] value by adding attributes one by one.
///
/// # Example
///
/// ```rust
/// use darwen::prelude::{AttributeName, Scalar, TupleBuilder};
///
/// let tuple = TupleBuilder::new()
///     .with_value(AttributeName::from("id"), Scalar::Integer(1))
///     .build()?;
///
/// assert_eq!(tuple.arity(), 1);
/// # Ok::<(), darwen::prelude::Error>(())
/// ```
#[derive(Debug, Default)]
pub struct TupleBuilder {
    values: Vec<(AttributeName, Scalar)>,
}

impl TupleBuilder {
    /// Creates an empty tuple builder.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::TupleBuilder;
    ///
    /// let builder = TupleBuilder::new();
    ///
    /// let _ = builder;
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an attribute value to the builder.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{AttributeName, Scalar, TupleBuilder};
    ///
    /// let tuple = TupleBuilder::new()
    ///     .with_value(AttributeName::from("name"), Scalar::String("Monica".into()))
    ///     .build()?;
    ///
    /// assert_eq!(
    ///     tuple.get(&AttributeName::from("name")),
    ///     Some(&Scalar::String("Monica".into())),
    /// );
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    #[must_use]
    pub fn with_value<A, S>(mut self, attribute: A, value: S) -> Self
    where
        A: Into<AttributeName>,
        S: Into<Scalar>,
    {
        self.values.push((attribute.into(), value.into()));
        self
    }

    /// Builds a [`Tuple`] from the collected values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{AttributeName, Scalar, TupleBuilder};
    ///
    /// let tuple = TupleBuilder::new()
    ///     .with_value(AttributeName::from("id"), Scalar::Integer(1))
    ///     .build()?;
    ///
    /// assert_eq!(tuple.arity(), 1);
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidTuple`] if the builder contains duplicate
    /// attribute names.
    pub fn build(self) -> Result<Tuple, Error> {
        Tuple::try_from(self.values)
    }
}

/// Represents one tuple in a relation body.
///
/// # Example
///
/// ```rust
/// use darwen::prelude::{AttributeName, Scalar, TupleBuilder};
///
/// let tuple = TupleBuilder::new()
///     .with_value(AttributeName::from("id"), Scalar::Integer(1))
///     .with_value(AttributeName::from("name"), Scalar::String("Monica".into()))
///     .build()?;
///
/// assert_eq!(tuple.arity(), 2);
/// # Ok::<(), darwen::prelude::Error>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tuple {
    pub(crate) values: BTreeMap<AttributeName, Scalar>,
}

impl Tuple {
    /// Returns the number of attributes stored in the tuple.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{AttributeName, Scalar, TupleBuilder};
    ///
    /// let tuple = TupleBuilder::new()
    ///     .with_value(AttributeName::from("id"), Scalar::Integer(1))
    ///     .with_value(AttributeName::from("active"), Scalar::Boolean(true))
    ///     .build()?;
    ///
    /// assert_eq!(tuple.arity(), 2);
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    #[must_use]
    pub fn arity(&self) -> usize {
        self.values.len()
    }

    /// Returns the value of an attribute if it exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{AttributeName, Scalar, TupleBuilder};
    ///
    /// let tuple = TupleBuilder::new()
    ///     .with_value(AttributeName::from("id"), Scalar::Integer(1))
    ///     .build()?;
    ///
    /// assert_eq!(
    ///     tuple.get(&AttributeName::from("id")),
    ///     Some(&Scalar::Integer(1)),
    /// );
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    #[must_use]
    pub fn get(&self, attribute: &AttributeName) -> Option<&Scalar> {
        self.values.get(attribute)
    }

    /// Iterates over tuple values in attribute-name order.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{AttributeName, Scalar, TupleBuilder};
    ///
    /// let tuple = TupleBuilder::new()
    ///     .with_value(AttributeName::from("name"), Scalar::String("Monica".into()))
    ///     .with_value(AttributeName::from("id"), Scalar::Integer(1))
    ///     .build()?;
    ///
    /// let names = tuple.iter().map(|(name, _)| name.clone()).collect::<Vec<_>>();
    ///
    /// assert_eq!(names, vec![AttributeName::from("id"), AttributeName::from("name")]);
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = (&AttributeName, &Scalar)> {
        self.values.iter()
    }
}

impl TryFrom<Vec<(AttributeName, Scalar)>> for Tuple {
    type Error = Error;

    fn try_from(input: Vec<(AttributeName, Scalar)>) -> Result<Self, Self::Error> {
        let mut values = BTreeMap::new();
        for (attribute, value) in input {
            if values.insert(attribute, value).is_some() {
                return Err(Error::InvalidTuple);
            }
        }

        Ok(Self { values })
    }
}

impl From<BTreeMap<AttributeName, Scalar>> for Tuple {
    fn from(input: BTreeMap<AttributeName, Scalar>) -> Self {
        Self { values: input }
    }
}

impl Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TUPLE {{ ")?;
        for (name, value) in &self.values {
            write!(f, "{name} {value}, ")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn test_arity() {
        let input = vec![(AttributeName::from("foo"), Scalar::Integer(42))];
        let tuple = Tuple::try_from(input).unwrap();
        assert_eq!(tuple.arity(), 1);
    }

    #[test]
    fn test_get() {
        let input = vec![(AttributeName::from("foo"), Scalar::Integer(42))];
        let tuple = Tuple::try_from(input).unwrap();
        assert_eq!(
            tuple.get(&AttributeName::from("foo")).unwrap(),
            &Scalar::Integer(42)
        );
    }

    #[test]
    fn test_from() {
        let input = vec![(AttributeName::from("foo"), Scalar::Integer(42))];
        let tuple = Tuple::try_from(input).unwrap();
        assert_eq!(tuple.values.len(), 1);
        assert_eq!(
            tuple.values.get(&AttributeName::from("foo")).unwrap(),
            &Scalar::Integer(42)
        );
    }

    #[test]
    fn test_tuple_equality() {
        let tuple1 = Tuple::try_from(vec![
            (AttributeName::from("foo"), Scalar::Integer(42)),
            (AttributeName::from("bar"), Scalar::Boolean(true)),
        ])
        .unwrap();
        let tuple2 = Tuple::try_from(vec![
            (AttributeName::from("bar"), Scalar::Boolean(true)),
            (AttributeName::from("foo"), Scalar::Integer(42)),
        ])
        .unwrap();
        assert_eq!(tuple1, tuple2);
    }

    #[test]
    fn test_tuple_not_equality() {
        let tuple1 =
            Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(42))]).unwrap();
        let tuple2 =
            Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Boolean(true))]).unwrap();
        assert_ne!(tuple1, tuple2);
    }

    #[test]
    fn test_get_missing_attribute_returns_none() {
        let tuple =
            Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(42))]).unwrap();
        assert_eq!(tuple.get(&AttributeName::from("bar")), None);
    }

    #[test]
    fn test_from_btreemap() {
        let mut values = BTreeMap::new();
        values.insert(AttributeName::from("foo"), Scalar::Integer(42));

        let tuple = Tuple::try_from(values).unwrap();

        assert_eq!(tuple.arity(), 1);
        assert_eq!(
            tuple.get(&AttributeName::from("foo")),
            Some(&Scalar::Integer(42))
        );
    }

    #[test]
    fn test_tuple_from_rejects_duplicate_attribute_names() {
        assert!(Tuple::try_from(vec![
            (AttributeName::from("foo"), Scalar::Integer(1)),
            (AttributeName::from("foo"), Scalar::Integer(2)),
        ])
        .is_err());
    }
}
