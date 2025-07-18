use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn rpl_free(ptr: *mut libc::c_void);
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
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
    fn rpl_regerror(
        __errcode: libc::c_int,
        __preg: *const regex_t,
        __errbuf: *mut libc::c_char,
        __errbuf_size: size_t,
    ) -> size_t;
    fn assign_string(dest: *mut *mut libc::c_char, src: *const libc::c_char);
    fn usage(_: libc::c_int);
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    fn _obstack_begin(
        _: *mut obstack,
        _: size_t,
        _: size_t,
        _: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    fn _obstack_free(_: *mut obstack, _: *mut libc::c_void);
}
pub type __int32_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut libc::c_char,
    pub next_free: *mut libc::c_char,
    pub chunk_limit: *mut libc::c_char,
    pub temp: C2RustUnnamed_1,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_0,
    pub freefun: C2RustUnnamed,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub plain: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut libc::c_char,
    pub prev: *mut _obstack_chunk,
    pub contents: [libc::c_char; 0],
}
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
pub struct transform {
    pub next: *mut transform,
    pub transform_type: transform_type,
    pub flags: libc::c_int,
    pub match_number: libc::c_uint,
    pub regex: regex_t,
    pub repl_head: *mut replace_segm,
    pub repl_tail: *mut replace_segm,
    pub segm_count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct replace_segm {
    pub next: *mut replace_segm,
    pub type_0: replace_segm_type,
    pub v: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub literal: C2RustUnnamed_3,
    pub ref_0: size_t,
    pub ctl: case_ctl_type,
}
pub type case_ctl_type = libc::c_uint;
pub const ctl_locase: case_ctl_type = 4;
pub const ctl_upcase: case_ctl_type = 3;
pub const ctl_locase_next: case_ctl_type = 2;
pub const ctl_upcase_next: case_ctl_type = 1;
pub const ctl_stop: case_ctl_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub ptr: *mut libc::c_char,
    pub size: size_t,
}
pub type replace_segm_type = libc::c_uint;
pub const segm_case_ctl: replace_segm_type = 2;
pub const segm_backref: replace_segm_type = 1;
pub const segm_literal: replace_segm_type = 0;
pub type transform_type = libc::c_uint;
pub const transform_global: transform_type = 1;
pub const transform_first: transform_type = 0;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut transform_flags: libc::c_int = 0x1 as libc::c_int | 0x2 as libc::c_int
    | 0x4 as libc::c_int;
static mut transform_head: *mut transform = 0 as *const transform as *mut transform;
static mut transform_tail: *mut transform = 0 as *const transform as *mut transform;
unsafe extern "C" fn new_transform() -> *mut transform {
    let mut p: *mut transform = xzalloc(
        ::std::mem::size_of::<transform>() as libc::c_ulong,
    ) as *mut transform;
    if !transform_tail.is_null() {
        (*transform_tail).next = p;
    } else {
        transform_head = p;
    }
    transform_tail = p;
    return p;
}
unsafe extern "C" fn add_segment(mut tf: *mut transform) -> *mut replace_segm {
    let mut segm: *mut replace_segm = xmalloc(
        ::std::mem::size_of::<replace_segm>() as libc::c_ulong,
    ) as *mut replace_segm;
    (*segm).next = 0 as *mut replace_segm;
    if !((*tf).repl_tail).is_null() {
        (*(*tf).repl_tail).next = segm;
    } else {
        (*tf).repl_head = segm;
    }
    (*tf).repl_tail = segm;
    (*tf).segm_count = ((*tf).segm_count).wrapping_add(1);
    (*tf).segm_count;
    return segm;
}
unsafe extern "C" fn add_literal_segment(
    mut tf: *mut transform,
    mut str: *const libc::c_char,
    mut end: *const libc::c_char,
) {
    let mut len: size_t = end.offset_from(str) as libc::c_long as size_t;
    if len != 0 {
        let mut segm: *mut replace_segm = add_segment(tf);
        (*segm).type_0 = segm_literal;
        (*segm)
            .v
            .literal
            .ptr = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        memcpy(
            (*segm).v.literal.ptr as *mut libc::c_void,
            str as *const libc::c_void,
            len,
        );
        *((*segm).v.literal.ptr).offset(len as isize) = 0 as libc::c_int as libc::c_char;
        (*segm).v.literal.size = len;
    }
}
unsafe extern "C" fn add_char_segment(mut tf: *mut transform, mut chr: libc::c_int) {
    let mut segm: *mut replace_segm = add_segment(tf);
    (*segm).type_0 = segm_literal;
    (*segm).v.literal.ptr = xmalloc(2 as libc::c_int as size_t) as *mut libc::c_char;
    *((*segm).v.literal.ptr).offset(0 as libc::c_int as isize) = chr as libc::c_char;
    *((*segm).v.literal.ptr)
        .offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    (*segm).v.literal.size = 1 as libc::c_int as size_t;
}
unsafe extern "C" fn add_backref_segment(mut tf: *mut transform, mut ref_0: size_t) {
    let mut segm: *mut replace_segm = add_segment(tf);
    (*segm).type_0 = segm_backref;
    (*segm).v.ref_0 = ref_0;
}
unsafe extern "C" fn parse_xform_flags(
    mut pflags: *mut libc::c_int,
    mut c: libc::c_int,
) -> libc::c_int {
    match c {
        114 => {
            *pflags |= 0x1 as libc::c_int;
        }
        82 => {
            *pflags &= !(0x1 as libc::c_int);
        }
        104 => {
            *pflags |= 0x2 as libc::c_int;
        }
        72 => {
            *pflags &= !(0x2 as libc::c_int);
        }
        115 => {
            *pflags |= 0x4 as libc::c_int;
        }
        83 => {
            *pflags &= !(0x4 as libc::c_int);
        }
        _ => return 1 as libc::c_int,
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_case_ctl_segment(
    mut tf: *mut transform,
    mut ctl: case_ctl_type,
) {
    let mut segm: *mut replace_segm = add_segment(tf);
    (*segm).type_0 = segm_case_ctl;
    (*segm).v.ctl = ctl;
}
unsafe extern "C" fn parse_transform_expr(
    mut expr: *const libc::c_char,
) -> *const libc::c_char {
    let mut delim: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut beg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cur: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut cflags: libc::c_int = 0 as libc::c_int;
    let mut tf: *mut transform = new_transform();
    if *expr.offset(0 as libc::c_int as isize) as libc::c_int != 's' as i32 {
        if strncmp(
            expr,
            b"flags=\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            transform_flags = 0 as libc::c_int;
            expr = expr.offset(6 as libc::c_int as isize);
            while *expr != 0 {
                if *expr as libc::c_int == ';' as i32 {
                    expr = expr.offset(1);
                    expr;
                    break;
                } else {
                    if parse_xform_flags(&mut transform_flags, *expr as libc::c_int) != 0
                    {
                        if error_hook.is_some() {
                            error_hook.unwrap()();
                        }
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Unknown transform flag: %c\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            *expr as libc::c_int,
                        );
                        usage(2 as libc::c_int);
                    }
                    expr = expr.offset(1);
                    expr;
                }
            }
            return expr;
        }
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid transform expression\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(2 as libc::c_int);
    }
    delim = *expr.offset(1 as libc::c_int as isize) as libc::c_int;
    if delim == 0 {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid transform expression\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(2 as libc::c_int);
    }
    i = 2 as libc::c_int;
    while *expr.offset(i as isize) as libc::c_int != 0
        && *expr.offset(i as isize) as libc::c_int != delim
    {
        if *expr.offset(i as isize) as libc::c_int == '\\' as i32
            && *expr.offset((i + 1 as libc::c_int) as isize) as libc::c_int != 0
        {
            i += 1;
            i;
        }
        i += 1;
        i;
    }
    if *expr.offset(i as isize) as libc::c_int != delim {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid transform expression\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(2 as libc::c_int);
    }
    j = i + 1 as libc::c_int;
    while *expr.offset(j as isize) as libc::c_int != 0
        && *expr.offset(j as isize) as libc::c_int != delim
    {
        if *expr.offset(j as isize) as libc::c_int == '\\' as i32
            && *expr.offset((j + 1 as libc::c_int) as isize) as libc::c_int != 0
        {
            j += 1;
            j;
        }
        j += 1;
        j;
    }
    if *expr.offset(j as isize) as libc::c_int != delim {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid transform expression\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(2 as libc::c_int);
    }
    (*tf).transform_type = transform_first;
    (*tf).flags = transform_flags;
    p = expr.offset(j as isize).offset(1 as libc::c_int as isize);
    while *p as libc::c_int != 0 && *p as libc::c_int != ';' as i32 {
        match *p as libc::c_int {
            103 => {
                (*tf).transform_type = transform_global;
            }
            105 => {
                cflags |= (1 as libc::c_int) << 1 as libc::c_int;
            }
            120 => {
                cflags |= 1 as libc::c_int;
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                (*tf)
                    .match_number = strtoul(
                    p,
                    &mut p as *mut *const libc::c_char as *mut *mut libc::c_char,
                    0 as libc::c_int,
                ) as libc::c_uint;
                p = p.offset(-1);
                p;
            }
            _ => {
                if parse_xform_flags(&mut (*tf).flags, *p as libc::c_int) != 0 {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Unknown flag in transform expression: %c\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *p as libc::c_int,
                    );
                    usage(2 as libc::c_int);
                }
            }
        }
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == ';' as i32 {
        p = p.offset(1);
        p;
    }
    str = xmalloc((i - 1 as libc::c_int) as size_t) as *mut libc::c_char;
    memcpy(
        str as *mut libc::c_void,
        expr.offset(2 as libc::c_int as isize) as *const libc::c_void,
        (i - 2 as libc::c_int) as libc::c_ulong,
    );
    *str.offset((i - 2 as libc::c_int) as isize) = 0 as libc::c_int as libc::c_char;
    rc = rpl_regcomp(&mut (*tf).regex, str, cflags);
    if rc != 0 {
        let mut errbuf: [libc::c_char; 512] = [0; 512];
        rpl_regerror(
            rc,
            &mut (*tf).regex,
            errbuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        );
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid transform expression: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            errbuf.as_mut_ptr(),
        );
        usage(2 as libc::c_int);
    }
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == '^' as i32
        || i > 2 as libc::c_int
            && *str.offset((i - 3 as libc::c_int) as isize) as libc::c_int == '$' as i32
    {
        (*tf).transform_type = transform_first;
    }
    rpl_free(str as *mut libc::c_void);
    i += 1;
    i;
    str = xmalloc((j - i + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    memcpy(
        str as *mut libc::c_void,
        expr.offset(i as isize) as *const libc::c_void,
        (j - i) as libc::c_ulong,
    );
    *str.offset((j - i) as isize) = 0 as libc::c_int as libc::c_char;
    beg = str;
    cur = beg;
    while *cur != 0 {
        if *cur as libc::c_int == '\\' as i32 {
            let mut n: size_t = 0;
            add_literal_segment(tf, beg, cur);
            cur = cur.offset(1);
            match *cur as libc::c_int {
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                    n = strtoul(cur, &mut cur, 10 as libc::c_int);
                    if n > (*tf).regex.re_nsub {
                        if error_hook.is_some() {
                            error_hook.unwrap()();
                        }
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Invalid transform replacement: back reference out of range\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        usage(2 as libc::c_int);
                    }
                    add_backref_segment(tf, n);
                }
                92 => {
                    add_char_segment(tf, '\\' as i32);
                    cur = cur.offset(1);
                    cur;
                }
                97 => {
                    add_char_segment(tf, '\u{7}' as i32);
                    cur = cur.offset(1);
                    cur;
                }
                98 => {
                    add_char_segment(tf, '\u{8}' as i32);
                    cur = cur.offset(1);
                    cur;
                }
                102 => {
                    add_char_segment(tf, '\u{c}' as i32);
                    cur = cur.offset(1);
                    cur;
                }
                110 => {
                    add_char_segment(tf, '\n' as i32);
                    cur = cur.offset(1);
                    cur;
                }
                114 => {
                    add_char_segment(tf, '\r' as i32);
                    cur = cur.offset(1);
                    cur;
                }
                116 => {
                    add_char_segment(tf, '\t' as i32);
                    cur = cur.offset(1);
                    cur;
                }
                118 => {
                    add_char_segment(tf, '\u{b}' as i32);
                    cur = cur.offset(1);
                    cur;
                }
                38 => {
                    add_char_segment(tf, '&' as i32);
                    cur = cur.offset(1);
                    cur;
                }
                76 => {
                    add_case_ctl_segment(tf, ctl_locase);
                    cur = cur.offset(1);
                    cur;
                }
                108 => {
                    add_case_ctl_segment(tf, ctl_locase_next);
                    cur = cur.offset(1);
                    cur;
                }
                85 => {
                    add_case_ctl_segment(tf, ctl_upcase);
                    cur = cur.offset(1);
                    cur;
                }
                117 => {
                    add_case_ctl_segment(tf, ctl_upcase_next);
                    cur = cur.offset(1);
                    cur;
                }
                69 => {
                    add_case_ctl_segment(tf, ctl_stop);
                    cur = cur.offset(1);
                    cur;
                }
                _ => {
                    if *cur as libc::c_int == delim {
                        add_char_segment(tf, delim);
                    } else {
                        let mut buf: [libc::c_char; 2] = [0; 2];
                        buf[0 as libc::c_int as usize] = '\\' as i32 as libc::c_char;
                        buf[1 as libc::c_int as usize] = *cur;
                        add_literal_segment(
                            tf,
                            buf.as_mut_ptr(),
                            buf.as_mut_ptr().offset(2 as libc::c_int as isize),
                        );
                    }
                    cur = cur.offset(1);
                    cur;
                }
            }
            beg = cur;
        } else if *cur as libc::c_int == '&' as i32 {
            add_literal_segment(tf, beg, cur);
            add_backref_segment(tf, 0 as libc::c_int as size_t);
            cur = cur.offset(1);
            beg = cur;
        } else {
            cur = cur.offset(1);
            cur;
        }
    }
    add_literal_segment(tf, beg, cur);
    rpl_free(str as *mut libc::c_void);
    return p;
}
pub unsafe extern "C" fn set_transform_expr(mut expr: *const libc::c_char) {
    while *expr != 0 {
        expr = parse_transform_expr(expr);
    }
}
unsafe extern "C" fn run_case_conv(
    mut case_ctl: case_ctl_type,
    mut ptr: *mut libc::c_char,
    mut size: size_t,
) -> *mut libc::c_char {
    static mut case_ctl_buffer: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut case_ctl_bufsize: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if case_ctl_bufsize < size {
        case_ctl_bufsize = size;
        case_ctl_buffer = xrealloc(
            case_ctl_buffer as *mut libc::c_void,
            case_ctl_bufsize,
        ) as *mut libc::c_char;
    }
    memcpy(case_ctl_buffer as *mut libc::c_void, ptr as *const libc::c_void, size);
    match case_ctl as libc::c_uint {
        1 => {
            *case_ctl_buffer
                .offset(
                    0 as libc::c_int as isize,
                ) = ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *case_ctl_buffer
                            .offset(0 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(
                            *case_ctl_buffer.offset(0 as libc::c_int as isize)
                                as libc::c_uchar as libc::c_int,
                        );
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(
                            *case_ctl_buffer.offset(0 as libc::c_int as isize)
                                as libc::c_uchar as libc::c_int as isize,
                        );
                }
                __res
            }) as libc::c_char;
        }
        2 => {
            *case_ctl_buffer
                .offset(
                    0 as libc::c_int as isize,
                ) = ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *case_ctl_buffer
                            .offset(0 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(
                            *case_ctl_buffer.offset(0 as libc::c_int as isize)
                                as libc::c_uchar as libc::c_int,
                        );
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(
                            *case_ctl_buffer.offset(0 as libc::c_int as isize)
                                as libc::c_uchar as libc::c_int as isize,
                        );
                }
                __res
            }) as libc::c_char;
        }
        3 => {
            p = case_ctl_buffer;
            while p < case_ctl_buffer.offset(size as isize) {
                *p = ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *p as libc::c_uchar
                                as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = toupper(*p as libc::c_uchar as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(*p as libc::c_uchar as libc::c_int as isize);
                    }
                    __res
                }) as libc::c_char;
                p = p.offset(1);
                p;
            }
        }
        4 => {
            p = case_ctl_buffer;
            while p < case_ctl_buffer.offset(size as isize) {
                *p = ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *p as libc::c_uchar
                                as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(*p as libc::c_uchar as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(*p as libc::c_uchar as libc::c_int as isize);
                    }
                    __res
                }) as libc::c_char;
                p = p.offset(1);
                p;
            }
        }
        0 | _ => {}
    }
    return case_ctl_buffer;
}
static mut stk: obstack = obstack {
    chunk_size: 0,
    chunk: 0 as *const _obstack_chunk as *mut _obstack_chunk,
    object_base: 0 as *const libc::c_char as *mut libc::c_char,
    next_free: 0 as *const libc::c_char as *mut libc::c_char,
    chunk_limit: 0 as *const libc::c_char as *mut libc::c_char,
    temp: C2RustUnnamed_1 { i: 0 },
    alignment_mask: 0,
    chunkfun: C2RustUnnamed_0 { plain: None },
    freefun: C2RustUnnamed { plain: None },
    extra_arg: 0 as *const libc::c_void as *mut libc::c_void,
    use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
    c2rust_padding: [0; 7],
};
static mut stk_init: bool = false;
unsafe extern "C" fn _single_transform_name_to_obstack(
    mut tf: *mut transform,
    mut input: *mut libc::c_char,
) {
    let mut rmp: *mut regmatch_t = 0 as *mut regmatch_t;
    let mut rc: libc::c_int = 0;
    let mut nmatches: size_t = 0 as libc::c_int as size_t;
    let mut case_ctl: case_ctl_type = ctl_stop;
    let mut save_ctl: case_ctl_type = ctl_stop;
    rmp = xmalloc(
        ((*tf).regex.re_nsub)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<regmatch_t>() as libc::c_ulong),
    ) as *mut regmatch_t;
    while *input != 0 {
        let mut disp: size_t = 0;
        let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
        rc = rpl_regexec(
            &mut (*tf).regex,
            input,
            ((*tf).regex.re_nsub).wrapping_add(1 as libc::c_int as libc::c_ulong),
            rmp,
            0 as libc::c_int,
        );
        if rc == 0 as libc::c_int {
            let mut segm: *mut replace_segm = 0 as *mut replace_segm;
            disp = (*rmp.offset(0 as libc::c_int as isize)).rm_eo as size_t;
            if (*rmp.offset(0 as libc::c_int as isize)).rm_so != 0 {
                let mut __o: *mut obstack = &mut stk;
                let mut __len: size_t = (*rmp.offset(0 as libc::c_int as isize)).rm_so
                    as size_t;
                if ({
                    let mut __o1: *const obstack = __o;
                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                        as size_t
                }) < __len
                {
                    _obstack_newchunk(__o, __len);
                }
                memcpy(
                    (*__o).next_free as *mut libc::c_void,
                    input as *const libc::c_void,
                    __len,
                );
                (*__o).next_free = ((*__o).next_free).offset(__len as isize);
            }
            nmatches = nmatches.wrapping_add(1);
            nmatches;
            if (*tf).match_number != 0 && nmatches < (*tf).match_number as libc::c_ulong
            {
                let mut __o_0: *mut obstack = &mut stk;
                let mut __len_0: size_t = disp;
                if ({
                    let mut __o1: *const obstack = __o_0;
                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                        as size_t
                }) < __len_0
                {
                    _obstack_newchunk(__o_0, __len_0);
                }
                memcpy(
                    (*__o_0).next_free as *mut libc::c_void,
                    input as *const libc::c_void,
                    __len_0,
                );
                (*__o_0).next_free = ((*__o_0).next_free).offset(__len_0 as isize);
                input = input.offset(disp as isize);
                continue;
            } else {
                segm = (*tf).repl_head;
                while !segm.is_null() {
                    match (*segm).type_0 as libc::c_uint {
                        0 => {
                            if case_ctl as libc::c_uint
                                == ctl_stop as libc::c_int as libc::c_uint
                            {
                                ptr = (*segm).v.literal.ptr;
                            } else {
                                ptr = run_case_conv(
                                    case_ctl,
                                    (*segm).v.literal.ptr,
                                    (*segm).v.literal.size,
                                );
                                if case_ctl as libc::c_uint
                                    == ctl_upcase_next as libc::c_int as libc::c_uint
                                    || case_ctl as libc::c_uint
                                        == ctl_locase_next as libc::c_int as libc::c_uint
                                {
                                    case_ctl = save_ctl;
                                    save_ctl = ctl_stop;
                                }
                            }
                            let mut __o_1: *mut obstack = &mut stk;
                            let mut __len_1: size_t = (*segm).v.literal.size;
                            if ({
                                let mut __o1: *const obstack = __o_1;
                                ((*__o1).chunk_limit).offset_from((*__o1).next_free)
                                    as libc::c_long as size_t
                            }) < __len_1
                            {
                                _obstack_newchunk(__o_1, __len_1);
                            }
                            memcpy(
                                (*__o_1).next_free as *mut libc::c_void,
                                ptr as *const libc::c_void,
                                __len_1,
                            );
                            (*__o_1)
                                .next_free = ((*__o_1).next_free).offset(__len_1 as isize);
                        }
                        1 => {
                            if (*rmp.offset((*segm).v.ref_0 as isize)).rm_so
                                != -(1 as libc::c_int) as libc::c_long
                                && (*rmp.offset((*segm).v.ref_0 as isize)).rm_eo
                                    != -(1 as libc::c_int) as libc::c_long
                            {
                                let mut size: size_t = ((*rmp
                                    .offset((*segm).v.ref_0 as isize))
                                    .rm_eo - (*rmp.offset((*segm).v.ref_0 as isize)).rm_so)
                                    as size_t;
                                ptr = input
                                    .offset(
                                        (*rmp.offset((*segm).v.ref_0 as isize)).rm_so as isize,
                                    );
                                if case_ctl as libc::c_uint
                                    != ctl_stop as libc::c_int as libc::c_uint
                                {
                                    ptr = run_case_conv(case_ctl, ptr, size);
                                    if case_ctl as libc::c_uint
                                        == ctl_upcase_next as libc::c_int as libc::c_uint
                                        || case_ctl as libc::c_uint
                                            == ctl_locase_next as libc::c_int as libc::c_uint
                                    {
                                        case_ctl = save_ctl;
                                        save_ctl = ctl_stop;
                                    }
                                }
                                let mut __o_2: *mut obstack = &mut stk;
                                let mut __len_2: size_t = size;
                                if ({
                                    let mut __o1: *const obstack = __o_2;
                                    ((*__o1).chunk_limit).offset_from((*__o1).next_free)
                                        as libc::c_long as size_t
                                }) < __len_2
                                {
                                    _obstack_newchunk(__o_2, __len_2);
                                }
                                memcpy(
                                    (*__o_2).next_free as *mut libc::c_void,
                                    ptr as *const libc::c_void,
                                    __len_2,
                                );
                                (*__o_2)
                                    .next_free = ((*__o_2).next_free).offset(__len_2 as isize);
                            }
                        }
                        2 => {
                            let mut current_block_58: u64;
                            match (*segm).v.ctl as libc::c_uint {
                                1 | 2 => {
                                    match save_ctl as libc::c_uint {
                                        0 | 3 | 4 => {
                                            save_ctl = case_ctl;
                                        }
                                        _ => {}
                                    }
                                    current_block_58 = 18358072663284907160;
                                }
                                3 | 4 | 0 => {
                                    current_block_58 = 18358072663284907160;
                                }
                                _ => {
                                    current_block_58 = 6545907279487748450;
                                }
                            }
                            match current_block_58 {
                                18358072663284907160 => {
                                    case_ctl = (*segm).v.ctl;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                    segm = (*segm).next;
                }
            }
        } else {
            disp = strlen(input);
            let mut __o_3: *mut obstack = &mut stk;
            let mut __len_3: size_t = disp;
            if ({
                let mut __o1: *const obstack = __o_3;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < __len_3
            {
                _obstack_newchunk(__o_3, __len_3);
            }
            memcpy(
                (*__o_3).next_free as *mut libc::c_void,
                input as *const libc::c_void,
                __len_3,
            );
            (*__o_3).next_free = ((*__o_3).next_free).offset(__len_3 as isize);
        }
        input = input.offset(disp as isize);
        if !((*tf).transform_type as libc::c_uint
            == transform_first as libc::c_int as libc::c_uint)
        {
            continue;
        }
        let mut __o_4: *mut obstack = &mut stk;
        let mut __len_4: size_t = strlen(input);
        if ({
            let mut __o1: *const obstack = __o_4;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < __len_4
        {
            _obstack_newchunk(__o_4, __len_4);
        }
        memcpy(
            (*__o_4).next_free as *mut libc::c_void,
            input as *const libc::c_void,
            __len_4,
        );
        (*__o_4).next_free = ((*__o_4).next_free).offset(__len_4 as isize);
        break;
    }
    let mut __o_5: *mut obstack = &mut stk;
    if ({
        let mut __o1: *const obstack = __o_5;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < 1 as libc::c_int as libc::c_ulong
    {
        _obstack_newchunk(__o_5, 1 as libc::c_int as size_t);
    }
    let fresh0 = (*__o_5).next_free;
    (*__o_5).next_free = ((*__o_5).next_free).offset(1);
    *fresh0 = 0 as libc::c_int as libc::c_char;
    rpl_free(rmp as *mut libc::c_void);
}
unsafe extern "C" fn _transform_name_to_obstack(
    mut flags: libc::c_int,
    mut input: *mut libc::c_char,
    mut output: *mut *mut libc::c_char,
) -> bool {
    let mut tf: *mut transform = 0 as *mut transform;
    let mut alloced: bool = 0 as libc::c_int != 0;
    if !stk_init {
        _obstack_begin(
            &mut stk,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
            Some(rpl_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        stk_init = 1 as libc::c_int != 0;
    }
    tf = transform_head;
    while !tf.is_null() {
        if (*tf).flags & flags != 0 {
            _single_transform_name_to_obstack(tf, input);
            input = ({
                let mut __o1: *mut obstack = &mut stk as *mut obstack;
                let mut __value: *mut libc::c_void = (*__o1).object_base
                    as *mut libc::c_void;
                if (*__o1).next_free == __value as *mut libc::c_char {
                    (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
                }
                (*__o1)
                    .next_free = (if (::std::mem::size_of::<ptrdiff_t>()
                    as libc::c_ulong)
                    < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                {
                    (*__o1).object_base
                } else {
                    0 as *mut libc::c_char
                })
                    .offset(
                        ((((*__o1).next_free)
                            .offset_from(
                                if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                    < ::std::mem::size_of::<*mut libc::c_void>()
                                        as libc::c_ulong
                                {
                                    (*__o1).object_base
                                } else {
                                    0 as *mut libc::c_char
                                },
                            ) as libc::c_long as libc::c_ulong)
                            .wrapping_add((*__o1).alignment_mask)
                            & !(*__o1).alignment_mask) as isize,
                    );
                if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
                    > ((*__o1).chunk_limit)
                        .offset_from((*__o1).chunk as *mut libc::c_char) as libc::c_long
                        as size_t
                {
                    (*__o1).next_free = (*__o1).chunk_limit;
                }
                (*__o1).object_base = (*__o1).next_free;
                __value
            }) as *mut libc::c_char;
            alloced = 1 as libc::c_int != 0;
        }
        tf = (*tf).next;
    }
    *output = input;
    return alloced;
}
pub unsafe extern "C" fn transform_name_fp(
    mut pinput: *mut *mut libc::c_char,
    mut flags: libc::c_int,
    mut fun: Option::<
        unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> *mut libc::c_char,
    >,
    mut dat: *mut libc::c_void,
) -> bool {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: bool = _transform_name_to_obstack(flags, *pinput, &mut str);
    if ret {
        assign_string(pinput, if fun.is_some() { fun.unwrap()(str, dat) } else { str });
        let mut __o: *mut obstack = &mut stk;
        let mut __obj: *mut libc::c_void = str as *mut libc::c_void;
        if __obj > (*__o).chunk as *mut libc::c_void
            && __obj < (*__o).chunk_limit as *mut libc::c_void
        {
            (*__o).object_base = __obj as *mut libc::c_char;
            (*__o).next_free = (*__o).object_base;
        } else {
            _obstack_free(__o, __obj);
        }
    } else if fun.is_some() {
        *pinput = 0 as *mut libc::c_char;
        assign_string(pinput, fun.unwrap()(str, dat));
        rpl_free(str as *mut libc::c_void);
        ret = 1 as libc::c_int != 0;
    }
    return ret;
}
pub unsafe extern "C" fn transform_name(
    mut pinput: *mut *mut libc::c_char,
    mut type_0: libc::c_int,
) -> bool {
    return transform_name_fp(pinput, type_0, None, 0 as *mut libc::c_void);
}
pub unsafe extern "C" fn transform_program_p() -> bool {
    return !transform_head.is_null();
}
