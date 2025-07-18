use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type buffer;
    fn rpl_free(_: *mut libc::c_void);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memrchr(
        __s: *const libc::c_void,
        __c: libc::c_int,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn rpl_re_set_syntax(__syntax: reg_syntax_t) -> reg_syntax_t;
    fn rpl_re_compile_pattern(
        __pattern: *const libc::c_char,
        __length: size_t,
        __buffer: *mut re_pattern_buffer,
    ) -> *const libc::c_char;
    fn rpl_re_search(
        __buffer: *mut re_pattern_buffer,
        __String: *const libc::c_char,
        __length: regoff_t,
        __start: regoff_t,
        __range: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    fn rpl_regfree(__preg: *mut regex_t);
    fn dfaalloc() -> *mut dfa;
    fn dfasyntax(_: *mut dfa, _: *const localeinfo, _: reg_syntax_t, _: libc::c_int);
    fn dfacomp(_: *const libc::c_char, _: idx_t, _: *mut dfa, _: bool);
    fn dfaexec(
        d: *mut dfa,
        begin: *const libc::c_char,
        end: *mut libc::c_char,
        allow_nl: bool,
        count: *mut idx_t,
        backref: *mut bool,
    ) -> *mut libc::c_char;
    fn dfasuperset(d: *const dfa) -> *mut dfa;
    fn dfaisfast(_: *const dfa) -> bool;
    fn dfafree(_: *mut dfa);
    fn panic(str: *const libc::c_char, _: ...);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn get_buffer(b: *const buffer) -> *mut libc::c_char;
    fn size_buffer(b: *const buffer) -> size_t;
    fn bad_prog(why: *const libc::c_char);
    fn normalize_text(
        text: *mut libc::c_char,
        len: size_t,
        buftype: text_types,
    ) -> size_t;
    static mut buffer_delimiter: libc::c_char;
    static mut extended_regexp_flags: libc::c_int;
    static mut localeinfo: localeinfo;
    static mut posixicity: posixicity_types;
}
pub type size_t = libc::c_ulong;
pub type wint_t = libc::c_uint;
pub type ptrdiff_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type idx_t = ptrdiff_t;
pub type __re_size_t = size_t;
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
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct localeinfo {
    pub multibyte: bool,
    pub simple: bool,
    pub using_utf8: bool,
    pub sbclen: [libc::c_schar; 256],
    pub sbctowc: [wint_t; 256],
}
pub type C2RustUnnamed = libc::c_uint;
pub const DFA_PLUS_WARN: C2RustUnnamed = 32;
pub const DFA_STAR_WARN: C2RustUnnamed = 16;
pub const DFA_STRAY_BACKSLASH_WARN: C2RustUnnamed = 8;
pub const DFA_CONFUSING_BRACKETS_ERROR: C2RustUnnamed = 4;
pub const DFA_EOL_NUL: C2RustUnnamed = 2;
pub const DFA_ANCHOR: C2RustUnnamed = 1;
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
pub type text_types = libc::c_uint;
pub const TEXT_REGEX: text_types = 2;
pub const TEXT_REPLACEMENT: text_types = 1;
pub const TEXT_BUFFER: text_types = 0;
pub type posixicity_types = libc::c_uint;
pub const POSIXLY_BASIC: posixicity_types = 2;
pub const POSIXLY_CORRECT: posixicity_types = 1;
pub const POSIXLY_EXTENDED: posixicity_types = 0;
static mut errors: [libc::c_char; 72] = unsafe {
    *::std::mem::transmute::<
        &[u8; 72],
        &[libc::c_char; 72],
    >(b"no previous regular expression\0cannot specify modifiers on empty regexp\0")
};
pub unsafe extern "C" fn dfaerror(mut mesg: *const libc::c_char) {
    panic(b"%s\0" as *const u8 as *const libc::c_char, mesg);
}
pub unsafe extern "C" fn dfawarn(mut mesg: *const libc::c_char) {
    if (getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char)).is_null() {
        dfaerror(mesg);
    }
}
unsafe extern "C" fn compile_regex_1(
    mut new_regex: *mut regex,
    mut needed_sub: libc::c_int,
) {
    let mut error: *const libc::c_char = 0 as *const libc::c_char;
    let mut syntax: libc::c_int = (if extended_regexp_flags & 1 as libc::c_int != 0 {
        ((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int
            | (((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int
    } else {
        ((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int
            | ((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
    }) as libc::c_int;
    syntax = (syntax as libc::c_ulong
        & !((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int))
        as libc::c_int;
    syntax = (syntax as libc::c_ulong
        | ((((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) as libc::c_int;
    match posixicity as libc::c_uint {
        0 => {
            syntax = (syntax as libc::c_ulong
                & !((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int)) as libc::c_int;
        }
        1 => {
            syntax = (syntax as libc::c_ulong
                | (((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) as libc::c_int;
        }
        2 => {
            syntax = (syntax as libc::c_ulong
                | ((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int)) as libc::c_int;
            if extended_regexp_flags & 1 as libc::c_int == 0 {
                syntax = (syntax as libc::c_ulong
                    | ((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_int;
            }
        }
        _ => {}
    }
    if (*new_regex).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        syntax = (syntax as libc::c_ulong
            | ((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) as libc::c_int;
    } else {
        (*new_regex)
            .pattern
            .fastmap = malloc(
            ((1 as libc::c_int)
                << (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)) as libc::c_ulong,
        ) as *mut libc::c_char;
    }
    syntax = (syntax as libc::c_ulong
        | if needed_sub != 0 {
            0 as libc::c_int as libc::c_ulong
        } else {
            (((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int
        }) as libc::c_int;
    if (*new_regex).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        syntax = (syntax as libc::c_ulong
            & !(((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int)) as libc::c_int;
        syntax = (syntax as libc::c_ulong
            | ((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) as libc::c_int;
    }
    rpl_re_set_syntax(syntax as reg_syntax_t);
    error = rpl_re_compile_pattern(
        ((*new_regex).re).as_mut_ptr(),
        (*new_regex).sz,
        &mut (*new_regex).pattern,
    );
    ((*new_regex).pattern)
        .set_newline_anchor(
            (buffer_delimiter as libc::c_int == '\n' as i32
                && (*new_regex).flags & (1 as libc::c_int) << 2 as libc::c_int
                    != 0 as libc::c_int) as libc::c_int as libc::c_uint,
        );
    (*new_regex).pattern.translate = 0 as *mut libc::c_uchar;
    if !error.is_null() {
        bad_prog(error);
    }
    if needed_sub != 0
        && (*new_regex).pattern.re_nsub
            < (needed_sub - 1 as libc::c_int) as libc::c_ulong
        && posixicity as libc::c_uint == POSIXLY_EXTENDED as libc::c_int as libc::c_uint
    {
        let mut buf: [libc::c_char; 200] = [0; 200];
        sprintf(
            buf.as_mut_ptr(),
            dcgettext(
                0 as *const libc::c_char,
                b"invalid reference \\%d on `s' command's RHS\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            needed_sub - 1 as libc::c_int,
        );
        bad_prog(buf.as_mut_ptr());
    }
    let mut dfaopts: libc::c_int = if buffer_delimiter as libc::c_int == '\n' as i32 {
        0 as libc::c_int
    } else {
        DFA_EOL_NUL as libc::c_int
    };
    (*new_regex).dfa = dfaalloc();
    dfasyntax((*new_regex).dfa, &mut localeinfo, syntax as reg_syntax_t, dfaopts);
    dfacomp(
        ((*new_regex).re).as_mut_ptr(),
        (*new_regex).sz as idx_t,
        (*new_regex).dfa,
        1 as libc::c_int != 0,
    );
    if (*new_regex).sz == 1 as libc::c_int as libc::c_ulong {
        if *((*new_regex).re).as_mut_ptr().offset(0 as libc::c_int as isize)
            as libc::c_int == '^' as i32
        {
            (*new_regex).begline = 1 as libc::c_int != 0;
        }
        if *((*new_regex).re).as_mut_ptr().offset(0 as libc::c_int as isize)
            as libc::c_int == '$' as i32
        {
            (*new_regex).endline = 1 as libc::c_int != 0;
        }
    }
}
pub unsafe extern "C" fn compile_regex(
    mut b: *mut buffer,
    mut flags: libc::c_int,
    mut needed_sub: libc::c_int,
) -> *mut regex {
    let mut new_regex: *mut regex = 0 as *mut regex;
    let mut re_len: size_t = 0;
    if size_buffer(b) == 0 as libc::c_int as libc::c_ulong {
        if flags > 0 as libc::c_int {
            bad_prog(
                dcgettext(
                    0 as *const libc::c_char,
                    errors
                        .as_ptr()
                        .offset(
                            ::std::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                                as isize,
                        ),
                    5 as libc::c_int,
                ),
            );
        }
        return 0 as *mut regex;
    }
    re_len = size_buffer(b);
    new_regex = xzalloc(
        (::std::mem::size_of::<regex>() as libc::c_ulong)
            .wrapping_add(re_len)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) as *mut regex;
    (*new_regex).flags = flags;
    memcpy(
        ((*new_regex).re).as_mut_ptr() as *mut libc::c_void,
        get_buffer(b) as *const libc::c_void,
        re_len,
    );
    (*new_regex).sz = normalize_text(((*new_regex).re).as_mut_ptr(), re_len, TEXT_REGEX);
    compile_regex_1(new_regex, needed_sub);
    return new_regex;
}
pub unsafe extern "C" fn match_regex(
    mut regex: *mut regex,
    mut buf: *mut libc::c_char,
    mut buflen: size_t,
    mut buf_start_offset: size_t,
    mut regarray: *mut re_registers,
    mut regsize: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    static mut regex_last: *mut regex = 0 as *const regex as *mut regex;
    if regex.is_null() {
        regex = regex_last;
        if regex_last.is_null() {
            bad_prog(
                dcgettext(0 as *const libc::c_char, errors.as_ptr(), 5 as libc::c_int),
            );
        }
    } else {
        regex_last = regex;
    }
    if buflen >= 2147483647 as libc::c_int as libc::c_ulong {
        panic(
            dcgettext(
                0 as *const libc::c_char,
                b"regex input buffer length larger than INT_MAX\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if ((*regex).pattern).no_sub() as libc::c_int != 0 && regsize != 0 {
        if !((*regex).dfa).is_null() {
            dfafree((*regex).dfa);
            rpl_free((*regex).dfa as *mut libc::c_void);
            (*regex).dfa = 0 as *mut dfa;
        }
        rpl_regfree(&mut (*regex).pattern);
        compile_regex_1(regex, regsize);
    }
    ((*regex).pattern).set_regs_allocated(1 as libc::c_int as libc::c_uint);
    if (*regex).begline as libc::c_int != 0 || (*regex).endline as libc::c_int != 0 {
        let mut offset: size_t = 0;
        if (*regex).endline {
            let mut p: *const libc::c_char = 0 as *const libc::c_char;
            if (*regex).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                p = memchr(
                    buf.offset(buf_start_offset as isize) as *const libc::c_void,
                    buffer_delimiter as libc::c_int,
                    buflen.wrapping_sub(buf_start_offset),
                ) as *const libc::c_char;
            }
            offset = if !p.is_null() {
                p.offset_from(buf) as libc::c_long as libc::c_ulong
            } else {
                buflen
            };
        } else if buf_start_offset == 0 as libc::c_int as libc::c_ulong {
            offset = 0 as libc::c_int as size_t;
        } else if (*regex).flags & (1 as libc::c_int) << 2 as libc::c_int == 0 {
            return 0 as libc::c_int
        } else if *buf
            .offset(
                buf_start_offset.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int == buffer_delimiter as libc::c_int
        {
            offset = buf_start_offset;
        } else {
            let mut p_0: *const libc::c_char = memchr(
                buf.offset(buf_start_offset as isize) as *const libc::c_void,
                buffer_delimiter as libc::c_int,
                buflen.wrapping_sub(buf_start_offset),
            ) as *const libc::c_char;
            if p_0.is_null() {
                return 0 as libc::c_int;
            }
            offset = (p_0.offset_from(buf) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t;
        }
        if regsize != 0 {
            let mut i: size_t = 0;
            if ((*regarray).start).is_null() {
                (*regarray)
                    .start = (if ::std::mem::size_of::<regoff_t>() as libc::c_ulong
                    == 1 as libc::c_int as libc::c_ulong
                {
                    xzalloc(1 as libc::c_int as size_t)
                } else {
                    xcalloc(
                        1 as libc::c_int as size_t,
                        ::std::mem::size_of::<regoff_t>() as libc::c_ulong,
                    )
                }) as *mut regoff_t;
                (*regarray)
                    .end = (if ::std::mem::size_of::<regoff_t>() as libc::c_ulong
                    == 1 as libc::c_int as libc::c_ulong
                {
                    xzalloc(1 as libc::c_int as size_t)
                } else {
                    xcalloc(
                        1 as libc::c_int as size_t,
                        ::std::mem::size_of::<regoff_t>() as libc::c_ulong,
                    )
                }) as *mut regoff_t;
                (*regarray).num_regs = 1 as libc::c_int as __re_size_t;
            }
            *((*regarray).start).offset(0 as libc::c_int as isize) = offset as regoff_t;
            *((*regarray).end).offset(0 as libc::c_int as isize) = offset as regoff_t;
            i = 1 as libc::c_int as size_t;
            while i < (*regarray).num_regs {
                let ref mut fresh0 = *((*regarray).end).offset(i as isize);
                *fresh0 = -(1 as libc::c_int) as regoff_t;
                *((*regarray).start).offset(i as isize) = *fresh0;
                i = i.wrapping_add(1);
                i;
            }
        }
        return 1 as libc::c_int;
    }
    if buf_start_offset == 0 as libc::c_int as libc::c_ulong {
        let mut superset: *mut dfa = dfasuperset((*regex).dfa);
        if !superset.is_null()
            && (dfaexec(
                superset,
                buf,
                buf.offset(buflen as isize),
                1 as libc::c_int != 0,
                0 as *mut idx_t,
                0 as *mut bool,
            ))
                .is_null()
        {
            return 0 as libc::c_int;
        }
        if regsize == 0 && (*regex).flags & (1 as libc::c_int) << 2 as libc::c_int != 0
            || superset.is_null() && dfaisfast((*regex).dfa) as libc::c_int != 0
        {
            let mut backref: bool = 0 as libc::c_int != 0;
            if (dfaexec(
                (*regex).dfa,
                buf,
                buf.offset(buflen as isize),
                1 as libc::c_int != 0,
                0 as *mut idx_t,
                &mut backref,
            ))
                .is_null()
            {
                return 0 as libc::c_int;
            }
            if regsize == 0
                && (*regex).flags & (1 as libc::c_int) << 2 as libc::c_int != 0
                && !backref
            {
                return 1 as libc::c_int;
            }
        }
    }
    if (*regex).flags & (1 as libc::c_int) << 2 as libc::c_int != 0
        && buffer_delimiter as libc::c_int != '\n' as i32
    {
        let mut beg: *const libc::c_char = 0 as *const libc::c_char;
        let mut end: *const libc::c_char = 0 as *const libc::c_char;
        let mut start: *const libc::c_char = 0 as *const libc::c_char;
        beg = buf;
        if buf_start_offset > 0 as libc::c_int as libc::c_ulong {
            let mut eol: *const libc::c_char = memrchr(
                buf as *const libc::c_void,
                buffer_delimiter as libc::c_int,
                buf_start_offset,
            ) as *const libc::c_char;
            if !eol.is_null() {
                beg = eol.offset(1 as libc::c_int as isize);
            }
        }
        start = buf.offset(buf_start_offset as isize);
        loop {
            end = memchr(
                beg as *const libc::c_void,
                buffer_delimiter as libc::c_int,
                buf.offset(buflen as isize).offset_from(beg) as libc::c_long
                    as libc::c_ulong,
            ) as *const libc::c_char;
            if end.is_null() {
                end = buf.offset(buflen as isize);
            }
            ret = rpl_re_search(
                &mut (*regex).pattern,
                beg,
                end.offset_from(beg) as libc::c_long,
                start.offset_from(beg) as libc::c_long,
                end.offset_from(start) as libc::c_long,
                if regsize != 0 { regarray } else { 0 as *mut re_registers },
            ) as libc::c_int;
            if ret > -(1 as libc::c_int) {
                let mut i_0: size_t = 0;
                ret = (ret as libc::c_long + beg.offset_from(buf) as libc::c_long)
                    as libc::c_int;
                if regsize != 0 {
                    i_0 = 0 as libc::c_int as size_t;
                    while i_0 < (*regarray).num_regs {
                        if *((*regarray).start).offset(i_0 as isize)
                            > -(1 as libc::c_int) as libc::c_long
                        {
                            let ref mut fresh1 = *((*regarray).start)
                                .offset(i_0 as isize);
                            *fresh1 += beg.offset_from(buf) as libc::c_long;
                        }
                        if *((*regarray).end).offset(i_0 as isize)
                            > -(1 as libc::c_int) as libc::c_long
                        {
                            let ref mut fresh2 = *((*regarray).end).offset(i_0 as isize);
                            *fresh2 += beg.offset_from(buf) as libc::c_long;
                        }
                        i_0 = i_0.wrapping_add(1);
                        i_0;
                    }
                }
                break;
            } else {
                if end == buf.offset(buflen as isize) as *const libc::c_char {
                    break;
                }
                start = end.offset(1 as libc::c_int as isize);
                beg = start;
            }
        }
    } else {
        ret = rpl_re_search(
            &mut (*regex).pattern,
            buf,
            buflen as regoff_t,
            buf_start_offset as regoff_t,
            buflen.wrapping_sub(buf_start_offset) as regoff_t,
            if regsize != 0 { regarray } else { 0 as *mut re_registers },
        ) as libc::c_int;
    }
    return (ret > -(1 as libc::c_int)) as libc::c_int;
}
