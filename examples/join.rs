//! Demonstrates how to combine relations on shared attributes with the `JOIN` operation.

use darwen::{
    heading,
    prelude::{Relation, RelationBuilder, ScalarType},
    tuple,
};

fn castaways() -> Relation {
    RelationBuilder::new()
        .with_heading(heading!(number = ScalarType::Integer, name = ScalarType::String).unwrap())
        .with_body(vec![
            tuple!(number = 4, name = "Monica").unwrap(),
            tuple!(number = 8, name = "Erica").unwrap(),
            tuple!(number = 15, name = "Rita").unwrap(),
            tuple!(number = 16, name = "Tina").unwrap(),
        ])
        .build()
        .unwrap()
}

fn hatch() -> Relation {
    RelationBuilder::new()
        .with_heading(heading!(number = ScalarType::Integer, element = ScalarType::String).unwrap())
        .with_body(vec![
            tuple!(number = 8, element = "Oxygen").unwrap(),
            tuple!(number = 15, element = "Phosphorus").unwrap(),
            tuple!(number = 23, element = "Vanadium").unwrap(),
        ])
        .build()
        .unwrap()
}

fn main() {
    let castaways = castaways();
    let hatch = hatch();

    println!("{}", castaways.join(&hatch).unwrap());
}
