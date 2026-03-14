# Darwen

![Codecov](https://img.shields.io/codecov/c/github/ya7on/darwen)
![GitHub License](https://img.shields.io/github/license/ya7on/darwen)
![Crates.io Version](https://img.shields.io/crates/v/darwen)

Darwen is an in-memory engine for relational algebra inspired by *The Third Manifesto* of Date, Codd, and Darwen.

This is the final version of the library. Further development is not planned, because the project is educational.

## Example

```rust
use darwen::{
    heading,
    tuple,
    prelude::{AttributeName, Predicate, RelationBuilder, Scalar, ScalarType},
};

let users = RelationBuilder::new()
    .with_heading(heading!(name = ScalarType::String, age = ScalarType::Integer)?)
    .with_body(vec![
        tuple!(name = "Monica", age = 18)?,
        tuple!(name = "Erica", age = 19)?,
        tuple!(name = "Rita", age = 20)?,
        tuple!(name = "Tina", age = 21)?,
        tuple!(name = "Sandra", age = 22)?,
        tuple!(name = "Mary", age = 23)?,
        tuple!(name = "Jessica", age = 18)?,
    ])
    .build()?;

let adults = users.restrict(&Predicate::gt(
    AttributeName::from("age"),
    Scalar::Integer(20),
))?;

let expected = RelationBuilder::new()
    .with_heading(heading!(name = ScalarType::String, age = ScalarType::Integer)?)
    .with_body(vec![
        tuple!(name = "Tina", age = 21)?,
        tuple!(name = "Sandra", age = 22)?,
        tuple!(name = "Mary", age = 23)?,
    ])
    .build()?;

assert_eq!(adults, expected);
# Ok::<(), darwen::prelude::Error>(())
```

## Predicates

Darwen supports six predicate forms:

- [`Not`](https://docs.rs/darwen/latest/darwen/enum.Predicate.html#variant.Not) / [`Predicate::not`](https://docs.rs/darwen/latest/darwen/enum.Predicate.html#method.not) negates another predicate.
- [`And`](https://docs.rs/darwen/latest/darwen/enum.Predicate.html#variant.And) / [`Predicate::and`](https://docs.rs/darwen/latest/darwen/enum.Predicate.html#method.and) performs logical conjunction; both sides are always evaluated and errors are not hidden.
- [`Or`](https://docs.rs/darwen/latest/darwen/enum.Predicate.html#variant.Or) / [`Predicate::or`](https://docs.rs/darwen/latest/darwen/enum.Predicate.html#method.or) performs logical disjunction; both sides are always evaluated and errors are not hidden.
- [`Eq`](https://docs.rs/darwen/latest/darwen/enum.Predicate.html#variant.Eq) / [`Predicate::eq`](https://docs.rs/darwen/latest/darwen/enum.Predicate.html#method.eq) compares two operands for equality; only `INTEGER = INTEGER`, `BOOLEAN = BOOLEAN`, `STRING = STRING`, and `BINARY = BINARY` are valid. Mixed-type comparisons return an error.
- [`Gt`](https://docs.rs/darwen/latest/darwen/enum.Predicate.html#variant.Gt) / [`Predicate::gt`](https://docs.rs/darwen/latest/darwen/enum.Predicate.html#method.gt) compares two operands with `>`; only `INTEGER > INTEGER` is valid. All other comparisons return an error.
- [`Lt`](https://docs.rs/darwen/latest/darwen/enum.Predicate.html#variant.Lt) / [`Predicate::lt`](https://docs.rs/darwen/latest/darwen/enum.Predicate.html#method.lt) compares two operands with `<`; only `INTEGER < INTEGER` is valid. All other comparisons return an error.

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

Code

```rust
let adults = people.restrict(&Predicate::gt(
    AttributeName::from("age"),
    Scalar::Integer(20),
))?;
```

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

Code

```rust
let names = people.project(&[AttributeName::from("name")])?;
```

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

Code

```rust
let renamed = people.rename(&[(
    AttributeName::from("name"),
    AttributeName::from("person_name"),
)])?;
```

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

Code

```rust
let result = a.union(&b)?;
```

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

Code

```rust
let result = a.difference(&b)?;
```

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

Code

```rust
let result = colors.product(&sizes)?;
```

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

Code

```rust
let result = users.join(&cities)?;
```

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

Code

```rust
let result = a.intersect(&b)?;
```

### DIVIDE (÷)
[Example](examples/divide.rs) - `cargo run --example divide`

`enrollments ÷ required_courses`

`enrollments`

| student | course |
| --- | --- |
| Ann | Math |
| Ann | Rust |
| Bob | Math |
| Bob | Rust |
| Bob | DB |
| Kate | Math |

`required_courses`

| course |
| --- |
| Math |
| Rust |

Output

| student |
| --- |
| Ann |
| Bob |

Code

```rust
let result = enrollments.divide(&required_courses)?;
```

## Sources

- [The Third Manifesto](https://www.dcs.warwick.ac.uk/~hugh/TTM/DTATRM.pdf) - the foundational manifesto of relational databases
- [TutorialD](https://www.dcs.warwick.ac.uk/~hugh/TTM/DBE-Chapter11.pdf) - a practical implementation of relational algebra
- [BNF](https://reldb.org/TutorialD.html) for TutorialD from RelDB project
