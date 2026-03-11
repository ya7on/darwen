# Darwen

![Codecov](https://img.shields.io/codecov/c/github/ya7on/darwen)
![GitHub License](https://img.shields.io/github/license/ya7on/darwen)
![Crates.io Version](https://img.shields.io/crates/v/darwen)

Darwen is an in-memory engine for relational algebra inspired by *The Third Manifesto* of Date, Codd, and Darwen.

This is the final version of the library. Further development is not planned, because the project is educational.

## Example

```rust
use darwen::prelude::{
    AttributeName, Expression, HeadingBuilder, Predicate, RelationBuilder, Scalar, ScalarType,
    TupleBuilder,
};

let users = RelationBuilder::new()
    .with_heading(
        HeadingBuilder::new()
            .with_attribute(AttributeName::from("name"), ScalarType::String)
            .with_attribute(AttributeName::from("age"), ScalarType::Integer)
            .build()?,
    )
    .with_body(vec![
        TupleBuilder::new()
            .with_value(AttributeName::from("name"), Scalar::String("Monica".into()))
            .with_value(AttributeName::from("age"), Scalar::Integer(18))
            .build()?,
        TupleBuilder::new()
            .with_value(AttributeName::from("name"), Scalar::String("Erica".into()))
            .with_value(AttributeName::from("age"), Scalar::Integer(19))
            .build()?,
        TupleBuilder::new()
            .with_value(AttributeName::from("name"), Scalar::String("Rita".into()))
            .with_value(AttributeName::from("age"), Scalar::Integer(20))
            .build()?,
        TupleBuilder::new()
            .with_value(AttributeName::from("name"), Scalar::String("Tina".into()))
            .with_value(AttributeName::from("age"), Scalar::Integer(21))
            .build()?,
        TupleBuilder::new()
            .with_value(AttributeName::from("name"), Scalar::String("Sandra".into()))
            .with_value(AttributeName::from("age"), Scalar::Integer(22))
            .build()?,
        TupleBuilder::new()
            .with_value(AttributeName::from("name"), Scalar::String("Mary".into()))
            .with_value(AttributeName::from("age"), Scalar::Integer(23))
            .build()?,
        TupleBuilder::new()
            .with_value(AttributeName::from("name"), Scalar::String("Jessica".into()))
            .with_value(AttributeName::from("age"), Scalar::Integer(18))
            .build()?,
    ])
    .build()?;

let adults = users.restrict(&Predicate::Gt(
    Expression::Attribute(AttributeName::from("age")),
    Expression::Const(Scalar::Integer(20)),
))?;

let expected = RelationBuilder::new()
    .with_heading(
        HeadingBuilder::new()
            .with_attribute(AttributeName::from("name"), ScalarType::String)
            .with_attribute(AttributeName::from("age"), ScalarType::Integer)
            .build()?,
    )
    .with_body(vec![
        TupleBuilder::new()
            .with_value(AttributeName::from("name"), Scalar::String("Tina".into()))
            .with_value(AttributeName::from("age"), Scalar::Integer(21))
            .build()?,
        TupleBuilder::new()
            .with_value(AttributeName::from("name"), Scalar::String("Sandra".into()))
            .with_value(AttributeName::from("age"), Scalar::Integer(22))
            .build()?,
        TupleBuilder::new()
            .with_value(AttributeName::from("name"), Scalar::String("Mary".into()))
            .with_value(AttributeName::from("age"), Scalar::Integer(23))
            .build()?,
    ])
    .build()?;

assert_eq!(adults, expected);
# Ok::<(), darwen::prelude::Error>(())
```

## Implemented Operations

### RESTRICT/SELECTION (σ)
[Example](examples/restrict.rs) - `cargo run --example restrict`

`σ age > 20 (people)`

`people`

| name | age |
| --- | --- |
| Ann | 19 |
| Bob | 24 |

Output

| name | age |
| --- | --- |
| Bob | 24 |

### PROJECT (π)
[Example](examples/project.rs) - `cargo run --example project`

`π name (people)`

`people`

| name | age |
| --- | --- |
| Ann | 19 |
| Bob | 24 |

Output

| name |
| --- |
| Ann |
| Bob |

### RENAME (ρ)
[Example](examples/rename.rs) - `cargo run --example rename`

`ρ person_name/name (people)`

`people`

| name |
| --- |
| Ann |
| Bob |

Output

| person_name |
| --- |
| Ann |
| Bob |

### UNION (⋃)
[Example](examples/union.rs) - `cargo run --example union`

`a ⋃ b`

`a`

| value |
| --- |
| foo |
| bar |

`b`

| value |
| --- |
| bar |
| baz |

Output

| value |
| --- |
| foo |
| bar |
| baz |

### DIFFERENCE (−)
[Example](examples/difference.rs) - `cargo run --example difference`

`a − b`

`a`

| value |
| --- |
| foo |
| bar |
| baz |

`b`

| value |
| --- |
| bar |

Output

| value |
| --- |
| foo |
| baz |

### PRODUCT (×)
[Example](examples/product.rs) - `cargo run --example product`

`colors × sizes`

`colors`

| color |
| --- |
| red |
| blue |

`sizes`

| size |
| --- |
| S |
| M |

Output

| color | size |
| --- | --- |
| red | S |
| red | M |
| blue | S |
| blue | M |

### JOIN (⋈)
[Example](examples/join.rs) - `cargo run --example join`

`users ⋈ cities`

`users`

| id | name |
| --- | --- |
| 1 | Ann |
| 2 | Bob |

`cities`

| id | city |
| --- | --- |
| 1 | Rome |
| 3 | Oslo |

Output

| id | name | city |
| --- | --- | --- |
| 1 | Ann | Rome |

### INTERSECT (∩)
[Example](examples/intersect.rs) - `cargo run --example intersect`

`a ∩ b`

`a`

| value |
| --- |
| foo |
| bar |

`b`

| value |
| --- |
| bar |
| baz |

Output

| value |
| --- |
| bar |

## Sources

- [The Third Manifesto](https://www.dcs.warwick.ac.uk/~hugh/TTM/DTATRM.pdf) - the foundational manifesto of relational databases
- [TutorialD](https://www.dcs.warwick.ac.uk/~hugh/TTM/DBE-Chapter11.pdf) - a practical implementation of relational algebra
- [BNF](https://reldb.org/TutorialD.html) for TutorialD from RelDB project
