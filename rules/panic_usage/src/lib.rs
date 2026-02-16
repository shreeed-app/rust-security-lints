#![feature(rustc_private)]

extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use rustc_errors::Diag;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext, LintStore};
use rustc_middle::ty::TyCtxt;
use rustc_session::{Session, declare_lint, declare_lint_pass};

declare_lint! {
    pub SECURITY_PANIC_USAGE,
    Deny,
    "Detects constructs that may panic at runtime."
}

declare_lint_pass!(SecurityPanicUsage => [SECURITY_PANIC_USAGE]);

/// Enum representing the different kinds of panic-related constructs that can
/// be detected by the `SECURITY_PANIC_USAGE` lint, such as calls to `unwrap`
/// and `expect` methods, as well as calls to panic-related functions in the
/// standard library.
#[derive(Debug, Clone, Copy)]
enum PanicKind {
    Unwrap,
    Expect,
}

impl PanicKind {
    fn from_method(name: &str) -> Option<Self> {
        match name {
            "unwrap" => Some(Self::Unwrap),
            "expect" => Some(Self::Expect),
            _ => None,
        }
    }
}

/// Enum representing the different panic backends that can be detected by the
/// `SECURITY_PANIC_USAGE` lint, such as the `panicking` module, the
/// `panic_fmt` function, the `panic_display` function, the `assert_failed`
/// function, and the `begin_panic` function in the standard library.
#[derive(Debug, Clone, Copy)]
enum PanicBackend {
    PanickingModule,
    PanicFmt,
    PanicDisplay,
    AssertFailed,
    BeginPanic,
}

impl PanicBackend {
    fn from_def_path(path: &str) -> Option<Self> {
        if path.contains("panicking::") {
            Some(Self::PanickingModule)
        } else if path.contains("panic_fmt") {
            Some(Self::PanicFmt)
        } else if path.contains("panic_display") {
            Some(Self::PanicDisplay)
        } else if path.contains("assert_failed") {
            Some(Self::AssertFailed)
        } else if path.contains("begin_panic") {
            Some(Self::BeginPanic)
        } else {
            None
        }
    }
}

impl<'tcx> LateLintPass<'tcx> for SecurityPanicUsage {
    /// Detect calls to panic-related functions and methods, such as `unwrap`,
    /// `expect`, and functions in the standard library's panic module.
    ///
    /// # Arguments
    /// * `context` (`&LateContext<'tcx>`) - The lint context providing access
    ///   to the compiler's internal state.
    /// * `expression` (`&'tcx Expr<'tcx>`) - The expression being checked for
    ///   panic-related function and method calls.
    fn check_expr(
        &mut self,
        context: &LateContext<'tcx>,
        expression: &'tcx Expr<'tcx>,
    ) {
        // Detect direct calls to `unwrap` and `expect` methods.
        if let ExprKind::MethodCall(segment, _, _, _) = &expression.kind
            && let Some(kind) =
                PanicKind::from_method(segment.ident.name.as_str())
        {
            context.span_lint(
                SECURITY_PANIC_USAGE,
                expression.span,
                |diagnostic: &mut Diag<'_, ()>| {
                    diagnostic.primary_message(format!(
                        "Call to panic backend `{kind:?}` detected."
                    ));
                },
            );
            return;
        }

        // Detect calls to panic-related functions in the standard library.
        if let ExprKind::Call(func, _) = &expression.kind
            && let ExprKind::Path(path) = &func.kind
            && let Some(def_id) =
                context.qpath_res(path, func.hir_id).opt_def_id()
            && let Some(kind) =
                PanicBackend::from_def_path(&context.tcx.def_path_str(def_id))
        {
            context.span_lint(
                SECURITY_PANIC_USAGE,
                expression.span.source_callsite(),
                |diag: &mut Diag<'_, ()>| {
                    diag.primary_message(format!(
                        "Call to panic backend `{kind:?}` detected."
                    ));
                },
            );
        }
    }
}

/// Registers the `SECURITY_PANIC_USAGE` lint and its corresponding lint pass
/// with the Rust compiler. This function is called by the compiler when the
/// library is loaded as a plugin. It initializes the lint configuration and
/// registers the lint and its corresponding lint pass with the compiler's lint
/// store. This allows the `SECURITY_PANIC_USAGE` lint to be used when
/// compiling code that depends on this library, and ensures that the lint will
/// be applied to the code being compiled, allowing it to detect and warn about
/// constructs that may panic at runtime.
///
/// # Arguments
/// * `session` (`&Session`) - The compiler session providing access to the
///   compiler's internal state.
/// * `lint_store` (`&mut LintStore`) - The lint store where the
///   `SECURITY_PANIC_USAGE` lint and its corresponding lint pass will be
///   registered.
#[unsafe(no_mangle)]
pub fn register_lints(session: &Session, lint_store: &mut LintStore) {
    dylint_linting::init_config(session);

    lint_store.register_lints(&[SECURITY_PANIC_USAGE]);
    lint_store
        .register_late_pass(|_: TyCtxt<'_>| Box::new(SecurityPanicUsage));
}

dylint_linting::dylint_library!();

/// UI test for the `SECURITY_PANIC_USAGE` lint. This test compiles the code in
/// the `ui` directory with the appropriate compiler flags for UI testing. The
/// test will pass if the expected warnings are emitted for calls to
/// panic-related functions and methods, and fail if any unexpected warnings
/// are emitted or if the expected warnings are not emitted.
#[cfg(test)]
mod tests {
    use dylint_testing::ui::Test;

    #[test]
    fn ui() {
        Test::src_base(env!("CARGO_PKG_NAME"), "ui")
            .rustc_flags(["-Z", "ui-testing"])
            .run();
    }
}
