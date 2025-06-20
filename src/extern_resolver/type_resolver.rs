use etrace::some_or;
use rustc_abi::VariantIdx;
use rustc_hash::{FxHashMap, FxHashSet};
use rustc_hir::{self as hir, definitions::DefPathData, intravisit};
use rustc_index::IndexVec;
use rustc_middle::{
    hir::nested_filter,
    ty::{self, List, TyCtxt, TypeSuperVisitable, TypeVisitor},
};
use rustc_span::{Symbol, def_id::LocalDefId};
use typed_arena::Arena;

use crate::{compile_util::Pass, disjoint_set::DisjointSets, graph_util, ir_util};

pub struct TypeResolver;

impl Pass for TypeResolver {
    type Out = ();

    fn run(&self, tcx: TyCtxt<'_>) -> Self::Out {
        let result = resolve(tcx);
        let ResolveResult {
            equiv_adts: _equiv_adts,
            equiv_fns: _equiv_fns,
            unlinked_foreign_fns: _unlinked_foreign_fns,
            equiv_statics: _equiv_statics,
            unlinked_foreign_statics: _unlinked_foreign_statics,
        } = result;

        // for (name, classes) in &result.equiv_adts {
        //     if classes.0.len() > 1 {
        //         println!("{name}");
        //         for (id, def_ids) in classes.0.iter_enumerated() {
        //             println!("  {id:?}");
        //             for def_id in def_ids {
        //                 println!("    {def_id:?}");
        //             }
        //         }
        //     }
        // }

        // for (name, classes) in &_equiv_fns {
        //     let unlinked = _unlinked_foreign_fns.get(name);
        //     if unlinked.is_some() {
        //         println!("{name}");
        //         for (id, def_ids) in classes.0.iter_enumerated() {
        //             println!("  {id:?}");
        //             for def_id in def_ids {
        //                 println!("    {def_id:?}");
        //             }
        //         }
        //         if let Some(unlinked) = unlinked {
        //             println!("  Unlinked");
        //             for def_id in unlinked {
        //                 println!("    {def_id:?}");
        //             }
        //         }
        //     }
        // }

        for (name, classes) in &_equiv_statics {
            let unlinked = _unlinked_foreign_statics.get(name);
            if classes.0.len() > 1 || unlinked.is_some() {
                println!("{name}");
                for (id, def_ids) in classes.0.iter_enumerated() {
                    println!("  {id:?}");
                    for def_id in def_ids {
                        println!("    {def_id:?}");
                    }
                }
                if let Some(unlinked) = unlinked {
                    println!("  Unlinked");
                    for def_id in unlinked {
                        println!("    {def_id:?}");
                    }
                }
            }
        }

        // let mut tys: FxHashMap<_, EquivClasses<LocalDefId>> = FxHashMap::default();
        // for def_id in hir_data.tys {
        //     let name = ir_util::def_id_to_ty_symbol(def_id, tcx).unwrap();
        //     let classes = tys.entry(name).or_insert_with(EquivClasses::new);
        //     classes.insert(def_id, |id1, id2| {
        //         let ty1 = tcx.type_of(*id1).skip_binder();
        //         let ty2 = tcx.type_of(*id2).skip_binder();
        //         cmp.cmp_tys(ty1, ty2, None, &FxHashSet::default())
        //     });
        // }
    }
}

struct ResolveResult {
    equiv_adts: FxHashMap<Symbol, EquivClasses<LocalDefId>>,
    equiv_fns: FxHashMap<Symbol, EquivClasses<LocalDefId>>,
    unlinked_foreign_fns: FxHashMap<Symbol, Vec<(LocalDefId, Vec<EquivClassId>)>>,
    equiv_statics: FxHashMap<Symbol, EquivClasses<LocalDefId>>,
    unlinked_foreign_statics: FxHashMap<Symbol, Vec<(LocalDefId, Vec<EquivClassId>)>>,
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
        let name = ir_util::def_id_to_ty_symbol(def_id, tcx).unwrap();
        name_to_adts.entry(name).or_default().push(def_id);
        dependencies.entry(name).or_default().extend(
            visitor
                .adts
                .into_iter()
                .map(|def_id| ir_util::def_id_to_ty_symbol(def_id, tcx).unwrap()),
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

    let mut equiv_fns: FxHashMap<_, EquivClasses<LocalDefId>> = FxHashMap::default();
    for def_id in hir_data.fns {
        let name = ir_util::def_id_to_value_symbol(def_id, tcx).unwrap();
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
            let len1 = span1.hi() - span1.lo();
            let len2 = span2.hi() - span2.lo();
            len1 == len2 && cmp.cmp_fn_sigs(*id1, *id2)
        });
    }
    let mut unlinked_foreign_fns: FxHashMap<_, Vec<_>> = FxHashMap::default();
    for def_id in hir_data.foreign_fns {
        let name = ir_util::def_id_to_value_symbol(def_id, tcx).unwrap();
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
        if let [i] = link_candidates[..] {
            classes.0[i].push(def_id);
        } else {
            unlinked_foreign_fns
                .entry(name)
                .or_default()
                .push((def_id, link_candidates));
        }
    }

    let mut equiv_statics: FxHashMap<_, EquivClasses<LocalDefId>> = FxHashMap::default();
    for def_id in hir_data.statics {
        let name = ir_util::def_id_to_value_symbol(def_id, tcx).unwrap();
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
    let mut unlinked_foreign_statics: FxHashMap<_, Vec<_>> = FxHashMap::default();
    for def_id in hir_data.foreign_statics {
        let name = ir_util::def_id_to_value_symbol(def_id, tcx).unwrap();
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
        if let [i] = link_candidates[..] {
            classes.0[i].push(def_id);
        } else {
            unlinked_foreign_statics
                .entry(name)
                .or_default()
                .push((def_id, link_candidates));
        }
    }

    ResolveResult {
        equiv_adts,
        equiv_fns,
        unlinked_foreign_fns,
        equiv_statics,
        unlinked_foreign_statics,
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
        let name = ir_util::def_id_to_ty_symbol(def_id1, self.tcx).unwrap();
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
                    let name1 = ir_util::def_id_to_ty_symbol(def_id1, self.tcx).unwrap();
                    let name2 = ir_util::def_id_to_ty_symbol(def_id2, self.tcx).unwrap();
                    return name1 == name2;
                }
                let Adt(adt_def2, args2) = ty2_kind else { return false };

                if args1.len() != args2.len() {
                    return false;
                }

                let def_id1 = adt_def1.did();
                let name1 = ir_util::def_id_to_ty_symbol(def_id1, self.tcx).unwrap();
                let def_id2 = adt_def2.did();
                let name2 = ir_util::def_id_to_ty_symbol(def_id2, self.tcx).unwrap();

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
                    let name1 = ir_util::def_id_to_ty_symbol(def_id1, self.tcx).unwrap();
                    let name2 = ir_util::def_id_to_ty_symbol(def_id2, self.tcx).unwrap();
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

// fn names_in_scc(
//     scc: &FxHashSet<LocalDefId>,
//     tcx: TyCtxt<'_>,
// ) -> (FxHashSet<Symbol>, FxHashMap<Symbol, LocalDefId>) {
//     let mut names = FxHashSet::default();
//     let mut name_to_def_id = FxHashMap::default();
//     for def_id in scc {
//         let name = ir_util::def_id_to_ty_symbol(*def_id, tcx).unwrap();
//         if is_unnamed(name.as_str()) {
//             continue;
//         }
//         names.insert(name);
//         name_to_def_id.insert(name, *def_id);
//     }
//     (names, name_to_def_id)
// }

fn is_unnamed(name: &str) -> bool {
    name.starts_with("C2RustUnnamed")
}

// let def_path = tcx.def_path(def_id.to_def_id());
// let mut path: Vec<_> = def_path
//     .data
//     .into_iter()
//     .map(|data| {
//         let DefPathData::TypeNs(name) = data.data else { panic!() };
//         name
//     })
//     .collect();
// let name = path.pop().unwrap();
// let module = ModulePath(path);
// let qualified_name = QualifiedName {
//     module: module.clone(),
//     name,
// };
// name_to_def_id.insert(qualified_name, def_id);
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// struct QualifiedName {
//     module: ModulePath,
//     name: Symbol,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// struct ModulePath(Vec<Symbol>);

struct EquivClasses<T>(IndexVec<EquivClassId, Vec<T>>);

impl<T> EquivClasses<T> {
    #[inline]
    fn new() -> Self {
        Self(IndexVec::new())
    }

    /// Inserts a value into the proper equivalence class.
    #[inline]
    fn insert<F: FnMut(&T, &T) -> bool>(&mut self, v: T, mut cmp: F) {
        for equiv_class in &mut self.0 {
            if cmp(&v, &equiv_class[0]) {
                equiv_class.push(v);
                return;
            }
        }
        self.0.push(vec![v]);
    }
}

// impl<T: Copy + Eq + std::hash::Hash> EquivClasses<T> {
//     fn get_id_map(&self) -> FxHashMap<T, EquivClassId> {
//         let mut map = FxHashMap::default();
//         for (id, equiv_class) in self.0.iter_enumerated() {
//             for &v in equiv_class {
//                 map.insert(v, id);
//             }
//         }
//         map
//     }
// }

rustc_index::newtype_index! {
    #[orderable]
    #[debug_format = "equiv{}"]
    /// A unique identifier for an equivalence class.
    pub struct EquivClassId {}
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
}

impl<'tcx> intravisit::Visitor<'tcx> for HirVisitor<'tcx> {
    type NestedFilter = nested_filter::OnlyBodies;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_item(&mut self, item: &'tcx hir::Item<'tcx>) -> Self::Result {
        let def_id = item.owner_id.def_id;
        if self.tcx.visibility(def_id).is_public() {
            match item.kind {
                hir::ItemKind::Fn { .. } => {
                    self.data.fns.push(def_id);
                }
                hir::ItemKind::Static { .. } => {
                    self.data.statics.push(def_id);
                }
                hir::ItemKind::Struct(_, _, _) | hir::ItemKind::Union(_, _, _) => {
                    self.data.adts.push(def_id);
                }
                hir::ItemKind::TyAlias(_, _, _) => {
                    self.data.tys.push(def_id);
                }
                _ => {}
            }
        }

        intravisit::walk_item(self, item)
    }

    fn visit_foreign_item(&mut self, item: &'tcx rustc_hir::ForeignItem<'tcx>) -> Self::Result {
        match item.kind {
            hir::ForeignItemKind::Fn(_, _, _) => {
                self.data.foreign_fns.push(item.owner_id.def_id);
            }
            hir::ForeignItemKind::Static(_, _, _) => {
                self.data.foreign_statics.push(item.owner_id.def_id);
            }
            hir::ForeignItemKind::Type => {
                self.data.foreign_tys.push(item.owner_id.def_id);
            }
        }

        intravisit::walk_foreign_item(self, item)
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
fn run_test(code1: &str, code2: &str, same: bool) {
    let code = format!("#![feature(extern_types)] mod a {{ {code1} }} mod b {{ {code2} }}");
    crate::compile_util::run_compiler(
        crate::compile_util::make_config(crate::compile_util::str_to_input(&code)),
        |tcx| {
            let res = resolve(tcx);
            for classes in res.equiv_adts.values() {
                if same {
                    assert_eq!(classes.0.len(), 1);
                } else {
                    for class in &classes.0 {
                        assert_eq!(class.len(), 1);
                    }
                }
            }
        },
    )
    .unwrap();
}

#[cfg(test)]
#[test]
fn test_simple() {
    let code1 = "
    pub struct s {
        x: i32,
    }
";
    let code2 = "
    pub struct s {
        x: i32,
    }
";
    run_test(code1, code2, true);
}

#[cfg(test)]
#[test]
fn test_simple_diff() {
    let code1 = "
    pub struct s {
        x: i32,
    }
";
    let code2 = "
    pub struct s {
        x: u32,
    }
";
    run_test(code1, code2, false);
}

#[cfg(test)]
#[test]
fn test_unnamed() {
    let code1 = "
    pub struct C2RustUnnamed {
        x: i32,
    }
    pub struct s {
        x: C2RustUnnamed,
        y: i32,
    }
";
    let code2 = "
    pub struct C2RustUnnamed_0 {
        x: i32,
    }
    pub struct s {
        x: C2RustUnnamed_0,
        y: i32,
    }
";
    run_test(code1, code2, true);
}

#[cfg(test)]
#[test]
fn test_unnamed_diff() {
    let code1 = "
    pub struct C2RustUnnamed {
        x: i32,
    }
    pub struct s {
        x: C2RustUnnamed,
        y: i32,
    }
";
    let code2 = "
    pub struct C2RustUnnamed_0 {
        x: i32,
    }
    pub struct s {
        x: C2RustUnnamed_0,
        y: u32,
    }
";
    run_test(code1, code2, false);
}

#[cfg(test)]
#[test]
fn test_recursion() {
    let code1 = "
    pub struct s {
        x: *mut s,
        y: i32,
    }
";
    let code2 = "
    pub struct s {
        x: *mut s,
        y: i32,
    }
";
    run_test(code1, code2, true);
}

#[cfg(test)]
#[test]
fn test_recursion_diff() {
    let code1 = "
    pub struct s {
        x: *mut s,
        y: i32,
    }
";
    let code2 = "
    pub struct s {
        x: *mut s,
        y: u32,
    }
";
    run_test(code1, code2, false);
}

#[cfg(test)]
#[test]
fn test_mutual_recursion() {
    let code1 = "
    pub struct s {
        x: *mut t,
        y: i32,
    }
    pub struct t {
        x: *mut s,
        y: i32,
    }
";
    let code2 = "
    pub struct s {
        x: *mut t,
        y: i32,
    }
    pub struct t {
        x: *mut s,
        y: i32,
    }
";
    run_test(code1, code2, true);
}

#[cfg(test)]
#[test]
fn test_mutual_recursion_diff() {
    let code1 = "
    pub struct s {
        x: *mut t,
        y: i32,
    }
    pub struct t {
        x: *mut s,
        y: i32,
    }
";
    let code2 = "
    pub struct s {
        x: *mut t,
        y: i32,
    }
    pub struct t {
        x: *mut s,
        y: u32,
    }
";
    run_test(code1, code2, false);
}

#[cfg(test)]
#[test]
fn test_unnamed_recursion() {
    let code1 = "
    pub struct C2RustUnnamed {
        x: *mut s,
    }
    pub struct s {
        x: C2RustUnnamed,
        y: i32,
    }
";
    let code2 = "
    pub struct C2RustUnnamed_0 {
        x: *mut s,
    }
    pub struct s {
        x: C2RustUnnamed_0,
        y: i32,
    }
";
    run_test(code1, code2, true);
}

#[cfg(test)]
#[test]
fn test_unnamed_recursion_diff() {
    let code1 = "
    pub struct C2RustUnnamed {
        x: *mut s,
        y: i32,
    }
    pub struct s {
        x: C2RustUnnamed,
        y: i32,
    }
";
    let code2 = "
    pub struct C2RustUnnamed_0 {
        x: *mut s,
        y: u32,
    }
    pub struct s {
        x: C2RustUnnamed_0,
        y: i32,
    }
";
    run_test(code1, code2, false);
}

#[cfg(test)]
#[test]
fn test_extern() {
    let code1 = r#"
    extern "C" {
        pub type s;
    }
    pub struct t {
        x: *mut s,
        y: i32,
    }
"#;
    let code2 = "
    pub struct s {
        y: i32,
    }
    pub struct t {
        x: *mut s,
        y: i32,
    }
";
    run_test(code1, code2, true);
}

#[cfg(test)]
#[test]
fn test_extern_diff() {
    let code1 = r#"
    extern "C" {
        pub type s;
    }
    pub struct t {
        x: *mut s,
        y: i32,
    }
"#;
    let code2 = "
    pub struct s {
        y: i32,
    }
    pub struct t {
        x: *mut s,
        y: u32,
    }
";
    run_test(code1, code2, false);
}

#[cfg(test)]
#[test]
fn test_extern_recursion() {
    let code1 = r#"
    extern "C" {
        pub type s;
    }
    pub struct t {
        x: *mut s,
        y: i32,
    }
"#;
    let code2 = "
    pub struct s {
        x: *mut t,
        y: i32,
    }
    pub struct t {
        x: *mut s,
        y: i32,
    }
";
    run_test(code1, code2, true);
}

#[cfg(test)]
#[test]
fn test_extern_recursion_diff() {
    let code1 = r#"
    extern "C" {
        pub type s;
    }
    pub struct t {
        x: *mut s,
        y: i32,
    }
"#;
    let code2 = "
    pub struct s {
        x: *mut t,
        y: i32,
    }
    pub struct t {
        x: *mut s,
        y: u32,
    }
";
    run_test(code1, code2, false);
}
