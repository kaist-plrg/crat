use ::libc;
extern "C" {
    fn llistxattr(
        __path: *const libc::c_char,
        __list: *mut libc::c_char,
        __size: size_t,
    ) -> ssize_t;
    fn listxattr(
        __path: *const libc::c_char,
        __list: *mut libc::c_char,
        __size: size_t,
    ) -> ssize_t;
    fn lgetxattr(
        __path: *const libc::c_char,
        __name: *const libc::c_char,
        __value: *mut libc::c_void,
        __size: size_t,
    ) -> ssize_t;
    fn getxattr(
        __path: *const libc::c_char,
        __name: *const libc::c_char,
        __value: *mut libc::c_void,
        __size: size_t,
    ) -> ssize_t;
    fn setxattr(
        __path: *const libc::c_char,
        __name: *const libc::c_char,
        __value: *const libc::c_void,
        __size: size_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn lsetxattr(
        __path: *const libc::c_char,
        __name: *const libc::c_char,
        __value: *const libc::c_void,
        __size: size_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn openat_restore_fail(_: libc::c_int);
    fn fchdir(__fd: libc::c_int) -> libc::c_int;
    fn openat_save_fail(_: libc::c_int);
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
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_cwd {
    pub desc: libc::c_int,
    pub name: *mut libc::c_char,
}
pub unsafe extern "C" fn lgetxattrat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut name: *const libc::c_char,
    mut value: *mut libc::c_void,
    mut size: size_t,
) -> ssize_t {
    if fd == -(100 as libc::c_int)
        || *file.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        return lgetxattr(file, name, value, size);
    }
    let mut saved_cwd: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut libc::c_char,
    };
    let mut saved_errno: libc::c_int = 0;
    let mut err: ssize_t = 0;
    let mut proc_buf: [libc::c_char; 4032] = [0; 4032];
    let mut proc_file: *mut libc::c_char = openat_proc_name(
        proc_buf.as_mut_ptr(),
        fd,
        file,
    );
    if !proc_file.is_null() {
        let mut proc_result: ssize_t = lgetxattr(proc_file, name, value, size);
        let mut proc_errno: libc::c_int = *__errno_location();
        if proc_file != proc_buf.as_mut_ptr() {
            rpl_free(proc_file as *mut libc::c_void);
        }
        if -(1 as libc::c_int) as libc::c_long != proc_result {
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
        return -(1 as libc::c_int) as ssize_t;
    }
    if fchdir(fd) != 0 as libc::c_int {
        saved_errno = *__errno_location();
        free_cwd(&mut saved_cwd);
        *__errno_location() = saved_errno;
        return -(1 as libc::c_int) as ssize_t;
    }
    err = lgetxattr(file, name, value, size);
    saved_errno = if err == -(1 as libc::c_int) as libc::c_long {
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
pub unsafe extern "C" fn llistxattrat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut list: *mut libc::c_char,
    mut size: size_t,
) -> ssize_t {
    if fd == -(100 as libc::c_int)
        || *file.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        return llistxattr(file, list, size);
    }
    let mut saved_cwd: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut libc::c_char,
    };
    let mut saved_errno: libc::c_int = 0;
    let mut err: ssize_t = 0;
    let mut proc_buf: [libc::c_char; 4032] = [0; 4032];
    let mut proc_file: *mut libc::c_char = openat_proc_name(
        proc_buf.as_mut_ptr(),
        fd,
        file,
    );
    if !proc_file.is_null() {
        let mut proc_result: ssize_t = llistxattr(proc_file, list, size);
        let mut proc_errno: libc::c_int = *__errno_location();
        if proc_file != proc_buf.as_mut_ptr() {
            rpl_free(proc_file as *mut libc::c_void);
        }
        if -(1 as libc::c_int) as libc::c_long != proc_result {
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
        return -(1 as libc::c_int) as ssize_t;
    }
    if fchdir(fd) != 0 as libc::c_int {
        saved_errno = *__errno_location();
        free_cwd(&mut saved_cwd);
        *__errno_location() = saved_errno;
        return -(1 as libc::c_int) as ssize_t;
    }
    err = llistxattr(file, list, size);
    saved_errno = if err == -(1 as libc::c_int) as libc::c_long {
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
pub unsafe extern "C" fn getxattrat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut name: *const libc::c_char,
    mut value: *mut libc::c_void,
    mut size: size_t,
) -> ssize_t {
    if fd == -(100 as libc::c_int)
        || *file.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        return getxattr(file, name, value, size);
    }
    let mut saved_cwd: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut libc::c_char,
    };
    let mut saved_errno: libc::c_int = 0;
    let mut err: ssize_t = 0;
    let mut proc_buf: [libc::c_char; 4032] = [0; 4032];
    let mut proc_file: *mut libc::c_char = openat_proc_name(
        proc_buf.as_mut_ptr(),
        fd,
        file,
    );
    if !proc_file.is_null() {
        let mut proc_result: ssize_t = getxattr(proc_file, name, value, size);
        let mut proc_errno: libc::c_int = *__errno_location();
        if proc_file != proc_buf.as_mut_ptr() {
            rpl_free(proc_file as *mut libc::c_void);
        }
        if -(1 as libc::c_int) as libc::c_long != proc_result {
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
        return -(1 as libc::c_int) as ssize_t;
    }
    if fchdir(fd) != 0 as libc::c_int {
        saved_errno = *__errno_location();
        free_cwd(&mut saved_cwd);
        *__errno_location() = saved_errno;
        return -(1 as libc::c_int) as ssize_t;
    }
    err = getxattr(file, name, value, size);
    saved_errno = if err == -(1 as libc::c_int) as libc::c_long {
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
pub unsafe extern "C" fn listxattrat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut list: *mut libc::c_char,
    mut size: size_t,
) -> ssize_t {
    if fd == -(100 as libc::c_int)
        || *file.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        return listxattr(file, list, size);
    }
    let mut saved_cwd: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut libc::c_char,
    };
    let mut saved_errno: libc::c_int = 0;
    let mut err: ssize_t = 0;
    let mut proc_buf: [libc::c_char; 4032] = [0; 4032];
    let mut proc_file: *mut libc::c_char = openat_proc_name(
        proc_buf.as_mut_ptr(),
        fd,
        file,
    );
    if !proc_file.is_null() {
        let mut proc_result: ssize_t = listxattr(proc_file, list, size);
        let mut proc_errno: libc::c_int = *__errno_location();
        if proc_file != proc_buf.as_mut_ptr() {
            rpl_free(proc_file as *mut libc::c_void);
        }
        if -(1 as libc::c_int) as libc::c_long != proc_result {
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
        return -(1 as libc::c_int) as ssize_t;
    }
    if fchdir(fd) != 0 as libc::c_int {
        saved_errno = *__errno_location();
        free_cwd(&mut saved_cwd);
        *__errno_location() = saved_errno;
        return -(1 as libc::c_int) as ssize_t;
    }
    err = listxattr(file, list, size);
    saved_errno = if err == -(1 as libc::c_int) as libc::c_long {
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
pub unsafe extern "C" fn setxattrat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut name: *const libc::c_char,
    mut value: *const libc::c_void,
    mut size: size_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    if fd == -(100 as libc::c_int)
        || *file.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        return setxattr(file, name, value, size, flags);
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
        let mut proc_result: libc::c_int = setxattr(proc_file, name, value, size, flags);
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
    err = setxattr(file, name, value, size, flags);
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
pub unsafe extern "C" fn lsetxattrat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut name: *const libc::c_char,
    mut value: *const libc::c_void,
    mut size: size_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    if fd == -(100 as libc::c_int)
        || *file.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        return lsetxattr(file, name, value, size, flags);
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
        let mut proc_result: libc::c_int = lsetxattr(
            proc_file,
            name,
            value,
            size,
            flags,
        );
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
    err = lsetxattr(file, name, value, size, flags);
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
