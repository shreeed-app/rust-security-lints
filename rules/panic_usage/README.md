# panic_usage

## What it does

`panic_usage` is a Dylint security lint that detects any usage of Rust's `panic!`-prone features.

It emits a denial when it encounters:

- `panic!` macros,
- `unwrap()` and `expect()` methods,
- `todo!()` and `unimplemented!()` macros,
- `assert!` and related macros.

The goal of this lint is to make panic-prone code explicitly visible during code review, especially in security-sensitive environments.

## Example

Code that triggers warnings:

```rust
#![warn(security_panic_usage)]

fn main() {
    let x: Option<i32> = None;
    x.unwrap(); // warning: Call to panic backend `Unwrap` detected.
    x.expect(""); // warning: Call to panic backend `Expect` detected.

    panic!(""); // warning: Call to panic backend `BeginPanic` detected.
    assert!(false); // warning: Call to panic backend `PanickingModule` detected.
    assert_eq!(0, 1); // warning: Call to panic backend `PanickingModule` detected.
    assert_ne!(0, 0); // warning: Call to panic backend `PanickingModule` detected.
    todo!(); // warning: Call to panic backend `PanickingModule` detected.
    unimplemented!(); // warning: Call to panic backend `PanickingModule` detected.
    unreachable!(); // warning: Call to panic backend `PanickingModule` detected.
}
```
