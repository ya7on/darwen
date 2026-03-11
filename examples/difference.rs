//! Demonstrates how to subtract one relation from another with the `DIFFERENCE` operation.

use darwen::prelude::{
    AttributeName, HeadingBuilder, Relation, RelationBuilder, Scalar, ScalarType, TupleBuilder,
};

fn all_numbers() -> Relation {
    RelationBuilder::new()
        .with_heading(
            HeadingBuilder::new()
                .with_attribute(AttributeName::from("number"), ScalarType::Integer)
                .build()
                .unwrap(),
        )
        .with_body(vec![
            TupleBuilder::new()
                .with_value(AttributeName::from("number"), Scalar::Integer(4))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("number"), Scalar::Integer(8))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("number"), Scalar::Integer(15))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("number"), Scalar::Integer(16))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("number"), Scalar::Integer(23))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("number"), Scalar::Integer(42))
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap()
}

fn already_typed() -> Relation {
    RelationBuilder::new()
        .with_heading(
            HeadingBuilder::new()
                .with_attribute(AttributeName::from("number"), ScalarType::Integer)
                .build()
                .unwrap(),
        )
        .with_body(vec![
            TupleBuilder::new()
                .with_value(AttributeName::from("number"), Scalar::Integer(4))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("number"), Scalar::Integer(8))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("number"), Scalar::Integer(15))
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap()
}

fn main() {
    let all_numbers = all_numbers();
    let already_typed = already_typed();

    println!("{}", all_numbers.difference(&already_typed).unwrap());
}
