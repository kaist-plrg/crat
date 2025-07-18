use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn fchdir(__fd: libc::c_int) -> libc::c_int;
    fn openat_restore_fail(_: libc::c_int);
    fn openat_save_fail(_: libc::c_int);
    fn save_cwd(cwd: *mut saved_cwd) -> libc::c_int;
    fn restore_cwd(cwd: *const saved_cwd) -> libc::c_int;
    fn free_cwd(cwd: *mut saved_cwd);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_cwd {
    pub desc: libc::c_int,
    pub name: *mut libc::c_char,
}
pub unsafe extern "C" fn run_in_dir(
    mut there: *const saved_cwd,
    mut callback: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    mut usercontext: *mut libc::c_void,
) -> libc::c_int {
    let mut err: libc::c_int = -(1 as libc::c_int);
    let mut saved_errno: libc::c_int = 0 as libc::c_int;
    let mut here: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut libc::c_char,
    };
    if 0 as libc::c_int == save_cwd(&mut here) {
        if 0 as libc::c_int == restore_cwd(there) {
            err = callback.unwrap()(usercontext);
            saved_errno = if err < 0 as libc::c_int {
                *__errno_location()
            } else {
                0 as libc::c_int
            };
        } else {
            openat_restore_fail(*__errno_location());
        }
        if restore_cwd(&mut here) != 0 as libc::c_int {
            openat_restore_fail(*__errno_location());
        }
        free_cwd(&mut here);
    } else {
        openat_save_fail(*__errno_location());
    }
    if saved_errno != 0 {
        *__errno_location() = saved_errno;
    }
    return err;
}
pub unsafe extern "C" fn run_in_dirfd(
    mut dir_fd: libc::c_int,
    mut callback: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    mut usercontext: *mut libc::c_void,
) -> libc::c_int {
    if dir_fd == -(100 as libc::c_int) {
        return (Some(callback.unwrap())).unwrap()(usercontext)
    } else {
        let mut saved_cwd: saved_cwd = saved_cwd {
            desc: 0,
            name: 0 as *mut libc::c_char,
        };
        let mut saved_errno: libc::c_int = 0;
        let mut err: libc::c_int = 0;
        if save_cwd(&mut saved_cwd) != 0 as libc::c_int {
            openat_save_fail(*__errno_location());
        }
        if fchdir(dir_fd) != 0 as libc::c_int {
            saved_errno = *__errno_location();
            free_cwd(&mut saved_cwd);
            *__errno_location() = saved_errno;
            return -(1 as libc::c_int);
        }
        err = (Some(callback.unwrap())).unwrap()(usercontext);
        saved_errno = if err < 0 as libc::c_int {
            *__errno_location()
        } else {
            0 as libc::c_int
        };
        if restore_cwd(&mut saved_cwd) != 0 as libc::c_int {
            openat_restore_fail(*__errno_location());
        }
        free_cwd(&mut saved_cwd);
        if saved_errno != 0 {
            *__errno_location() = saved_errno;
        }
        return err;
    };
}
