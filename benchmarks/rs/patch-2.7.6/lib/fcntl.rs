use ::libc;
extern "C" {
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
static mut have_dupfd_cloexec: libc::c_int = 0;
pub unsafe extern "C" fn rpl_fcntl(
    mut fd: libc::c_int,
    mut action: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut arg: ::std::ffi::VaListImpl;
    let mut result: libc::c_int = -(1 as libc::c_int);
    arg = args.clone();
    let mut current_block_14: u64;
    match action {
        1030 => {
            let mut target: libc::c_int = arg.arg::<libc::c_int>();
            if 0 as libc::c_int <= have_dupfd_cloexec {
                result = fcntl(fd, action, target);
                if 0 as libc::c_int <= result || *__errno_location() != 22 as libc::c_int
                {
                    have_dupfd_cloexec = 1 as libc::c_int;
                    current_block_14 = 11650488183268122163;
                } else {
                    result = rpl_fcntl(fd, 0 as libc::c_int, target);
                    if result < 0 as libc::c_int {
                        current_block_14 = 13797916685926291137;
                    } else {
                        have_dupfd_cloexec = -(1 as libc::c_int);
                        current_block_14 = 11650488183268122163;
                    }
                }
            } else {
                result = rpl_fcntl(fd, 0 as libc::c_int, target);
                current_block_14 = 11650488183268122163;
            }
            match current_block_14 {
                13797916685926291137 => {}
                _ => {
                    if 0 as libc::c_int <= result
                        && have_dupfd_cloexec == -(1 as libc::c_int)
                    {
                        let mut flags: libc::c_int = fcntl(result, 1 as libc::c_int);
                        if flags < 0 as libc::c_int
                            || fcntl(result, 2 as libc::c_int, flags | 1 as libc::c_int)
                                == -(1 as libc::c_int)
                        {
                            let mut saved_errno: libc::c_int = *__errno_location();
                            close(result);
                            *__errno_location() = saved_errno;
                            result = -(1 as libc::c_int);
                        }
                    }
                }
            }
        }
        _ => {
            let mut p: *mut libc::c_void = arg.arg::<*mut libc::c_void>();
            result = fcntl(fd, action, p);
        }
    }
    return result;
}
unsafe extern "C" fn run_static_initializers() {
    have_dupfd_cloexec = if 0 as libc::c_int != 0 {
        -(1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
