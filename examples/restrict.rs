//! Demonstrates how to filter a relation with the `RESTRICT` operation.

use darwen::prelude::{
    AttributeName, Expression, HeadingBuilder, Predicate, Relation, RelationBuilder, Scalar,
    ScalarType, TupleBuilder,
};

fn users() -> Relation {
    RelationBuilder::new()
        .with_heading(
            HeadingBuilder::new()
                .with_attribute(AttributeName::from("name"), ScalarType::String)
                .with_attribute(AttributeName::from("age"), ScalarType::Integer)
                .build()
                .unwrap(),
        )
        .with_body(vec![
            TupleBuilder::new()
                .with_value(AttributeName::from("name"), Scalar::String("Monica".into()))
                .with_value(AttributeName::from("age"), Scalar::Integer(18))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("name"), Scalar::String("Erica".into()))
                .with_value(AttributeName::from("age"), Scalar::Integer(19))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("name"), Scalar::String("Rita".into()))
                .with_value(AttributeName::from("age"), Scalar::Integer(20))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("name"), Scalar::String("Tina".into()))
                .with_value(AttributeName::from("age"), Scalar::Integer(21))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("name"), Scalar::String("Sandra".into()))
                .with_value(AttributeName::from("age"), Scalar::Integer(22))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("name"), Scalar::String("Mary".into()))
                .with_value(AttributeName::from("age"), Scalar::Integer(23))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(
                    AttributeName::from("name"),
                    Scalar::String("Jessica".into()),
                )
                .with_value(AttributeName::from("age"), Scalar::Integer(18))
                .build()
                .unwrap(),
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
