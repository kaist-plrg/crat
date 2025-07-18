use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type dfa;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stdout: *mut FILE;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    static mut mb_cur_max: libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = libc::c_ulong;
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
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type FILE = _IO_FILE;
pub type countT = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vector {
    pub v: *mut sed_cmd,
    pub v_allocated: size_t,
    pub v_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sed_cmd {
    pub a1: *mut addr,
    pub a2: *mut addr,
    pub range_state: addr_state,
    pub addr_bang: libc::c_char,
    pub cmd: libc::c_char,
    pub x: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub cmd_txt: text_buf,
    pub int_arg: libc::c_int,
    pub jump_index: countT,
    pub readcmd: readcmd,
    pub cmd_subst: *mut subst,
    pub outf: *mut output,
    pub inf: *mut output,
    pub translate: *mut libc::c_uchar,
    pub translatemb: *mut *mut libc::c_char,
    pub label_name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output {
    pub name: *mut libc::c_char,
    pub missing_newline: bool,
    pub fp: *mut FILE,
    pub link: *mut output,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct subst {
    pub regx: *mut regex,
    pub replacement: *mut replacement,
    pub numb: countT,
    pub outf: *mut output,
    #[bitfield(name = "global", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "print", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "eval", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "max_id", ty = "libc::c_uint", bits = "4..=7")]
    pub global_print_eval_max_id: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct replacement {
    pub prefix: *mut libc::c_char,
    pub prefix_length: size_t,
    pub subst_id: libc::c_int,
    pub repl_type: replacement_types,
    pub next: *mut replacement,
}
pub type replacement_types = libc::c_uint;
pub const REPL_LOWERCASE_LOWERCASE: replacement_types = 10;
pub const REPL_LOWERCASE_UPPERCASE: replacement_types = 9;
pub const REPL_UPPERCASE_LOWERCASE: replacement_types = 6;
pub const REPL_UPPERCASE_UPPERCASE: replacement_types = 5;
pub const REPL_MODIFIERS: replacement_types = 12;
pub const REPL_LOWERCASE_FIRST: replacement_types = 8;
pub const REPL_UPPERCASE_FIRST: replacement_types = 4;
pub const REPL_LOWERCASE: replacement_types = 2;
pub const REPL_UPPERCASE: replacement_types = 1;
pub const REPL_ASIS: replacement_types = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regex {
    pub pattern: regex_t,
    pub flags: libc::c_int,
    pub sz: size_t,
    pub dfa: *mut dfa,
    pub begline: bool,
    pub endline: bool,
    pub re: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct readcmd {
    pub fname: *mut libc::c_char,
    pub append: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct text_buf {
    pub text: *mut libc::c_char,
    pub text_length: size_t,
}
pub type addr_state = libc::c_uint;
pub const RANGE_CLOSED: addr_state = 2;
pub const RANGE_ACTIVE: addr_state = 1;
pub const RANGE_INACTIVE: addr_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addr {
    pub addr_type: addr_types,
    pub addr_number: countT,
    pub addr_step: countT,
    pub addr_regex: *mut regex,
}
pub type addr_types = libc::c_uint;
pub const ADDR_IS_LAST: addr_types = 6;
pub const ADDR_IS_STEP_MOD: addr_types = 5;
pub const ADDR_IS_STEP: addr_types = 4;
pub const ADDR_IS_NUM_MOD: addr_types = 3;
pub const ADDR_IS_NUM: addr_types = 2;
pub const ADDR_IS_REGEX: addr_types = 1;
pub const ADDR_IS_NULL: addr_types = 0;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
#[inline]
unsafe extern "C" fn putc_unlocked(
    mut __c: libc::c_int,
    mut __stream: *mut FILE,
) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh0 = __c as libc::c_char;
        *fresh0 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int) -> libc::c_int {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(stdout, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh1 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = ((*stdout)._IO_write_ptr).offset(1);
        *fresh1 = __c as libc::c_char;
        *fresh1 as libc::c_uchar as libc::c_int
    };
}
static mut block_level: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn debug_print_char(mut c: libc::c_char) {
    if 1 as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
        && c as libc::c_int != '\\' as i32
    {
        putchar_unlocked(c as libc::c_int);
        return;
    }
    putchar_unlocked('\\' as i32);
    match c as libc::c_int {
        7 => {
            putchar_unlocked('a' as i32);
        }
        12 => {
            putchar_unlocked('f' as i32);
        }
        13 => {
            putchar_unlocked('r' as i32);
        }
        9 => {
            putchar_unlocked('t' as i32);
        }
        11 => {
            putchar_unlocked('v' as i32);
        }
        10 => {
            putchar_unlocked('n' as i32);
        }
        92 => {
            putchar_unlocked('\\' as i32);
        }
        _ => {
            printf(b"o%03o\0" as *const u8 as *const libc::c_char, c as libc::c_uint);
        }
    };
}
unsafe extern "C" fn debug_print_regex_pattern(
    mut pat: *const libc::c_char,
    mut len: size_t,
) {
    let mut p: *const libc::c_char = pat;
    loop {
        let fresh2 = len;
        len = len.wrapping_sub(1);
        if !(fresh2 != 0) {
            break;
        }
        if *p as libc::c_int == '/' as i32 {
            fputs_unlocked(b"\\/\0" as *const u8 as *const libc::c_char, stdout);
        } else {
            debug_print_char(*p);
        }
        p = p.offset(1);
        p;
    };
}
unsafe extern "C" fn debug_print_regex_flags(mut r: *const regex, mut addr: bool) {
    if r.is_null() {
        return;
    }
    if (*r).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        putchar_unlocked(if addr as libc::c_int != 0 { 'I' as i32 } else { 'i' as i32 });
    }
    if (*r).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        putchar_unlocked(if addr as libc::c_int != 0 { 'M' as i32 } else { 'm' as i32 });
    }
}
unsafe extern "C" fn debug_print_regex(mut r: *const regex) {
    if r.is_null() {
        fputs_unlocked(b"//\0" as *const u8 as *const libc::c_char, stdout);
        return;
    }
    putchar_unlocked('/' as i32);
    debug_print_regex_pattern(((*r).re).as_ptr(), (*r).sz);
    putchar_unlocked('/' as i32);
}
unsafe extern "C" fn debug_print_addr(mut a: *const addr) {
    if a.is_null() {
        return;
    }
    match (*a).addr_type as libc::c_uint {
        0 => {
            fputs_unlocked(b"[ADDR-NULL]\0" as *const u8 as *const libc::c_char, stdout);
        }
        1 => {
            debug_print_regex((*a).addr_regex);
            debug_print_regex_flags((*a).addr_regex, 1 as libc::c_int != 0);
        }
        2 => {
            printf(b"%lu\0" as *const u8 as *const libc::c_char, (*a).addr_number);
        }
        3 => {
            printf(
                b"%lu~%lu\0" as *const u8 as *const libc::c_char,
                (*a).addr_number,
                (*a).addr_step,
            );
        }
        4 => {
            printf(b"+%lu\0" as *const u8 as *const libc::c_char, (*a).addr_step);
        }
        5 => {
            printf(b"~%lu\0" as *const u8 as *const libc::c_char, (*a).addr_step);
        }
        6 => {
            putchar_unlocked('$' as i32);
        }
        _ => {}
    };
}
unsafe extern "C" fn debug_print_subst_replacement(mut r: *const replacement) {
    let mut last_repl_type: replacement_types = REPL_ASIS;
    if r.is_null() {
        return;
    }
    let mut p: *const replacement = r;
    while !p.is_null() {
        if (*p).repl_type as libc::c_uint != last_repl_type as libc::c_uint {
            putchar_unlocked('\\' as i32);
            if (*p).repl_type as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                putchar_unlocked('E' as i32);
            } else if (*p).repl_type as libc::c_uint
                == REPL_UPPERCASE as libc::c_int as libc::c_uint
            {
                putchar_unlocked('U' as i32);
            } else if (*p).repl_type as libc::c_uint
                == REPL_LOWERCASE as libc::c_int as libc::c_uint
            {
                putchar_unlocked('L' as i32);
            } else if (*p).repl_type as libc::c_uint
                & REPL_MODIFIERS as libc::c_int as libc::c_uint
                == REPL_UPPERCASE_FIRST as libc::c_int as libc::c_uint
            {
                putchar_unlocked('u' as i32);
            } else if (*p).repl_type as libc::c_uint
                & REPL_MODIFIERS as libc::c_int as libc::c_uint
                == REPL_LOWERCASE_FIRST as libc::c_int as libc::c_uint
            {
                putchar_unlocked('l' as i32);
            }
            last_repl_type = (*p).repl_type;
        }
        if (*p).prefix_length != 0 {
            if 0 != 0 && 0 != 0
                && (1 as libc::c_int as size_t).wrapping_mul((*p).prefix_length)
                    <= 8 as libc::c_int as libc::c_ulong
                && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
            {
                ({
                    let mut __ptr: *const libc::c_char = (*p).prefix
                        as *const libc::c_char;
                    let mut __stream: *mut FILE = stdout;
                    let mut __cnt: size_t = 0;
                    __cnt = (1 as libc::c_int as size_t)
                        .wrapping_mul((*p).prefix_length);
                    while __cnt > 0 as libc::c_int as libc::c_ulong {
                        let fresh3 = __ptr;
                        __ptr = __ptr.offset(1);
                        if putc_unlocked(*fresh3 as libc::c_int, __stream)
                            == -(1 as libc::c_int)
                        {
                            break;
                        }
                        __cnt = __cnt.wrapping_sub(1);
                        __cnt;
                    }
                    0u8
                });
            } else {
                if 0 != 0
                    && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
                    || 0 != 0 && (*p).prefix_length == 0 as libc::c_int as libc::c_ulong
                {} else {
                    fwrite_unlocked(
                        (*p).prefix as *const libc::c_void,
                        1 as libc::c_int as size_t,
                        (*p).prefix_length,
                        stdout,
                    );
                };
            };
            0u8;
        }
        if (*p).subst_id != -(1 as libc::c_int) {
            if (*p).subst_id == 0 as libc::c_int {
                putchar_unlocked('&' as i32);
            } else {
                printf(b"\\%d\0" as *const u8 as *const libc::c_char, (*p).subst_id);
            }
        }
        p = (*p).next;
    }
}
unsafe extern "C" fn debug_print_output_file(mut o: *const output) {
    if o.is_null() {
        return;
    }
    fputs_unlocked((*o).name, stdout);
}
unsafe extern "C" fn debug_print_subst(mut s: *const subst) {
    if s.is_null() {
        return;
    }
    debug_print_regex((*s).regx);
    debug_print_subst_replacement((*s).replacement);
    putchar_unlocked('/' as i32);
    debug_print_regex_flags((*s).regx, 0 as libc::c_int != 0);
    if (*s).global() != 0 {
        putchar_unlocked('g' as i32);
    }
    if (*s).eval() != 0 {
        putchar_unlocked('e' as i32);
    }
    if (*s).print() != 0 {
        putchar_unlocked('p' as i32);
    }
    if (*s).numb != 0 {
        printf(b"%lu\0" as *const u8 as *const libc::c_char, (*s).numb);
    }
    if !((*s).outf).is_null() {
        putchar_unlocked('w' as i32);
        debug_print_output_file((*s).outf);
    }
}
unsafe extern "C" fn debug_print_translation(mut sc: *const sed_cmd) {
    let mut i: libc::c_uint = 0;
    if mb_cur_max > 1 as libc::c_int {
        putchar_unlocked('/' as i32);
        i = 0 as libc::c_int as libc::c_uint;
        while !(*((*sc).x.translatemb)
            .offset((2 as libc::c_int as libc::c_uint).wrapping_mul(i) as isize))
            .is_null()
        {
            fputs_unlocked(
                *((*sc).x.translatemb)
                    .offset((2 as libc::c_int as libc::c_uint).wrapping_mul(i) as isize),
                stdout,
            );
            i = i.wrapping_add(1);
            i;
        }
        putchar_unlocked('/' as i32);
        i = 0 as libc::c_int as libc::c_uint;
        while !(*((*sc).x.translatemb)
            .offset((2 as libc::c_int as libc::c_uint).wrapping_mul(i) as isize))
            .is_null()
        {
            fputs_unlocked(
                *((*sc).x.translatemb)
                    .offset(
                        (2 as libc::c_int as libc::c_uint)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                    ),
                stdout,
            );
            i = i.wrapping_add(1);
            i;
        }
        putchar_unlocked('/' as i32);
    } else {
        putchar_unlocked('/' as i32);
        i = 0 as libc::c_int as libc::c_uint;
        while i < 256 as libc::c_int as libc::c_uint {
            if *((*sc).x.translate).offset(i as isize) as libc::c_int
                != i as libc::c_uchar as libc::c_int
            {
                putchar_unlocked(i as libc::c_uchar as libc::c_int);
            }
            i = i.wrapping_add(1);
            i;
        }
        putchar_unlocked('/' as i32);
        i = 0 as libc::c_int as libc::c_uint;
        while i < 256 as libc::c_int as libc::c_uint {
            if *((*sc).x.translate).offset(i as isize) as libc::c_int
                != i as libc::c_uchar as libc::c_int
            {
                putchar_unlocked(*((*sc).x.translate).offset(i as isize) as libc::c_int);
            }
            i = i.wrapping_add(1);
            i;
        }
        putchar_unlocked('/' as i32);
    };
}
unsafe extern "C" fn debug_print_function(
    mut program: *const vector,
    mut sc: *const sed_cmd,
) {
    if sc.is_null() {
        return;
    }
    putchar_unlocked((*sc).cmd as libc::c_int);
    let mut current_block_26: u64;
    match (*sc).cmd as libc::c_int {
        58 => {
            printf(b"%s\0" as *const u8 as *const libc::c_char, (*sc).x.label_name);
            current_block_26 = 5330834795799507926;
        }
        35 => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"sed/debug.c\0" as *const u8 as *const libc::c_char,
                291 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"void debug_print_function(const struct vector *, const struct sed_cmd *)\0",
                ))
                    .as_ptr(),
            );
            'c_6224: {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"sed/debug.c\0" as *const u8 as *const libc::c_char,
                    291 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 73],
                        &[libc::c_char; 73],
                    >(
                        b"void debug_print_function(const struct vector *, const struct sed_cmd *)\0",
                    ))
                        .as_ptr(),
                );
            };
            current_block_26 = 8370701411959146836;
        }
        97 | 99 | 105 => {
            current_block_26 = 8370701411959146836;
        }
        98 | 116 | 84 => {
            if (*sc).x.jump_index < (*program).v_length {
                let mut label_name: *const libc::c_char = (*((*program).v)
                    .offset((*sc).x.jump_index as isize))
                    .x
                    .label_name;
                if !label_name.is_null() {
                    printf(b" %s\0" as *const u8 as *const libc::c_char, label_name);
                }
            }
            current_block_26 = 5330834795799507926;
        }
        101 => {
            putchar_unlocked(' ' as i32);
            if 0 != 0 && 0 != 0
                && (1 as libc::c_int as size_t).wrapping_mul((*sc).x.cmd_txt.text_length)
                    <= 8 as libc::c_int as libc::c_ulong
                && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
            {
                ({
                    let mut __ptr: *const libc::c_char = (*sc).x.cmd_txt.text
                        as *const libc::c_char;
                    let mut __stream: *mut FILE = stdout;
                    let mut __cnt: size_t = 0;
                    __cnt = (1 as libc::c_int as size_t)
                        .wrapping_mul((*sc).x.cmd_txt.text_length);
                    while __cnt > 0 as libc::c_int as libc::c_ulong {
                        let fresh5 = __ptr;
                        __ptr = __ptr.offset(1);
                        if putc_unlocked(*fresh5 as libc::c_int, __stream)
                            == -(1 as libc::c_int)
                        {
                            break;
                        }
                        __cnt = __cnt.wrapping_sub(1);
                        __cnt;
                    }
                    0u8
                });
            } else {
                if 0 != 0
                    && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
                    || 0 != 0
                        && (*sc).x.cmd_txt.text_length
                            == 0 as libc::c_int as libc::c_ulong
                {} else {
                    fwrite_unlocked(
                        (*sc).x.cmd_txt.text as *const libc::c_void,
                        1 as libc::c_int as size_t,
                        (*sc).x.cmd_txt.text_length,
                        stdout,
                    );
                };
            };
            0u8;
            current_block_26 = 5330834795799507926;
        }
        76 | 108 | 113 | 81 => {
            if (*sc).x.int_arg != -(1 as libc::c_int) {
                printf(b" %d\0" as *const u8 as *const libc::c_char, (*sc).x.int_arg);
            }
            current_block_26 = 5330834795799507926;
        }
        114 => {
            putchar_unlocked(' ' as i32);
            fputs_unlocked((*sc).x.readcmd.fname, stdout);
            current_block_26 = 5330834795799507926;
        }
        82 => {
            putchar_unlocked(' ' as i32);
            fputs_unlocked((*(*sc).x.inf).name, stdout);
            current_block_26 = 5330834795799507926;
        }
        115 => {
            debug_print_subst((*sc).x.cmd_subst);
            current_block_26 = 5330834795799507926;
        }
        118 => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"sed/debug.c\0" as *const u8 as *const libc::c_char,
                382 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"void debug_print_function(const struct vector *, const struct sed_cmd *)\0",
                ))
                    .as_ptr(),
            );
            'c_4836: {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"sed/debug.c\0" as *const u8 as *const libc::c_char,
                    382 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 73],
                        &[libc::c_char; 73],
                    >(
                        b"void debug_print_function(const struct vector *, const struct sed_cmd *)\0",
                    ))
                        .as_ptr(),
                );
            };
            current_block_26 = 6498688920109107990;
        }
        87 => {
            current_block_26 = 6498688920109107990;
        }
        119 => {
            debug_print_output_file((*sc).x.outf);
            current_block_26 = 5330834795799507926;
        }
        121 => {
            debug_print_translation(sc);
            current_block_26 = 5330834795799507926;
        }
        61 | 123 | 125 | 68 | 100 | 70 | 103 | 71 | 104 | 72 | 110 | 78 | 80 | 112 | 120
        | 122 => {
            current_block_26 = 5330834795799507926;
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"sed/debug.c\0" as *const u8 as *const libc::c_char,
                404 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"void debug_print_function(const struct vector *, const struct sed_cmd *)\0",
                ))
                    .as_ptr(),
            );
            'c_4521: {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"sed/debug.c\0" as *const u8 as *const libc::c_char,
                    404 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 73],
                        &[libc::c_char; 73],
                    >(
                        b"void debug_print_function(const struct vector *, const struct sed_cmd *)\0",
                    ))
                        .as_ptr(),
                );
            };
            current_block_26 = 5330834795799507926;
        }
    }
    match current_block_26 {
        8370701411959146836 => {
            fputs_unlocked(b"\\\0" as *const u8 as *const libc::c_char, stdout);
            if (*sc).x.cmd_txt.text_length != 0 {
                if 0 != 0 && 0 != 0
                    && (1 as libc::c_int as size_t)
                        .wrapping_mul((*sc).x.cmd_txt.text_length)
                        <= 8 as libc::c_int as libc::c_ulong
                    && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
                {
                    ({
                        let mut __ptr: *const libc::c_char = (*sc).x.cmd_txt.text
                            as *const libc::c_char;
                        let mut __stream: *mut FILE = stdout;
                        let mut __cnt: size_t = 0;
                        __cnt = (1 as libc::c_int as size_t)
                            .wrapping_mul((*sc).x.cmd_txt.text_length);
                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                            let fresh4 = __ptr;
                            __ptr = __ptr.offset(1);
                            if putc_unlocked(*fresh4 as libc::c_int, __stream)
                                == -(1 as libc::c_int)
                            {
                                break;
                            }
                            __cnt = __cnt.wrapping_sub(1);
                            __cnt;
                        }
                        0u8
                    });
                } else {
                    if 0 != 0
                        && 1 as libc::c_int as size_t
                            == 0 as libc::c_int as libc::c_ulong
                        || 0 != 0
                            && (*sc).x.cmd_txt.text_length
                                == 0 as libc::c_int as libc::c_ulong
                    {} else {
                        fwrite_unlocked(
                            (*sc).x.cmd_txt.text as *const libc::c_void,
                            1 as libc::c_int as size_t,
                            (*sc).x.cmd_txt.text_length,
                            stdout,
                        );
                    };
                };
                0u8;
            }
        }
        6498688920109107990 => {
            debug_print_output_file((*sc).x.outf);
        }
        _ => {}
    };
}
pub unsafe extern "C" fn debug_print_command(
    mut program: *const vector,
    mut sc: *const sed_cmd,
) {
    let mut addr_bang: bool = false;
    if program.is_null() {
        return;
    }
    if (*sc).cmd as libc::c_int == '}' as i32 {
        block_level -= 1;
        block_level;
    }
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < block_level {
        fputs_unlocked(b"  \0" as *const u8 as *const libc::c_char, stdout);
        j += 1;
        j;
    }
    debug_print_addr((*sc).a1);
    if !((*sc).a2).is_null() {
        putchar_unlocked(',' as i32);
    }
    debug_print_addr((*sc).a2);
    addr_bang = (*sc).addr_bang != 0;
    if (*sc).cmd as libc::c_int == '{' as i32 {
        addr_bang = !addr_bang;
    }
    if addr_bang {
        putchar_unlocked('!' as i32);
    }
    if !((*sc).a1).is_null() || !((*sc).a2).is_null() {
        putchar_unlocked(' ' as i32);
    }
    debug_print_function(program, sc);
    putchar_unlocked('\n' as i32);
    if (*sc).cmd as libc::c_int == '{' as i32 {
        block_level += 1;
        block_level;
    }
}
pub unsafe extern "C" fn debug_print_program(mut program: *const vector) {
    if program.is_null() {
        return;
    }
    block_level = 1 as libc::c_int;
    puts(b"SED PROGRAM:\0" as *const u8 as *const libc::c_char);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*program).v_length {
        debug_print_command(program, &mut *((*program).v).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    block_level = 0 as libc::c_int;
}
