use std::collections::{BTreeMap, HashSet};

use crate::{
    error::Error,
    prelude::{Heading, Relation, Tuple},
    types::AttributeName,
};

impl Heading {
    pub(crate) fn project(&self, attributes: &[AttributeName]) -> Result<Heading, Error> {
        let mut seen = HashSet::with_capacity(attributes.len());
        let mut heading_attributes = BTreeMap::new();
        for attribute in attributes {
            let Some(ty) = self.get(attribute) else {
                return Err(Error::AttributeNotFound {
                    name: attribute.clone(),
                });
            };
            if !seen.insert(attribute.clone()) {
                return Err(Error::AttributeAlreadyExists {
                    name: attribute.clone(),
                });
            }
            heading_attributes.insert(attribute.clone(), *ty);
        }
        Ok(Heading::new(heading_attributes))
    }
}

impl Tuple {
    pub(crate) fn project(&self, attributes: &[AttributeName]) -> Result<Tuple, Error> {
        let mut new_tuple = Vec::with_capacity(attributes.len());
        for attribute in attributes {
            let Some(value) = self.get(attribute) else {
                return Err(Error::AttributeNotFound {
                    name: attribute.clone(),
                });
            };
            new_tuple.push((attribute.clone(), value.clone()));
        }
        Tuple::try_from(new_tuple)
    }
}

impl Relation {
    /// Projects a relation onto a subset of attributes (`π`).
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
    /// | `id` |
    /// | --- |
    /// | `1` |
    /// | `2` |
    ///
    /// # Errors
    ///
    /// Returns [`Error::AttributeNotFound`] if one of the requested attributes
    /// does not exist in the relation heading.
    /// Returns [`Error::AttributeAlreadyExists`] if the projection list repeats
    /// the same attribute name more than once.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{
    ///     AttributeName, Heading, Relation, Scalar, ScalarType, Tuple,
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
    /// let ids = people.project(&[AttributeName::from("id")])?;
    ///
    /// assert_eq!(
    ///     ids,
    ///     Relation::new_from_iter(
    ///         Heading::try_from(vec![(AttributeName::from("id"), ScalarType::Integer)])?,
    ///         vec![
    ///             Tuple::try_from(vec![(AttributeName::from("id"), Scalar::Integer(1))])?,
    ///             Tuple::try_from(vec![(AttributeName::from("id"), Scalar::Integer(2))])?,
    ///         ],
    ///     )?
    /// );
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    pub fn project(&self, attributes: &[AttributeName]) -> Result<Relation, Error> {
        let heading = self.heading.project(attributes)?;
        let mut relation = Relation::new(heading);
        for tuple in &self.body {
            relation.insert(tuple.project(attributes)?)?;
        }
        Ok(relation)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Heading, Scalar, ScalarType, Tuple};

    use super::*;

    #[test]
    fn test_project() {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![
                (AttributeName::from("foo"), ScalarType::Integer),
                (AttributeName::from("bar"), ScalarType::Boolean),
            ])
            .unwrap(),
            vec![
                Tuple::try_from(vec![
                    (AttributeName::from("foo"), Scalar::Integer(1)),
                    (AttributeName::from("bar"), Scalar::Boolean(true)),
                ])
                .unwrap(),
                Tuple::try_from(vec![
                    (AttributeName::from("foo"), Scalar::Integer(2)),
                    (AttributeName::from("bar"), Scalar::Boolean(true)),
                ])
                .unwrap(),
                Tuple::try_from(vec![
                    (AttributeName::from("foo"), Scalar::Integer(2)),
                    (AttributeName::from("bar"), Scalar::Boolean(false)),
                ])
                .unwrap(),
                Tuple::try_from(vec![
                    (AttributeName::from("foo"), Scalar::Integer(3)),
                    (AttributeName::from("bar"), Scalar::Boolean(true)),
                ])
                .unwrap(),
                Tuple::try_from(vec![
                    (AttributeName::from("foo"), Scalar::Integer(3)),
                    (AttributeName::from("bar"), Scalar::Boolean(false)),
                ])
                .unwrap(),
            ],
        )
        .unwrap();

        assert_eq!(
            relation.project(&vec![AttributeName::from("foo")]).unwrap(),
            Relation::new_from_iter(
                Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer),])
                    .unwrap(),
                vec![
                    Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))])
                        .unwrap(),
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
    fn test_project_all_attributes_is_identity() -> Result<(), Error> {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![
                (AttributeName::from("foo"), ScalarType::Integer),
                (AttributeName::from("bar"), ScalarType::Boolean),
            ])
            .unwrap(),
            vec![Tuple::try_from(vec![
                (AttributeName::from("foo"), Scalar::Integer(1)),
                (AttributeName::from("bar"), Scalar::Boolean(true)),
            ])
            .unwrap()],
        )?;

        assert_eq!(
            relation.project(&[AttributeName::from("foo"), AttributeName::from("bar")])?,
            relation
        );
        Ok(())
    }

    #[test]
    fn test_project_rejects_unknown_attribute() -> Result<(), Error> {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
            vec![Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))]).unwrap()],
        )?;

        assert_eq!(
            relation.project(&[AttributeName::from("bar")]),
            Err(Error::AttributeNotFound { name: "bar".into() })
        );
        Ok(())
    }

    #[test]
    fn test_project_rejects_duplicate_attributes_in_projection_list() -> Result<(), Error> {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("foo"), ScalarType::Integer)]).unwrap(),
            vec![Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))]).unwrap()],
        )?;

        assert_eq!(
            relation.project(&[AttributeName::from("foo"), AttributeName::from("foo")]),
            Err(Error::AttributeAlreadyExists { name: "foo".into() })
        );
        Ok(())
    }

    #[test]
    fn test_tuple_project_rejects_missing_attribute() {
        let tuple =
            Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(1))]).unwrap();

        assert_eq!(
            tuple.project(&[AttributeName::from("bar")]),
            Err(Error::AttributeNotFound { name: "bar".into() })
        );
    }
}
