//! Demonstrates how to subtract one relation from another with the `DIFFERENCE` operation.

use darwen::prelude::{HeadingBuilder, Relation, RelationBuilder, ScalarType, TupleBuilder};
use darwen::{heading, tuple};

fn all_numbers() -> Relation {
    RelationBuilder::new()
        .with_heading(heading!(number = ScalarType::Integer).unwrap())
        .with_body(vec![
            tuple!(number = 4).unwrap(),
            tuple!(number = 8).unwrap(),
            tuple!(number = 15).unwrap(),
            tuple!(number = 16).unwrap(),
            tuple!(number = 23).unwrap(),
            tuple!(number = 42).unwrap(),
        ])
        .build()
        .unwrap()
}

fn already_typed() -> Relation {
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

fn main() {
    let all_numbers = all_numbers();
    let already_typed = already_typed();

    println!("{}", all_numbers.difference(&already_typed).unwrap());
}
