use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn xfree(_: pointer);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type pointer = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub const REQUIRE_ORDER: C2RustUnnamed = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const RETURN_IN_ORDER: C2RustUnnamed = 2;
pub const PERMUTE: C2RustUnnamed = 1;
pub static mut gnu_optarg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut gnu_optind: libc::c_int = 0 as libc::c_int;
static mut nextchar: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut gnu_opterr: libc::c_int = 1 as libc::c_int;
static mut ordering: C2RustUnnamed = REQUIRE_ORDER;
static mut first_nonopt: libc::c_int = 0;
static mut last_nonopt: libc::c_int = 0;
unsafe extern "C" fn exchange(mut argv: *mut *mut libc::c_char) {
    let mut nonopts_size: size_t = ((last_nonopt - first_nonopt) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong);
    let mut temp: *mut *mut libc::c_char = malloc(nonopts_size)
        as *mut *mut libc::c_char;
    if temp.is_null() {
        abort();
    }
    memcpy(
        temp as *mut libc::c_char as *mut libc::c_void,
        &mut *argv.offset(first_nonopt as isize) as *mut *mut libc::c_char
            as *mut libc::c_char as *const libc::c_void,
        nonopts_size,
    );
    memcpy(
        &mut *argv.offset(first_nonopt as isize) as *mut *mut libc::c_char
            as *mut libc::c_char as *mut libc::c_void,
        &mut *argv.offset(last_nonopt as isize) as *mut *mut libc::c_char
            as *mut libc::c_char as *const libc::c_void,
        ((gnu_optind - last_nonopt) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    );
    memcpy(
        &mut *argv.offset((first_nonopt + gnu_optind - last_nonopt) as isize)
            as *mut *mut libc::c_char as *mut libc::c_char as *mut libc::c_void,
        temp as *mut libc::c_char as *const libc::c_void,
        nonopts_size,
    );
    xfree(temp as pointer);
    first_nonopt += gnu_optind - last_nonopt;
    last_nonopt = gnu_optind;
}
pub unsafe extern "C" fn _getopt_internal(
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
    mut optstring: *const libc::c_char,
    mut longopts: *const option,
    mut longind: *mut libc::c_int,
    mut long_only: libc::c_int,
) -> libc::c_int {
    let mut option_index: libc::c_int = 0;
    gnu_optarg = 0 as *mut libc::c_char;
    if gnu_optind == 0 as libc::c_int {
        gnu_optind = 1 as libc::c_int;
        last_nonopt = gnu_optind;
        first_nonopt = last_nonopt;
        nextchar = 0 as *mut libc::c_char;
        if *optstring.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
            ordering = RETURN_IN_ORDER;
            optstring = optstring.offset(1);
            optstring;
        } else if *optstring.offset(0 as libc::c_int as isize) as libc::c_int
            == '+' as i32
        {
            ordering = REQUIRE_ORDER;
            optstring = optstring.offset(1);
            optstring;
        } else if !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char))
            .is_null()
        {
            ordering = REQUIRE_ORDER;
        } else {
            ordering = PERMUTE;
        }
    }
    if nextchar.is_null() || *nextchar as libc::c_int == '\0' as i32 {
        if ordering as libc::c_uint == PERMUTE as libc::c_int as libc::c_uint {
            if first_nonopt != last_nonopt && last_nonopt != gnu_optind {
                exchange(argv as *mut *mut libc::c_char);
            } else if last_nonopt != gnu_optind {
                first_nonopt = gnu_optind;
            }
            while gnu_optind < argc
                && (*(*argv.offset(gnu_optind as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
                    || *(*argv.offset(gnu_optind as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32)
            {
                gnu_optind += 1;
                gnu_optind;
            }
            last_nonopt = gnu_optind;
        }
        if gnu_optind != argc
            && strcmp(
                *argv.offset(gnu_optind as isize),
                b"--\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            gnu_optind += 1;
            gnu_optind;
            if first_nonopt != last_nonopt && last_nonopt != gnu_optind {
                exchange(argv as *mut *mut libc::c_char);
            } else if first_nonopt == last_nonopt {
                first_nonopt = gnu_optind;
            }
            last_nonopt = argc;
            gnu_optind = argc;
        }
        if gnu_optind == argc {
            if first_nonopt != last_nonopt {
                gnu_optind = first_nonopt;
            }
            return -(1 as libc::c_int);
        }
        if *(*argv.offset(gnu_optind as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int != '-' as i32
            || *(*argv.offset(gnu_optind as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int == '\0' as i32
        {
            if ordering as libc::c_uint == REQUIRE_ORDER as libc::c_int as libc::c_uint {
                return -(1 as libc::c_int);
            }
            let fresh0 = gnu_optind;
            gnu_optind = gnu_optind + 1;
            gnu_optarg = *argv.offset(fresh0 as isize);
            return 1 as libc::c_int;
        }
        nextchar = (*argv.offset(gnu_optind as isize))
            .offset(1 as libc::c_int as isize)
            .offset(
                (!longopts.is_null()
                    && *(*argv.offset(gnu_optind as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32)
                    as libc::c_int as isize,
            );
    }
    if !longopts.is_null()
        && (*(*argv.offset(gnu_optind as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '-' as i32
            && (*(*argv.offset(gnu_optind as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int == '-' as i32 || long_only != 0))
    {
        let mut p: *const option = 0 as *const option;
        let mut s: *mut libc::c_char = nextchar;
        let mut exact: libc::c_int = 0 as libc::c_int;
        let mut ambig: libc::c_int = 0 as libc::c_int;
        let mut pfound: *const option = 0 as *const option;
        let mut indfound: libc::c_int = 0 as libc::c_int;
        while *s as libc::c_int != 0 && *s as libc::c_int != '=' as i32 {
            s = s.offset(1);
            s;
        }
        p = longopts;
        option_index = 0 as libc::c_int;
        while !((*p).name).is_null() {
            if strncmp(
                (*p).name,
                nextchar,
                s.offset_from(nextchar) as libc::c_long as size_t,
            ) == 0
            {
                if s.offset_from(nextchar) as libc::c_long as libc::c_ulong
                    == strlen((*p).name)
                {
                    pfound = p;
                    indfound = option_index;
                    exact = 1 as libc::c_int;
                    break;
                } else if pfound.is_null() {
                    pfound = p;
                    indfound = option_index;
                } else {
                    ambig = 1 as libc::c_int;
                }
            }
            p = p.offset(1);
            p;
            option_index += 1;
            option_index;
        }
        if ambig != 0 && exact == 0 {
            if gnu_opterr != 0 {
                fprintf(
                    stderr,
                    b"%s: option `%s' is ambiguous\n\0" as *const u8
                        as *const libc::c_char,
                    *argv.offset(0 as libc::c_int as isize),
                    *argv.offset(gnu_optind as isize),
                );
            }
            nextchar = nextchar.offset(strlen(nextchar) as isize);
            gnu_optind += 1;
            gnu_optind;
            return '?' as i32;
        }
        if !pfound.is_null() {
            option_index = indfound;
            gnu_optind += 1;
            gnu_optind;
            if *s != 0 {
                if (*pfound).has_arg != 0 {
                    gnu_optarg = s.offset(1 as libc::c_int as isize);
                } else {
                    if gnu_opterr != 0 {
                        if *(*argv.offset((gnu_optind - 1 as libc::c_int) as isize))
                            .offset(1 as libc::c_int as isize) as libc::c_int
                            == '-' as i32
                        {
                            fprintf(
                                stderr,
                                b"%s: option `--%s' doesn't allow an argument\n\0"
                                    as *const u8 as *const libc::c_char,
                                *argv.offset(0 as libc::c_int as isize),
                                (*pfound).name,
                            );
                        } else {
                            fprintf(
                                stderr,
                                b"%s: option `%c%s' doesn't allow an argument\n\0"
                                    as *const u8 as *const libc::c_char,
                                *argv.offset(0 as libc::c_int as isize),
                                *(*argv.offset((gnu_optind - 1 as libc::c_int) as isize))
                                    .offset(0 as libc::c_int as isize) as libc::c_int,
                                (*pfound).name,
                            );
                        }
                    }
                    nextchar = nextchar.offset(strlen(nextchar) as isize);
                    return '?' as i32;
                }
            } else if (*pfound).has_arg == 1 as libc::c_int {
                if gnu_optind < argc {
                    let fresh1 = gnu_optind;
                    gnu_optind = gnu_optind + 1;
                    gnu_optarg = *argv.offset(fresh1 as isize);
                } else {
                    if gnu_opterr != 0 {
                        fprintf(
                            stderr,
                            b"%s: option `%s' requires an argument\n\0" as *const u8
                                as *const libc::c_char,
                            *argv.offset(0 as libc::c_int as isize),
                            *argv.offset((gnu_optind - 1 as libc::c_int) as isize),
                        );
                    }
                    nextchar = nextchar.offset(strlen(nextchar) as isize);
                    return '?' as i32;
                }
            }
            nextchar = nextchar.offset(strlen(nextchar) as isize);
            if !longind.is_null() {
                *longind = option_index;
            }
            if !((*pfound).flag).is_null() {
                *(*pfound).flag = (*pfound).val;
                return 0 as libc::c_int;
            }
            return (*pfound).val;
        }
        if long_only == 0
            || *(*argv.offset(gnu_optind as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int == '-' as i32
            || (strchr(optstring, *nextchar as libc::c_int)).is_null()
        {
            if gnu_opterr != 0 {
                if *(*argv.offset(gnu_optind as isize)).offset(1 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
                {
                    fprintf(
                        stderr,
                        b"%s: unrecognized option `--%s'\n\0" as *const u8
                            as *const libc::c_char,
                        *argv.offset(0 as libc::c_int as isize),
                        nextchar,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"%s: unrecognized option `%c%s'\n\0" as *const u8
                            as *const libc::c_char,
                        *argv.offset(0 as libc::c_int as isize),
                        *(*argv.offset(gnu_optind as isize))
                            .offset(0 as libc::c_int as isize) as libc::c_int,
                        nextchar,
                    );
                }
            }
            nextchar = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            gnu_optind += 1;
            gnu_optind;
            return '?' as i32;
        }
    }
    let fresh2 = nextchar;
    nextchar = nextchar.offset(1);
    let mut c: libc::c_char = *fresh2;
    let mut temp: *mut libc::c_char = strchr(optstring, c as libc::c_int);
    if *nextchar as libc::c_int == '\0' as i32 {
        gnu_optind += 1;
        gnu_optind;
    }
    if temp.is_null() || c as libc::c_int == ':' as i32 {
        if gnu_opterr != 0 {
            if (c as libc::c_int) < 0o40 as libc::c_int
                || c as libc::c_int >= 0o177 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"%s: unrecognized option, character code 0%o\n\0" as *const u8
                        as *const libc::c_char,
                    *argv.offset(0 as libc::c_int as isize),
                    c as libc::c_uchar as libc::c_int,
                );
            } else {
                fprintf(
                    stderr,
                    b"%s: unrecognized option `-%c'\n\0" as *const u8
                        as *const libc::c_char,
                    *argv.offset(0 as libc::c_int as isize),
                    c as libc::c_int,
                );
            }
        }
        return '?' as i32;
    }
    if *temp.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32 {
        if *temp.offset(2 as libc::c_int as isize) as libc::c_int == ':' as i32 {
            if *nextchar as libc::c_int != '\0' as i32 {
                gnu_optarg = nextchar;
                gnu_optind += 1;
                gnu_optind;
            } else {
                gnu_optarg = 0 as *mut libc::c_char;
            }
            nextchar = 0 as *mut libc::c_char;
        } else {
            if *nextchar as libc::c_int != '\0' as i32 {
                gnu_optarg = nextchar;
                gnu_optind += 1;
                gnu_optind;
            } else if gnu_optind == argc {
                if gnu_opterr != 0 {
                    fprintf(
                        stderr,
                        b"%s: option `-%c' requires an argument\n\0" as *const u8
                            as *const libc::c_char,
                        *argv.offset(0 as libc::c_int as isize),
                        c as libc::c_int,
                    );
                }
                c = '?' as i32 as libc::c_char;
            } else {
                let fresh3 = gnu_optind;
                gnu_optind = gnu_optind + 1;
                gnu_optarg = *argv.offset(fresh3 as isize);
            }
            nextchar = 0 as *mut libc::c_char;
        }
    }
    return c as libc::c_int;
}
pub unsafe extern "C" fn gnu_getopt(
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
    mut optstring: *const libc::c_char,
) -> libc::c_int {
    return _getopt_internal(
        argc,
        argv,
        optstring,
        0 as *const option,
        0 as *mut libc::c_int,
        0 as libc::c_int,
    );
}
