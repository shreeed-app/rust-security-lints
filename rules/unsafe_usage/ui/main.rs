#![warn(security_unsafe_usage)]

unsafe fn unsafe_function() {} // Should trigger.
fn safe_function() {} // Should not trigger.

unsafe trait UnsafeTrait {} // Should trigger.
trait SafeTrait {} // Should not trigger.

struct MyType;

unsafe impl UnsafeTrait for MyType {} // Should trigger.
impl SafeTrait for MyType {} // Should not trigger.

/// The `main` function demonstrates the usage of unsafe blocks and functions.
/// It contains an unsafe block that calls an unsafe function, which should
/// trigger the `SECURITY_UNSAFE_USAGE` lint. It also contains a safe block
/// that calls a safe function, which should not trigger the lint. This
/// function serves as a test case to verify that the lint correctly identifies
/// unsafe usage while allowing safe usage without emitting warnings.
fn main() {
    panic!("This is a panic message."); // Should not trigger (safe code).
    unsafe {
        unsafe_function(); // Should trigger (unsafe block).
    }

    {
        safe_function(); // Safe block: should not trigger.
    }
}
