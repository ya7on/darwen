//! Demonstrates simple and composite predicates with the `RESTRICT` operation.

use darwen::prelude::{Expression, Predicate, Relation, RelationBuilder, Scalar, ScalarType};
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
        Predicate::Eq(
            Expression::Attribute(AttributeName::from("city")),
            Expression::Const(Scalar::from("Berlin")),
        ),
    );

    show(
        "Predicate::eq(city, home_city)",
        &users,
        Predicate::Eq(
            Expression::Attribute(AttributeName::from("city")),
            Expression::Attribute(AttributeName::from("home_city")),
        ),
    );

    show(
        "Predicate::gt(age, 20)",
        &users,
        Predicate::Gt(
            Expression::Attribute(AttributeName::from("age")),
            Expression::Const(Scalar::from(20_i64)),
        ),
    );

    show(
        "Predicate::lt(age, 21)",
        &users,
        Predicate::Lt(
            Expression::Attribute(AttributeName::from("age")),
            Expression::Const(Scalar::from(21_i64)),
        ),
    );

    show(
        "Predicate::And(gt(age, 20), eq(city, \"Berlin\"))",
        &users,
        Predicate::And(
            Box::new(Predicate::Gt(
                Expression::Attribute(AttributeName::from("age")),
                Expression::Const(Scalar::from(20_i64)),
            )),
            Box::new(Predicate::Eq(
                Expression::Attribute(AttributeName::from("city")),
                Expression::Const(Scalar::from("Berlin")),
            )),
        ),
    );

    show(
        "Predicate::Or(eq(city, \"Berlin\"), eq(city, \"Paris\"))",
        &users,
        Predicate::Or(
            Box::new(Predicate::Eq(
                Expression::Attribute(AttributeName::from("city")),
                Expression::Const(Scalar::from("Berlin")),
            )),
            Box::new(Predicate::Eq(
                Expression::Attribute(AttributeName::from("city")),
                Expression::Const(Scalar::from("Paris")),
            )),
        ),
    );

    show(
        "Predicate::Not(eq(city, \"Tbilisi\"))",
        &users,
        Predicate::Not(Box::new(Predicate::Eq(
            Expression::Attribute(AttributeName::from("city")),
            Expression::Const(Scalar::from("Tbilisi")),
        ))),
    );
}
