use ::libc;
pub type error_t = libc::c_int;
pub static mut argp_err_exit_status: error_t = 64 as libc::c_int;
