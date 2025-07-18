use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optind: libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn close_stdout();
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn explain_how_to_report_bugs(
        f: *mut FILE,
        program_name_0: *const libc::c_char,
    ) -> libc::c_int;
    fn display_findutils_version(official_name: *const libc::c_char);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _gl_dummy: libc::c_int,
}
unsafe extern "C" fn put_short(mut c: libc::c_int, mut fp: *mut FILE) -> libc::c_int {
    if c <= 32767 as libc::c_int {} else {
        __assert_fail(
            b"c <= SHRT_MAX\0" as *const u8 as *const libc::c_char,
            b"frcode.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"int put_short(int, FILE *)\0"))
                .as_ptr(),
        );
    }
    'c_3311: {
        if c <= 32767 as libc::c_int {} else {
            __assert_fail(
                b"c <= SHRT_MAX\0" as *const u8 as *const libc::c_char,
                b"frcode.c\0" as *const u8 as *const libc::c_char,
                104 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"int put_short(int, FILE *)\0"))
                    .as_ptr(),
            );
        }
    };
    if c >= -(32767 as libc::c_int) - 1 as libc::c_int {} else {
        __assert_fail(
            b"c >= SHRT_MIN\0" as *const u8 as *const libc::c_char,
            b"frcode.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"int put_short(int, FILE *)\0"))
                .as_ptr(),
        );
    }
    'c_3262: {
        if c >= -(32767 as libc::c_int) - 1 as libc::c_int {} else {
            __assert_fail(
                b"c >= SHRT_MIN\0" as *const u8 as *const libc::c_char,
                b"frcode.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"int put_short(int, FILE *)\0"))
                    .as_ptr(),
            );
        }
    };
    return (putc(c >> 8 as libc::c_int, fp) != -(1 as libc::c_int)
        && putc(c, fp) != -(1 as libc::c_int)) as libc::c_int;
}
unsafe extern "C" fn prefix_length(
    mut s1: *mut libc::c_char,
    mut s2: *mut libc::c_char,
) -> libc::c_int {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut limit: libc::c_int = 2147483647 as libc::c_int;
    start = s1;
    while *s1 as libc::c_int == *s2 as libc::c_int && *s1 as libc::c_int != '\0' as i32 {
        limit -= 1;
        if 0 as libc::c_int == limit {
            break;
        }
        s1 = s1.offset(1);
        s1;
        s2 = s2.offset(1);
        s2;
    }
    return s1.offset_from(start) as libc::c_long as libc::c_int;
}
static mut longopts: [option; 4] = [
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"null\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: '0' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn usage(mut status: libc::c_int) -> ! {
    if status != 0 as libc::c_int {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Try '%s --help' for more information.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        exit(status);
    }
    fprintf(
        stdout,
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s [-0 | --null] [--version] [--help]\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        program_name,
    );
    explain_how_to_report_bugs(stdout, program_name);
    exit(status);
}
unsafe extern "C" fn get_seclevel(mut s: *mut libc::c_char) -> libc::c_long {
    let mut result: libc::c_long = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    result = strtol(s, &mut p, 10 as libc::c_int);
    if 0 as libc::c_int as libc::c_long == result && p == optarg {
        if ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"You need to specify a security level as a decimal integer.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"You need to specify a security level as a decimal integer.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
        return -(1 as libc::c_int) as libc::c_long;
    } else if (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long == result
        || 9223372036854775807 as libc::c_long == result) && *__errno_location() != 0
    {
        if ::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Security level %s is outside the convertible range.\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                s,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Security level %s is outside the convertible range.\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                s,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
        return -(1 as libc::c_int) as libc::c_long;
    } else if *p != 0 {
        if ::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Security level %s has unexpected suffix %s.\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                s,
                p,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Security level %s has unexpected suffix %s.\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                s,
                p,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
        return -(1 as libc::c_int) as libc::c_long;
    } else {
        return result
    };
}
unsafe extern "C" fn outerr() {
    if ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
        error(
            1 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"write error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    } else {
        error(
            1 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"write error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oldpath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pathsize: size_t = 0;
    let mut oldpathsize: size_t = 0;
    let mut count: libc::c_int = 0;
    let mut oldcount: libc::c_int = 0;
    let mut diffcount: libc::c_int = 0;
    let mut line_len: libc::c_int = 0;
    let mut delimiter: libc::c_int = '\n' as i32;
    let mut optc: libc::c_int = 0;
    let mut slocate_compat: libc::c_int = 0 as libc::c_int;
    let mut slocate_seclevel: libc::c_long = 0 as libc::c_long;
    if !(*argv.offset(0 as libc::c_int as isize)).is_null() {
        set_program_name(*argv.offset(0 as libc::c_int as isize));
    } else {
        set_program_name(b"frcode\0" as *const u8 as *const libc::c_char);
    }
    if atexit(Some(close_stdout as unsafe extern "C" fn() -> ())) != 0 {
        if ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"The atexit library function failed\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"The atexit library function failed\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    oldpathsize = 1026 as libc::c_int as size_t;
    pathsize = oldpathsize;
    path = xmalloc(pathsize) as *mut libc::c_char;
    oldpath = xmalloc(oldpathsize) as *mut libc::c_char;
    *oldpath.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    oldcount = 0 as libc::c_int;
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"hv0S:\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            48 => {
                delimiter = 0 as libc::c_int;
            }
            83 => {
                slocate_compat = 1 as libc::c_int;
                slocate_seclevel = get_seclevel(optarg);
                if slocate_seclevel < 0 as libc::c_int as libc::c_long
                    || slocate_seclevel > 1 as libc::c_int as libc::c_long
                {
                    if ::std::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"slocate security level %ld is unsupported.\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            slocate_seclevel,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"slocate security level %ld is unsupported.\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            slocate_seclevel,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            }
            104 => {
                usage(0 as libc::c_int);
            }
            118 => {
                display_findutils_version(
                    b"frcode\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    if optind != argc {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"no argument expected.\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    }
    if slocate_compat != 0 {
        fputc(if slocate_seclevel != 0 { '1' as i32 } else { '0' as i32 }, stdout);
        fputc(0 as libc::c_int, stdout);
    } else if fwrite(
        b"\0LOCATE02\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
        stdout,
    ) != ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
    {
        if ::std::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to write to standard output\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to write to standard output\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    loop {
        line_len = getdelim(&mut path, &mut pathsize, delimiter, stdin) as libc::c_int;
        if !(line_len > 0 as libc::c_int) {
            break;
        }
        if *path.offset((line_len - 1 as libc::c_int) as isize) as libc::c_int
            != delimiter
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"The input file should end with the delimiter\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            *path
                .offset(
                    (line_len - 1 as libc::c_int) as isize,
                ) = '\0' as i32 as libc::c_char;
        }
        count = prefix_length(oldpath, path);
        diffcount = count - oldcount;
        if diffcount > 32767 as libc::c_int
            || diffcount < -(32767 as libc::c_int) - 1 as libc::c_int
        {
            count = 0 as libc::c_int;
            diffcount = -oldcount;
        }
        oldcount = count;
        if slocate_compat != 0 {
            slocate_compat = 0 as libc::c_int;
        } else if diffcount < -(127 as libc::c_int) || diffcount > 127 as libc::c_int {
            if -(1 as libc::c_int) == putc(0x80 as libc::c_int, stdout) {
                outerr();
            }
            if put_short(diffcount, stdout) == 0 {
                outerr();
            }
        } else if -(1 as libc::c_int) == putc(diffcount, stdout) {
            outerr();
        }
        if -(1 as libc::c_int) == fputs(path.offset(count as isize), stdout)
            || -(1 as libc::c_int) == putc('\0' as i32, stdout)
        {
            outerr();
        }
        let mut tmppath: *mut libc::c_char = oldpath;
        let mut tmppathsize: size_t = oldpathsize;
        oldpath = path;
        oldpathsize = pathsize;
        path = tmppath;
        pathsize = tmppathsize;
    }
    rpl_free(path as *mut libc::c_void);
    rpl_free(oldpath as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
