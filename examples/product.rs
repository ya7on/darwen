//! Demonstrates how to build combinations with the `PRODUCT` operation.

use darwen::prelude::{HeadingBuilder, Relation, RelationBuilder, ScalarType, TupleBuilder};
use darwen::{heading, tuple};

fn singers() -> Relation {
    RelationBuilder::new()
        .with_heading(heading!(name = ScalarType::String).unwrap())
        .with_body(vec![
            tuple!(name = "Monica").unwrap(),
            tuple!(name = "Erica").unwrap(),
            tuple!(name = "Rita").unwrap(),
        ])
        .build()
        .unwrap()
}

fn elements() -> Relation {
    RelationBuilder::new()
        .with_heading(heading!(element = ScalarType::String).unwrap())
        .with_body(vec![
            tuple!(element = "Helium").unwrap(),
            tuple!(element = "Neon").unwrap(),
            tuple!(element = "Argon").unwrap(),
        ])
        .build()
        .unwrap()
}

fn main() {
    let singers = singers();
    let elements = elements();

    println!("{}", singers.product(&elements).unwrap());
}
