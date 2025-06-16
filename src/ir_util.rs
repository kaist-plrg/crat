//! utils for working with HIR and MIR

use rustc_hir::definitions::DefPathData;
use rustc_middle::{query::IntoQueryParam, ty::TyCtxt};
use rustc_span::{Symbol, def_id::DefId};

#[inline]
pub fn def_id_to_value_symbol(id: impl IntoQueryParam<DefId>, tcx: TyCtxt<'_>) -> Option<Symbol> {
    let key = tcx.def_key(id);
    let DefPathData::ValueNs(name) = key.disambiguated_data.data else { return None };
    Some(name)
}

#[inline]
pub fn def_id_to_ty_symbol(id: impl IntoQueryParam<DefId>, tcx: TyCtxt<'_>) -> Option<Symbol> {
    let key = tcx.def_key(id);
    let DefPathData::TypeNs(name) = key.disambiguated_data.data else { return None };
    Some(name)
}
