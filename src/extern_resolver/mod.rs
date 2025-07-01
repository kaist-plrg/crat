//! Replaces extern items with `use`.

use ast::ptr::P;
use etrace::some_or;
use rustc_abi::VariantIdx;
use rustc_ast::{
    self as ast,
    mut_visit::{self, MutVisitor as _},
};
use rustc_hash::{FxHashMap, FxHashSet};
use rustc_hir::{self as hir, HirId, def::Res, definitions::DefPathData, intravisit};
use rustc_middle::{
    hir::nested_filter,
    ty::{self, List, TyCtxt, TypeSuperVisitable, TypeVisitor},
};
use rustc_span::{Span, Symbol, def_id::LocalDefId};
use serde::Deserialize;
use smallvec::SmallVec;
use thin_vec::ThinVec;
use typed_arena::Arena;

use crate::{
    ast_util::{self, TransformationResult},
    disjoint_set::DisjointSets,
    equiv_classes::{EquivClassId, EquivClasses},
    graph_util, ir_util,
};

#[derive(Debug, Default, Deserialize)]
struct ResolveHints {
    functions: Option<Vec<LinkHint>>,
    variables: Option<Vec<LinkHint>>,
    types: Option<Vec<LinkHint>>,
}

#[derive(Debug, Deserialize)]
struct LinkHint {
    from: String,
    to: String,
}

pub fn resolve_extern(hints: Option<String>, tcx: TyCtxt<'_>) -> TransformationResult {
    let hints: ResolveHints = hints
        .map(|hints| toml::from_str(&hints).unwrap())
        .unwrap_or_default();

    let result = resolve(tcx);

    let mut resolve_map = FxHashMap::default();
    for classes in result
        .equiv_adts
        .values()
        .chain(result.equiv_tys.values())
        .chain(result.equiv_fns.values())
        .chain(result.equiv_statics.values())
        .chain(std::iter::once(&result.equiv_unnameds))
    {
        for class in &classes.0 {
            let rep = find_representative_def_id(class, tcx);
            for def_id in class {
                if *def_id != rep {
                    resolve_map.insert(*def_id, rep);
                }
            }
        }
    }

    let mut link_failed = false;
    link_failed |= link_externs(
        "Type",
        &result.extern_adts,
        &result.equiv_adts,
        &mut resolve_map,
        hints.types.as_ref(),
        tcx,
    );
    link_failed |= link_externs(
        "Function",
        &result.extern_fns,
        &result.equiv_fns,
        &mut resolve_map,
        hints.functions.as_ref(),
        tcx,
    );
    link_failed |= link_externs(
        "Static",
        &result.extern_statics,
        &result.equiv_statics,
        &mut resolve_map,
        hints.variables.as_ref(),
        tcx,
    );

    assert!(!link_failed);

    let mut visitor = AstVisitor {
        tcx,
        span_to_def_id: result.span_to_def_id,
        resolve_map,
        used: FxHashSet::default(),
        updated: false,
    };
    ast_util::transform_ast(
        |krate| {
            visitor.updated = false;
            visitor.visit_crate(krate);
            let mut used: Vec<_> = visitor
                .used
                .drain()
                .map(|def_id| tcx.def_path_str(def_id))
                .collect();
            used.sort();
            let mut items: ThinVec<_> = used
                .into_iter()
                .map(|s| P(item!("use crate::{};", s)))
                .collect();
            items.append(&mut krate.items);
            krate.items = items;
            visitor.updated
        },
        tcx,
    )
}

#[inline]
fn link_externs(
    kind: &str,
    externs: &[(LocalDefId, Vec<EquivClassId>)],
    equivs: &FxHashMap<Symbol, EquivClasses<LocalDefId>>,
    resolve_map: &mut FxHashMap<LocalDefId, LocalDefId>,
    hints: Option<&Vec<LinkHint>>,
    tcx: TyCtxt<'_>,
) -> bool {
    let hints: FxHashMap<_, _> = hints
        .map(|hints| {
            hints
                .iter()
                .map(|hint| (hint.from.as_str(), hint.to.as_str()))
                .collect()
        })
        .unwrap_or_default();

    let mut link_failed = false;
    for (def_id, link_candidates) in externs {
        let name = ir_util::def_id_to_symbol(*def_id, tcx).unwrap();
        let classes = &equivs[&name];

        let mut link_candidates = link_candidates;
        let candidates: Vec<_>;
        if link_candidates.len() > 1
            && let Some(hint) = hints.get(tcx.def_path_str(*def_id).as_str())
        {
            candidates = link_candidates
                .iter()
                .copied()
                .filter(|id| {
                    let class = &classes.0[*id];
                    class
                        .iter()
                        .any(|def_id| tcx.def_path_str(*def_id) == *hint)
                })
                .collect();
            link_candidates = &candidates;
        }

        if let [id] = link_candidates[..] {
            let class = &classes.0[id];
            let rep = find_representative_def_id(class, tcx);
            resolve_map.insert(*def_id, rep);
        } else {
            eprintln!(
                "{kind} link failed: {} ({link_candidates:?})",
                tcx.def_path_str(*def_id)
            );
            for (id, class) in classes.0.iter_enumerated() {
                eprintln!("  {id:?}");
                for def_id in class {
                    eprintln!("    {}", tcx.def_path_str(*def_id));
                }
            }
            link_failed = true;
        }
    }
    link_failed
}

#[inline]
fn find_representative_def_id(def_ids: &[LocalDefId], tcx: TyCtxt<'_>) -> LocalDefId {
    def_ids
        .iter()
        .copied()
        .min_by_key(|def_id| tcx.def_path_str(*def_id))
        .unwrap()
}

struct ResolveResult {
    span_to_def_id: FxHashMap<Span, LocalDefId>,

    equiv_adts: FxHashMap<Symbol, EquivClasses<LocalDefId>>,
    equiv_unnameds: EquivClasses<LocalDefId>,
    extern_adts: Vec<(LocalDefId, Vec<EquivClassId>)>,

    equiv_tys: FxHashMap<Symbol, EquivClasses<LocalDefId>>,

    equiv_fns: FxHashMap<Symbol, EquivClasses<LocalDefId>>,
    extern_fns: Vec<(LocalDefId, Vec<EquivClassId>)>,

    equiv_statics: FxHashMap<Symbol, EquivClasses<LocalDefId>>,
    extern_statics: Vec<(LocalDefId, Vec<EquivClassId>)>,
}

fn resolve(tcx: TyCtxt<'_>) -> ResolveResult {
    let mut visitor = HirVisitor::new(tcx);
    tcx.hir_visit_all_item_likes_in_crate(&mut visitor);
    let hir_data = visitor.data;

    let mut name_to_adts: FxHashMap<_, Vec<_>> = FxHashMap::default();
    let mut dependencies: FxHashMap<_, FxHashSet<_>> = FxHashMap::default();
    for def_id in hir_data.adts.iter().copied() {
        let adt_def = tcx.adt_def(def_id);
        let variant = adt_def.variant(VariantIdx::ZERO);
        let mut visitor = AdtVisitor::default();
        for fd in &variant.fields {
            let ty = fd.ty(tcx, List::empty());
            visitor.visit_ty(ty);
        }
        let name = ir_util::def_id_to_symbol(def_id, tcx).unwrap();
        name_to_adts.entry(name).or_default().push(def_id);
        dependencies.entry(name).or_default().extend(
            visitor
                .adts
                .into_iter()
                .map(|def_id| ir_util::def_id_to_symbol(def_id, tcx).unwrap()),
        );
    }

    let sccs = graph_util::sccs_copied(&dependencies);
    let arena = Arena::new();
    let mut cmp = TypeComparator {
        tcx,
        equiv_adts: DisjointSets::new(&arena),
        equiv_unnameds: DisjointSets::new(&arena),
        compared_names: FxHashSet::default(),
        possibly_equiv_unnameds: FxHashSet::default(),
        visited_names: FxHashSet::default(),
    };
    let mut equiv_adts = FxHashMap::default();
    for scc_id in sccs.post_order() {
        let scc = &sccs.sccs[scc_id];
        for name in scc {
            if is_unnamed(name.as_str()) {
                continue;
            }
            let adts = &name_to_adts[name];
            let mut classes = EquivClasses::new();
            for def_id in adts {
                classes.insert(*def_id, |id1, id2| {
                    let res = cmp.cmp_adts(*id1, *id2);
                    let possibly_equiv_unnameds = cmp.possibly_equiv_unnameds.drain();
                    if res {
                        cmp.equiv_adts.union(*id1, *id2);
                        for (unnamed1, unnamed2) in possibly_equiv_unnameds {
                            cmp.equiv_unnameds.union(unnamed1, unnamed2);
                        }
                    }
                    res
                });
            }
            equiv_adts.insert(*name, classes);
            cmp.compared_names.insert(*name);
        }
    }
    let equiv_unnameds = cmp.equiv_unnameds.to_equiv_classes();
    let mut extern_adts = vec![];
    for def_id in hir_data.foreign_tys {
        let name = ir_util::def_id_to_symbol(def_id, tcx).unwrap();
        let classes = some_or!(equiv_adts.get_mut(&name), continue);
        let mut link_candidates: Vec<_> = classes.0.indices().collect();
        filter_by_common_def_path(&mut link_candidates, def_id, classes, tcx);
        extern_adts.push((def_id, link_candidates));
    }

    let mut equiv_tys: FxHashMap<_, EquivClasses<LocalDefId>> = FxHashMap::default();
    for def_id in hir_data.tys {
        let name = ir_util::def_id_to_symbol(def_id, tcx).unwrap();
        if is_unnamed(name.as_str()) {
            continue;
        }
        let classes = equiv_tys.entry(name).or_insert_with(EquivClasses::new);
        classes.insert(def_id, |id1, id2| cmp.cmp_type_of(*id1, *id2));
    }

    let mut equiv_fns: FxHashMap<_, EquivClasses<LocalDefId>> = FxHashMap::default();
    for def_id in hir_data.fns {
        let name = ir_util::def_id_to_symbol(def_id, tcx).unwrap();
        let name_str = name.as_str();
        if name_str == "main" {
            continue;
        }
        let classes = equiv_fns.entry(name).or_insert_with(EquivClasses::new);
        classes.insert(def_id, |id1, id2| {
            let hir::Node::Item(item1) = tcx.hir_node_by_def_id(*id1) else { panic!() };
            let hir::Node::Item(item2) = tcx.hir_node_by_def_id(*id2) else { panic!() };
            let span1 = item1.span;
            let span2 = item2.span;
            let source_map = tcx.sess.source_map();
            let str1 = source_map.span_to_snippet(span1).unwrap();
            let str2 = source_map.span_to_snippet(span2).unwrap();
            str1 == str2 && cmp.cmp_fn_sigs(*id1, *id2)
        });
    }
    let mut extern_fns = vec![];
    for def_id in hir_data.foreign_fns {
        let name = ir_util::def_id_to_symbol(def_id, tcx).unwrap();
        let classes = some_or!(equiv_fns.get_mut(&name), continue);
        let mut link_candidates: Vec<_> = classes
            .0
            .iter_enumerated()
            .filter_map(|(id, class)| {
                if cmp.cmp_fn_sigs(class[0], def_id) {
                    Some(id)
                } else {
                    None
                }
            })
            .collect();
        filter_by_common_def_path(&mut link_candidates, def_id, classes, tcx);
        extern_fns.push((def_id, link_candidates));
    }

    let mut equiv_statics: FxHashMap<_, EquivClasses<LocalDefId>> = FxHashMap::default();
    for def_id in hir_data.statics {
        let name = ir_util::def_id_to_symbol(def_id, tcx).unwrap();
        let classes = equiv_statics.entry(name).or_insert_with(EquivClasses::new);
        classes.insert(def_id, |id1, id2| {
            if !cmp.cmp_type_of(*id1, *id2) {
                return false;
            }
            let init1 = tcx.eval_static_initializer(*id1);
            let init2 = tcx.eval_static_initializer(*id2);
            init1 == init2
        });
    }
    let mut extern_statics = vec![];
    for def_id in hir_data.foreign_statics {
        let name = ir_util::def_id_to_symbol(def_id, tcx).unwrap();
        let classes = some_or!(equiv_statics.get_mut(&name), continue);
        let mut link_candidates: Vec<_> = classes
            .0
            .iter_enumerated()
            .filter_map(|(id, class)| {
                if cmp.cmp_type_of(class[0], def_id) {
                    Some(id)
                } else {
                    None
                }
            })
            .collect();
        filter_by_common_def_path(&mut link_candidates, def_id, classes, tcx);
        extern_statics.push((def_id, link_candidates));
    }

    ResolveResult {
        span_to_def_id: hir_data.span_to_def_id,
        equiv_adts,
        equiv_unnameds,
        extern_adts,
        equiv_tys,
        equiv_fns,
        extern_fns,
        equiv_statics,
        extern_statics,
    }
}

fn filter_by_common_def_path(
    candidates: &mut Vec<EquivClassId>,
    def_id: LocalDefId,
    classes: &EquivClasses<LocalDefId>,
    tcx: TyCtxt<'_>,
) {
    if candidates.len() > 1 {
        let common_lengths: Vec<_> = candidates
            .iter()
            .map(|id| {
                let class = &classes.0[*id];
                let max = class
                    .iter()
                    .map(|def_id0| common_def_path_len(def_id, *def_id0, tcx))
                    .max()
                    .unwrap();
                (*id, max)
            })
            .collect();
        candidates.clear();
        let mut max = 0;
        for (i, len) in common_lengths {
            if len > max {
                max = len;
                candidates.clear();
                candidates.push(i);
            } else if len == max {
                candidates.push(i);
            }
        }
    }
}

fn common_def_path_len(def_id1: LocalDefId, def_id2: LocalDefId, tcx: TyCtxt<'_>) -> usize {
    let def_path1 = tcx.def_path(def_id1.to_def_id());
    let def_path2 = tcx.def_path(def_id2.to_def_id());
    def_path1
        .data
        .into_iter()
        .zip(def_path2.data)
        .take_while(|(data1, data2)| {
            let DefPathData::TypeNs(name1) = data1.data else { panic!() };
            let DefPathData::TypeNs(name2) = data2.data else { panic!() };
            name1 == name2
        })
        .count()
}

struct TypeComparator<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,

    equiv_adts: DisjointSets<'a, LocalDefId>,
    equiv_unnameds: DisjointSets<'a, LocalDefId>,
    compared_names: FxHashSet<Symbol>,

    possibly_equiv_unnameds: FxHashSet<(LocalDefId, LocalDefId)>,
    visited_names: FxHashSet<Symbol>,
}

impl<'tcx> TypeComparator<'_, 'tcx> {
    fn cmp_adts(&mut self, def_id1: LocalDefId, def_id2: LocalDefId) -> bool {
        let name = ir_util::def_id_to_symbol(def_id1, self.tcx).unwrap();
        let unnamed = is_unnamed(name.as_str());
        if !unnamed {
            if self.compared_names.contains(&name) {
                return self.equiv_adts.equiv(def_id1, def_id2) == Some(true);
            }
            self.visited_names.insert(name);
        }
        let res = self.cmp_adts_inner(def_id1, def_id2);
        if !unnamed {
            assert!(self.visited_names.remove(&name));
        }
        res
    }

    fn cmp_adts_inner(&mut self, def_id1: LocalDefId, def_id2: LocalDefId) -> bool {
        let adt1 = self.tcx.adt_def(def_id1);
        let adt2 = self.tcx.adt_def(def_id2);
        if adt1.adt_kind() != adt2.adt_kind() {
            return false;
        }

        let variant1 = adt1.variant(VariantIdx::ZERO);
        let variant2 = adt2.variant(VariantIdx::ZERO);
        if variant1.fields.len() != variant2.fields.len() {
            return false;
        }

        for (fd1, fd2) in variant1.fields.iter().zip(variant2.fields.iter()) {
            let ty1 = fd1.ty(self.tcx, List::empty());
            let ty2 = fd2.ty(self.tcx, List::empty());
            if !self.cmp_tys(ty1, ty2) {
                return false;
            }
        }

        true
    }

    fn cmp_tys(&mut self, ty1: ty::Ty<'tcx>, ty2: ty::Ty<'tcx>) -> bool {
        use ty::*;
        let ty1_kind = ty1.kind();
        let ty2_kind = ty2.kind();
        match ty1_kind {
            Bool | Char | Int(_) | Uint(_) | Float(_) | Never => ty1_kind == ty2_kind,
            Adt(adt_def1, args1) => {
                if let Foreign(def_id2) = ty2_kind {
                    let def_id1 = adt_def1.did();
                    let name1 = ir_util::def_id_to_symbol(def_id1, self.tcx).unwrap();
                    let name2 = ir_util::def_id_to_symbol(def_id2, self.tcx).unwrap();
                    return name1 == name2;
                }
                let Adt(adt_def2, args2) = ty2_kind else { return false };

                if args1.len() != args2.len() {
                    return false;
                }

                let def_id1 = adt_def1.did();
                let name1 = ir_util::def_id_to_symbol(def_id1, self.tcx).unwrap();
                let def_id2 = adt_def2.did();
                let name2 = ir_util::def_id_to_symbol(def_id2, self.tcx).unwrap();

                match (def_id1.as_local(), def_id2.as_local()) {
                    (Some(def_id1), Some(def_id2)) => {
                        assert!(args1.is_empty());
                        match (is_unnamed(name1.as_str()), is_unnamed(name2.as_str())) {
                            (true, true) => {
                                let res = self.cmp_adts(def_id1, def_id2);
                                if res {
                                    self.possibly_equiv_unnameds.insert((def_id1, def_id2));
                                }
                                res
                            }
                            (false, false) => {
                                if name1 != name2 {
                                    false
                                } else if self.visited_names.contains(&name1) {
                                    true
                                } else {
                                    self.cmp_adts(def_id1, def_id2)
                                }
                            }
                            _ => false,
                        }
                    }
                    (None, None) => {
                        def_id1 == def_id2
                            && args1.iter().zip(args2.iter()).all(|(arg1, arg2)| {
                                use rustc_type_ir::GenericArgKind::*;
                                match (arg1.unpack(), arg2.unpack()) {
                                    (Type(ty1), Type(ty2)) => self.cmp_tys(ty1, ty2),
                                    (Lifetime(_), Lifetime(_)) => true,
                                    (Const(_), Const(_)) => true,
                                    _ => false,
                                }
                            })
                    }
                    _ => false,
                }
            }
            Foreign(def_id1) => match ty2_kind {
                Adt(_, _) => self.cmp_tys(ty2, ty1),
                Foreign(def_id2) => {
                    let name1 = ir_util::def_id_to_symbol(def_id1, self.tcx).unwrap();
                    let name2 = ir_util::def_id_to_symbol(def_id2, self.tcx).unwrap();
                    name1 == name2
                }
                _ => false,
            },
            Array(ty1, _) => {
                let Array(ty2, _) = ty2_kind else { return false };
                self.cmp_tys(*ty1, *ty2)
            }
            RawPtr(ty1, m1) => {
                let RawPtr(ty2, m2) = ty2_kind else { return false };
                m1 == m2 && self.cmp_tys(*ty1, *ty2)
            }
            FnPtr(sig1, header1) => {
                let FnPtr(sig2, header2) = ty2_kind else { return false };
                if header1 != header2 {
                    return false;
                }
                let sig1 = sig1.skip_binder();
                let sig2 = sig2.skip_binder();
                if !self.cmp_tys(sig1.output(), sig2.output()) {
                    return false;
                }
                let inputs1 = sig1.inputs();
                let inputs2 = sig2.inputs();
                if inputs1.len() != inputs2.len() {
                    return false;
                }
                inputs1
                    .iter()
                    .zip(inputs2)
                    .all(|(ty1, ty2)| self.cmp_tys(*ty1, *ty2))
            }
            Tuple(tys1) => {
                let Tuple(tys2) = ty2_kind else { return false };
                // Only appears as (), being the return type of a function.
                assert_eq!(tys1.len(), 0);
                assert_eq!(tys2.len(), 0);
                true
            }
            _ => panic!("{ty1_kind:?}"),
        }
    }

    fn cmp_fn_sigs(&mut self, def_id1: LocalDefId, def_id2: LocalDefId) -> bool {
        let sig1 = self.tcx.fn_sig(def_id1).skip_binder().skip_binder();
        let sig2 = self.tcx.fn_sig(def_id2).skip_binder().skip_binder();
        let inputs1 = sig1.inputs();
        let inputs2 = sig2.inputs();
        if sig1.c_variadic != sig2.c_variadic
            || sig1.safety != sig2.safety
            || sig1.abi != sig2.abi
            || inputs1.len() != inputs2.len()
        {
            return false;
        }
        for (ty1, ty2) in inputs1.iter().zip(inputs2) {
            if !self.cmp_tys(*ty1, *ty2) {
                return false;
            }
        }
        self.cmp_tys(sig1.output(), sig2.output())
    }

    fn cmp_type_of(&mut self, def_id1: LocalDefId, def_id2: LocalDefId) -> bool {
        let ty1 = self.tcx.type_of(def_id1).skip_binder();
        let ty2 = self.tcx.type_of(def_id2).skip_binder();
        self.cmp_tys(ty1, ty2)
    }
}

fn is_unnamed(name: &str) -> bool {
    name.starts_with("C2RustUnnamed")
}

struct AstVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    span_to_def_id: FxHashMap<Span, LocalDefId>,
    resolve_map: FxHashMap<LocalDefId, LocalDefId>,
    used: FxHashSet<LocalDefId>,
    updated: bool,
}

impl mut_visit::MutVisitor for AstVisitor<'_> {
    fn flat_map_item(&mut self, item: P<ast::Item>) -> SmallVec<[P<ast::Item>; 1]> {
        if let ast::ItemKind::Fn(box ast::Fn { ident, .. })
        | ast::ItemKind::Static(box ast::StaticItem { ident, .. })
        | ast::ItemKind::Struct(ident, _, _)
        | ast::ItemKind::Union(ident, _, _)
        | ast::ItemKind::TyAlias(box ast::TyAlias { ident, .. }) = &item.kind
            && self
                .resolve_map
                .contains_key(&self.span_to_def_id[&ident.span])
        {
            self.updated = true;
            SmallVec::new()
        } else {
            let mut items = mut_visit::walk_flat_map_item(self, item);
            items.retain(|item| {
                if let ast::ItemKind::ForeignMod(fm) = &item.kind {
                    !fm.items.is_empty()
                } else {
                    true
                }
            });
            items
        }
    }

    fn flat_map_foreign_item(
        &mut self,
        item: P<ast::ForeignItem>,
    ) -> SmallVec<[P<ast::ForeignItem>; 1]> {
        if let ast::ForeignItemKind::Fn(box ast::Fn { ident, .. })
        | ast::ForeignItemKind::Static(box ast::StaticItem { ident, .. })
        | ast::ForeignItemKind::TyAlias(box ast::TyAlias { ident, .. }) = &item.kind
            && self
                .resolve_map
                .contains_key(&self.span_to_def_id[&ident.span])
        {
            self.updated = true;
            SmallVec::new()
        } else {
            mut_visit::walk_flat_map_foreign_item(self, item)
        }
    }

    fn visit_path(&mut self, path: &mut ast::Path) {
        if let Some(def_id) = self.span_to_def_id.get(&path.span)
            && let Some(resolved) = self.resolve_map.get(def_id)
        {
            self.updated = true;
            let name = ir_util::def_id_to_symbol(*resolved, self.tcx).unwrap();
            if is_unnamed(name.as_str()) {
                *path = path!("crate::{}", self.tcx.def_path_str(*resolved));
            } else {
                path.segments.last_mut().unwrap().ident.name = name;
                self.used.insert(*resolved);
            }
        }

        walk_path(self, path);
    }
}

fn walk_path<T: mut_visit::MutVisitor>(
    vis: &mut T,
    ast::Path {
        segments,
        span,
        tokens: _,
    }: &mut ast::Path,
) {
    for segment in segments {
        vis.visit_path_segment(segment);
    }
    vis.visit_span(span);
}

struct HirVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    data: HirData,
}

impl<'tcx> HirVisitor<'tcx> {
    #[inline]
    fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self {
            tcx,
            data: HirData::default(),
        }
    }
}

#[derive(Default)]
struct HirData {
    fns: Vec<LocalDefId>,
    statics: Vec<LocalDefId>,
    adts: Vec<LocalDefId>,
    tys: Vec<LocalDefId>,
    foreign_fns: Vec<LocalDefId>,
    foreign_statics: Vec<LocalDefId>,
    foreign_tys: Vec<LocalDefId>,

    span_to_def_id: FxHashMap<Span, LocalDefId>,
}

impl<'tcx> intravisit::Visitor<'tcx> for HirVisitor<'tcx> {
    type NestedFilter = nested_filter::OnlyBodies;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_item(&mut self, item: &'tcx hir::Item<'tcx>) -> Self::Result {
        let ident_and_vec = match item.kind {
            hir::ItemKind::Fn { ident, .. } => Some((ident, &mut self.data.fns)),
            hir::ItemKind::Static(ident, ..) => Some((ident, &mut self.data.statics)),
            hir::ItemKind::Struct(ident, _, _) | hir::ItemKind::Union(ident, _, _) => {
                Some((ident, &mut self.data.adts))
            }
            hir::ItemKind::TyAlias(ident, _, _) => Some((ident, &mut self.data.tys)),
            _ => None,
        };
        let def_id = item.owner_id.def_id;
        if let Some((ident, vec)) = ident_and_vec {
            if self.tcx.visibility(def_id).is_public() {
                vec.push(def_id);
            }
            self.data.span_to_def_id.insert(ident.span, def_id);
        }

        intravisit::walk_item(self, item)
    }

    fn visit_foreign_item(&mut self, item: &'tcx hir::ForeignItem<'tcx>) -> Self::Result {
        let def_id = item.owner_id.def_id;
        match item.kind {
            hir::ForeignItemKind::Fn(_, _, _) => {
                self.data.foreign_fns.push(def_id);
            }
            hir::ForeignItemKind::Static(_, _, _) => {
                self.data.foreign_statics.push(def_id);
            }
            hir::ForeignItemKind::Type => {
                self.data.foreign_tys.push(def_id);
            }
        }
        self.data.span_to_def_id.insert(item.ident.span, def_id);

        intravisit::walk_foreign_item(self, item)
    }

    fn visit_path(&mut self, path: &hir::Path<'tcx>, _hir_id: HirId) -> Self::Result {
        if let Res::Def(_, def_id) = path.res
            && let Some(def_id) = def_id.as_local()
        {
            self.data.span_to_def_id.insert(path.span, def_id);
        }

        intravisit::walk_path(self, path)
    }
}

#[derive(Default)]
struct AdtVisitor {
    adts: FxHashSet<LocalDefId>,
}

impl<'tcx> TypeVisitor<TyCtxt<'tcx>> for AdtVisitor {
    fn visit_ty(&mut self, t: ty::Ty<'tcx>) -> Self::Result {
        if let ty::TyKind::Adt(adt_def, _) = t.kind()
            && let Some(def_id) = adt_def.did().as_local()
        {
            self.adts.insert(def_id);
        }
        t.super_visit_with(self)
    }
}

#[cfg(test)]
mod tests;
