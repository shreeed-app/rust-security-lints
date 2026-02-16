# Rust Security Lints

A collection of security-focused custom Rust lints built on top of `dylint`.

This repository provides reusable compiler lints that enforce stricter coding rules than the Rust defaults, especially around explicit typing and security-oriented best practices.

## Compatibility

| OS                 | Status |
| ------------------ | ------ |
| macOS              | ✅     |
| Linux              | ✅     |
| Windows (via WSL2) | ✅     |
| Native Windows     | ✅     |

## Prerequisites

- [Rust](https://rust-lang.org/tools/install/) and Cargo
- [Dylint](https://github.com/trailofbits/dylint)
- [Docker](https://www.docker.com) and Docker Compose
- [Act](https://github.com/nektos/act) for local GitHub Actions testing

## Installation

```bash
# Install the dylint toolchain and the dylint-link plugin to enable dynamic linting.
cargo install cargo-dylint dylint-link
```

## Included lints

### `missing_type`

Provides:

- `missing_let_type`  
  Warns when a `let` binding does not explicitly declare its type.

- `missing_closure_param_type`  
  Warns when closure parameters are missing explicit type annotations.

Example:

```rust
let x = 5; // warning: missing explicit type annotation
let y: i32 = 5; // OK
let add = |a, b| a + b; // warning: missing explicit type annotations on closure parameters
let add = |a: i32, b: i32| a + b; // OK
```

### `unsafe_usage`

Provides:

- `security_unsafe_usage`  
  Warns when any usage of Rust's `unsafe` features is detected, including:
  - `unsafe fn`,
  - `unsafe trait`,
  - `unsafe impl`,
  - `unsafe {}` blocks.
  
Example:

```rust
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
