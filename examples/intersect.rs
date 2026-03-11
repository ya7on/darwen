//! Demonstrates how to keep only shared tuples with the `INTERSECT` operation.

use darwen::prelude::{
    AttributeName, HeadingBuilder, Relation, RelationBuilder, Scalar, ScalarType, TupleBuilder,
};

fn side_a() -> Relation {
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
            TupleBuilder::new()
                .with_value(AttributeName::from("name"), Scalar::String("Tina".into()))
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap()
}

fn side_b() -> Relation {
    RelationBuilder::new()
        .with_heading(
            HeadingBuilder::new()
                .with_attribute(AttributeName::from("name"), ScalarType::String)
                .build()
                .unwrap(),
        )
        .with_body(vec![
            TupleBuilder::new()
                .with_value(AttributeName::from("name"), Scalar::String("Rita".into()))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("name"), Scalar::String("Sandra".into()))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(AttributeName::from("name"), Scalar::String("Mary".into()))
                .build()
                .unwrap(),
            TupleBuilder::new()
                .with_value(
                    AttributeName::from("name"),
                    Scalar::String("Jessica".into()),
                )
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap()
}

fn main() {
    let side_a = side_a();
    let side_b = side_b();

    println!("{}", side_a.intersect(&side_b).unwrap());
}
