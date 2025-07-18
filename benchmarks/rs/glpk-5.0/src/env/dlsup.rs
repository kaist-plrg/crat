use ::libc;
extern "C" {
    fn _glp_put_err_msg(msg: *const libc::c_char);
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
}
pub unsafe extern "C" fn _glp_dlopen(
    mut module: *const libc::c_char,
) -> *mut libc::c_void {
    (module == module
        || {
            glp_assert_(
                b"module == module\0" as *const u8 as *const libc::c_char,
                b"env/dlsup.c\0" as *const u8 as *const libc::c_char,
                145 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_put_err_msg(
        b"Shared libraries not supported\0" as *const u8 as *const libc::c_char,
    );
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn _glp_dlsym(
    mut h: *mut libc::c_void,
    mut symbol: *const libc::c_char,
) -> *mut libc::c_void {
    (h != h
        || {
            glp_assert_(
                b"h != h\0" as *const u8 as *const libc::c_char,
                b"env/dlsup.c\0" as *const u8 as *const libc::c_char,
                152 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (symbol != symbol
        || {
            glp_assert_(
                b"symbol != symbol\0" as *const u8 as *const libc::c_char,
                b"env/dlsup.c\0" as *const u8 as *const libc::c_char,
                153 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn _glp_dlclose(mut h: *mut libc::c_void) {
    (h != h
        || {
            glp_assert_(
                b"h != h\0" as *const u8 as *const libc::c_char,
                b"env/dlsup.c\0" as *const u8 as *const libc::c_char,
                159 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
}
