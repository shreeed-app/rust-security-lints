# unsafe_usage

## What it does

`unsafe_usage` is a Dylint security lint that detects any usage of Rust's `unsafe` features.

It emits a warning when it encounters:

- `unsafe fn`,
- `unsafe trait`,
- `unsafe impl`,
- `unsafe {}` blocks.

The goal of this lint is to make unsafe code explicitly visible during code review, especially in security-sensitive environments.

## Example

Code that triggers warnings:

```rust
#![warn(security_unsafe_usage)]

unsafe fn dangerous() {} // warning: unsafe function detected

unsafe trait UnsafeTrait {} // warning: unsafe trait detected

struct MyType;
unsafe impl UnsafeTrait for MyType {} // warning: unsafe implementation detected

fn main() {
    // warning: usage of unsafe block detected
    unsafe {
        dangerous();
    }
}
```
