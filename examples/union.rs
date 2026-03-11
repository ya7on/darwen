//! Demonstrates how to combine compatible relations with the `UNION` operation.

use darwen::prelude::{
    AttributeName, HeadingBuilder, Relation, RelationBuilder, Scalar, ScalarType, TupleBuilder,
};

fn station_a() -> Relation {
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

fn station_b() -> Relation {
    RelationBuilder::new()
        .with_heading(
            HeadingBuilder::new()
                .with_attribute(AttributeName::from("number"), ScalarType::Integer)
                .build()
                .unwrap(),
        )
        .with_body(vec![
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

fn main() {
    let station_a = station_a();
    let station_b = station_b();

    println!("{}", station_a.union(&station_b).unwrap());
}
