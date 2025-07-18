use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _getopt_data {
    pub optind: libc::c_int,
    pub opterr: libc::c_int,
    pub optopt: libc::c_int,
    pub optarg: *mut libc::c_char,
    pub __initialized: libc::c_int,
    pub __nextchar: *mut libc::c_char,
    pub __ordering: C2RustUnnamed,
    pub __posixly_correct: libc::c_int,
    pub __first_nonopt: libc::c_int,
    pub __last_nonopt: libc::c_int,
}
pub type C2RustUnnamed = libc::c_uint;
pub const RETURN_IN_ORDER: C2RustUnnamed = 2;
pub const PERMUTE: C2RustUnnamed = 1;
pub const REQUIRE_ORDER: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option_list {
    pub p: *const option,
    pub next: *mut option_list,
    pub needs_free: libc::c_int,
}
pub static mut optarg: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut optind: libc::c_int = 1 as libc::c_int;
pub static mut opterr: libc::c_int = 1 as libc::c_int;
pub static mut optopt: libc::c_int = '?' as i32;
static mut getopt_data: _getopt_data = _getopt_data {
    optind: 0,
    opterr: 0,
    optopt: 0,
    optarg: 0 as *const libc::c_char as *mut libc::c_char,
    __initialized: 0,
    __nextchar: 0 as *const libc::c_char as *mut libc::c_char,
    __ordering: REQUIRE_ORDER,
    __posixly_correct: 0,
    __first_nonopt: 0,
    __last_nonopt: 0,
};
unsafe extern "C" fn exchange(
    mut argv: *mut *mut libc::c_char,
    mut d: *mut _getopt_data,
) {
    let mut bottom: libc::c_int = (*d).__first_nonopt;
    let mut middle: libc::c_int = (*d).__last_nonopt;
    let mut top: libc::c_int = (*d).optind;
    let mut tem: *mut libc::c_char = 0 as *mut libc::c_char;
    while top > middle && middle > bottom {
        if top - middle > middle - bottom {
            let mut len: libc::c_int = middle - bottom;
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < len {
                tem = *argv.offset((bottom + i) as isize);
                let ref mut fresh0 = *argv.offset((bottom + i) as isize);
                *fresh0 = *argv.offset((top - (middle - bottom) + i) as isize);
                let ref mut fresh1 = *argv
                    .offset((top - (middle - bottom) + i) as isize);
                *fresh1 = tem;
                i += 1;
                i;
            }
            top -= len;
        } else {
            let mut len_0: libc::c_int = top - middle;
            let mut i_0: libc::c_int = 0;
            i_0 = 0 as libc::c_int;
            while i_0 < len_0 {
                tem = *argv.offset((bottom + i_0) as isize);
                let ref mut fresh2 = *argv.offset((bottom + i_0) as isize);
                *fresh2 = *argv.offset((middle + i_0) as isize);
                let ref mut fresh3 = *argv.offset((middle + i_0) as isize);
                *fresh3 = tem;
                i_0 += 1;
                i_0;
            }
            bottom += len_0;
        }
    }
    (*d).__first_nonopt += (*d).optind - (*d).__last_nonopt;
    (*d).__last_nonopt = (*d).optind;
}
unsafe extern "C" fn _getopt_initialize(
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
    mut optstring: *const libc::c_char,
    mut d: *mut _getopt_data,
    mut posixly_correct: libc::c_int,
) -> *const libc::c_char {
    (*d).__last_nonopt = (*d).optind;
    (*d).__first_nonopt = (*d).__last_nonopt;
    (*d).__nextchar = 0 as *mut libc::c_char;
    (*d)
        .__posixly_correct = posixly_correct
        | !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char)).is_null()
            as libc::c_int;
    if *optstring.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
        (*d).__ordering = RETURN_IN_ORDER;
        optstring = optstring.offset(1);
        optstring;
    } else if *optstring.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32 {
        (*d).__ordering = REQUIRE_ORDER;
        optstring = optstring.offset(1);
        optstring;
    } else if (*d).__posixly_correct != 0 {
        (*d).__ordering = REQUIRE_ORDER;
    } else {
        (*d).__ordering = PERMUTE;
    }
    return optstring;
}
pub unsafe extern "C" fn _getopt_internal_r(
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
    mut optstring: *const libc::c_char,
    mut longopts: *const option,
    mut longind: *mut libc::c_int,
    mut long_only: libc::c_int,
    mut d: *mut _getopt_data,
    mut posixly_correct: libc::c_int,
) -> libc::c_int {
    let mut print_errors: libc::c_int = (*d).opterr;
    if argc < 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*d).optarg = 0 as *mut libc::c_char;
    if (*d).optind == 0 as libc::c_int || (*d).__initialized == 0 {
        if (*d).optind == 0 as libc::c_int {
            (*d).optind = 1 as libc::c_int;
        }
        optstring = _getopt_initialize(argc, argv, optstring, d, posixly_correct);
        (*d).__initialized = 1 as libc::c_int;
    } else if *optstring.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
        || *optstring.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
    {
        optstring = optstring.offset(1);
        optstring;
    }
    if *optstring.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32 {
        print_errors = 0 as libc::c_int;
    }
    if ((*d).__nextchar).is_null() || *(*d).__nextchar as libc::c_int == '\0' as i32 {
        if (*d).__last_nonopt > (*d).optind {
            (*d).__last_nonopt = (*d).optind;
        }
        if (*d).__first_nonopt > (*d).optind {
            (*d).__first_nonopt = (*d).optind;
        }
        if (*d).__ordering as libc::c_uint == PERMUTE as libc::c_int as libc::c_uint {
            if (*d).__first_nonopt != (*d).__last_nonopt
                && (*d).__last_nonopt != (*d).optind
            {
                exchange(argv as *mut *mut libc::c_char, d);
            } else if (*d).__last_nonopt != (*d).optind {
                (*d).__first_nonopt = (*d).optind;
            }
            while (*d).optind < argc
                && (*(*argv.offset((*d).optind as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
                    || *(*argv.offset((*d).optind as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32)
            {
                (*d).optind += 1;
                (*d).optind;
            }
            (*d).__last_nonopt = (*d).optind;
        }
        if (*d).optind != argc
            && strcmp(
                *argv.offset((*d).optind as isize),
                b"--\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            (*d).optind += 1;
            (*d).optind;
            if (*d).__first_nonopt != (*d).__last_nonopt
                && (*d).__last_nonopt != (*d).optind
            {
                exchange(argv as *mut *mut libc::c_char, d);
            } else if (*d).__first_nonopt == (*d).__last_nonopt {
                (*d).__first_nonopt = (*d).optind;
            }
            (*d).__last_nonopt = argc;
            (*d).optind = argc;
        }
        if (*d).optind == argc {
            if (*d).__first_nonopt != (*d).__last_nonopt {
                (*d).optind = (*d).__first_nonopt;
            }
            return -(1 as libc::c_int);
        }
        if *(*argv.offset((*d).optind as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int != '-' as i32
            || *(*argv.offset((*d).optind as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int == '\0' as i32
        {
            if (*d).__ordering as libc::c_uint
                == REQUIRE_ORDER as libc::c_int as libc::c_uint
            {
                return -(1 as libc::c_int);
            }
            let fresh4 = (*d).optind;
            (*d).optind = (*d).optind + 1;
            (*d).optarg = *argv.offset(fresh4 as isize);
            return 1 as libc::c_int;
        }
        (*d)
            .__nextchar = (*argv.offset((*d).optind as isize))
            .offset(1 as libc::c_int as isize)
            .offset(
                (!longopts.is_null()
                    && *(*argv.offset((*d).optind as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32)
                    as libc::c_int as isize,
            );
    }
    if !longopts.is_null()
        && (*(*argv.offset((*d).optind as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int == '-' as i32
            || long_only != 0
                && (*(*argv.offset((*d).optind as isize))
                    .offset(2 as libc::c_int as isize) as libc::c_int != 0
                    || (strchr(
                        optstring,
                        *(*argv.offset((*d).optind as isize))
                            .offset(1 as libc::c_int as isize) as libc::c_int,
                    ))
                        .is_null()))
    {
        let mut nameend: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut namelen: libc::c_uint = 0;
        let mut p: *const option = 0 as *const option;
        let mut pfound: *const option = 0 as *const option;
        let mut ambig_list: *mut option_list = 0 as *mut option_list;
        let mut exact: libc::c_int = 0 as libc::c_int;
        let mut indfound: libc::c_int = -(1 as libc::c_int);
        let mut option_index: libc::c_int = 0;
        nameend = (*d).__nextchar;
        while *nameend as libc::c_int != 0 && *nameend as libc::c_int != '=' as i32 {
            nameend = nameend.offset(1);
            nameend;
        }
        namelen = nameend.offset_from((*d).__nextchar) as libc::c_long as libc::c_uint;
        p = longopts;
        option_index = 0 as libc::c_int;
        while !((*p).name).is_null() {
            if strncmp((*p).name, (*d).__nextchar, namelen as libc::c_ulong) == 0 {
                if namelen == strlen((*p).name) as libc::c_uint {
                    pfound = p;
                    indfound = option_index;
                    exact = 1 as libc::c_int;
                    break;
                } else if pfound.is_null() {
                    pfound = p;
                    indfound = option_index;
                } else if long_only != 0 || (*pfound).has_arg != (*p).has_arg
                    || (*pfound).flag != (*p).flag || (*pfound).val != (*p).val
                {
                    let mut newp: *mut option_list = malloc(
                        ::std::mem::size_of::<option_list>() as libc::c_ulong,
                    ) as *mut option_list;
                    (*newp).p = p;
                    (*newp).needs_free = 1 as libc::c_int;
                    (*newp).next = ambig_list;
                    ambig_list = newp;
                }
            }
            p = p.offset(1);
            p;
            option_index += 1;
            option_index;
        }
        if !ambig_list.is_null() && exact == 0 {
            if print_errors != 0 {
                let mut first: option_list = option_list {
                    p: 0 as *const option,
                    next: 0 as *mut option_list,
                    needs_free: 0,
                };
                first.p = pfound;
                first.next = ambig_list;
                first.needs_free = 0 as libc::c_int;
                ambig_list = &mut first;
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: option '%s' is ambiguous; possibilities:\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *argv.offset(0 as libc::c_int as isize),
                    *argv.offset((*d).optind as isize),
                );
                loop {
                    let mut tmp_next: *mut option_list = 0 as *mut option_list;
                    fprintf(
                        stderr,
                        b" '--%s'\0" as *const u8 as *const libc::c_char,
                        (*(*ambig_list).p).name,
                    );
                    tmp_next = (*ambig_list).next;
                    if (*ambig_list).needs_free != 0 {
                        free(ambig_list as *mut libc::c_void);
                    }
                    ambig_list = tmp_next;
                    if ambig_list.is_null() {
                        break;
                    }
                }
                fputc('\n' as i32, stderr);
            }
            (*d).__nextchar = ((*d).__nextchar).offset(strlen((*d).__nextchar) as isize);
            (*d).optind += 1;
            (*d).optind;
            (*d).optopt = 0 as libc::c_int;
            return '?' as i32;
        }
        if !pfound.is_null() {
            option_index = indfound;
            (*d).optind += 1;
            (*d).optind;
            if *nameend != 0 {
                if (*pfound).has_arg != 0 {
                    (*d).optarg = nameend.offset(1 as libc::c_int as isize);
                } else {
                    if print_errors != 0 {
                        if *(*argv.offset(((*d).optind - 1 as libc::c_int) as isize))
                            .offset(1 as libc::c_int as isize) as libc::c_int
                            == '-' as i32
                        {
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%s: option '--%s' doesn't allow an argument\n\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                *argv.offset(0 as libc::c_int as isize),
                                (*pfound).name,
                            );
                        } else {
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%s: option '%c%s' doesn't allow an argument\n\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                *argv.offset(0 as libc::c_int as isize),
                                *(*argv.offset(((*d).optind - 1 as libc::c_int) as isize))
                                    .offset(0 as libc::c_int as isize) as libc::c_int,
                                (*pfound).name,
                            );
                        }
                    }
                    (*d)
                        .__nextchar = ((*d).__nextchar)
                        .offset(strlen((*d).__nextchar) as isize);
                    (*d).optopt = (*pfound).val;
                    return '?' as i32;
                }
            } else if (*pfound).has_arg == 1 as libc::c_int {
                if (*d).optind < argc {
                    let fresh5 = (*d).optind;
                    (*d).optind = (*d).optind + 1;
                    (*d).optarg = *argv.offset(fresh5 as isize);
                } else {
                    if print_errors != 0 {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s: option '--%s' requires an argument\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            *argv.offset(0 as libc::c_int as isize),
                            (*pfound).name,
                        );
                    }
                    (*d)
                        .__nextchar = ((*d).__nextchar)
                        .offset(strlen((*d).__nextchar) as isize);
                    (*d).optopt = (*pfound).val;
                    return if *optstring.offset(0 as libc::c_int as isize) as libc::c_int
                        == ':' as i32
                    {
                        ':' as i32
                    } else {
                        '?' as i32
                    };
                }
            }
            (*d).__nextchar = ((*d).__nextchar).offset(strlen((*d).__nextchar) as isize);
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
            || *(*argv.offset((*d).optind as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int == '-' as i32
            || (strchr(optstring, *(*d).__nextchar as libc::c_int)).is_null()
        {
            if print_errors != 0 {
                if *(*argv.offset((*d).optind as isize))
                    .offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
                {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: unrecognized option '--%s'\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *argv.offset(0 as libc::c_int as isize),
                        (*d).__nextchar,
                    );
                } else {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: unrecognized option '%c%s'\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *argv.offset(0 as libc::c_int as isize),
                        *(*argv.offset((*d).optind as isize))
                            .offset(0 as libc::c_int as isize) as libc::c_int,
                        (*d).__nextchar,
                    );
                }
            }
            (*d)
                .__nextchar = b"\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            (*d).optind += 1;
            (*d).optind;
            (*d).optopt = 0 as libc::c_int;
            return '?' as i32;
        }
    }
    let fresh6 = (*d).__nextchar;
    (*d).__nextchar = ((*d).__nextchar).offset(1);
    let mut c: libc::c_char = *fresh6;
    let mut temp: *mut libc::c_char = strchr(optstring, c as libc::c_int);
    if *(*d).__nextchar as libc::c_int == '\0' as i32 {
        (*d).optind += 1;
        (*d).optind;
    }
    if temp.is_null() || c as libc::c_int == ':' as i32 || c as libc::c_int == ';' as i32
    {
        if print_errors != 0 {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: invalid option -- '%c'\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                *argv.offset(0 as libc::c_int as isize),
                c as libc::c_int,
            );
        }
        (*d).optopt = c as libc::c_int;
        return '?' as i32;
    }
    if *temp.offset(0 as libc::c_int as isize) as libc::c_int == 'W' as i32
        && *temp.offset(1 as libc::c_int as isize) as libc::c_int == ';' as i32
    {
        let mut nameend_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p_0: *const option = 0 as *const option;
        let mut pfound_0: *const option = 0 as *const option;
        let mut exact_0: libc::c_int = 0 as libc::c_int;
        let mut ambig: libc::c_int = 0 as libc::c_int;
        let mut indfound_0: libc::c_int = 0 as libc::c_int;
        let mut option_index_0: libc::c_int = 0;
        if !longopts.is_null() {
            if *(*d).__nextchar as libc::c_int != '\0' as i32 {
                (*d).optarg = (*d).__nextchar;
                (*d).optind += 1;
                (*d).optind;
            } else if (*d).optind == argc {
                if print_errors != 0 {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: option requires an argument -- '%c'\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *argv.offset(0 as libc::c_int as isize),
                        c as libc::c_int,
                    );
                }
                (*d).optopt = c as libc::c_int;
                if *optstring.offset(0 as libc::c_int as isize) as libc::c_int
                    == ':' as i32
                {
                    c = ':' as i32 as libc::c_char;
                } else {
                    c = '?' as i32 as libc::c_char;
                }
                return c as libc::c_int;
            } else {
                let fresh7 = (*d).optind;
                (*d).optind = (*d).optind + 1;
                (*d).optarg = *argv.offset(fresh7 as isize);
            }
            nameend_0 = (*d).optarg;
            (*d).__nextchar = nameend_0;
            while *nameend_0 as libc::c_int != 0
                && *nameend_0 as libc::c_int != '=' as i32
            {
                nameend_0 = nameend_0.offset(1);
                nameend_0;
            }
            p_0 = longopts;
            option_index_0 = 0 as libc::c_int;
            while !((*p_0).name).is_null() {
                if strncmp(
                    (*p_0).name,
                    (*d).__nextchar,
                    nameend_0.offset_from((*d).__nextchar) as libc::c_long
                        as libc::c_ulong,
                ) == 0
                {
                    if nameend_0.offset_from((*d).__nextchar) as libc::c_long
                        as libc::c_uint as libc::c_ulong == strlen((*p_0).name)
                    {
                        pfound_0 = p_0;
                        indfound_0 = option_index_0;
                        exact_0 = 1 as libc::c_int;
                        break;
                    } else if pfound_0.is_null() {
                        pfound_0 = p_0;
                        indfound_0 = option_index_0;
                    } else if long_only != 0 || (*pfound_0).has_arg != (*p_0).has_arg
                        || (*pfound_0).flag != (*p_0).flag
                        || (*pfound_0).val != (*p_0).val
                    {
                        ambig = 1 as libc::c_int;
                    }
                }
                p_0 = p_0.offset(1);
                p_0;
                option_index_0 += 1;
                option_index_0;
            }
            if ambig != 0 && exact_0 == 0 {
                if print_errors != 0 {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: option '-W %s' is ambiguous\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *argv.offset(0 as libc::c_int as isize),
                        (*d).optarg,
                    );
                }
                (*d)
                    .__nextchar = ((*d).__nextchar)
                    .offset(strlen((*d).__nextchar) as isize);
                (*d).optind += 1;
                (*d).optind;
                return '?' as i32;
            }
            if !pfound_0.is_null() {
                option_index_0 = indfound_0;
                if *nameend_0 != 0 {
                    if (*pfound_0).has_arg != 0 {
                        (*d).optarg = nameend_0.offset(1 as libc::c_int as isize);
                    } else {
                        if print_errors != 0 {
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%s: option '-W %s' doesn't allow an argument\n\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                *argv.offset(0 as libc::c_int as isize),
                                (*pfound_0).name,
                            );
                        }
                        (*d)
                            .__nextchar = ((*d).__nextchar)
                            .offset(strlen((*d).__nextchar) as isize);
                        return '?' as i32;
                    }
                } else if (*pfound_0).has_arg == 1 as libc::c_int {
                    if (*d).optind < argc {
                        let fresh8 = (*d).optind;
                        (*d).optind = (*d).optind + 1;
                        (*d).optarg = *argv.offset(fresh8 as isize);
                    } else {
                        if print_errors != 0 {
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%s: option '-W %s' requires an argument\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                *argv.offset(0 as libc::c_int as isize),
                                (*pfound_0).name,
                            );
                        }
                        (*d)
                            .__nextchar = ((*d).__nextchar)
                            .offset(strlen((*d).__nextchar) as isize);
                        return if *optstring.offset(0 as libc::c_int as isize)
                            as libc::c_int == ':' as i32
                        {
                            ':' as i32
                        } else {
                            '?' as i32
                        };
                    }
                } else {
                    (*d).optarg = 0 as *mut libc::c_char;
                }
                (*d)
                    .__nextchar = ((*d).__nextchar)
                    .offset(strlen((*d).__nextchar) as isize);
                if !longind.is_null() {
                    *longind = option_index_0;
                }
                if !((*pfound_0).flag).is_null() {
                    *(*pfound_0).flag = (*pfound_0).val;
                    return 0 as libc::c_int;
                }
                return (*pfound_0).val;
            }
        }
        (*d).__nextchar = 0 as *mut libc::c_char;
        return 'W' as i32;
    }
    if *temp.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32 {
        if *temp.offset(2 as libc::c_int as isize) as libc::c_int == ':' as i32 {
            if *(*d).__nextchar as libc::c_int != '\0' as i32 {
                (*d).optarg = (*d).__nextchar;
                (*d).optind += 1;
                (*d).optind;
            } else {
                (*d).optarg = 0 as *mut libc::c_char;
            }
            (*d).__nextchar = 0 as *mut libc::c_char;
        } else {
            if *(*d).__nextchar as libc::c_int != '\0' as i32 {
                (*d).optarg = (*d).__nextchar;
                (*d).optind += 1;
                (*d).optind;
            } else if (*d).optind == argc {
                if print_errors != 0 {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: option requires an argument -- '%c'\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *argv.offset(0 as libc::c_int as isize),
                        c as libc::c_int,
                    );
                }
                (*d).optopt = c as libc::c_int;
                if *optstring.offset(0 as libc::c_int as isize) as libc::c_int
                    == ':' as i32
                {
                    c = ':' as i32 as libc::c_char;
                } else {
                    c = '?' as i32 as libc::c_char;
                }
            } else {
                let fresh9 = (*d).optind;
                (*d).optind = (*d).optind + 1;
                (*d).optarg = *argv.offset(fresh9 as isize);
            }
            (*d).__nextchar = 0 as *mut libc::c_char;
        }
    }
    return c as libc::c_int;
}
pub unsafe extern "C" fn _getopt_internal(
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
    mut optstring: *const libc::c_char,
    mut longopts: *const option,
    mut longind: *mut libc::c_int,
    mut long_only: libc::c_int,
    mut posixly_correct: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    getopt_data.optind = optind;
    getopt_data.opterr = opterr;
    result = _getopt_internal_r(
        argc,
        argv,
        optstring,
        longopts,
        longind,
        long_only,
        &mut getopt_data,
        posixly_correct,
    );
    optind = getopt_data.optind;
    optarg = getopt_data.optarg;
    optopt = getopt_data.optopt;
    return result;
}
pub unsafe extern "C" fn getopt(
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
        0 as libc::c_int,
    );
}
