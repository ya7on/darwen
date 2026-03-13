//! Demonstrates how to filter a relation with the `RESTRICT` operation.

use darwen::prelude::{
    Expression, HeadingBuilder, Predicate, Relation, RelationBuilder, Scalar, ScalarType,
    TupleBuilder,
};
use darwen::{heading, tuple};

fn users() -> Relation {
    RelationBuilder::new()
        .with_heading(heading!(name = ScalarType::String, age = ScalarType::Integer).unwrap())
        .with_body(vec![
            tuple!(name = "Monica", age = 18).unwrap(),
            tuple!(name = "Erica", age = 19).unwrap(),
            tuple!(name = "Rita", age = 20).unwrap(),
            tuple!(name = "Tina", age = 21).unwrap(),
            tuple!(name = "Sandra", age = 22).unwrap(),
            tuple!(name = "Mary", age = 23).unwrap(),
            tuple!(name = "Jessica", age = 18).unwrap(),
        ])
        .build()
        .unwrap()
}

fn main() {
    let users = users();

    println!(
        "{}",
        users
            .restrict(&Predicate::Gt(
                Expression::Attribute("age".to_string()),
                Expression::Const(Scalar::Integer(20)),
            ))
            .unwrap()
    )
}
