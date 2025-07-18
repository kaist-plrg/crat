use ::libc;
extern "C" {
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __mode_t = libc::c_uint;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct permission_context {
    pub mode: mode_t,
}
pub unsafe extern "C" fn chmod_or_fchmod(
    mut name: *const libc::c_char,
    mut desc: libc::c_int,
    mut mode: mode_t,
) -> libc::c_int {
    if 1 as libc::c_int != 0 && desc != -(1 as libc::c_int) {
        return fchmod(desc, mode)
    } else {
        return chmod(name, mode)
    };
}
pub unsafe extern "C" fn set_permissions(
    mut ctx: *mut permission_context,
    mut name: *const libc::c_char,
    mut desc: libc::c_int,
) -> libc::c_int {
    let mut acls_set: bool = 0 as libc::c_int != 0;
    let mut early_chmod: bool = false;
    let mut must_chmod: bool = 0 as libc::c_int != 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    early_chmod = 1 as libc::c_int != 0;
    if early_chmod {
        ret = chmod_or_fchmod(name, desc, (*ctx).mode);
        if ret != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    if must_chmod as libc::c_int != 0 && !early_chmod {
        let mut saved_errno: libc::c_int = if ret != 0 {
            *__errno_location()
        } else {
            0 as libc::c_int
        };
        ret = chmod_or_fchmod(name, desc, (*ctx).mode);
        if saved_errno != 0 {
            *__errno_location() = saved_errno;
            ret = -(1 as libc::c_int);
        }
    }
    return ret;
}
