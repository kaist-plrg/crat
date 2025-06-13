use rustc_middle::ty::TyCtxt;

use crate::compile_util::Pass;

pub struct TypeChecker;

impl Pass for TypeChecker {
    type Out = ();

    fn run(&self, tcx: TyCtxt<'_>) -> Self::Out {
        let () = tcx.analysis(());
    }
}
