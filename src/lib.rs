#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

// extern crate rustc_arena;

// extern crate rustc_ast;
// extern crate rustc_ast_pretty;
// extern crate rustc_attr;
// extern crate rustc_data_structures;
// extern crate rustc_errors;
extern crate rustc_hir;
// extern crate rustc_hir_pretty;
// extern crate rustc_index;
// extern crate rustc_infer;
// extern crate rustc_lexer;
// extern crate rustc_middle;
// extern crate rustc_mir_dataflow;
// extern crate rustc_parse;
// extern crate rustc_span;
// extern crate rustc_target;
// extern crate rustc_trait_selection;

use rustc_hir::{Expr, ExprKind, Path, QPath};
use rustc_lint::LateLintPass;

dylint_linting::declare_late_lint! {
    /// ### What it does
    /// Detects usage of `flecs::With` which should be avoided.
    ///
    /// ### Why is this bad?
    /// Usage of flecs::With has been deprecated in favor of alternative approaches.
    ///
    /// ### Example
    /// ```rust
    /// use flecs::With;
    /// // Or any usage of flecs::With
    /// ```
    /// Use instead:
    /// ```rust
    /// // Use alternative approach without flecs::With
    /// ```
    pub NO_FLECS_WITH,
    Warn,
    "usage of flecs::With is discouraged"
}

use rustc_lint::LintContext;

impl<'tcx> LateLintPass<'tcx> for NoFlecsWith {
    fn check_expr(&mut self, cx: &rustc_lint::LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        match expr.kind {
            ExprKind::Path(ref qpath) => {
                let QPath::Resolved(_, path) = qpath else {
                    return;
                };

                if path_matches_flecs_with(path) {
                    cx.span_lint(NO_FLECS_WITH, expr.span, |diag| {
                        diag.note("usage of flecs::With is discouraged");
                    });
                }
            }
            ExprKind::Call(func, _) => {
                // Only check the function's path for generic arguments
                if let ExprKind::Path(QPath::Resolved(_, path)) = &func.kind {
                    if let Some(segment) = path.segments.last() {
                        if let Some(args) = &segment.args {
                            // Check if any of the generic arguments contain flecs::With
                            if args.args.iter().any(|arg| {
                                if let rustc_hir::GenericArg::Type(ty) = arg {
                                    if let rustc_hir::TyKind::Path(QPath::Resolved(_, path)) =
                                        &ty.kind
                                    {
                                        return path_matches_flecs_with(path);
                                    }
                                }
                                false
                            }) {
                                cx.span_lint(NO_FLECS_WITH, expr.span, |diag| {
                                    diag.note(
                                        "usage of flecs::With in generic argument is discouraged",
                                    );
                                });
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn path_matches_flecs_with(path: &Path) -> bool {
    path.segments.len() >= 2
        && path.segments[0].ident.as_str() == "flecs"
        && path.segments[1].ident.as_str() == "With"
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
