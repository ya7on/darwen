//! Demonstrates simple and composite predicates with the `RESTRICT` operation.

use darwen::prelude::{Predicate, Relation, RelationBuilder, Scalar, ScalarType};
use darwen::{heading, tuple, AttributeName};

fn users() -> Relation {
    RelationBuilder::new()
        .with_heading(
            heading!(
                name = ScalarType::String,
                age = ScalarType::Integer,
                city = ScalarType::String,
                home_city = ScalarType::String
            )
            .unwrap(),
        )
        .with_body(vec![
            tuple!(
                name = "Monica",
                age = 18,
                city = "Sochi",
                home_city = "Sochi"
            )
            .unwrap(),
            tuple!(
                name = "Erica",
                age = 19,
                city = "Tbilisi",
                home_city = "Paris"
            )
            .unwrap(),
            tuple!(
                name = "Rita",
                age = 20,
                city = "Berlin",
                home_city = "Berlin"
            )
            .unwrap(),
            tuple!(
                name = "Tina",
                age = 21,
                city = "Tbilisi",
                home_city = "Tbilisi"
            )
            .unwrap(),
            tuple!(
                name = "Sandra",
                age = 22,
                city = "Paris",
                home_city = "Berlin"
            )
            .unwrap(),
            tuple!(
                name = "Mary",
                age = 23,
                city = "Berlin",
                home_city = "Berlin"
            )
            .unwrap(),
        ])
        .build()
        .unwrap()
}

fn show(title: &str, relation: &Relation, predicate: Predicate) {
    println!("{title}");
    println!("{}", relation.restrict(&predicate).unwrap());
}

fn main() {
    let users = users();

    println!("Source relation");
    println!("{users}");

    show(
        "Predicate::eq(city, \"Berlin\")",
        &users,
        Predicate::eq(AttributeName::from("city"), Scalar::from("Berlin")),
    );

    show(
        "Predicate::eq(city, home_city)",
        &users,
        Predicate::eq(
            AttributeName::from("city"),
            AttributeName::from("home_city"),
        ),
    );

    show(
        "Predicate::gt(age, 20)",
        &users,
        Predicate::gt(AttributeName::from("age"), Scalar::from(20_i64)),
    );

    show(
        "Predicate::lt(age, 21)",
        &users,
        Predicate::lt(AttributeName::from("age"), Scalar::from(21_i64)),
    );

    show(
        "Predicate::and(gt(age, 20), eq(city, \"Berlin\"))",
        &users,
        Predicate::and(
            Predicate::gt(AttributeName::from("age"), Scalar::from(20_i64)),
            Predicate::eq(AttributeName::from("city"), Scalar::from("Berlin")),
        ),
    );

    show(
        "Predicate::or(eq(city, \"Berlin\"), eq(city, \"Paris\"))",
        &users,
        Predicate::or(
            Predicate::eq(AttributeName::from("city"), Scalar::from("Berlin")),
            Predicate::eq(AttributeName::from("city"), Scalar::from("Paris")),
        ),
    );

    show(
        "Predicate::not(eq(city, \"Tbilisi\"))",
        &users,
        Predicate::not(Predicate::eq(
            AttributeName::from("city"),
            Scalar::from("Tbilisi"),
        )),
    );
}
