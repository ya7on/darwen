//! Demonstrates how to build combinations with the `PRODUCT` operation.

use darwen::prelude::{
    AttributeName, HeadingBuilder, Relation, RelationBuilder, Scalar, ScalarType, TupleBuilder,
};

fn singers() -> Relation {
    RelationBuilder::new()
        .with_heading(
            HeadingBuilder::new()
                .with_attribute(AttributeName::from("name"), ScalarType::String)
                .build()
                .unwrap(),
        )
        .with_body(vec![
            TupleBuilder::new()
                .with_value(AttributeName::from("name"), Scalar::String("Monica".into()))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("name"), Scalar::String("Erica".into()))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("name"), Scalar::String("Rita".into()))
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap()
}

fn elements() -> Relation {
    RelationBuilder::new()
        .with_heading(
            HeadingBuilder::new()
                .with_attribute(AttributeName::from("element"), ScalarType::String)
                .build()
                .unwrap(),
        )
        .with_body(vec![
            TupleBuilder::new()
                .with_value(
                    AttributeName::from("element"),
                    Scalar::String("Helium".into()),
                )
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(
                    AttributeName::from("element"),
                    Scalar::String("Neon".into()),
                )
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(
                    AttributeName::from("element"),
                    Scalar::String("Argon".into()),
                )
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap()
}

fn main() {
    let singers = singers();
    let elements = elements();

    println!("{}", singers.product(&elements).unwrap());
}
