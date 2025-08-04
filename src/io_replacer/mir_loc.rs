use rustc_abi::FieldIdx;
use rustc_middle::mir::Local;
use rustc_span::def_id::LocalDefId;

use crate::ir_util;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum MirLoc {
    Stdin,
    Stdout,
    Stderr,
    Var(LocalDefId, Local),
    Field(LocalDefId, FieldIdx),
}

impl std::fmt::Debug for MirLoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stdin => write!(f, "stdin"),
            Self::Stdout => write!(f, "stdout"),
            Self::Stderr => write!(f, "stderr"),
            Self::Var(def_id, local) => {
                ir_util::fmt_def_id(f, *def_id)?;
                write!(f, ":{}", local.index())
            }
            Self::Field(def_id, field) => {
                ir_util::fmt_def_id(f, *def_id)?;
                write!(f, ".{}", field.index())
            }
        }
    }
}
