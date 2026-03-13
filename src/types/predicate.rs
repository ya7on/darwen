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

impl From<AttributeName> for Expression {
    fn from(attr: AttributeName) -> Self {
        Expression::Attribute(attr)
    }
}

impl From<Scalar> for Expression {
    fn from(val: Scalar) -> Self {
        Expression::Const(val)
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
    /// Logical negation of a predicate.
    Not(Box<Predicate>),
    /// Logical conjunction of two predicates.
    And(Box<Predicate>, Box<Predicate>),
    /// Logical disjunction of two predicates.
    Or(Box<Predicate>, Box<Predicate>),
    /// Equality comparison between two expressions.
    Eq(Expression, Expression),
    /// Greater-than comparison between two expressions.
    Gt(Expression, Expression),
    /// Less-than comparison between two expressions.
    Lt(Expression, Expression),
}

impl Predicate {
    /// Creates an equality predicate from two expressions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::{tuple, prelude::{AttributeName, Predicate, Scalar}};
    /// # use darwen::prelude::TupleBuilder;
    ///
    /// let tuple = tuple!(name = "Monica")?;
    /// let predicate = Predicate::eq(AttributeName::from("name"), Scalar::from("Monica"));
    ///
    /// assert!(predicate.eval(&tuple)?);
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    pub fn eq<L, R>(lhs: L, rhs: R) -> Self
    where
        L: Into<Expression>,
        R: Into<Expression>,
    {
        Predicate::Eq(lhs.into(), rhs.into())
    }

    /// Creates a greater-than predicate from two expressions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::{tuple, prelude::{AttributeName, Predicate, Scalar}};
    /// # use darwen::prelude::TupleBuilder;
    ///
    /// let tuple = tuple!(age = 21)?;
    /// let predicate = Predicate::gt(AttributeName::from("age"), Scalar::from(20_i64));
    ///
    /// assert!(predicate.eval(&tuple)?);
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    pub fn gt<L, R>(lhs: L, rhs: R) -> Self
    where
        L: Into<Expression>,
        R: Into<Expression>,
    {
        Predicate::Gt(lhs.into(), rhs.into())
    }

    /// Creates a less-than predicate from two expressions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::{tuple, prelude::{AttributeName, Predicate, Scalar}};
    /// # use darwen::prelude::TupleBuilder;
    ///
    /// let tuple = tuple!(age = 21)?;
    /// let predicate = Predicate::lt(AttributeName::from("age"), Scalar::from(30_i64));
    ///
    /// assert!(predicate.eval(&tuple)?);
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    pub fn lt<L, R>(lhs: L, rhs: R) -> Self
    where
        L: Into<Expression>,
        R: Into<Expression>,
    {
        Predicate::Lt(lhs.into(), rhs.into())
    }

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
            Predicate::Not(expr) => {
                let expr = expr.eval(tuple)?;
                Ok(!expr)
            }
            Predicate::And(lhs, rhs) => {
                let lhs = lhs.eval(tuple)?;
                let rhs = rhs.eval(tuple)?;
                Ok(lhs && rhs)
            }
            Predicate::Or(lhs, rhs) => {
                let lhs = lhs.eval(tuple)?;
                let rhs = rhs.eval(tuple)?;
                Ok(lhs || rhs)
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
            Predicate::Lt(lhs, rhs) => {
                let lhs = lhs.eval(tuple)?;
                let rhs = rhs.eval(tuple)?;
                if lhs.ty() != rhs.ty() {
                    return Err(Error::InvalidAttribute);
                }
                Ok(lhs < rhs)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn integer_tuple(name: &str, value: i64) -> Tuple {
        Tuple::try_from(vec![(AttributeName::from(name), Scalar::Integer(value))]).unwrap()
    }

    #[test]
    fn test_and() {
        let tuple = integer_tuple("foo", 42);
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
        let tuple = integer_tuple("foo", 42);
        let predicate = Predicate::Eq(
            Expression::Attribute(AttributeName::from("foo")),
            Expression::Const(Scalar::Integer(42)),
        );
        assert!(predicate.eval(&tuple).unwrap());
    }

    #[test]
    fn test_not() {
        let tuple = integer_tuple("foo", 42);
        let predicate = Predicate::Not(Box::new(Predicate::Eq(
            Expression::Attribute(AttributeName::from("foo")),
            Expression::Const(Scalar::Integer(42)),
        )));

        assert!(!predicate.eval(&tuple).unwrap());
    }

    #[test]
    fn test_or() {
        let tuple = integer_tuple("foo", 42);
        let predicate = Predicate::Or(
            Box::new(Predicate::Eq(
                Expression::Attribute(AttributeName::from("foo")),
                Expression::Const(Scalar::Integer(0)),
            )),
            Box::new(Predicate::Eq(
                Expression::Attribute(AttributeName::from("foo")),
                Expression::Const(Scalar::Integer(42)),
            )),
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
        let tuple = integer_tuple("foo", 42);
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
        let tuple = integer_tuple("foo", 42);
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
    fn test_or_propagates_attribute_errors() {
        let tuple = integer_tuple("foo", 42);
        let predicate = Predicate::Or(
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
    fn test_gt() {
        let tuple = integer_tuple("age", 42);
        let predicate = Predicate::Gt(
            Expression::Attribute(AttributeName::from("age")),
            Expression::Const(Scalar::Integer(20)),
        );

        assert!(predicate.eval(&tuple).unwrap());
    }

    #[test]
    fn test_gt_rejects_heterogeneous_scalar_types() {
        let tuple = integer_tuple("age", 42);
        let predicate = Predicate::Gt(
            Expression::Attribute(AttributeName::from("age")),
            Expression::Const(Scalar::String("20".into())),
        );

        assert!(
            predicate.eval(&tuple).is_err(),
            "greater-than comparison between different scalar types must return an error"
        );
    }

    #[test]
    fn test_lt() {
        let tuple = integer_tuple("age", 18);
        let predicate = Predicate::Lt(
            Expression::Attribute(AttributeName::from("age")),
            Expression::Const(Scalar::Integer(20)),
        );

        assert!(predicate.eval(&tuple).unwrap());
    }

    #[test]
    fn test_eq_rejects_heterogeneous_scalar_types() {
        let tuple = integer_tuple("age", 42);
        let predicate = Predicate::Eq(
            Expression::Attribute(AttributeName::from("age")),
            Expression::Const(Scalar::String("42".into())),
        );

        assert!(
            predicate.eval(&tuple).is_err(),
            "equality comparison between different scalar types must return an error"
        );
    }

    #[test]
    fn test_lt_rejects_heterogeneous_scalar_types() {
        let tuple = integer_tuple("age", 42);
        let predicate = Predicate::Lt(
            Expression::Attribute(AttributeName::from("age")),
            Expression::Const(Scalar::String("20".into())),
        );

        assert!(
            predicate.eval(&tuple).is_err(),
            "less-than comparison between different scalar types must return an error"
        );
    }
}
