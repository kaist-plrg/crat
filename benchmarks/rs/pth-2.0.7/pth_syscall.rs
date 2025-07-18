use ::libc;
pub static mut pth_syscall_soft: libc::c_int = 0 as libc::c_int;
pub static mut pth_syscall_hard: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn __pth_syscall_init() {}
pub unsafe extern "C" fn __pth_syscall_kill() {}
