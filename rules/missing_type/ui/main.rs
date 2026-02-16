/// UI test for the `missing_type` lint. This test compiles the code in the
/// `ui` directory and checks that the expected warnings are emitted for
/// missing explicit type annotations on let bindings and closure parameters,
/// while ensuring that no warnings are emitted for cases where the pattern is
/// `_`. The test uses the `dylint_testing` crate to run the UI tests with the
/// appropriate compiler flags for UI testing. The test will pass if the
/// expected warnings are emitted and fail if any unexpected warnings are
/// emitted or if the expected warnings are not emitted.
fn main() {
    // Let without type annotation (should trigger).
    let x = 5;

    // Let with type annotation (should not trigger).
    let y: i32 = 10;

    // Let with `_` pattern (should not trigger).
    let _ = 42;

    // Closure without explicit parameter types (should trigger).
    let add: fn(i32, i32) -> i32 = |a, b| a + b;

    // Closure with explicit types (should trigger).
    let sub = |a: i32, b: i32| a - b;

    // Closure with one parameter missing type annotation (should trigger).
    let mul = |a: i32, b| a * b;
    let _: i32 = mul(2, 3);

    // Closure with `_` pattern (should not trigger).
    let ignore: fn(i32) -> i32 = |_| 0;
}

/// Asynchronous function example to demonstrate that the `missing_type` lint
/// does not trigger for async functions, as they may have implicit return
/// types and parameters that are not explicitly annotated. This function takes
/// an `i32` parameter and returns a future that resolves to an `i32`. The lint
/// should not emit a warning for this function, as it is common for async
/// functions to have implicit return types and parameters without explicit
/// type annotations, especially when using async/await syntax.
async fn async_example(x: i32) -> i32 {
    x + 1
}

async fn async_with_let() -> i32 {
    let value: i32 = 10;
    value
}
