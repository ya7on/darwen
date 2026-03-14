#![allow(dead_code)]

use darwen::prelude::{AttributeName, Heading, Relation, Scalar, ScalarType, Tuple};

pub fn relation(attributes: Vec<(&str, ScalarType)>, body: Vec<Tuple>) -> Relation {
    Relation::new_from_iter(
        Heading::try_from(
            attributes
                .into_iter()
                .map(|(name, ty)| (AttributeName::from(name), ty))
                .collect::<Vec<_>>(),
        )
        .unwrap(),
        body,
    )
    .unwrap()
}

pub fn unary_integer_relation(name: &str, start: i64, len: i64) -> Relation {
    relation(
        vec![(name, ScalarType::Integer)],
        (start..start + len)
            .map(|value| {
                Tuple::try_from(vec![(AttributeName::from(name), Scalar::Integer(value))]).unwrap()
            })
            .collect::<Vec<_>>(),
    )
}

pub fn keyed_integer_relation(value_name: &str, start: i64, len: i64, multiplier: i64) -> Relation {
    relation(
        vec![
            ("id", ScalarType::Integer),
            (value_name, ScalarType::Integer),
        ],
        (start..start + len)
            .map(|id| {
                Tuple::try_from(vec![
                    (AttributeName::from("id"), Scalar::Integer(id)),
                    (
                        AttributeName::from(value_name),
                        Scalar::Integer(id * multiplier),
                    ),
                ])
                .unwrap()
            })
            .collect::<Vec<_>>(),
    )
}

pub fn user_relation(len: i64) -> Relation {
    relation(
        vec![
            ("id", ScalarType::Integer),
            ("age", ScalarType::Integer),
            ("score", ScalarType::Integer),
            ("active", ScalarType::Boolean),
        ],
        (0..len)
            .map(|id| {
                Tuple::try_from(vec![
                    (AttributeName::from("id"), Scalar::Integer(id)),
                    (AttributeName::from("age"), Scalar::Integer(18 + (id % 50))),
                    (AttributeName::from("score"), Scalar::Integer(id * 10)),
                    (AttributeName::from("active"), Scalar::Boolean(id % 2 == 0)),
                ])
                .unwrap()
            })
            .collect::<Vec<_>>(),
    )
}

pub fn enrollment_relation(students: i64, courses: i64) -> Relation {
    relation(
        vec![
            ("student", ScalarType::Integer),
            ("course", ScalarType::Integer),
        ],
        (0..students)
            .flat_map(|student| {
                let enrolled_courses = if student % 2 == 0 {
                    courses
                } else {
                    courses - 1
                };
                (0..enrolled_courses).map(move |course| {
                    Tuple::try_from(vec![
                        (AttributeName::from("student"), Scalar::Integer(student)),
                        (AttributeName::from("course"), Scalar::Integer(course)),
                    ])
                    .unwrap()
                })
            })
            .collect::<Vec<_>>(),
    )
}

pub fn required_courses(courses: i64) -> Relation {
    unary_integer_relation("course", 0, courses)
}
