use crate::{error::Error, prelude::Relation};

impl Relation {
    /// Returns tuples common to both relations (`∩`).
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
    /// | `2` |
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
    /// let intersection = a.intersect(&b)?;
    ///
    /// assert_eq!(
    ///     intersection,
    ///     Relation::new_from_iter(
    ///         Heading::try_from(vec![(AttributeName::from("value"), ScalarType::Integer)])?,
    ///         vec![Tuple::try_from(vec![(AttributeName::from("value"), Scalar::Integer(2))])?],
    ///     )?
    /// );
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    pub fn intersect(&self, other: &Relation) -> Result<Relation, Error> {
        if self.heading != other.heading {
            return Err(Error::HeadingMismatch);
        }
        let mut relation = Relation::new(self.heading.clone());
        for tuple in &self.body {
            if other.body.contains(tuple) {
                relation.insert(tuple.clone())?;
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
            relation.intersect(&other).unwrap(),
            Relation::new_from_iter(
                Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
                vec![
                    Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(2))])
                        .unwrap(),
                    Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(3))])
                        .unwrap(),
                ],
            )
            .unwrap()
        );
    }

    #[test]
    fn test_intersect_rejects_incompatible_headings() -> Result<(), Error> {
        let lhs = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
            vec![Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))]).unwrap()],
        )?;
        let rhs = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("bar"), ScalarType::Integer)]).unwrap(),
            vec![Tuple::try_from(vec![(AttributeName::from("bar"), Scalar::Integer(1))]).unwrap()],
        )?;

        assert_eq!(lhs.intersect(&rhs), Err(Error::HeadingMismatch));
        Ok(())
    }

    #[test]
    fn test_intersect_with_self_is_identity() -> Result<(), Error> {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
            vec![
                Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))]).unwrap(),
                Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(2))]).unwrap(),
            ],
        )?;

        assert_eq!(relation.intersect(&relation)?, relation);
        Ok(())
    }
}
