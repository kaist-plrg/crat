use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const libc::c_char,
    pub lineno: libc::c_ulong,
    pub offset: libc::c_ulong,
}
pub unsafe extern "C" fn guile_gmake_setup(mut flocp: *const floc) -> libc::c_int {
    return 1 as libc::c_int;
}
