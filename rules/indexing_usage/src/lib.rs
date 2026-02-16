#![feature(rustc_private)]

extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use rustc_errors::Diag;
use rustc_hir::{Expr, ExprKind, Item, ItemKind};
use rustc_lint::{LateContext, LateLintPass, LintContext, LintStore};
use rustc_middle::ty::TyCtxt;
use rustc_session::{Session, declare_lint, declare_lint_pass};

declare_lint! {
    pub SECURITY_INDEXING_USAGE,
    Deny,
    "Detects usage of indexing and slicing operations."
}
declare_lint_pass!(SecurityIndexingUsage => [SECURITY_INDEXING_USAGE]);

impl<'tcx> LateLintPass<'tcx> for SecurityIndexingUsage {
    /// Detect indexing and slicing operations.
    ///
    /// # Arguments
    /// * `context` (`&LateContext<'tcx>`) - The lint context, providing access
    ///   to compiler information and utilities.
    /// * `expression` (`&'tcx Expr<'tcx>`) - The expression being checked for
    ///   indexing and slicing operations.
    fn check_expr(
        &mut self,
        context: &LateContext<'tcx>,
        expression: &'tcx Expr<'tcx>,
    ) {
        match &expression.kind {
            ExprKind::Index(_, index_expr, _) => {
                match &index_expr.kind {
                    // Literal indexing: array[0].
                    ExprKind::Lit(_) => {
                        context.span_lint(
                            SECURITY_INDEXING_USAGE,
                            expression.span,
                            |diagnostic: &mut Diag<'_, ()>| {
                                diagnostic.primary_message(
                                    "Usage of indexing operation detected.",
                                );
                            },
                        );
                    },

                    // Range slicing: array[1..], array[..], array[a..b].
                    ExprKind::Struct(_, _, _) => {
                        context.span_lint(
                            SECURITY_INDEXING_USAGE,
                            expression.span,
                            |diagnostic: &mut Diag<'_, ()>| {
                                diagnostic.primary_message(
                                    "Usage of slicing operation detected.",
                                );
                            },
                        );
                    },

                    // Any other dynamic indexing: array[i].
                    _ => {
                        context.span_lint(
                            SECURITY_INDEXING_USAGE,
                            expression.span,
                            |diagnostic: &mut Diag<'_, ()>| {
                                diagnostic.primary_message(
                                    "Usage of indexing operation detected.",
                                );
                            },
                        );
                    },
                }
            },

            _ => {},
        }
    }

    /// Detect implementations of indexing traits, such as `Index` and
    /// `IndexMut`.
    ///
    /// # Arguments
    /// * `context` (`&LateContext<'tcx>`) - The lint context, providing access
    ///   to compiler information and utilities.
    /// * `item` (`&'tcx Item<'tcx>`) - The item being checked for trait
    ///   implementations.
    fn check_item(
        &mut self,
        context: &LateContext<'tcx>,
        item: &'tcx Item<'tcx>,
    ) {
        if let ItemKind::Impl(implementation) = &item.kind
            && let Some(trait_ref) = implementation.of_trait
            && let Some(def_id) = trait_ref.trait_ref.path.res.opt_def_id()
        {
            if context.tcx.lang_items().index_trait() == Some(def_id)
                || context.tcx.lang_items().index_mut_trait() == Some(def_id)
            {
                context.span_lint(
                    SECURITY_INDEXING_USAGE,
                    item.span,
                    |diagnostic: &mut Diag<'_, ()>| {
                        diagnostic.primary_message(
                            "Implementation of Index/IndexMut trait detected.",
                        );
                    },
                );
            }
        }
    }
}

/// This module defines the `SECURITY_INDEXING_USAGE` lint, which detects the
/// use of indexing and slicing operations in Rust code. The lint checks for
/// array indexing (e.g., `array[index]`), slicing (e.g., `array[1..]`), and
/// implementations of indexing traits (e.g., `Index` and `IndexMut`). The lint
/// emits a warning whenever it detects any of these patterns, helping
/// developers identify potential security issues related to indexing and
/// slicing operations.
#[unsafe(no_mangle)]
pub fn register_lints(session: &Session, lint_store: &mut LintStore) {
    dylint_linting::init_config(session);

    lint_store.register_lints(&[SECURITY_INDEXING_USAGE]);
    lint_store
        .register_late_pass(|_: TyCtxt<'_>| Box::new(SecurityIndexingUsage));
}

dylint_linting::dylint_library!();

/// This module defines the `SECURITY_INDEXING_USAGE` lint, which detects the
/// use of indexing and slicing operations in Rust code. The lint checks for
/// array indexing (e.g., `array[index]`), slicing (e.g., `array[1..]`), and
/// implementations of indexing traits (e.g., `Index` and `IndexMut`). The lint
/// emits a warning whenever it detects any of these patterns, helping
/// developers identify potential security issues related to indexing and
/// slicing operations. The lint is designed to be used in security-sensitive
/// contexts, where the use of indexing and slicing operations may lead to
/// out-of-bounds access or other vulnerabilities if not used carefully.
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
