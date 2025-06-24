use rustc_middle::ty::TyCtxt;

pub fn type_check(tcx: TyCtxt<'_>) {
    let () = tcx.analysis(());
}
