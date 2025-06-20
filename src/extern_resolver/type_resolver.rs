use rustc_abi::VariantIdx;
use rustc_hash::{FxHashMap, FxHashSet};
use rustc_hir::{self as hir, intravisit};
use rustc_index::IndexVec;
use rustc_middle::{
    hir::nested_filter,
    ty::{self, List, TyCtxt, TypeSuperVisitable, TypeVisitor},
};
use rustc_span::{Symbol, def_id::LocalDefId};
use typed_arena::Arena;

use crate::{
    compile_util::Pass,
    disjoint_set::DisjointSets,
    graph_util::{self, SccId},
    ir_util,
};

pub struct TypeResolver;

impl Pass for TypeResolver {
    type Out = ();

    fn run(&self, tcx: TyCtxt<'_>) -> Self::Out {
        let result = resolve(tcx);

        let mut num = 0;
        for (name, classes) in &result.equiv_adts {
            if classes.0.len() > 1 {
                println!("{name}");
                for (id, def_ids) in classes.0.iter_enumerated() {
                    println!("  {id:?}");
                    for def_id in def_ids {
                        println!("    {def_id:?}");
                    }
                }
            } else {
                num += classes.0.iter().map(|v| v.len()).sum::<usize>();
            }
        }
        println!("{} {num}", result.equiv_adts.len());

        // let mut fns: FxHashMap<_, EquivClasses<LocalDefId>> = FxHashMap::default();
        // for def_id in hir_data.fns {
        //     let name = ir_util::def_id_to_value_symbol(def_id, tcx).unwrap();
        //     let classes = fns.entry(name).or_insert_with(EquivClasses::new);
        //     classes.insert(def_id, |id1, id2| {
        //         let sig1 = tcx.fn_sig(*id1).skip_binder().skip_binder();
        //         let sig2 = tcx.fn_sig(*id2).skip_binder().skip_binder();
        //         let inputs1 = sig1.inputs();
        //         let inputs2 = sig2.inputs();
        //         if sig1.c_variadic != sig2.c_variadic
        //             || sig1.safety != sig2.safety
        //             || sig1.abi != sig2.abi
        //             || inputs1.len() != inputs2.len()
        //         {
        //             return false;
        //         }
        //         for (ty1, ty2) in inputs1.iter().zip(inputs2) {
        //             if !cmp.cmp_tys(*ty1, *ty2, None, &FxHashSet::default()) {
        //                 return false;
        //             }
        //         }
        //         cmp.cmp_tys(sig1.output(), sig2.output(), None, &FxHashSet::default())
        //     });
        // }

        // let mut statics: FxHashMap<_, EquivClasses<LocalDefId>> = FxHashMap::default();
        // for def_id in hir_data.statics {
        //     let name = ir_util::def_id_to_value_symbol(def_id, tcx).unwrap();
        //     let classes = statics.entry(name).or_insert_with(EquivClasses::new);
        //     classes.insert(def_id, |id1, id2| {
        //         let ty1 = tcx.type_of(*id1).skip_binder();
        //         let ty2 = tcx.type_of(*id2).skip_binder();
        //         if !cmp.cmp_tys(ty1, ty2, None, &FxHashSet::default()) {
        //             return false;
        //         }
        //         let init1 = tcx.eval_static_initializer(*id1);
        //         let init2 = tcx.eval_static_initializer(*id2);
        //         init1 == init2
        //     });
        // }

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

        // for equiv_class in scc_classes.0 {
        //     for scc_id in equiv_class {
        //         let scc = &sccs.sccs[scc_id];
        //         println!("{scc:?}");
        //     }
        //     println!();
        // }

        // for unnamed in cmp.equiv_unnameds.iter() {
        //     let rep = cmp.equiv_unnameds.find_set(unnamed);
        //     println!("{rep:?} = {unnamed:?}");
        // }

        // for (name, equiv_classes) in fns {
        //     println!("{name}");
        //     for equiv_class in equiv_classes.0 {
        //         for def_id in equiv_class {
        //             println!("  {def_id:?}");
        //         }
        //         println!();
        //     }
        //     println!();
        // }

        // for (name, equiv_classes) in tys {
        //     println!("{name}");
        //     for equiv_class in equiv_classes.0 {
        //         for def_id in equiv_class {
        //             println!("  {def_id:?}");
        //         }
        //         println!();
        //     }
        //     println!();
        // }

        // for (name, equiv_classes) in statics {
        //     println!("{name}");
        //     for equiv_class in equiv_classes.0 {
        //         for def_id in equiv_class {
        //             println!("  {def_id:?}");
        //         }
        //         println!();
        //     }
        //     println!();
        // }
    }
}

struct ResolveResult {
    equiv_adts: FxHashMap<Symbol, EquivClasses<LocalDefId>>,
}

fn resolve(tcx: TyCtxt<'_>) -> ResolveResult {
    let mut visitor = HirVisitor::new(tcx);
    tcx.hir_visit_all_item_likes_in_crate(&mut visitor);
    let hir_data = visitor.data;

    let mut dependencies: FxHashMap<_, _> = FxHashMap::default();
    for def_id in hir_data.adts.iter().copied() {
        let adt_def = tcx.adt_def(def_id);
        let variant = adt_def.variant(VariantIdx::ZERO);
        let mut visitor = AdtVisitor::default();
        for fd in &variant.fields {
            let ty = fd.ty(tcx, List::empty());
            visitor.visit_ty(ty);
        }
        dependencies.insert(def_id, visitor.adts);
    }

    let sccs = graph_util::sccs_copied(&dependencies);
    let area = Arena::new();
    let equiv_sccs = DisjointSets::new(&area);
    let arena = Arena::new();
    let equiv_unnameds = DisjointSets::new(&arena);
    let mut cmp = TypeComparator {
        tcx,
        sccs: &sccs,
        equiv_sccs,
        equiv_unnameds,
        finished: false,
    };

    let mut scc_classes: EquivClasses<SccId> = EquivClasses::new();
    for scc_id in sccs.post_order() {
        let scc = &sccs.sccs[scc_id];
        if scc.iter().all(|def_id| {
            let name = ir_util::def_id_to_ty_symbol(*def_id, tcx).unwrap();
            is_unnamed(name.as_str())
        }) {
            continue;
        }
        scc_classes.insert(scc_id, |id1, id2| cmp.cmp_sccs(*id1, *id2));
    }
    cmp.finished = true;

    let scc_id_to_equiv_class = scc_classes.get_id_map();

    let mut equiv_adts: FxHashMap<_, EquivClasses<LocalDefId>> = FxHashMap::default();
    for def_id in &hir_data.adts {
        let name = ir_util::def_id_to_ty_symbol(*def_id, tcx).unwrap();
        if is_unnamed(name.as_str()) {
            continue;
        }
        let classes = equiv_adts.entry(name).or_insert_with(EquivClasses::new);
        classes.insert(*def_id, |id1, id2| {
            let scc_id1 = sccs.indices[id1];
            let scc_id2 = sccs.indices[id2];
            let equiv_class1 = scc_id_to_equiv_class[&scc_id1];
            let equiv_class2 = scc_id_to_equiv_class[&scc_id2];
            equiv_class1 == equiv_class2
        });
    }

    ResolveResult { equiv_adts }
}

struct TypeComparator<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    sccs: &'a graph_util::Sccs<LocalDefId>,
    equiv_sccs: DisjointSets<'a, SccId>,
    equiv_unnameds: DisjointSets<'a, LocalDefId>,
    finished: bool,
}

impl<'tcx> TypeComparator<'_, 'tcx> {
    fn cmp_sccs(&mut self, scc_id1: SccId, scc_id2: SccId) -> bool {
        if let Some(b) = self.equiv_sccs.equiv(scc_id1, scc_id2) {
            if self.finished || b {
                return b;
            }
        }

        let scc1 = &self.sccs.sccs[scc_id1];
        let (names1, name_to_id1) = names_in_scc(scc1, self.tcx);
        let scc2 = &self.sccs.sccs[scc_id2];
        let (names2, name_to_id2) = names_in_scc(scc2, self.tcx);
        if names1 != names2 {
            return false;
        }

        let mut unnameds = FxHashSet::default();

        for (name, def_id1) in name_to_id1 {
            let def_id2 = name_to_id2[&name];
            if !self.cmp_adts(def_id1, def_id2, Some(&mut unnameds), &names1) {
                return false;
            }
        }

        self.equiv_sccs.union(scc_id1, scc_id2);
        for (def_id1, def_id2) in unnameds {
            self.equiv_unnameds.union(def_id1, def_id2);
        }

        true
    }

    fn cmp_adts(
        &mut self,
        def_id1: LocalDefId,
        def_id2: LocalDefId,
        mut unnameds: Option<&mut FxHashSet<(LocalDefId, LocalDefId)>>,
        names: &FxHashSet<Symbol>,
    ) -> bool {
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
            if !self.cmp_tys(ty1, ty2, unnameds.as_deref_mut(), names) {
                return false;
            }
        }

        true
    }

    fn cmp_tys(
        &mut self,
        ty1: ty::Ty<'tcx>,
        ty2: ty::Ty<'tcx>,
        mut unnameds: Option<&mut FxHashSet<(LocalDefId, LocalDefId)>>,
        names: &FxHashSet<Symbol>,
    ) -> bool {
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
                                let res =
                                    self.cmp_adts(def_id1, def_id2, unnameds.as_deref_mut(), names);
                                if let Some(unnameds) = unnameds
                                    && res
                                {
                                    unnameds.insert((def_id1, def_id2));
                                }
                                res
                            }
                            (false, false) => {
                                if name1 != name2 {
                                    false
                                } else if names.contains(&name1) {
                                    true
                                } else {
                                    let scc1 = self.sccs.indices[&def_id1];
                                    let scc2 = self.sccs.indices[&def_id2];
                                    self.cmp_sccs(scc1, scc2)
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
                                    (Type(ty1), Type(ty2)) => {
                                        self.cmp_tys(ty1, ty2, unnameds.as_deref_mut(), names)
                                    }
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
                Adt(_, _) => self.cmp_tys(ty2, ty1, unnameds, names),
                Foreign(def_id2) => {
                    let name1 = ir_util::def_id_to_ty_symbol(def_id1, self.tcx).unwrap();
                    let name2 = ir_util::def_id_to_ty_symbol(def_id2, self.tcx).unwrap();
                    name1 == name2
                }
                _ => false,
            },
            Array(ty1, len1) => {
                let Array(ty2, len2) = ty2_kind else { return false };
                len1 == len2 && self.cmp_tys(*ty1, *ty2, unnameds, names)
            }
            RawPtr(ty1, m1) => {
                let RawPtr(ty2, m2) = ty2_kind else { return false };
                m1 == m2 && self.cmp_tys(*ty1, *ty2, unnameds, names)
            }
            FnPtr(sig1, header1) => {
                let FnPtr(sig2, header2) = ty2_kind else { return false };
                if header1 != header2 {
                    return false;
                }
                let sig1 = sig1.skip_binder();
                let sig2 = sig2.skip_binder();
                if !self.cmp_tys(sig1.output(), sig2.output(), unnameds.as_deref_mut(), names) {
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
                    .all(|(ty1, ty2)| self.cmp_tys(*ty1, *ty2, unnameds.as_deref_mut(), names))
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
}

fn names_in_scc(
    scc: &FxHashSet<LocalDefId>,
    tcx: TyCtxt<'_>,
) -> (FxHashSet<Symbol>, FxHashMap<Symbol, LocalDefId>) {
    let mut names = FxHashSet::default();
    let mut name_to_def_id = FxHashMap::default();
    for def_id in scc {
        let name = ir_util::def_id_to_ty_symbol(*def_id, tcx).unwrap();
        if is_unnamed(name.as_str()) {
            continue;
        }
        names.insert(name);
        name_to_def_id.insert(name, *def_id);
    }
    (names, name_to_def_id)
}

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

impl<T: Copy + Eq + std::hash::Hash> EquivClasses<T> {
    fn get_id_map(&self) -> FxHashMap<T, EquivClassId> {
        let mut map = FxHashMap::default();
        for (id, equiv_class) in self.0.iter_enumerated() {
            for &v in equiv_class {
                map.insert(v, id);
            }
        }
        map
    }
}

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
