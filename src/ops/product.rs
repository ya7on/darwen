use crate::{
    error::Error,
    prelude::{Heading, Relation, Tuple},
};

impl Heading {
    pub(crate) fn product(&self, other: &Heading) -> Result<Heading, Error> {
        let mut attributes = self.attributes.clone();
        for (name, ty) in &other.attributes {
            if attributes.contains_key(name) {
                return Err(Error::AttributeAlreadyExists { name: name.clone() });
            }
            attributes.insert(name.clone(), *ty);
        }
        Ok(Heading::new(attributes))
    }
}

impl Tuple {
    pub(crate) fn product(&self, other: &Tuple) -> Result<Tuple, Error> {
        let mut values = self.values.clone();
        for (name, value) in &other.values {
            if values.contains_key(name) {
                return Err(Error::AttributeAlreadyExists { name: name.clone() });
            }
            values.insert(name.clone(), value.clone());
        }
        Ok(Tuple::from(values))
    }
}

impl Relation {
    /// Returns the Cartesian product of two relations (`×`).
    ///
    /// `colors`
    ///
    /// | `left` |
    /// | --- |
    /// | `1` |
    /// | `2` |
    ///
    /// `sizes`
    ///
    /// | `right` |
    /// | --- |
    /// | `10` |
    /// | `20` |
    ///
    /// Output
    ///
    /// | `left` | `right` |
    /// | --- | --- |
    /// | `1` | `10` |
    /// | `1` | `20` |
    /// | `2` | `10` |
    /// | `2` | `20` |
    ///
    /// # Errors
    ///
    /// Returns [`Error::AttributeAlreadyExists`] if both relations contain the
    /// same attribute name, making the Cartesian product ambiguous.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{AttributeName, Heading, Relation, Scalar, ScalarType, Tuple};
    ///
    /// let colors = Relation::new_from_iter(
    ///     Heading::try_from(vec![(AttributeName::from("left"), ScalarType::Integer)])?,
    ///     vec![
    ///         Tuple::try_from(vec![(AttributeName::from("left"), Scalar::Integer(1))])?,
    ///         Tuple::try_from(vec![(AttributeName::from("left"), Scalar::Integer(2))])?,
    ///     ],
    /// )?;
    /// let sizes = Relation::new_from_iter(
    ///     Heading::try_from(vec![(AttributeName::from("right"), ScalarType::Integer)])?,
    ///     vec![
    ///         Tuple::try_from(vec![(AttributeName::from("right"), Scalar::Integer(10))])?,
    ///         Tuple::try_from(vec![(AttributeName::from("right"), Scalar::Integer(20))])?,
    ///     ],
    /// )?;
    ///
    /// let combinations = colors.product(&sizes)?;
    ///
    /// assert_eq!(
    ///     combinations,
    ///     Relation::new_from_iter(
    ///         Heading::try_from(vec![
    ///             (AttributeName::from("left"), ScalarType::Integer),
    ///             (AttributeName::from("right"), ScalarType::Integer),
    ///         ])?,
    ///         vec![
    ///             Tuple::try_from(vec![
    ///                 (AttributeName::from("left"), Scalar::Integer(1)),
    ///                 (AttributeName::from("right"), Scalar::Integer(10)),
    ///             ])?,
    ///             Tuple::try_from(vec![
    ///                 (AttributeName::from("left"), Scalar::Integer(1)),
    ///                 (AttributeName::from("right"), Scalar::Integer(20)),
    ///             ])?,
    ///             Tuple::try_from(vec![
    ///                 (AttributeName::from("left"), Scalar::Integer(2)),
    ///                 (AttributeName::from("right"), Scalar::Integer(10)),
    ///             ])?,
    ///             Tuple::try_from(vec![
    ///                 (AttributeName::from("left"), Scalar::Integer(2)),
    ///                 (AttributeName::from("right"), Scalar::Integer(20)),
    ///             ])?,
    ///         ],
    ///     )?
    /// );
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    pub fn product(&self, other: &Relation) -> Result<Relation, Error> {
        let heading = self.heading.product(&other.heading)?;
        let mut relation = Relation::new(heading);
        for t in &self.body {
            for u in &other.body {
                relation.body.insert(t.product(u)?);
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
    fn test_product() {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
            vec![
                Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))]).unwrap(),
                Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(2))]).unwrap(),
            ],
        )
        .unwrap();
        let other = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("bar"), ScalarType::Boolean)]).unwrap(),
            vec![
                Tuple::try_from(vec![(AttributeName::from("bar"), Scalar::Boolean(true))]).unwrap(),
                Tuple::try_from(vec![(AttributeName::from("bar"), Scalar::Boolean(false))])
                    .unwrap(),
            ],
        )
        .unwrap();

        assert_eq!(
            relation.product(&other).unwrap(),
            Relation::new_from_iter(
                Heading::try_from(vec![
                    (AttributeName::from("foo"), ScalarType::Integer),
                    (AttributeName::from("bar"), ScalarType::Boolean)
                ])
                .unwrap(),
                vec![
                    Tuple::try_from(vec![
                        (AttributeName::from("foo"), Scalar::Integer(1)),
                        (AttributeName::from("bar"), Scalar::Boolean(true))
                    ])
                    .unwrap(),
                    Tuple::try_from(vec![
                        (AttributeName::from("foo"), Scalar::Integer(1)),
                        (AttributeName::from("bar"), Scalar::Boolean(false))
                    ])
                    .unwrap(),
                    Tuple::try_from(vec![
                        (AttributeName::from("foo"), Scalar::Integer(2)),
                        (AttributeName::from("bar"), Scalar::Boolean(true))
                    ])
                    .unwrap(),
                    Tuple::try_from(vec![
                        (AttributeName::from("foo"), Scalar::Integer(2)),
                        (AttributeName::from("bar"), Scalar::Boolean(false))
                    ])
                    .unwrap(),
                ],
            )
            .unwrap()
        );
    }

    #[test]
    fn test_product_with_empty_relation_is_empty() -> Result<(), Error> {
        let lhs = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
            vec![Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))]).unwrap()],
        )?;
        let rhs = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("bar"), ScalarType::Boolean)]).unwrap(),
            Vec::new(),
        )?;

        assert_eq!(
            lhs.product(&rhs)?,
            Relation::new_from_iter(
                Heading::try_from(vec![
                    (AttributeName::from("foo"), ScalarType::Integer),
                    (AttributeName::from("bar"), ScalarType::Boolean),
                ])
                .unwrap(),
                Vec::new(),
            )?
        );
        Ok(())
    }

    #[test]
    fn test_product_rejects_overlapping_attributes() -> Result<(), Error> {
        let lhs = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
            vec![Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))]).unwrap()],
        )?;
        let rhs = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
            vec![Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(2))]).unwrap()],
        )?;

        assert_eq!(
            lhs.product(&rhs),
            Err(Error::AttributeAlreadyExists { name: "foo".into() })
        );
        Ok(())
    }

    #[test]
    fn test_tuple_product_rejects_overlapping_attributes() {
        let lhs = Tuple::try_from(vec![
            (AttributeName::from("foo"), Scalar::Integer(1)),
            (AttributeName::from("shared"), Scalar::Integer(10)),
        ])
        .unwrap();
        let rhs = Tuple::try_from(vec![
            (AttributeName::from("bar"), Scalar::Boolean(true)),
            (AttributeName::from("shared"), Scalar::Integer(20)),
        ])
        .unwrap();

        assert_eq!(
            lhs.product(&rhs),
            Err(Error::AttributeAlreadyExists {
                name: "shared".into()
            })
        );
    }
}
