use crate::{error::Error, prelude::Relation};

impl Relation {
    /// Returns the union of two compatible relations (`⋃`).
    ///
    /// `a`
    ///
    /// | `value` |
    /// | --- |
    /// | `1` |
    /// | `2` |
    ///
    /// `b`
    ///
    /// | `value` |
    /// | --- |
    /// | `2` |
    /// | `3` |
    ///
    /// Output
    ///
    /// | `value` |
    /// | --- |
    /// | `1` |
    /// | `2` |
    /// | `3` |
    ///
    /// # Errors
    ///
    /// Returns [`Error::HeadingMismatch`] if the relations do not have the same
    /// heading.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{AttributeName, Heading, Relation, Scalar, ScalarType, Tuple};
    ///
    /// let a = Relation::new_from_iter(
    ///     Heading::try_from(vec![(AttributeName::from("value"), ScalarType::Integer)])?,
    ///     vec![
    ///         Tuple::try_from(vec![(AttributeName::from("value"), Scalar::Integer(1))])?,
    ///         Tuple::try_from(vec![(AttributeName::from("value"), Scalar::Integer(2))])?,
    ///     ],
    /// )?;
    /// let b = Relation::new_from_iter(
    ///     Heading::try_from(vec![(AttributeName::from("value"), ScalarType::Integer)])?,
    ///     vec![
    ///         Tuple::try_from(vec![(AttributeName::from("value"), Scalar::Integer(2))])?,
    ///         Tuple::try_from(vec![(AttributeName::from("value"), Scalar::Integer(3))])?,
    ///     ],
    /// )?;
    ///
    /// let union = a.union(&b)?;
    ///
    /// assert_eq!(
    ///     union,
    ///     Relation::new_from_iter(
    ///         Heading::try_from(vec![(AttributeName::from("value"), ScalarType::Integer)])?,
    ///         vec![
    ///             Tuple::try_from(vec![(AttributeName::from("value"), Scalar::Integer(1))])?,
    ///             Tuple::try_from(vec![(AttributeName::from("value"), Scalar::Integer(2))])?,
    ///             Tuple::try_from(vec![(AttributeName::from("value"), Scalar::Integer(3))])?,
    ///         ],
    ///     )?
    /// );
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    pub fn union(&self, other: &Relation) -> Result<Relation, Error> {
        if self.heading != other.heading {
            return Err(Error::HeadingMismatch);
        }
        let mut relation = Relation::new(self.heading.clone());
        for tuple in self.body.iter().chain(other.body.iter()) {
            relation.insert(tuple.clone())?;
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
    fn test_union() {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
            vec![
                Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))]).unwrap(),
                Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(2))]).unwrap(),
                Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(3))]).unwrap(),
            ],
        )
        .unwrap();
        let other = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
            vec![
                Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(2))]).unwrap(),
                Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(3))]).unwrap(),
                Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(4))]).unwrap(),
            ],
        )
        .unwrap();

        assert_eq!(
            relation.union(&other).unwrap(),
            Relation::new_from_iter(
                Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
                vec![
                    Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))])
                        .unwrap(),
                    Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(2))])
                        .unwrap(),
                    Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(3))])
                        .unwrap(),
                    Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(4))])
                        .unwrap(),
                ],
            )
            .unwrap()
        );
    }

    #[test]
    fn test_union_rejects_incompatible_headings_even_when_other_is_empty() -> Result<(), Error> {
        let lhs = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
            vec![Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))]).unwrap()],
        )?;
        let rhs = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("bar"), ScalarType::Integer)]).unwrap(),
            Vec::new(),
        )?;

        assert_eq!(lhs.union(&rhs), Err(Error::HeadingMismatch));
        Ok(())
    }

    #[test]
    fn test_union_with_empty_relation_is_identity() -> Result<(), Error> {
        let lhs = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
            vec![Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))]).unwrap()],
        )?;
        let rhs = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
            Vec::new(),
        )?;

        assert_eq!(lhs.union(&rhs)?, lhs);
        Ok(())
    }
}
