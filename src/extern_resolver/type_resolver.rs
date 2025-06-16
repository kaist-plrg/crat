use rustc_abi::VariantIdx;
use rustc_hash::{FxHashMap, FxHashSet};
use rustc_hir::{self as hir};
use rustc_middle::ty::{self, List, TyCtxt, TypeSuperVisitable, TypeVisitor};
use rustc_span::{Symbol, def_id::LocalDefId};

use crate::{
    compile_util::Pass,
    graph_util::{self, SccId},
    ir_util,
};

pub struct TypeResolver;

impl Pass for TypeResolver {
    type Out = ();

    fn run(&self, tcx: TyCtxt<'_>) -> Self::Out {
        let mut dependencies: FxHashMap<_, _> = FxHashMap::default();

        for item_id in tcx.hir_free_items() {
            let item = tcx.hir_item(item_id);
            if !matches!(
                item.kind,
                hir::ItemKind::Struct(_, _, _) | hir::ItemKind::Union(_, _, _)
            ) {
                continue;
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

            let def_id = item_id.owner_id.def_id;
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
        let ctx = CmpCtx { tcx, sccs: &sccs };

        let mut equiv_classes: Vec<Vec<SccId>> = vec![];
        for scc in sccs.post_order() {
            let mut found = false;
            for equiv_class in &mut equiv_classes {
                let rep_scc = equiv_class[0];
                if cmp_sccs(rep_scc, scc, ctx) {
                    equiv_class.push(scc);
                    found = true;
                    break;
                }
            }
            if !found {
                equiv_classes.push(vec![scc]);
            }
        }

        for equiv_class in equiv_classes {
            for scc_id in equiv_class {
                let scc = &sccs.sccs[scc_id];
                println!("{scc:?}");
            }
            println!();
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

#[derive(Clone, Copy)]
struct CmpCtx<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    sccs: &'a graph_util::Sccs<LocalDefId>,
}

fn cmp_sccs(scc_id1: SccId, scc_id2: SccId, ctx: CmpCtx<'_, '_>) -> bool {
    let scc1 = &ctx.sccs.sccs[scc_id1];
    let (names1, name_to_id1) = names_in_scc(scc1, ctx.tcx);
    let scc2 = &ctx.sccs.sccs[scc_id2];
    let (names2, name_to_id2) = names_in_scc(scc2, ctx.tcx);
    if names1 != names2 {
        return false;
    }

    for (name, def_id1) in name_to_id1 {
        let def_id2 = name_to_id2[&name];
        if !cmp_adts(def_id1, def_id2, &names1, ctx) {
            return false;
        }
    }

    true
}

fn cmp_adts<'tcx>(
    def_id1: LocalDefId,
    def_id2: LocalDefId,
    names: &FxHashSet<Symbol>,
    ctx: CmpCtx<'_, 'tcx>,
) -> bool {
    let adt1 = ctx.tcx.adt_def(def_id1);
    let adt2 = ctx.tcx.adt_def(def_id2);
    if adt1.adt_kind() != adt2.adt_kind() {
        return false;
    }

    let variant1 = adt1.variant(VariantIdx::ZERO);
    let variant2 = adt2.variant(VariantIdx::ZERO);
    if variant1.fields.len() != variant2.fields.len() {
        return false;
    }

    for (fd1, fd2) in variant1.fields.iter().zip(variant2.fields.iter()) {
        let ty1 = fd1.ty(ctx.tcx, List::empty());
        let ty2 = fd2.ty(ctx.tcx, List::empty());
        if !cmp_tys(ty1, ty2, names, ctx) {
            return false;
        }
    }

    true
}

fn cmp_tys<'tcx>(
    ty1: ty::Ty<'tcx>,
    ty2: ty::Ty<'tcx>,
    names: &FxHashSet<Symbol>,
    ctx: CmpCtx<'_, 'tcx>,
) -> bool {
    use ty::*;
    let ty1 = ty1.kind();
    let ty2 = ty2.kind();
    match ty1 {
        Bool | Char | Int(_) | Uint(_) | Float(_) => ty1 == ty2,
        Adt(adt_def1, args1) => {
            let Adt(adt_def2, args2) = ty2 else { return false };

            if args1.len() != args2.len() {
                return false;
            }

            let def_id1 = adt_def1.did();
            let name1 = ir_util::def_id_to_ty_symbol(def_id1, ctx.tcx).unwrap();
            let def_id2 = adt_def2.did();
            let name2 = ir_util::def_id_to_ty_symbol(def_id2, ctx.tcx).unwrap();

            if args1.is_empty() {
                match (def_id1.as_local(), def_id2.as_local()) {
                    (Some(def_id1), Some(def_id2)) => {
                        match (is_unnamed(name1.as_str()), is_unnamed(name2.as_str())) {
                            (true, true) => cmp_adts(def_id1, def_id2, names, ctx),
                            (false, false) => {
                                if name1 != name2 {
                                    false
                                } else if names.contains(&name1) {
                                    true
                                } else {
                                    let scc1 = ctx.sccs.indices[&def_id1];
                                    let scc2 = ctx.sccs.indices[&def_id2];
                                    cmp_sccs(scc1, scc2, ctx)
                                }
                            }
                            _ => false,
                        }
                    }
                    (None, None) => {
                        assert_eq!(name1.as_str(), "c_void");
                        assert_eq!(name2.as_str(), "c_void");
                        true
                    }
                    _ => false,
                }
            } else {
                assert_eq!(args1.len(), 1);
                assert_eq!(name1.as_str(), "Option");
                assert_eq!(name2.as_str(), "Option");
                cmp_tys(args1[0].expect_ty(), args2[0].expect_ty(), names, ctx)
            }
        }
        Foreign(def_id1) => {
            let Foreign(def_id2) = ty2 else { return false };
            let name1 = ir_util::def_id_to_ty_symbol(def_id1, ctx.tcx).unwrap();
            let name2 = ir_util::def_id_to_ty_symbol(def_id2, ctx.tcx).unwrap();
            name1 == name2
        }
        Array(ty1, len1) => {
            let Array(ty2, len2) = ty2 else { return false };
            len1 == len2 && cmp_tys(*ty1, *ty2, names, ctx)
        }
        RawPtr(ty1, m1) => {
            let RawPtr(ty2, m2) = ty2 else { return false };
            m1 == m2 && cmp_tys(*ty1, *ty2, names, ctx)
        }
        FnPtr(sig1, header1) => {
            let FnPtr(sig2, header2) = ty2 else { return false };
            if header1 != header2 {
                return false;
            }
            let sig1 = sig1.skip_binder();
            let sig2 = sig2.skip_binder();
            if !cmp_tys(sig1.output(), sig2.output(), names, ctx) {
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
                .all(|(ty1, ty2)| cmp_tys(*ty1, *ty2, names, ctx))
        }
        Tuple(tys1) => {
            let Tuple(tys2) = ty2 else { return false };
            // Only appears as (), being the return type of a function.
            assert_eq!(tys1.len(), 0);
            assert_eq!(tys2.len(), 0);
            true
        }
        _ => panic!("{ty1:?}"),
    }
}

fn is_unnamed(name: &str) -> bool {
    name.starts_with("C2RustUnnamed")
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// struct QualifiedName {
//     module: ModulePath,
//     name: Symbol,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// struct ModulePath(Vec<Symbol>);

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
