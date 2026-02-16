#![warn(security_indexing_usage)]

/// This module defines the `SECURITY_INDEXING_USAGE` lint, which detects the
/// use of indexing and slicing operations in Rust code. The lint is designed
/// to help developers identify potential security issues that may arise from
/// using indexing and slicing, such as out-of-bounds access. The lint checks
/// for the use of indexing operations (e.g., `arr[index]`) and slicing
/// operations (e.g., `arr[a..b]`) and emits warnings when such operations are
/// detected. The lint encourages developers to use safer alternatives, such as
/// the `.get()` method for indexing and safe accessors for slicing, to avoid
/// potential runtime panics and security vulnerabilities.
fn main() {
    let array: [i32; 3] = [1, 2, 3];
    let x: i32 = array[0]; // Should trigger.

    let slice: &[i32] = &array[1..]; // Should trigger.

    use std::ops::Index;

    struct MyVec(Vec<i32>);

    impl Index<usize> for MyVec {
        type Output = i32;

        fn index(&self, index: usize) -> &Self::Output {
            &self.0[index] // Should trigger.
        }
    }
}
