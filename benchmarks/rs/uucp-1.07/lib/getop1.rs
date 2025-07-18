use ::libc;
extern "C" {
    fn _getopt_internal(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
        longopts: *const option,
        longind: *mut libc::c_int,
        long_only: libc::c_int,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub unsafe extern "C" fn getopt_long(
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
    mut options: *const libc::c_char,
    mut long_options: *const option,
    mut opt_index: *mut libc::c_int,
) -> libc::c_int {
    return _getopt_internal(
        argc,
        argv,
        options,
        long_options,
        opt_index,
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn getopt_long_only(
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
    mut options: *const libc::c_char,
    mut long_options: *const option,
    mut opt_index: *mut libc::c_int,
) -> libc::c_int {
    return _getopt_internal(
        argc,
        argv,
        options,
        long_options,
        opt_index,
        1 as libc::c_int,
    );
}
