use ::libc;
pub unsafe extern "C" fn _glp_amd_defaults(mut Control: *mut libc::c_double) {
    let mut i: libc::c_int = 0;
    if !Control.is_null() {
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            *Control.offset(i as isize) = 0 as libc::c_int as libc::c_double;
            i += 1;
            i;
        }
        *Control.offset(0 as libc::c_int as isize) = 10.0f64;
        *Control.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    }
}
