use std::{collections::BTreeMap, fmt::Display};

use crate::{
    error::Error,
    prelude::Tuple,
    types::{scalar::ScalarType, AttributeName},
};

/// Builds a [`Heading`] from `name = type` pairs.
///
/// # Example
///
/// ```rust
/// use darwen::{
///     heading,
///     prelude::{HeadingBuilder, ScalarType},
/// };
///
/// let heading = heading!(name = ScalarType::String, age = ScalarType::Integer)?;
///
/// assert_eq!(heading.degree(), 2);
/// # Ok::<(), darwen::prelude::Error>(())
/// ```
#[macro_export]
macro_rules! heading {
    ($($key:ident = $value:expr),* $(,)?) => {
        $crate::HeadingBuilder::new()
            $(
                .with_attribute(stringify!($key), $value)
            )*
            .build()
    };
}

/// Builds a [`Heading`] attribute by attribute.
///
/// # Example
///
/// ```rust
/// use darwen::prelude::{AttributeName, HeadingBuilder, ScalarType};
///
/// let heading = HeadingBuilder::new()
///     .with_attribute(AttributeName::from("name"), ScalarType::String)
///     .with_attribute(AttributeName::from("age"), ScalarType::Integer)
///     .build()?;
///
/// assert_eq!(heading.degree(), 2);
/// # Ok::<(), darwen::prelude::Error>(())
/// ```
#[derive(Debug, Default)]
pub struct HeadingBuilder {
    attributes: Vec<(AttributeName, ScalarType)>,
}

impl HeadingBuilder {
    /// Creates an empty heading builder.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::HeadingBuilder;
    ///
    /// let builder = HeadingBuilder::new();
    ///
    /// let _ = builder;
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends an attribute definition to the builder.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{AttributeName, HeadingBuilder, ScalarType};
    ///
    /// let heading = HeadingBuilder::new()
    ///     .with_attribute(AttributeName::from("id"), ScalarType::Integer)
    ///     .build()?;
    ///
    /// assert!(heading.contains(&AttributeName::from("id")));
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    #[must_use]
    pub fn with_attribute<A>(mut self, name: A, ty: ScalarType) -> Self
    where
        A: Into<AttributeName>,
    {
        self.attributes.push((name.into(), ty));
        self
    }

    /// Builds a [`Heading`] from the collected attributes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{AttributeName, HeadingBuilder, ScalarType};
    ///
    /// let heading = HeadingBuilder::new()
    ///     .with_attribute(AttributeName::from("id"), ScalarType::Integer)
    ///     .build()?;
    ///
    /// assert_eq!(heading.degree(), 1);
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidAttribute`] if the builder contains duplicate
    /// attribute names.
    pub fn build(self) -> Result<Heading, Error> {
        Heading::try_from(self.attributes)
    }
}

/// Defines the schema of a relation as a set of named attributes and types.
///
/// # Example
///
/// ```rust
/// use darwen::{heading, prelude::{AttributeName, ScalarType}};
/// # use darwen::prelude::HeadingBuilder;
///
/// let heading = heading!(id = ScalarType::Integer, name = ScalarType::String)?;
///
/// assert!(heading.contains(&AttributeName::from("name")));
/// # Ok::<(), darwen::prelude::Error>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    pub(crate) attributes: BTreeMap<AttributeName, ScalarType>,
}

impl Heading {
    /// Creates a heading from a map of attributes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::BTreeMap;
    ///
    /// use darwen::prelude::{AttributeName, Heading, ScalarType};
    ///
    /// let heading = Heading::new(BTreeMap::from([(
    ///     AttributeName::from("id"),
    ///     ScalarType::Integer,
    /// )]));
    ///
    /// assert_eq!(heading.degree(), 1);
    /// ```
    #[must_use]
    pub fn new(attributes: BTreeMap<AttributeName, ScalarType>) -> Self {
        Self { attributes }
    }

    /// Returns the number of attributes in the heading.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::{heading, prelude::ScalarType};
    /// # use darwen::prelude::HeadingBuilder;
    ///
    /// let heading = heading!(id = ScalarType::Integer, name = ScalarType::String)?;
    ///
    /// assert_eq!(heading.degree(), 2);
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    #[must_use]
    pub fn degree(&self) -> usize {
        self.attributes.len()
    }

    /// Checks whether a tuple conforms to the heading.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::{heading, tuple, prelude::ScalarType};
    /// # use darwen::prelude::{HeadingBuilder, TupleBuilder};
    ///
    /// let heading = heading!(id = ScalarType::Integer)?;
    /// let tuple = tuple!(id = 1)?;
    ///
    /// assert!(heading.validate_tuple(&tuple));
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    #[must_use]
    pub fn validate_tuple(&self, tuple: &Tuple) -> bool {
        if self.degree() != tuple.arity() {
            return false;
        }
        for (name, ty) in &self.attributes {
            let Some(value) = tuple.get(&AttributeName::from(name)) else {
                return false;
            };
            if value.ty() != *ty {
                return false;
            }
        }
        true
    }

    /// Returns the type of an attribute if it exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::{heading, prelude::{AttributeName, ScalarType}};
    /// # use darwen::prelude::HeadingBuilder;
    ///
    /// let heading = heading!(id = ScalarType::Integer)?;
    ///
    /// assert_eq!(
    ///     heading.get(&AttributeName::from("id")),
    ///     Some(&ScalarType::Integer),
    /// );
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    #[must_use]
    pub fn get(&self, name: &AttributeName) -> Option<&ScalarType> {
        self.attributes.get(name)
    }

    /// Returns `true` if the heading contains the attribute.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::{heading, prelude::{AttributeName, ScalarType}};
    /// # use darwen::prelude::HeadingBuilder;
    ///
    /// let heading = heading!(id = ScalarType::Integer)?;
    ///
    /// assert!(heading.contains(&AttributeName::from("id")));
    /// assert!(!heading.contains(&AttributeName::from("age")));
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    #[must_use]
    pub fn contains(&self, name: &AttributeName) -> bool {
        self.attributes.contains_key(name)
    }

    /// Iterates over heading attributes in key order.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::{heading, prelude::{AttributeName, ScalarType}};
    /// # use darwen::prelude::HeadingBuilder;
    ///
    /// let heading = heading!(name = ScalarType::String, id = ScalarType::Integer)?;
    ///
    /// let names = heading.iter().map(|(name, _)| name.clone()).collect::<Vec<_>>();
    ///
    /// assert_eq!(names, vec![AttributeName::from("id"), AttributeName::from("name")]);
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = (&AttributeName, &ScalarType)> {
        self.attributes.iter()
    }

    /// Returns the attribute names shared by two headings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::{heading, prelude::{AttributeName, ScalarType}};
    /// # use darwen::prelude::HeadingBuilder;
    ///
    /// let lhs = heading!(id = ScalarType::Integer, name = ScalarType::String)?;
    /// let rhs = heading!(id = ScalarType::Integer, age = ScalarType::Integer)?;
    ///
    /// assert_eq!(lhs.common(&rhs)?, vec![AttributeName::from("id")]);
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidAttribute`] if the same attribute name exists in
    /// both headings with different scalar types.
    pub fn common(&self, other: &Heading) -> Result<Vec<AttributeName>, Error> {
        let mut common = Vec::with_capacity(self.degree());
        for (attr, ty) in self.iter() {
            let Some(other_ty) = other.get(attr) else {
                continue;
            };
            if ty != other_ty {
                return Err(Error::InvalidAttribute);
            }
            common.push(attr.clone());
        }
        Ok(common)
    }
}

impl TryFrom<Vec<(AttributeName, ScalarType)>> for Heading {
    type Error = Error;

    fn try_from(value: Vec<(AttributeName, ScalarType)>) -> Result<Self, Self::Error> {
        let mut attributes = BTreeMap::new();
        for (name, ty) in value {
            if attributes.insert(name.clone(), ty).is_some() {
                return Err(Error::InvalidAttribute);
            }
        }
        Ok(Self { attributes })
    }
}

impl Display for Heading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ ")?;
        for (name, ty) in &self.attributes {
            write!(f, "{name} {ty}, ")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Scalar;

    use super::*;

    #[test]
    fn test_new() {
        let heading = Heading::try_from(vec![
            ("foo".to_string(), ScalarType::Boolean),
            ("bar".to_string(), ScalarType::Integer),
        ])
        .unwrap();
        assert_eq!(heading.attributes.len(), 2);
        assert_eq!(heading.attributes["foo"], ScalarType::Boolean);
        assert_eq!(heading.attributes["bar"], ScalarType::Integer);
    }

    #[test]
    fn test_degree() {
        let heading = Heading::try_from(vec![
            ("foo".to_string(), ScalarType::Boolean),
            ("bar".to_string(), ScalarType::Integer),
        ])
        .unwrap();
        assert_eq!(heading.degree(), 2);
    }

    #[test]
    fn test_validate_tuple() {
        let heading = Heading::try_from(vec![
            ("foo".to_string(), ScalarType::Boolean),
            ("bar".to_string(), ScalarType::Integer),
        ])
        .unwrap();
        let tuple = Tuple::try_from(vec![
            (AttributeName::from("foo"), Scalar::Boolean(true)),
            (AttributeName::from("bar"), Scalar::Integer(42)),
        ])
        .unwrap();
        assert!(heading.validate_tuple(&tuple));
    }

    #[test]
    fn test_validate_tuple_mismatch() {
        let heading = Heading::try_from(vec![
            ("foo".to_string(), ScalarType::Boolean),
            ("bar".to_string(), ScalarType::Integer),
        ])
        .unwrap();
        let tuple = Tuple::try_from(vec![
            (AttributeName::from("foo"), Scalar::Boolean(true)),
            (AttributeName::from("bar"), Scalar::Boolean(false)),
        ])
        .unwrap();
        assert!(!heading.validate_tuple(&tuple));
    }

    #[test]
    fn test_common_returns_shared_attributes() -> Result<(), Error> {
        let lhs = Heading::try_from(vec![
            (AttributeName::from("foo"), ScalarType::Boolean),
            (AttributeName::from("bar"), ScalarType::Integer),
        ])
        .unwrap();
        let rhs = Heading::try_from(vec![
            (AttributeName::from("bar"), ScalarType::Integer),
            (AttributeName::from("baz"), ScalarType::Boolean),
        ])
        .unwrap();

        assert_eq!(lhs.common(&rhs)?, vec![AttributeName::from("bar")]);
        Ok(())
    }

    #[test]
    fn test_common_rejects_type_mismatches() {
        let lhs =
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap();
        let rhs =
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Boolean)]).unwrap();

        assert_eq!(lhs.common(&rhs), Err(Error::InvalidAttribute));
    }

    #[test]
    #[should_panic]
    fn test_heading_from_rejects_duplicate_attribute_names() {
        let _ = Heading::try_from(vec![
            (AttributeName::from("foo"), ScalarType::Integer),
            (AttributeName::from("foo"), ScalarType::Boolean),
        ])
        .unwrap();
    }
}
