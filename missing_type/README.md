# missing_type

## What it does

This lint library provides two lints:

- `missing_let_type`  
  Warns when a `let` binding does not include an explicit type annotation.

- `missing_closure_param_type`  
  Warns when closure parameters do not explicitly declare their types.

The goal is to reduce reliance on type inference and enforce explicit typing in local bindings and closures.

## Example

```rust
#![warn(missing_let_type, missing_closure_param_type)]

fn main() {
    let x = 5; // warning: missing explicit type annotation

    let add = |a, b| a + b; 
    // warnings:
    // - missing explicit type annotation on let binding
    // - closure parameter missing explicit type annotation
}
```
