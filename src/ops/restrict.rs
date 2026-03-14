use crate::{
    error::Error,
    prelude::{Predicate, Relation},
};

impl Relation {
    /// Restricts a relation to tuples that satisfy a predicate (`σ`).
    ///
    /// `people`
    ///
    /// | `id` | `age` |
    /// | --- | --- |
    /// | `1` | `19` |
    /// | `2` | `24` |
    ///
    /// Output
    ///
    /// | `id` | `age` |
    /// | --- | --- |
    /// | `2` | `24` |
    ///
    /// # Errors
    ///
    /// Returns [`Error::AttributeNotFound`] if the predicate references an
    /// attribute that does not exist in the relation tuples.
    /// Returns [`Error::ScalarTypeMismatch`] if the predicate uses `Eq` to
    /// compare values of different scalar types.
    /// Returns [`Error::NonComparableTypes`] if the predicate uses `<` or `>`
    /// with non-integer operands.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{
    ///     AttributeName, Heading, Predicate, Relation, Scalar, ScalarType, Tuple,
    /// };
    ///
    /// let people = Relation::new_from_iter(
    ///     Heading::try_from(vec![
    ///         (AttributeName::from("id"), ScalarType::Integer),
    ///         (AttributeName::from("age"), ScalarType::Integer),
    ///     ])?,
    ///     vec![
    ///         Tuple::try_from(vec![
    ///             (AttributeName::from("id"), Scalar::Integer(1)),
    ///             (AttributeName::from("age"), Scalar::Integer(19)),
    ///         ])?,
    ///         Tuple::try_from(vec![
    ///             (AttributeName::from("id"), Scalar::Integer(2)),
    ///             (AttributeName::from("age"), Scalar::Integer(24)),
    ///         ])?,
    ///     ],
    /// )?;
    ///
    /// let adults = people.restrict(&Predicate::eq(
    ///     AttributeName::from("age"),
    ///     Scalar::Integer(24),
    /// ))?;
    ///
    /// assert_eq!(
    ///     adults,
    ///     Relation::new_from_iter(
    ///         Heading::try_from(vec![
    ///             (AttributeName::from("id"), ScalarType::Integer),
    ///             (AttributeName::from("age"), ScalarType::Integer),
    ///         ])?,
    ///         vec![Tuple::try_from(vec![
    ///             (AttributeName::from("id"), Scalar::Integer(2)),
    ///             (AttributeName::from("age"), Scalar::Integer(24)),
    ///         ])?],
    ///     )?
    /// );
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    pub fn restrict(&self, predicate: &Predicate) -> Result<Relation, Error> {
        let mut relation = Relation::new(self.heading.clone());
        for tuple in &self.body {
            if predicate.eval(tuple)? {
                relation.body.insert(tuple.clone());
            }
        }
        Ok(relation)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        prelude::{Heading, Scalar, ScalarType, Tuple},
        types::AttributeName,
    };

    use super::*;

    #[test]
    fn test_restrict() {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
            vec![
                Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))]).unwrap(),
                Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(2))]).unwrap(),
                Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(3))]).unwrap(),
            ],
        )
        .unwrap();

        assert_eq!(
            relation
                .restrict(&Predicate::eq(
                    AttributeName::from("foo"),
                    Scalar::Integer(2)
                ))
                .unwrap(),
            Relation::new_from_iter(
                Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
                vec![
                    Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(2))])
                        .unwrap(),
                ],
            )
            .unwrap()
        );
    }

    #[test]
    fn test_restrict_returns_empty_relation_when_nothing_matches() -> Result<(), Error> {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
            vec![Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))]).unwrap()],
        )?;

        assert_eq!(
            relation.restrict(&Predicate::eq(
                AttributeName::from("foo"),
                Scalar::Integer(2)
            ))?,
            Relation::new_from_iter(
                Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
                Vec::new(),
            )?
        );
        Ok(())
    }

    #[test]
    fn test_restrict_returns_error_for_unknown_attribute() -> Result<(), Error> {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
            vec![Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))]).unwrap()],
        )?;

        assert_eq!(
            relation.restrict(&Predicate::eq(
                AttributeName::from("bar"),
                Scalar::Integer(1)
            )),
            Err(Error::AttributeNotFound {
                name: AttributeName::from("bar")
            })
        );
        Ok(())
    }
}
