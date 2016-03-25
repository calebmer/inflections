# inflections
A Rust library for transforming one style of string to another focused on high performance.

Read the [documentation](https://calebmer.github.io/inflections) for more.

# Example
```rust
use inflections::Inflect;

assert_eq!("Hello World".to_camel_case(), "helloWorld".to_owned());
```
