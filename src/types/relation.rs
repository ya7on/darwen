use std::{collections::BTreeSet, fmt::Display};

use crate::{
    error::Error,
    types::{heading::Heading, tuple::Tuple},
};

/// Builds a [`Relation`] from a heading and a body of tuples.
///
/// # Example
///
/// ```rust
/// use darwen::prelude::{
///     AttributeName, HeadingBuilder, RelationBuilder, Scalar, ScalarType, TupleBuilder,
/// };
///
/// let relation = RelationBuilder::new()
///     .with_heading(
///         HeadingBuilder::new()
///             .with_attribute(AttributeName::from("id"), ScalarType::Integer)
///             .build()?,
///     )
///     .with_body(vec![
///         TupleBuilder::new()
///             .with_value(AttributeName::from("id"), Scalar::Integer(1))
///             .build()?,
///     ])
///     .build()?;
///
/// assert_eq!(
///     relation,
///     darwen::prelude::Relation::new_from_iter(
///         HeadingBuilder::new()
///             .with_attribute(AttributeName::from("id"), ScalarType::Integer)
///             .build()?,
///         vec![
///             TupleBuilder::new()
///                 .with_value(AttributeName::from("id"), Scalar::Integer(1))
///                 .build()?,
///         ],
///     )?,
/// );
/// # Ok::<(), darwen::prelude::Error>(())
/// ```
#[derive(Debug, Default)]
pub struct RelationBuilder {
    heading: Option<Heading>,
    body: Vec<Tuple>,
}

impl RelationBuilder {
    /// Creates an empty relation builder.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::RelationBuilder;
    ///
    /// let builder = RelationBuilder::new();
    ///
    /// let _ = builder;
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the relation heading.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{AttributeName, HeadingBuilder, RelationBuilder, ScalarType};
    ///
    /// let relation = RelationBuilder::new()
    ///     .with_heading(
    ///         HeadingBuilder::new()
    ///             .with_attribute(AttributeName::from("id"), ScalarType::Integer)
    ///             .build()?,
    ///     )
    ///     .build()?;
    ///
    /// let _ = relation;
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    #[must_use]
    pub fn with_heading(mut self, heading: Heading) -> Self {
        self.heading = Some(heading);
        self
    }

    /// Replaces the builder body with tuples from the iterator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{
    ///     AttributeName, HeadingBuilder, RelationBuilder, Scalar, ScalarType, TupleBuilder,
    /// };
    ///
    /// let relation = RelationBuilder::new()
    ///     .with_heading(
    ///         HeadingBuilder::new()
    ///             .with_attribute(AttributeName::from("id"), ScalarType::Integer)
    ///             .build()?,
    ///     )
    ///     .with_body(vec![
    ///         TupleBuilder::new()
    ///             .with_value(AttributeName::from("id"), Scalar::Integer(1))
    ///             .build()?,
    ///     ])
    ///     .build()?;
    ///
    /// let _ = relation;
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    #[must_use]
    pub fn with_body<T>(mut self, body: T) -> Self
    where
        T: IntoIterator<Item = Tuple>,
    {
        self.body = body.into_iter().collect();
        self
    }

    /// Builds a [`Relation`] from the collected heading and tuples.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{
    ///     AttributeName, HeadingBuilder, RelationBuilder, Scalar, ScalarType, TupleBuilder,
    /// };
    ///
    /// let relation = RelationBuilder::new()
    ///     .with_heading(
    ///         HeadingBuilder::new()
    ///             .with_attribute(AttributeName::from("id"), ScalarType::Integer)
    ///             .build()?,
    ///     )
    ///     .with_body(vec![
    ///         TupleBuilder::new()
    ///             .with_value(AttributeName::from("id"), Scalar::Integer(1))
    ///             .build()?,
    ///     ])
    ///     .build()?;
    ///
    /// let _ = relation;
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::HeadingMissing`] if the heading was not provided.
    /// Returns [`Error::InvalidWidth`] if one of the body tuples has a
    /// different arity than the heading degree.
    /// Returns [`Error::AttributeNotFound`] if one of the body tuples is
    /// missing an attribute required by the heading.
    /// Returns [`Error::ScalarTypeMismatch`] if one of the body tuples
    /// contains a value with a different scalar type than the heading
    /// requires.
    pub fn build(self) -> Result<Relation, Error> {
        let heading = self.heading.ok_or(Error::HeadingMissing)?;
        let mut relation = Relation::new(heading);
        for tuple in self.body {
            relation.insert(tuple)?;
        }
        Ok(relation)
    }
}

/// Represents a relation with a heading and a set of tuples.
///
/// # Example
///
/// ```rust
/// use darwen::prelude::{AttributeName, HeadingBuilder, Relation, ScalarType};
///
/// let relation = Relation::new(
///     HeadingBuilder::new()
///         .with_attribute(AttributeName::from("id"), ScalarType::Integer)
///         .build()?,
/// );
///
/// assert_eq!(
///     relation,
///     Relation::new_from_iter(
///         HeadingBuilder::new()
///             .with_attribute(AttributeName::from("id"), ScalarType::Integer)
///             .build()?,
///         Vec::new(),
///     )?,
/// );
/// # Ok::<(), darwen::prelude::Error>(())
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Relation {
    pub(crate) heading: Heading,
    pub(crate) body: BTreeSet<Tuple>,
}

impl Relation {
    /// Creates an empty relation with the given heading.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{AttributeName, HeadingBuilder, Relation, ScalarType};
    ///
    /// let relation = Relation::new(
    ///     HeadingBuilder::new()
    ///         .with_attribute(AttributeName::from("id"), ScalarType::Integer)
    ///         .build()?,
    /// );
    ///
    /// assert_eq!(
    ///     relation,
    ///     Relation::new_from_iter(
    ///         HeadingBuilder::new()
    ///             .with_attribute(AttributeName::from("id"), ScalarType::Integer)
    ///             .build()?,
    ///         Vec::new(),
    ///     )?,
    /// );
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    #[must_use]
    pub fn new(heading: Heading) -> Self {
        Self {
            heading,
            body: BTreeSet::new(),
        }
    }

    /// Builds a relation from a heading and an iterator of tuples.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{
    ///     AttributeName, HeadingBuilder, Relation, Scalar, ScalarType, TupleBuilder,
    /// };
    ///
    /// let relation = Relation::new_from_iter(
    ///     HeadingBuilder::new()
    ///         .with_attribute(AttributeName::from("id"), ScalarType::Integer)
    ///         .build()?,
    ///     vec![
    ///         TupleBuilder::new()
    ///             .with_value(AttributeName::from("id"), Scalar::Integer(1))
    ///             .build()?,
    ///     ],
    /// )?;
    ///
    /// let _ = relation;
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidWidth`] if any tuple has a different arity than
    /// the provided heading degree.
    /// Returns [`Error::AttributeNotFound`] if any tuple is missing an
    /// attribute required by the provided heading.
    /// Returns [`Error::ScalarTypeMismatch`] if any tuple contains a value with
    /// a different scalar type than the provided heading requires.
    pub fn new_from_iter<T>(heading: Heading, body: T) -> Result<Self, Error>
    where
        T: IntoIterator<Item = Tuple>,
    {
        let mut relation = Self::new(heading);
        for item in body {
            relation.insert(item)?;
        }
        Ok(relation)
    }

    /// Inserts a tuple into the relation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{
    ///     AttributeName, HeadingBuilder, Relation, Scalar, ScalarType, TupleBuilder,
    /// };
    ///
    /// let mut relation = Relation::new(
    ///     HeadingBuilder::new()
    ///         .with_attribute(AttributeName::from("id"), ScalarType::Integer)
    ///         .build()?,
    /// );
    /// relation.insert(
    ///     TupleBuilder::new()
    ///         .with_value(AttributeName::from("id"), Scalar::Integer(1))
    ///         .build()?,
    /// )?;
    ///
    /// assert_eq!(
    ///     relation,
    ///     Relation::new_from_iter(
    ///         HeadingBuilder::new()
    ///             .with_attribute(AttributeName::from("id"), ScalarType::Integer)
    ///             .build()?,
    ///         vec![
    ///             TupleBuilder::new()
    ///                 .with_value(AttributeName::from("id"), Scalar::Integer(1))
    ///                 .build()?,
    ///         ],
    ///     )?,
    /// );
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidWidth`] if the tuple arity does not match the
    /// relation heading degree.
    /// Returns [`Error::AttributeNotFound`] if the tuple is missing an
    /// attribute required by the relation heading.
    /// Returns [`Error::ScalarTypeMismatch`] if the tuple contains a value with
    /// a different scalar type than the relation heading requires.
    pub fn insert(&mut self, tuple: Tuple) -> Result<(), Error> {
        self.heading.validate_tuple(&tuple)?;
        self.body.insert(tuple);
        Ok(())
    }
}

impl Display for Relation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RELATION ")?;
        write!(f, "{}", self.heading)?;
        for tuple in &self.body {
            write!(f, "\n\t{tuple}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{AttributeName, Scalar, ScalarType};

    use super::*;

    #[test]
    fn test_insert() {
        let mut relation = Relation::new(
            Heading::try_from(vec![
                ("a".to_string(), ScalarType::Integer),
                ("b".to_string(), ScalarType::Integer),
            ])
            .unwrap(),
        );
        let tuple =
            Tuple::try_from(vec![("a", Scalar::Integer(1)), ("b", Scalar::Integer(2))]).unwrap();
        assert!(relation.insert(tuple).is_ok());
    }

    #[test]
    fn test_insert_invalid_tuple() {
        let mut relation = Relation::new(
            Heading::try_from(vec![
                ("a".to_string(), ScalarType::Integer),
                ("b".to_string(), ScalarType::Integer),
            ])
            .unwrap(),
        );
        let tuple = Tuple::try_from(vec![
            ("a".to_string(), Scalar::Integer(1)),
            ("b".to_string(), Scalar::Boolean(true)),
        ])
        .unwrap();
        assert!(relation.insert(tuple).is_err());
    }

    #[test]
    fn test_new_creates_empty_body() {
        let relation = Relation::new(
            Heading::try_from(vec![(AttributeName::from("a"), ScalarType::Integer)]).unwrap(),
        );

        assert!(relation.body.is_empty());
    }

    #[test]
    fn test_new_from_iter_rejects_invalid_tuple() {
        assert_eq!(
            Relation::new_from_iter(
                Heading::try_from(vec![(AttributeName::from("a"), ScalarType::Integer)]).unwrap(),
                vec![
                    Tuple::try_from(vec![(AttributeName::from("a"), Scalar::Boolean(true))])
                        .unwrap(),
                ],
            ),
            Err(Error::ScalarTypeMismatch {
                lhs: ScalarType::Boolean,
                rhs: ScalarType::Integer
            })
        );
    }

    #[test]
    fn test_new_from_iter_deduplicates_duplicate_tuples() -> Result<(), Error> {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("a"), ScalarType::Integer)]).unwrap(),
            vec![
                Tuple::try_from(vec![(AttributeName::from("a"), Scalar::Integer(1))]).unwrap(),
                Tuple::try_from(vec![(AttributeName::from("a"), Scalar::Integer(1))]).unwrap(),
            ],
        )?;

        assert_eq!(relation.body.len(), 1);
        Ok(())
    }
}
