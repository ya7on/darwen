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
/// use darwen::prelude::{AttributeName, Predicate, Scalar};
///
/// let predicate = Predicate::gt(AttributeName::from("age"), Scalar::Integer(18));
///
/// assert!(matches!(predicate, Predicate::Gt(_, _)));
/// ```
#[derive(Debug)]
pub enum Predicate {
    /// Logical negation of a predicate.
    Not(Box<Predicate>),
    /// Logical conjunction of two predicates.
    ///
    /// Both operands are always evaluated; this operator does not short-circuit.
    And(Box<Predicate>, Box<Predicate>),
    /// Logical disjunction of two predicates.
    ///
    /// Both operands are always evaluated; this operator does not short-circuit.
    Or(Box<Predicate>, Box<Predicate>),
    /// Equality comparison between two expressions.
    ///
    /// `=` is only valid for operands of the same scalar type:
    /// `INTEGER = INTEGER`, `BOOLEAN = BOOLEAN`, `STRING = STRING`,
    /// and `BINARY = BINARY`. Mixed-type comparisons return an error.
    Eq(Expression, Expression),
    /// Greater-than comparison between two expressions.
    ///
    /// `>` is only valid for `INTEGER > INTEGER`. All other comparisons return
    /// an error.
    Gt(Expression, Expression),
    /// Less-than comparison between two expressions.
    ///
    /// `<` is only valid for `INTEGER < INTEGER`. All other comparisons return
    /// an error.
    Lt(Expression, Expression),
}

impl Predicate {
    /// Creates a negated predicate.
    ///
    /// The nested predicate is always evaluated when the resulting predicate is
    /// evaluated.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::{tuple, prelude::{AttributeName, Predicate, Scalar}};
    /// # use darwen::prelude::TupleBuilder;
    ///
    /// let tuple = tuple!(active = true)?;
    /// let predicate = Predicate::not(Predicate::eq(
    ///     AttributeName::from("active"),
    ///     Scalar::Boolean(false),
    /// ));
    ///
    /// assert!(predicate.eval(&tuple)?);
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    pub fn not<P>(predicate: P) -> Self
    where
        P: Into<Predicate>,
    {
        Self::Not(Box::new(predicate.into()))
    }

    /// Creates a conjunction of two predicates.
    ///
    /// Both operands are always evaluated; this operator does not short-circuit.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::{tuple, prelude::{AttributeName, Predicate, Scalar}};
    /// # use darwen::prelude::TupleBuilder;
    ///
    /// let tuple = tuple!(age = 21, active = true)?;
    /// let predicate = Predicate::and(
    ///     Predicate::gt(AttributeName::from("age"), Scalar::Integer(18)),
    ///     Predicate::eq(AttributeName::from("active"), Scalar::Boolean(true)),
    /// );
    ///
    /// assert!(predicate.eval(&tuple)?);
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    pub fn and<L, R>(lhs: L, rhs: R) -> Self
    where
        L: Into<Predicate>,
        R: Into<Predicate>,
    {
        Self::And(Box::new(lhs.into()), Box::new(rhs.into()))
    }

    /// Creates a disjunction of two predicates.
    ///
    /// Both operands are always evaluated; this operator does not short-circuit.
    ///
    /// # Example
    ///
    /// ```rust
    /// use darwen::{tuple, prelude::{AttributeName, Predicate, Scalar}};
    /// # use darwen::prelude::TupleBuilder;
    ///
    /// let tuple = tuple!(city = "Berlin")?;
    /// let predicate = Predicate::or(
    ///     Predicate::eq(AttributeName::from("city"), Scalar::from("Berlin")),
    ///     Predicate::eq(AttributeName::from("city"), Scalar::from("Paris")),
    /// );
    ///
    /// assert!(predicate.eval(&tuple)?);
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    pub fn or<L, R>(lhs: L, rhs: R) -> Self
    where
        L: Into<Predicate>,
        R: Into<Predicate>,
    {
        Self::Or(Box::new(lhs.into()), Box::new(rhs.into()))
    }

    /// Creates an equality predicate from two expressions.
    ///
    /// `=` is only valid for operands of the same scalar type:
    /// `INTEGER = INTEGER`, `BOOLEAN = BOOLEAN`, `STRING = STRING`,
    /// and `BINARY = BINARY`. Mixed-type comparisons return an error.
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
    ///
    /// Passing an owned `String` directly is intentionally rejected to avoid
    /// ambiguity between attribute names and string constants.
    ///
    /// ```compile_fail
    /// use darwen::prelude::{AttributeName, Predicate};
    ///
    /// let city_name = String::from("Berlin");
    /// let _predicate = Predicate::eq(AttributeName::from("city"), city_name);
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
    /// `>` is only valid for `INTEGER > INTEGER`. All other comparisons return
    /// an error.
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
    /// `<` is only valid for `INTEGER < INTEGER`. All other comparisons return
    /// an error.
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
    /// use darwen::prelude::{AttributeName, Predicate, Scalar, TupleBuilder};
    ///
    /// let tuple = TupleBuilder::new()
    ///     .with_value(AttributeName::from("age"), Scalar::Integer(21))
    ///     .build()?;
    ///
    /// let predicate = Predicate::gt(AttributeName::from("age"), Scalar::Integer(18));
    ///
    /// assert!(predicate.eval(&tuple)?);
    /// # Ok::<(), darwen::prelude::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidAttribute`] if one of the predicate expressions
    /// references an attribute that does not exist in the tuple, or if the
    /// comparison is invalid for the operand types.
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
                if !matches!(lhs, Scalar::Integer(_)) && !matches!(rhs, Scalar::Integer(_)) {
                    return Err(Error::InvalidAttribute);
                }
                if lhs.ty() != rhs.ty() {
                    return Err(Error::InvalidAttribute);
                }
                Ok(lhs > rhs)
            }
            Predicate::Lt(lhs, rhs) => {
                let lhs = lhs.eval(tuple)?;
                let rhs = rhs.eval(tuple)?;
                if !matches!(lhs, Scalar::Integer(_)) && !matches!(rhs, Scalar::Integer(_)) {
                    return Err(Error::InvalidAttribute);
                }
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

    fn boolean_tuple(name: &str, value: bool) -> Tuple {
        Tuple::try_from(vec![(AttributeName::from(name), Scalar::Boolean(value))]).unwrap()
    }

    fn binary_tuple(name: &str, value: Vec<u8>) -> Tuple {
        Tuple::try_from(vec![(AttributeName::from(name), Scalar::Binary(value))]).unwrap()
    }

    fn string_tuple(name: &str, value: &str) -> Tuple {
        Tuple::try_from(vec![(
            AttributeName::from(name),
            Scalar::String(value.to_string()),
        )])
        .unwrap()
    }

    #[test]
    fn test_expression_from_attribute_name() {
        let expression = Expression::from(AttributeName::from("age"));

        match expression {
            Expression::Attribute(attribute) => assert_eq!(attribute, AttributeName::from("age")),
            Expression::Const(_) => panic!("expected attribute expression"),
        }
    }

    #[test]
    fn test_expression_from_scalar() {
        let expression = Expression::from(Scalar::Integer(42));

        match expression {
            Expression::Const(value) => assert_eq!(value, Scalar::Integer(42)),
            Expression::Attribute(_) => panic!("expected const expression"),
        }
    }

    #[test]
    fn test_and() {
        let tuple = integer_tuple("foo", 42);
        let predicate = Predicate::and(
            Predicate::eq(AttributeName::from("foo"), Scalar::Integer(42)),
            Predicate::eq(AttributeName::from("foo"), Scalar::Integer(42)),
        );
        assert!(predicate.eval(&tuple).unwrap());
    }

    #[test]
    fn test_predicate_eq_helper() {
        let predicate = Predicate::eq(AttributeName::from("age"), Scalar::Integer(42));

        match predicate {
            Predicate::Eq(Expression::Attribute(attribute), Expression::Const(value)) => {
                assert_eq!(attribute, AttributeName::from("age"));
                assert_eq!(value, Scalar::Integer(42));
            }
            _ => panic!("expected eq predicate"),
        }
    }

    #[test]
    fn test_eq() {
        let tuple = integer_tuple("foo", 42);
        let predicate = Predicate::eq(AttributeName::from("foo"), Scalar::Integer(42));
        assert!(predicate.eval(&tuple).unwrap());
    }

    #[test]
    fn test_eq_supports_boolean_operands() {
        let tuple = boolean_tuple("active", true);
        let predicate = Predicate::eq(AttributeName::from("active"), Scalar::Boolean(true));

        assert!(predicate.eval(&tuple).unwrap());
    }

    #[test]
    fn test_eq_supports_string_operands() {
        let tuple = string_tuple("city", "Berlin");
        let predicate = Predicate::eq(AttributeName::from("city"), Scalar::String("Berlin".into()));

        assert!(predicate.eval(&tuple).unwrap());
    }

    #[test]
    fn test_eq_supports_binary_operands() {
        let tuple = binary_tuple("payload", vec![1, 2, 3]);
        let predicate = Predicate::eq(
            AttributeName::from("payload"),
            Scalar::Binary(vec![1, 2, 3]),
        );

        assert!(predicate.eval(&tuple).unwrap());
    }

    #[test]
    fn test_predicate_gt_helper() {
        let predicate = Predicate::gt(AttributeName::from("age"), Scalar::Integer(18));

        match predicate {
            Predicate::Gt(Expression::Attribute(attribute), Expression::Const(value)) => {
                assert_eq!(attribute, AttributeName::from("age"));
                assert_eq!(value, Scalar::Integer(18));
            }
            _ => panic!("expected gt predicate"),
        }
    }

    #[test]
    fn test_not() {
        let tuple = integer_tuple("foo", 42);
        let predicate = Predicate::not(Predicate::eq(
            AttributeName::from("foo"),
            Scalar::Integer(42),
        ));

        assert!(!predicate.eval(&tuple).unwrap());
    }

    #[test]
    fn test_predicate_lt_helper() {
        let predicate = Predicate::lt(AttributeName::from("age"), Scalar::Integer(30));

        match predicate {
            Predicate::Lt(Expression::Attribute(attribute), Expression::Const(value)) => {
                assert_eq!(attribute, AttributeName::from("age"));
                assert_eq!(value, Scalar::Integer(30));
            }
            _ => panic!("expected lt predicate"),
        }
    }

    #[test]
    fn test_or() {
        let tuple = integer_tuple("foo", 42);
        let predicate = Predicate::or(
            Predicate::eq(AttributeName::from("foo"), Scalar::Integer(0)),
            Predicate::eq(AttributeName::from("foo"), Scalar::Integer(42)),
        );

        assert!(predicate.eval(&tuple).unwrap());
    }

    #[test]
    fn test_expression_const_returns_value() -> Result<(), Error> {
        let tuple = Tuple::empty();

        assert_eq!(
            Expression::Const(Scalar::Boolean(true)).eval(&tuple)?,
            Scalar::Boolean(true)
        );
        Ok(())
    }

    #[test]
    fn test_expression_attribute_missing_is_error() {
        let tuple = Tuple::empty();

        assert_eq!(
            Expression::Attribute(AttributeName::from("missing")).eval(&tuple),
            Err(Error::InvalidAttribute)
        );
    }

    #[test]
    fn test_and_propagates_attribute_errors() {
        let tuple = integer_tuple("foo", 42);
        let predicate = Predicate::and(
            Predicate::eq(AttributeName::from("foo"), Scalar::Integer(42)),
            Predicate::eq(AttributeName::from("missing"), Scalar::Integer(42)),
        );

        assert_eq!(predicate.eval(&tuple), Err(Error::InvalidAttribute));
    }

    #[test]
    fn test_and_does_not_hide_missing_attribute_when_lhs_is_false() {
        let tuple = integer_tuple("foo", 42);
        let predicate = Predicate::and(
            Predicate::eq(AttributeName::from("foo"), Scalar::Integer(0)),
            Predicate::eq(AttributeName::from("missing"), Scalar::Integer(42)),
        );

        assert_eq!(predicate.eval(&tuple), Err(Error::InvalidAttribute));
    }

    #[test]
    fn test_or_propagates_attribute_errors() {
        let tuple = integer_tuple("foo", 42);
        let predicate = Predicate::or(
            Predicate::eq(AttributeName::from("foo"), Scalar::Integer(42)),
            Predicate::eq(AttributeName::from("missing"), Scalar::Integer(42)),
        );

        assert_eq!(predicate.eval(&tuple), Err(Error::InvalidAttribute));
    }

    #[test]
    fn test_gt() {
        let tuple = integer_tuple("age", 42);
        let predicate = Predicate::gt(AttributeName::from("age"), Scalar::Integer(20));

        assert!(predicate.eval(&tuple).unwrap());
    }

    #[test]
    fn test_gt_rejects_heterogeneous_scalar_types() {
        let tuple = integer_tuple("age", 42);
        let predicate = Predicate::gt(AttributeName::from("age"), Scalar::String("20".into()));

        assert!(
            predicate.eval(&tuple).is_err(),
            "greater-than comparison between different scalar types must return an error"
        );
    }

    #[test]
    fn test_gt_rejects_string_comparisons() {
        let tuple = string_tuple("city", "Berlin");
        let predicate = Predicate::gt(
            AttributeName::from("city"),
            Scalar::String("Amsterdam".into()),
        );

        assert!(
            predicate.eval(&tuple).is_err(),
            "greater-than comparison for string operands must return an error"
        );
    }

    #[test]
    fn test_gt_rejects_boolean_comparisons() {
        let tuple = boolean_tuple("active", true);
        let predicate = Predicate::gt(AttributeName::from("active"), Scalar::Boolean(false));

        assert!(
            predicate.eval(&tuple).is_err(),
            "greater-than comparison for boolean operands must return an error"
        );
    }

    #[test]
    fn test_gt_rejects_binary_comparisons() {
        let tuple = binary_tuple("payload", vec![1, 2, 3]);
        let predicate = Predicate::gt(
            AttributeName::from("payload"),
            Scalar::Binary(vec![0, 1, 2]),
        );

        assert!(
            predicate.eval(&tuple).is_err(),
            "greater-than comparison for binary operands must return an error"
        );
    }

    #[test]
    fn test_lt() {
        let tuple = integer_tuple("age", 18);
        let predicate = Predicate::lt(AttributeName::from("age"), Scalar::Integer(20));

        assert!(predicate.eval(&tuple).unwrap());
    }

    #[test]
    fn test_eq_rejects_heterogeneous_scalar_types() {
        let tuple = integer_tuple("age", 42);
        let predicate = Predicate::eq(AttributeName::from("age"), Scalar::String("42".into()));

        assert!(
            predicate.eval(&tuple).is_err(),
            "equality comparison between different scalar types must return an error"
        );
    }

    #[test]
    fn test_lt_rejects_heterogeneous_scalar_types() {
        let tuple = integer_tuple("age", 42);
        let predicate = Predicate::lt(AttributeName::from("age"), Scalar::String("20".into()));

        assert!(
            predicate.eval(&tuple).is_err(),
            "less-than comparison between different scalar types must return an error"
        );
    }

    #[test]
    fn test_lt_rejects_string_comparisons() {
        let tuple = string_tuple("city", "Berlin");
        let predicate = Predicate::lt(AttributeName::from("city"), Scalar::String("Paris".into()));

        assert!(
            predicate.eval(&tuple).is_err(),
            "less-than comparison for string operands must return an error"
        );
    }

    #[test]
    fn test_lt_rejects_boolean_comparisons() {
        let tuple = boolean_tuple("active", true);
        let predicate = Predicate::lt(AttributeName::from("active"), Scalar::Boolean(false));

        assert!(
            predicate.eval(&tuple).is_err(),
            "less-than comparison for boolean operands must return an error"
        );
    }

    #[test]
    fn test_lt_rejects_binary_comparisons() {
        let tuple = binary_tuple("payload", vec![1, 2, 3]);
        let predicate = Predicate::lt(
            AttributeName::from("payload"),
            Scalar::Binary(vec![4, 5, 6]),
        );

        assert!(
            predicate.eval(&tuple).is_err(),
            "less-than comparison for binary operands must return an error"
        );
    }
}
