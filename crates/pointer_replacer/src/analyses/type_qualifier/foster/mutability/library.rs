use rustc_hir::{def_id::DefId, definitions::DefPathData};
use rustc_middle::{
    mir::{HasLocalDecls, Operand, Place},
    ty::TyCtxt,
};
use rustc_span::source_map::Spanned;

use super::{EnsureNoDeref, MutCtxt, Mutability, conservative_call, place_vars};
use crate::analyses::type_qualifier::foster::{
    StructFields, Var,
    constraint_system::{BooleanSystem, ConstraintSystem},
};

pub fn library_call<'tcx>(
    destination: &Place<'tcx>,
    args: &[Spanned<Operand<'tcx>>],
    callee: DefId,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut BooleanSystem<Mutability>,
    tcx: TyCtxt<'tcx>,
) {
    let def_path = tcx.def_path(callee);

    if let [DefPathData::TypeNs(cmp), _, DefPathData::ValueNs(eq), ..] = &def_path
        .data
        .iter()
        .map(|data| data.data)
        .collect::<smallvec::SmallVec<[_; 4]>>()[..]
    {
        if cmp.as_str() == "cmp" && eq.as_str() == "eq" {
            // unconstrained call
            return;
        }
    }

    if let [
        DefPathData::TypeNs(option),
        _,
        DefPathData::ValueNs(func),
        ..,
    ] = &def_path
        .data
        .iter()
        .map(|data| data.data)
        .collect::<smallvec::SmallVec<[_; 4]>>()[..]
    {
        if option.as_str() == "option" {
            // unconstrained call
            match func.as_str() {
                "is_some" | "expect" | "is_none" => return,
                _ => {} // fall
            }
        }
    }

    if let [
        DefPathData::TypeNs(slice),
        _,
        DefPathData::ValueNs(as_mut_ptr),
        ..,
    ] = &def_path
        .data
        .iter()
        .map(|data| data.data)
        .collect::<smallvec::SmallVec<[_; 4]>>()[..]
    {
        if slice.as_str() == "slice" && as_mut_ptr.as_str() == "as_mut_ptr" {
            return call_as_mut_ptr(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            );
        }
    }

    let def_path_str = tcx.def_path_str(callee);

    if def_path_str.ends_with("::ptr_offset_from") {
        return call_offset_from(
            destination,
            args,
            local_decls,
            locals,
            struct_fields,
            database,
        );
    }

    // if it is a library call in core::ptr
    if def_path
        .data
        .get(0)
        .map(|d| match d.data {
            rustc_hir::definitions::DefPathData::TypeNs(s) if s.as_str() == "ptr" => true,
            _ => false,
        })
        .is_some()
    {
        // if it is core::ptr::<..>::..
        if let Some(d) = def_path.data.get(3) {
            if let rustc_hir::definitions::DefPathData::ValueNs(s) = d.data {
                match s.as_str() {
                    "is_null" => {
                        return call_is_null(
                            destination,
                            args,
                            local_decls,
                            locals,
                            struct_fields,
                            database,
                        );
                    }
                    "offset" => {
                        return call_offset(
                            destination,
                            args,
                            local_decls,
                            locals,
                            struct_fields,
                            database,
                        );
                    }
                    "offset_from" => {
                        return call_offset_from(
                            destination,
                            args,
                            local_decls,
                            locals,
                            struct_fields,
                            database,
                        );
                    }
                    "addr" => {
                        // no constraint
                        return;
                    }
                    _ => {}
                }
            }
        }

        conservative_call(
            destination,
            args,
            local_decls,
            locals,
            struct_fields,
            database,
        );
    }
}

fn call_is_null<'tcx>(
    destination: &Place<'tcx>,
    args: &[Spanned<Operand<'tcx>>],
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut BooleanSystem<Mutability>,
) {
    let dest_vars =
        place_vars::<MutCtxt>(destination, local_decls, locals, struct_fields, database);
    assert!(dest_vars.is_empty());
    // no constraint on args
    let _ = args;
}

fn call_offset<'tcx>(
    destination: &Place<'tcx>,
    args: &[Spanned<Operand<'tcx>>],
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut BooleanSystem<Mutability>,
) {
    let dest_vars =
        place_vars::<MutCtxt>(destination, local_decls, locals, struct_fields, database);
    if let Some(arg) = args[0].node.place() {
        let arg_vars =
            place_vars::<EnsureNoDeref>(&arg, local_decls, locals, struct_fields, &mut ());
        let mut dest_arg = dest_vars.zip(arg_vars);

        if let Some((dest, arg)) = dest_arg.next() {
            database.guard(dest, arg)
        }
        for (dest, arg) in dest_arg {
            database.guard(arg, dest);
            database.guard(dest, arg);
        }
    }
}

fn call_offset_from<'tcx>(
    destination: &Place<'tcx>,
    args: &[Spanned<Operand<'tcx>>],
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut BooleanSystem<Mutability>,
) {
    let dest_vars =
        place_vars::<MutCtxt>(destination, local_decls, locals, struct_fields, database);
    assert!(dest_vars.is_empty());
    // no constraint on args
    let _ = args;
}

fn call_as_mut_ptr<'tcx>(
    destination: &Place<'tcx>,
    args: &[Spanned<Operand<'tcx>>],
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut BooleanSystem<Mutability>,
) {
    let dest_vars =
        place_vars::<MutCtxt>(destination, local_decls, locals, struct_fields, database);
    if let Some(arg) = args[0].node.place() {
        let arg_vars =
            place_vars::<EnsureNoDeref>(&arg, local_decls, locals, struct_fields, &mut ());
        let mut dest_arg = dest_vars.zip(arg_vars);

        if let Some((dest, arg)) = dest_arg.next() {
            database.guard(dest, arg)
        }
        for (dest, arg) in dest_arg {
            database.guard(arg, dest);
            database.guard(dest, arg);
        }
    }
}
