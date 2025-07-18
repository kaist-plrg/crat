use ::libc;
extern "C" {
    fn rpl_free(ptr: *mut libc::c_void);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fchdir(__fd: libc::c_int) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn chdir_long(dir: *mut libc::c_char) -> libc::c_int;
    fn fd_safer_flag(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn open_safer(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_cwd {
    pub desc: libc::c_int,
    pub name: *mut libc::c_char,
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn save_cwd(mut cwd: *mut saved_cwd) -> libc::c_int {
    (*cwd).name = 0 as *mut libc::c_char;
    (*cwd)
        .desc = open_safer(
        b".\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int | 0o2000000 as libc::c_int,
    );
    if 1 as libc::c_int == 0 {
        (*cwd).desc = fd_safer_flag((*cwd).desc, 0o2000000 as libc::c_int);
    }
    if (*cwd).desc < 0 as libc::c_int {
        (*cwd).name = getcwd(0 as *mut libc::c_char, 0 as libc::c_int as size_t);
        return if !((*cwd).name).is_null() {
            0 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn restore_cwd(mut cwd: *const saved_cwd) -> libc::c_int {
    if 0 as libc::c_int <= (*cwd).desc {
        return fchdir((*cwd).desc)
    } else {
        return chdir_long((*cwd).name)
    };
}
pub unsafe extern "C" fn free_cwd(mut cwd: *mut saved_cwd) {
    if (*cwd).desc >= 0 as libc::c_int {
        close((*cwd).desc);
    }
    rpl_free((*cwd).name as *mut libc::c_void);
}
