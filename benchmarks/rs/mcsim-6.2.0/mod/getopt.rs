use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn ReportError(_: PINPUTBUF, _: WORD, _: PSTR, _: PSTR);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
pub type PSTR = *mut libc::c_char;
pub type WORD = libc::c_uint;
pub type PINPUTBUF = *mut tagINPUTBUF;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagINPUTBUF {
    pub pfileIn: PFILE,
    pub pbufOrg: PBUF,
    pub lBufSize: libc::c_long,
    pub pbufCur: PBUF,
    pub iLineNum: libc::c_int,
    pub iLNPrev: libc::c_int,
    pub cErrors: libc::c_int,
    pub pInfo: PVOID,
    pub pTempInfo: PVOID,
}
pub type PVOID = *mut libc::c_void;
pub type PBUF = PSTR;
pub type PFILE = *mut FILE;
pub static mut optarg: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut optind: libc::c_int = 0 as libc::c_int;
static mut nextchar: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut opterr: libc::c_int = 1 as libc::c_int;
static mut ordering: C2RustUnnamed = REQUIRE_ORDER;
unsafe extern "C" fn my_index(
    mut string: *mut libc::c_char,
    mut chr: libc::c_int,
) -> *mut libc::c_char {
    while *string != 0 {
        if *string as libc::c_int == chr {
            return string;
        }
        string = string.offset(1);
        string;
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn my_bcopy(
    mut from: *mut libc::c_char,
    mut to: *mut libc::c_char,
    mut size: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *to.offset(i as isize) = *from.offset(i as isize);
        i += 1;
        i;
    }
}
static mut first_nonopt: libc::c_int = 0;
static mut last_nonopt: libc::c_int = 0;
unsafe extern "C" fn exchange(mut argv: *mut *mut libc::c_char) {
    let mut nonopts_size: libc::c_int = ((last_nonopt - first_nonopt) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        as libc::c_int;
    let mut temp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if nonopts_size == 0 as libc::c_int {
        printf(
            b"Error: zero length array allocation in exchange - Exiting\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    temp = malloc(nonopts_size as libc::c_ulong) as *mut *mut libc::c_char;
    if temp.is_null() {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"exchange\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    my_bcopy(
        &mut *(*argv.offset(first_nonopt as isize)).offset(0 as libc::c_int as isize),
        *temp.offset(0 as libc::c_int as isize),
        nonopts_size,
    );
    my_bcopy(
        &mut *(*argv.offset(last_nonopt as isize)).offset(0 as libc::c_int as isize),
        &mut *(*argv.offset(first_nonopt as isize)).offset(0 as libc::c_int as isize),
        ((optind - last_nonopt) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as libc::c_int,
    );
    my_bcopy(
        *temp.offset(0 as libc::c_int as isize),
        &mut *(*argv.offset((first_nonopt + optind - last_nonopt) as isize))
            .offset(0 as libc::c_int as isize),
        nonopts_size,
    );
    first_nonopt += optind - last_nonopt;
    last_nonopt = optind;
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
    optarg = 0 as *mut libc::c_char;
    if optind == 0 as libc::c_int {
        optind = 1 as libc::c_int;
        last_nonopt = optind;
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
            if first_nonopt != last_nonopt && last_nonopt != optind {
                exchange(argv as *mut *mut libc::c_char);
            } else if last_nonopt != optind {
                first_nonopt = optind;
            }
            while optind < argc
                && (*(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int != '-' as i32
                    || *(*argv.offset(optind as isize)).offset(1 as libc::c_int as isize)
                        as libc::c_int == '\0' as i32)
            {
                optind += 1;
                optind;
            }
            last_nonopt = optind;
        }
        if optind != argc
            && strcmp(
                *argv.offset(optind as isize),
                b"--\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            optind += 1;
            optind;
            if first_nonopt != last_nonopt && last_nonopt != optind {
                exchange(argv as *mut *mut libc::c_char);
            } else if first_nonopt == last_nonopt {
                first_nonopt = optind;
            }
            last_nonopt = argc;
            optind = argc;
        }
        if optind == argc {
            if first_nonopt != last_nonopt {
                optind = first_nonopt;
            }
            return -(1 as libc::c_int);
        }
        if *(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int != '-' as i32
            || *(*argv.offset(optind as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int == '\0' as i32
        {
            if ordering as libc::c_uint == REQUIRE_ORDER as libc::c_int as libc::c_uint {
                return -(1 as libc::c_int);
            }
            let fresh0 = optind;
            optind = optind + 1;
            optarg = *argv.offset(fresh0 as isize);
            return 1 as libc::c_int;
        }
        nextchar = (*argv.offset(optind as isize))
            .offset(1 as libc::c_int as isize)
            .offset(
                (!longopts.is_null()
                    && *(*argv.offset(optind as isize)).offset(1 as libc::c_int as isize)
                        as libc::c_int == '-' as i32) as libc::c_int as isize,
            );
    }
    if !longopts.is_null()
        && (*(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '-' as i32
            && (*(*argv.offset(optind as isize)).offset(1 as libc::c_int as isize)
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
                s.offset_from(nextchar) as libc::c_long as libc::c_ulong,
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
            if opterr != 0 {
                printf(
                    b"%s: option `%s' is ambiguous\n\0" as *const u8
                        as *const libc::c_char,
                    *argv.offset(0 as libc::c_int as isize),
                    *argv.offset(optind as isize),
                );
            }
            nextchar = nextchar.offset(strlen(nextchar) as isize);
            optind += 1;
            optind;
            return '?' as i32;
        }
        if !pfound.is_null() {
            option_index = indfound;
            optind += 1;
            optind;
            if *s != 0 {
                if (*pfound).has_arg != 0 {
                    optarg = s.offset(1 as libc::c_int as isize);
                } else {
                    if opterr != 0 {
                        if *(*argv.offset((optind - 1 as libc::c_int) as isize))
                            .offset(1 as libc::c_int as isize) as libc::c_int
                            == '-' as i32
                        {
                            printf(
                                b"%s: option `--%s' doesn't allow an argument\n\0"
                                    as *const u8 as *const libc::c_char,
                                *argv.offset(0 as libc::c_int as isize),
                                (*pfound).name,
                            );
                        } else {
                            printf(
                                b"%s: option `%c%s' doesn't allow an argument\n\0"
                                    as *const u8 as *const libc::c_char,
                                *argv.offset(0 as libc::c_int as isize),
                                *(*argv.offset((optind - 1 as libc::c_int) as isize))
                                    .offset(0 as libc::c_int as isize) as libc::c_int,
                                (*pfound).name,
                            );
                        }
                    }
                    nextchar = nextchar.offset(strlen(nextchar) as isize);
                    return '?' as i32;
                }
            } else if (*pfound).has_arg == 1 as libc::c_int {
                if optind < argc {
                    let fresh1 = optind;
                    optind = optind + 1;
                    optarg = *argv.offset(fresh1 as isize);
                } else {
                    if opterr != 0 {
                        printf(
                            b"%s: option `%s' requires an argument\n\0" as *const u8
                                as *const libc::c_char,
                            *argv.offset(0 as libc::c_int as isize),
                            *argv.offset((optind - 1 as libc::c_int) as isize),
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
            || *(*argv.offset(optind as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int == '-' as i32
            || (my_index(optstring as *mut libc::c_char, *nextchar as libc::c_int))
                .is_null()
        {
            if opterr != 0 {
                if *(*argv.offset(optind as isize)).offset(1 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
                {
                    printf(
                        b"%s: unrecognized option `--%s'\n\0" as *const u8
                            as *const libc::c_char,
                        *argv.offset(0 as libc::c_int as isize),
                        nextchar,
                    );
                } else {
                    printf(
                        b"%s: unrecognized option `%c%s'\n\0" as *const u8
                            as *const libc::c_char,
                        *argv.offset(0 as libc::c_int as isize),
                        *(*argv.offset(optind as isize))
                            .offset(0 as libc::c_int as isize) as libc::c_int,
                        nextchar,
                    );
                }
            }
            nextchar = nextchar.offset(strlen(nextchar) as isize);
            optind += 1;
            optind;
            return '?' as i32;
        }
    }
    let fresh2 = nextchar;
    nextchar = nextchar.offset(1);
    let mut c: libc::c_char = *fresh2;
    let mut temp: *mut libc::c_char = my_index(
        optstring as *mut libc::c_char,
        c as libc::c_int,
    );
    if *nextchar as libc::c_int == '\0' as i32 {
        optind += 1;
        optind;
    }
    if temp.is_null() || c as libc::c_int == ':' as i32 {
        if opterr != 0 {
            if (c as libc::c_int) < 0o40 as libc::c_int
                || c as libc::c_int >= 0o177 as libc::c_int
            {
                printf(
                    b"%s: unrecognized option, character code 0%o\n\0" as *const u8
                        as *const libc::c_char,
                    *argv.offset(0 as libc::c_int as isize),
                    c as libc::c_int,
                );
            } else {
                printf(
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
                optarg = nextchar;
                optind += 1;
                optind;
            } else {
                optarg = 0 as *mut libc::c_char;
            }
            nextchar = 0 as *mut libc::c_char;
        } else {
            if *nextchar as libc::c_int != 0 as libc::c_int {
                optarg = nextchar;
                optind += 1;
                optind;
            } else if optind == argc {
                if opterr != 0 {
                    printf(
                        b"%s: option `-%c' requires an argument\n\0" as *const u8
                            as *const libc::c_char,
                        *argv.offset(0 as libc::c_int as isize),
                        c as libc::c_int,
                    );
                }
                c = '?' as i32 as libc::c_char;
            } else {
                let fresh3 = optind;
                optind = optind + 1;
                optarg = *argv.offset(fresh3 as isize);
            }
            nextchar = 0 as *mut libc::c_char;
        }
    }
    return c as libc::c_int;
}
pub unsafe extern "C" fn _getopt(
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
