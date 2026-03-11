//! Demonstrates how to rename attributes with the `RENAME` operation.

use darwen::prelude::{
    AttributeName, HeadingBuilder, Relation, RelationBuilder, Scalar, ScalarType, TupleBuilder,
};

fn contacts() -> Relation {
    RelationBuilder::new()
        .with_heading(
            HeadingBuilder::new()
                .with_attribute(AttributeName::from("name"), ScalarType::String)
                .with_attribute(AttributeName::from("phone"), ScalarType::String)
                .build()
                .unwrap(),
        )
        .with_body(vec![
            TupleBuilder::new()
                .with_value(AttributeName::from("name"), Scalar::String("Rita".into()))
                .with_value(
                    AttributeName::from("phone"),
                    Scalar::String("212-85-06".into()),
                )
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("name"), Scalar::String("Tina".into()))
                .with_value(
                    AttributeName::from("phone"),
                    Scalar::String("212-85-06".into()),
                )
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("name"), Scalar::String("Sandra".into()))
                .with_value(
                    AttributeName::from("phone"),
                    Scalar::String("212-85-06".into()),
                )
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap()
}

fn main() {
    let contacts = contacts();

    println!(
        "{}",
        contacts
            .rename(&[(
                AttributeName::from("phone"),
                AttributeName::from("red_phone")
            )])
            .unwrap()
    );
}
