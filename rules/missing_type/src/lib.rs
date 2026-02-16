#![feature(rustc_private)]

extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_session;

use rustc_errors::Diag;
use rustc_hir::{Body, Expr, ExprKind, LetStmt, PatKind};
use rustc_lint::{LateContext, LateLintPass, LintContext, LintStore};
use rustc_middle::ty::TyCtxt;
use rustc_session::{Session, declare_lint, declare_lint_pass};

// This lint detects missing explicit type annotations on let bindings, except
// when the pattern is `_`. It also detects missing explicit type annotations
// on closure parameters, except when the parameter pattern is `_`. The lint
// will emit a warning for each case where an explicit type annotation is
// missing.
// e.g. `let x = 5;` will trigger a warning, while `let _ = 5;` will not.
// Similarly, `|a, b| a + b` will trigger a warning for both parameters, while
// `|_, b| b` will only trigger a warning for the second parameter.
declare_lint! {
    pub MISSING_LET_TYPE,
    Warn,
    "Detects missing explicit type annotation on let bindings."
}

declare_lint! {
    pub MISSING_CLOSURE_PARAM_TYPE,
    Warn,
    "Detects missing explicit type annotation on closure parameters."
}

declare_lint_pass!(MissingType => [
    MISSING_LET_TYPE,
    MISSING_CLOSURE_PARAM_TYPE
]);

impl<'tcx> LateLintPass<'tcx> for MissingType {
    /// Checks for missing explicit type annotations on let bindings, except
    /// when the pattern is `_`.
    ///
    /// # Arguments
    /// * `context` (`&LateContext<'tcx>`) - The lint context providing access
    ///   to the compiler's internal state.
    /// * `local` (`&'tcx LetStmt<'tcx>`) - The let statement being checked for
    ///   a type annotation.
    fn check_local(
        &mut self,
        context: &LateContext<'tcx>,
        local: &'tcx LetStmt<'tcx>,
    ) {
        // Skip if the pattern is `_`.
        if matches!(local.pat.kind, PatKind::Wild) {
            return;
        }
        // Skip if the let statement is from a macro expansion, as it may not
        // be possible to determine the type annotation in that case.
        // Ignore anything coming from macro expansion (async_trait, derives,
        // etc.)
        if local.span.from_expansion() {
            return;
        }

        // Ignore desugared constructs (async lowering, ?, for loops, etc.)
        if local.span.desugaring_kind().is_some() {
            return;
        }

        // Check if the let statement has an explicit type annotation. If not,
        // emit a warning.
        if local.ty.is_none() {
            context.span_lint(
                MISSING_LET_TYPE,
                local.pat.span,
                |diagnostic: &mut Diag<'_, ()>| {
                    diagnostic.primary_message(
                        "Missing explicit type annotation on let binding.",
                    );
                },
            );
        }
    }

    /// Checks for missing explicit type annotations on closure parameters,
    /// except when the parameter pattern is `_`.
    ///
    /// # Arguments
    /// * `context` (`&LateContext<'tcx>`) - The lint context providing access
    ///   to the compiler's internal state.
    /// * `expression` (`&'tcx Expr<'tcx>`) - The expression being checked for
    ///   closure parameters with missing type annotations.
    fn check_expr(
        &mut self,
        context: &LateContext<'tcx>,
        expression: &'tcx Expr<'tcx>,
    ) {
        if expression.span.from_expansion() {
            return;
        }

        // Only check closure expressions.
        let ExprKind::Closure(closure): &ExprKind<'tcx> = &expression.kind
        else {
            return;
        };

        // Skip if the expression is from a macro expansion, as it may not be
        // possible to determine the type annotation in that case.
        if matches!(closure.kind, rustc_hir::ClosureKind::Coroutine(_)) {
            return;
        }

        // Get the body of the closure to access its parameters.
        let body: &Body<'_> = context.tcx.hir_body(closure.body);

        // Iterate over the parameters of the closure. For each parameter,
        // check if it has an explicit type annotation. If not, and if
        // the parameter pattern is not `_`, emit a warning.
        for param in body.params {
            // Skip if the parameter pattern is `_`.
            if matches!(param.pat.kind, PatKind::Wild) {
                continue;
            }

            // Check if the parameter has an explicit type annotation. If not,
            // emit a warning.
            if param.ty_span.is_empty() || param.ty_span == param.pat.span {
                context.span_lint(
                    MISSING_CLOSURE_PARAM_TYPE,
                    param.pat.span,
                    |diagnostic: &mut Diag<'_, ()>| {
                        diagnostic.primary_message(
                            "Closure parameter missing explicit type annotation.",
                        );
                    },
                );
            }
        }
    }
}

/// Registers the lints defined in this library with the Rust compiler. This
/// function is called by the compiler when the library is loaded as a plugin.
/// It initializes the lint configuration and registers the lints and their
/// corresponding lint pass with the compiler's lint store.
///
/// # Arguments
/// * `session` (`&Session`) - The compiler session providing access to the
///   compiler's internal state and configuration.
/// * `lint_store` (`&mut LintStore`) - The lint store where the lints defined
///   in this library will be registered.
#[unsafe(no_mangle)]
pub fn register_lints(session: &Session, lint_store: &mut LintStore) {
    dylint_linting::init_config(session);

    lint_store.register_lints(&[MISSING_LET_TYPE, MISSING_CLOSURE_PARAM_TYPE]);
    lint_store.register_late_pass(|_: TyCtxt<'_>| Box::new(MissingType));
}

dylint_linting::dylint_library!();

/// UI test for the `missing_type` lint. This test compiles the code in the
/// `ui` directory and checks that the expected warnings are emitted for
/// missing explicit type annotations on let bindings and closure parameters,
/// while ensuring that no warnings are emitted for cases where the pattern is
/// `_`. The test uses the `dylint_testing` crate to run the UI tests with the
/// appropriate compiler flags for UI testing. The test will pass if the
/// expected warnings are emitted and fail if any unexpected warnings are
/// emitted or if the expected warnings are not emitted.
#[cfg(test)]
mod tests {
    use dylint_testing::ui::Test;

    #[test]
    fn ui() {
        Test::src_base(env!("CARGO_PKG_NAME"), "ui")
            .rustc_flags(["--edition=2024", "-Z", "ui-testing"])
            .run();
    }
}
