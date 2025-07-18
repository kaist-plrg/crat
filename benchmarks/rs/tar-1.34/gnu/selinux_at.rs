use ::libc;
extern "C" {
    fn rpl_getfilecon(
        path: *const libc::c_char,
        con: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn rpl_lgetfilecon(
        path: *const libc::c_char,
        con: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn setfilecon(path: *const libc::c_char, con: *const libc::c_char) -> libc::c_int;
    fn lsetfilecon(path: *const libc::c_char, con: *const libc::c_char) -> libc::c_int;
    fn openat_restore_fail(_: libc::c_int);
    fn openat_save_fail(_: libc::c_int);
    fn fchdir(__fd: libc::c_int) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn save_cwd(cwd: *mut saved_cwd) -> libc::c_int;
    fn restore_cwd(cwd: *const saved_cwd) -> libc::c_int;
    fn free_cwd(cwd: *mut saved_cwd);
    fn openat_proc_name(
        buf: *mut libc::c_char,
        fd: libc::c_int,
        file: *const libc::c_char,
    ) -> *mut libc::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_cwd {
    pub desc: libc::c_int,
    pub name: *mut libc::c_char,
}
pub unsafe extern "C" fn lsetfileconat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut con: *const libc::c_char,
) -> libc::c_int {
    if fd == -(100 as libc::c_int)
        || *file.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        return lsetfilecon(file, con);
    }
    let mut saved_cwd: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut libc::c_char,
    };
    let mut saved_errno: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut proc_buf: [libc::c_char; 4032] = [0; 4032];
    let mut proc_file: *mut libc::c_char = openat_proc_name(
        proc_buf.as_mut_ptr(),
        fd,
        file,
    );
    if !proc_file.is_null() {
        let mut proc_result: libc::c_int = lsetfilecon(proc_file, con);
        let mut proc_errno: libc::c_int = *__errno_location();
        if proc_file != proc_buf.as_mut_ptr() {
            rpl_free(proc_file as *mut libc::c_void);
        }
        if -(1 as libc::c_int) != proc_result {
            return proc_result;
        }
        if !(proc_errno == 20 as libc::c_int || proc_errno == 2 as libc::c_int
            || proc_errno == 1 as libc::c_int || proc_errno == 13 as libc::c_int
            || proc_errno == 38 as libc::c_int || proc_errno == 95 as libc::c_int)
        {
            *__errno_location() = proc_errno;
            return proc_result;
        }
    }
    if save_cwd(&mut saved_cwd) != 0 as libc::c_int {
        openat_save_fail(*__errno_location());
    }
    if 0 as libc::c_int <= fd && fd == saved_cwd.desc {
        free_cwd(&mut saved_cwd);
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if fchdir(fd) != 0 as libc::c_int {
        saved_errno = *__errno_location();
        free_cwd(&mut saved_cwd);
        *__errno_location() = saved_errno;
        return -(1 as libc::c_int);
    }
    err = lsetfilecon(file, con);
    saved_errno = if err == -(1 as libc::c_int) {
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
}
pub unsafe extern "C" fn lgetfileconat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut con: *mut *mut libc::c_char,
) -> libc::c_int {
    if fd == -(100 as libc::c_int)
        || *file.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        return rpl_lgetfilecon(file, con);
    }
    let mut saved_cwd: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut libc::c_char,
    };
    let mut saved_errno: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut proc_buf: [libc::c_char; 4032] = [0; 4032];
    let mut proc_file: *mut libc::c_char = openat_proc_name(
        proc_buf.as_mut_ptr(),
        fd,
        file,
    );
    if !proc_file.is_null() {
        let mut proc_result: libc::c_int = rpl_lgetfilecon(proc_file, con);
        let mut proc_errno: libc::c_int = *__errno_location();
        if proc_file != proc_buf.as_mut_ptr() {
            rpl_free(proc_file as *mut libc::c_void);
        }
        if -(1 as libc::c_int) != proc_result {
            return proc_result;
        }
        if !(proc_errno == 20 as libc::c_int || proc_errno == 2 as libc::c_int
            || proc_errno == 1 as libc::c_int || proc_errno == 13 as libc::c_int
            || proc_errno == 38 as libc::c_int || proc_errno == 95 as libc::c_int)
        {
            *__errno_location() = proc_errno;
            return proc_result;
        }
    }
    if save_cwd(&mut saved_cwd) != 0 as libc::c_int {
        openat_save_fail(*__errno_location());
    }
    if 0 as libc::c_int <= fd && fd == saved_cwd.desc {
        free_cwd(&mut saved_cwd);
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if fchdir(fd) != 0 as libc::c_int {
        saved_errno = *__errno_location();
        free_cwd(&mut saved_cwd);
        *__errno_location() = saved_errno;
        return -(1 as libc::c_int);
    }
    err = rpl_lgetfilecon(file, con);
    saved_errno = if err == -(1 as libc::c_int) {
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
}
pub unsafe extern "C" fn setfileconat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut con: *const libc::c_char,
) -> libc::c_int {
    if fd == -(100 as libc::c_int)
        || *file.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        return setfilecon(file, con);
    }
    let mut saved_cwd: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut libc::c_char,
    };
    let mut saved_errno: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut proc_buf: [libc::c_char; 4032] = [0; 4032];
    let mut proc_file: *mut libc::c_char = openat_proc_name(
        proc_buf.as_mut_ptr(),
        fd,
        file,
    );
    if !proc_file.is_null() {
        let mut proc_result: libc::c_int = setfilecon(proc_file, con);
        let mut proc_errno: libc::c_int = *__errno_location();
        if proc_file != proc_buf.as_mut_ptr() {
            rpl_free(proc_file as *mut libc::c_void);
        }
        if -(1 as libc::c_int) != proc_result {
            return proc_result;
        }
        if !(proc_errno == 20 as libc::c_int || proc_errno == 2 as libc::c_int
            || proc_errno == 1 as libc::c_int || proc_errno == 13 as libc::c_int
            || proc_errno == 38 as libc::c_int || proc_errno == 95 as libc::c_int)
        {
            *__errno_location() = proc_errno;
            return proc_result;
        }
    }
    if save_cwd(&mut saved_cwd) != 0 as libc::c_int {
        openat_save_fail(*__errno_location());
    }
    if 0 as libc::c_int <= fd && fd == saved_cwd.desc {
        free_cwd(&mut saved_cwd);
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if fchdir(fd) != 0 as libc::c_int {
        saved_errno = *__errno_location();
        free_cwd(&mut saved_cwd);
        *__errno_location() = saved_errno;
        return -(1 as libc::c_int);
    }
    err = setfilecon(file, con);
    saved_errno = if err == -(1 as libc::c_int) {
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
}
pub unsafe extern "C" fn getfileconat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut con: *mut *mut libc::c_char,
) -> libc::c_int {
    if fd == -(100 as libc::c_int)
        || *file.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        return rpl_getfilecon(file, con);
    }
    let mut saved_cwd: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut libc::c_char,
    };
    let mut saved_errno: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut proc_buf: [libc::c_char; 4032] = [0; 4032];
    let mut proc_file: *mut libc::c_char = openat_proc_name(
        proc_buf.as_mut_ptr(),
        fd,
        file,
    );
    if !proc_file.is_null() {
        let mut proc_result: libc::c_int = rpl_getfilecon(proc_file, con);
        let mut proc_errno: libc::c_int = *__errno_location();
        if proc_file != proc_buf.as_mut_ptr() {
            rpl_free(proc_file as *mut libc::c_void);
        }
        if -(1 as libc::c_int) != proc_result {
            return proc_result;
        }
        if !(proc_errno == 20 as libc::c_int || proc_errno == 2 as libc::c_int
            || proc_errno == 1 as libc::c_int || proc_errno == 13 as libc::c_int
            || proc_errno == 38 as libc::c_int || proc_errno == 95 as libc::c_int)
        {
            *__errno_location() = proc_errno;
            return proc_result;
        }
    }
    if save_cwd(&mut saved_cwd) != 0 as libc::c_int {
        openat_save_fail(*__errno_location());
    }
    if 0 as libc::c_int <= fd && fd == saved_cwd.desc {
        free_cwd(&mut saved_cwd);
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if fchdir(fd) != 0 as libc::c_int {
        saved_errno = *__errno_location();
        free_cwd(&mut saved_cwd);
        *__errno_location() = saved_errno;
        return -(1 as libc::c_int);
    }
    err = rpl_getfilecon(file, con);
    saved_errno = if err == -(1 as libc::c_int) {
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
}
