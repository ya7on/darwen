//! Demonstrates how to rename attributes with the `RENAME` operation.

use darwen::prelude::{
    AttributeName, HeadingBuilder, Relation, RelationBuilder, ScalarType, TupleBuilder,
};
use darwen::{heading, tuple};

fn contacts() -> Relation {
    RelationBuilder::new()
        .with_heading(heading!(name = ScalarType::String, phone = ScalarType::String).unwrap())
        .with_body(vec![
            tuple!(name = "Rita", phone = "212-85-06").unwrap(),
            tuple!(name = "Tina", phone = "212-85-06").unwrap(),
            tuple!(name = "Sandra", phone = "212-85-06").unwrap(),
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
