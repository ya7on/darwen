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
