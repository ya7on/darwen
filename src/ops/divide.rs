use crate::{Error, Heading, HeadingBuilder, Relation, RelationBuilder};

impl Heading {
    pub(crate) fn divide(&self, other: &Heading) -> Result<Heading, Error> {
        if !other.is_subset_of(self) {
            return Err(Error::HeadingMismatch);
        }

        let mut heading = HeadingBuilder::new();
        for (name, ty) in self.iter() {
            if other.contains(name) {
                continue;
            }
            heading = heading.with_attribute(name.clone(), *ty);
        }

        heading.build()
    }
}

impl Relation {
    /// Divides one relation by another (`÷`).
    ///
    /// `students`
    ///
    /// | `name` | `course` |
    /// | --- | --- |
    /// | `Alice` | `Rust` |
    /// | `Alice` | `Math` |
    /// | `Bob` | `Rust` |
    /// | `Bob` | `Math` |
    /// | `Ann` | `Math` |
    ///
    /// `courses`
    ///
    /// | `course` |
    /// | --- |
    /// | `Rust` |
    /// | `Math` |
    ///
    /// Output
    ///
    /// | `name` |
    /// | --- |
    /// | `Alice` |
    /// | `Bob` |
    ///
    /// # Errors
    ///
    /// Returns [`Error::HeadingMismatch`] if the divisor heading is not a
    /// subset of the dividend heading.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::{
    ///     heading,
    ///     tuple,
    ///     prelude::{RelationBuilder, ScalarType},
    /// };
    ///
    /// let students = RelationBuilder::new()
    ///     .with_heading(heading!(name = ScalarType::String, course = ScalarType::String)?)
    ///     .with_body(vec![
    ///         tuple!(name = "Alice", course = "Rust")?,
    ///         tuple!(name = "Alice", course = "Math")?,
    ///         tuple!(name = "Bob", course = "Rust")?,
    ///         tuple!(name = "Bob", course = "Math")?,
    ///         tuple!(name = "Ann", course = "Math")?,
    ///     ])
    ///     .build()?;
    ///
    /// let courses = RelationBuilder::new()
    ///     .with_heading(heading!(course = ScalarType::String)?)
    ///     .with_body(vec![
    ///         tuple!(course = "Rust")?,
    ///         tuple!(course = "Math")?,
    ///     ])
    ///     .build()?;
    ///
    /// let result = students.divide(&courses)?;
    ///
    /// let expected = RelationBuilder::new()
    ///     .with_heading(heading!(name = ScalarType::String)?)
    ///     .with_body(vec![
    ///         tuple!(name = "Alice")?,
    ///         tuple!(name = "Bob")?,
    ///     ])
    ///     .build()?;
    ///
    /// assert_eq!(result, expected);
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    pub fn divide(&self, other: &Relation) -> Result<Relation, Error> {
        let new_heading = self.heading.divide(&other.heading)?;
        let attributes = new_heading
            .iter()
            .map(|(name, _ty)| name)
            .cloned()
            .collect::<Vec<_>>();
        let candidates = self.project(&attributes)?;
        let required = candidates.product(other)?;
        let missing = RelationBuilder::new()
            .with_heading(new_heading.clone())
            .with_body(
                required
                    .iter()
                    .filter(|t| !self.body.contains(t))
                    .map(|t| t.project(&attributes))
                    .collect::<Result<Vec<_>, _>>()?,
            )
            .build()?;
        RelationBuilder::new()
            .with_heading(new_heading)
            .with_body(
                candidates
                    .iter()
                    .filter(|t| !missing.body.contains(t))
                    .cloned()
                    .collect::<Vec<_>>(),
            )
            .build()
    }
}

#[cfg(test)]
mod tests {
    use crate::{heading, tuple, Error, Heading, Relation, RelationBuilder, ScalarType, Tuple};

    #[test]
    fn test_divide_returns_matching_projection() {
        let students = relation_name_course(&[
            ("Alice", "Rust"),
            ("Alice", "Math"),
            ("Bob", "Rust"),
            ("Bob", "Math"),
            ("Ann", "Math"),
            ("Kate", "DB"),
        ]);
        let courses = relation_course(&["Rust", "Math"]);

        assert_eq!(
            students.divide(&courses).unwrap(),
            relation_name(&["Alice", "Bob"])
        );
    }

    #[test]
    fn test_divide_with_empty_divisor_returns_projection_of_dividend() {
        let students = relation_name_course(&[
            ("Alice", "Rust"),
            ("Alice", "Math"),
            ("Bob", "Rust"),
            ("Bob", "Math"),
            ("Ann", "Math"),
            ("Kate", "DB"),
        ]);
        let courses = empty_relation(heading!(course = ScalarType::String).unwrap());

        assert_eq!(
            students.divide(&courses).unwrap(),
            relation_name(&["Alice", "Ann", "Bob", "Kate"])
        );
    }

    #[test]
    fn test_divide_returns_empty_relation_when_no_candidate_covers_divisor() {
        let students = relation_name_course(&[("Alice", "Rust"), ("Bob", "Math"), ("Kate", "DB")]);
        let courses = relation_course(&["Rust", "Math"]);

        assert_eq!(
            students.divide(&courses).unwrap(),
            empty_relation(heading!(name = ScalarType::String).unwrap())
        );
    }

    #[test]
    fn test_divide_with_empty_dividend_returns_empty_relation() {
        let students = empty_relation(
            heading!(name = ScalarType::String, course = ScalarType::String).unwrap(),
        );
        let courses = relation_course(&["Rust", "Math"]);

        assert_eq!(
            students.divide(&courses).unwrap(),
            empty_relation(heading!(name = ScalarType::String).unwrap())
        );
    }

    #[test]
    fn test_divide_with_same_heading_returns_relation_with_empty_tuple_when_dividend_covers_divisor(
    ) {
        let available_courses = relation_course(&["Rust", "Math", "DB"]);
        let required_courses = relation_course(&["Rust", "Math"]);

        assert_eq!(available_courses.divide(&required_courses).unwrap(), dee());
    }

    #[test]
    fn test_divide_with_same_heading_returns_empty_relation_when_dividend_does_not_cover_divisor() {
        let available_courses = relation_course(&["Rust"]);
        let required_courses = relation_course(&["Rust", "Math"]);

        assert_eq!(available_courses.divide(&required_courses).unwrap(), dum());
    }

    #[test]
    fn test_divide_rejects_divisor_with_missing_attribute() {
        let students = relation_name_course(&[("Alice", "Rust")]);
        let semesters = RelationBuilder::new()
            .with_heading(heading!(semester = ScalarType::String).unwrap())
            .with_body(vec![tuple!(semester = "Fall").unwrap()])
            .build()
            .unwrap();

        assert_eq!(students.divide(&semesters), Err(Error::HeadingMismatch));
    }

    #[test]
    fn test_divide_rejects_divisor_with_incompatible_attribute_type() {
        let students = relation_name_course(&[("Alice", "Rust")]);
        let courses = RelationBuilder::new()
            .with_heading(heading!(course = ScalarType::Integer).unwrap())
            .with_body(vec![tuple!(course = 1).unwrap()])
            .build()
            .unwrap();

        assert_eq!(students.divide(&courses), Err(Error::HeadingMismatch));
    }

    fn relation_name_course(rows: &[(&str, &str)]) -> Relation {
        RelationBuilder::new()
            .with_heading(heading!(name = ScalarType::String, course = ScalarType::String).unwrap())
            .with_body(
                rows.iter()
                    .map(|(name, course)| tuple!(name = *name, course = *course).unwrap())
                    .collect::<Vec<_>>(),
            )
            .build()
            .unwrap()
    }

    fn relation_course(rows: &[&str]) -> Relation {
        RelationBuilder::new()
            .with_heading(heading!(course = ScalarType::String).unwrap())
            .with_body(
                rows.iter()
                    .map(|course| tuple!(course = *course).unwrap())
                    .collect::<Vec<_>>(),
            )
            .build()
            .unwrap()
    }

    fn relation_name(rows: &[&str]) -> Relation {
        RelationBuilder::new()
            .with_heading(heading!(name = ScalarType::String).unwrap())
            .with_body(
                rows.iter()
                    .map(|name| tuple!(name = *name).unwrap())
                    .collect::<Vec<_>>(),
            )
            .build()
            .unwrap()
    }

    fn empty_relation(heading: Heading) -> Relation {
        RelationBuilder::new()
            .with_heading(heading)
            .with_body(Vec::<Tuple>::new())
            .build()
            .unwrap()
    }

    fn dee() -> Relation {
        RelationBuilder::new()
            .with_heading(heading!().unwrap())
            .with_body(vec![Tuple::empty()])
            .build()
            .unwrap()
    }

    fn dum() -> Relation {
        empty_relation(heading!().unwrap())
    }
}
