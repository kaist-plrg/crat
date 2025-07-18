use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type variable_set_list;
    pub type commands;
    fn free(__ptr: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn concat(_: libc::c_uint, _: ...) -> *const libc::c_char;
    fn error(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...) -> !;
    fn perror_with_name(_: *const libc::c_char, _: *const libc::c_char);
    fn xcalloc(_: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn alpha_compare(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int;
    fn ar_member_touch(
        arname: *const libc::c_char,
        memname: *const libc::c_char,
    ) -> libc::c_int;
    fn strcache_add(str: *const libc::c_char) -> *const libc::c_char;
    fn ar_name_equal(
        name: *const libc::c_char,
        mem: *const libc::c_char,
        truncated: libc::c_int,
    ) -> libc::c_int;
    fn ar_scan(
        archive: *const libc::c_char,
        function: ar_member_func_t,
        arg: *const libc::c_void,
    ) -> intmax_t;
    fn file_exists_p(_: *const libc::c_char) -> libc::c_int;
    fn lookup_file(name: *const libc::c_char) -> *mut file;
    fn enter_file(name: *const libc::c_char) -> *mut file;
    fn f_mtime(file: *mut file, search: libc::c_int) -> uintmax_t;
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct file {
    pub name: *const libc::c_char,
    pub hname: *const libc::c_char,
    pub vpath: *const libc::c_char,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub stem: *const libc::c_char,
    pub also_make: *mut dep,
    pub prev: *mut file,
    pub last: *mut file,
    pub renamed: *mut file,
    pub variables: *mut variable_set_list,
    pub pat_variables: *mut variable_set_list,
    pub parent: *mut file,
    pub double_colon: *mut file,
    pub last_mtime: uintmax_t,
    pub mtime_before_update: uintmax_t,
    pub considered: libc::c_uint,
    pub command_flags: libc::c_int,
    #[bitfield(name = "update_status", ty = "update_status", bits = "0..=1")]
    #[bitfield(name = "command_state", ty = "cmd_state", bits = "2..=3")]
    #[bitfield(name = "builtin", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "precious", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "loaded", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "unloaded", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "low_resolution_time", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "tried_implicit", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "updating", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "updated", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "is_target", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "cmd_target", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "phony", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "intermediate", ty = "libc::c_uint", bits = "15..=15")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "16..=16")]
    #[bitfield(name = "secondary", ty = "libc::c_uint", bits = "17..=17")]
    #[bitfield(name = "notintermediate", ty = "libc::c_uint", bits = "18..=18")]
    #[bitfield(name = "dontcare", ty = "libc::c_uint", bits = "19..=19")]
    #[bitfield(name = "ignore_vpath", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "pat_searched", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "no_diag", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "was_shuffled", ty = "libc::c_uint", bits = "23..=23")]
    #[bitfield(name = "snapped", ty = "libc::c_uint", bits = "24..=24")]
    pub update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub type cmd_state = libc::c_uint;
pub const cs_finished: cmd_state = 3;
pub const cs_running: cmd_state = 2;
pub const cs_deps_running: cmd_state = 1;
pub const cs_not_started: cmd_state = 0;
pub type update_status = libc::c_uint;
pub const us_failed: update_status = 3;
pub const us_question: update_status = 2;
pub const us_none: update_status = 1;
pub const us_success: update_status = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct dep {
    pub next: *mut dep,
    pub name: *const libc::c_char,
    pub file: *mut file,
    pub shuf: *mut dep,
    pub stem: *const libc::c_char,
    #[bitfield(name = "flags", ty = "libc::c_uint", bits = "0..=7")]
    #[bitfield(name = "changed", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "ignore_mtime", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "staticpattern", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "need_2nd_expansion", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "ignore_automatic_vars", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "wait_here", ty = "libc::c_uint", bits = "14..=14")]
    pub flags_changed_ignore_mtime_staticpattern_need_2nd_expansion_ignore_automatic_vars_is_explicit_wait_here: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const libc::c_char,
    pub lineno: libc::c_ulong,
    pub offset: libc::c_ulong,
}
pub type ar_member_func_t = Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *const libc::c_char,
        libc::c_int,
        libc::c_long,
        libc::c_long,
        libc::c_long,
        intmax_t,
        libc::c_int,
        libc::c_int,
        libc::c_uint,
        *const libc::c_void,
    ) -> intmax_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ar_glob_state {
    pub arname: *const libc::c_char,
    pub pattern: *const libc::c_char,
    pub size: size_t,
    pub chain: *mut nameseq,
    pub n: libc::c_uint,
}
pub unsafe extern "C" fn ar_name(mut name: *const libc::c_char) -> libc::c_int {
    let mut p: *const libc::c_char = strchr(name, '(' as i32);
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    if p.is_null() || p == name {
        return 0 as libc::c_int;
    }
    end = p.offset(strlen(p) as isize).offset(-(1 as libc::c_int as isize));
    if *end as libc::c_int != ')' as i32 || end == p.offset(1 as libc::c_int as isize) {
        return 0 as libc::c_int;
    }
    if *p.offset(1 as libc::c_int as isize) as libc::c_int == '(' as i32
        && *end.offset(-(1 as libc::c_int) as isize) as libc::c_int == ')' as i32
    {
        fatal(
            0 as *mut floc,
            strlen(name),
            dcgettext(
                0 as *const libc::c_char,
                b"attempt to use unsupported feature: '%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn ar_parse_name(
    mut name: *const libc::c_char,
    mut arname_p: *mut *mut libc::c_char,
    mut memname_p: *mut *mut libc::c_char,
) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    *arname_p = xstrdup(name);
    p = strchr(*arname_p, '(' as i32);
    if p.is_null() {
        fatal(
            0 as *mut floc,
            strlen(*arname_p),
            b"Internal: ar_parse_name: bad name '%s'\0" as *const u8
                as *const libc::c_char,
            *arname_p,
        );
    }
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = '\0' as i32 as libc::c_char;
    *p
        .offset(
            (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = '\0' as i32 as libc::c_char;
    *memname_p = p;
}
unsafe extern "C" fn ar_member_date_1(
    mut desc: libc::c_int,
    mut mem: *const libc::c_char,
    mut truncated: libc::c_int,
    mut hdrpos: libc::c_long,
    mut datapos: libc::c_long,
    mut size: libc::c_long,
    mut date: intmax_t,
    mut uid: libc::c_int,
    mut gid: libc::c_int,
    mut mode: libc::c_uint,
    mut name: *const libc::c_void,
) -> intmax_t {
    return if ar_name_equal(name as *const libc::c_char, mem, truncated) != 0 {
        date
    } else {
        0 as libc::c_int as libc::c_long
    };
}
pub unsafe extern "C" fn ar_member_date(mut name: *const libc::c_char) -> time_t {
    let mut arname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut memname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: intmax_t = 0;
    ar_parse_name(name, &mut arname, &mut memname);
    let mut arfile: *mut file = 0 as *mut file;
    arfile = lookup_file(arname);
    if arfile.is_null() && file_exists_p(arname) != 0 {
        arfile = enter_file(strcache_add(arname));
    }
    if !arfile.is_null() {
        f_mtime(arfile, 0 as libc::c_int);
    }
    val = ar_scan(
        arname,
        Some(
            ar_member_date_1
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_char,
                    libc::c_int,
                    libc::c_long,
                    libc::c_long,
                    libc::c_long,
                    intmax_t,
                    libc::c_int,
                    libc::c_int,
                    libc::c_uint,
                    *const libc::c_void,
                ) -> intmax_t,
        ),
        memname as *const libc::c_void,
    );
    free(arname as *mut libc::c_void);
    return if (0 as libc::c_int as libc::c_long) < val
        && val
            <= (if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
                -(1 as libc::c_int) as time_t
            } else {
                (((1 as libc::c_int as time_t)
                    << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            })
    {
        val
    } else {
        -(1 as libc::c_int) as libc::c_long
    };
}
pub unsafe extern "C" fn ar_touch(mut name: *const libc::c_char) -> libc::c_int {
    let mut arname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut memname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: libc::c_int = 0;
    ar_parse_name(name, &mut arname, &mut memname);
    let mut arfile: *mut file = 0 as *mut file;
    arfile = enter_file(strcache_add(arname));
    f_mtime(arfile, 0 as libc::c_int);
    val = 1 as libc::c_int;
    match ar_member_touch(arname, memname) {
        -1 => {
            error(
                0 as *mut floc,
                strlen(arname),
                dcgettext(
                    0 as *const libc::c_char,
                    b"touch: Archive '%s' does not exist\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                arname,
            );
        }
        -2 => {
            error(
                0 as *mut floc,
                strlen(arname),
                dcgettext(
                    0 as *const libc::c_char,
                    b"touch: '%s' is not a valid archive\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                arname,
            );
        }
        -3 => {
            perror_with_name(b"touch: \0" as *const u8 as *const libc::c_char, arname);
        }
        1 => {
            error(
                0 as *mut floc,
                (strlen(memname)).wrapping_add(strlen(arname)),
                dcgettext(
                    0 as *const libc::c_char,
                    b"touch: Member '%s' does not exist in '%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                memname,
                arname,
            );
        }
        0 => {
            val = 0 as libc::c_int;
        }
        _ => {
            error(
                0 as *mut floc,
                strlen(name),
                dcgettext(
                    0 as *const libc::c_char,
                    b"touch: Bad return code from ar_member_touch on '%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
        }
    }
    free(arname as *mut libc::c_void);
    return val;
}
unsafe extern "C" fn ar_glob_match(
    mut desc: libc::c_int,
    mut mem: *const libc::c_char,
    mut truncated: libc::c_int,
    mut hdrpos: libc::c_long,
    mut datapos: libc::c_long,
    mut size: libc::c_long,
    mut date: intmax_t,
    mut uid: libc::c_int,
    mut gid: libc::c_int,
    mut mode: libc::c_uint,
    mut arg: *const libc::c_void,
) -> intmax_t {
    let mut state: *mut ar_glob_state = arg as *mut ar_glob_state;
    if fnmatch(
        (*state).pattern,
        mem,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
    ) == 0 as libc::c_int
    {
        let mut new: *mut nameseq = xcalloc((*state).size) as *mut nameseq;
        (*new)
            .name = strcache_add(
            concat(
                4 as libc::c_int as libc::c_uint,
                (*state).arname,
                b"(\0" as *const u8 as *const libc::c_char,
                mem,
                b")\0" as *const u8 as *const libc::c_char,
            ),
        );
        (*new).next = (*state).chain;
        (*state).chain = new;
        (*state).n = ((*state).n).wrapping_add(1);
        (*state).n;
    }
    return 0 as libc::c_int as intmax_t;
}
unsafe extern "C" fn ar_glob_pattern_p(
    mut pattern: *const libc::c_char,
    mut quote: libc::c_int,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut opened: libc::c_int = 0 as libc::c_int;
    p = pattern;
    while *p as libc::c_int != '\0' as i32 {
        match *p as libc::c_int {
            63 | 42 => return 1 as libc::c_int,
            92 => {
                if quote != 0 {
                    p = p.offset(1);
                    p;
                }
            }
            91 => {
                opened = 1 as libc::c_int;
            }
            93 => {
                if opened != 0 {
                    return 1 as libc::c_int;
                }
            }
            _ => {}
        }
        p = p.offset(1);
        p;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ar_glob(
    mut arname: *const libc::c_char,
    mut member_pattern: *const libc::c_char,
    mut size: size_t,
) -> *mut nameseq {
    let mut state: ar_glob_state = ar_glob_state {
        arname: 0 as *const libc::c_char,
        pattern: 0 as *const libc::c_char,
        size: 0,
        chain: 0 as *mut nameseq,
        n: 0,
    };
    let mut n: *mut nameseq = 0 as *mut nameseq;
    let mut names: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut i: libc::c_uint = 0;
    if ar_glob_pattern_p(member_pattern, 1 as libc::c_int) == 0 {
        return 0 as *mut nameseq;
    }
    state.arname = arname;
    state.pattern = member_pattern;
    state.size = size;
    state.chain = 0 as *mut nameseq;
    state.n = 0 as libc::c_int as libc::c_uint;
    ar_scan(
        arname,
        Some(
            ar_glob_match
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_char,
                    libc::c_int,
                    libc::c_long,
                    libc::c_long,
                    libc::c_long,
                    intmax_t,
                    libc::c_int,
                    libc::c_int,
                    libc::c_uint,
                    *const libc::c_void,
                ) -> intmax_t,
        ),
        &mut state as *mut ar_glob_state as *const libc::c_void,
    );
    if (state.chain).is_null() {
        return 0 as *mut nameseq;
    }
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (state.n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as usize,
    );
    names = fresh1.leak().as_mut_ptr() as *mut *const libc::c_char;
    i = 0 as libc::c_int as libc::c_uint;
    n = state.chain;
    while !n.is_null() {
        let fresh2 = i;
        i = i.wrapping_add(1);
        let ref mut fresh3 = *names.offset(fresh2 as isize);
        *fresh3 = (*n).name;
        n = (*n).next;
    }
    qsort(
        names as *mut libc::c_void,
        i as size_t,
        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
        Some(
            alpha_compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int as libc::c_uint;
    n = state.chain;
    while !n.is_null() {
        let fresh4 = i;
        i = i.wrapping_add(1);
        (*n).name = *names.offset(fresh4 as isize);
        n = (*n).next;
    }
    return state.chain;
}
