use crate::{
    error::Error,
    prelude::{Scalar, Tuple},
    types::AttributeName,
};

/// Describes a value used during predicate evaluation.
///
/// # Example
///
/// ```rust
/// use darwen::prelude::{AttributeName, Expression, Scalar};
///
/// let expression = Expression::Attribute(AttributeName::from("age"));
/// let constant = Expression::Const(Scalar::Integer(18));
///
/// assert!(matches!(expression, Expression::Attribute(_)));
/// assert!(matches!(constant, Expression::Const(_)));
/// ```
#[derive(Debug)]
pub enum Expression {
    /// Reads a value from a tuple attribute.
    Attribute(AttributeName),
    /// Uses a constant scalar value.
    Const(Scalar),
}

impl Expression {
    /// Evaluates an expression against a tuple.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{AttributeName, Expression, Scalar, TupleBuilder};
    ///
    /// let tuple = TupleBuilder::new()
    ///     .with_value(AttributeName::from("age"), Scalar::Integer(21))
    ///     .build()?;
    ///
    /// assert_eq!(
    ///     Expression::Attribute(AttributeName::from("age")).eval(&tuple)?,
    ///     Scalar::Integer(21)
    /// );
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidAttribute`] if the expression references an
    /// attribute that does not exist in the tuple.
    pub fn eval(&self, tuple: &Tuple) -> Result<Scalar, Error> {
        match self {
            Expression::Attribute(attr) => tuple.get(attr).cloned().ok_or(Error::InvalidAttribute),
            Expression::Const(val) => Ok(val.clone()),
        }
    }
}

/// Represents a boolean condition evaluated against a tuple.
///
/// # Example
///
/// ```rust
/// use darwen::prelude::{AttributeName, Expression, Predicate, Scalar};
///
/// let predicate = Predicate::Gt(
///     Expression::Attribute(AttributeName::from("age")),
///     Expression::Const(Scalar::Integer(18)),
/// );
///
/// assert!(matches!(predicate, Predicate::Gt(_, _)));
/// ```
#[derive(Debug)]
pub enum Predicate {
    /// Logical conjunction of two predicates.
    And(Box<Predicate>, Box<Predicate>),
    /// Equality comparison between two expressions.
    Eq(Expression, Expression),
    /// Greater-than comparison between two expressions.
    Gt(Expression, Expression),
}

impl Predicate {
    /// Evaluates a predicate against a tuple.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::prelude::{AttributeName, Expression, Predicate, Scalar, TupleBuilder};
    ///
    /// let tuple = TupleBuilder::new()
    ///     .with_value(AttributeName::from("age"), Scalar::Integer(21))
    ///     .build()?;
    ///
    /// let predicate = Predicate::Gt(
    ///     Expression::Attribute(AttributeName::from("age")),
    ///     Expression::Const(Scalar::Integer(18)),
    /// );
    ///
    /// assert!(predicate.eval(&tuple)?);
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidAttribute`] if one of the predicate expressions
    /// references an attribute that does not exist in the tuple.
    pub fn eval(&self, tuple: &Tuple) -> Result<bool, Error> {
        match self {
            Predicate::And(lhs, rhs) => {
                let lhs = lhs.eval(tuple)?;
                let rhs = rhs.eval(tuple)?;
                Ok(lhs && rhs)
            }
            Predicate::Eq(lhs, rhs) => {
                let lhs = lhs.eval(tuple)?;
                let rhs = rhs.eval(tuple)?;
                if lhs.ty() != rhs.ty() {
                    return Err(Error::InvalidAttribute);
                }
                Ok(lhs == rhs)
            }
            Predicate::Gt(lhs, rhs) => {
                let lhs = lhs.eval(tuple)?;
                let rhs = rhs.eval(tuple)?;
                if lhs.ty() != rhs.ty() {
                    return Err(Error::InvalidAttribute);
                }
                Ok(lhs > rhs)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        let tuple =
            Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(42))]).unwrap();
        let predicate = Predicate::And(
            Box::new(Predicate::Eq(
                Expression::Attribute(AttributeName::from("foo")),
                Expression::Const(Scalar::Integer(42)),
            )),
            Box::new(Predicate::Eq(
                Expression::Attribute(AttributeName::from("foo")),
                Expression::Const(Scalar::Integer(42)),
            )),
        );
        assert!(predicate.eval(&tuple).unwrap());
    }

    #[test]
    fn test_eq() {
        let tuple =
            Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(42))]).unwrap();
        let predicate = Predicate::Eq(
            Expression::Attribute(AttributeName::from("foo")),
            Expression::Const(Scalar::Integer(42)),
        );
        assert!(predicate.eval(&tuple).unwrap());
    }

    #[test]
    fn test_expression_const_returns_value() -> Result<(), Error> {
        let tuple = Tuple::try_from(vec![]).unwrap();

        assert_eq!(
            Expression::Const(Scalar::Boolean(true)).eval(&tuple)?,
            Scalar::Boolean(true)
        );
        Ok(())
    }

    #[test]
    fn test_expression_attribute_missing_is_error() {
        let tuple = Tuple::try_from(vec![]).unwrap();

        assert_eq!(
            Expression::Attribute(AttributeName::from("missing")).eval(&tuple),
            Err(Error::InvalidAttribute)
        );
    }

    #[test]
    fn test_and_propagates_attribute_errors() {
        let tuple =
            Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(42))]).unwrap();
        let predicate = Predicate::And(
            Box::new(Predicate::Eq(
                Expression::Attribute(AttributeName::from("foo")),
                Expression::Const(Scalar::Integer(42)),
            )),
            Box::new(Predicate::Eq(
                Expression::Attribute(AttributeName::from("missing")),
                Expression::Const(Scalar::Integer(42)),
            )),
        );

        assert_eq!(predicate.eval(&tuple), Err(Error::InvalidAttribute));
    }

    #[test]
    fn test_and_does_not_hide_missing_attribute_when_lhs_is_false() {
        let tuple =
            Tuple::try_from(vec![(AttributeName::from("foo"), Scalar::Integer(42))]).unwrap();
        let predicate = Predicate::And(
            Box::new(Predicate::Eq(
                Expression::Attribute(AttributeName::from("foo")),
                Expression::Const(Scalar::Integer(0)),
            )),
            Box::new(Predicate::Eq(
                Expression::Attribute(AttributeName::from("missing")),
                Expression::Const(Scalar::Integer(42)),
            )),
        );

        assert_eq!(predicate.eval(&tuple), Err(Error::InvalidAttribute));
    }

    #[test]
    fn test_gt_rejects_heterogeneous_scalar_types() {
        let tuple =
            Tuple::try_from(vec![(AttributeName::from("age"), Scalar::Integer(42))]).unwrap();
        let predicate = Predicate::Gt(
            Expression::Attribute(AttributeName::from("age")),
            Expression::Const(Scalar::String("20".into())),
        );

        assert!(
            predicate.eval(&tuple).is_err(),
            "greater-than comparison between different scalar types must return an error"
        );
    }
}
