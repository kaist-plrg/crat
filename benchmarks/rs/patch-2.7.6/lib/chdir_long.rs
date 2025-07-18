use ::libc;
extern "C" {
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn fchdir(__fd: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn openat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __oflag: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn memrchr(
        __s: *const libc::c_void,
        __c: libc::c_int,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cd_buf {
    pub fd: libc::c_int,
}
unsafe extern "C" fn cdb_init(mut cdb: *mut cd_buf) {
    (*cdb).fd = -(100 as libc::c_int);
}
unsafe extern "C" fn cdb_fchdir(mut cdb: *const cd_buf) -> libc::c_int {
    return fchdir((*cdb).fd);
}
unsafe extern "C" fn cdb_free(mut cdb: *const cd_buf) {
    if 0 as libc::c_int <= (*cdb).fd {
        let mut close_fail: bool = close((*cdb).fd) != 0;
        if !close_fail {} else {
            __assert_fail(
                b"! close_fail\0" as *const u8 as *const libc::c_char,
                b"chdir-long.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void cdb_free(const struct cd_buf *)\0"))
                    .as_ptr(),
            );
        }
        'c_1417: {
            if !close_fail {} else {
                __assert_fail(
                    b"! close_fail\0" as *const u8 as *const libc::c_char,
                    b"chdir-long.c\0" as *const u8 as *const libc::c_char,
                    64 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 37],
                        &[libc::c_char; 37],
                    >(b"void cdb_free(const struct cd_buf *)\0"))
                        .as_ptr(),
                );
            }
        };
    }
}
unsafe extern "C" fn cdb_advance_fd(
    mut cdb: *mut cd_buf,
    mut dir: *const libc::c_char,
) -> libc::c_int {
    let mut new_fd: libc::c_int = openat(
        (*cdb).fd,
        dir,
        0 as libc::c_int | 0o200000 as libc::c_int | 0o400 as libc::c_int
            | 0o4000 as libc::c_int,
    );
    if new_fd < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    cdb_free(cdb);
    (*cdb).fd = new_fd;
    return 0 as libc::c_int;
}
unsafe extern "C" fn find_non_slash(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut n_slash: size_t = strspn(s, b"/\0" as *const u8 as *const libc::c_char);
    return (s as *mut libc::c_char).offset(n_slash as isize);
}
pub unsafe extern "C" fn chdir_long(mut dir: *mut libc::c_char) -> libc::c_int {
    let mut current_block: u64;
    let mut e: libc::c_int = chdir(dir);
    if e == 0 as libc::c_int || *__errno_location() != 36 as libc::c_int {
        return e;
    }
    let mut len: size_t = strlen(dir);
    let mut dir_end: *mut libc::c_char = dir.offset(len as isize);
    let mut cdb: cd_buf = cd_buf { fd: 0 };
    let mut n_leading_slash: size_t = 0;
    cdb_init(&mut cdb);
    if (0 as libc::c_int as libc::c_ulong) < len {} else {
        __assert_fail(
            b"0 < len\0" as *const u8 as *const libc::c_char,
            b"chdir-long.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int chdir_long(char *)\0"))
                .as_ptr(),
        );
    }
    'c_2016: {
        if (0 as libc::c_int as libc::c_ulong) < len {} else {
            __assert_fail(
                b"0 < len\0" as *const u8 as *const libc::c_char,
                b"chdir-long.c\0" as *const u8 as *const libc::c_char,
                126 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int chdir_long(char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if 4096 as libc::c_int as libc::c_ulong <= len {} else {
        __assert_fail(
            b"4096 <= len\0" as *const u8 as *const libc::c_char,
            b"chdir-long.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int chdir_long(char *)\0"))
                .as_ptr(),
        );
    }
    'c_1978: {
        if 4096 as libc::c_int as libc::c_ulong <= len {} else {
            __assert_fail(
                b"4096 <= len\0" as *const u8 as *const libc::c_char,
                b"chdir-long.c\0" as *const u8 as *const libc::c_char,
                127 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int chdir_long(char *)\0"))
                    .as_ptr(),
            );
        }
    };
    n_leading_slash = strspn(dir, b"/\0" as *const u8 as *const libc::c_char);
    if n_leading_slash == 2 as libc::c_int as libc::c_ulong {
        let mut err: libc::c_int = 0;
        let mut slash: *mut libc::c_char = memchr(
            dir.offset(3 as libc::c_int as isize) as *const libc::c_void,
            '/' as i32,
            dir_end.offset_from(dir.offset(3 as libc::c_int as isize)) as libc::c_long
                as libc::c_ulong,
        ) as *mut libc::c_char;
        if slash.is_null() {
            *__errno_location() = 36 as libc::c_int;
            return -(1 as libc::c_int);
        }
        *slash = '\0' as i32 as libc::c_char;
        err = cdb_advance_fd(&mut cdb, dir);
        *slash = '/' as i32 as libc::c_char;
        if err != 0 as libc::c_int {
            current_block = 15949835238375093025;
        } else {
            dir = find_non_slash(slash.offset(1 as libc::c_int as isize));
            current_block = 4956146061682418353;
        }
    } else if n_leading_slash != 0 {
        if cdb_advance_fd(&mut cdb, b"/\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
        {
            current_block = 15949835238375093025;
        } else {
            dir = dir.offset(n_leading_slash as isize);
            current_block = 4956146061682418353;
        }
    } else {
        current_block = 4956146061682418353;
    }
    match current_block {
        4956146061682418353 => {
            if *dir as libc::c_int != '/' as i32 {} else {
                __assert_fail(
                    b"*dir != '/'\0" as *const u8 as *const libc::c_char,
                    b"chdir-long.c\0" as *const u8 as *const libc::c_char,
                    162 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"int chdir_long(char *)\0"))
                        .as_ptr(),
                );
            }
            'c_1807: {
                if *dir as libc::c_int != '/' as i32 {} else {
                    __assert_fail(
                        b"*dir != '/'\0" as *const u8 as *const libc::c_char,
                        b"chdir-long.c\0" as *const u8 as *const libc::c_char,
                        162 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 23],
                            &[libc::c_char; 23],
                        >(b"int chdir_long(char *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if dir <= dir_end {} else {
                __assert_fail(
                    b"dir <= dir_end\0" as *const u8 as *const libc::c_char,
                    b"chdir-long.c\0" as *const u8 as *const libc::c_char,
                    163 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"int chdir_long(char *)\0"))
                        .as_ptr(),
                );
            }
            'c_1768: {
                if dir <= dir_end {} else {
                    __assert_fail(
                        b"dir <= dir_end\0" as *const u8 as *const libc::c_char,
                        b"chdir-long.c\0" as *const u8 as *const libc::c_char,
                        163 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 23],
                            &[libc::c_char; 23],
                        >(b"int chdir_long(char *)\0"))
                            .as_ptr(),
                    );
                }
            };
            loop {
                if !(4096 as libc::c_int as libc::c_long
                    <= dir_end.offset_from(dir) as libc::c_long)
                {
                    current_block = 14763689060501151050;
                    break;
                }
                let mut err_0: libc::c_int = 0;
                let mut slash_0: *mut libc::c_char = memrchr(
                    dir as *const libc::c_void,
                    '/' as i32,
                    4096 as libc::c_int as size_t,
                ) as *mut libc::c_char;
                if slash_0.is_null() {
                    *__errno_location() = 36 as libc::c_int;
                    return -(1 as libc::c_int);
                }
                *slash_0 = '\0' as i32 as libc::c_char;
                if (slash_0.offset_from(dir) as libc::c_long)
                    < 4096 as libc::c_int as libc::c_long
                {} else {
                    __assert_fail(
                        b"slash - dir < 4096\0" as *const u8 as *const libc::c_char,
                        b"chdir-long.c\0" as *const u8 as *const libc::c_char,
                        179 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 23],
                            &[libc::c_char; 23],
                        >(b"int chdir_long(char *)\0"))
                            .as_ptr(),
                    );
                }
                'c_1688: {
                    if (slash_0.offset_from(dir) as libc::c_long)
                        < 4096 as libc::c_int as libc::c_long
                    {} else {
                        __assert_fail(
                            b"slash - dir < 4096\0" as *const u8 as *const libc::c_char,
                            b"chdir-long.c\0" as *const u8 as *const libc::c_char,
                            179 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 23],
                                &[libc::c_char; 23],
                            >(b"int chdir_long(char *)\0"))
                                .as_ptr(),
                        );
                    }
                };
                err_0 = cdb_advance_fd(&mut cdb, dir);
                *slash_0 = '/' as i32 as libc::c_char;
                if err_0 != 0 as libc::c_int {
                    current_block = 15949835238375093025;
                    break;
                }
                dir = find_non_slash(slash_0.offset(1 as libc::c_int as isize));
            }
            match current_block {
                15949835238375093025 => {}
                _ => {
                    if dir < dir_end {
                        if cdb_advance_fd(&mut cdb, dir) != 0 as libc::c_int {
                            current_block = 15949835238375093025;
                        } else {
                            current_block = 5689316957504528238;
                        }
                    } else {
                        current_block = 5689316957504528238;
                    }
                    match current_block {
                        15949835238375093025 => {}
                        _ => {
                            if !(cdb_fchdir(&mut cdb) != 0 as libc::c_int) {
                                cdb_free(&mut cdb);
                                return 0 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    let mut saved_errno: libc::c_int = *__errno_location();
    cdb_free(&mut cdb);
    *__errno_location() = saved_errno;
    return -(1 as libc::c_int);
}
