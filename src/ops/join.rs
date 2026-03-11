use crate::{
    error::Error,
    prelude::{Heading, Relation, Tuple},
};

impl Relation {
    /// Performs a natural join on attributes shared by both relations (`⋈`).
    ///
    /// `users`
    ///
    /// | `id` | `score` |
    /// | --- | --- |
    /// | `1` | `10` |
    /// | `2` | `20` |
    ///
    /// `cities`
    ///
    /// | `id` | `level` |
    /// | --- | --- |
    /// | `1` | `100` |
    /// | `3` | `300` |
    ///
    /// Output
    ///
    /// | `id` | `score` | `level` |
    /// | --- | --- | --- |
    /// | `1` | `10` | `100` |
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidAttribute`] if shared attribute names have
    /// incompatible scalar types or if tuple merging produces an invalid
    /// heading.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{AttributeName, Heading, Relation, Scalar, ScalarType, Tuple};
    ///
    /// let users = Relation::new_from_iter(
    ///     Heading::try_from(vec![
    ///         (AttributeName::from("id"), ScalarType::Integer),
    ///         (AttributeName::from("score"), ScalarType::Integer),
    ///     ])?,
    ///     vec![
    ///         Tuple::try_from(vec![
    ///             (AttributeName::from("id"), Scalar::Integer(1)),
    ///             (AttributeName::from("score"), Scalar::Integer(10)),
    ///         ])?,
    ///         Tuple::try_from(vec![
    ///             (AttributeName::from("id"), Scalar::Integer(2)),
    ///             (AttributeName::from("score"), Scalar::Integer(20)),
    ///         ])?,
    ///     ],
    /// )?;
    /// let cities = Relation::new_from_iter(
    ///     Heading::try_from(vec![
    ///         (AttributeName::from("id"), ScalarType::Integer),
    ///         (AttributeName::from("level"), ScalarType::Integer),
    ///     ])?,
    ///     vec![
    ///         Tuple::try_from(vec![
    ///             (AttributeName::from("id"), Scalar::Integer(1)),
    ///             (AttributeName::from("level"), Scalar::Integer(100)),
    ///         ])?,
    ///         Tuple::try_from(vec![
    ///             (AttributeName::from("id"), Scalar::Integer(3)),
    ///             (AttributeName::from("level"), Scalar::Integer(300)),
    ///         ])?,
    ///     ],
    /// )?;
    ///
    /// let users_with_levels = users.join(&cities)?;
    ///
    /// assert_eq!(
    ///     users_with_levels,
    ///     Relation::new_from_iter(
    ///         Heading::try_from(vec![
    ///             (AttributeName::from("id"), ScalarType::Integer),
    ///             (AttributeName::from("score"), ScalarType::Integer),
    ///             (AttributeName::from("level"), ScalarType::Integer),
    ///         ])?,
    ///         vec![Tuple::try_from(vec![
    ///             (AttributeName::from("id"), Scalar::Integer(1)),
    ///             (AttributeName::from("score"), Scalar::Integer(10)),
    ///             (AttributeName::from("level"), Scalar::Integer(100)),
    ///         ])?],
    ///     )?
    /// );
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    pub fn join(&self, other: &Relation) -> Result<Relation, Error> {
        let common = self.heading.common(&other.heading)?;

        if common.is_empty() {
            // TODO: error?
            return self.product(other);
        }

        let mut attributes = self.heading.attributes.clone();
        for (name, ty) in other.heading.iter() {
            if !attributes.contains_key(name) {
                attributes.insert(name.clone(), *ty);
            }
        }

        let mut result = Relation::new(Heading::new(attributes));

        for lhs in &self.body {
            for rhs in &other.body {
                if !common.iter().all(|name| lhs.get(name) == rhs.get(name)) {
                    continue;
                }

                let mut values = lhs.values.clone();
                for (name, value) in rhs.iter() {
                    if !values.contains_key(name) {
                        values.insert(name.clone(), value.clone());
                    }
                }

                result.insert(Tuple::from(values))?;
            }
        }

        Ok(result)
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
    fn test_join() {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![
                (AttributeName::from("a"), ScalarType::Boolean),
                (AttributeName::from("b"), ScalarType::Integer),
            ])
            .unwrap(),
            vec![
                Tuple::try_from(vec![
                    (AttributeName::from("a"), Scalar::Boolean(true)),
                    (AttributeName::from("b"), Scalar::Integer(1)),
                ])
                .unwrap(),
                Tuple::try_from(vec![
                    (AttributeName::from("a"), Scalar::Boolean(true)),
                    (AttributeName::from("b"), Scalar::Integer(2)),
                ])
                .unwrap(),
                Tuple::try_from(vec![
                    (AttributeName::from("a"), Scalar::Boolean(true)),
                    (AttributeName::from("b"), Scalar::Integer(3)),
                ])
                .unwrap(),
            ],
        )
        .unwrap();
        let other = Relation::new_from_iter(
            Heading::try_from(vec![
                (AttributeName::from("b"), ScalarType::Integer),
                (AttributeName::from("c"), ScalarType::Integer),
            ])
            .unwrap(),
            vec![
                Tuple::try_from(vec![
                    (AttributeName::from("b"), Scalar::Integer(1)),
                    (AttributeName::from("c"), Scalar::Integer(100)),
                ])
                .unwrap(),
                Tuple::try_from(vec![
                    (AttributeName::from("b"), Scalar::Integer(3)),
                    (AttributeName::from("c"), Scalar::Integer(300)),
                ])
                .unwrap(),
                Tuple::try_from(vec![
                    (AttributeName::from("b"), Scalar::Integer(5)),
                    (AttributeName::from("c"), Scalar::Integer(500)),
                ])
                .unwrap(),
            ],
        )
        .unwrap();

        assert_eq!(
            relation.join(&other).unwrap(),
            Relation::new_from_iter(
                Heading::try_from(vec![
                    (AttributeName::from("a"), ScalarType::Boolean),
                    (AttributeName::from("b"), ScalarType::Integer),
                    (AttributeName::from("c"), ScalarType::Integer)
                ])
                .unwrap(),
                vec![
                    Tuple::try_from(vec![
                        (AttributeName::from("a"), Scalar::Boolean(true)),
                        (AttributeName::from("b"), Scalar::Integer(1)),
                        (AttributeName::from("c"), Scalar::Integer(100)),
                    ])
                    .unwrap(),
                    Tuple::try_from(vec![
                        (AttributeName::from("a"), Scalar::Boolean(true)),
                        (AttributeName::from("b"), Scalar::Integer(3)),
                        (AttributeName::from("c"), Scalar::Integer(300)),
                    ])
                    .unwrap(),
                ],
            )
            .unwrap()
        );
    }

    #[test]
    fn test_join_without_matches_is_empty() -> Result<(), Error> {
        let lhs = Relation::new_from_iter(
            Heading::try_from(vec![
                (AttributeName::from("id"), ScalarType::Integer),
                (AttributeName::from("left"), ScalarType::Integer),
            ])
            .unwrap(),
            vec![Tuple::try_from(vec![
                (AttributeName::from("id"), Scalar::Integer(1)),
                (AttributeName::from("left"), Scalar::Integer(10)),
            ])
            .unwrap()],
        )?;
        let rhs = Relation::new_from_iter(
            Heading::try_from(vec![
                (AttributeName::from("id"), ScalarType::Integer),
                (AttributeName::from("right"), ScalarType::Integer),
            ])
            .unwrap(),
            vec![Tuple::try_from(vec![
                (AttributeName::from("id"), Scalar::Integer(2)),
                (AttributeName::from("right"), Scalar::Integer(20)),
            ])
            .unwrap()],
        )?;

        assert_eq!(
            lhs.join(&rhs)?,
            Relation::new_from_iter(
                Heading::try_from(vec![
                    (AttributeName::from("id"), ScalarType::Integer),
                    (AttributeName::from("left"), ScalarType::Integer),
                    (AttributeName::from("right"), ScalarType::Integer),
                ])
                .unwrap(),
                Vec::new(),
            )?
        );
        Ok(())
    }

    #[test]
    fn test_join_without_common_attributes_behaves_like_product() -> Result<(), Error> {
        let lhs = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("left"), ScalarType::Integer)]).unwrap(),
            vec![Tuple::try_from(vec![(AttributeName::from("left"), Scalar::Integer(1))]).unwrap()],
        )?;
        let rhs = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("right"), ScalarType::Integer)]).unwrap(),
            vec![
                Tuple::try_from(vec![(AttributeName::from("right"), Scalar::Integer(2))]).unwrap(),
            ],
        )?;

        assert_eq!(lhs.join(&rhs)?, lhs.product(&rhs)?);
        Ok(())
    }
}
