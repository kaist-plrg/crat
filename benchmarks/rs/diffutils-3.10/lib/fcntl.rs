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
pub unsafe extern "C" fn rpl_fcntl(
    mut fd: libc::c_int,
    mut action: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut arg: ::std::ffi::VaListImpl;
    let mut result: libc::c_int = -(1 as libc::c_int);
    arg = args.clone();
    match action {
        0 => {
            let mut target: libc::c_int = arg.arg::<libc::c_int>();
            result = rpl_fcntl_DUPFD(fd, target);
        }
        1030 => {
            let mut target_0: libc::c_int = arg.arg::<libc::c_int>();
            result = rpl_fcntl_DUPFD_CLOEXEC(fd, target_0);
        }
        _ => {
            let mut current_block_7: u64;
            match action {
                1 => {
                    current_block_7 = 4251714036681004048;
                }
                3 => {
                    current_block_7 = 4251714036681004048;
                }
                1025 => {
                    current_block_7 = 9121811622057238928;
                }
                9 => {
                    current_block_7 = 5544887021832600539;
                }
                1032 => {
                    current_block_7 = 13960030303447277661;
                }
                1034 => {
                    current_block_7 = 5055406634176892754;
                }
                11 => {
                    current_block_7 = 4083966596980890686;
                }
                1033 => {
                    current_block_7 = 2567727553271361248;
                }
                0 => {
                    current_block_7 = 2567727553271361248;
                }
                1030 => {
                    current_block_7 = 374709647620725240;
                }
                1026 => {
                    current_block_7 = 8906923678235178254;
                }
                2 => {
                    current_block_7 = 12639127870197313613;
                }
                4 => {
                    current_block_7 = 17369357150903078169;
                }
                8 => {
                    current_block_7 = 5373097413690669091;
                }
                1031 => {
                    current_block_7 = 5373097413690669091;
                }
                1024 | 10 => {
                    current_block_7 = 632017910184161063;
                }
                _ => {
                    let mut p: *mut libc::c_void = arg.arg::<*mut libc::c_void>();
                    result = fcntl(fd, action, p);
                    current_block_7 = 7175849428784450219;
                }
            }
            match current_block_7 {
                4251714036681004048 => {
                    current_block_7 = 9121811622057238928;
                }
                2567727553271361248 => {
                    current_block_7 = 374709647620725240;
                }
                5373097413690669091 => {
                    current_block_7 = 632017910184161063;
                }
                _ => {}
            }
            match current_block_7 {
                9121811622057238928 => {
                    current_block_7 = 5544887021832600539;
                }
                374709647620725240 => {
                    current_block_7 = 8906923678235178254;
                }
                _ => {}
            }
            match current_block_7 {
                5544887021832600539 => {
                    current_block_7 = 13960030303447277661;
                }
                8906923678235178254 => {
                    current_block_7 = 12639127870197313613;
                }
                _ => {}
            }
            match current_block_7 {
                13960030303447277661 => {
                    current_block_7 = 5055406634176892754;
                }
                12639127870197313613 => {
                    current_block_7 = 17369357150903078169;
                }
                _ => {}
            }
            match current_block_7 {
                5055406634176892754 => {
                    current_block_7 = 4083966596980890686;
                }
                17369357150903078169 => {
                    current_block_7 = 632017910184161063;
                }
                _ => {}
            }
            match current_block_7 {
                4083966596980890686 => {
                    result = fcntl(fd, action);
                }
                632017910184161063 => {
                    let mut x: libc::c_int = arg.arg::<libc::c_int>();
                    result = fcntl(fd, action, x);
                }
                _ => {}
            }
        }
    }
    return result;
}
unsafe extern "C" fn rpl_fcntl_DUPFD(
    mut fd: libc::c_int,
    mut target: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    result = fcntl(fd, 0 as libc::c_int, target);
    return result;
}
static mut have_dupfd_cloexec: libc::c_int = 0;
unsafe extern "C" fn rpl_fcntl_DUPFD_CLOEXEC(
    mut fd: libc::c_int,
    mut target: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    if 0 as libc::c_int <= have_dupfd_cloexec {
        result = fcntl(fd, 1030 as libc::c_int, target);
        if 0 as libc::c_int <= result || *__errno_location() != 22 as libc::c_int {
            have_dupfd_cloexec = 1 as libc::c_int;
        } else {
            result = rpl_fcntl_DUPFD(fd, target);
            if result >= 0 as libc::c_int {
                have_dupfd_cloexec = -(1 as libc::c_int);
            }
        }
    } else {
        result = rpl_fcntl_DUPFD(fd, target);
    }
    if 0 as libc::c_int <= result && have_dupfd_cloexec == -(1 as libc::c_int) {
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
