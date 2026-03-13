//! Demonstrates how to keep only shared tuples with the `INTERSECT` operation.

use darwen::prelude::{Relation, RelationBuilder, ScalarType};
use darwen::{heading, tuple};

fn side_a() -> Relation {
    RelationBuilder::new()
        .with_heading(heading!(name = ScalarType::String).unwrap())
        .with_body(vec![
            tuple!(name = "Monica").unwrap(),
            tuple!(name = "Erica").unwrap(),
            tuple!(name = "Rita").unwrap(),
            tuple!(name = "Tina").unwrap(),
        ])
        .build()
        .unwrap()
}

fn side_b() -> Relation {
    RelationBuilder::new()
        .with_heading(heading!(name = ScalarType::String).unwrap())
        .with_body(vec![
            tuple!(name = "Rita").unwrap(),
            tuple!(name = "Sandra").unwrap(),
            tuple!(name = "Mary").unwrap(),
            tuple!(name = "Jessica").unwrap(),
        ])
        .build()
        .unwrap()
}

fn main() {
    let side_a = side_a();
    let side_b = side_b();

    println!("{}", side_a.intersect(&side_b).unwrap());
}
