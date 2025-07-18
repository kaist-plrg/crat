use ::libc;
pub unsafe extern "C" fn acl_errno_valid(mut errnum: libc::c_int) -> bool {
    match errnum {
        16 => return 0 as libc::c_int != 0,
        22 => return 0 as libc::c_int != 0,
        38 => return 0 as libc::c_int != 0,
        95 => return 0 as libc::c_int != 0,
        _ => return 1 as libc::c_int != 0,
    };
}
