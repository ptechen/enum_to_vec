# enum_to_vec

[![Version info](https://img.shields.io/crates/v/enum_to_vec.svg)](https://crates.io/crates/enum_to_vec)
[![Downloads](https://img.shields.io/crates/d/enum_to_vec.svg?style=flat-square)](https://crates.io/crates/enum_to_vec)
[![docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/enum_to_vec)
[![dependency status](https://deps.rs/crate/enum_to_vec/0.1.0/status.svg)](https://deps.rs/crate/enum_to_vec)

# Example
```rust
use enum_to_vec::ToVec;

#[derive(ToVec)]
#[snake_case]
enum Test {
    Test,
    TestA,
    TestB,
}
#[test]
fn test_vec() {
    assert_eq!(vec!["test", "test_a", "test_b"], Test::to_vec());
}

```