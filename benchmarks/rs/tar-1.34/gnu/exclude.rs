use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type hash_table;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    static mut stdin: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn rpl_free(ptr: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn mbscasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn mbsinit(__ps: *const mbstate_t) -> libc::c_int;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn towlower(__wc: wint_t) -> wint_t;
    fn rpl_regcomp(
        __preg: *mut regex_t,
        __pattern: *const libc::c_char,
        __cflags: libc::c_int,
    ) -> libc::c_int;
    fn rpl_regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    fn rpl_regfree(__preg: *mut regex_t);
    fn hash_lookup(
        table: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_string(string: *const libc::c_char, n_buckets: size_t) -> size_t;
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_free(table: *mut Hash_table);
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strnlen1(string: *const libc::c_char, maxlen: size_t) -> size_t;
    static is_basic_table: [libc::c_uint; 0];
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xalloc_die();
    fn x2realloc(p: *mut libc::c_void, pn: *mut size_t) -> *mut libc::c_void;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
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
pub type ssize_t = __ssize_t;
pub type wint_t = libc::c_uint;
pub type __mbstate_t = mbstate_t;
pub type __re_long_size_t = size_t;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exclude {
    pub head: *mut exclude_segment,
    pub patbuf: *mut pattern_buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pattern_buffer {
    pub next: *mut pattern_buffer,
    pub base: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exclude_segment {
    pub next: *mut exclude_segment,
    pub type_0: exclude_type,
    pub options: libc::c_int,
    pub v: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub table: *mut Hash_table,
    pub pat: exclude_pattern,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exclude_pattern {
    pub exclude: *mut patopts,
    pub exclude_alloc: size_t,
    pub exclude_count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct patopts {
    pub options: libc::c_int,
    pub v: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub pattern: *const libc::c_char,
    pub re: regex_t,
}
pub type Hash_table = hash_table;
pub type exclude_type = libc::c_uint;
pub const exclude_pattern: exclude_type = 1;
pub const exclude_hash: exclude_type = 0;
pub type mbchar_t = mbchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const libc::c_char,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: wchar_t,
    pub buf: [libc::c_char; 24],
}
pub type mbui_iterator_t = mbuiter_multi;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuiter_multi {
    pub in_shift: bool,
    pub state: mbstate_t,
    pub next_done: bool,
    pub cur: mbchar,
}
pub type Hash_tuning = hash_tuning;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub const DEFAULT_MXFAST: C2RustUnnamed_3 = 128;
pub type C2RustUnnamed_3 = libc::c_uint;
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int
        as libc::c_long != 0
    {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = ((*__fp)._IO_read_ptr).offset(1);
        *(fresh0 as *mut libc::c_uchar) as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn mbuiter_multi_next(mut iter: *mut mbuiter_multi) {
    let mut current_block: u64;
    if (*iter).next_done {
        return;
    }
    if (*iter).in_shift {
        current_block = 16070860581997986326;
    } else if is_basic(*(*iter).cur.ptr) {
        (*iter).cur.bytes = 1 as libc::c_int as size_t;
        (*iter).cur.wc = *(*iter).cur.ptr as wchar_t;
        (*iter).cur.wc_valid = 1 as libc::c_int != 0;
        current_block = 15089075282327824602;
    } else {
        if mbsinit(&mut (*iter).state) != 0 {} else {
            __assert_fail(
                b"mbsinit (&iter->state)\0" as *const u8 as *const libc::c_char,
                b"./mbuiter.h\0" as *const u8 as *const libc::c_char,
                143 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                    .as_ptr(),
            );
        }
        'c_4684: {
            if mbsinit(&mut (*iter).state) != 0 {} else {
                __assert_fail(
                    b"mbsinit (&iter->state)\0" as *const u8 as *const libc::c_char,
                    b"./mbuiter.h\0" as *const u8 as *const libc::c_char,
                    143 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 48],
                        &[libc::c_char; 48],
                    >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                        .as_ptr(),
                );
            }
        };
        (*iter).in_shift = 1 as libc::c_int != 0;
        current_block = 16070860581997986326;
    }
    match current_block {
        16070860581997986326 => {
            (*iter)
                .cur
                .bytes = rpl_mbrtowc(
                &mut (*iter).cur.wc,
                (*iter).cur.ptr,
                strnlen1((*iter).cur.ptr, __ctype_get_mb_cur_max()),
                &mut (*iter).state,
            );
            if (*iter).cur.bytes == -(1 as libc::c_int) as size_t {
                (*iter).cur.bytes = 1 as libc::c_int as size_t;
                (*iter).cur.wc_valid = 0 as libc::c_int != 0;
            } else if (*iter).cur.bytes == -(2 as libc::c_int) as size_t {
                (*iter).cur.bytes = strlen((*iter).cur.ptr);
                (*iter).cur.wc_valid = 0 as libc::c_int != 0;
            } else {
                if (*iter).cur.bytes == 0 as libc::c_int as libc::c_ulong {
                    (*iter).cur.bytes = 1 as libc::c_int as size_t;
                    if *(*iter).cur.ptr as libc::c_int == '\0' as i32 {} else {
                        __assert_fail(
                            b"*iter->cur.ptr == '\\0'\0" as *const u8
                                as *const libc::c_char,
                            b"./mbuiter.h\0" as *const u8 as *const libc::c_char,
                            171 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 48],
                                &[libc::c_char; 48],
                            >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_4519: {
                        if *(*iter).cur.ptr as libc::c_int == '\0' as i32 {} else {
                            __assert_fail(
                                b"*iter->cur.ptr == '\\0'\0" as *const u8
                                    as *const libc::c_char,
                                b"./mbuiter.h\0" as *const u8 as *const libc::c_char,
                                171 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 48],
                                    &[libc::c_char; 48],
                                >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    if (*iter).cur.wc == 0 as libc::c_int {} else {
                        __assert_fail(
                            b"iter->cur.wc == 0\0" as *const u8 as *const libc::c_char,
                            b"./mbuiter.h\0" as *const u8 as *const libc::c_char,
                            172 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 48],
                                &[libc::c_char; 48],
                            >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_4466: {
                        if (*iter).cur.wc == 0 as libc::c_int {} else {
                            __assert_fail(
                                b"iter->cur.wc == 0\0" as *const u8 as *const libc::c_char,
                                b"./mbuiter.h\0" as *const u8 as *const libc::c_char,
                                172 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 48],
                                    &[libc::c_char; 48],
                                >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                }
                (*iter).cur.wc_valid = 1 as libc::c_int != 0;
                if mbsinit(&mut (*iter).state) != 0 {
                    (*iter).in_shift = 0 as libc::c_int != 0;
                }
            }
        }
        _ => {}
    }
    (*iter).next_done = 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn is_basic(mut c: libc::c_char) -> bool {
    return *is_basic_table
        .as_ptr()
        .offset((c as libc::c_uchar as libc::c_int >> 5 as libc::c_int) as isize)
        >> (c as libc::c_uchar as libc::c_int & 31 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint != 0;
}
#[inline]
unsafe extern "C" fn x2nrealloc(
    mut p: *mut libc::c_void,
    mut pn: *mut size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut n: size_t = *pn;
    if p.is_null() {
        if n == 0 {
            n = (DEFAULT_MXFAST as libc::c_int as libc::c_ulong).wrapping_div(s);
            n = (n as libc::c_ulong)
                .wrapping_add((n == 0) as libc::c_int as libc::c_ulong) as size_t
                as size_t;
        }
        if (if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        })
            .wrapping_div(s) < n
        {
            xalloc_die();
        }
    } else {
        if (if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            18446744073709551615 as libc::c_ulong
        })
            .wrapping_div(3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_div(s) <= n
        {
            xalloc_die();
        }
        n = (n as libc::c_ulong)
            .wrapping_add(
                n
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
    }
    *pn = n;
    return xrealloc(p, n.wrapping_mul(s));
}
pub unsafe extern "C" fn exclude_add_pattern_buffer(
    mut ex: *mut exclude,
    mut buf: *mut libc::c_char,
) {
    let mut pbuf: *mut pattern_buffer = xmalloc(
        ::std::mem::size_of::<pattern_buffer>() as libc::c_ulong,
    ) as *mut pattern_buffer;
    (*pbuf).base = buf;
    (*pbuf).next = (*ex).patbuf;
    (*ex).patbuf = pbuf;
}
pub unsafe extern "C" fn fnmatch_pattern_has_wildcards(
    mut str: *const libc::c_char,
    mut options: libc::c_int,
) -> bool {
    loop {
        let fresh1 = str;
        str = str.offset(1);
        match *fresh1 as libc::c_int {
            46 | 123 | 125 | 40 | 41 => {
                if options & (1 as libc::c_int) << 27 as libc::c_int != 0 {
                    return 1 as libc::c_int != 0;
                }
            }
            92 => {
                if options & (1 as libc::c_int) << 27 as libc::c_int != 0 {
                    continue;
                }
                str = str
                    .offset(
                        (options & (1 as libc::c_int) << 1 as libc::c_int == 0
                            && *str as libc::c_int != 0) as libc::c_int as isize,
                    );
            }
            43 | 64 | 33 => {
                if options & (1 as libc::c_int) << 5 as libc::c_int != 0
                    && *str as libc::c_int == '(' as i32
                {
                    return 1 as libc::c_int != 0;
                }
            }
            63 | 42 | 91 => return 1 as libc::c_int != 0,
            0 => return 0 as libc::c_int != 0,
            _ => {}
        }
    };
}
unsafe extern "C" fn unescape_pattern(mut str: *mut libc::c_char) {
    let mut q: *const libc::c_char = str;
    loop {
        q = q
            .offset(
                (*q as libc::c_int == '\\' as i32
                    && *q.offset(1 as libc::c_int as isize) as libc::c_int != 0)
                    as libc::c_int as isize,
            );
        let fresh2 = q;
        q = q.offset(1);
        let fresh3 = str;
        str = str.offset(1);
        *fresh3 = *fresh2;
        if !(*fresh3 != 0) {
            break;
        }
    };
}
pub unsafe extern "C" fn new_exclude() -> *mut exclude {
    return xzalloc(::std::mem::size_of::<exclude>() as libc::c_ulong) as *mut exclude;
}
unsafe extern "C" fn string_hasher(
    mut data: *const libc::c_void,
    mut n_buckets: size_t,
) -> size_t {
    let mut p: *const libc::c_char = data as *const libc::c_char;
    return hash_string(p, n_buckets);
}
unsafe extern "C" fn string_hasher_ci(
    mut data: *const libc::c_void,
    mut n_buckets: size_t,
) -> size_t {
    let mut p: *const libc::c_char = data as *const libc::c_char;
    let mut iter: mbui_iterator_t = mbui_iterator_t {
        in_shift: false,
        state: mbstate_t {
            __count: 0,
            __value: C2RustUnnamed_0 { __wch: 0 },
        },
        next_done: false,
        cur: mbchar {
            ptr: 0 as *const libc::c_char,
            bytes: 0,
            wc_valid: false,
            wc: 0,
            buf: [0; 24],
        },
    };
    let mut value: size_t = 0 as libc::c_int as size_t;
    iter.cur.ptr = p;
    iter.in_shift = 0 as libc::c_int != 0;
    memset(
        &mut iter.state as *mut mbstate_t as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    iter.next_done = 0 as libc::c_int != 0;
    loop {
        mbuiter_multi_next(&mut iter);
        if !(!(iter.cur.wc_valid as libc::c_int != 0 && iter.cur.wc == 0 as libc::c_int)
            as libc::c_int != 0)
        {
            break;
        }
        let mut m: mbchar_t = iter.cur;
        let mut wc: wchar_t = 0;
        if m.wc_valid {
            wc = towlower(m.wc as wint_t) as wchar_t;
        } else {
            wc = *m.ptr as wchar_t;
        }
        value = value
            .wrapping_mul(31 as libc::c_int as libc::c_ulong)
            .wrapping_add(wc as libc::c_ulong)
            .wrapping_rem(n_buckets);
        iter.cur.ptr = (iter.cur.ptr).offset(iter.cur.bytes as isize);
        iter.next_done = 0 as libc::c_int != 0;
    }
    return value;
}
unsafe extern "C" fn string_compare(
    mut data1: *const libc::c_void,
    mut data2: *const libc::c_void,
) -> bool {
    let mut p1: *const libc::c_char = data1 as *const libc::c_char;
    let mut p2: *const libc::c_char = data2 as *const libc::c_char;
    return strcmp(p1, p2) == 0 as libc::c_int;
}
unsafe extern "C" fn string_compare_ci(
    mut data1: *const libc::c_void,
    mut data2: *const libc::c_void,
) -> bool {
    let mut p1: *const libc::c_char = data1 as *const libc::c_char;
    let mut p2: *const libc::c_char = data2 as *const libc::c_char;
    return mbscasecmp(p1, p2) == 0 as libc::c_int;
}
unsafe extern "C" fn string_free(mut data: *mut libc::c_void) {
    rpl_free(data);
}
unsafe extern "C" fn new_exclude_segment(
    mut ex: *mut exclude,
    mut type_0: exclude_type,
    mut options: libc::c_int,
) {
    let mut sp: *mut exclude_segment = xzalloc(
        ::std::mem::size_of::<exclude_segment>() as libc::c_ulong,
    ) as *mut exclude_segment;
    (*sp).type_0 = type_0;
    (*sp).options = options;
    match type_0 as libc::c_uint {
        0 => {
            (*sp)
                .v
                .table = hash_initialize(
                0 as libc::c_int as size_t,
                0 as *const Hash_tuning,
                if options & (1 as libc::c_int) << 4 as libc::c_int != 0 {
                    Some(
                        string_hasher_ci
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                size_t,
                            ) -> size_t,
                    )
                } else {
                    Some(
                        string_hasher
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                size_t,
                            ) -> size_t,
                    )
                },
                if options & (1 as libc::c_int) << 4 as libc::c_int != 0 {
                    Some(
                        string_compare_ci
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> bool,
                    )
                } else {
                    Some(
                        string_compare
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> bool,
                    )
                },
                Some(string_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            );
        }
        1 | _ => {}
    }
    (*sp).next = (*ex).head;
    (*ex).head = sp;
}
unsafe extern "C" fn free_exclude_segment(mut seg: *mut exclude_segment) {
    let mut i: size_t = 0;
    match (*seg).type_0 as libc::c_uint {
        1 => {
            i = 0 as libc::c_int as size_t;
            while i < (*seg).v.pat.exclude_count {
                if (*((*seg).v.pat.exclude).offset(i as isize)).options
                    & (1 as libc::c_int) << 27 as libc::c_int != 0
                {
                    rpl_regfree(&mut (*((*seg).v.pat.exclude).offset(i as isize)).v.re);
                }
                i = i.wrapping_add(1);
                i;
            }
            rpl_free((*seg).v.pat.exclude as *mut libc::c_void);
        }
        0 => {
            hash_free((*seg).v.table);
        }
        _ => {}
    }
    rpl_free(seg as *mut libc::c_void);
}
pub unsafe extern "C" fn free_exclude(mut ex: *mut exclude) {
    let mut seg: *mut exclude_segment = 0 as *mut exclude_segment;
    let mut pbuf: *mut pattern_buffer = 0 as *mut pattern_buffer;
    seg = (*ex).head;
    while !seg.is_null() {
        let mut next: *mut exclude_segment = (*seg).next;
        free_exclude_segment(seg);
        seg = next;
    }
    pbuf = (*ex).patbuf;
    while !pbuf.is_null() {
        let mut next_0: *mut pattern_buffer = (*pbuf).next;
        rpl_free((*pbuf).base as *mut libc::c_void);
        rpl_free(pbuf as *mut libc::c_void);
        pbuf = next_0;
    }
    rpl_free(ex as *mut libc::c_void);
}
unsafe extern "C" fn fnmatch_no_wildcards(
    mut pattern: *const libc::c_char,
    mut f: *const libc::c_char,
    mut options: libc::c_int,
) -> libc::c_int {
    if options & (1 as libc::c_int) << 3 as libc::c_int == 0 {
        return if options & (1 as libc::c_int) << 4 as libc::c_int != 0 {
            mbscasecmp(pattern, f)
        } else {
            strcmp(pattern, f)
        }
    } else if options & (1 as libc::c_int) << 4 as libc::c_int == 0 {
        let mut patlen: size_t = strlen(pattern);
        let mut r: libc::c_int = strncmp(pattern, f, patlen);
        if r == 0 {
            r = *f.offset(patlen as isize) as libc::c_int;
            if r == '/' as i32 {
                r = 0 as libc::c_int;
            }
        }
        return r;
    } else {
        let mut fcopy: *mut libc::c_char = xstrdup(f);
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut r_0: libc::c_int = 0;
        p = fcopy;
        loop {
            p = strchr(p, '/' as i32);
            if !p.is_null() {
                *p = '\0' as i32 as libc::c_char;
            }
            r_0 = mbscasecmp(pattern, fcopy);
            if p.is_null() || r_0 <= 0 as libc::c_int {
                break;
            }
            let fresh4 = p;
            p = p.offset(1);
            *fresh4 = '/' as i32 as libc::c_char;
        }
        rpl_free(fcopy as *mut libc::c_void);
        return r_0;
    };
}
pub unsafe extern "C" fn exclude_fnmatch(
    mut pattern: *const libc::c_char,
    mut f: *const libc::c_char,
    mut options: libc::c_int,
) -> bool {
    let mut matcher: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    > = if options & (1 as libc::c_int) << 28 as libc::c_int != 0 {
        Some(
            fnmatch
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    libc::c_int,
                ) -> libc::c_int,
        )
    } else {
        Some(
            fnmatch_no_wildcards
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    libc::c_int,
                ) -> libc::c_int,
        )
    };
    let mut matched: bool = (Some(matcher.unwrap())).unwrap()(pattern, f, options)
        == 0 as libc::c_int;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if options & (1 as libc::c_int) << 30 as libc::c_int == 0 {
        p = f;
        while *p as libc::c_int != 0 && !matched {
            if *p as libc::c_int == '/' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int != '/' as i32
            {
                matched = (Some(matcher.unwrap()))
                    .unwrap()(pattern, p.offset(1 as libc::c_int as isize), options)
                    == 0 as libc::c_int;
            }
            p = p.offset(1);
            p;
        }
    }
    return matched;
}
unsafe extern "C" fn exclude_patopts(
    mut opts: *const patopts,
    mut f: *const libc::c_char,
) -> bool {
    let mut options: libc::c_int = (*opts).options;
    return if options & (1 as libc::c_int) << 27 as libc::c_int != 0 {
        (rpl_regexec(
            &(*opts).v.re,
            f,
            0 as libc::c_int as size_t,
            0 as *mut regmatch_t,
            0 as libc::c_int,
        ) == 0 as libc::c_int) as libc::c_int
    } else {
        exclude_fnmatch((*opts).v.pattern, f, options) as libc::c_int
    } != 0;
}
unsafe extern "C" fn file_pattern_matches(
    mut seg: *const exclude_segment,
    mut f: *const libc::c_char,
) -> bool {
    let mut exclude_count: size_t = (*seg).v.pat.exclude_count;
    let mut exclude: *const patopts = (*seg).v.pat.exclude;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < exclude_count {
        if exclude_patopts(exclude.offset(i as isize), f) {
            return 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn file_name_matches(
    mut seg: *const exclude_segment,
    mut f: *const libc::c_char,
    mut buffer: *mut libc::c_char,
) -> bool {
    let mut options: libc::c_int = (*seg).options;
    let mut table: *mut Hash_table = (*seg).v.table;
    loop {
        strcpy(buffer, f);
        loop {
            if !(hash_lookup(table, buffer as *const libc::c_void)).is_null() {
                return 1 as libc::c_int != 0;
            }
            if !(options & (1 as libc::c_int) << 3 as libc::c_int != 0) {
                break;
            }
            let mut p: *mut libc::c_char = strrchr(buffer, '/' as i32);
            if p.is_null() {
                break;
            }
            *p = 0 as libc::c_int as libc::c_char;
        }
        if !(options & (1 as libc::c_int) << 30 as libc::c_int == 0) {
            break;
        }
        f = strchr(f, '/' as i32);
        if !f.is_null() {
            f = f.offset(1);
            f;
        }
        if f.is_null() {
            break;
        }
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn excluded_file_name(
    mut ex: *const exclude,
    mut f: *const libc::c_char,
) -> bool {
    let mut seg: *mut exclude_segment = 0 as *mut exclude_segment;
    let mut invert: bool = 0 as libc::c_int != 0;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*ex).head).is_null() {
        return 0 as libc::c_int != 0;
    }
    seg = (*ex).head;
    loop {
        if (*seg).type_0 as libc::c_uint == exclude_hash as libc::c_int as libc::c_uint {
            if filename.is_null() {
                filename = xmalloc(
                    (strlen(f)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
            }
            if file_name_matches(seg, f, filename) {
                break;
            }
        } else if file_pattern_matches(seg, f) {
            break;
        }
        if ((*seg).next).is_null() {
            invert = 1 as libc::c_int != 0;
            break;
        } else {
            seg = (*seg).next;
        }
    }
    rpl_free(filename as *mut libc::c_void);
    return invert as libc::c_int
        ^ ((*seg).options & (1 as libc::c_int) << 29 as libc::c_int == 0) as libc::c_int
        != 0;
}
pub unsafe extern "C" fn add_exclude(
    mut ex: *mut exclude,
    mut pattern: *const libc::c_char,
    mut options: libc::c_int,
) {
    let mut seg: *mut exclude_segment = 0 as *mut exclude_segment;
    let mut pat: *mut exclude_pattern = 0 as *mut exclude_pattern;
    let mut patopts: *mut patopts = 0 as *mut patopts;
    if options
        & ((1 as libc::c_int) << 27 as libc::c_int
            | (1 as libc::c_int) << 28 as libc::c_int) != 0
        && fnmatch_pattern_has_wildcards(pattern, options) as libc::c_int != 0
    {
        if !(!((*ex).head).is_null()
            && (*(*ex).head).type_0 as libc::c_uint
                == exclude_pattern as libc::c_int as libc::c_uint
            && (*(*ex).head).options & (1 as libc::c_int) << 29 as libc::c_int
                == options & (1 as libc::c_int) << 29 as libc::c_int)
        {
            new_exclude_segment(ex, exclude_pattern, options);
        }
        seg = (*ex).head;
        pat = &mut (*seg).v.pat;
        if (*pat).exclude_count == (*pat).exclude_alloc {
            (*pat)
                .exclude = x2nrealloc(
                (*pat).exclude as *mut libc::c_void,
                &mut (*pat).exclude_alloc,
                ::std::mem::size_of::<patopts>() as libc::c_ulong,
            ) as *mut patopts;
        }
        let fresh5 = (*pat).exclude_count;
        (*pat).exclude_count = ((*pat).exclude_count).wrapping_add(1);
        patopts = &mut *((*pat).exclude).offset(fresh5 as isize) as *mut patopts;
        (*patopts).options = options;
        if options & (1 as libc::c_int) << 27 as libc::c_int != 0 {
            let mut rc: libc::c_int = 0;
            let mut cflags: libc::c_int = (1 as libc::c_int) << 3 as libc::c_int
                | 1 as libc::c_int
                | (if options & (1 as libc::c_int) << 4 as libc::c_int != 0 {
                    (1 as libc::c_int) << 1 as libc::c_int
                } else {
                    0 as libc::c_int
                });
            if options & (1 as libc::c_int) << 3 as libc::c_int != 0 {
                let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut len: size_t = strlen(pattern);
                while len > 0 as libc::c_int as libc::c_ulong
                    && *pattern
                        .offset(
                            len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == '/' as i32
                {
                    len = len.wrapping_sub(1);
                    len;
                }
                if len == 0 as libc::c_int as libc::c_ulong {
                    rc = 1 as libc::c_int;
                } else {
                    tmp = xmalloc(len.wrapping_add(7 as libc::c_int as libc::c_ulong))
                        as *mut libc::c_char;
                    memcpy(
                        tmp as *mut libc::c_void,
                        pattern as *const libc::c_void,
                        len,
                    );
                    strcpy(
                        tmp.offset(len as isize),
                        b"(/.*)?\0" as *const u8 as *const libc::c_char,
                    );
                    rc = rpl_regcomp(&mut (*patopts).v.re, tmp, cflags);
                    rpl_free(tmp as *mut libc::c_void);
                }
            } else {
                rc = rpl_regcomp(&mut (*patopts).v.re, pattern, cflags);
            }
            if rc != 0 {
                (*pat).exclude_count = ((*pat).exclude_count).wrapping_sub(1);
                (*pat).exclude_count;
                return;
            }
        } else {
            if options & (1 as libc::c_int) << 26 as libc::c_int != 0 {
                pattern = xstrdup(pattern);
                exclude_add_pattern_buffer(ex, pattern as *mut libc::c_char);
            }
            (*patopts).v.pattern = pattern;
        }
    } else {
        let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut exclude_hash_flags: libc::c_int = (1 as libc::c_int) << 29 as libc::c_int
            | (1 as libc::c_int) << 30 as libc::c_int
            | (1 as libc::c_int) << 3 as libc::c_int
            | (1 as libc::c_int) << 4 as libc::c_int;
        if !(!((*ex).head).is_null()
            && (*(*ex).head).type_0 as libc::c_uint
                == exclude_hash as libc::c_int as libc::c_uint
            && (*(*ex).head).options & exclude_hash_flags
                == options & exclude_hash_flags)
        {
            new_exclude_segment(ex, exclude_hash, options);
        }
        seg = (*ex).head;
        str = xstrdup(pattern);
        if options
            & ((1 as libc::c_int) << 28 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int)
            == (1 as libc::c_int) << 28 as libc::c_int
        {
            unescape_pattern(str);
        }
        p = hash_insert((*seg).v.table, str as *const libc::c_void) as *mut libc::c_char;
        if p != str {
            rpl_free(str as *mut libc::c_void);
        }
    };
}
pub unsafe extern "C" fn add_exclude_fp(
    mut add_func: Option::<
        unsafe extern "C" fn(
            *mut exclude,
            *const libc::c_char,
            libc::c_int,
            *mut libc::c_void,
        ) -> (),
    >,
    mut ex: *mut exclude,
    mut fp: *mut FILE,
    mut options: libc::c_int,
    mut line_end: libc::c_char,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pattern: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lim: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf_alloc: size_t = 0 as libc::c_int as size_t;
    let mut buf_count: size_t = 0 as libc::c_int as size_t;
    let mut c: libc::c_int = 0;
    let mut e: libc::c_int = 0 as libc::c_int;
    loop {
        c = getc_unlocked(fp);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        if buf_count == buf_alloc {
            buf = x2realloc(buf as *mut libc::c_void, &mut buf_alloc)
                as *mut libc::c_char;
        }
        let fresh6 = buf_count;
        buf_count = buf_count.wrapping_add(1);
        *buf.offset(fresh6 as isize) = c as libc::c_char;
    }
    if ferror_unlocked(fp) != 0 {
        e = *__errno_location();
    }
    buf = xrealloc(
        buf as *mut libc::c_void,
        buf_count.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    *buf.offset(buf_count as isize) = line_end;
    lim = buf
        .offset(buf_count as isize)
        .offset(
            !(buf_count == 0 as libc::c_int as libc::c_ulong
                || *buf
                    .offset(
                        buf_count.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int == line_end as libc::c_int) as libc::c_int as isize,
        );
    exclude_add_pattern_buffer(ex, buf);
    pattern = buf;
    p = buf;
    while p < lim as *mut libc::c_char {
        if *p as libc::c_int == line_end as libc::c_int {
            let mut current_block_15: u64;
            let mut pattern_end: *mut libc::c_char = p;
            if *(*__ctype_b_loc())
                .offset(line_end as libc::c_uchar as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                current_block_15 = 17407779659766490442;
            } else {
                current_block_15 = 7175849428784450219;
            }
            loop {
                match current_block_15 {
                    17407779659766490442 => {
                        if pattern_end == pattern {
                            break;
                        }
                        if *(*__ctype_b_loc())
                            .offset(
                                *pattern_end.offset(-(1 as libc::c_int) as isize)
                                    as libc::c_uchar as libc::c_int as isize,
                            ) as libc::c_int
                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                            == 0
                        {
                            current_block_15 = 7175849428784450219;
                            continue;
                        }
                        pattern_end = pattern_end.offset(-1);
                        pattern_end;
                        current_block_15 = 17407779659766490442;
                    }
                    _ => {
                        *pattern_end = '\0' as i32 as libc::c_char;
                        (Some(add_func.unwrap())).unwrap()(ex, pattern, options, data);
                        break;
                    }
                }
            }
            pattern = p.offset(1 as libc::c_int as isize);
        }
        p = p.offset(1);
        p;
    }
    *__errno_location() = e;
    return if e != 0 { -(1 as libc::c_int) } else { 0 as libc::c_int };
}
unsafe extern "C" fn call_addfn(
    mut ex: *mut exclude,
    mut pattern: *const libc::c_char,
    mut options: libc::c_int,
    mut data: *mut libc::c_void,
) {
    let mut addfnptr: *mut Option::<
        unsafe extern "C" fn(*mut exclude, *const libc::c_char, libc::c_int) -> (),
    > = data
        as *mut Option::<
            unsafe extern "C" fn(*mut exclude, *const libc::c_char, libc::c_int) -> (),
        >;
    (*addfnptr).unwrap()(ex, pattern, options);
}
pub unsafe extern "C" fn add_exclude_file(
    mut add_func: Option::<
        unsafe extern "C" fn(*mut exclude, *const libc::c_char, libc::c_int) -> (),
    >,
    mut ex: *mut exclude,
    mut file_name: *const libc::c_char,
    mut options: libc::c_int,
    mut line_end: libc::c_char,
) -> libc::c_int {
    let mut use_stdin: bool = *file_name.offset(0 as libc::c_int as isize) as libc::c_int
        == '-' as i32 && *file_name.offset(1 as libc::c_int as isize) == 0;
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut rc: libc::c_int = 0 as libc::c_int;
    if use_stdin {
        in_0 = stdin;
    } else {
        in_0 = fopen(file_name, b"re\0" as *const u8 as *const libc::c_char);
        if in_0.is_null() {
            return -(1 as libc::c_int);
        }
    }
    rc = add_exclude_fp(
        Some(
            call_addfn
                as unsafe extern "C" fn(
                    *mut exclude,
                    *const libc::c_char,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> (),
        ),
        ex,
        in_0,
        options,
        line_end,
        &mut add_func
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut exclude,
                    *const libc::c_char,
                    libc::c_int,
                ) -> (),
            > as *mut libc::c_void,
    );
    if !use_stdin && fclose(in_0) != 0 as libc::c_int {
        rc = -(1 as libc::c_int);
    }
    return rc;
}
