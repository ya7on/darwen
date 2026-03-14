use std::collections::{BTreeMap, HashMap, HashSet};

use crate::{
    error::Error,
    prelude::{Heading, Relation, Tuple},
    types::AttributeName,
};

impl Heading {
    pub(crate) fn rename(
        &self,
        mapping: &[(AttributeName, AttributeName)],
    ) -> Result<Heading, Error> {
        let mut rename_map = HashMap::new();
        for (old, new) in mapping {
            if !self.contains(old) {
                return Err(Error::AttributeNotFound { name: old.clone() });
            }
            if rename_map.insert(old.clone(), new.clone()).is_some() {
                return Err(Error::InvalidRenameMapping { name: old.clone() });
            }
        }
        for old in rename_map.keys() {
            if !self.contains(old) {
                return Err(Error::AttributeNotFound { name: old.clone() });
            }
        }

        let mut attributes = BTreeMap::new();
        let mut skip = HashSet::with_capacity(self.attributes.len());

        for (name, attribute) in self.iter() {
            let final_name = rename_map
                .get(name)
                .cloned()
                .unwrap_or_else(|| name.clone());

            if !skip.insert(final_name.clone()) {
                return Err(Error::AttributeAlreadyExists { name: final_name });
            }

            attributes.insert(final_name, *attribute);
        }

        Ok(Heading::new(attributes))
    }
}

impl Tuple {
    pub(crate) fn rename(
        &self,
        mapping: &[(AttributeName, AttributeName)],
    ) -> Result<Tuple, Error> {
        let mut tuple = Vec::with_capacity(mapping.len());
        let skip = mapping
            .iter()
            .flat_map(|(old, new)| vec![old, new])
            .collect::<HashSet<_>>();
        for (old, new) in mapping {
            let Some(value) = self.get(old) else {
                return Err(Error::AttributeNotFound { name: old.clone() });
            };
            tuple.push((new.clone(), value.clone()));
        }
        for (name, ty) in self.iter() {
            if !skip.contains(name) {
                tuple.push((name.clone(), ty.clone()));
            }
        }
        Tuple::try_from(tuple)
    }
}

impl Relation {
    /// Renames relation attributes according to a mapping (`ρ`).
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
    /// | `user_id` | `age` |
    /// | --- | --- |
    /// | `1` | `19` |
    /// | `2` | `24` |
    ///
    /// # Errors
    ///
    /// Returns [`Error::AttributeNotFound`] if the rename mapping references an
    /// unknown attribute.
    /// Returns [`Error::InvalidRenameMapping`] if the rename mapping repeats
    /// the same source attribute.
    /// Returns [`Error::AttributeAlreadyExists`] if the rename result would
    /// produce duplicate target names.
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
    /// let renamed_people = people.rename(&[(
    ///     AttributeName::from("id"),
    ///     AttributeName::from("user_id"),
    /// )])?;
    ///
    /// assert_eq!(
    ///     renamed_people,
    ///     Relation::new_from_iter(
    ///         Heading::try_from(vec![
    ///             (AttributeName::from("user_id"), ScalarType::Integer),
    ///             (AttributeName::from("age"), ScalarType::Integer),
    ///         ])?,
    ///         vec![
    ///             Tuple::try_from(vec![
    ///                 (AttributeName::from("user_id"), Scalar::Integer(1)),
    ///                 (AttributeName::from("age"), Scalar::Integer(19)),
    ///             ])?,
    ///             Tuple::try_from(vec![
    ///                 (AttributeName::from("user_id"), Scalar::Integer(2)),
    ///                 (AttributeName::from("age"), Scalar::Integer(24)),
    ///             ])?,
    ///         ],
    ///     )?
    /// );
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    pub fn rename(&self, mapping: &[(AttributeName, AttributeName)]) -> Result<Relation, Error> {
        let heading = self.heading.rename(mapping)?;
        let mut relation = Relation::new(heading);
        for tuple in &self.body {
            relation.body.insert(tuple.rename(mapping)?);
        }
        Ok(relation)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Heading, Scalar, ScalarType, Tuple};

    use super::*;

    #[test]
    fn test_rename() {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![
                (AttributeName::from("foo"), ScalarType::Integer),
                (AttributeName::from("bar"), ScalarType::Boolean),
                (AttributeName::from("other"), ScalarType::Integer),
            ])
            .unwrap(),
            vec![
                Tuple::try_from(vec![
                    (AttributeName::from("foo"), Scalar::Integer(1)),
                    (AttributeName::from("bar"), Scalar::Boolean(true)),
                    (AttributeName::from("other"), Scalar::Integer(1)),
                ])
                .unwrap(),
                Tuple::try_from(vec![
                    (AttributeName::from("foo"), Scalar::Integer(2)),
                    (AttributeName::from("bar"), Scalar::Boolean(true)),
                    (AttributeName::from("other"), Scalar::Integer(1)),
                ])
                .unwrap(),
                Tuple::try_from(vec![
                    (AttributeName::from("foo"), Scalar::Integer(2)),
                    (AttributeName::from("bar"), Scalar::Boolean(false)),
                    (AttributeName::from("other"), Scalar::Integer(1)),
                ])
                .unwrap(),
                Tuple::try_from(vec![
                    (AttributeName::from("foo"), Scalar::Integer(3)),
                    (AttributeName::from("bar"), Scalar::Boolean(true)),
                    (AttributeName::from("other"), Scalar::Integer(1)),
                ])
                .unwrap(),
                Tuple::try_from(vec![
                    (AttributeName::from("foo"), Scalar::Integer(3)),
                    (AttributeName::from("bar"), Scalar::Boolean(false)),
                    (AttributeName::from("other"), Scalar::Integer(1)),
                ])
                .unwrap(),
            ],
        )
        .unwrap();

        assert_eq!(
            relation
                .rename(&[
                    (AttributeName::from("foo"), AttributeName::from("foo1")),
                    (AttributeName::from("bar"), AttributeName::from("bar1"))
                ])
                .unwrap(),
            Relation::new_from_iter(
                Heading::try_from(vec![
                    (AttributeName::from("foo1"), ScalarType::Integer),
                    (AttributeName::from("bar1"), ScalarType::Boolean),
                    (AttributeName::from("other"), ScalarType::Integer)
                ])
                .unwrap(),
                vec![
                    Tuple::try_from(vec![
                        (AttributeName::from("foo1"), Scalar::Integer(1)),
                        (AttributeName::from("bar1"), Scalar::Boolean(true)),
                        (AttributeName::from("other"), Scalar::Integer(1)),
                    ])
                    .unwrap(),
                    Tuple::try_from(vec![
                        (AttributeName::from("foo1"), Scalar::Integer(2)),
                        (AttributeName::from("bar1"), Scalar::Boolean(true)),
                        (AttributeName::from("other"), Scalar::Integer(1)),
                    ])
                    .unwrap(),
                    Tuple::try_from(vec![
                        (AttributeName::from("foo1"), Scalar::Integer(2)),
                        (AttributeName::from("bar1"), Scalar::Boolean(false)),
                        (AttributeName::from("other"), Scalar::Integer(1)),
                    ])
                    .unwrap(),
                    Tuple::try_from(vec![
                        (AttributeName::from("foo1"), Scalar::Integer(3)),
                        (AttributeName::from("bar1"), Scalar::Boolean(true)),
                        (AttributeName::from("other"), Scalar::Integer(1)),
                    ])
                    .unwrap(),
                    Tuple::try_from(vec![
                        (AttributeName::from("foo1"), Scalar::Integer(3)),
                        (AttributeName::from("bar1"), Scalar::Boolean(false)),
                        (AttributeName::from("other"), Scalar::Integer(1)),
                    ])
                    .unwrap(),
                ],
            )
            .unwrap()
        );
    }

    #[test]
    fn test_rename_rejects_target_name_collisions() -> Result<(), Error> {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![
                (AttributeName::from("a"), ScalarType::Integer),
                (AttributeName::from("b"), ScalarType::Integer),
            ])
            .unwrap(),
            vec![Tuple::try_from(vec![
                (AttributeName::from("a"), Scalar::Integer(1)),
                (AttributeName::from("b"), Scalar::Integer(2)),
            ])
            .unwrap()],
        )?;

        assert_eq!(
            relation.rename(&[
                (AttributeName::from("a"), AttributeName::from("x")),
                (AttributeName::from("b"), AttributeName::from("x")),
            ]),
            Err(Error::AttributeAlreadyExists {
                name: AttributeName::from("x")
            })
        );
        Ok(())
    }

    #[test]
    fn test_rename_rejects_target_names_that_already_exist() -> Result<(), Error> {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![
                (AttributeName::from("a"), ScalarType::Integer),
                (AttributeName::from("b"), ScalarType::Integer),
            ])
            .unwrap(),
            vec![Tuple::try_from(vec![
                (AttributeName::from("a"), Scalar::Integer(1)),
                (AttributeName::from("b"), Scalar::Integer(2)),
            ])
            .unwrap()],
        )?;

        assert_eq!(
            relation.rename(&[(AttributeName::from("a"), AttributeName::from("b"))]),
            Err(Error::AttributeAlreadyExists {
                name: AttributeName::from("b")
            })
        );
        Ok(())
    }

    #[test]
    fn test_rename_with_empty_mapping_is_identity() -> Result<(), Error> {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![
                (AttributeName::from("a"), ScalarType::Integer),
                (AttributeName::from("b"), ScalarType::Integer),
            ])
            .unwrap(),
            vec![Tuple::try_from(vec![
                (AttributeName::from("a"), Scalar::Integer(1)),
                (AttributeName::from("b"), Scalar::Integer(2)),
            ])
            .unwrap()],
        )?;

        assert_eq!(
            relation.rename(&[] as &[(AttributeName, AttributeName)])?,
            relation
        );
        Ok(())
    }

    #[test]
    fn test_rename_allows_swapping_attribute_names() -> Result<(), Error> {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![
                (AttributeName::from("a"), ScalarType::Integer),
                (AttributeName::from("b"), ScalarType::Integer),
            ])
            .unwrap(),
            vec![Tuple::try_from(vec![
                (AttributeName::from("a"), Scalar::Integer(1)),
                (AttributeName::from("b"), Scalar::Integer(2)),
            ])
            .unwrap()],
        )?;

        assert_eq!(
            relation.rename(&[
                (AttributeName::from("a"), AttributeName::from("b")),
                (AttributeName::from("b"), AttributeName::from("a")),
            ])?,
            Relation::new_from_iter(
                Heading::try_from(vec![
                    (AttributeName::from("a"), ScalarType::Integer),
                    (AttributeName::from("b"), ScalarType::Integer),
                ])
                .unwrap(),
                vec![Tuple::try_from(vec![
                    (AttributeName::from("a"), Scalar::Integer(2)),
                    (AttributeName::from("b"), Scalar::Integer(1)),
                ])
                .unwrap()],
            )?
        );
        Ok(())
    }

    #[test]
    fn test_rename_rejects_duplicate_source_attributes() -> Result<(), Error> {
        let relation = Relation::new_from_iter(
            Heading::try_from(vec![(AttributeName::from("a"), ScalarType::Integer)]).unwrap(),
            vec![Tuple::try_from(vec![(AttributeName::from("a"), Scalar::Integer(1))]).unwrap()],
        )?;

        assert_eq!(
            relation.rename(&[
                (AttributeName::from("a"), AttributeName::from("x")),
                (AttributeName::from("a"), AttributeName::from("y")),
            ]),
            Err(Error::InvalidRenameMapping {
                name: AttributeName::from("a")
            })
        );
        Ok(())
    }

    #[test]
    fn test_tuple_rename_rejects_missing_source_attribute() {
        let tuple = Tuple::try_from(vec![(AttributeName::from("a"), Scalar::Integer(1))]).unwrap();

        assert_eq!(
            tuple.rename(&[(AttributeName::from("b"), AttributeName::from("c"))]),
            Err(Error::AttributeNotFound {
                name: AttributeName::from("b")
            })
        );
    }
}
