//! Remove unnecessary `unsafe`.

use rustc_ast::{
    self as ast,
    mut_visit::{self, MutVisitor as _},
};
use rustc_hash::{FxHashMap, FxHashSet};
use rustc_hir as hir;
use rustc_index::bit_set::ChunkedBitSet;
use rustc_middle::ty::TyCtxt;
use rustc_span::{Span, def_id::LocalDefId};

use crate::{ast_util, check_unsafety, graph_util};

pub fn resolve_unsafe(tcx: TyCtxt<'_>) {
    let unsafe_fns = find_unsafe_fns(tcx)
        .into_iter()
        .map(|def_id| {
            let hir::Node::Item(item) = tcx.hir_node_by_def_id(def_id) else { panic!() };
            let hir::ItemKind::Fn { ident, .. } = &item.kind else { panic!() };
            ident.span
        })
        .collect();
    let mut visitor = AstVisitor {
        unsafe_fns,
        updated: false,
    };
    let res = ast_util::transform_ast(
        |krate| {
            visitor.updated = false;
            visitor.visit_crate(krate);
            visitor.updated
        },
        tcx,
    );
    res.apply();
}

struct AstVisitor {
    unsafe_fns: FxHashSet<Span>,
    updated: bool,
}

impl mut_visit::MutVisitor for AstVisitor {
    fn visit_item(&mut self, item: &mut ast::Item) {
        if let ast::ItemKind::Fn(box ast::Fn { ident, sig, .. }) = &mut item.kind
            && !self.unsafe_fns.contains(&ident.span)
            && matches!(sig.header.safety, ast::Safety::Unsafe(_))
        {
            sig.header.safety = ast::Safety::Default;
            self.updated = true;
        }
        mut_visit::walk_item(self, item);
    }
}

#[derive(Default)]
struct UnsafetyHandler {
    callees: FxHashSet<LocalDefId>,
    is_unsafe: bool,
}

impl check_unsafety::UnsafetyHandler for UnsafetyHandler {
    fn handle_unsafety(&mut self, kind: check_unsafety::UnsafeOpKind, _: Span, tcx: TyCtxt<'_>) {
        if let check_unsafety::UnsafeOpKind::CallToUnsafeFunction(Some(def_id)) = kind
            && let Some(def_id) = def_id.as_local()
            && let hir::Node::Item(item) = tcx.hir_node_by_def_id(def_id)
            && matches!(item.kind, hir::ItemKind::Fn { .. })
        {
            self.callees.insert(def_id);
            return;
        }
        self.is_unsafe = true;
    }
}

fn find_unsafe_fns(tcx: TyCtxt<'_>) -> FxHashSet<LocalDefId> {
    let mut call_graph = FxHashMap::default();
    let mut self_unsafe_fns = FxHashSet::default();
    for item_id in tcx.hir_free_items() {
        let def_id = item_id.owner_id.def_id;
        let item = tcx.hir_item(item_id);
        let rustc_hir::ItemKind::Fn { sig, .. } = item.kind else { continue };
        let mut handler = UnsafetyHandler::default();
        check_unsafety::check_unsafety(def_id, &mut handler, tcx);
        call_graph.insert(def_id, handler.callees);
        if handler.is_unsafe || sig.decl.c_variadic {
            self_unsafe_fns.insert(def_id);
        }
    }

    let sccs = graph_util::sccs_copied(&call_graph);

    let mut is_scc_unsafe = ChunkedBitSet::new_empty(sccs.sccs.len());
    for scc_id in sccs.post_order() {
        if !is_scc_unsafe.contains(scc_id) {
            let scc = &sccs.sccs[scc_id];
            if scc.intersection(&self_unsafe_fns).next().is_some() {
                is_scc_unsafe.insert(scc_id);
            } else {
                continue;
            }
        }
        for pred in sccs.predecessors(scc_id) {
            is_scc_unsafe.insert(*pred);
        }
    }

    let mut unsafe_fns: FxHashSet<LocalDefId> = FxHashSet::default();
    for scc_id in is_scc_unsafe.iter() {
        unsafe_fns.extend(&sccs.sccs[scc_id]);
    }
    unsafe_fns
}

#[cfg(test)]
mod tests;
