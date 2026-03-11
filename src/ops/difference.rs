use crate::{error::Error, prelude::Relation};

impl Relation {
    /// Returns tuples that exist in `self` but not in `other` (`−`).
    ///
    /// `a`
    ///
    /// | `value` |
    /// | --- |
    /// | `1` |
    /// | `2` |
    /// | `3` |
    ///
    /// `b`
    ///
    /// | `value` |
    /// | --- |
    /// | `2` |
    ///
    /// Output
    ///
    /// | `value` |
    /// | --- |
    /// | `1` |
    /// | `3` |
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidHeading`] if the relations do not have the same
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
    ///         Tuple::try_from(vec![(AttributeName::from("value"), Scalar::Integer(3))])?,
    ///     ],
    /// )?;
    /// let b = Relation::new_from_iter(
    ///     Heading::try_from(vec![(AttributeName::from("value"), ScalarType::Integer)])?,
    ///     vec![Tuple::try_from(vec![(AttributeName::from("value"), Scalar::Integer(2))])?],
    /// )?;
    ///
    /// let difference = a.difference(&b)?;
    ///
    /// assert_eq!(
    ///     difference,
    ///     Relation::new_from_iter(
    ///         Heading::try_from(vec![(AttributeName::from("value"), ScalarType::Integer)])?,
    ///         vec![
    ///             Tuple::try_from(vec![(AttributeName::from("value"), Scalar::Integer(1))])?,
    ///             Tuple::try_from(vec![(AttributeName::from("value"), Scalar::Integer(3))])?,
    ///         ],
    ///     )?
    /// );
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    pub fn difference(&self, other: &Relation) -> Result<Relation, Error> {
        if self.heading != other.heading {
            return Err(Error::InvalidHeading);
        }
        let mut relation = Relation::new(self.heading.clone());
        for tuple in &self.body {
            if other.body.contains(tuple) {
                continue;
            }
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
    fn test_difference() {
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
            relation.difference(&other).unwrap(),
            Relation::new_from_iter(
                Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
                vec![
                    Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))])
                        .unwrap()
                ],
            )
            .unwrap()
        );

        assert_eq!(
            other.difference(&relation).unwrap(),
            Relation::new_from_iter(
                Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
                vec![
                    Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(4))])
                        .unwrap()
                ],
            )
            .unwrap()
        );
    }

    #[test]
    fn test_difference_rejects_incompatible_headings() -> Result<(), Error> {
        let lhs = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
            vec![Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))]).unwrap()],
        )?;
        let rhs = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("bar"), ScalarType::Integer)]).unwrap(),
            vec![Tuple::try_from(vec![(AttributeName::from("bar"), Scalar::Integer(1))]).unwrap()],
        )?;

        assert_eq!(lhs.difference(&rhs), Err(Error::InvalidHeading));
        Ok(())
    }

    #[test]
    fn test_difference_with_self_is_empty() -> Result<(), Error> {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
            vec![
                Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))]).unwrap(),
                Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(2))]).unwrap(),
            ],
        )?;

        assert_eq!(
            relation.difference(&relation)?,
            Relation::new_from_iter(
                Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
                Vec::new(),
            )?
        );
        Ok(())
    }
}
