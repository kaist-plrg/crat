use ::libc;
extern "C" {
    fn glp_printf(fmt: *const libc::c_char, _: ...);
}
pub unsafe extern "C" fn _glp_amd_control(mut Control: *mut libc::c_double) {
    let mut alpha: libc::c_double = 0.;
    let mut aggressive: libc::c_int = 0;
    if !Control.is_null() {
        alpha = *Control.offset(0 as libc::c_int as isize);
        aggressive = (*Control.offset(1 as libc::c_int as isize)
            != 0 as libc::c_int as libc::c_double) as libc::c_int;
    } else {
        alpha = 10.0f64;
        aggressive = 1 as libc::c_int;
    }
    glp_printf(
        b"\nAMD version %d.%d.%d, %s: approximate minimum degree ordering\n    dense row parameter: %g\n\0"
            as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
        b"May 31, 2007\0" as *const u8 as *const libc::c_char,
        alpha,
    );
    if alpha < 0 as libc::c_int as libc::c_double {
        glp_printf(
            b"    no rows treated as dense\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        glp_printf(
            b"    (rows with more than max (%g * sqrt (n), 16) entries are\n    considered \"dense\", and placed last in output permutation)\n\0"
                as *const u8 as *const libc::c_char,
            alpha,
        );
    }
    if aggressive != 0 {
        glp_printf(
            b"    aggressive absorption:  yes\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        glp_printf(
            b"    aggressive absorption:  no\n\0" as *const u8 as *const libc::c_char,
        );
    }
    glp_printf(
        b"    size of AMD integer: %d\n\n\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
}
