#![warn(security_panic_usage)]

/// This module defines the `SECURITY_PANIC_USAGE` lint, which detects the use
/// of constructs that may cause panics at runtime, such as `unwrap()`,
/// `expect()`, `panic!()`, `assert!()`, and related macros. The lint is
/// designed to help developers identify and review panic-prone code,
/// especially in security-sensitive contexts.
fn main() {
    let x: Option<i32> = None;
    x.unwrap(); // Should trigger.
    x.expect(""); // should trigger.

    panic!(""); // Should trigger.

    assert!(false); // Should trigger.
    assert_eq!(0, 1); // Should trigger.
    assert_ne!(0, 0); // Should trigger.

    todo!(); // Should trigger.
    unimplemented!(); // Should trigger.
    unreachable!(); // Should trigger.
}
