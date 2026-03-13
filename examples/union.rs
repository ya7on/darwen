//! Demonstrates how to combine compatible relations with the `UNION` operation.

use darwen::prelude::{Relation, RelationBuilder, ScalarType};
use darwen::{heading, tuple};

fn station_a() -> Relation {
    RelationBuilder::new()
        .with_heading(heading!(number = ScalarType::Integer).unwrap())
        .with_body(vec![
            tuple!(number = 4).unwrap(),
            tuple!(number = 8).unwrap(),
            tuple!(number = 15).unwrap(),
        ])
        .build()
        .unwrap()
}

fn station_b() -> Relation {
    RelationBuilder::new()
        .with_heading(heading!(number = ScalarType::Integer).unwrap())
        .with_body(vec![
            tuple!(number = 15).unwrap(),
            tuple!(number = 16).unwrap(),
            tuple!(number = 23).unwrap(),
            tuple!(number = 42).unwrap(),
        ])
        .build()
        .unwrap()
}

fn main() {
    let station_a = station_a();
    let station_b = station_b();

    println!("{}", station_a.union(&station_b).unwrap());
}
