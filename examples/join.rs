//! Demonstrates how to combine relations on shared attributes with the `JOIN` operation.

use darwen::prelude::{
    AttributeName, HeadingBuilder, Relation, RelationBuilder, Scalar, ScalarType, TupleBuilder,
};

fn castaways() -> Relation {
    RelationBuilder::new()
        .with_heading(
            HeadingBuilder::new()
                .with_attribute(AttributeName::from("number"), ScalarType::Integer)
                .with_attribute(AttributeName::from("name"), ScalarType::String)
                .build()
                .unwrap(),
        )
        .with_body(vec![
            TupleBuilder::new()
                .with_value(AttributeName::from("number"), Scalar::Integer(4))
                .with_value(AttributeName::from("name"), Scalar::String("Monica".into()))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("number"), Scalar::Integer(8))
                .with_value(AttributeName::from("name"), Scalar::String("Erica".into()))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("number"), Scalar::Integer(15))
                .with_value(AttributeName::from("name"), Scalar::String("Rita".into()))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("number"), Scalar::Integer(16))
                .with_value(AttributeName::from("name"), Scalar::String("Tina".into()))
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap()
}

fn hatch() -> Relation {
    RelationBuilder::new()
        .with_heading(
            HeadingBuilder::new()
                .with_attribute(AttributeName::from("number"), ScalarType::Integer)
                .with_attribute(AttributeName::from("element"), ScalarType::String)
                .build()
                .unwrap(),
        )
        .with_body(vec![
            TupleBuilder::new()
                .with_value(AttributeName::from("number"), Scalar::Integer(8))
                .with_value(
                    AttributeName::from("element"),
                    Scalar::String("Oxygen".into()),
                )
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("number"), Scalar::Integer(15))
                .with_value(
                    AttributeName::from("element"),
                    Scalar::String("Phosphorus".into()),
                )
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("number"), Scalar::Integer(23))
                .with_value(
                    AttributeName::from("element"),
                    Scalar::String("Vanadium".into()),
                )
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap()
}

fn main() {
    let castaways = castaways();
    let hatch = hatch();

    println!("{}", castaways.join(&hatch).unwrap());
}
