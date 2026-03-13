//! Demonstrates how to keep only selected attributes with the `PROJECT` operation.

use darwen::prelude::{AttributeName, Relation, RelationBuilder, ScalarType};
use darwen::{heading, tuple};

fn hotline() -> Relation {
    RelationBuilder::new()
        .with_heading(
            heading!(
                name = ScalarType::String,
                phone = ScalarType::String,
                number = ScalarType::Integer
            )
            .unwrap(),
        )
        .with_body(vec![
            tuple!(name = "Monica", phone = "212-85-06", number = 4).unwrap(),
            tuple!(name = "Erica", phone = "212-85-06", number = 8).unwrap(),
            tuple!(name = "Jessica", phone = "212-85-06", number = 42).unwrap(),
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
