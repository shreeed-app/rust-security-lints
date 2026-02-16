#![feature(rustc_private)]

extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_session;

use rustc_errors::Diag;
use rustc_hir::{
    BlockCheckMode,
    Expr,
    ExprKind,
    HeaderSafety,
    Item,
    ItemKind,
    Safety,
    UnsafeSource,
};
use rustc_lint::{LateContext, LateLintPass, LintContext, LintStore};
use rustc_middle::ty::TyCtxt;
use rustc_session::{Session, declare_lint, declare_lint_pass};

declare_lint! {
    pub SECURITY_UNSAFE_USAGE,
    Deny,
    "Detects usage of unsafe blocks, unsafe functions, unsafe
    traits and unsafe implementations."
}

declare_lint_pass!(SecurityUnsafeUsage => [SECURITY_UNSAFE_USAGE]);

impl<'tcx> LateLintPass<'tcx> for SecurityUnsafeUsage {
    /// Detect unsafe blocks with user-provided unsafe source.
    ///
    /// # Arguments
    /// * `context` (`&LateContext<'tcx>`) - The lint context providing access
    ///   to the compiler's internal state.
    /// * `expression` (`&'tcx Expr<'tcx>`) - The expression being checked for
    ///   unsafe block usage.
    fn check_expr(
        &mut self,
        context: &LateContext<'tcx>,
        expression: &'tcx Expr<'tcx>,
    ) {
        // Only check block expressions.
        if let ExprKind::Block(block, _) = &expression.kind
            && let BlockCheckMode::UnsafeBlock(UnsafeSource::UserProvided) =
                block.rules
        {
            context.span_lint(
                SECURITY_UNSAFE_USAGE,
                expression.span,
                |diagnostic: &mut Diag<'_, ()>| {
                    diagnostic
                        .primary_message("Usage of unsafe block detected.");
                },
            );
        }
    }

    /// Detect unsafe function, trait and implementation definitions.
    ///
    /// # Arguments
    /// * `context` (`&LateContext<'tcx>`) - The lint context providing access
    ///   to the compiler's internal state.
    /// * `item` (`&'tcx Item<'tcx>`) - The item being checked for unsafe
    ///   function, trait or implementation definitions.
    fn check_item(
        &mut self,
        context: &LateContext<'tcx>,
        item: &'tcx Item<'tcx>,
    ) {
        match &item.kind {
            // Unsafe function.
            ItemKind::Fn { sig, .. } => {
                if matches!(
                    sig.header.safety,
                    HeaderSafety::Normal(Safety::Unsafe)
                ) {
                    context.span_lint(
                        SECURITY_UNSAFE_USAGE,
                        item.span,
                        |diagnostic: &mut Diag<'_, ()>| {
                            diagnostic
                                .primary_message("Unsafe function detected.");
                        },
                    );
                }
            },

            // Unsafe trait.
            ItemKind::Trait(_, _, safety, _, _, _, _) => {
                if *safety == Safety::Unsafe {
                    context.span_lint(
                        SECURITY_UNSAFE_USAGE,
                        item.span,
                        |diagnostic: &mut Diag<'_, ()>| {
                            diagnostic
                                .primary_message("Unsafe trait detected.");
                        },
                    );
                }
            },

            // Unsafe implementation.
            ItemKind::Impl(impl_) => {
                if let Some(trait_impl) = impl_.of_trait
                    && trait_impl.safety == Safety::Unsafe
                {
                    context.span_lint(
                        SECURITY_UNSAFE_USAGE,
                        item.span,
                        |diagnostic: &mut Diag<'_, ()>| {
                            diagnostic
                                .primary_message("Unsafe impl detected.");
                        },
                    );
                }
            },

            _ => {},
        }
    }
}

/// Registers the `SECURITY_UNSAFE_USAGE` lint and its corresponding lint pass
/// with the Rust compiler.
///
/// # Arguments
/// * `session` (`&Session`) - The compiler session providing access to the
///   compiler's internal state.
/// * `lint_store` (`&mut LintStore`) - The lint store where the
///   `SECURITY_UNSAFE_USAGE` lint and its corresponding lint pass will be
///   registered.
#[unsafe(no_mangle)]
pub fn register_lints(session: &Session, lint_store: &mut LintStore) {
    dylint_linting::init_config(session);

    lint_store.register_lints(&[SECURITY_UNSAFE_USAGE]);
    lint_store
        .register_late_pass(|_: TyCtxt<'_>| Box::new(SecurityUnsafeUsage));
}

dylint_linting::dylint_library!();

/// UI tests for the `SECURITY_UNSAFE_USAGE` lint. These tests are located in
/// the `ui` directory and are compiled with the appropriate compiler flags for
/// UI testing. The tests check that the expected warnings are emitted for
/// unsafe block usage, unsafe function definitions, unsafe trait definitions
/// and unsafe implementation definitions, while ensuring that no warnings are
/// emitted for safe code. The tests will pass if the expected warnings are
/// emitted and fail if any unexpected warnings are emitted or if the expected
/// warnings are not emitted.
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
