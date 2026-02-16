# indexing_usage

## What it does

The `indexing_usage` lint provides warnings and denials related to the usage of indexing operations in Rust. It includes the following lints:

- `security_indexing_usage`  
  Denies when any usage of indexing operations is detected, including:
  - Indexing with `[]` syntax,
  - Slicing with `[]` syntax,
  - Usage of the `Index` and `IndexMut` traits.

## Example

Code that triggers warnings:

```rust
#![warn(security_indexing_usage)]

fn main() {
    let arr = [1, 2, 3];
    let x = arr[0]; // warning: Usage of indexing operation detected.

    let slice = &arr[1..]; // warning: Usage of slicing operation detected.

    use std::ops::Index;
    struct MyVec(Vec<i32>);

    impl Index<usize> for MyVec {
        type Output = i32;
        fn index(&self, index: usize) -> &Self::Output {
            &self.0[index] // warning: Implementation of Index/IndexMut trait detected.
        }
    }
}
```
