use rustc_hash::FxHashMap;
use rustc_hir::{
    HirId, ItemKind, Mutability, Path,
    def::Res,
    def_id::DefId,
    intravisit::{self, Visitor as GVisitor},
};
use rustc_middle::{hir::nested_filter, ty::TyCtxt};
use rustc_span::{Span, Symbol};

pub fn run(tcx: TyCtxt<'_>) {
    let bindings = collect_global_bindings(tcx);
    let mut occurrences = 0;
    for (def_id, name) in &bindings.bindings {
        let owner = tcx.def_path_str(def_id.to_owned());
        for span in &bindings.bound_occurrences[def_id] {
            println!("GV binding `{name}` in `{owner}` at `{span:?}`");
        }
        occurrences += 1;
    }
    println!("Found {occurrences} global variable bindings.");
}

fn collect_global_bindings(tcx: TyCtxt<'_>) -> Bindings {
    let mut visitor = BindingCollector::new(tcx);
    tcx.hir_visit_all_item_likes_in_crate(&mut visitor);
    visitor.ctx
}

#[derive(Default)]
struct Bindings {
    bindings: FxHashMap<DefId, Symbol>,
    bound_occurrences: FxHashMap<DefId, Vec<Span>>,
}

struct BindingCollector<'tcx> {
    tcx: TyCtxt<'tcx>,
    ctx: Bindings,
}

// Lifetime Annotations in Method Definitions
impl<'tcx> BindingCollector<'tcx> {
    #[inline]
    fn new(tcx: TyCtxt<'tcx>) -> Self {
        BindingCollector {
            tcx,
            ctx: Bindings::default(),
        }
    }
}

impl<'tcx> GVisitor<'tcx> for BindingCollector<'tcx> {
    type NestedFilter = nested_filter::OnlyBodies;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_item(&mut self, item: &'tcx rustc_hir::Item<'tcx>) {
        if let ItemKind::Static(Mutability::Mut, ident, ..) = item.kind {
            self.ctx
                .bindings
                .insert(item.owner_id.to_def_id(), ident.name);
            self.ctx
                .bound_occurrences
                .entry(item.owner_id.to_def_id())
                .or_default();
        }
        intravisit::walk_item(self, item);
    }

    fn visit_path(&mut self, path: &Path<'tcx>, _id: HirId) {
        if let Res::Def(_, def_id) = path.res
            && self.ctx.bindings.contains_key(&def_id)
        {
            self.ctx
                .bound_occurrences
                .entry(def_id)
                .or_default()
                .push(path.span);
        }
        intravisit::walk_path(self, path);
    }
}
