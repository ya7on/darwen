//! Darwen is an in-memory engine for relational algebra.
//!
//! The project is inspired by *The Third Manifesto* by Date, Codd, and
//! Darwen, and focuses on a compact in-memory model for experimenting with
//! relational operators.
//!
//! # Example
//!
//! ```rust
//! use darwen::{
//!     heading,
//!     tuple,
//!     prelude::{
//!         AttributeName, HeadingBuilder, Predicate, RelationBuilder, Scalar,
//!         ScalarType, TupleBuilder,
//!     },
//! };
//!
//! let users = RelationBuilder::new()
//!     .with_heading(heading!(name = ScalarType::String, age = ScalarType::Integer)?)
//!     .with_body(vec![
//!         tuple!(name = "Monica", age = 18)?,
//!         tuple!(name = "Erica", age = 19)?,
//!         tuple!(name = "Rita", age = 20)?,
//!         tuple!(name = "Tina", age = 21)?,
//!         tuple!(name = "Sandra", age = 22)?,
//!         tuple!(name = "Mary", age = 23)?,
//!         tuple!(name = "Jessica", age = 18)?,
//!     ])
//!     .build()?;
//!
//! let adults = users.restrict(&Predicate::gt(
//!     AttributeName::from("age"),
//!     Scalar::Integer(20),
//! ))?;
//!
//! let expected = RelationBuilder::new()
//!     .with_heading(heading!(name = ScalarType::String, age = ScalarType::Integer)?)
//!     .with_body(vec![
//!         tuple!(name = "Tina", age = 21)?,
//!         tuple!(name = "Sandra", age = 22)?,
//!         tuple!(name = "Mary", age = 23)?,
//!     ])
//!     .build()?;
//!
//! assert_eq!(adults, expected);
//! # Ok::<(), darwen::prelude::Error>(())
//! ```
//!
//! # Predicates
//!
//! Darwen supports six predicate forms:
//!
//! - [`Predicate::Not`] / [`Predicate::not`] negates another predicate.
//! - [`Predicate::And`] / [`Predicate::and`] performs logical conjunction;
//!   both sides are always evaluated and errors are not hidden.
//! - [`Predicate::Or`] / [`Predicate::or`] performs logical disjunction;
//!   both sides are always evaluated and errors are not hidden.
//! - [`Predicate::Eq`] / [`Predicate::eq`] compares two operands for equality;
//!   only `INTEGER = INTEGER`, `BOOLEAN = BOOLEAN`, `STRING = STRING`, and
//!   `BINARY = BINARY` are valid. Mixed-type comparisons return an error.
//! - [`Predicate::Gt`] / [`Predicate::gt`] compares two operands with `>`;
//!   only `INTEGER > INTEGER` is valid. All other comparisons return an error.
//! - [`Predicate::Lt`] / [`Predicate::lt`] compares two operands with `<`;
//!   only `INTEGER < INTEGER` is valid. All other comparisons return an error.
//!
//! # Operators
//!
//! ## Restrict
//!
//! [`Relation::restrict`] implements selection (`σ`) and returns only tuples
//! that satisfy a [`Predicate`].
//!
//! ## Project
//!
//! [`Relation::project`] implements projection (`π`) and keeps only the
//! requested attributes.
//!
//! ## Rename
//!
//! [`Relation::rename`] implements renaming (`ρ`) and changes attribute names
//! according to a mapping.
//!
//! ## Union
//!
//! [`Relation::union`] implements union (`⋃`) for relations with identical
//! headings.
//!
//! ## Difference
//!
//! [`Relation::difference`] implements difference (`−`) for relations with
//! identical headings.
//!
//! ## Product
//!
//! [`Relation::product`] implements Cartesian product (`×`) for relations with
//! disjoint headings.
//!
//! ## Join
//!
//! [`Relation::join`] implements natural join (`⋈`) on shared attributes.
//!
//! ## Intersect
//!
//! [`Relation::intersect`] implements intersection (`∩`) for relations with
//! identical headings.
//!
//! ## Divide
//!
//! [`Relation::divide`] implements relational division (`÷`).
mod error;
mod ops;
mod types;

pub use crate::types::heading::{Heading, HeadingBuilder};
pub use crate::types::predicate::{Expression, Predicate};
pub use crate::types::relation::{Relation, RelationBuilder};
pub use crate::types::scalar::{Scalar, ScalarType};
pub use crate::types::tuple::{Tuple, TupleBuilder};
pub use crate::types::AttributeName;

pub use crate::error::Error;

/// Convenient re-exports for the public relational algebra API.
pub mod prelude {
    pub use crate::types::heading::{Heading, HeadingBuilder};
    pub use crate::types::predicate::{Expression, Predicate};
    pub use crate::types::relation::{Relation, RelationBuilder};
    pub use crate::types::scalar::{Scalar, ScalarType};
    pub use crate::types::tuple::{Tuple, TupleBuilder};
    pub use crate::types::AttributeName;

    pub use crate::error::Error;
}
