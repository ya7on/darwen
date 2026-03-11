//! Additional tests for public API behaviors that were not covered elsewhere.

use darwen::prelude::{
    AttributeName, Error, Expression, Heading, HeadingBuilder, Predicate, Relation,
    RelationBuilder, Scalar, ScalarType, Tuple, TupleBuilder,
};

fn heading(attributes: Vec<(&str, ScalarType)>) -> Heading {
    Heading::try_from(
        attributes
            .into_iter()
            .map(|(name, ty)| (AttributeName::from(name), ty))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn empty_heading() -> Heading {
    Heading::try_from(Vec::<(AttributeName, ScalarType)>::new()).unwrap()
}

fn tuple(values: Vec<(&str, Scalar)>) -> Tuple {
    Tuple::try_from(
        values
            .into_iter()
            .map(|(name, value)| (AttributeName::from(name), value))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn empty_tuple() -> Tuple {
    Tuple::try_from(Vec::<(AttributeName, Scalar)>::new()).unwrap()
}

fn relation(attributes: Vec<(&str, ScalarType)>, rows: Vec<Vec<(&str, Scalar)>>) -> Relation {
    Relation::new_from_iter(
        heading(attributes),
        rows.into_iter().map(tuple).collect::<Vec<_>>(),
    )
    .unwrap()
}

#[test]
fn test_heading_builder_returns_error_for_duplicate_attributes() {
    assert_eq!(
        HeadingBuilder::new()
            .with_attribute(AttributeName::from("id"), ScalarType::Integer)
            .with_attribute(AttributeName::from("id"), ScalarType::String)
            .build(),
        Err(Error::InvalidAttribute)
    );
}

#[test]
fn test_tuple_builder_returns_error_for_duplicate_attributes() {
    assert_eq!(
        TupleBuilder::new()
            .with_value(AttributeName::from("id"), Scalar::Integer(1))
            .with_value(AttributeName::from("id"), Scalar::Integer(2))
            .build(),
        Err(Error::InvalidTuple)
    );
}

#[test]
fn test_relation_builder_requires_heading() {
    assert_eq!(RelationBuilder::new().build(), Err(Error::InvalidHeading));
}

#[test]
fn test_relation_builder_rejects_tuples_that_do_not_match_heading() {
    let result = RelationBuilder::new()
        .with_heading(heading(vec![("id", ScalarType::Integer)]))
        .with_body(vec![tuple(vec![("id", Scalar::Boolean(true))])])
        .build();

    assert_eq!(result, Err(Error::InvalidTuple));
}

#[test]
fn test_relation_builder_deduplicates_duplicate_tuples() {
    let result = RelationBuilder::new()
        .with_heading(heading(vec![("id", ScalarType::Integer)]))
        .with_body(vec![
            tuple(vec![("id", Scalar::Integer(1))]),
            tuple(vec![("id", Scalar::Integer(1))]),
        ])
        .build()
        .unwrap();

    assert_eq!(
        result,
        relation(
            vec![("id", ScalarType::Integer)],
            vec![vec![("id", Scalar::Integer(1))]],
        )
    );
}

#[test]
fn test_scalar_type_display_formats_all_variants() {
    assert_eq!(ScalarType::Boolean.to_string(), "BOOLEAN");
    assert_eq!(ScalarType::Integer.to_string(), "INTEGER");
    assert_eq!(ScalarType::String.to_string(), "STRING");
}

#[test]
fn test_scalar_display_and_type_cover_string_values() {
    let value = Scalar::String("Monica".into());

    assert_eq!(value.ty(), ScalarType::String);
    assert_eq!(value.to_string(), "STRING(Monica)");
}

#[test]
fn test_scalar_display_formats_all_variants() {
    assert_eq!(Scalar::Boolean(true).to_string(), "BOOLEAN(true)");
    assert_eq!(Scalar::Integer(42).to_string(), "INTEGER(42)");
    assert_eq!(Scalar::String("text".into()).to_string(), "STRING(text)");
}

#[test]
fn test_heading_validate_tuple_rejects_unknown_attribute_even_with_same_arity() {
    let heading = heading(vec![("id", ScalarType::Integer)]);
    let tuple = tuple(vec![("age", Scalar::Integer(20))]);

    assert!(!heading.validate_tuple(&tuple));
}

#[test]
fn test_heading_common_returns_empty_when_there_are_no_shared_attributes() {
    let lhs = heading(vec![
        ("id", ScalarType::Integer),
        ("name", ScalarType::String),
    ]);
    let rhs = heading(vec![("age", ScalarType::Integer)]);

    assert_eq!(lhs.common(&rhs).unwrap(), Vec::<AttributeName>::new());
}

#[test]
fn test_heading_display_sorts_attributes_by_name() {
    let heading = heading(vec![
        ("name", ScalarType::String),
        ("id", ScalarType::Integer),
    ]);

    assert_eq!(heading.to_string(), "{ id INTEGER, name STRING, }");
}

#[test]
fn test_tuple_display_sorts_values_by_attribute_name() {
    let tuple = tuple(vec![
        ("name", Scalar::String("Monica".into())),
        ("id", Scalar::Integer(1)),
    ]);

    assert_eq!(
        tuple.to_string(),
        "TUPLE { id INTEGER(1), name STRING(Monica), }"
    );
}

#[test]
fn test_relation_display_includes_heading_and_sorted_body() {
    let relation = relation(
        vec![("id", ScalarType::Integer)],
        vec![
            vec![("id", Scalar::Integer(2))],
            vec![("id", Scalar::Integer(1))],
        ],
    );

    assert_eq!(
        relation.to_string(),
        "RELATION { id INTEGER, }\n\tTUPLE { id INTEGER(1), }\n\tTUPLE { id INTEGER(2), }"
    );
}

#[test]
fn test_inserting_same_tuple_twice_keeps_set_semantics() {
    let heading = heading(vec![("id", ScalarType::Integer)]);
    let row = tuple(vec![("id", Scalar::Integer(1))]);
    let mut relation = Relation::new(heading.clone());

    relation.insert(row.clone()).unwrap();
    relation.insert(row).unwrap();

    assert_eq!(
        relation,
        Relation::new_from_iter(heading, vec![tuple(vec![("id", Scalar::Integer(1))])]).unwrap()
    );
}

#[test]
fn test_restrict_supports_composite_predicates() {
    let users = relation(
        vec![
            ("id", ScalarType::Integer),
            ("age", ScalarType::Integer),
            ("active", ScalarType::Boolean),
        ],
        vec![
            vec![
                ("id", Scalar::Integer(1)),
                ("age", Scalar::Integer(16)),
                ("active", Scalar::Boolean(true)),
            ],
            vec![
                ("id", Scalar::Integer(2)),
                ("age", Scalar::Integer(20)),
                ("active", Scalar::Boolean(true)),
            ],
            vec![
                ("id", Scalar::Integer(3)),
                ("age", Scalar::Integer(21)),
                ("active", Scalar::Boolean(false)),
            ],
        ],
    );

    let adults = users
        .restrict(&Predicate::And(
            Box::new(Predicate::Gt(
                Expression::Attribute(AttributeName::from("age")),
                Expression::Const(Scalar::Integer(18)),
            )),
            Box::new(Predicate::Eq(
                Expression::Attribute(AttributeName::from("active")),
                Expression::Const(Scalar::Boolean(true)),
            )),
        ))
        .unwrap();

    assert_eq!(
        adults,
        relation(
            vec![
                ("id", ScalarType::Integer),
                ("age", ScalarType::Integer),
                ("active", ScalarType::Boolean),
            ],
            vec![vec![
                ("id", Scalar::Integer(2)),
                ("age", Scalar::Integer(20)),
                ("active", Scalar::Boolean(true)),
            ]],
        )
    );
}

#[test]
fn test_project_to_empty_heading_returns_single_empty_tuple_for_non_empty_relation() {
    let relation = relation(
        vec![("id", ScalarType::Integer), ("name", ScalarType::String)],
        vec![
            vec![
                ("id", Scalar::Integer(1)),
                ("name", Scalar::String("Monica".into())),
            ],
            vec![
                ("id", Scalar::Integer(2)),
                ("name", Scalar::String("Sandra".into())),
            ],
        ],
    );

    assert_eq!(
        relation.project(&[]).unwrap(),
        Relation::new_from_iter(empty_heading(), vec![empty_tuple()]).unwrap()
    );
}

#[test]
fn test_rename_rejects_unknown_source_attribute() {
    let users = relation(
        vec![("id", ScalarType::Integer)],
        vec![vec![("id", Scalar::Integer(1))]],
    );

    assert_eq!(
        users.rename(&[(AttributeName::from("age"), AttributeName::from("years"))]),
        Err(Error::InvalidAttribute)
    );
}

#[test]
fn test_rename_to_same_name_is_identity() {
    let users = relation(
        vec![("id", ScalarType::Integer), ("name", ScalarType::String)],
        vec![vec![
            ("id", Scalar::Integer(1)),
            ("name", Scalar::String("Monica".into())),
        ]],
    );

    assert_eq!(
        users
            .rename(&[(AttributeName::from("id"), AttributeName::from("id"))])
            .unwrap(),
        users
    );
}

#[test]
fn test_join_rejects_shared_attributes_with_different_types() {
    let lhs = relation(
        vec![("id", ScalarType::Integer), ("left", ScalarType::String)],
        vec![vec![
            ("id", Scalar::Integer(1)),
            ("left", Scalar::String("lhs".into())),
        ]],
    );
    let rhs = relation(
        vec![("id", ScalarType::String), ("right", ScalarType::String)],
        vec![vec![
            ("id", Scalar::String("1".into())),
            ("right", Scalar::String("rhs".into())),
        ]],
    );

    assert_eq!(lhs.join(&rhs), Err(Error::InvalidAttribute));
}

#[test]
fn test_join_uses_all_shared_attributes_when_matching_rows() {
    let lhs = relation(
        vec![
            ("id", ScalarType::Integer),
            ("kind", ScalarType::String),
            ("left", ScalarType::Integer),
        ],
        vec![
            vec![
                ("id", Scalar::Integer(1)),
                ("kind", Scalar::String("a".into())),
                ("left", Scalar::Integer(10)),
            ],
            vec![
                ("id", Scalar::Integer(1)),
                ("kind", Scalar::String("b".into())),
                ("left", Scalar::Integer(20)),
            ],
        ],
    );
    let rhs = relation(
        vec![
            ("id", ScalarType::Integer),
            ("kind", ScalarType::String),
            ("right", ScalarType::Integer),
        ],
        vec![
            vec![
                ("id", Scalar::Integer(1)),
                ("kind", Scalar::String("a".into())),
                ("right", Scalar::Integer(100)),
            ],
            vec![
                ("id", Scalar::Integer(1)),
                ("kind", Scalar::String("c".into())),
                ("right", Scalar::Integer(300)),
            ],
        ],
    );

    assert_eq!(
        lhs.join(&rhs).unwrap(),
        relation(
            vec![
                ("id", ScalarType::Integer),
                ("kind", ScalarType::String),
                ("left", ScalarType::Integer),
                ("right", ScalarType::Integer),
            ],
            vec![vec![
                ("id", Scalar::Integer(1)),
                ("kind", Scalar::String("a".into())),
                ("left", Scalar::Integer(10)),
                ("right", Scalar::Integer(100)),
            ]],
        )
    );
}

#[test]
fn test_difference_with_empty_relation_is_identity() {
    let lhs = relation(
        vec![("id", ScalarType::Integer)],
        vec![vec![("id", Scalar::Integer(1))]],
    );
    let rhs = Relation::new(heading(vec![("id", ScalarType::Integer)]));

    assert_eq!(lhs.difference(&rhs).unwrap(), lhs);
}

#[test]
fn test_intersect_with_empty_relation_is_empty() {
    let lhs = relation(
        vec![("id", ScalarType::Integer)],
        vec![vec![("id", Scalar::Integer(1))]],
    );
    let rhs = Relation::new(heading(vec![("id", ScalarType::Integer)]));

    assert_eq!(
        lhs.intersect(&rhs).unwrap(),
        Relation::new_from_iter(heading(vec![("id", ScalarType::Integer)]), Vec::new()).unwrap()
    );
}
