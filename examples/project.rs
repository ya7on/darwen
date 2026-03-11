//! Demonstrates how to keep only selected attributes with the `PROJECT` operation.

use darwen::prelude::{
    AttributeName, HeadingBuilder, Relation, RelationBuilder, Scalar, ScalarType, TupleBuilder,
};

fn hotline() -> Relation {
    RelationBuilder::new()
        .with_heading(
            HeadingBuilder::new()
                .with_attribute(AttributeName::from("name"), ScalarType::String)
                .with_attribute(AttributeName::from("phone"), ScalarType::String)
                .with_attribute(AttributeName::from("number"), ScalarType::Integer)
                .build()
                .unwrap(),
        )
        .with_body(vec![
            TupleBuilder::new()
                .with_value(AttributeName::from("name"), Scalar::String("Monica".into()))
                .with_value(
                    AttributeName::from("phone"),
                    Scalar::String("212-85-06".into()),
                )
                .with_value(AttributeName::from("number"), Scalar::Integer(4))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("name"), Scalar::String("Erica".into()))
                .with_value(
                    AttributeName::from("phone"),
                    Scalar::String("212-85-06".into()),
                )
                .with_value(AttributeName::from("number"), Scalar::Integer(8))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(
                    AttributeName::from("name"),
                    Scalar::String("Jessica".into()),
                )
                .with_value(
                    AttributeName::from("phone"),
                    Scalar::String("212-85-06".into()),
                )
                .with_value(AttributeName::from("number"), Scalar::Integer(42))
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap()
}

fn main() {
    let hotline = hotline();

    println!(
        "{}",
        hotline
            .project(&[AttributeName::from("name"), AttributeName::from("number")])
            .unwrap()
    );
}
