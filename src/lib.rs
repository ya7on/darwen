//! Darwen is an in-memory engine for relational algebra.
//!
//! The project is inspired by *The Third Manifesto* by Date, Codd, and
//! Darwen, and focuses on a compact in-memory model for experimenting with
//! relational operators.
//!
//! # Example
//!
//! ```rust
//! use darwen::prelude::{
//!     AttributeName, Expression, HeadingBuilder, Predicate, RelationBuilder, Scalar,
//!     ScalarType, TupleBuilder,
//! };
//!
//! let users = RelationBuilder::new()
//!     .with_heading(
//!         HeadingBuilder::new()
//!             .with_attribute(AttributeName::from("name"), ScalarType::String)
//!             .with_attribute(AttributeName::from("age"), ScalarType::Integer)
//!             .build()?,
//!     )
//!     .with_body(vec![
//!         TupleBuilder::new()
//!             .with_value(AttributeName::from("name"), Scalar::String("Monica".into()))
//!             .with_value(AttributeName::from("age"), Scalar::Integer(18))
//!             .build()?,
//!         TupleBuilder::new()
//!             .with_value(AttributeName::from("name"), Scalar::String("Erica".into()))
//!             .with_value(AttributeName::from("age"), Scalar::Integer(19))
//!             .build()?,
//!         TupleBuilder::new()
//!             .with_value(AttributeName::from("name"), Scalar::String("Rita".into()))
//!             .with_value(AttributeName::from("age"), Scalar::Integer(20))
//!             .build()?,
//!         TupleBuilder::new()
//!             .with_value(AttributeName::from("name"), Scalar::String("Tina".into()))
//!             .with_value(AttributeName::from("age"), Scalar::Integer(21))
//!             .build()?,
//!         TupleBuilder::new()
//!             .with_value(AttributeName::from("name"), Scalar::String("Sandra".into()))
//!             .with_value(AttributeName::from("age"), Scalar::Integer(22))
//!             .build()?,
//!         TupleBuilder::new()
//!             .with_value(AttributeName::from("name"), Scalar::String("Mary".into()))
//!             .with_value(AttributeName::from("age"), Scalar::Integer(23))
//!             .build()?,
//!         TupleBuilder::new()
//!             .with_value(AttributeName::from("name"), Scalar::String("Jessica".into()))
//!             .with_value(AttributeName::from("age"), Scalar::Integer(18))
//!             .build()?,
//!     ])
//!     .build()?;
//!
//! let adults = users.restrict(&Predicate::Gt(
//!     Expression::Attribute(AttributeName::from("age")),
//!     Expression::Const(Scalar::Integer(20)),
//! ))?;
//!
//! let expected = RelationBuilder::new()
//!     .with_heading(
//!         HeadingBuilder::new()
//!             .with_attribute(AttributeName::from("name"), ScalarType::String)
//!             .with_attribute(AttributeName::from("age"), ScalarType::Integer)
//!             .build()?,
//!     )
//!     .with_body(vec![
//!         TupleBuilder::new()
//!             .with_value(AttributeName::from("name"), Scalar::String("Tina".into()))
//!             .with_value(AttributeName::from("age"), Scalar::Integer(21))
//!             .build()?,
//!         TupleBuilder::new()
//!             .with_value(AttributeName::from("name"), Scalar::String("Sandra".into()))
//!             .with_value(AttributeName::from("age"), Scalar::Integer(22))
//!             .build()?,
//!         TupleBuilder::new()
//!             .with_value(AttributeName::from("name"), Scalar::String("Mary".into()))
//!             .with_value(AttributeName::from("age"), Scalar::Integer(23))
//!             .build()?,
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
