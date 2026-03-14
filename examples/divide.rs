//! Demonstrates how to find tuples related to all tuples of another relation with the `DIVIDE` operation.

use darwen::{
    heading,
    prelude::{Relation, RelationBuilder, ScalarType},
    tuple,
};

fn enrollments() -> Relation {
    RelationBuilder::new()
        .with_heading(heading!(student = ScalarType::String, course = ScalarType::String).unwrap())
        .with_body(vec![
            tuple!(student = "Ann", course = "Math").unwrap(),
            tuple!(student = "Ann", course = "Rust").unwrap(),
            tuple!(student = "Bob", course = "Math").unwrap(),
            tuple!(student = "Bob", course = "Rust").unwrap(),
            tuple!(student = "Bob", course = "DB").unwrap(),
            tuple!(student = "Kate", course = "Math").unwrap(),
        ])
        .build()
        .unwrap()
}

fn required_courses() -> Relation {
    RelationBuilder::new()
        .with_heading(heading!(course = ScalarType::String).unwrap())
        .with_body(vec![
            tuple!(course = "Math").unwrap(),
            tuple!(course = "Rust").unwrap(),
        ])
        .build()
        .unwrap()
}

fn main() {
    let enrollments = enrollments();
    let required_courses = required_courses();

    println!("{}", enrollments.divide(&required_courses).unwrap());
}
