use ::libc;
extern "C" {
    fn rpl_getopt_internal(
        ___argc: libc::c_int,
        ___argv: *mut *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const rpl_option,
        __longind: *mut libc::c_int,
        __long_only: libc::c_int,
        __posixly_correct: libc::c_int,
    ) -> libc::c_int;
    fn _getopt_internal_r(
        ___argc: libc::c_int,
        ___argv: *mut *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const rpl_option,
        __longind: *mut libc::c_int,
        __long_only: libc::c_int,
        __data: *mut _getopt_data,
        __posixly_correct: libc::c_int,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpl_option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type __ord = libc::c_uint;
pub const RETURN_IN_ORDER: __ord = 2;
pub const PERMUTE: __ord = 1;
pub const REQUIRE_ORDER: __ord = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _getopt_data {
    pub rpl_optind: libc::c_int,
    pub rpl_opterr: libc::c_int,
    pub rpl_optopt: libc::c_int,
    pub rpl_optarg: *mut libc::c_char,
    pub __initialized: libc::c_int,
    pub __nextchar: *mut libc::c_char,
    pub __ordering: __ord,
    pub __first_nonopt: libc::c_int,
    pub __last_nonopt: libc::c_int,
}
pub unsafe extern "C" fn rpl_getopt_long(
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
    mut options: *const libc::c_char,
    mut long_options: *const rpl_option,
    mut opt_index: *mut libc::c_int,
) -> libc::c_int {
    return rpl_getopt_internal(
        argc,
        argv as *mut *mut libc::c_char,
        options,
        long_options,
        opt_index,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn _getopt_long_r(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut options: *const libc::c_char,
    mut long_options: *const rpl_option,
    mut opt_index: *mut libc::c_int,
    mut d: *mut _getopt_data,
) -> libc::c_int {
    return _getopt_internal_r(
        argc,
        argv,
        options,
        long_options,
        opt_index,
        0 as libc::c_int,
        d,
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn rpl_getopt_long_only(
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
    mut options: *const libc::c_char,
    mut long_options: *const rpl_option,
    mut opt_index: *mut libc::c_int,
) -> libc::c_int {
    return rpl_getopt_internal(
        argc,
        argv as *mut *mut libc::c_char,
        options,
        long_options,
        opt_index,
        1 as libc::c_int,
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn _getopt_long_only_r(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut options: *const libc::c_char,
    mut long_options: *const rpl_option,
    mut opt_index: *mut libc::c_int,
    mut d: *mut _getopt_data,
) -> libc::c_int {
    return _getopt_internal_r(
        argc,
        argv,
        options,
        long_options,
        opt_index,
        1 as libc::c_int,
        d,
        0 as libc::c_int,
    );
}
