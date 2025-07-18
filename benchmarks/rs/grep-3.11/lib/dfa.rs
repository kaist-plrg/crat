use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn abort() -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn rpl_free(ptr: *mut libc::c_void);
    fn dfawarn(_: *const libc::c_char);
    fn dfaerror(_: *const libc::c_char);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn isalnum(_: libc::c_int) -> libc::c_int;
    fn isalpha(_: libc::c_int) -> libc::c_int;
    fn iscntrl(_: libc::c_int) -> libc::c_int;
    fn isdigit(_: libc::c_int) -> libc::c_int;
    fn islower(_: libc::c_int) -> libc::c_int;
    fn isgraph(_: libc::c_int) -> libc::c_int;
    fn isprint(_: libc::c_int) -> libc::c_int;
    fn ispunct(_: libc::c_int) -> libc::c_int;
    fn isspace(_: libc::c_int) -> libc::c_int;
    fn isupper(_: libc::c_int) -> libc::c_int;
    fn isxdigit(_: libc::c_int) -> libc::c_int;
    fn isblank(_: libc::c_int) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn wcrtomb(__s: *mut libc::c_char, __wc: wchar_t, __ps: *mut mbstate_t) -> size_t;
    fn wctob(__c: wint_t) -> libc::c_int;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn iswprint(__wc: wint_t) -> libc::c_int;
    fn iswspace(__wc: wint_t) -> libc::c_int;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn ximalloc(s: idx_t) -> *mut libc::c_void;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn xizalloc(s: idx_t) -> *mut libc::c_void;
    fn xicalloc(n: idx_t, s: idx_t) -> *mut libc::c_void;
    fn xirealloc(p: *mut libc::c_void, s: idx_t) -> *mut libc::c_void;
    fn xreallocarray(p: *mut libc::c_void, n: size_t, s: size_t) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    fn ximemdup0(p: *const libc::c_void, s: idx_t) -> *mut libc::c_char;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn case_folded_counterparts(_: wint_t, _: *mut wchar_t) -> libc::c_int;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type __int32_t = libc::c_int;
pub type __uint64_t = libc::c_ulong;
pub type __uint_least64_t = __uint64_t;
pub type uint_least64_t = __uint_least64_t;
pub type idx_t = ptrdiff_t;
pub type reg_syntax_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct localeinfo {
    pub multibyte: bool,
    pub simple: bool,
    pub using_utf8: bool,
    pub sbclen: [libc::c_schar; 256],
    pub sbctowc: [wint_t; 256],
}
pub type wint_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dfamust {
    pub exact: bool,
    pub begline: bool,
    pub endline: bool,
    pub must: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dfa {
    pub charclasses: *mut charclass,
    pub cindex: idx_t,
    pub calloc: idx_t,
    pub canychar: ptrdiff_t,
    pub lex: lexer_state,
    pub parse: parser_state,
    pub tokens: *mut token,
    pub tindex: idx_t,
    pub talloc: idx_t,
    pub depth: idx_t,
    pub nleaves: idx_t,
    pub nregexps: idx_t,
    pub fast: bool,
    pub epsilon: bool,
    pub utf8_anychar_classes: [token; 9],
    pub mbs: mbstate_t,
    pub multibyte_prop: *mut libc::c_char,
    pub superset: *mut dfa,
    pub states: *mut dfa_state,
    pub sindex: state_num,
    pub salloc: idx_t,
    pub follows: *mut position_set,
    pub searchflag: bool,
    pub constraints: *mut libc::c_int,
    pub separates: *mut libc::c_int,
    pub tralloc: state_num,
    pub trcount: libc::c_int,
    pub min_trcount: libc::c_int,
    pub trans: *mut *mut state_num,
    pub fails: *mut *mut state_num,
    pub success: *mut libc::c_char,
    pub newlines: *mut state_num,
    pub initstate_notbol: state_num,
    pub mb_follows: position_set,
    pub mb_trans: *mut *mut state_num,
    pub mb_trcount: state_num,
    pub syntax: regex_syntax,
    pub dfaexec: Option::<
        unsafe extern "C" fn(
            *mut dfa,
            *const libc::c_char,
            *mut libc::c_char,
            bool,
            *mut idx_t,
            *mut bool,
        ) -> *mut libc::c_char,
    >,
    pub localeinfo: localeinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regex_syntax {
    pub syntax_bits: reg_syntax_t,
    pub dfaopts: libc::c_int,
    pub syntax_bits_set: bool,
    pub case_fold: bool,
    pub eolbyte: libc::c_uchar,
    pub sbit: [libc::c_char; 256],
    pub never_trail: [bool; 256],
    pub letters: charclass,
    pub newline: charclass,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct charclass {
    pub w: [charclass_word; 4],
}
pub type charclass_word = uint_least64_t;
pub type state_num = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct position_set {
    pub elems: *mut position,
    pub nelem: idx_t,
    pub alloc: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct position {
    pub index: idx_t,
    pub constraint: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dfa_state {
    pub hash: size_t,
    pub elems: position_set,
    pub context: libc::c_uchar,
    pub constraint: libc::c_ushort,
    pub mbps: position_set,
    pub mb_trindex: state_num,
}
pub type __mbstate_t = mbstate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
pub type token = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_state {
    pub tok: token,
    pub depth: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lexer_state {
    pub ptr: *const libc::c_char,
    pub left: idx_t,
    pub lasttok: token,
    pub parens: idx_t,
    pub minrep: libc::c_int,
    pub maxrep: libc::c_int,
    pub wctok: wint_t,
    pub brack: mb_char_classes,
    pub laststart: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mb_char_classes {
    pub cset: ptrdiff_t,
    pub invert: bool,
    pub chars: *mut wchar_t,
    pub nchars: idx_t,
    pub nchars_alloc: idx_t,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const DFA_PLUS_WARN: C2RustUnnamed_0 = 32;
pub const DFA_STAR_WARN: C2RustUnnamed_0 = 16;
pub const DFA_STRAY_BACKSLASH_WARN: C2RustUnnamed_0 = 8;
pub const DFA_CONFUSING_BRACKETS_ERROR: C2RustUnnamed_0 = 4;
pub const DFA_EOL_NUL: C2RustUnnamed_0 = 2;
pub const DFA_ANCHOR: C2RustUnnamed_0 = 1;
pub const CHARCLASS_WORD_BITS: C2RustUnnamed_9 = 64;
pub const CTX_NEWLINE: C2RustUnnamed_11 = 4;
pub const CTX_LETTER: C2RustUnnamed_11 = 2;
pub const CTX_NONE: C2RustUnnamed_11 = 1;
pub const _ISalnum: C2RustUnnamed_7 = 8;
pub const NOTCHAR: C2RustUnnamed_8 = 256;
pub const NO_CONSTRAINT: C2RustUnnamed_12 = 511;
pub const BACKREF: C2RustUnnamed_13 = 274;
pub const CTX_ANY: C2RustUnnamed_11 = 7;
pub const CHARCLASS_WORDS: C2RustUnnamed_10 = 4;
pub const ANYCHAR: C2RustUnnamed_13 = 266;
pub const CSET: C2RustUnnamed_13 = 276;
pub const MAX_TRCOUNT: C2RustUnnamed_14 = 1024;
pub const TRANSALLOC_SIZE: C2RustUnnamed_1 = 8192;
pub type C2RustUnnamed_1 = libc::c_uint;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const TRANSPTR_SIZE: C2RustUnnamed_2 = 8;
pub const OR: C2RustUnnamed_13 = 262;
pub const NOTLIMWORD: C2RustUnnamed_13 = 273;
pub const LIMWORD: C2RustUnnamed_13 = 272;
pub const ENDWORD: C2RustUnnamed_13 = 271;
pub const BEGWORD: C2RustUnnamed_13 = 270;
pub const ENDLINE: C2RustUnnamed_13 = 269;
pub const BEGLINE: C2RustUnnamed_13 = 268;
pub const EMPTY: C2RustUnnamed_13 = 256;
pub const CAT: C2RustUnnamed_13 = 261;
pub const PLUS: C2RustUnnamed_13 = 259;
pub const STAR: C2RustUnnamed_13 = 258;
pub const QMARK: C2RustUnnamed_13 = 257;
pub const MBCSET: C2RustUnnamed_13 = 275;
pub const END: C2RustUnnamed_13 = -1;
pub const _ISalpha: C2RustUnnamed_7 = 1024;
pub const WCHAR: C2RustUnnamed_13 = 265;
pub type predicate = unsafe extern "C" fn(libc::c_int) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dfa_ctype {
    pub name: *const libc::c_char,
    pub func: Option::<predicate>,
    pub single_byte_only: bool,
}
pub const MAX_BRACKET_STRING_LEN: C2RustUnnamed_3 = 32;
pub type C2RustUnnamed_3 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lexptr {
    pub ptr: *const libc::c_char,
    pub left: idx_t,
}
pub const _ISspace: C2RustUnnamed_7 = 8192;
pub const RPAREN: C2RustUnnamed_13 = 264;
pub const LPAREN: C2RustUnnamed_13 = 263;
pub const REPMN: C2RustUnnamed_13 = 260;
pub const C: C2RustUnnamed_5 = 2;
pub const K: C2RustUnnamed_5 = 7;
pub const M: C2RustUnnamed_5 = 8;
pub const L_token: C2RustUnnamed_4 = 244;
pub const J: C2RustUnnamed_5 = 6;
pub const I_token: C2RustUnnamed_4 = 240;
pub const F: C2RustUnnamed_5 = 4;
pub const H: C2RustUnnamed_5 = 5;
pub const G_token: C2RustUnnamed_4 = 237;
pub const E: C2RustUnnamed_5 = 3;
pub const D_token: C2RustUnnamed_4 = 224;
pub const B: C2RustUnnamed_5 = 1;
pub const A: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const BEG: C2RustUnnamed_13 = 267;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct must {
    pub in_0: *mut *mut libc::c_char,
    pub left: *mut libc::c_char,
    pub right: *mut libc::c_char,
    pub is: *mut libc::c_char,
    pub begline: bool,
    pub endline: bool,
    pub prev: *mut must,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub nullable: bool,
    pub nfirstpos: idx_t,
    pub nlastpos: idx_t,
}
pub const OPT_QUEUED: C2RustUnnamed_15 = 16;
pub const OPT_REPEAT: C2RustUnnamed_15 = 1;
pub const OPT_RPAREN: C2RustUnnamed_15 = 4;
pub const OPT_LPAREN: C2RustUnnamed_15 = 2;
pub const OPT_WALKED: C2RustUnnamed_15 = 8;
pub const NOTLIMWORD_CONSTRAINT: C2RustUnnamed_12 = 341;
pub const LIMWORD_CONSTRAINT: C2RustUnnamed_12 = 170;
pub const ENDWORD_CONSTRAINT: C2RustUnnamed_12 = 130;
pub const BEGWORD_CONSTRAINT: C2RustUnnamed_12 = 40;
pub const ENDLINE_CONSTRAINT: C2RustUnnamed_12 = 448;
pub const BEGLINE_CONSTRAINT: C2RustUnnamed_12 = 292;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const _ISpunct: C2RustUnnamed_7 = 4;
pub const _IScntrl: C2RustUnnamed_7 = 2;
pub const _ISblank: C2RustUnnamed_7 = 1;
pub const _ISgraph: C2RustUnnamed_7 = 32768;
pub const _ISprint: C2RustUnnamed_7 = 16384;
pub const _ISxdigit: C2RustUnnamed_7 = 4096;
pub const _ISdigit: C2RustUnnamed_7 = 2048;
pub const _ISlower: C2RustUnnamed_7 = 512;
pub const _ISupper: C2RustUnnamed_7 = 256;
pub type C2RustUnnamed_8 = libc::c_uint;
pub type C2RustUnnamed_9 = libc::c_uint;
pub type C2RustUnnamed_10 = libc::c_uint;
pub type C2RustUnnamed_11 = libc::c_uint;
pub type C2RustUnnamed_12 = libc::c_uint;
pub type C2RustUnnamed_13 = libc::c_int;
pub type C2RustUnnamed_14 = libc::c_uint;
pub type C2RustUnnamed_15 = libc::c_uint;
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn str_eq(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
) -> bool {
    return strcmp(a, b) == 0 as libc::c_int;
}
unsafe extern "C" fn c_isdigit(mut c: libc::c_char) -> bool {
    return '0' as i32 <= c as libc::c_int && c as libc::c_int <= '9' as i32;
}
#[inline]
unsafe extern "C" fn wctob_unused(mut __wc: wint_t) -> libc::c_int {
    return if 0 != 0 && __wc >= '\0' as i32 as libc::c_uint
        && __wc <= '\u{7f}' as i32 as libc::c_uint
    {
        __wc as libc::c_int
    } else {
panic!()
    };
}
static mut CHARCLASS_WORD_MASK: charclass_word = 0;
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
unsafe extern "C" fn newline_constraint(mut constraint: libc::c_int) -> libc::c_int {
    return constraint >> 6 as libc::c_int & 7 as libc::c_int;
}
unsafe extern "C" fn letter_constraint(mut constraint: libc::c_int) -> libc::c_int {
    return constraint >> 3 as libc::c_int & 7 as libc::c_int;
}
unsafe extern "C" fn other_constraint(mut constraint: libc::c_int) -> libc::c_int {
    return constraint & 7 as libc::c_int;
}
unsafe extern "C" fn succeeds_in_context(
    mut constraint: libc::c_int,
    mut prev: libc::c_int,
    mut curr: libc::c_int,
) -> bool {
    return ((if curr & CTX_NONE as libc::c_int != 0 {
        other_constraint(constraint)
    } else {
        0 as libc::c_int
    })
        | (if curr & CTX_LETTER as libc::c_int != 0 {
            letter_constraint(constraint)
        } else {
            0 as libc::c_int
        })
        | (if curr & CTX_NEWLINE as libc::c_int != 0 {
            newline_constraint(constraint)
        } else {
            0 as libc::c_int
        })) & prev != 0;
}
unsafe extern "C" fn prev_newline_dependent(mut constraint: libc::c_int) -> bool {
    return (constraint ^ constraint >> 2 as libc::c_int) & 0o111 as libc::c_int
        != 0 as libc::c_int;
}
unsafe extern "C" fn prev_letter_dependent(mut constraint: libc::c_int) -> bool {
    return (constraint ^ constraint >> 1 as libc::c_int) & 0o111 as libc::c_int
        != 0 as libc::c_int;
}
static mut TOKEN_MAX: token = 9223372036854775807 as libc::c_long;
unsafe extern "C" fn accepting(mut s: state_num, mut r: *const dfa) -> bool {
    return (*((*r).states).offset(s as isize)).constraint as libc::c_int
        != 0 as libc::c_int;
}
unsafe extern "C" fn accepts_in_context(
    mut prev: libc::c_int,
    mut curr: libc::c_int,
    mut state: state_num,
    mut dfa: *const dfa,
) -> bool {
    return succeeds_in_context(
        (*((*dfa).states).offset(state as isize)).constraint as libc::c_int,
        prev,
        curr,
    );
}
unsafe extern "C" fn mbs_to_wchar(
    mut pwc: *mut wint_t,
    mut s: *const libc::c_char,
    mut n: idx_t,
    mut d: *mut dfa,
) -> libc::c_int {
    let mut uc: libc::c_uchar = *s.offset(0 as libc::c_int as isize) as libc::c_uchar;
    let mut wc: wint_t = (*d).localeinfo.sbctowc[uc as usize];
    if wc == 0xffffffff as libc::c_uint {
        let mut wch: wchar_t = 0;
        let mut nbytes: size_t = rpl_mbrtowc(&mut wch, s, n as size_t, &mut (*d).mbs);
        if (0 as libc::c_int as libc::c_ulong) < nbytes
            && nbytes < -(2 as libc::c_int) as size_t
        {
            *pwc = wch as wint_t;
            return nbytes as libc::c_int;
        }
        memset(
            &mut (*d).mbs as *mut mbstate_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
    }
    *pwc = wc;
    return 1 as libc::c_int;
}
unsafe extern "C" fn tstbit(mut b: libc::c_uint, mut c: *const charclass) -> bool {
    return (*c)
        .w[b.wrapping_div(CHARCLASS_WORD_BITS as libc::c_int as libc::c_uint) as usize]
        >> b.wrapping_rem(CHARCLASS_WORD_BITS as libc::c_int as libc::c_uint)
        & 1 as libc::c_int as libc::c_ulong != 0;
}
unsafe extern "C" fn setbit(mut b: libc::c_uint, mut c: *mut charclass) {
    let mut one: charclass_word = 1 as libc::c_int as charclass_word;
    (*c).w[b.wrapping_div(CHARCLASS_WORD_BITS as libc::c_int as libc::c_uint) as usize]
        |= one << b.wrapping_rem(CHARCLASS_WORD_BITS as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn clrbit(mut b: libc::c_uint, mut c: *mut charclass) {
    let mut one: charclass_word = 1 as libc::c_int as charclass_word;
    (*c).w[b.wrapping_div(CHARCLASS_WORD_BITS as libc::c_int as libc::c_uint) as usize]
        &= !(one << b.wrapping_rem(CHARCLASS_WORD_BITS as libc::c_int as libc::c_uint));
}
unsafe extern "C" fn zeroset(mut s: *mut charclass) {
    memset(
        s as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<charclass>() as libc::c_ulong,
    );
}
unsafe extern "C" fn fillset(mut s: *mut charclass) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < CHARCLASS_WORDS as libc::c_int {
        (*s).w[i as usize] = CHARCLASS_WORD_MASK;
        i += 1;
        i;
    }
}
unsafe extern "C" fn notset(mut s: *mut charclass) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < CHARCLASS_WORDS as libc::c_int {
        (*s).w[i as usize] = CHARCLASS_WORD_MASK & !(*s).w[i as usize];
        i += 1;
        i;
    }
}
unsafe extern "C" fn equal(mut s1: *const charclass, mut s2: *const charclass) -> bool {
    let mut w: charclass_word = 0 as libc::c_int as charclass_word;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < CHARCLASS_WORDS as libc::c_int {
        w |= (*s1).w[i as usize] ^ (*s2).w[i as usize];
        i += 1;
        i;
    }
    return w == 0 as libc::c_int as libc::c_ulong;
}
unsafe extern "C" fn emptyset(mut s: *const charclass) -> bool {
    let mut w: charclass_word = 0 as libc::c_int as charclass_word;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < CHARCLASS_WORDS as libc::c_int {
        w |= (*s).w[i as usize];
        i += 1;
        i;
    }
    return w == 0 as libc::c_int as libc::c_ulong;
}
unsafe extern "C" fn maybe_realloc(
    mut pa: *mut libc::c_void,
    mut i: idx_t,
    mut nitems: *mut idx_t,
    mut nitems_max: ptrdiff_t,
    mut item_size: idx_t,
) -> *mut libc::c_void {
    if i < *nitems {
        return pa;
    }
    return xpalloc(pa, nitems, 1 as libc::c_int as idx_t, nitems_max, item_size);
}
unsafe extern "C" fn charclass_index(mut d: *mut dfa, mut s: *const charclass) -> idx_t {
    let mut i: idx_t = 0;
    i = 0 as libc::c_int as idx_t;
    while i < (*d).cindex {
        if equal(s, &mut *((*d).charclasses).offset(i as isize)) {
            return i;
        }
        i += 1;
        i;
    }
    (*d)
        .charclasses = maybe_realloc(
        (*d).charclasses as *mut libc::c_void,
        (*d).cindex,
        &mut (*d).calloc,
        TOKEN_MAX - CSET as libc::c_int as libc::c_long,
        ::std::mem::size_of::<charclass>() as libc::c_ulong as idx_t,
    ) as *mut charclass;
    (*d).cindex += 1;
    (*d).cindex;
    *((*d).charclasses).offset(i as isize) = *s;
    return i;
}
unsafe extern "C" fn unibyte_word_constituent(
    mut dfa: *const dfa,
    mut c: libc::c_uchar,
) -> bool {
    return (*dfa).localeinfo.sbctowc[c as usize] != 0xffffffff as libc::c_uint
        && (*(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
            || c as libc::c_int == '_' as i32);
}
unsafe extern "C" fn char_context(
    mut dfa: *const dfa,
    mut c: libc::c_uchar,
) -> libc::c_int {
    if c as libc::c_int == (*dfa).syntax.eolbyte as libc::c_int
        && (*dfa).syntax.dfaopts & DFA_ANCHOR as libc::c_int == 0
    {
        return CTX_NEWLINE as libc::c_int;
    }
    if unibyte_word_constituent(dfa, c) {
        return CTX_LETTER as libc::c_int;
    }
    return CTX_NONE as libc::c_int;
}
unsafe extern "C" fn setbit_wc(mut wc: wint_t, mut c: *mut charclass) -> bool {
    let mut b: libc::c_int = wctob(wc);
    if b < 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    setbit(b as libc::c_uint, c);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn setbit_case_fold_c(mut b: libc::c_int, mut c: *mut charclass) {
    let mut ub: libc::c_int = {
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = b;
                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_toupper_loc()).offset(__c as isize)
                };
            } else {
                __res = toupper(b);
            }
        } else {
            __res = *(*__ctype_toupper_loc()).offset(b as isize);
        }
        __res
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < NOTCHAR as libc::c_int {
        if ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = i;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    };
                } else {
                    __res = toupper(i);
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(i as isize);
            }
            __res
        }) == ub
        {
            setbit(i as libc::c_uint, c);
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn fetch_wc(mut dfa: *mut dfa) -> libc::c_int {
    let mut nbytes: libc::c_int = mbs_to_wchar(
        &mut (*dfa).lex.wctok,
        (*dfa).lex.ptr,
        (*dfa).lex.left,
        dfa,
    );
    let mut c: libc::c_int = if nbytes == 1 as libc::c_int {
        to_uchar(*((*dfa).lex.ptr).offset(0 as libc::c_int as isize)) as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    (*dfa).lex.ptr = ((*dfa).lex.ptr).offset(nbytes as isize);
    (*dfa).lex.left -= nbytes as libc::c_long;
    return c;
}
unsafe extern "C" fn bracket_fetch_wc(mut dfa: *mut dfa) -> libc::c_int {
    if (*dfa).lex.left == 0 {
        dfaerror(
            dcgettext(
                0 as *const libc::c_char,
                b"unbalanced [\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return fetch_wc(dfa);
}
static mut prednames: [dfa_ctype; 13] = unsafe {
    [
        {
            let mut init = dfa_ctype {
                name: b"alpha\0" as *const u8 as *const libc::c_char,
                func: Some(isalpha as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
                single_byte_only: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = dfa_ctype {
                name: b"upper\0" as *const u8 as *const libc::c_char,
                func: Some(isupper as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
                single_byte_only: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = dfa_ctype {
                name: b"lower\0" as *const u8 as *const libc::c_char,
                func: Some(islower as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
                single_byte_only: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = dfa_ctype {
                name: b"digit\0" as *const u8 as *const libc::c_char,
                func: Some(isdigit as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
                single_byte_only: 1 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = dfa_ctype {
                name: b"xdigit\0" as *const u8 as *const libc::c_char,
                func: Some(isxdigit as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
                single_byte_only: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = dfa_ctype {
                name: b"space\0" as *const u8 as *const libc::c_char,
                func: Some(isspace as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
                single_byte_only: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = dfa_ctype {
                name: b"punct\0" as *const u8 as *const libc::c_char,
                func: Some(ispunct as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
                single_byte_only: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = dfa_ctype {
                name: b"alnum\0" as *const u8 as *const libc::c_char,
                func: Some(isalnum as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
                single_byte_only: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = dfa_ctype {
                name: b"print\0" as *const u8 as *const libc::c_char,
                func: Some(isprint as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
                single_byte_only: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = dfa_ctype {
                name: b"graph\0" as *const u8 as *const libc::c_char,
                func: Some(isgraph as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
                single_byte_only: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = dfa_ctype {
                name: b"cntrl\0" as *const u8 as *const libc::c_char,
                func: Some(iscntrl as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
                single_byte_only: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = dfa_ctype {
                name: b"blank\0" as *const u8 as *const libc::c_char,
                func: Some(isblank as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
                single_byte_only: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = dfa_ctype {
                name: 0 as *const libc::c_char,
                func: None,
                single_byte_only: 0 as libc::c_int != 0,
            };
            init
        },
    ]
};
unsafe extern "C" fn find_pred(mut str: *const libc::c_char) -> *const dfa_ctype {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(prednames[i as usize].name).is_null() {
        if str_eq(str, prednames[i as usize].name) {
            return &*prednames.as_ptr().offset(i as isize) as *const dfa_ctype;
        }
        i += 1;
        i;
    }
    return 0 as *const dfa_ctype;
}
unsafe extern "C" fn parse_bracket_exp(mut dfa: *mut dfa) -> token {
    let mut known_bracket_exp: bool = 1 as libc::c_int != 0;
    let mut colon_warning_state: libc::c_int = 0;
    (*dfa).lex.brack.nchars = 0 as libc::c_int as idx_t;
    let mut ccl: charclass = charclass { w: [0; 4] };
    zeroset(&mut ccl);
    let mut c: libc::c_int = bracket_fetch_wc(dfa);
    let mut invert: bool = c == '^' as i32;
    if invert {
        c = bracket_fetch_wc(dfa);
        known_bracket_exp = (*dfa).localeinfo.simple;
    }
    let mut wc: wint_t = (*dfa).lex.wctok;
    let mut c1: libc::c_int = 0;
    let mut wc1: wint_t = 0;
    colon_warning_state = (c == ':' as i32) as libc::c_int;
    let mut current_block_73: u64;
    loop {
        c1 = NOTCHAR as libc::c_int;
        colon_warning_state &= !(2 as libc::c_int);
        if c == '[' as i32 {
            c1 = bracket_fetch_wc(dfa);
            wc1 = (*dfa).lex.wctok;
            if c1 == ':' as i32
                && (*dfa).syntax.syntax_bits
                    & ((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int != 0 || c1 == '.' as i32 || c1 == '=' as i32
            {
                let mut str: [libc::c_char; 33] = [0; 33];
                let mut len: libc::c_int = 0 as libc::c_int;
                loop {
                    c = bracket_fetch_wc(dfa);
                    if (*dfa).lex.left == 0 as libc::c_int as libc::c_long
                        || c == c1
                            && *((*dfa).lex.ptr).offset(0 as libc::c_int as isize)
                                as libc::c_int == ']' as i32
                    {
                        break;
                    }
                    if len < MAX_BRACKET_STRING_LEN as libc::c_int {
                        let fresh0 = len;
                        len = len + 1;
                        str[fresh0 as usize] = c as libc::c_char;
                    } else {
                        str[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                    }
                }
                str[len as usize] = '\0' as i32 as libc::c_char;
                c = bracket_fetch_wc(dfa);
                wc = (*dfa).lex.wctok;
                if c1 == ':' as i32 {
                    let mut class: *const libc::c_char = if (*dfa).syntax.case_fold
                        as libc::c_int != 0
                        && (str_eq(
                            str.as_mut_ptr(),
                            b"upper\0" as *const u8 as *const libc::c_char,
                        ) as libc::c_int != 0
                            || str_eq(
                                str.as_mut_ptr(),
                                b"lower\0" as *const u8 as *const libc::c_char,
                            ) as libc::c_int != 0)
                    {
                        b"alpha\0" as *const u8 as *const libc::c_char
                    } else {
                        str.as_mut_ptr() as *const libc::c_char
                    };
                    let mut pred: *const dfa_ctype = find_pred(class);
                    if pred.is_null() {
                        dfaerror(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid character class\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    if (*dfa).localeinfo.multibyte as libc::c_int != 0
                        && !(*pred).single_byte_only
                    {
                        known_bracket_exp = 0 as libc::c_int != 0;
                    } else {
                        let mut c2: libc::c_int = 0 as libc::c_int;
                        while c2 < NOTCHAR as libc::c_int {
                            if ((*pred).func).unwrap()(c2) != 0 {
                                setbit(c2 as libc::c_uint, &mut ccl);
                            }
                            c2 += 1;
                            c2;
                        }
                    }
                } else {
                    known_bracket_exp = 0 as libc::c_int != 0;
                }
                colon_warning_state |= 8 as libc::c_int;
                c1 = bracket_fetch_wc(dfa);
                wc1 = (*dfa).lex.wctok;
                current_block_73 = 14523784380283086299;
            } else {
                current_block_73 = 2873832966593178012;
            }
        } else {
            current_block_73 = 2873832966593178012;
        }
        match current_block_73 {
            2873832966593178012 => {
                if c == '\\' as i32
                    && (*dfa).syntax.syntax_bits & 1 as libc::c_int as libc::c_ulong != 0
                {
                    c = bracket_fetch_wc(dfa);
                    wc = (*dfa).lex.wctok;
                }
                if c1 == NOTCHAR as libc::c_int {
                    c1 = bracket_fetch_wc(dfa);
                    wc1 = (*dfa).lex.wctok;
                }
                if c1 == '-' as i32 {
                    let mut c2_0: libc::c_int = bracket_fetch_wc(dfa);
                    let mut wc2: wint_t = (*dfa).lex.wctok;
                    if c2_0 == '[' as i32
                        && *((*dfa).lex.ptr).offset(0 as libc::c_int as isize)
                            as libc::c_int == '.' as i32
                    {
                        known_bracket_exp = 0 as libc::c_int != 0;
                        c2_0 = ']' as i32;
                    }
                    if c2_0 == ']' as i32 {
                        (*dfa).lex.ptr = ((*dfa).lex.ptr).offset(-1);
                        (*dfa).lex.ptr;
                        (*dfa).lex.left += 1;
                        (*dfa).lex.left;
                        current_block_73 = 5807581744382915773;
                    } else {
                        if c2_0 == '\\' as i32
                            && (*dfa).syntax.syntax_bits
                                & 1 as libc::c_int as libc::c_ulong != 0
                        {
                            c2_0 = bracket_fetch_wc(dfa);
                            wc2 = (*dfa).lex.wctok;
                        }
                        colon_warning_state |= 8 as libc::c_int;
                        c1 = bracket_fetch_wc(dfa);
                        wc1 = (*dfa).lex.wctok;
                        if wc != wc2 || wc == 0xffffffff as libc::c_uint {
                            if (*dfa).localeinfo.simple as libc::c_int != 0
                                || c_isdigit(c as libc::c_char) as libc::c_int
                                    & c_isdigit(c2_0 as libc::c_char) as libc::c_int != 0
                            {
                                let mut ci: libc::c_int = c;
                                while ci <= c2_0 {
                                    if (*dfa).syntax.case_fold as libc::c_int != 0
                                        && *(*__ctype_b_loc()).offset(ci as isize) as libc::c_int
                                            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                                            != 0
                                    {
                                        setbit_case_fold_c(ci, &mut ccl);
                                    } else {
                                        setbit(ci as libc::c_uint, &mut ccl);
                                    }
                                    ci += 1;
                                    ci;
                                }
                            } else {
                                known_bracket_exp = 0 as libc::c_int != 0;
                            }
                            current_block_73 = 14523784380283086299;
                        } else {
                            current_block_73 = 5807581744382915773;
                        }
                    }
                } else {
                    current_block_73 = 5807581744382915773;
                }
                match current_block_73 {
                    14523784380283086299 => {}
                    _ => {
                        colon_warning_state
                            |= if c == ':' as i32 {
                                2 as libc::c_int
                            } else {
                                4 as libc::c_int
                            };
                        if !(*dfa).localeinfo.multibyte {
                            if (*dfa).syntax.case_fold as libc::c_int != 0
                                && *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                            {
                                setbit_case_fold_c(c, &mut ccl);
                            } else {
                                setbit(c as libc::c_uint, &mut ccl);
                            }
                        } else if wc == 0xffffffff as libc::c_uint {
                            known_bracket_exp = 0 as libc::c_int != 0;
                        } else {
                            let mut folded: [wchar_t; 33] = [0; 33];
                            let mut n: libc::c_int = if (*dfa).syntax.case_fold
                                as libc::c_int != 0
                            {
                                case_folded_counterparts(
                                    wc,
                                    folded.as_mut_ptr().offset(1 as libc::c_int as isize),
                                ) + 1 as libc::c_int
                            } else {
                                1 as libc::c_int
                            };
                            folded[0 as libc::c_int as usize] = wc as wchar_t;
                            let mut i: libc::c_int = 0 as libc::c_int;
                            while i < n {
                                if !setbit_wc(folded[i as usize] as wint_t, &mut ccl) {
                                    (*dfa)
                                        .lex
                                        .brack
                                        .chars = maybe_realloc(
                                        (*dfa).lex.brack.chars as *mut libc::c_void,
                                        (*dfa).lex.brack.nchars,
                                        &mut (*dfa).lex.brack.nchars_alloc,
                                        -(1 as libc::c_int) as ptrdiff_t,
                                        ::std::mem::size_of::<wchar_t>() as libc::c_ulong as idx_t,
                                    ) as *mut wchar_t;
                                    let fresh1 = (*dfa).lex.brack.nchars;
                                    (*dfa).lex.brack.nchars = (*dfa).lex.brack.nchars + 1;
                                    *((*dfa).lex.brack.chars)
                                        .offset(fresh1 as isize) = folded[i as usize];
                                }
                                i += 1;
                                i;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        wc = wc1;
        c = c1;
        if !(c != ']' as i32) {
            break;
        }
    }
    if colon_warning_state == 7 as libc::c_int {
        let mut msg: *const libc::c_char = dcgettext(
            0 as *const libc::c_char,
            b"character class syntax is [[:space:]], not [:space:]\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        );
        if (*dfa).syntax.dfaopts & DFA_CONFUSING_BRACKETS_ERROR as libc::c_int != 0 {
            dfaerror(msg);
        } else {
            dfawarn(msg);
        }
    }
    if !known_bracket_exp {
        return BACKREF as libc::c_int as token;
    }
    if (*dfa).localeinfo.multibyte as libc::c_int != 0
        && (invert as libc::c_int != 0
            || (*dfa).lex.brack.nchars != 0 as libc::c_int as libc::c_long)
    {
        (*dfa).lex.brack.invert = invert;
        (*dfa)
            .lex
            .brack
            .cset = if emptyset(&mut ccl) as libc::c_int != 0 {
            -(1 as libc::c_int) as libc::c_long
        } else {
            charclass_index(dfa, &mut ccl)
        };
        return MBCSET as libc::c_int as token;
    }
    if invert {
        notset(&mut ccl);
        if (*dfa).syntax.syntax_bits
            & ((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int != 0
        {
            clrbit('\n' as i32 as libc::c_uint, &mut ccl);
        }
    }
    return CSET as libc::c_int as libc::c_long + charclass_index(dfa, &mut ccl);
}
unsafe extern "C" fn push_lex_state(
    mut dfa: *mut dfa,
    mut ls: *mut lexptr,
    mut s: *const libc::c_char,
) {
    (*ls).ptr = (*dfa).lex.ptr;
    (*ls).left = (*dfa).lex.left;
    (*dfa).lex.ptr = s;
    (*dfa).lex.left = strlen(s) as idx_t;
}
unsafe extern "C" fn pop_lex_state(mut dfa: *mut dfa, mut ls: *const lexptr) {
    (*dfa).lex.ptr = (*ls).ptr;
    (*dfa).lex.left = (*ls).left;
}
unsafe extern "C" fn lex(mut dfa: *mut dfa) -> token {
    let mut current_block: u64;
    let mut backslash: bool = 0 as libc::c_int != 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    loop {
        if 2 as libc::c_int <= i {
            abort();
        }
        if (*dfa).lex.left == 0 {
            (*dfa).lex.lasttok = END as libc::c_int as token;
            return (*dfa).lex.lasttok;
        }
        let mut c: libc::c_int = fetch_wc(dfa);
        match c {
            92 => {
                if backslash {
                    current_block = 9551997267762527044;
                } else {
                    if (*dfa).lex.left == 0 as libc::c_int as libc::c_long {
                        dfaerror(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"unfinished \\ escape\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    backslash = 1 as libc::c_int != 0;
                    i += 1;
                    i;
                    continue;
                }
            }
            94 => {
                if backslash {
                    current_block = 9551997267762527044;
                } else {
                    if (*dfa).syntax.syntax_bits
                        & (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int != 0
                        || (*dfa).lex.lasttok == END as libc::c_int as libc::c_long
                        || (*dfa).lex.lasttok == LPAREN as libc::c_int as libc::c_long
                        || (*dfa).lex.lasttok == OR as libc::c_int as libc::c_long
                    {
                        (*dfa).lex.lasttok = BEGLINE as libc::c_int as token;
                        return (*dfa).lex.lasttok;
                    }
                    current_block = 9551997267762527044;
                }
            }
            36 => {
                if backslash {
                    current_block = 9551997267762527044;
                } else {
                    if (*dfa).syntax.syntax_bits
                        & (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int != 0
                        || (*dfa).lex.left == 0 as libc::c_int as libc::c_long
                        || (*dfa).lex.left
                            > ((*dfa).syntax.syntax_bits
                                & (((((((((((((1 as libc::c_int as libc::c_ulong)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int == 0) as libc::c_int as libc::c_long
                            && *((*dfa).lex.ptr)
                                .offset(
                                    (((*dfa).syntax.syntax_bits
                                        & (((((((((((((1 as libc::c_int as libc::c_ulong)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int == 0) as libc::c_int
                                        & (*((*dfa).lex.ptr).offset(0 as libc::c_int as isize)
                                            as libc::c_int == '\\' as i32) as libc::c_int) as isize,
                                ) as libc::c_int == ')' as i32
                        || (*dfa).lex.left
                            > ((*dfa).syntax.syntax_bits
                                & (((((((((((((((1 as libc::c_int as libc::c_ulong)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int == 0) as libc::c_int as libc::c_long
                            && *((*dfa).lex.ptr)
                                .offset(
                                    (((*dfa).syntax.syntax_bits
                                        & (((((((((((((((1 as libc::c_int as libc::c_ulong)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int) << 1 as libc::c_int)
                                            << 1 as libc::c_int == 0) as libc::c_int
                                        & (*((*dfa).lex.ptr).offset(0 as libc::c_int as isize)
                                            as libc::c_int == '\\' as i32) as libc::c_int) as isize,
                                ) as libc::c_int == '|' as i32
                        || (*dfa).syntax.syntax_bits
                            & (((((((((((1 as libc::c_int as libc::c_ulong)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int != 0
                            && (*dfa).lex.left > 0 as libc::c_int as libc::c_long
                            && *((*dfa).lex.ptr).offset(0 as libc::c_int as isize)
                                as libc::c_int == '\n' as i32
                    {
                        (*dfa).lex.lasttok = ENDLINE as libc::c_int as token;
                        return (*dfa).lex.lasttok;
                    }
                    current_block = 9551997267762527044;
                }
            }
            49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                if !backslash {
                    current_block = 9551997267762527044;
                } else if (*dfa).syntax.syntax_bits
                    & ((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int != 0
                {
                    current_block = 14059846588259325495;
                } else {
                    (*dfa).lex.laststart = 0 as libc::c_int != 0;
                    (*dfa).lex.lasttok = BACKREF as libc::c_int as token;
                    return (*dfa).lex.lasttok;
                }
            }
            96 => {
                if !backslash {
                    current_block = 9551997267762527044;
                } else if (*dfa).syntax.syntax_bits
                    & (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int != 0
                {
                    current_block = 14059846588259325495;
                } else {
                    (*dfa).lex.lasttok = BEGLINE as libc::c_int as token;
                    return (*dfa).lex.lasttok;
                }
            }
            39 => {
                if !backslash {
                    current_block = 9551997267762527044;
                } else if (*dfa).syntax.syntax_bits
                    & (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int != 0
                {
                    current_block = 14059846588259325495;
                } else {
                    (*dfa).lex.lasttok = ENDLINE as libc::c_int as token;
                    return (*dfa).lex.lasttok;
                }
            }
            60 => {
                if !backslash {
                    current_block = 9551997267762527044;
                } else if (*dfa).syntax.syntax_bits
                    & (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int != 0
                {
                    current_block = 14059846588259325495;
                } else {
                    (*dfa).lex.lasttok = BEGWORD as libc::c_int as token;
                    return (*dfa).lex.lasttok;
                }
            }
            62 => {
                if !backslash {
                    current_block = 9551997267762527044;
                } else if (*dfa).syntax.syntax_bits
                    & (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int != 0
                {
                    current_block = 14059846588259325495;
                } else {
                    (*dfa).lex.lasttok = ENDWORD as libc::c_int as token;
                    return (*dfa).lex.lasttok;
                }
            }
            98 => {
                if !backslash {
                    current_block = 9551997267762527044;
                } else if (*dfa).syntax.syntax_bits
                    & (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int != 0
                {
                    current_block = 14059846588259325495;
                } else {
                    (*dfa).lex.lasttok = LIMWORD as libc::c_int as token;
                    return (*dfa).lex.lasttok;
                }
            }
            66 => {
                if !backslash {
                    current_block = 9551997267762527044;
                } else if (*dfa).syntax.syntax_bits
                    & (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int != 0
                {
                    current_block = 14059846588259325495;
                } else {
                    (*dfa).lex.lasttok = NOTLIMWORD as libc::c_int as token;
                    return (*dfa).lex.lasttok;
                }
            }
            63 => {
                if (*dfa).syntax.syntax_bits
                    & ((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    != 0
                {
                    current_block = 11160950852560677892;
                } else if backslash as libc::c_int
                    != ((*dfa).syntax.syntax_bits
                        & (1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int
                        != 0 as libc::c_int as libc::c_ulong) as libc::c_int
                {
                    current_block = 9551997267762527044;
                } else {
                    if (*dfa).lex.laststart {
                        if (*dfa).syntax.syntax_bits
                            & ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int == 0
                        {
                            current_block = 11160950852560677892;
                        } else {
                            if (*dfa).syntax.dfaopts & DFA_PLUS_WARN as libc::c_int != 0
                            {
                                dfawarn(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"? at start of expression\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            current_block = 10399321362245223758;
                        }
                    } else {
                        current_block = 10399321362245223758;
                    }
                    match current_block {
                        11160950852560677892 => {}
                        _ => {
                            (*dfa).lex.lasttok = QMARK as libc::c_int as token;
                            return (*dfa).lex.lasttok;
                        }
                    }
                }
            }
            42 => {
                if backslash {
                    current_block = 9551997267762527044;
                } else {
                    if (*dfa).lex.laststart {
                        if (*dfa).syntax.syntax_bits
                            & ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int == 0
                        {
                            current_block = 11160950852560677892;
                        } else {
                            if (*dfa).syntax.dfaopts & DFA_STAR_WARN as libc::c_int != 0
                            {
                                dfawarn(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"* at start of expression\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            current_block = 17747245473264231573;
                        }
                    } else {
                        current_block = 17747245473264231573;
                    }
                    match current_block {
                        11160950852560677892 => {}
                        _ => {
                            (*dfa).lex.lasttok = STAR as libc::c_int as token;
                            return (*dfa).lex.lasttok;
                        }
                    }
                }
            }
            43 => {
                if (*dfa).syntax.syntax_bits
                    & ((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    != 0
                {
                    current_block = 11160950852560677892;
                } else if backslash as libc::c_int
                    != ((*dfa).syntax.syntax_bits
                        & (1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int
                        != 0 as libc::c_int as libc::c_ulong) as libc::c_int
                {
                    current_block = 9551997267762527044;
                } else {
                    if (*dfa).lex.laststart {
                        if (*dfa).syntax.syntax_bits
                            & ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int == 0
                        {
                            current_block = 11160950852560677892;
                        } else {
                            if (*dfa).syntax.dfaopts & DFA_PLUS_WARN as libc::c_int != 0
                            {
                                dfawarn(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"+ at start of expression\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                            current_block = 13303144130133872306;
                        }
                    } else {
                        current_block = 13303144130133872306;
                    }
                    match current_block {
                        11160950852560677892 => {}
                        _ => {
                            (*dfa).lex.lasttok = PLUS as libc::c_int as token;
                            return (*dfa).lex.lasttok;
                        }
                    }
                }
            }
            123 => {
                if (*dfa).syntax.syntax_bits
                    & (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int == 0
                {
                    current_block = 11160950852560677892;
                } else if backslash as libc::c_int
                    != ((*dfa).syntax.syntax_bits
                        & ((((((((((((1 as libc::c_int as libc::c_ulong)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int
                        == 0 as libc::c_int as libc::c_ulong) as libc::c_int
                {
                    current_block = 9551997267762527044;
                } else {
                    let mut p: *const libc::c_char = (*dfa).lex.ptr;
                    let mut lim: *const libc::c_char = p
                        .offset((*dfa).lex.left as isize);
                    (*dfa).lex.maxrep = -(1 as libc::c_int);
                    (*dfa).lex.minrep = (*dfa).lex.maxrep;
                    while p != lim && c_isdigit(*p) as libc::c_int != 0 {
                        (*dfa)
                            .lex
                            .minrep = if (*dfa).lex.minrep < 0 as libc::c_int {
                            *p as libc::c_int - '0' as i32
                        } else if (0x7fff as libc::c_int + 1 as libc::c_int)
                            < (*dfa).lex.minrep * 10 as libc::c_int + *p as libc::c_int
                                - '0' as i32
                        {
                            0x7fff as libc::c_int + 1 as libc::c_int
                        } else {
                            (*dfa).lex.minrep * 10 as libc::c_int + *p as libc::c_int
                                - '0' as i32
                        };
                        p = p.offset(1);
                        p;
                    }
                    if p != lim {
                        if *p as libc::c_int != ',' as i32 {
                            (*dfa).lex.maxrep = (*dfa).lex.minrep;
                        } else {
                            if (*dfa).lex.minrep < 0 as libc::c_int {
                                (*dfa).lex.minrep = 0 as libc::c_int;
                            }
                            loop {
                                p = p.offset(1);
                                if !(p != lim && c_isdigit(*p) as libc::c_int != 0) {
                                    break;
                                }
                                (*dfa)
                                    .lex
                                    .maxrep = if (*dfa).lex.maxrep < 0 as libc::c_int {
                                    *p as libc::c_int - '0' as i32
                                } else if (0x7fff as libc::c_int + 1 as libc::c_int)
                                    < (*dfa).lex.maxrep * 10 as libc::c_int + *p as libc::c_int
                                        - '0' as i32
                                {
                                    0x7fff as libc::c_int + 1 as libc::c_int
                                } else {
                                    (*dfa).lex.maxrep * 10 as libc::c_int + *p as libc::c_int
                                        - '0' as i32
                                };
                            }
                        }
                    }
                    let mut invalid_content: bool = !((!backslash
                        || p != lim
                            && {
                                let fresh4 = p;
                                p = p.offset(1);
                                *fresh4 as libc::c_int == '\\' as i32
                            }) && p != lim
                        && {
                            let fresh5 = p;
                            p = p.offset(1);
                            *fresh5 as libc::c_int == '}' as i32
                        } && 0 as libc::c_int <= (*dfa).lex.minrep
                        && ((*dfa).lex.maxrep < 0 as libc::c_int
                            || (*dfa).lex.minrep <= (*dfa).lex.maxrep));
                    if invalid_content as libc::c_int != 0
                        && (*dfa).syntax.syntax_bits
                            & (((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int != 0
                    {
                        current_block = 9551997267762527044;
                    } else {
                        if (*dfa).lex.laststart {
                            if (*dfa).syntax.syntax_bits
                                & ((((1 as libc::c_int as libc::c_ulong)
                                    << 1 as libc::c_int) << 1 as libc::c_int)
                                    << 1 as libc::c_int) << 1 as libc::c_int == 0
                            {
                                current_block = 11160950852560677892;
                            } else {
                                if (*dfa).syntax.dfaopts & DFA_PLUS_WARN as libc::c_int != 0
                                {
                                    dfawarn(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"{...} at start of expression\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                }
                                current_block = 17485376261910781866;
                            }
                        } else {
                            current_block = 17485376261910781866;
                        }
                        match current_block {
                            11160950852560677892 => {}
                            _ => {
                                if invalid_content {
                                    dfaerror(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"invalid content of \\{\\}\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                }
                                if (0x7fff as libc::c_int) < (*dfa).lex.maxrep {
                                    dfaerror(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"regular expression too big\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                }
                                (*dfa).lex.ptr = p;
                                (*dfa).lex.left = lim.offset_from(p) as libc::c_long;
                                (*dfa).lex.laststart = 0 as libc::c_int != 0;
                                (*dfa).lex.lasttok = REPMN as libc::c_int as token;
                                return (*dfa).lex.lasttok;
                            }
                        }
                    }
                }
            }
            124 => {
                if (*dfa).syntax.syntax_bits
                    & ((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    != 0
                {
                    current_block = 11160950852560677892;
                } else if backslash as libc::c_int
                    != ((*dfa).syntax.syntax_bits
                        & (((((((((((((((1 as libc::c_int as libc::c_ulong)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                        == 0 as libc::c_int as libc::c_ulong) as libc::c_int
                {
                    current_block = 9551997267762527044;
                } else {
                    (*dfa).lex.laststart = 1 as libc::c_int != 0;
                    (*dfa).lex.lasttok = OR as libc::c_int as token;
                    return (*dfa).lex.lasttok;
                }
            }
            10 => {
                if (*dfa).syntax.syntax_bits
                    & (((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int == 0
                {
                    current_block = 11160950852560677892;
                } else if backslash {
                    current_block = 9551997267762527044;
                } else {
                    (*dfa).lex.laststart = 1 as libc::c_int != 0;
                    (*dfa).lex.lasttok = OR as libc::c_int as token;
                    return (*dfa).lex.lasttok;
                }
            }
            40 => {
                if backslash as libc::c_int
                    != ((*dfa).syntax.syntax_bits
                        & (((((((((((((1 as libc::c_int as libc::c_ulong)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                        == 0 as libc::c_int as libc::c_ulong) as libc::c_int
                {
                    current_block = 9551997267762527044;
                } else {
                    (*dfa).lex.parens += 1;
                    (*dfa).lex.parens;
                    (*dfa).lex.laststart = 1 as libc::c_int != 0;
                    (*dfa).lex.lasttok = LPAREN as libc::c_int as token;
                    return (*dfa).lex.lasttok;
                }
            }
            41 => {
                if backslash as libc::c_int
                    != ((*dfa).syntax.syntax_bits
                        & (((((((((((((1 as libc::c_int as libc::c_ulong)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                        == 0 as libc::c_int as libc::c_ulong) as libc::c_int
                {
                    current_block = 9551997267762527044;
                } else if (*dfa).lex.parens == 0 as libc::c_int as libc::c_long
                    && (*dfa).syntax.syntax_bits
                        & (((((((((((((((((1 as libc::c_int as libc::c_ulong)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                        != 0
                {
                    current_block = 9551997267762527044;
                } else {
                    (*dfa).lex.parens -= 1;
                    (*dfa).lex.parens;
                    (*dfa).lex.laststart = 0 as libc::c_int != 0;
                    (*dfa).lex.lasttok = RPAREN as libc::c_int as token;
                    return (*dfa).lex.lasttok;
                }
            }
            46 => {
                if backslash {
                    current_block = 9551997267762527044;
                } else {
                    if (*dfa).canychar < 0 as libc::c_int as libc::c_long {
                        let mut ccl: charclass = charclass { w: [0; 4] };
                        fillset(&mut ccl);
                        if (*dfa).syntax.syntax_bits
                            & ((((((1 as libc::c_int as libc::c_ulong)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int == 0
                        {
                            clrbit('\n' as i32 as libc::c_uint, &mut ccl);
                        }
                        if (*dfa).syntax.syntax_bits
                            & (((((((1 as libc::c_int as libc::c_ulong)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int) << 1 as libc::c_int)
                                << 1 as libc::c_int != 0
                        {
                            clrbit('\0' as i32 as libc::c_uint, &mut ccl);
                        }
                        if (*dfa).localeinfo.multibyte {
                            let mut c2: libc::c_int = 0 as libc::c_int;
                            while c2 < NOTCHAR as libc::c_int {
                                if (*dfa).localeinfo.sbctowc[c2 as usize]
                                    == 0xffffffff as libc::c_uint
                                {
                                    clrbit(c2 as libc::c_uint, &mut ccl);
                                }
                                c2 += 1;
                                c2;
                            }
                        }
                        (*dfa).canychar = charclass_index(dfa, &mut ccl);
                    }
                    (*dfa).lex.laststart = 0 as libc::c_int != 0;
                    (*dfa)
                        .lex
                        .lasttok = if (*dfa).localeinfo.multibyte as libc::c_int != 0 {
                        ANYCHAR as libc::c_int as libc::c_long
                    } else {
                        CSET as libc::c_int as libc::c_long + (*dfa).canychar
                    };
                    return (*dfa).lex.lasttok;
                }
            }
            115 | 83 => {
                if !backslash {
                    current_block = 9551997267762527044;
                } else if (*dfa).syntax.syntax_bits
                    & (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int != 0
                {
                    current_block = 14059846588259325495;
                } else {
                    if !(*dfa).localeinfo.multibyte {
                        let mut ccl_0: charclass = charclass { w: [0; 4] };
                        zeroset(&mut ccl_0);
                        let mut c2_0: libc::c_int = 0 as libc::c_int;
                        while c2_0 < NOTCHAR as libc::c_int {
                            if *(*__ctype_b_loc()).offset(c2_0 as isize) as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                            {
                                setbit(c2_0 as libc::c_uint, &mut ccl_0);
                            }
                            c2_0 += 1;
                            c2_0;
                        }
                        if c == 'S' as i32 {
                            notset(&mut ccl_0);
                        }
                        (*dfa).lex.laststart = 0 as libc::c_int != 0;
                        (*dfa)
                            .lex
                            .lasttok = CSET as libc::c_int as libc::c_long
                            + charclass_index(dfa, &mut ccl_0);
                        return (*dfa).lex.lasttok;
                    }
                    let mut ls: lexptr = lexptr {
                        ptr: 0 as *const libc::c_char,
                        left: 0,
                    };
                    push_lex_state(
                        dfa,
                        &mut ls,
                        &*(b"^[:space:]]\0" as *const u8 as *const libc::c_char)
                            .offset((c == 's' as i32) as libc::c_int as isize),
                    );
                    (*dfa).lex.lasttok = parse_bracket_exp(dfa);
                    pop_lex_state(dfa, &mut ls);
                    (*dfa).lex.laststart = 0 as libc::c_int != 0;
                    return (*dfa).lex.lasttok;
                }
            }
            119 | 87 => {
                if !backslash {
                    current_block = 9551997267762527044;
                } else if (*dfa).syntax.syntax_bits
                    & (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int != 0
                {
                    current_block = 14059846588259325495;
                } else {
                    if !(*dfa).localeinfo.multibyte {
                        let mut ccl_1: charclass = charclass { w: [0; 4] };
                        zeroset(&mut ccl_1);
                        let mut c2_1: libc::c_int = 0 as libc::c_int;
                        while c2_1 < NOTCHAR as libc::c_int {
                            if (*dfa).syntax.sbit[c2_1 as usize] as libc::c_int
                                == CTX_LETTER as libc::c_int
                            {
                                setbit(c2_1 as libc::c_uint, &mut ccl_1);
                            }
                            c2_1 += 1;
                            c2_1;
                        }
                        if c == 'W' as i32 {
                            notset(&mut ccl_1);
                        }
                        (*dfa).lex.laststart = 0 as libc::c_int != 0;
                        (*dfa)
                            .lex
                            .lasttok = CSET as libc::c_int as libc::c_long
                            + charclass_index(dfa, &mut ccl_1);
                        return (*dfa).lex.lasttok;
                    }
                    let mut ls_0: lexptr = lexptr {
                        ptr: 0 as *const libc::c_char,
                        left: 0,
                    };
                    push_lex_state(
                        dfa,
                        &mut ls_0,
                        &*(b"^_[:alnum:]]\0" as *const u8 as *const libc::c_char)
                            .offset((c == 'w' as i32) as libc::c_int as isize),
                    );
                    (*dfa).lex.lasttok = parse_bracket_exp(dfa);
                    pop_lex_state(dfa, &mut ls_0);
                    (*dfa).lex.laststart = 0 as libc::c_int != 0;
                    return (*dfa).lex.lasttok;
                }
            }
            91 => {
                if backslash {
                    current_block = 9551997267762527044;
                } else {
                    (*dfa).lex.laststart = 0 as libc::c_int != 0;
                    (*dfa).lex.lasttok = parse_bracket_exp(dfa);
                    return (*dfa).lex.lasttok;
                }
            }
            93 | 125 => {
                current_block = 9551997267762527044;
            }
            _ => {
                current_block = 11160950852560677892;
            }
        }
        match current_block {
            11160950852560677892 => {
                if !backslash {
                    current_block = 9551997267762527044;
                } else {
                    current_block = 14059846588259325495;
                }
            }
            _ => {}
        }
        match current_block {
            14059846588259325495 => {
                if (*dfa).syntax.dfaopts & DFA_STRAY_BACKSLASH_WARN as libc::c_int != 0 {
                    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
                    let mut msgbuf: [libc::c_char; 100] = [0; 100];
                    if iswprint((*dfa).lex.wctok) == 0 {
                        msg = dcgettext(
                            0 as *const libc::c_char,
                            b"stray \\ before unprintable character\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        );
                    } else if iswspace((*dfa).lex.wctok) != 0 {
                        msg = dcgettext(
                            0 as *const libc::c_char,
                            b"stray \\ before white space\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        );
                    } else {
                        let mut n: libc::c_int = snprintf(
                            msgbuf.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 100]>()
                                as libc::c_ulong,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"stray \\ before %lc\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*dfa).lex.wctok,
                        );
                        msg = if 0 as libc::c_int <= n
                            && (n as libc::c_ulong)
                                < ::std::mem::size_of::<[libc::c_char; 100]>()
                                    as libc::c_ulong
                        {
                            msgbuf.as_mut_ptr()
                        } else {
                            dcgettext(
                                0 as *const libc::c_char,
                                b"stray \\\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            )
                        };
                    }
                    dfawarn(msg);
                }
            }
            _ => {}
        }
        (*dfa).lex.laststart = 0 as libc::c_int != 0;
        if (*dfa).localeinfo.multibyte {
            (*dfa).lex.lasttok = WCHAR as libc::c_int as token;
            return (*dfa).lex.lasttok;
        }
        if (*dfa).syntax.case_fold as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            let mut ccl_2: charclass = charclass { w: [0; 4] };
            zeroset(&mut ccl_2);
            setbit_case_fold_c(c, &mut ccl_2);
            (*dfa)
                .lex
                .lasttok = CSET as libc::c_int as libc::c_long
                + charclass_index(dfa, &mut ccl_2);
            return (*dfa).lex.lasttok;
        }
        (*dfa).lex.lasttok = c as token;
        return (*dfa).lex.lasttok;
    };
}
unsafe extern "C" fn addtok_mb(
    mut dfa: *mut dfa,
    mut t: token,
    mut mbprop: libc::c_char,
) {
    if (*dfa).talloc == (*dfa).tindex {
        (*dfa)
            .tokens = xpalloc(
            (*dfa).tokens as *mut libc::c_void,
            &mut (*dfa).talloc,
            1 as libc::c_int as idx_t,
            -(1 as libc::c_int) as ptrdiff_t,
            ::std::mem::size_of::<token>() as libc::c_ulong as idx_t,
        ) as *mut token;
        if (*dfa).localeinfo.multibyte {
            (*dfa)
                .multibyte_prop = xreallocarray(
                (*dfa).multibyte_prop as *mut libc::c_void,
                (*dfa).talloc as size_t,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            ) as *mut libc::c_char;
        }
    }
    if (*dfa).localeinfo.multibyte {
        *((*dfa).multibyte_prop).offset((*dfa).tindex as isize) = mbprop;
    }
    let fresh6 = (*dfa).tindex;
    (*dfa).tindex = (*dfa).tindex + 1;
    *((*dfa).tokens).offset(fresh6 as isize) = t;
    let mut current_block_16: u64;
    match t {
        257 | 258 | 259 => {
            current_block_16 = 26972500619410423;
        }
        261 | 262 => {
            (*dfa).parse.depth -= 1;
            (*dfa).parse.depth;
            current_block_16 = 26972500619410423;
        }
        256 => {
            (*dfa).epsilon = 1 as libc::c_int != 0;
            current_block_16 = 11365751679955571920;
        }
        274 => {
            (*dfa).fast = 0 as libc::c_int != 0;
            current_block_16 = 14583082600678932368;
        }
        268 | 269 | 270 | 271 | 272 | 273 => {
            (*dfa).epsilon = 1 as libc::c_int != 0;
            current_block_16 = 14583082600678932368;
        }
        _ => {
            current_block_16 = 14583082600678932368;
        }
    }
    match current_block_16 {
        14583082600678932368 => {
            (*dfa).nleaves += 1;
            (*dfa).nleaves;
            current_block_16 = 11365751679955571920;
        }
        _ => {}
    }
    match current_block_16 {
        11365751679955571920 => {
            (*dfa).parse.depth += 1;
            (*dfa).parse.depth;
            if (*dfa).depth < (*dfa).parse.depth {
                (*dfa).depth = (*dfa).parse.depth;
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn addtok(mut dfa: *mut dfa, mut t: token) {
    if (*dfa).localeinfo.multibyte as libc::c_int != 0
        && t == MBCSET as libc::c_int as libc::c_long
    {
        let mut need_or: bool = 0 as libc::c_int != 0;
        let mut i: idx_t = 0 as libc::c_int as idx_t;
        while i < (*dfa).lex.brack.nchars {
            addtok_wc(dfa, *((*dfa).lex.brack.chars).offset(i as isize) as wint_t);
            if need_or {
                addtok(dfa, OR as libc::c_int as token);
            }
            need_or = 1 as libc::c_int != 0;
            i += 1;
            i;
        }
        (*dfa).lex.brack.nchars = 0 as libc::c_int as idx_t;
        if (*dfa).lex.brack.cset != -(1 as libc::c_int) as libc::c_long {
            addtok(dfa, CSET as libc::c_int as libc::c_long + (*dfa).lex.brack.cset);
            if need_or {
                addtok(dfa, OR as libc::c_int as token);
            }
        }
    } else {
        addtok_mb(dfa, t, 3 as libc::c_int as libc::c_char);
    };
}
unsafe extern "C" fn addtok_wc(mut dfa: *mut dfa, mut wc: wint_t) {
    let mut buf: [libc::c_uchar; 16] = [0; 16];
    let mut s: mbstate_t = {
        let mut init = mbstate_t {
            __count: 0 as libc::c_int,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    let mut stored_bytes: size_t = wcrtomb(
        buf.as_mut_ptr() as *mut libc::c_char,
        wc as wchar_t,
        &mut s,
    );
    let mut buflen: libc::c_int = 0;
    if stored_bytes != -(1 as libc::c_int) as size_t {
        buflen = stored_bytes as libc::c_int;
    } else {
        buflen = 1 as libc::c_int;
        buf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    }
    addtok_mb(
        dfa,
        buf[0 as libc::c_int as usize] as token,
        (if buflen == 1 as libc::c_int { 3 as libc::c_int } else { 1 as libc::c_int })
            as libc::c_char,
    );
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < buflen {
        addtok_mb(
            dfa,
            buf[i as usize] as token,
            (if i == buflen - 1 as libc::c_int {
                2 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_char,
        );
        addtok(dfa, CAT as libc::c_int as token);
        i += 1;
        i;
    }
}
static mut utf8_classes: [charclass; 9] = [charclass { w: [0; 4] }; 9];
unsafe extern "C" fn add_utf8_anychar(mut dfa: *mut dfa) {
    if (*dfa).utf8_anychar_classes[0 as libc::c_int as usize]
        == 0 as libc::c_int as libc::c_long
    {
        let mut c: charclass = utf8_classes[0 as libc::c_int as usize];
        if (*dfa).syntax.syntax_bits
            & ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int == 0
        {
            clrbit('\n' as i32 as libc::c_uint, &mut c);
        }
        if (*dfa).syntax.syntax_bits
            & (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int != 0
        {
            clrbit('\0' as i32 as libc::c_uint, &mut c);
        }
        (*dfa)
            .utf8_anychar_classes[0 as libc::c_int
            as usize] = CSET as libc::c_int as libc::c_long
            + charclass_index(dfa, &mut c);
        let mut i: libc::c_int = 1 as libc::c_int;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[charclass; 9]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<charclass>() as libc::c_ulong)
        {
            (*dfa)
                .utf8_anychar_classes[i
                as usize] = CSET as libc::c_int as libc::c_long
                + charclass_index(dfa, &*utf8_classes.as_ptr().offset(i as isize));
            i += 1;
            i;
        }
    }
    addtok(dfa, (*dfa).utf8_anychar_classes[A as libc::c_int as usize]);
    addtok(dfa, (*dfa).utf8_anychar_classes[B as libc::c_int as usize]);
    addtok(dfa, D_token as libc::c_int as token);
    addtok(dfa, (*dfa).utf8_anychar_classes[E as libc::c_int as usize]);
    addtok(dfa, CAT as libc::c_int as token);
    addtok(dfa, OR as libc::c_int as token);
    addtok(dfa, G_token as libc::c_int as token);
    addtok(dfa, (*dfa).utf8_anychar_classes[H as libc::c_int as usize]);
    addtok(dfa, CAT as libc::c_int as token);
    addtok(dfa, OR as libc::c_int as token);
    addtok(dfa, (*dfa).utf8_anychar_classes[F as libc::c_int as usize]);
    addtok(dfa, I_token as libc::c_int as token);
    addtok(dfa, (*dfa).utf8_anychar_classes[J as libc::c_int as usize]);
    addtok(dfa, CAT as libc::c_int as token);
    addtok(dfa, OR as libc::c_int as token);
    addtok(dfa, L_token as libc::c_int as token);
    addtok(dfa, (*dfa).utf8_anychar_classes[M as libc::c_int as usize]);
    addtok(dfa, CAT as libc::c_int as token);
    addtok(dfa, OR as libc::c_int as token);
    addtok(dfa, (*dfa).utf8_anychar_classes[K as libc::c_int as usize]);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 3 as libc::c_int {
        addtok(dfa, (*dfa).utf8_anychar_classes[C as libc::c_int as usize]);
        addtok(dfa, CAT as libc::c_int as token);
        addtok(dfa, OR as libc::c_int as token);
        i_0 += 1;
        i_0;
    }
}
unsafe extern "C" fn atom(mut dfa: *mut dfa) {
    if 0 as libc::c_int as libc::c_long <= (*dfa).parse.tok
        && (*dfa).parse.tok < NOTCHAR as libc::c_int as libc::c_long
        || (*dfa).parse.tok >= CSET as libc::c_int as libc::c_long
        || (*dfa).parse.tok == BEG as libc::c_int as libc::c_long
        || (*dfa).parse.tok == BACKREF as libc::c_int as libc::c_long
        || (*dfa).parse.tok == BEGLINE as libc::c_int as libc::c_long
        || (*dfa).parse.tok == ENDLINE as libc::c_int as libc::c_long
        || (*dfa).parse.tok == BEGWORD as libc::c_int as libc::c_long
        || (*dfa).parse.tok == ENDWORD as libc::c_int as libc::c_long
        || (*dfa).parse.tok == LIMWORD as libc::c_int as libc::c_long
        || (*dfa).parse.tok == NOTLIMWORD as libc::c_int as libc::c_long
        || (*dfa).parse.tok == ANYCHAR as libc::c_int as libc::c_long
        || (*dfa).parse.tok == MBCSET as libc::c_int as libc::c_long
    {
        if (*dfa).parse.tok == ANYCHAR as libc::c_int as libc::c_long
            && (*dfa).localeinfo.using_utf8 as libc::c_int != 0
        {
            add_utf8_anychar(dfa);
        } else {
            addtok(dfa, (*dfa).parse.tok);
        }
        (*dfa).parse.tok = lex(dfa);
    } else if (*dfa).parse.tok == WCHAR as libc::c_int as libc::c_long {
        if (*dfa).lex.wctok == 0xffffffff as libc::c_uint {
            addtok(dfa, BACKREF as libc::c_int as token);
        } else {
            addtok_wc(dfa, (*dfa).lex.wctok);
            if (*dfa).syntax.case_fold {
                let mut folded: [wchar_t; 32] = [0; 32];
                let mut n: libc::c_int = case_folded_counterparts(
                    (*dfa).lex.wctok,
                    folded.as_mut_ptr(),
                );
                let mut i: libc::c_int = 0 as libc::c_int;
                while i < n {
                    addtok_wc(dfa, folded[i as usize] as wint_t);
                    addtok(dfa, OR as libc::c_int as token);
                    i += 1;
                    i;
                }
            }
        }
        (*dfa).parse.tok = lex(dfa);
    } else if (*dfa).parse.tok == LPAREN as libc::c_int as libc::c_long {
        (*dfa).parse.tok = lex(dfa);
        regexp(dfa);
        if (*dfa).parse.tok != RPAREN as libc::c_int as libc::c_long {
            dfaerror(
                dcgettext(
                    0 as *const libc::c_char,
                    b"unbalanced (\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        (*dfa).parse.tok = lex(dfa);
    } else {
        addtok(dfa, EMPTY as libc::c_int as token);
    };
}
unsafe extern "C" fn nsubtoks(mut dfa: *const dfa, mut tindex: idx_t) -> idx_t {
    match *((*dfa).tokens).offset((tindex - 1 as libc::c_int as libc::c_long) as isize) {
        257 | 258 | 259 => {
            return 1 as libc::c_int as libc::c_long
                + nsubtoks(dfa, tindex - 1 as libc::c_int as libc::c_long);
        }
        261 | 262 => {
            let mut ntoks1: idx_t = nsubtoks(
                dfa,
                tindex - 1 as libc::c_int as libc::c_long,
            );
            return 1 as libc::c_int as libc::c_long + ntoks1
                + nsubtoks(dfa, tindex - 1 as libc::c_int as libc::c_long - ntoks1);
        }
        _ => return 1 as libc::c_int as idx_t,
    };
}
unsafe extern "C" fn copytoks(mut dfa: *mut dfa, mut tindex: idx_t, mut ntokens: idx_t) {
    if (*dfa).localeinfo.multibyte {
        let mut i: idx_t = 0 as libc::c_int as idx_t;
        while i < ntokens {
            addtok_mb(
                dfa,
                *((*dfa).tokens).offset((tindex + i) as isize),
                *((*dfa).multibyte_prop).offset((tindex + i) as isize),
            );
            i += 1;
            i;
        }
    } else {
        let mut i_0: idx_t = 0 as libc::c_int as idx_t;
        while i_0 < ntokens {
            addtok_mb(
                dfa,
                *((*dfa).tokens).offset((tindex + i_0) as isize),
                3 as libc::c_int as libc::c_char,
            );
            i_0 += 1;
            i_0;
        }
    };
}
unsafe extern "C" fn closure(mut dfa: *mut dfa) {
    atom(dfa);
    while (*dfa).parse.tok == QMARK as libc::c_int as libc::c_long
        || (*dfa).parse.tok == STAR as libc::c_int as libc::c_long
        || (*dfa).parse.tok == PLUS as libc::c_int as libc::c_long
        || (*dfa).parse.tok == REPMN as libc::c_int as libc::c_long
    {
        if (*dfa).parse.tok == REPMN as libc::c_int as libc::c_long
            && ((*dfa).lex.minrep != 0 || (*dfa).lex.maxrep != 0)
        {
            let mut ntokens: idx_t = nsubtoks(dfa, (*dfa).tindex);
            let mut tindex: idx_t = (*dfa).tindex - ntokens;
            if (*dfa).lex.maxrep < 0 as libc::c_int {
                addtok(dfa, PLUS as libc::c_int as token);
            }
            if (*dfa).lex.minrep == 0 as libc::c_int {
                addtok(dfa, QMARK as libc::c_int as token);
            }
            let mut i: libc::c_int = 0;
            i = 1 as libc::c_int;
            while i < (*dfa).lex.minrep {
                copytoks(dfa, tindex, ntokens);
                addtok(dfa, CAT as libc::c_int as token);
                i += 1;
                i;
            }
            while i < (*dfa).lex.maxrep {
                copytoks(dfa, tindex, ntokens);
                addtok(dfa, QMARK as libc::c_int as token);
                addtok(dfa, CAT as libc::c_int as token);
                i += 1;
                i;
            }
            (*dfa).parse.tok = lex(dfa);
        } else if (*dfa).parse.tok == REPMN as libc::c_int as libc::c_long {
            (*dfa).tindex -= nsubtoks(dfa, (*dfa).tindex);
            (*dfa).parse.tok = lex(dfa);
            closure(dfa);
        } else {
            addtok(dfa, (*dfa).parse.tok);
            (*dfa).parse.tok = lex(dfa);
        }
    }
}
unsafe extern "C" fn branch(mut dfa: *mut dfa) {
    closure(dfa);
    while (*dfa).parse.tok != RPAREN as libc::c_int as libc::c_long
        && (*dfa).parse.tok != OR as libc::c_int as libc::c_long
        && (*dfa).parse.tok >= 0 as libc::c_int as libc::c_long
    {
        closure(dfa);
        addtok(dfa, CAT as libc::c_int as token);
    }
}
unsafe extern "C" fn regexp(mut dfa: *mut dfa) {
    branch(dfa);
    while (*dfa).parse.tok == OR as libc::c_int as libc::c_long {
        (*dfa).parse.tok = lex(dfa);
        branch(dfa);
        addtok(dfa, OR as libc::c_int as token);
    }
}
pub unsafe extern "C" fn dfaparse(
    mut s: *const libc::c_char,
    mut len: idx_t,
    mut d: *mut dfa,
) {
    (*d).lex.ptr = s;
    (*d).lex.left = len;
    (*d).lex.lasttok = END as libc::c_int as token;
    (*d).lex.laststart = 1 as libc::c_int != 0;
    if !(*d).syntax.syntax_bits_set {
        dfaerror(
            dcgettext(
                0 as *const libc::c_char,
                b"no syntax specified\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if (*d).nregexps == 0 {
        addtok(d, BEG as libc::c_int as token);
    }
    (*d).parse.tok = lex(d);
    (*d).parse.depth = (*d).depth;
    regexp(d);
    if (*d).parse.tok != END as libc::c_int as libc::c_long {
        dfaerror(
            dcgettext(
                0 as *const libc::c_char,
                b"unbalanced )\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    addtok(d, END as libc::c_int as libc::c_long - (*d).nregexps);
    addtok(d, CAT as libc::c_int as token);
    if (*d).nregexps != 0 {
        addtok(d, OR as libc::c_int as token);
    }
    (*d).nregexps += 1;
    (*d).nregexps;
}
unsafe extern "C" fn copy(mut src: *const position_set, mut dst: *mut position_set) {
    if (*dst).alloc < (*src).nelem {
        rpl_free((*dst).elems as *mut libc::c_void);
        (*dst)
            .elems = xpalloc(
            0 as *mut libc::c_void,
            &mut (*dst).alloc,
            (*src).nelem - (*dst).alloc,
            -(1 as libc::c_int) as ptrdiff_t,
            ::std::mem::size_of::<position>() as libc::c_ulong as idx_t,
        ) as *mut position;
    }
    (*dst).nelem = (*src).nelem;
    if (*src).nelem != 0 as libc::c_int as libc::c_long {
        memcpy(
            (*dst).elems as *mut libc::c_void,
            (*src).elems as *const libc::c_void,
            ((*src).nelem as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<position>() as libc::c_ulong),
        );
    }
}
unsafe extern "C" fn alloc_position_set(mut s: *mut position_set, mut size: idx_t) {
    (*s)
        .elems = xnmalloc(
        size as size_t,
        ::std::mem::size_of::<position>() as libc::c_ulong,
    ) as *mut position;
    (*s).alloc = size;
    (*s).nelem = 0 as libc::c_int as idx_t;
}
unsafe extern "C" fn insert(mut p: position, mut s: *mut position_set) {
    let mut count: idx_t = (*s).nelem;
    let mut lo: idx_t = 0 as libc::c_int as idx_t;
    let mut hi: idx_t = count;
    while lo < hi {
        let mut mid: idx_t = lo + hi >> 1 as libc::c_int;
        if (*((*s).elems).offset(mid as isize)).index < p.index {
            lo = mid + 1 as libc::c_int as libc::c_long;
        } else if (*((*s).elems).offset(mid as isize)).index == p.index {
            (*((*s).elems).offset(mid as isize)).constraint |= p.constraint;
            return;
        } else {
            hi = mid;
        }
    }
    (*s)
        .elems = maybe_realloc(
        (*s).elems as *mut libc::c_void,
        count,
        &mut (*s).alloc,
        -(1 as libc::c_int) as ptrdiff_t,
        ::std::mem::size_of::<position>() as libc::c_ulong as idx_t,
    ) as *mut position;
    let mut i: idx_t = count;
    while i > lo {
        *((*s).elems)
            .offset(
                i as isize,
            ) = *((*s).elems).offset((i - 1 as libc::c_int as libc::c_long) as isize);
        i -= 1;
        i;
    }
    *((*s).elems).offset(lo as isize) = p;
    (*s).nelem += 1;
    (*s).nelem;
}
unsafe extern "C" fn append(mut p: position, mut s: *mut position_set) {
    let mut count: idx_t = (*s).nelem;
    (*s)
        .elems = maybe_realloc(
        (*s).elems as *mut libc::c_void,
        count,
        &mut (*s).alloc,
        -(1 as libc::c_int) as ptrdiff_t,
        ::std::mem::size_of::<position>() as libc::c_ulong as idx_t,
    ) as *mut position;
    let fresh7 = (*s).nelem;
    (*s).nelem = (*s).nelem + 1;
    *((*s).elems).offset(fresh7 as isize) = p;
}
unsafe extern "C" fn merge_constrained(
    mut s1: *const position_set,
    mut s2: *const position_set,
    mut c2: libc::c_uint,
    mut m: *mut position_set,
) {
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    let mut j: idx_t = 0 as libc::c_int as idx_t;
    if (*m).alloc - (*s1).nelem < (*s2).nelem {
        rpl_free((*m).elems as *mut libc::c_void);
        (*m).alloc = (*s1).nelem;
        (*m)
            .elems = xpalloc(
            0 as *mut libc::c_void,
            &mut (*m).alloc,
            (*s2).nelem,
            -(1 as libc::c_int) as ptrdiff_t,
            ::std::mem::size_of::<position>() as libc::c_ulong as idx_t,
        ) as *mut position;
    }
    (*m).nelem = 0 as libc::c_int as idx_t;
    while i < (*s1).nelem || j < (*s2).nelem {
        if !(j < (*s2).nelem)
            || i < (*s1).nelem
                && (*((*s1).elems).offset(i as isize)).index
                    <= (*((*s2).elems).offset(j as isize)).index
        {
            let mut c: libc::c_uint = if i < (*s1).nelem && j < (*s2).nelem
                && (*((*s1).elems).offset(i as isize)).index
                    == (*((*s2).elems).offset(j as isize)).index
            {
                let fresh8 = j;
                j = j + 1;
                (*((*s2).elems).offset(fresh8 as isize)).constraint & c2
            } else {
                0 as libc::c_int as libc::c_uint
            };
            (*((*m).elems).offset((*m).nelem as isize))
                .index = (*((*s1).elems).offset(i as isize)).index;
            let fresh9 = i;
            i = i + 1;
            let fresh10 = (*m).nelem;
            (*m).nelem = (*m).nelem + 1;
            (*((*m).elems).offset(fresh10 as isize))
                .constraint = (*((*s1).elems).offset(fresh9 as isize)).constraint | c;
        } else {
            if (*((*s2).elems).offset(j as isize)).constraint & c2 != 0 {
                (*((*m).elems).offset((*m).nelem as isize))
                    .index = (*((*s2).elems).offset(j as isize)).index;
                let fresh11 = (*m).nelem;
                (*m).nelem = (*m).nelem + 1;
                (*((*m).elems).offset(fresh11 as isize))
                    .constraint = (*((*s2).elems).offset(j as isize)).constraint & c2;
            }
            j += 1;
            j;
        }
    }
}
unsafe extern "C" fn merge(
    mut s1: *const position_set,
    mut s2: *const position_set,
    mut m: *mut position_set,
) {
    merge_constrained(s1, s2, -(1 as libc::c_int) as libc::c_uint, m);
}
unsafe extern "C" fn merge2(
    mut dst: *mut position_set,
    mut src: *const position_set,
    mut m: *mut position_set,
) {
    if (*src).nelem < 4 as libc::c_int as libc::c_long {
        let mut i: idx_t = 0 as libc::c_int as idx_t;
        while i < (*src).nelem {
            insert(*((*src).elems).offset(i as isize), dst);
            i += 1;
            i;
        }
    } else {
        merge(src, dst, m);
        copy(m, dst);
    };
}
unsafe extern "C" fn delete(mut del: idx_t, mut s: *mut position_set) -> libc::c_uint {
    let mut count: idx_t = (*s).nelem;
    let mut lo: idx_t = 0 as libc::c_int as idx_t;
    let mut hi: idx_t = count;
    while lo < hi {
        let mut mid: idx_t = lo + hi >> 1 as libc::c_int;
        if (*((*s).elems).offset(mid as isize)).index < del {
            lo = mid + 1 as libc::c_int as libc::c_long;
        } else if (*((*s).elems).offset(mid as isize)).index == del {
            let mut c: libc::c_uint = (*((*s).elems).offset(mid as isize)).constraint;
            let mut i: idx_t = 0;
            i = mid;
            while (i + 1 as libc::c_int as libc::c_long) < count {
                *((*s).elems)
                    .offset(
                        i as isize,
                    ) = *((*s).elems)
                    .offset((i + 1 as libc::c_int as libc::c_long) as isize);
                i += 1;
                i;
            }
            (*s).nelem = i;
            return c;
        } else {
            hi = mid;
        }
    }
    return 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn replace(
    mut dst: *mut position_set,
    mut del: idx_t,
    mut add: *mut position_set,
    mut constraint: libc::c_uint,
    mut tmp: *mut position_set,
) {
    let mut c: libc::c_uint = delete(del, dst) & constraint;
    if c != 0 {
        copy(dst, tmp);
        merge_constrained(tmp, add, c, dst);
    }
}
unsafe extern "C" fn state_index(
    mut d: *mut dfa,
    mut s: *const position_set,
    mut context: libc::c_int,
) -> state_num {
    let mut hash: size_t = 0 as libc::c_int as size_t;
    let mut constraint: libc::c_int = 0 as libc::c_int;
    let mut i: state_num = 0;
    i = 0 as libc::c_int as state_num;
    while i < (*s).nelem {
        let mut ind: idx_t = (*((*s).elems).offset(i as isize)).index;
        hash
            ^= (ind + (*((*s).elems).offset(i as isize)).constraint as libc::c_long)
                as libc::c_ulong;
        i += 1;
        i;
    }
    i = 0 as libc::c_int as state_num;
    while i < (*d).sindex {
        if !(hash != (*((*d).states).offset(i as isize)).hash
            || (*s).nelem != (*((*d).states).offset(i as isize)).elems.nelem
            || context != (*((*d).states).offset(i as isize)).context as libc::c_int)
        {
            let mut j: state_num = 0;
            j = 0 as libc::c_int as state_num;
            while j < (*s).nelem {
                if (*((*s).elems).offset(j as isize)).constraint
                    != (*((*((*d).states).offset(i as isize)).elems.elems)
                        .offset(j as isize))
                        .constraint
                    || (*((*s).elems).offset(j as isize)).index
                        != (*((*((*d).states).offset(i as isize)).elems.elems)
                            .offset(j as isize))
                            .index
                {
                    break;
                }
                j += 1;
                j;
            }
            if j == (*s).nelem {
                return i;
            }
        }
        i += 1;
        i;
    }
    let mut j_0: state_num = 0 as libc::c_int as state_num;
    while j_0 < (*s).nelem {
        let mut c: libc::c_int = *((*d).constraints)
            .offset((*((*s).elems).offset(j_0 as isize)).index as isize);
        if c != 0 as libc::c_int {
            if succeeds_in_context(c, context, CTX_ANY as libc::c_int) {
                constraint |= c;
            }
        } else if *((*d).tokens)
            .offset((*((*s).elems).offset(j_0 as isize)).index as isize)
            == BACKREF as libc::c_int as libc::c_long
        {
            constraint = NO_CONSTRAINT as libc::c_int;
        }
        j_0 += 1;
        j_0;
    }
    (*d)
        .states = maybe_realloc(
        (*d).states as *mut libc::c_void,
        (*d).sindex,
        &mut (*d).salloc,
        -(1 as libc::c_int) as ptrdiff_t,
        ::std::mem::size_of::<dfa_state>() as libc::c_ulong as idx_t,
    ) as *mut dfa_state;
    (*((*d).states).offset(i as isize)).hash = hash;
    alloc_position_set(&mut (*((*d).states).offset(i as isize)).elems, (*s).nelem);
    copy(s, &mut (*((*d).states).offset(i as isize)).elems);
    (*((*d).states).offset(i as isize)).context = context as libc::c_uchar;
    (*((*d).states).offset(i as isize)).constraint = constraint as libc::c_ushort;
    (*((*d).states).offset(i as isize)).mbps.nelem = 0 as libc::c_int as idx_t;
    let ref mut fresh12 = (*((*d).states).offset(i as isize)).mbps.elems;
    *fresh12 = 0 as *mut position;
    (*((*d).states).offset(i as isize)).mb_trindex = -(1 as libc::c_int) as state_num;
    (*d).sindex += 1;
    (*d).sindex;
    return i;
}
unsafe extern "C" fn epsclosure(mut d: *const dfa, mut backward: *mut position_set) {
    let mut tmp: position_set = position_set {
        elems: 0 as *mut position,
        nelem: 0,
        alloc: 0,
    };
    alloc_position_set(&mut tmp, (*d).nleaves);
    let mut current_block_12: u64;
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    while i < (*d).tindex {
        if (0 as libc::c_int as libc::c_long)
            < (*((*d).follows).offset(i as isize)).nelem
        {
            let mut constraint: libc::c_uint = 0;
            match *((*d).tokens).offset(i as isize) {
                268 => {
                    current_block_12 = 18352545835874387041;
                    match current_block_12 {
                        16971046806842451031 => {
                            constraint = NO_CONSTRAINT as libc::c_int as libc::c_uint;
                        }
                        8787322078181601953 => {
                            constraint = ENDLINE_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        8716995421382701094 => {
                            constraint = BEGWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        9374350438963963669 => {
                            constraint = ENDWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        6814113300505932684 => {
                            constraint = LIMWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        8579187331575899491 => {
                            constraint = NOTLIMWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        _ => {
                            constraint = BEGLINE_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                    }
                    delete(i, &mut *((*d).follows).offset(i as isize));
                    let mut j: idx_t = 0 as libc::c_int as idx_t;
                    while j < (*backward.offset(i as isize)).nelem {
                        replace(
                            &mut *((*d).follows)
                                .offset(
                                    (*((*backward.offset(i as isize)).elems).offset(j as isize))
                                        .index as isize,
                                ),
                            i,
                            &mut *((*d).follows).offset(i as isize),
                            constraint,
                            &mut tmp,
                        );
                        j += 1;
                        j;
                    }
                    let mut j_0: idx_t = 0 as libc::c_int as idx_t;
                    while j_0 < (*((*d).follows).offset(i as isize)).nelem {
                        replace(
                            &mut *backward
                                .offset(
                                    (*((*((*d).follows).offset(i as isize)).elems)
                                        .offset(j_0 as isize))
                                        .index as isize,
                                ),
                            i,
                            &mut *backward.offset(i as isize),
                            NO_CONSTRAINT as libc::c_int as libc::c_uint,
                            &mut tmp,
                        );
                        j_0 += 1;
                        j_0;
                    }
                }
                269 => {
                    current_block_12 = 8787322078181601953;
                    match current_block_12 {
                        16971046806842451031 => {
                            constraint = NO_CONSTRAINT as libc::c_int as libc::c_uint;
                        }
                        8787322078181601953 => {
                            constraint = ENDLINE_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        8716995421382701094 => {
                            constraint = BEGWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        9374350438963963669 => {
                            constraint = ENDWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        6814113300505932684 => {
                            constraint = LIMWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        8579187331575899491 => {
                            constraint = NOTLIMWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        _ => {
                            constraint = BEGLINE_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                    }
                    delete(i, &mut *((*d).follows).offset(i as isize));
                    let mut j: idx_t = 0 as libc::c_int as idx_t;
                    while j < (*backward.offset(i as isize)).nelem {
                        replace(
                            &mut *((*d).follows)
                                .offset(
                                    (*((*backward.offset(i as isize)).elems).offset(j as isize))
                                        .index as isize,
                                ),
                            i,
                            &mut *((*d).follows).offset(i as isize),
                            constraint,
                            &mut tmp,
                        );
                        j += 1;
                        j;
                    }
                    let mut j_0: idx_t = 0 as libc::c_int as idx_t;
                    while j_0 < (*((*d).follows).offset(i as isize)).nelem {
                        replace(
                            &mut *backward
                                .offset(
                                    (*((*((*d).follows).offset(i as isize)).elems)
                                        .offset(j_0 as isize))
                                        .index as isize,
                                ),
                            i,
                            &mut *backward.offset(i as isize),
                            NO_CONSTRAINT as libc::c_int as libc::c_uint,
                            &mut tmp,
                        );
                        j_0 += 1;
                        j_0;
                    }
                }
                270 => {
                    current_block_12 = 8716995421382701094;
                    match current_block_12 {
                        16971046806842451031 => {
                            constraint = NO_CONSTRAINT as libc::c_int as libc::c_uint;
                        }
                        8787322078181601953 => {
                            constraint = ENDLINE_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        8716995421382701094 => {
                            constraint = BEGWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        9374350438963963669 => {
                            constraint = ENDWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        6814113300505932684 => {
                            constraint = LIMWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        8579187331575899491 => {
                            constraint = NOTLIMWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        _ => {
                            constraint = BEGLINE_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                    }
                    delete(i, &mut *((*d).follows).offset(i as isize));
                    let mut j: idx_t = 0 as libc::c_int as idx_t;
                    while j < (*backward.offset(i as isize)).nelem {
                        replace(
                            &mut *((*d).follows)
                                .offset(
                                    (*((*backward.offset(i as isize)).elems).offset(j as isize))
                                        .index as isize,
                                ),
                            i,
                            &mut *((*d).follows).offset(i as isize),
                            constraint,
                            &mut tmp,
                        );
                        j += 1;
                        j;
                    }
                    let mut j_0: idx_t = 0 as libc::c_int as idx_t;
                    while j_0 < (*((*d).follows).offset(i as isize)).nelem {
                        replace(
                            &mut *backward
                                .offset(
                                    (*((*((*d).follows).offset(i as isize)).elems)
                                        .offset(j_0 as isize))
                                        .index as isize,
                                ),
                            i,
                            &mut *backward.offset(i as isize),
                            NO_CONSTRAINT as libc::c_int as libc::c_uint,
                            &mut tmp,
                        );
                        j_0 += 1;
                        j_0;
                    }
                }
                271 => {
                    current_block_12 = 9374350438963963669;
                    match current_block_12 {
                        16971046806842451031 => {
                            constraint = NO_CONSTRAINT as libc::c_int as libc::c_uint;
                        }
                        8787322078181601953 => {
                            constraint = ENDLINE_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        8716995421382701094 => {
                            constraint = BEGWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        9374350438963963669 => {
                            constraint = ENDWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        6814113300505932684 => {
                            constraint = LIMWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        8579187331575899491 => {
                            constraint = NOTLIMWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        _ => {
                            constraint = BEGLINE_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                    }
                    delete(i, &mut *((*d).follows).offset(i as isize));
                    let mut j: idx_t = 0 as libc::c_int as idx_t;
                    while j < (*backward.offset(i as isize)).nelem {
                        replace(
                            &mut *((*d).follows)
                                .offset(
                                    (*((*backward.offset(i as isize)).elems).offset(j as isize))
                                        .index as isize,
                                ),
                            i,
                            &mut *((*d).follows).offset(i as isize),
                            constraint,
                            &mut tmp,
                        );
                        j += 1;
                        j;
                    }
                    let mut j_0: idx_t = 0 as libc::c_int as idx_t;
                    while j_0 < (*((*d).follows).offset(i as isize)).nelem {
                        replace(
                            &mut *backward
                                .offset(
                                    (*((*((*d).follows).offset(i as isize)).elems)
                                        .offset(j_0 as isize))
                                        .index as isize,
                                ),
                            i,
                            &mut *backward.offset(i as isize),
                            NO_CONSTRAINT as libc::c_int as libc::c_uint,
                            &mut tmp,
                        );
                        j_0 += 1;
                        j_0;
                    }
                }
                272 => {
                    current_block_12 = 6814113300505932684;
                    match current_block_12 {
                        16971046806842451031 => {
                            constraint = NO_CONSTRAINT as libc::c_int as libc::c_uint;
                        }
                        8787322078181601953 => {
                            constraint = ENDLINE_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        8716995421382701094 => {
                            constraint = BEGWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        9374350438963963669 => {
                            constraint = ENDWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        6814113300505932684 => {
                            constraint = LIMWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        8579187331575899491 => {
                            constraint = NOTLIMWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        _ => {
                            constraint = BEGLINE_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                    }
                    delete(i, &mut *((*d).follows).offset(i as isize));
                    let mut j: idx_t = 0 as libc::c_int as idx_t;
                    while j < (*backward.offset(i as isize)).nelem {
                        replace(
                            &mut *((*d).follows)
                                .offset(
                                    (*((*backward.offset(i as isize)).elems).offset(j as isize))
                                        .index as isize,
                                ),
                            i,
                            &mut *((*d).follows).offset(i as isize),
                            constraint,
                            &mut tmp,
                        );
                        j += 1;
                        j;
                    }
                    let mut j_0: idx_t = 0 as libc::c_int as idx_t;
                    while j_0 < (*((*d).follows).offset(i as isize)).nelem {
                        replace(
                            &mut *backward
                                .offset(
                                    (*((*((*d).follows).offset(i as isize)).elems)
                                        .offset(j_0 as isize))
                                        .index as isize,
                                ),
                            i,
                            &mut *backward.offset(i as isize),
                            NO_CONSTRAINT as libc::c_int as libc::c_uint,
                            &mut tmp,
                        );
                        j_0 += 1;
                        j_0;
                    }
                }
                273 => {
                    current_block_12 = 8579187331575899491;
                    match current_block_12 {
                        16971046806842451031 => {
                            constraint = NO_CONSTRAINT as libc::c_int as libc::c_uint;
                        }
                        8787322078181601953 => {
                            constraint = ENDLINE_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        8716995421382701094 => {
                            constraint = BEGWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        9374350438963963669 => {
                            constraint = ENDWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        6814113300505932684 => {
                            constraint = LIMWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        8579187331575899491 => {
                            constraint = NOTLIMWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        _ => {
                            constraint = BEGLINE_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                    }
                    delete(i, &mut *((*d).follows).offset(i as isize));
                    let mut j: idx_t = 0 as libc::c_int as idx_t;
                    while j < (*backward.offset(i as isize)).nelem {
                        replace(
                            &mut *((*d).follows)
                                .offset(
                                    (*((*backward.offset(i as isize)).elems).offset(j as isize))
                                        .index as isize,
                                ),
                            i,
                            &mut *((*d).follows).offset(i as isize),
                            constraint,
                            &mut tmp,
                        );
                        j += 1;
                        j;
                    }
                    let mut j_0: idx_t = 0 as libc::c_int as idx_t;
                    while j_0 < (*((*d).follows).offset(i as isize)).nelem {
                        replace(
                            &mut *backward
                                .offset(
                                    (*((*((*d).follows).offset(i as isize)).elems)
                                        .offset(j_0 as isize))
                                        .index as isize,
                                ),
                            i,
                            &mut *backward.offset(i as isize),
                            NO_CONSTRAINT as libc::c_int as libc::c_uint,
                            &mut tmp,
                        );
                        j_0 += 1;
                        j_0;
                    }
                }
                256 => {
                    current_block_12 = 16971046806842451031;
                    match current_block_12 {
                        16971046806842451031 => {
                            constraint = NO_CONSTRAINT as libc::c_int as libc::c_uint;
                        }
                        8787322078181601953 => {
                            constraint = ENDLINE_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        8716995421382701094 => {
                            constraint = BEGWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        9374350438963963669 => {
                            constraint = ENDWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        6814113300505932684 => {
                            constraint = LIMWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        8579187331575899491 => {
                            constraint = NOTLIMWORD_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                        _ => {
                            constraint = BEGLINE_CONSTRAINT as libc::c_int
                                as libc::c_uint;
                        }
                    }
                    delete(i, &mut *((*d).follows).offset(i as isize));
                    let mut j: idx_t = 0 as libc::c_int as idx_t;
                    while j < (*backward.offset(i as isize)).nelem {
                        replace(
                            &mut *((*d).follows)
                                .offset(
                                    (*((*backward.offset(i as isize)).elems).offset(j as isize))
                                        .index as isize,
                                ),
                            i,
                            &mut *((*d).follows).offset(i as isize),
                            constraint,
                            &mut tmp,
                        );
                        j += 1;
                        j;
                    }
                    let mut j_0: idx_t = 0 as libc::c_int as idx_t;
                    while j_0 < (*((*d).follows).offset(i as isize)).nelem {
                        replace(
                            &mut *backward
                                .offset(
                                    (*((*((*d).follows).offset(i as isize)).elems)
                                        .offset(j_0 as isize))
                                        .index as isize,
                                ),
                            i,
                            &mut *backward.offset(i as isize),
                            NO_CONSTRAINT as libc::c_int as libc::c_uint,
                            &mut tmp,
                        );
                        j_0 += 1;
                        j_0;
                    }
                }
                _ => {}
            }
        }
        i += 1;
        i;
    }
    rpl_free(tmp.elems as *mut libc::c_void);
}
unsafe extern "C" fn charclass_context(
    mut dfa: *const dfa,
    mut c: *const charclass,
) -> libc::c_int {
    let mut context: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < CHARCLASS_WORDS as libc::c_int {
        if (*c).w[j as usize] & (*dfa).syntax.newline.w[j as usize] != 0 {
            context |= CTX_NEWLINE as libc::c_int;
        }
        if (*c).w[j as usize] & (*dfa).syntax.letters.w[j as usize] != 0 {
            context |= CTX_LETTER as libc::c_int;
        }
        if (*c).w[j as usize]
            & !((*dfa).syntax.letters.w[j as usize]
                | (*dfa).syntax.newline.w[j as usize]) != 0
        {
            context |= CTX_NONE as libc::c_int;
        }
        j += 1;
        j;
    }
    return context;
}
unsafe extern "C" fn state_separate_contexts(
    mut d: *mut dfa,
    mut s: *const position_set,
) -> libc::c_int {
    let mut separate_contexts: libc::c_int = 0 as libc::c_int;
    let mut j: idx_t = 0 as libc::c_int as idx_t;
    while j < (*s).nelem {
        separate_contexts
            |= *((*d).separates)
                .offset((*((*s).elems).offset(j as isize)).index as isize);
        j += 1;
        j;
    }
    return separate_contexts;
}
unsafe extern "C" fn merge_nfa_state(
    mut d: *mut dfa,
    mut tindex: idx_t,
    mut flags: *mut libc::c_char,
    mut merged: *mut position_set,
) {
    let mut follows: *mut position_set = (*d).follows;
    let mut nelem: idx_t = 0 as libc::c_int as idx_t;
    let mut current_block_7: u64;
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    while i < (*follows.offset(tindex as isize)).nelem {
        let mut sindex: idx_t = (*((*follows.offset(tindex as isize)).elems)
            .offset(i as isize))
            .index;
        let mut iconstraint: libc::c_uint = (*((*follows.offset(tindex as isize)).elems)
            .offset(i as isize))
            .constraint;
        if !(iconstraint == 0 as libc::c_int as libc::c_uint) {
            if *((*d).tokens)
                .offset(
                    (*((*follows.offset(tindex as isize)).elems).offset(i as isize))
                        .index as isize,
                ) <= END as libc::c_int as libc::c_long
            {
                let ref mut fresh13 = *((*d).constraints).offset(tindex as isize);
                *fresh13 = (*fresh13 as libc::c_uint
                    | (*((*follows.offset(tindex as isize)).elems).offset(i as isize))
                        .constraint) as libc::c_int;
            } else {
                if sindex != tindex
                    && *flags.offset(sindex as isize) as libc::c_int
                        & (OPT_LPAREN as libc::c_int | OPT_RPAREN as libc::c_int) == 0
                {
                    let mut j: idx_t = 0;
                    j = 0 as libc::c_int as idx_t;
                    while j < nelem {
                        let mut dindex: idx_t = (*((*follows.offset(tindex as isize))
                            .elems)
                            .offset(j as isize))
                            .index;
                        if !(dindex == tindex) {
                            if !((*((*follows.offset(tindex as isize)).elems)
                                .offset(j as isize))
                                .constraint != iconstraint)
                            {
                                if !(*flags.offset(dindex as isize) as libc::c_int
                                    & (OPT_LPAREN as libc::c_int | OPT_RPAREN as libc::c_int)
                                    != 0)
                                {
                                    if !(*((*d).tokens).offset(sindex as isize)
                                        != *((*d).tokens).offset(dindex as isize))
                                    {
                                        if !((*flags.offset(sindex as isize) as libc::c_int
                                            ^ *flags.offset(dindex as isize) as libc::c_int)
                                            & OPT_REPEAT as libc::c_int != 0)
                                        {
                                            if *flags.offset(sindex as isize) as libc::c_int
                                                & OPT_REPEAT as libc::c_int != 0
                                            {
                                                delete(sindex, &mut *follows.offset(sindex as isize));
                                            }
                                            merge2(
                                                &mut *follows.offset(dindex as isize),
                                                &mut *follows.offset(sindex as isize),
                                                merged,
                                            );
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        j += 1;
                        j;
                    }
                    if j < nelem {
                        current_block_7 = 16668937799742929182;
                    } else {
                        current_block_7 = 13242334135786603907;
                    }
                } else {
                    current_block_7 = 13242334135786603907;
                }
                match current_block_7 {
                    16668937799742929182 => {}
                    _ => {
                        let fresh14 = nelem;
                        nelem = nelem + 1;
                        *((*follows.offset(tindex as isize)).elems)
                            .offset(
                                fresh14 as isize,
                            ) = *((*follows.offset(tindex as isize)).elems)
                            .offset(i as isize);
                        let ref mut fresh15 = *flags.offset(sindex as isize);
                        *fresh15 = (*fresh15 as libc::c_int | OPT_QUEUED as libc::c_int)
                            as libc::c_char;
                    }
                }
            }
        }
        i += 1;
        i;
    }
    (*follows.offset(tindex as isize)).nelem = nelem;
}
unsafe extern "C" fn compare(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut p: *const position = a as *const position;
    let mut q: *const position = b as *const position;
    return ((*p).index > (*q).index) as libc::c_int
        - ((*p).index < (*q).index) as libc::c_int;
}
unsafe extern "C" fn reorder_tokens(mut d: *mut dfa) {
    let mut nleaves: idx_t = 0 as libc::c_int as idx_t;
    let mut map: *mut ptrdiff_t = xnmalloc(
        (*d).tindex as size_t,
        ::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong,
    ) as *mut ptrdiff_t;
    let fresh16 = nleaves;
    nleaves = nleaves + 1;
    *map.offset(0 as libc::c_int as isize) = fresh16;
    let mut i: idx_t = 1 as libc::c_int as idx_t;
    while i < (*d).tindex {
        *map.offset(i as isize) = -(1 as libc::c_int) as ptrdiff_t;
        i += 1;
        i;
    }
    let mut tokens: *mut token = xnmalloc(
        (*d).nleaves as size_t,
        ::std::mem::size_of::<token>() as libc::c_ulong,
    ) as *mut token;
    let mut follows: *mut position_set = xnmalloc(
        (*d).nleaves as size_t,
        ::std::mem::size_of::<position_set>() as libc::c_ulong,
    ) as *mut position_set;
    let mut constraints: *mut libc::c_int = xnmalloc(
        (*d).nleaves as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut multibyte_prop: *mut libc::c_char = (if (*d).localeinfo.multibyte
        as libc::c_int != 0
    {
        xnmalloc(
            (*d).nleaves as size_t,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        )
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_char;
    let mut i_0: idx_t = 0 as libc::c_int as idx_t;
    while i_0 < (*d).tindex {
        if *map.offset(i_0 as isize) < 0 as libc::c_int as libc::c_long {
            rpl_free((*((*d).follows).offset(i_0 as isize)).elems as *mut libc::c_void);
            let ref mut fresh17 = (*((*d).follows).offset(i_0 as isize)).elems;
            *fresh17 = 0 as *mut position;
            (*((*d).follows).offset(i_0 as isize)).nelem = 0 as libc::c_int as idx_t;
        } else {
            *tokens
                .offset(
                    *map.offset(i_0 as isize) as isize,
                ) = *((*d).tokens).offset(i_0 as isize);
            *follows
                .offset(
                    *map.offset(i_0 as isize) as isize,
                ) = *((*d).follows).offset(i_0 as isize);
            *constraints
                .offset(
                    *map.offset(i_0 as isize) as isize,
                ) = *((*d).constraints).offset(i_0 as isize);
            if !multibyte_prop.is_null() {
                *multibyte_prop
                    .offset(
                        *map.offset(i_0 as isize) as isize,
                    ) = *((*d).multibyte_prop).offset(i_0 as isize);
            }
            let mut j: idx_t = 0 as libc::c_int as idx_t;
            while j < (*((*d).follows).offset(i_0 as isize)).nelem {
                if *map
                    .offset(
                        (*((*((*d).follows).offset(i_0 as isize)).elems)
                            .offset(j as isize))
                            .index as isize,
                    ) == -(1 as libc::c_int) as libc::c_long
                {
                    let fresh18 = nleaves;
                    nleaves = nleaves + 1;
                    *map
                        .offset(
                            (*((*((*d).follows).offset(i_0 as isize)).elems)
                                .offset(j as isize))
                                .index as isize,
                        ) = fresh18;
                }
                (*((*((*d).follows).offset(i_0 as isize)).elems).offset(j as isize))
                    .index = *map
                    .offset(
                        (*((*((*d).follows).offset(i_0 as isize)).elems)
                            .offset(j as isize))
                            .index as isize,
                    );
                j += 1;
                j;
            }
            qsort(
                (*((*d).follows).offset(i_0 as isize)).elems as *mut libc::c_void,
                (*((*d).follows).offset(i_0 as isize)).nelem as size_t,
                ::std::mem::size_of::<position>() as libc::c_ulong,
                Some(
                    compare
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
        }
        i_0 += 1;
        i_0;
    }
    let mut i_1: idx_t = 0 as libc::c_int as idx_t;
    while i_1 < nleaves {
        *((*d).tokens).offset(i_1 as isize) = *tokens.offset(i_1 as isize);
        *((*d).follows).offset(i_1 as isize) = *follows.offset(i_1 as isize);
        *((*d).constraints).offset(i_1 as isize) = *constraints.offset(i_1 as isize);
        if !multibyte_prop.is_null() {
            *((*d).multibyte_prop)
                .offset(i_1 as isize) = *multibyte_prop.offset(i_1 as isize);
        }
        i_1 += 1;
        i_1;
    }
    (*d).nleaves = nleaves;
    (*d).tindex = (*d).nleaves;
    rpl_free(tokens as *mut libc::c_void);
    rpl_free(follows as *mut libc::c_void);
    rpl_free(constraints as *mut libc::c_void);
    rpl_free(multibyte_prop as *mut libc::c_void);
    rpl_free(map as *mut libc::c_void);
}
unsafe extern "C" fn dfaoptimize(mut d: *mut dfa) {
    let mut flags: *mut libc::c_char = xizalloc((*d).tindex) as *mut libc::c_char;
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    while i < (*d).tindex {
        let mut j: idx_t = 0 as libc::c_int as idx_t;
        while j < (*((*d).follows).offset(i as isize)).nelem {
            if (*((*((*d).follows).offset(i as isize)).elems).offset(j as isize)).index
                == i
            {
                let ref mut fresh19 = *flags
                    .offset(
                        (*((*((*d).follows).offset(i as isize)).elems)
                            .offset(j as isize))
                            .index as isize,
                    );
                *fresh19 = (*fresh19 as libc::c_int | OPT_REPEAT as libc::c_int)
                    as libc::c_char;
            } else if (*((*((*d).follows).offset(i as isize)).elems).offset(j as isize))
                .index < i
            {
                let ref mut fresh20 = *flags
                    .offset(
                        (*((*((*d).follows).offset(i as isize)).elems)
                            .offset(j as isize))
                            .index as isize,
                    );
                *fresh20 = (*fresh20 as libc::c_int | OPT_LPAREN as libc::c_int)
                    as libc::c_char;
            } else {
                let ref mut fresh21 = *flags
                    .offset(
                        (*((*((*d).follows).offset(i as isize)).elems)
                            .offset(j as isize))
                            .index as isize,
                    );
                *fresh21 = (*fresh21 as libc::c_int & OPT_WALKED as libc::c_int)
                    as libc::c_char;
                if *fresh21 != 0 {
                    let ref mut fresh22 = *flags
                        .offset(
                            (*((*((*d).follows).offset(i as isize)).elems)
                                .offset(j as isize))
                                .index as isize,
                        );
                    *fresh22 = (*fresh22 as libc::c_int | OPT_RPAREN as libc::c_int)
                        as libc::c_char;
                } else {
                    let ref mut fresh23 = *flags
                        .offset(
                            (*((*((*d).follows).offset(i as isize)).elems)
                                .offset(j as isize))
                                .index as isize,
                        );
                    *fresh23 = (*fresh23 as libc::c_int | OPT_WALKED as libc::c_int)
                        as libc::c_char;
                }
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    let ref mut fresh24 = *flags.offset(0 as libc::c_int as isize);
    *fresh24 = (*fresh24 as libc::c_int | OPT_QUEUED as libc::c_int) as libc::c_char;
    let mut merged0: position_set = position_set {
        elems: 0 as *mut position,
        nelem: 0,
        alloc: 0,
    };
    let mut merged: *mut position_set = &mut merged0;
    alloc_position_set(merged, (*d).nleaves);
    (*d)
        .constraints = xicalloc(
        (*d).tindex,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as idx_t,
    ) as *mut libc::c_int;
    let mut i_0: idx_t = 0 as libc::c_int as idx_t;
    while i_0 < (*d).tindex {
        if *flags.offset(i_0 as isize) as libc::c_int & OPT_QUEUED as libc::c_int != 0 {
            merge_nfa_state(d, i_0, flags, merged);
        }
        i_0 += 1;
        i_0;
    }
    reorder_tokens(d);
    rpl_free((*merged).elems as *mut libc::c_void);
    rpl_free(flags as *mut libc::c_void);
}
unsafe extern "C" fn dfaanalyze(mut d: *mut dfa, mut searchflag: bool) {
    let mut posalloc: *mut position = xnmalloc(
        (*d).nleaves as size_t,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<position>() as libc::c_ulong),
    ) as *mut position;
    let mut firstpos: *mut position = posalloc;
    let mut lastpos: *mut position = firstpos.offset((*d).nleaves as isize);
    let mut pos: position = position {
        index: 0,
        constraint: 0,
    };
    let mut tmp: position_set = position_set {
        elems: 0 as *mut position,
        nelem: 0,
        alloc: 0,
    };
    let mut stkalloc: *mut C2RustUnnamed_6 = xnmalloc(
        (*d).depth as size_t,
        ::std::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong,
    ) as *mut C2RustUnnamed_6;
    let mut stk: *mut C2RustUnnamed_6 = stkalloc;
    let mut merged: position_set = position_set {
        elems: 0 as *mut position,
        nelem: 0,
        alloc: 0,
    };
    addtok(d, CAT as libc::c_int as token);
    let mut tindex: idx_t = (*d).tindex;
    (*d).searchflag = searchflag;
    alloc_position_set(&mut merged, (*d).nleaves);
    (*d)
        .follows = xicalloc(
        tindex,
        ::std::mem::size_of::<position_set>() as libc::c_ulong as idx_t,
    ) as *mut position_set;
    let mut backward: *mut position_set = (if (*d).epsilon as libc::c_int != 0 {
        xicalloc(tindex, ::std::mem::size_of::<position_set>() as libc::c_ulong as idx_t)
    } else {
        0 as *mut libc::c_void
    }) as *mut position_set;
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    while i < tindex {
        let mut current_block_53: u64;
        match *((*d).tokens).offset(i as isize) {
            256 => {
                (*stk).nullable = 1 as libc::c_int != 0;
                (*stk).nlastpos = 0 as libc::c_int as idx_t;
                (*stk).nfirstpos = (*stk).nlastpos;
                stk = stk.offset(1);
                stk;
                current_block_53 = 1724319918354933278;
            }
            258 | 259 => {
                if (*d).epsilon {
                    tmp
                        .elems = lastpos
                        .offset(
                            -((*stk.offset(-(1 as libc::c_int) as isize)).nlastpos
                                as isize),
                        );
                    tmp.nelem = (*stk.offset(-(1 as libc::c_int) as isize)).nlastpos;
                    let mut p: *mut position = firstpos
                        .offset(
                            -((*stk.offset(-(1 as libc::c_int) as isize)).nfirstpos
                                as isize),
                        );
                    while p < firstpos {
                        merge2(
                            &mut *backward.offset((*p).index as isize),
                            &mut tmp,
                            &mut merged,
                        );
                        p = p.offset(1);
                        p;
                    }
                }
                tmp
                    .elems = firstpos
                    .offset(
                        -((*stk.offset(-(1 as libc::c_int) as isize)).nfirstpos as isize),
                    );
                tmp.nelem = (*stk.offset(-(1 as libc::c_int) as isize)).nfirstpos;
                let mut p_0: *mut position = lastpos
                    .offset(
                        -((*stk.offset(-(1 as libc::c_int) as isize)).nlastpos as isize),
                    );
                while p_0 < lastpos {
                    merge2(
                        &mut *((*d).follows).offset((*p_0).index as isize),
                        &mut tmp,
                        &mut merged,
                    );
                    p_0 = p_0.offset(1);
                    p_0;
                }
                current_block_53 = 6103263293371866127;
            }
            257 => {
                current_block_53 = 6103263293371866127;
            }
            261 => {
                if !backward.is_null() {
                    tmp.nelem = (*stk.offset(-(2 as libc::c_int) as isize)).nlastpos;
                    tmp
                        .elems = lastpos
                        .offset(
                            -((*stk.offset(-(1 as libc::c_int) as isize)).nlastpos
                                as isize),
                        )
                        .offset(
                            -((*stk.offset(-(2 as libc::c_int) as isize)).nlastpos
                                as isize),
                        );
                    let mut p_1: *mut position = firstpos
                        .offset(
                            -((*stk.offset(-(1 as libc::c_int) as isize)).nfirstpos
                                as isize),
                        );
                    while p_1 < firstpos {
                        merge2(
                            &mut *backward.offset((*p_1).index as isize),
                            &mut tmp,
                            &mut merged,
                        );
                        p_1 = p_1.offset(1);
                        p_1;
                    }
                }
                tmp.nelem = (*stk.offset(-(1 as libc::c_int) as isize)).nfirstpos;
                tmp
                    .elems = firstpos
                    .offset(
                        -((*stk.offset(-(1 as libc::c_int) as isize)).nfirstpos as isize),
                    );
                let mut plim: *mut position = lastpos
                    .offset(
                        -((*stk.offset(-(1 as libc::c_int) as isize)).nlastpos as isize),
                    );
                let mut p_2: *mut position = plim
                    .offset(
                        -((*stk.offset(-(2 as libc::c_int) as isize)).nlastpos as isize),
                    );
                while p_2 < plim {
                    merge2(
                        &mut *((*d).follows).offset((*p_2).index as isize),
                        &mut tmp,
                        &mut merged,
                    );
                    p_2 = p_2.offset(1);
                    p_2;
                }
                if (*stk.offset(-(2 as libc::c_int) as isize)).nullable {
                    let ref mut fresh25 = (*stk.offset(-(2 as libc::c_int) as isize))
                        .nfirstpos;
                    *fresh25 += (*stk.offset(-(1 as libc::c_int) as isize)).nfirstpos;
                } else {
                    firstpos = firstpos
                        .offset(
                            -((*stk.offset(-(1 as libc::c_int) as isize)).nfirstpos
                                as isize),
                        );
                }
                if (*stk.offset(-(1 as libc::c_int) as isize)).nullable {
                    let ref mut fresh26 = (*stk.offset(-(2 as libc::c_int) as isize))
                        .nlastpos;
                    *fresh26 += (*stk.offset(-(1 as libc::c_int) as isize)).nlastpos;
                } else {
                    let mut p_3: *mut position = lastpos
                        .offset(
                            -((*stk.offset(-(1 as libc::c_int) as isize)).nlastpos
                                as isize),
                        )
                        .offset(
                            -((*stk.offset(-(2 as libc::c_int) as isize)).nlastpos
                                as isize),
                        );
                    let mut j: idx_t = 0 as libc::c_int as idx_t;
                    while j < (*stk.offset(-(1 as libc::c_int) as isize)).nlastpos {
                        *p_3
                            .offset(
                                j as isize,
                            ) = *p_3
                            .offset(
                                (j + (*stk.offset(-(2 as libc::c_int) as isize)).nlastpos)
                                    as isize,
                            );
                        j += 1;
                        j;
                    }
                    lastpos = lastpos
                        .offset(
                            -((*stk.offset(-(2 as libc::c_int) as isize)).nlastpos
                                as isize),
                        );
                    (*stk.offset(-(2 as libc::c_int) as isize))
                        .nlastpos = (*stk.offset(-(1 as libc::c_int) as isize)).nlastpos;
                }
                let ref mut fresh27 = (*stk.offset(-(2 as libc::c_int) as isize))
                    .nullable;
                *fresh27 = (*fresh27 as libc::c_int
                    & (*stk.offset(-(1 as libc::c_int) as isize)).nullable
                        as libc::c_int) != 0;
                stk = stk.offset(-1);
                stk;
                current_block_53 = 1724319918354933278;
            }
            262 => {
                let ref mut fresh28 = (*stk.offset(-(2 as libc::c_int) as isize))
                    .nfirstpos;
                *fresh28 += (*stk.offset(-(1 as libc::c_int) as isize)).nfirstpos;
                let ref mut fresh29 = (*stk.offset(-(2 as libc::c_int) as isize))
                    .nlastpos;
                *fresh29 += (*stk.offset(-(1 as libc::c_int) as isize)).nlastpos;
                let ref mut fresh30 = (*stk.offset(-(2 as libc::c_int) as isize))
                    .nullable;
                *fresh30 = (*fresh30 as libc::c_int
                    | (*stk.offset(-(1 as libc::c_int) as isize)).nullable
                        as libc::c_int) != 0;
                stk = stk.offset(-1);
                stk;
                current_block_53 = 1724319918354933278;
            }
            _ => {
                (*stk)
                    .nullable = *((*d).tokens).offset(i as isize)
                    == BACKREF as libc::c_int as libc::c_long;
                (*stk).nlastpos = 1 as libc::c_int as idx_t;
                (*stk).nfirstpos = (*stk).nlastpos;
                stk = stk.offset(1);
                stk;
                (*lastpos).index = i;
                (*firstpos).index = (*lastpos).index;
                (*lastpos).constraint = NO_CONSTRAINT as libc::c_int as libc::c_uint;
                (*firstpos).constraint = (*lastpos).constraint;
                firstpos = firstpos.offset(1);
                firstpos;
                lastpos = lastpos.offset(1);
                lastpos;
                current_block_53 = 1724319918354933278;
            }
        }
        match current_block_53 {
            6103263293371866127 => {
                if *((*d).tokens).offset(i as isize)
                    != PLUS as libc::c_int as libc::c_long
                {
                    (*stk.offset(-(1 as libc::c_int) as isize))
                        .nullable = 1 as libc::c_int != 0;
                }
            }
            _ => {}
        }
        i += 1;
        i;
    }
    if !backward.is_null() {
        epsclosure(d, backward);
        let mut i_0: idx_t = 0 as libc::c_int as idx_t;
        while i_0 < tindex {
            rpl_free((*backward.offset(i_0 as isize)).elems as *mut libc::c_void);
            i_0 += 1;
            i_0;
        }
        rpl_free(backward as *mut libc::c_void);
    }
    dfaoptimize(d);
    pos.index = 0 as libc::c_int as idx_t;
    pos.constraint = NO_CONSTRAINT as libc::c_int as libc::c_uint;
    alloc_position_set(&mut tmp, 1 as libc::c_int as idx_t);
    append(pos, &mut tmp);
    (*d)
        .separates = xicalloc(
        tindex,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as idx_t,
    ) as *mut libc::c_int;
    let mut i_1: idx_t = 0 as libc::c_int as idx_t;
    while i_1 < tindex {
        if prev_newline_dependent(*((*d).constraints).offset(i_1 as isize)) {
            *((*d).separates).offset(i_1 as isize) |= CTX_NEWLINE as libc::c_int;
        }
        if prev_letter_dependent(*((*d).constraints).offset(i_1 as isize)) {
            *((*d).separates).offset(i_1 as isize) |= CTX_LETTER as libc::c_int;
        }
        let mut j_0: idx_t = 0 as libc::c_int as idx_t;
        while j_0 < (*((*d).follows).offset(i_1 as isize)).nelem {
            if prev_newline_dependent(
                (*((*((*d).follows).offset(i_1 as isize)).elems).offset(j_0 as isize))
                    .constraint as libc::c_int,
            ) {
                *((*d).separates).offset(i_1 as isize) |= CTX_NEWLINE as libc::c_int;
            }
            if prev_letter_dependent(
                (*((*((*d).follows).offset(i_1 as isize)).elems).offset(j_0 as isize))
                    .constraint as libc::c_int,
            ) {
                *((*d).separates).offset(i_1 as isize) |= CTX_LETTER as libc::c_int;
            }
            j_0 += 1;
            j_0;
        }
        i_1 += 1;
        i_1;
    }
    let mut separate_contexts: libc::c_int = state_separate_contexts(d, &mut tmp);
    if separate_contexts & CTX_NEWLINE as libc::c_int != 0 {
        state_index(d, &mut tmp, CTX_NEWLINE as libc::c_int);
    }
    (*d)
        .min_trcount = state_index(
        d,
        &mut tmp,
        separate_contexts ^ CTX_ANY as libc::c_int,
    ) as libc::c_int;
    (*d).initstate_notbol = (*d).min_trcount as state_num;
    if separate_contexts & CTX_LETTER as libc::c_int != 0 {
        (*d)
            .min_trcount = state_index(d, &mut tmp, CTX_LETTER as libc::c_int)
            as libc::c_int;
    }
    (*d).min_trcount += 1;
    (*d).min_trcount;
    (*d).trcount = 0 as libc::c_int;
    rpl_free(posalloc as *mut libc::c_void);
    rpl_free(stkalloc as *mut libc::c_void);
    rpl_free(merged.elems as *mut libc::c_void);
    rpl_free(tmp.elems as *mut libc::c_void);
}
unsafe extern "C" fn realloc_trans_if_necessary(mut d: *mut dfa) {
    let mut oldalloc: state_num = (*d).tralloc;
    if oldalloc < (*d).sindex {
        let mut realtrans: *mut *mut state_num = if !((*d).trans).is_null() {
            ((*d).trans).offset(-(2 as libc::c_int as isize))
        } else {
            0 as *mut *mut state_num
        };
        let mut newalloc1: idx_t = if !realtrans.is_null() {
            (*d).tralloc + 2 as libc::c_int as libc::c_long
        } else {
            0 as libc::c_int as libc::c_long
        };
        realtrans = xpalloc(
            realtrans as *mut libc::c_void,
            &mut newalloc1,
            (*d).sindex - oldalloc,
            -(1 as libc::c_int) as ptrdiff_t,
            ::std::mem::size_of::<*mut state_num>() as libc::c_ulong as idx_t,
        ) as *mut *mut state_num;
        let ref mut fresh31 = *realtrans.offset(1 as libc::c_int as isize);
        *fresh31 = 0 as *mut state_num;
        let ref mut fresh32 = *realtrans.offset(0 as libc::c_int as isize);
        *fresh32 = *fresh31;
        (*d).trans = realtrans.offset(2 as libc::c_int as isize);
        (*d).tralloc = newalloc1 - 2 as libc::c_int as libc::c_long;
        let mut newalloc: idx_t = (*d).tralloc;
        (*d)
            .fails = xreallocarray(
            (*d).fails as *mut libc::c_void,
            newalloc as size_t,
            ::std::mem::size_of::<*mut state_num>() as libc::c_ulong,
        ) as *mut *mut state_num;
        (*d)
            .success = xreallocarray(
            (*d).success as *mut libc::c_void,
            newalloc as size_t,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
        (*d)
            .newlines = xreallocarray(
            (*d).newlines as *mut libc::c_void,
            newalloc as size_t,
            ::std::mem::size_of::<state_num>() as libc::c_ulong,
        ) as *mut state_num;
        if (*d).localeinfo.multibyte {
            realtrans = if !((*d).mb_trans).is_null() {
                ((*d).mb_trans).offset(-(2 as libc::c_int as isize))
            } else {
                0 as *mut *mut state_num
            };
            realtrans = xreallocarray(
                realtrans as *mut libc::c_void,
                newalloc1 as size_t,
                ::std::mem::size_of::<*mut state_num>() as libc::c_ulong,
            ) as *mut *mut state_num;
            if oldalloc == 0 as libc::c_int as libc::c_long {
                let ref mut fresh33 = *realtrans.offset(1 as libc::c_int as isize);
                *fresh33 = 0 as *mut state_num;
                let ref mut fresh34 = *realtrans.offset(0 as libc::c_int as isize);
                *fresh34 = *fresh33;
            }
            (*d).mb_trans = realtrans.offset(2 as libc::c_int as isize);
        }
        while oldalloc < newalloc {
            let ref mut fresh35 = *((*d).trans).offset(oldalloc as isize);
            *fresh35 = 0 as *mut state_num;
            let ref mut fresh36 = *((*d).fails).offset(oldalloc as isize);
            *fresh36 = 0 as *mut state_num;
            if (*d).localeinfo.multibyte {
                let ref mut fresh37 = *((*d).mb_trans).offset(oldalloc as isize);
                *fresh37 = 0 as *mut state_num;
            }
            oldalloc += 1;
            oldalloc;
        }
    }
}
unsafe extern "C" fn build_state(
    mut s: state_num,
    mut d: *mut dfa,
    mut uc: libc::c_uchar,
) -> state_num {
    let mut follows: position_set = position_set {
        elems: 0 as *mut position,
        nelem: 0,
        alloc: 0,
    };
    let mut group: position_set = position_set {
        elems: 0 as *mut position,
        nelem: 0,
        alloc: 0,
    };
    let mut tmp: position_set = position_set {
        elems: 0 as *mut position,
        nelem: 0,
        alloc: 0,
    };
    let mut state: state_num = 0;
    let mut state_newline: state_num = 0;
    let mut state_letter: state_num = 0;
    let mut ptrans: *mut *mut state_num = (if accepting(s, d) as libc::c_int != 0 {
        (*d).fails
    } else {
        (*d).trans
    })
        .offset(s as isize);
    let mut trans: *mut state_num = *ptrans;
    if trans.is_null() {
        if MAX_TRCOUNT as libc::c_int <= (*d).trcount {
            let mut i: state_num = (*d).min_trcount as state_num;
            while i < (*d).tralloc {
                rpl_free(*((*d).trans).offset(i as isize) as *mut libc::c_void);
                rpl_free(*((*d).fails).offset(i as isize) as *mut libc::c_void);
                let ref mut fresh38 = *((*d).fails).offset(i as isize);
                *fresh38 = 0 as *mut state_num;
                let ref mut fresh39 = *((*d).trans).offset(i as isize);
                *fresh39 = *fresh38;
                i += 1;
                i;
            }
            (*d).trcount = 0 as libc::c_int;
        }
        (*d).trcount += 1;
        (*d).trcount;
        trans = xmalloc(
            (NOTCHAR as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<state_num>() as libc::c_ulong),
        ) as *mut state_num;
        *ptrans = trans;
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < NOTCHAR as libc::c_int {
            *trans.offset(i_0 as isize) = -(2 as libc::c_int) as state_num;
            i_0 += 1;
            i_0;
        }
    }
    *((*d).success).offset(s as isize) = 0 as libc::c_int as libc::c_char;
    if accepts_in_context(
        (*((*d).states).offset(s as isize)).context as libc::c_int,
        CTX_NEWLINE as libc::c_int,
        s,
        d,
    ) {
        let ref mut fresh40 = *((*d).success).offset(s as isize);
        *fresh40 = (*fresh40 as libc::c_int | CTX_NEWLINE as libc::c_int)
            as libc::c_char;
    }
    if accepts_in_context(
        (*((*d).states).offset(s as isize)).context as libc::c_int,
        CTX_LETTER as libc::c_int,
        s,
        d,
    ) {
        let ref mut fresh41 = *((*d).success).offset(s as isize);
        *fresh41 = (*fresh41 as libc::c_int | CTX_LETTER as libc::c_int) as libc::c_char;
    }
    if accepts_in_context(
        (*((*d).states).offset(s as isize)).context as libc::c_int,
        CTX_NONE as libc::c_int,
        s,
        d,
    ) {
        let ref mut fresh42 = *((*d).success).offset(s as isize);
        *fresh42 = (*fresh42 as libc::c_int | CTX_NONE as libc::c_int) as libc::c_char;
    }
    alloc_position_set(&mut follows, (*d).nleaves);
    let mut j: idx_t = 0 as libc::c_int as idx_t;
    while j < (*((*d).states).offset(s as isize)).elems.nelem {
        let mut k: idx_t = 0 as libc::c_int as idx_t;
        while k
            < (*((*d).follows)
                .offset(
                    (*((*((*d).states).offset(s as isize)).elems.elems)
                        .offset(j as isize))
                        .index as isize,
                ))
                .nelem
        {
            insert(
                *((*((*d).follows)
                    .offset(
                        (*((*((*d).states).offset(s as isize)).elems.elems)
                            .offset(j as isize))
                            .index as isize,
                    ))
                    .elems)
                    .offset(k as isize),
                &mut follows,
            );
            k += 1;
            k;
        }
        j += 1;
        j;
    }
    alloc_position_set(&mut group, (*d).nleaves);
    let mut label: charclass = charclass { w: [0; 4] };
    fillset(&mut label);
    let mut current_block_63: u64;
    let mut i_1: idx_t = 0 as libc::c_int as idx_t;
    while i_1 < follows.nelem {
        let mut matches: charclass = charclass { w: [0; 4] };
        let mut pos: position = *(follows.elems).offset(i_1 as isize);
        let mut matched: bool = 0 as libc::c_int != 0;
        if *((*d).tokens).offset(pos.index as isize) >= 0 as libc::c_int as libc::c_long
            && *((*d).tokens).offset(pos.index as isize)
                < NOTCHAR as libc::c_int as libc::c_long
        {
            zeroset(&mut matches);
            setbit(
                *((*d).tokens).offset(pos.index as isize) as libc::c_uint,
                &mut matches,
            );
            if *((*d).tokens).offset(pos.index as isize) == uc as libc::c_long {
                matched = 1 as libc::c_int != 0;
            }
            current_block_63 = 14775119014532381840;
        } else if *((*d).tokens).offset(pos.index as isize)
            >= CSET as libc::c_int as libc::c_long
        {
            matches = *((*d).charclasses)
                .offset(
                    (*((*d).tokens).offset(pos.index as isize)
                        - CSET as libc::c_int as libc::c_long) as isize,
                );
            if tstbit(uc as libc::c_uint, &mut matches) {
                matched = 1 as libc::c_int != 0;
            }
            current_block_63 = 14775119014532381840;
        } else if *((*d).tokens).offset(pos.index as isize)
            == ANYCHAR as libc::c_int as libc::c_long
        {
            matches = *((*d).charclasses).offset((*d).canychar as isize);
            if tstbit(uc as libc::c_uint, &mut matches) {
                matched = 1 as libc::c_int != 0;
            }
            if succeeds_in_context(
                pos.constraint as libc::c_int,
                (*((*d).states).offset(s as isize)).context as libc::c_int,
                CTX_NONE as libc::c_int,
            ) {
                if (*((*d).states).offset(s as isize)).mbps.nelem
                    == 0 as libc::c_int as libc::c_long
                {
                    alloc_position_set(
                        &mut (*((*d).states).offset(s as isize)).mbps,
                        1 as libc::c_int as idx_t,
                    );
                }
                insert(pos, &mut (*((*d).states).offset(s as isize)).mbps);
            }
            current_block_63 = 14775119014532381840;
        } else {
            current_block_63 = 1608152415753874203;
        }
        match current_block_63 {
            14775119014532381840 => {
                if pos.constraint != NO_CONSTRAINT as libc::c_int as libc::c_uint {
                    if !succeeds_in_context(
                        pos.constraint as libc::c_int,
                        (*((*d).states).offset(s as isize)).context as libc::c_int,
                        CTX_NEWLINE as libc::c_int,
                    ) {
                        let mut j_0: libc::c_int = 0 as libc::c_int;
                        while j_0 < CHARCLASS_WORDS as libc::c_int {
                            matches.w[j_0 as usize]
                                &= !(*d).syntax.newline.w[j_0 as usize];
                            j_0 += 1;
                            j_0;
                        }
                    }
                    if !succeeds_in_context(
                        pos.constraint as libc::c_int,
                        (*((*d).states).offset(s as isize)).context as libc::c_int,
                        CTX_LETTER as libc::c_int,
                    ) {
                        let mut j_1: libc::c_int = 0 as libc::c_int;
                        while j_1 < CHARCLASS_WORDS as libc::c_int {
                            matches.w[j_1 as usize]
                                &= !(*d).syntax.letters.w[j_1 as usize];
                            j_1 += 1;
                            j_1;
                        }
                    }
                    if !succeeds_in_context(
                        pos.constraint as libc::c_int,
                        (*((*d).states).offset(s as isize)).context as libc::c_int,
                        CTX_NONE as libc::c_int,
                    ) {
                        let mut j_2: libc::c_int = 0 as libc::c_int;
                        while j_2 < CHARCLASS_WORDS as libc::c_int {
                            matches.w[j_2 as usize]
                                &= (*d).syntax.letters.w[j_2 as usize]
                                    | (*d).syntax.newline.w[j_2 as usize];
                            j_2 += 1;
                            j_2;
                        }
                    }
                    if emptyset(&mut matches) {
                        current_block_63 = 1608152415753874203;
                    } else {
                        if !tstbit(uc as libc::c_uint, &mut matches) {
                            matched = 0 as libc::c_int != 0;
                        }
                        current_block_63 = 3392087639489470149;
                    }
                } else {
                    current_block_63 = 3392087639489470149;
                }
                match current_block_63 {
                    1608152415753874203 => {}
                    _ => {
                        if matched {
                            let mut k_0: libc::c_int = 0 as libc::c_int;
                            while k_0 < CHARCLASS_WORDS as libc::c_int {
                                label.w[k_0 as usize] &= matches.w[k_0 as usize];
                                k_0 += 1;
                                k_0;
                            }
                            append(pos, &mut group);
                        } else {
                            let mut k_1: libc::c_int = 0 as libc::c_int;
                            while k_1 < CHARCLASS_WORDS as libc::c_int {
                                label.w[k_1 as usize] &= !matches.w[k_1 as usize];
                                k_1 += 1;
                                k_1;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        i_1 += 1;
        i_1;
    }
    alloc_position_set(&mut tmp, (*d).nleaves);
    if group.nelem > 0 as libc::c_int as libc::c_long {
        if (*d).searchflag {
            let mut mergeit: bool = !(*d).localeinfo.multibyte;
            if !mergeit {
                mergeit = 1 as libc::c_int != 0;
                let mut j_3: idx_t = 0 as libc::c_int as idx_t;
                while mergeit as libc::c_int != 0 && j_3 < group.nelem {
                    mergeit = (mergeit as libc::c_int
                        & *((*d).multibyte_prop)
                            .offset((*(group.elems).offset(j_3 as isize)).index as isize)
                            as libc::c_int) != 0;
                    j_3 += 1;
                    j_3;
                }
            }
            if mergeit {
                merge2(
                    &mut group,
                    &mut (*((*d).states).offset(0 as libc::c_int as isize)).elems,
                    &mut tmp,
                );
            }
        }
        let mut possible_contexts: libc::c_int = charclass_context(d, &mut label);
        let mut separate_contexts: libc::c_int = state_separate_contexts(d, &mut group);
        if possible_contexts & !separate_contexts != 0 {
            state = state_index(
                d,
                &mut group,
                separate_contexts ^ CTX_ANY as libc::c_int,
            );
        } else {
            state = -(1 as libc::c_int) as state_num;
        }
        if separate_contexts & possible_contexts & CTX_NEWLINE as libc::c_int != 0 {
            state_newline = state_index(d, &mut group, CTX_NEWLINE as libc::c_int);
        } else {
            state_newline = state;
        }
        if separate_contexts & possible_contexts & CTX_LETTER as libc::c_int != 0 {
            state_letter = state_index(d, &mut group, CTX_LETTER as libc::c_int);
        } else {
            state_letter = state;
        }
        realloc_trans_if_necessary(d);
    } else if (*d).searchflag {
        state_newline = 0 as libc::c_int as state_num;
        state_letter = ((*d).min_trcount - 1 as libc::c_int) as state_num;
        state = (*d).initstate_notbol;
    } else {
        state_newline = -(1 as libc::c_int) as state_num;
        state_letter = -(1 as libc::c_int) as state_num;
        state = -(1 as libc::c_int) as state_num;
    }
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < NOTCHAR as libc::c_int {
        if tstbit(i_2 as libc::c_uint, &mut label) {
            match (*d).syntax.sbit[i_2 as usize] as libc::c_int {
                4 => {
                    *trans.offset(i_2 as isize) = state_newline;
                }
                2 => {
                    *trans.offset(i_2 as isize) = state_letter;
                }
                _ => {
                    *trans.offset(i_2 as isize) = state;
                }
            }
        }
        i_2 += 1;
        i_2;
    }
    rpl_free(group.elems as *mut libc::c_void);
    rpl_free(follows.elems as *mut libc::c_void);
    rpl_free(tmp.elems as *mut libc::c_void);
    if tstbit((*d).syntax.eolbyte as libc::c_uint, &mut label) {
        *((*d).newlines)
            .offset(s as isize) = *trans.offset((*d).syntax.eolbyte as isize);
        *trans.offset((*d).syntax.eolbyte as isize) = -(1 as libc::c_int) as state_num;
    }
    return *trans.offset(uc as isize);
}
unsafe extern "C" fn transit_state_singlebyte(
    mut d: *mut dfa,
    mut s: state_num,
    mut pp: *mut *const libc::c_uchar,
) -> state_num {
    let mut t: *mut state_num = 0 as *mut state_num;
    if !(*((*d).trans).offset(s as isize)).is_null() {
        t = *((*d).trans).offset(s as isize);
    } else if !(*((*d).fails).offset(s as isize)).is_null() {
        t = *((*d).fails).offset(s as isize);
    } else {
        build_state(s, d, **pp);
        if !(*((*d).trans).offset(s as isize)).is_null() {
            t = *((*d).trans).offset(s as isize);
        } else {
            t = *((*d).fails).offset(s as isize);
            if !t.is_null() {} else {
                __assert_fail(
                    b"t\0" as *const u8 as *const libc::c_char,
                    b"dfa.c\0" as *const u8 as *const libc::c_char,
                    3286 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 84],
                        &[libc::c_char; 84],
                    >(
                        b"state_num transit_state_singlebyte(struct dfa *, state_num, const unsigned char **)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_6267: {
                if !t.is_null() {} else {
                    __assert_fail(
                        b"t\0" as *const u8 as *const libc::c_char,
                        b"dfa.c\0" as *const u8 as *const libc::c_char,
                        3286 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 84],
                            &[libc::c_char; 84],
                        >(
                            b"state_num transit_state_singlebyte(struct dfa *, state_num, const unsigned char **)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
        }
    }
    if *t.offset(**pp as isize) == -(2 as libc::c_int) as libc::c_long {
        build_state(s, d, **pp);
    }
    let fresh43 = *pp;
    *pp = (*pp).offset(1);
    return *t.offset(*fresh43 as isize);
}
unsafe extern "C" fn transit_state(
    mut d: *mut dfa,
    mut s: state_num,
    mut pp: *mut *const libc::c_uchar,
    mut end: *const libc::c_uchar,
) -> state_num {
    let mut wc: wint_t = 0;
    let mut mbclen: libc::c_int = mbs_to_wchar(
        &mut wc,
        *pp as *const libc::c_char,
        end.offset_from(*pp) as libc::c_long,
        d,
    );
    (*d).mb_follows.nelem = 0 as libc::c_int as idx_t;
    let mut s1: state_num = s;
    let mut mbci: libc::c_int = 0;
    mbci = 0 as libc::c_int;
    while mbci < mbclen
        && (mbci == 0 as libc::c_int || (*d).min_trcount as libc::c_long <= s)
    {
        s = transit_state_singlebyte(d, s, pp);
        mbci += 1;
        mbci;
    }
    *pp = (*pp).offset((mbclen - mbci) as isize);
    if wc == 0xffffffff as libc::c_uint {
        return s;
    }
    if (*((*d).states).offset(s1 as isize)).mb_trindex < 0 as libc::c_int as libc::c_long
    {
        if MAX_TRCOUNT as libc::c_int as libc::c_long <= (*d).mb_trcount {
            let mut s3: state_num = 0;
            s3 = -(1 as libc::c_int) as state_num;
            while s3 < (*d).tralloc {
                rpl_free(*((*d).mb_trans).offset(s3 as isize) as *mut libc::c_void);
                let ref mut fresh44 = *((*d).mb_trans).offset(s3 as isize);
                *fresh44 = 0 as *mut state_num;
                s3 += 1;
                s3;
            }
            let mut i: state_num = 0 as libc::c_int as state_num;
            while i < (*d).sindex {
                (*((*d).states).offset(i as isize))
                    .mb_trindex = -(1 as libc::c_int) as state_num;
                i += 1;
                i;
            }
            (*d).mb_trcount = 0 as libc::c_int as state_num;
        }
        let fresh45 = (*d).mb_trcount;
        (*d).mb_trcount = (*d).mb_trcount + 1;
        (*((*d).states).offset(s1 as isize)).mb_trindex = fresh45;
    }
    if (*((*d).mb_trans).offset(s as isize)).is_null() {
        let ref mut fresh46 = *((*d).mb_trans).offset(s as isize);
        *fresh46 = xmalloc(TRANSALLOC_SIZE as libc::c_int as size_t) as *mut state_num;
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < MAX_TRCOUNT as libc::c_int {
            *(*((*d).mb_trans).offset(s as isize))
                .offset(i_0 as isize) = -(1 as libc::c_int) as state_num;
            i_0 += 1;
            i_0;
        }
    } else if *(*((*d).mb_trans).offset(s as isize))
        .offset((*((*d).states).offset(s1 as isize)).mb_trindex as isize)
        >= 0 as libc::c_int as libc::c_long
    {
        return *(*((*d).mb_trans).offset(s as isize))
            .offset((*((*d).states).offset(s1 as isize)).mb_trindex as isize)
    }
    if s == -(1 as libc::c_int) as libc::c_long {
        copy(&mut (*((*d).states).offset(s1 as isize)).mbps, &mut (*d).mb_follows);
    } else {
        merge(
            &mut (*((*d).states).offset(s1 as isize)).mbps,
            &mut (*((*d).states).offset(s as isize)).elems,
            &mut (*d).mb_follows,
        );
    }
    let mut separate_contexts: libc::c_int = state_separate_contexts(
        d,
        &mut (*d).mb_follows,
    );
    let mut s2: state_num = state_index(
        d,
        &mut (*d).mb_follows,
        separate_contexts ^ CTX_ANY as libc::c_int,
    );
    realloc_trans_if_necessary(d);
    *(*((*d).mb_trans).offset(s as isize))
        .offset((*((*d).states).offset(s1 as isize)).mb_trindex as isize) = s2;
    return s2;
}
unsafe extern "C" fn skip_remains_mb(
    mut d: *mut dfa,
    mut p: *const libc::c_uchar,
    mut mbp: *const libc::c_uchar,
    mut end: *const libc::c_char,
) -> *const libc::c_uchar {
    if (*d).syntax.never_trail[*p as usize] {
        return p;
    }
    while mbp < p {
        let mut wc: wint_t = 0;
        mbp = mbp
            .offset(
                mbs_to_wchar(
                    &mut wc,
                    mbp as *const libc::c_char,
                    end.offset_from(mbp as *const libc::c_char) as libc::c_long,
                    d,
                ) as isize,
            );
    }
    return mbp;
}
#[inline]
unsafe extern "C" fn dfaexec_main(
    mut d: *mut dfa,
    mut begin: *const libc::c_char,
    mut end: *mut libc::c_char,
    mut allow_nl: bool,
    mut count: *mut idx_t,
    mut multibyte: bool,
) -> *mut libc::c_char {
    if MAX_TRCOUNT as libc::c_int as libc::c_long <= (*d).sindex {
        let mut s: state_num = (*d).min_trcount as state_num;
        while s < (*d).sindex {
            rpl_free(
                (*((*d).states).offset(s as isize)).elems.elems as *mut libc::c_void,
            );
            rpl_free(
                (*((*d).states).offset(s as isize)).mbps.elems as *mut libc::c_void,
            );
            s += 1;
            s;
        }
        (*d).sindex = (*d).min_trcount as state_num;
        if !((*d).trans).is_null() {
            let mut s_0: state_num = 0 as libc::c_int as state_num;
            while s_0 < (*d).tralloc {
                rpl_free(*((*d).trans).offset(s_0 as isize) as *mut libc::c_void);
                rpl_free(*((*d).fails).offset(s_0 as isize) as *mut libc::c_void);
                let ref mut fresh47 = *((*d).fails).offset(s_0 as isize);
                *fresh47 = 0 as *mut state_num;
                let ref mut fresh48 = *((*d).trans).offset(s_0 as isize);
                *fresh48 = *fresh47;
                s_0 += 1;
                s_0;
            }
            (*d).trcount = 0 as libc::c_int;
        }
        if (*d).localeinfo.multibyte as libc::c_int != 0 && !((*d).mb_trans).is_null() {
            let mut s_1: state_num = -(1 as libc::c_int) as state_num;
            while s_1 < (*d).tralloc {
                rpl_free(*((*d).mb_trans).offset(s_1 as isize) as *mut libc::c_void);
                let ref mut fresh49 = *((*d).mb_trans).offset(s_1 as isize);
                *fresh49 = 0 as *mut state_num;
                s_1 += 1;
                s_1;
            }
            let mut s_2: state_num = 0 as libc::c_int as state_num;
            while s_2 < (*d).min_trcount as libc::c_long {
                (*((*d).states).offset(s_2 as isize))
                    .mb_trindex = -(1 as libc::c_int) as state_num;
                s_2 += 1;
                s_2;
            }
            (*d).mb_trcount = 0 as libc::c_int as state_num;
        }
    }
    if (*d).tralloc == 0 {
        realloc_trans_if_necessary(d);
    }
    let mut s_3: state_num = 0 as libc::c_int as state_num;
    let mut s1: state_num = 0 as libc::c_int as state_num;
    let mut p: *const libc::c_uchar = begin as *const libc::c_uchar;
    let mut mbp: *const libc::c_uchar = p;
    let mut trans: *mut *mut state_num = (*d).trans;
    let mut eol: libc::c_uchar = (*d).syntax.eolbyte;
    let mut saved_end: libc::c_uchar = *(end as *mut libc::c_uchar);
    *end = eol as libc::c_char;
    if multibyte {
        memset(
            &mut (*d).mbs as *mut mbstate_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
        if (*d).mb_follows.alloc == 0 as libc::c_int as libc::c_long {
            alloc_position_set(&mut (*d).mb_follows, (*d).nleaves);
        }
    }
    let mut nlcount: idx_t = 0 as libc::c_int as idx_t;
    loop {
        let mut t: *mut state_num = 0 as *mut state_num;
        loop {
            t = *trans.offset(s_3 as isize);
            if t.is_null() {
                break;
            }
            if s_3 < (*d).min_trcount as libc::c_long {
                if !multibyte
                    || (*((*d).states).offset(s_3 as isize)).mbps.nelem
                        == 0 as libc::c_int as libc::c_long
                {
                    while *t.offset(*p as isize) == s_3 {
                        p = p.offset(1);
                        p;
                    }
                }
                if multibyte {
                    mbp = skip_remains_mb(d, p, mbp, end);
                    p = mbp;
                }
            }
            if multibyte {
                s1 = s_3;
                if (*((*d).states).offset(s_3 as isize)).mbps.nelem
                    == 0 as libc::c_int as libc::c_long
                    || (*d).localeinfo.sbctowc[*p as usize] != 0xffffffff as libc::c_uint
                    || p as *mut libc::c_char >= end
                {
                    let fresh50 = p;
                    p = p.offset(1);
                    s_3 = *t.offset(*fresh50 as isize);
                } else {
                    s_3 = transit_state(d, s_3, &mut p, end as *mut libc::c_uchar);
                    mbp = p;
                    trans = (*d).trans;
                }
            } else {
                let fresh51 = p;
                p = p.offset(1);
                s1 = *t.offset(*fresh51 as isize);
                t = *trans.offset(s1 as isize);
                if t.is_null() {
                    let mut tmp: state_num = s_3;
                    s_3 = s1;
                    s1 = tmp;
                    break;
                } else {
                    if s_3 < (*d).min_trcount as libc::c_long {
                        while *t.offset(*p as isize) == s1 {
                            p = p.offset(1);
                            p;
                        }
                    }
                    let fresh52 = p;
                    p = p.offset(1);
                    s_3 = *t.offset(*fresh52 as isize);
                }
            }
        }
        if s_3 < 0 as libc::c_int as libc::c_long {
            if s_3 == -(2 as libc::c_int) as libc::c_long {
                s_3 = build_state(s1, d, *p.offset(-(1 as libc::c_int) as isize));
                trans = (*d).trans;
            } else if p as *mut libc::c_char <= end
                && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == eol as libc::c_int
                && 0 as libc::c_int as libc::c_long
                    <= *((*d).newlines).offset(s1 as isize)
            {
                nlcount += 1;
                nlcount;
                mbp = p;
                s_3 = if allow_nl as libc::c_int != 0 {
                    *((*d).newlines).offset(s1 as isize)
                } else if (*d).syntax.sbit[eol as usize] as libc::c_int
                    == CTX_NEWLINE as libc::c_int
                {
                    0 as libc::c_int as libc::c_long
                } else if (*d).syntax.sbit[eol as usize] as libc::c_int
                    == CTX_LETTER as libc::c_int
                {
                    ((*d).min_trcount - 1 as libc::c_int) as libc::c_long
                } else {
                    (*d).initstate_notbol
                };
            } else {
                p = 0 as *const libc::c_uchar;
                break;
            }
        } else if !(*((*d).fails).offset(s_3 as isize)).is_null() {
            if *((*d).success).offset(s_3 as isize) as libc::c_int
                & (*d).syntax.sbit[*p as usize] as libc::c_int != 0
                || p as *mut libc::c_char == end
                    && accepts_in_context(
                        (*((*d).states).offset(s_3 as isize)).context as libc::c_int,
                        CTX_NEWLINE as libc::c_int,
                        s_3,
                        d,
                    ) as libc::c_int != 0
            {
                break;
            }
            if multibyte as libc::c_int != 0 && s_3 < (*d).min_trcount as libc::c_long {
                mbp = skip_remains_mb(d, p, mbp, end);
                p = mbp;
            }
            s1 = s_3;
            if !multibyte
                || (*((*d).states).offset(s_3 as isize)).mbps.nelem
                    == 0 as libc::c_int as libc::c_long
                || (*d).localeinfo.sbctowc[*p as usize] != 0xffffffff as libc::c_uint
                || p as *mut libc::c_char >= end
            {
                let fresh53 = p;
                p = p.offset(1);
                s_3 = *(*((*d).fails).offset(s_3 as isize)).offset(*fresh53 as isize);
            } else {
                s_3 = transit_state(d, s_3, &mut p, end as *mut libc::c_uchar);
                mbp = p;
                trans = (*d).trans;
            }
        } else {
            build_state(s_3, d, *p.offset(0 as libc::c_int as isize));
            trans = (*d).trans;
        }
    }
    if !count.is_null() {
        *count += nlcount;
    }
    *end = saved_end as libc::c_char;
    return p as *mut libc::c_char;
}
unsafe extern "C" fn dfaexec_mb(
    mut d: *mut dfa,
    mut begin: *const libc::c_char,
    mut end: *mut libc::c_char,
    mut allow_nl: bool,
    mut count: *mut idx_t,
    mut backref: *mut bool,
) -> *mut libc::c_char {
    return dfaexec_main(d, begin, end, allow_nl, count, 1 as libc::c_int != 0);
}
unsafe extern "C" fn dfaexec_sb(
    mut d: *mut dfa,
    mut begin: *const libc::c_char,
    mut end: *mut libc::c_char,
    mut allow_nl: bool,
    mut count: *mut idx_t,
    mut backref: *mut bool,
) -> *mut libc::c_char {
    return dfaexec_main(d, begin, end, allow_nl, count, 0 as libc::c_int != 0);
}
unsafe extern "C" fn dfaexec_noop(
    mut d: *mut dfa,
    mut begin: *const libc::c_char,
    mut end: *mut libc::c_char,
    mut allow_nl: bool,
    mut count: *mut idx_t,
    mut backref: *mut bool,
) -> *mut libc::c_char {
    *backref = 1 as libc::c_int != 0;
    return begin as *mut libc::c_char;
}
pub unsafe extern "C" fn dfaexec(
    mut d: *mut dfa,
    mut begin: *const libc::c_char,
    mut end: *mut libc::c_char,
    mut allow_nl: bool,
    mut count: *mut idx_t,
    mut backref: *mut bool,
) -> *mut libc::c_char {
    return ((*d).dfaexec).unwrap()(d, begin, end, allow_nl, count, backref);
}
pub unsafe extern "C" fn dfasuperset(mut d: *const dfa) -> *mut dfa {
    return (*d).superset;
}
pub unsafe extern "C" fn dfaisfast(mut d: *const dfa) -> bool {
    return (*d).fast;
}
unsafe extern "C" fn free_mbdata(mut d: *mut dfa) {
    rpl_free((*d).multibyte_prop as *mut libc::c_void);
    rpl_free((*d).lex.brack.chars as *mut libc::c_void);
    rpl_free((*d).mb_follows.elems as *mut libc::c_void);
    if !((*d).mb_trans).is_null() {
        let mut s: state_num = 0;
        s = -(1 as libc::c_int) as state_num;
        while s < (*d).tralloc {
            rpl_free(*((*d).mb_trans).offset(s as isize) as *mut libc::c_void);
            s += 1;
            s;
        }
        rpl_free(
            ((*d).mb_trans).offset(-(2 as libc::c_int as isize)) as *mut libc::c_void,
        );
    }
}
pub unsafe extern "C" fn dfasupported(mut d: *const dfa) -> bool {
    let mut current_block_1: u64;
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    while i < (*d).tindex {
        match *((*d).tokens).offset(i as isize) {
            270 | 271 | 272 | 273 => {
                if !(*d).localeinfo.multibyte {
                    current_block_1 = 792017965103506125;
                } else {
                    current_block_1 = 696343685305885120;
                }
            }
            274 | 275 => {
                current_block_1 = 696343685305885120;
            }
            _ => {
                current_block_1 = 792017965103506125;
            }
        }
        match current_block_1 {
            792017965103506125 => {
                i += 1;
                i;
            }
            _ => return 0 as libc::c_int != 0,
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn maybe_disable_superset_dfa(mut d: *mut dfa) {
    if !(*d).localeinfo.using_utf8 {
        return;
    }
    let mut have_backref: bool = 0 as libc::c_int != 0;
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    while i < (*d).tindex {
        match *((*d).tokens).offset(i as isize) {
            266 => {
                abort();
            }
            274 => {
                have_backref = 1 as libc::c_int != 0;
            }
            275 => return,
            _ => {}
        }
        i += 1;
        i;
    }
    if !have_backref && !((*d).superset).is_null() {
        dfafree((*d).superset);
        rpl_free((*d).superset as *mut libc::c_void);
        (*d).superset = 0 as *mut dfa;
    }
    free_mbdata(d);
    (*d).localeinfo.multibyte = 0 as libc::c_int != 0;
    (*d)
        .dfaexec = Some(
        dfaexec_sb
            as unsafe extern "C" fn(
                *mut dfa,
                *const libc::c_char,
                *mut libc::c_char,
                bool,
                *mut idx_t,
                *mut bool,
            ) -> *mut libc::c_char,
    );
    (*d).fast = 1 as libc::c_int != 0;
}
unsafe extern "C" fn dfassbuild(mut d: *mut dfa) {
    let mut sup: *mut dfa = dfaalloc();
    *sup = *d;
    (*sup).localeinfo.multibyte = 0 as libc::c_int != 0;
    (*sup)
        .dfaexec = Some(
        dfaexec_sb
            as unsafe extern "C" fn(
                *mut dfa,
                *const libc::c_char,
                *mut libc::c_char,
                bool,
                *mut idx_t,
                *mut bool,
            ) -> *mut libc::c_char,
    );
    (*sup).multibyte_prop = 0 as *mut libc::c_char;
    (*sup).superset = 0 as *mut dfa;
    (*sup).states = 0 as *mut dfa_state;
    (*sup).sindex = 0 as libc::c_int as state_num;
    (*sup).constraints = 0 as *mut libc::c_int;
    (*sup).separates = 0 as *mut libc::c_int;
    (*sup).follows = 0 as *mut position_set;
    (*sup).tralloc = 0 as libc::c_int as state_num;
    (*sup).trans = 0 as *mut *mut state_num;
    (*sup).fails = 0 as *mut *mut state_num;
    (*sup).success = 0 as *mut libc::c_char;
    (*sup).newlines = 0 as *mut state_num;
    (*sup)
        .charclasses = xnmalloc(
        (*sup).calloc as size_t,
        ::std::mem::size_of::<charclass>() as libc::c_ulong,
    ) as *mut charclass;
    if (*d).cindex != 0 {
        memcpy(
            (*sup).charclasses as *mut libc::c_void,
            (*d).charclasses as *const libc::c_void,
            ((*d).cindex as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<charclass>() as libc::c_ulong),
        );
    }
    (*sup)
        .tokens = xnmalloc(
        (*d).tindex as size_t,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<token>() as libc::c_ulong),
    ) as *mut token;
    (*sup).talloc = (*d).tindex * 2 as libc::c_int as libc::c_long;
    let mut have_achar: bool = 0 as libc::c_int != 0;
    let mut have_nchar: bool = 0 as libc::c_int != 0;
    let mut j: idx_t = 0;
    j = 0 as libc::c_int as idx_t;
    let mut i: idx_t = j;
    while i < (*d).tindex {
        let mut current_block_32: u64;
        match *((*d).tokens).offset(i as isize) {
            266 | 275 | 274 => {
                let mut ccl: charclass = charclass { w: [0; 4] };
                fillset(&mut ccl);
                let fresh54 = j;
                j = j + 1;
                *((*sup).tokens)
                    .offset(
                        fresh54 as isize,
                    ) = CSET as libc::c_int as libc::c_long
                    + charclass_index(sup, &mut ccl);
                let fresh55 = j;
                j = j + 1;
                *((*sup).tokens).offset(fresh55 as isize) = STAR as libc::c_int as token;
                if *((*d).tokens).offset((i + 1 as libc::c_int as libc::c_long) as isize)
                    == QMARK as libc::c_int as libc::c_long
                    || *((*d).tokens)
                        .offset((i + 1 as libc::c_int as libc::c_long) as isize)
                        == STAR as libc::c_int as libc::c_long
                    || *((*d).tokens)
                        .offset((i + 1 as libc::c_int as libc::c_long) as isize)
                        == PLUS as libc::c_int as libc::c_long
                {
                    i += 1;
                    i;
                }
                have_achar = 1 as libc::c_int != 0;
                current_block_32 = 11636175345244025579;
            }
            270 | 271 | 272 | 273 => {
                if (*d).localeinfo.multibyte {
                    let fresh56 = j;
                    j = j + 1;
                    *((*sup).tokens)
                        .offset(fresh56 as isize) = EMPTY as libc::c_int as token;
                    current_block_32 = 11636175345244025579;
                } else {
                    current_block_32 = 8451627392259642034;
                }
            }
            _ => {
                current_block_32 = 8451627392259642034;
            }
        }
        match current_block_32 {
            8451627392259642034 => {
                let fresh57 = j;
                j = j + 1;
                *((*sup).tokens)
                    .offset(fresh57 as isize) = *((*d).tokens).offset(i as isize);
                if 0 as libc::c_int as libc::c_long <= *((*d).tokens).offset(i as isize)
                    && *((*d).tokens).offset(i as isize)
                        < NOTCHAR as libc::c_int as libc::c_long
                    || *((*d).tokens).offset(i as isize)
                        >= CSET as libc::c_int as libc::c_long
                {
                    have_nchar = 1 as libc::c_int != 0;
                }
            }
            _ => {}
        }
        i += 1;
        i;
    }
    (*sup).tindex = j;
    if have_nchar as libc::c_int != 0
        && (have_achar as libc::c_int != 0
            || (*d).localeinfo.multibyte as libc::c_int != 0)
    {
        (*d).superset = sup;
    } else {
        dfafree(sup);
        rpl_free(sup as *mut libc::c_void);
    };
}
pub unsafe extern "C" fn dfacomp(
    mut s: *const libc::c_char,
    mut len: idx_t,
    mut d: *mut dfa,
    mut searchflag: bool,
) {
    if !s.is_null() {
        dfaparse(s, len, d);
    }
    dfassbuild(d);
    if dfasupported(d) {
        maybe_disable_superset_dfa(d);
        dfaanalyze(d, searchflag);
    } else {
        (*d)
            .dfaexec = Some(
            dfaexec_noop
                as unsafe extern "C" fn(
                    *mut dfa,
                    *const libc::c_char,
                    *mut libc::c_char,
                    bool,
                    *mut idx_t,
                    *mut bool,
                ) -> *mut libc::c_char,
        );
    }
    if !((*d).superset).is_null() {
        (*d).fast = 1 as libc::c_int != 0;
        dfaanalyze((*d).superset, searchflag);
    }
}
pub unsafe extern "C" fn dfafree(mut d: *mut dfa) {
    rpl_free((*d).charclasses as *mut libc::c_void);
    rpl_free((*d).tokens as *mut libc::c_void);
    if (*d).localeinfo.multibyte {
        free_mbdata(d);
    }
    rpl_free((*d).constraints as *mut libc::c_void);
    rpl_free((*d).separates as *mut libc::c_void);
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    while i < (*d).sindex {
        rpl_free((*((*d).states).offset(i as isize)).elems.elems as *mut libc::c_void);
        rpl_free((*((*d).states).offset(i as isize)).mbps.elems as *mut libc::c_void);
        i += 1;
        i;
    }
    rpl_free((*d).states as *mut libc::c_void);
    if !((*d).follows).is_null() {
        let mut i_0: idx_t = 0 as libc::c_int as idx_t;
        while i_0 < (*d).tindex {
            rpl_free((*((*d).follows).offset(i_0 as isize)).elems as *mut libc::c_void);
            i_0 += 1;
            i_0;
        }
        rpl_free((*d).follows as *mut libc::c_void);
    }
    if !((*d).trans).is_null() {
        let mut i_1: idx_t = 0 as libc::c_int as idx_t;
        while i_1 < (*d).tralloc {
            rpl_free(*((*d).trans).offset(i_1 as isize) as *mut libc::c_void);
            rpl_free(*((*d).fails).offset(i_1 as isize) as *mut libc::c_void);
            i_1 += 1;
            i_1;
        }
        rpl_free(((*d).trans).offset(-(2 as libc::c_int as isize)) as *mut libc::c_void);
        rpl_free((*d).fails as *mut libc::c_void);
        rpl_free((*d).newlines as *mut libc::c_void);
        rpl_free((*d).success as *mut libc::c_void);
    }
    if !((*d).superset).is_null() {
        dfafree((*d).superset);
        rpl_free((*d).superset as *mut libc::c_void);
    }
}
unsafe extern "C" fn icatalloc(
    mut old: *mut libc::c_char,
    mut new: *const libc::c_char,
) -> *mut libc::c_char {
    let mut newsize: idx_t = strlen(new) as idx_t;
    if newsize == 0 as libc::c_int as libc::c_long {
        return old;
    }
    let mut oldsize: idx_t = strlen(old) as idx_t;
    let mut result: *mut libc::c_char = xirealloc(
        old as *mut libc::c_void,
        oldsize + newsize + 1 as libc::c_int as libc::c_long,
    ) as *mut libc::c_char;
    memcpy(
        result.offset(oldsize as isize) as *mut libc::c_void,
        new as *const libc::c_void,
        (newsize + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
    );
    return result;
}
unsafe extern "C" fn freelist(mut cpp: *mut *mut libc::c_char) {
    while !(*cpp).is_null() {
        let fresh58 = cpp;
        cpp = cpp.offset(1);
        rpl_free(*fresh58 as *mut libc::c_void);
    }
}
unsafe extern "C" fn enlistnew(
    mut cpp: *mut *mut libc::c_char,
    mut new: *mut libc::c_char,
) -> *mut *mut libc::c_char {
    let mut i: idx_t = 0;
    i = 0 as libc::c_int as idx_t;
    while !(*cpp.offset(i as isize)).is_null() {
        if !(strstr(*cpp.offset(i as isize), new)).is_null() {
            rpl_free(new as *mut libc::c_void);
            return cpp;
        }
        i += 1;
        i;
    }
    let mut j: idx_t = 0 as libc::c_int as idx_t;
    while !(*cpp.offset(j as isize)).is_null() {
        if (strstr(new, *cpp.offset(j as isize))).is_null() {
            j += 1;
            j;
        } else {
            rpl_free(*cpp.offset(j as isize) as *mut libc::c_void);
            i -= 1;
            if i == j {
                break;
            }
            let ref mut fresh59 = *cpp.offset(j as isize);
            *fresh59 = *cpp.offset(i as isize);
            let ref mut fresh60 = *cpp.offset(i as isize);
            *fresh60 = 0 as *mut libc::c_char;
        }
    }
    cpp = xreallocarray(
        cpp as *mut libc::c_void,
        (i + 2 as libc::c_int as libc::c_long) as size_t,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    let ref mut fresh61 = *cpp.offset(i as isize);
    *fresh61 = new;
    let ref mut fresh62 = *cpp.offset((i + 1 as libc::c_int as libc::c_long) as isize);
    *fresh62 = 0 as *mut libc::c_char;
    return cpp;
}
unsafe extern "C" fn enlist(
    mut cpp: *mut *mut libc::c_char,
    mut str: *const libc::c_char,
    mut len: idx_t,
) -> *mut *mut libc::c_char {
    return enlistnew(cpp, ximemdup0(str as *const libc::c_void, len));
}
unsafe extern "C" fn comsubs(
    mut left: *mut libc::c_char,
    mut right: *const libc::c_char,
) -> *mut *mut libc::c_char {
    let mut cpp: *mut *mut libc::c_char = xzalloc(
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    let mut lcp: *mut libc::c_char = left;
    while *lcp as libc::c_int != '\0' as i32 {
        let mut len: idx_t = 0 as libc::c_int as idx_t;
        let mut rcp: *mut libc::c_char = strchr(right, *lcp as libc::c_int);
        while !rcp.is_null() {
            let mut i: idx_t = 0;
            i = 1 as libc::c_int as idx_t;
            while *lcp.offset(i as isize) as libc::c_int != '\0' as i32
                && *lcp.offset(i as isize) as libc::c_int
                    == *rcp.offset(i as isize) as libc::c_int
            {
                i += 1;
                i;
            }
            if i > len {
                len = i;
            }
            rcp = strchr(rcp.offset(1 as libc::c_int as isize), *lcp as libc::c_int);
        }
        if len != 0 as libc::c_int as libc::c_long {
            cpp = enlist(cpp, lcp, len);
        }
        lcp = lcp.offset(1);
        lcp;
    }
    return cpp;
}
unsafe extern "C" fn addlists(
    mut old: *mut *mut libc::c_char,
    mut new: *mut *mut libc::c_char,
) -> *mut *mut libc::c_char {
    while !(*new).is_null() {
        old = enlistnew(old, xstrdup(*new));
        new = new.offset(1);
        new;
    }
    return old;
}
unsafe extern "C" fn inboth(
    mut left: *mut *mut libc::c_char,
    mut right: *mut *mut libc::c_char,
) -> *mut *mut libc::c_char {
    let mut both: *mut *mut libc::c_char = xzalloc(
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    let mut lnum: idx_t = 0 as libc::c_int as idx_t;
    while !(*left.offset(lnum as isize)).is_null() {
        let mut rnum: idx_t = 0 as libc::c_int as idx_t;
        while !(*right.offset(rnum as isize)).is_null() {
            let mut temp: *mut *mut libc::c_char = comsubs(
                *left.offset(lnum as isize),
                *right.offset(rnum as isize),
            );
            both = addlists(both, temp);
            freelist(temp);
            rpl_free(temp as *mut libc::c_void);
            rnum += 1;
            rnum;
        }
        lnum += 1;
        lnum;
    }
    return both;
}
unsafe extern "C" fn allocmust(mut mp: *mut must, mut size: idx_t) -> *mut must {
    let mut new_mp: *mut must = xmalloc(::std::mem::size_of::<must>() as libc::c_ulong)
        as *mut must;
    (*new_mp)
        .in_0 = xzalloc(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        as *mut *mut libc::c_char;
    (*new_mp).left = xizalloc(size) as *mut libc::c_char;
    (*new_mp).right = xizalloc(size) as *mut libc::c_char;
    (*new_mp).is = xizalloc(size) as *mut libc::c_char;
    (*new_mp).begline = 0 as libc::c_int != 0;
    (*new_mp).endline = 0 as libc::c_int != 0;
    (*new_mp).prev = mp;
    return new_mp;
}
unsafe extern "C" fn resetmust(mut mp: *mut must) {
    freelist((*mp).in_0);
    let ref mut fresh63 = *((*mp).in_0).offset(0 as libc::c_int as isize);
    *fresh63 = 0 as *mut libc::c_char;
    let ref mut fresh64 = *((*mp).is).offset(0 as libc::c_int as isize);
    *fresh64 = '\0' as i32 as libc::c_char;
    let ref mut fresh65 = *((*mp).right).offset(0 as libc::c_int as isize);
    *fresh65 = *fresh64;
    *((*mp).left).offset(0 as libc::c_int as isize) = *fresh65;
    (*mp).begline = 0 as libc::c_int != 0;
    (*mp).endline = 0 as libc::c_int != 0;
}
unsafe extern "C" fn freemust(mut mp: *mut must) {
    freelist((*mp).in_0);
    rpl_free((*mp).in_0 as *mut libc::c_void);
    rpl_free((*mp).left as *mut libc::c_void);
    rpl_free((*mp).right as *mut libc::c_void);
    rpl_free((*mp).is as *mut libc::c_void);
    rpl_free(mp as *mut libc::c_void);
}
pub unsafe extern "C" fn dfamust(mut d: *const dfa) -> *mut dfamust {
    let mut current_block: u64;
    let mut mp: *mut must = 0 as *mut must;
    let mut result: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut exact: bool = 0 as libc::c_int != 0;
    let mut begline: bool = 0 as libc::c_int != 0;
    let mut endline: bool = 0 as libc::c_int != 0;
    let mut need_begline: bool = 0 as libc::c_int != 0;
    let mut need_endline: bool = 0 as libc::c_int != 0;
    let mut case_fold_unibyte: bool = (*d).syntax.case_fold as libc::c_int
        & !(*d).localeinfo.multibyte as libc::c_int != 0;
    let mut ri: idx_t = 1 as libc::c_int as idx_t;
    while (ri + 1 as libc::c_int as libc::c_long) < (*d).tindex {
        let mut t: token = *((*d).tokens).offset(ri as isize);
        match t {
            268 => {
                mp = allocmust(mp, 2 as libc::c_int as idx_t);
                (*mp).begline = 1 as libc::c_int != 0;
                need_begline = 1 as libc::c_int != 0;
                current_block = 4644295000439058019;
            }
            269 => {
                mp = allocmust(mp, 2 as libc::c_int as idx_t);
                (*mp).endline = 1 as libc::c_int != 0;
                need_endline = 1 as libc::c_int != 0;
                current_block = 4644295000439058019;
            }
            263 | 264 => {
                if (b"neither LPAREN nor RPAREN may appear here\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
                {} else {
                    __assert_fail(
                        b"!\"neither LPAREN nor RPAREN may appear here\"\0" as *const u8
                            as *const libc::c_char,
                        b"dfa.c\0" as *const u8 as *const libc::c_char,
                        4157 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 44],
                            &[libc::c_char; 44],
                        >(b"struct dfamust *dfamust(const struct dfa *)\0"))
                            .as_ptr(),
                    );
                }
                'c_17879: {
                    if (b"neither LPAREN nor RPAREN may appear here\0" as *const u8
                        as *const libc::c_char)
                        .is_null()
                    {} else {
                        __assert_fail(
                            b"!\"neither LPAREN nor RPAREN may appear here\"\0"
                                as *const u8 as *const libc::c_char,
                            b"dfa.c\0" as *const u8 as *const libc::c_char,
                            4157 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 44],
                                &[libc::c_char; 44],
                            >(b"struct dfamust *dfamust(const struct dfa *)\0"))
                                .as_ptr(),
                        );
                    }
                };
                current_block = 15284946753588281703;
            }
            256 | 270 | 271 | 272 | 273 | 274 | 266 | 275 => {
                current_block = 15284946753588281703;
            }
            258 | 257 => {
                if !mp.is_null() {} else {
                    unreachable!();
                };
                resetmust(mp);
                current_block = 4644295000439058019;
            }
            262 => {
                let mut new: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
                let mut rmp: *mut must = mp;
                if !rmp.is_null() {} else {
                    unreachable!();
                };
                mp = (*mp).prev;
                let mut lmp: *mut must = mp;
                if !lmp.is_null() {} else {
                    unreachable!();
                };
                let mut j: idx_t = 0;
                let mut ln: idx_t = 0;
                let mut rn: idx_t = 0;
                let mut n: idx_t = 0;
                if str_eq((*lmp).is, (*rmp).is) {
                    (*lmp)
                        .begline = ((*lmp).begline as libc::c_int
                        & (*rmp).begline as libc::c_int) != 0;
                    (*lmp)
                        .endline = ((*lmp).endline as libc::c_int
                        & (*rmp).endline as libc::c_int) != 0;
                } else {
                    *((*lmp).is)
                        .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                    (*lmp).begline = 0 as libc::c_int != 0;
                    (*lmp).endline = 0 as libc::c_int != 0;
                }
                let mut i: idx_t = 0 as libc::c_int as idx_t;
                while *((*lmp).left).offset(i as isize) as libc::c_int != '\0' as i32
                    && *((*lmp).left).offset(i as isize) as libc::c_int
                        == *((*rmp).left).offset(i as isize) as libc::c_int
                {
                    i += 1;
                    i;
                }
                *((*lmp).left).offset(i as isize) = '\0' as i32 as libc::c_char;
                ln = strlen((*lmp).right) as idx_t;
                rn = strlen((*rmp).right) as idx_t;
                n = ln;
                if n > rn {
                    n = rn;
                }
                i = 0 as libc::c_int as idx_t;
                while i < n {
                    if *((*lmp).right)
                        .offset((ln - i - 1 as libc::c_int as libc::c_long) as isize)
                        as libc::c_int
                        != *((*rmp).right)
                            .offset((rn - i - 1 as libc::c_int as libc::c_long) as isize)
                            as libc::c_int
                    {
                        break;
                    }
                    i += 1;
                    i;
                }
                j = 0 as libc::c_int as idx_t;
                while j < i {
                    *((*lmp).right)
                        .offset(
                            j as isize,
                        ) = *((*lmp).right).offset((ln - i + j) as isize);
                    j += 1;
                    j;
                }
                *((*lmp).right).offset(j as isize) = '\0' as i32 as libc::c_char;
                new = inboth((*lmp).in_0, (*rmp).in_0);
                freelist((*lmp).in_0);
                rpl_free((*lmp).in_0 as *mut libc::c_void);
                (*lmp).in_0 = new;
                freemust(rmp);
                current_block = 4644295000439058019;
            }
            259 => {
                if !mp.is_null() {} else {
                    unreachable!();
                };
                *((*mp).is)
                    .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                current_block = 4644295000439058019;
            }
            -1 => {
                if !mp.is_null() {} else {
                    unreachable!();
                };
                if ((*mp).prev).is_null() {} else {
                    __assert_fail(
                        b"!mp->prev\0" as *const u8 as *const libc::c_char,
                        b"dfa.c\0" as *const u8 as *const libc::c_char,
                        4229 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 44],
                            &[libc::c_char; 44],
                        >(b"struct dfamust *dfamust(const struct dfa *)\0"))
                            .as_ptr(),
                    );
                }
                'c_17073: {
                    if ((*mp).prev).is_null() {} else {
                        __assert_fail(
                            b"!mp->prev\0" as *const u8 as *const libc::c_char,
                            b"dfa.c\0" as *const u8 as *const libc::c_char,
                            4229 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 44],
                                &[libc::c_char; 44],
                            >(b"struct dfamust *dfamust(const struct dfa *)\0"))
                                .as_ptr(),
                        );
                    }
                };
                let mut i_0: idx_t = 0 as libc::c_int as idx_t;
                while !(*((*mp).in_0).offset(i_0 as isize)).is_null() {
                    if strlen(*((*mp).in_0).offset(i_0 as isize)) > strlen(result) {
                        result = *((*mp).in_0).offset(i_0 as isize);
                    }
                    i_0 += 1;
                    i_0;
                }
                if str_eq(result, (*mp).is) {
                    if (!need_begline || (*mp).begline as libc::c_int != 0)
                        && (!need_endline || (*mp).endline as libc::c_int != 0)
                    {
                        exact = 1 as libc::c_int != 0;
                    }
                    begline = (*mp).begline;
                    endline = (*mp).endline;
                }
                break;
            }
            261 => {
                let mut rmp_0: *mut must = mp;
                if !rmp_0.is_null() {} else {
                    unreachable!();
                };
                mp = (*mp).prev;
                let mut lmp_0: *mut must = mp;
                if !lmp_0.is_null() {} else {
                    unreachable!();
                };
                (*lmp_0).in_0 = addlists((*lmp_0).in_0, (*rmp_0).in_0);
                if *((*lmp_0).right).offset(0 as libc::c_int as isize) as libc::c_int
                    != '\0' as i32
                    && *((*rmp_0).left).offset(0 as libc::c_int as isize) as libc::c_int
                        != '\0' as i32
                {
                    let mut lrlen: idx_t = strlen((*lmp_0).right) as idx_t;
                    let mut rllen: idx_t = strlen((*rmp_0).left) as idx_t;
                    let mut tp: *mut libc::c_char = ximalloc(
                        lrlen + rllen + 1 as libc::c_int as libc::c_long,
                    ) as *mut libc::c_char;
                    memcpy(
                        tp.offset(lrlen as isize) as *mut libc::c_void,
                        (*rmp_0).left as *const libc::c_void,
                        (rllen + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                    );
                    memcpy(
                        tp as *mut libc::c_void,
                        (*lmp_0).right as *const libc::c_void,
                        lrlen as libc::c_ulong,
                    );
                    (*lmp_0).in_0 = enlistnew((*lmp_0).in_0, tp);
                }
                if *((*lmp_0).is).offset(0 as libc::c_int as isize) as libc::c_int
                    != '\0' as i32
                {
                    (*lmp_0).left = icatalloc((*lmp_0).left, (*rmp_0).left);
                }
                if *((*rmp_0).is).offset(0 as libc::c_int as isize) as libc::c_int
                    == '\0' as i32
                {
                    *((*lmp_0).right)
                        .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                }
                (*lmp_0).right = icatalloc((*lmp_0).right, (*rmp_0).right);
                if (*((*lmp_0).is).offset(0 as libc::c_int as isize) as libc::c_int
                    != '\0' as i32 || (*lmp_0).begline as libc::c_int != 0)
                    && (*((*rmp_0).is).offset(0 as libc::c_int as isize) as libc::c_int
                        != '\0' as i32 || (*rmp_0).endline as libc::c_int != 0)
                {
                    (*lmp_0).is = icatalloc((*lmp_0).is, (*rmp_0).is);
                    (*lmp_0).endline = (*rmp_0).endline;
                } else {
                    *((*lmp_0).is)
                        .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                    (*lmp_0).begline = 0 as libc::c_int != 0;
                    (*lmp_0).endline = 0 as libc::c_int != 0;
                }
                freemust(rmp_0);
                current_block = 4644295000439058019;
            }
            0 => {
                break;
            }
            _ => {
                if CSET as libc::c_int as libc::c_long <= t {
                    let mut ccl: *mut charclass = &mut *((*d).charclasses)
                        .offset((t - CSET as libc::c_int as libc::c_long) as isize)
                        as *mut charclass;
                    let mut j_0: libc::c_int = 0;
                    j_0 = 0 as libc::c_int;
                    while j_0 < NOTCHAR as libc::c_int {
                        if tstbit(j_0 as libc::c_uint, ccl) {
                            break;
                        }
                        j_0 += 1;
                        j_0;
                    }
                    if !(j_0 < NOTCHAR as libc::c_int) {
                        mp = allocmust(mp, 2 as libc::c_int as idx_t);
                        current_block = 4644295000439058019;
                    } else {
                        t = j_0 as token;
                        loop {
                            j_0 += 1;
                            if !(j_0 < NOTCHAR as libc::c_int) {
                                break;
                            }
                            if tstbit(j_0 as libc::c_uint, ccl) as libc::c_int != 0
                                && !(case_fold_unibyte as libc::c_int != 0
                                    && ({
                                        let mut __res: libc::c_int = 0;
                                        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            > 1 as libc::c_int as libc::c_ulong
                                        {
                                            if 0 != 0 {
                                                let mut __c: libc::c_int = j_0;
                                                __res = if __c < -(128 as libc::c_int)
                                                    || __c > 255 as libc::c_int
                                                {
                                                    __c
                                                } else {
                                                    *(*__ctype_toupper_loc()).offset(__c as isize)
                                                };
                                            } else {
                                                __res = toupper(j_0);
                                            }
                                        } else {
                                            __res = *(*__ctype_toupper_loc()).offset(j_0 as isize);
                                        }
                                        __res
                                    })
                                        == ({
                                            let mut __res: libc::c_int = 0;
                                            if ::std::mem::size_of::<token>() as libc::c_ulong
                                                > 1 as libc::c_int as libc::c_ulong
                                            {
                                                if 0 != 0 {
                                                    let mut __c: libc::c_int = t as libc::c_int;
                                                    __res = if __c < -(128 as libc::c_int)
                                                        || __c > 255 as libc::c_int
                                                    {
                                                        __c
                                                    } else {
                                                        *(*__ctype_toupper_loc()).offset(__c as isize)
                                                    };
                                                } else {
                                                    __res = toupper(t as libc::c_int);
                                                }
                                            } else {
                                                __res = *(*__ctype_toupper_loc())
                                                    .offset(t as libc::c_int as isize);
                                            }
                                            __res
                                        }))
                            {
                                break;
                            }
                        }
                        if j_0 < NOTCHAR as libc::c_int {
                            mp = allocmust(mp, 2 as libc::c_int as idx_t);
                            current_block = 4644295000439058019;
                        } else {
                            current_block = 10261677128829721533;
                        }
                    }
                } else {
                    current_block = 10261677128829721533;
                }
                match current_block {
                    4644295000439058019 => {}
                    _ => {
                        let mut rj: idx_t = ri + 2 as libc::c_int as libc::c_long;
                        if *((*d).tokens)
                            .offset((ri + 1 as libc::c_int as libc::c_long) as isize)
                            == CAT as libc::c_int as libc::c_long
                        {
                            while rj < (*d).tindex - 1 as libc::c_int as libc::c_long {
                                if rj != ri
                                    && (*((*d).tokens).offset(rj as isize)
                                        <= 0 as libc::c_int as libc::c_long
                                        || NOTCHAR as libc::c_int as libc::c_long
                                            <= *((*d).tokens).offset(rj as isize))
                                    || *((*d).tokens)
                                        .offset((rj + 1 as libc::c_int as libc::c_long) as isize)
                                        != CAT as libc::c_int as libc::c_long
                                {
                                    break;
                                }
                                rj += 2 as libc::c_int as libc::c_long;
                            }
                        }
                        mp = allocmust(
                            mp,
                            (rj - ri >> 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_long,
                        );
                        let ref mut fresh66 = *((*mp).right)
                            .offset(0 as libc::c_int as isize);
                        *fresh66 = (if case_fold_unibyte as libc::c_int != 0 {
                            ({
                                let mut __res: libc::c_int = 0;
                                if ::std::mem::size_of::<token>() as libc::c_ulong
                                    > 1 as libc::c_int as libc::c_ulong
                                {
                                    if 0 != 0 {
                                        let mut __c: libc::c_int = t as libc::c_int;
                                        __res = if __c < -(128 as libc::c_int)
                                            || __c > 255 as libc::c_int
                                        {
                                            __c
                                        } else {
                                            *(*__ctype_toupper_loc()).offset(__c as isize)
                                        };
                                    } else {
                                        __res = toupper(t as libc::c_int);
                                    }
                                } else {
                                    __res = *(*__ctype_toupper_loc())
                                        .offset(t as libc::c_int as isize);
                                }
                                __res
                            }) as libc::c_long
                        } else {
                            t
                        }) as libc::c_char;
                        let ref mut fresh67 = *((*mp).left)
                            .offset(0 as libc::c_int as isize);
                        *fresh67 = *fresh66;
                        *((*mp).is).offset(0 as libc::c_int as isize) = *fresh67;
                        let mut i_1: idx_t = 0;
                        i_1 = 1 as libc::c_int as idx_t;
                        while (ri + 2 as libc::c_int as libc::c_long) < rj {
                            ri += 2 as libc::c_int as libc::c_long;
                            t = *((*d).tokens).offset(ri as isize);
                            let ref mut fresh68 = *((*mp).right).offset(i_1 as isize);
                            *fresh68 = (if case_fold_unibyte as libc::c_int != 0 {
                                ({
                                    let mut __res: libc::c_int = 0;
                                    if ::std::mem::size_of::<token>() as libc::c_ulong
                                        > 1 as libc::c_int as libc::c_ulong
                                    {
                                        if 0 != 0 {
                                            let mut __c: libc::c_int = t as libc::c_int;
                                            __res = if __c < -(128 as libc::c_int)
                                                || __c > 255 as libc::c_int
                                            {
                                                __c
                                            } else {
                                                *(*__ctype_toupper_loc()).offset(__c as isize)
                                            };
                                        } else {
                                            __res = toupper(t as libc::c_int);
                                        }
                                    } else {
                                        __res = *(*__ctype_toupper_loc())
                                            .offset(t as libc::c_int as isize);
                                    }
                                    __res
                                }) as libc::c_long
                            } else {
                                t
                            }) as libc::c_char;
                            let ref mut fresh69 = *((*mp).left).offset(i_1 as isize);
                            *fresh69 = *fresh68;
                            *((*mp).is).offset(i_1 as isize) = *fresh69;
                            i_1 += 1;
                            i_1;
                        }
                        let ref mut fresh70 = *((*mp).right).offset(i_1 as isize);
                        *fresh70 = '\0' as i32 as libc::c_char;
                        let ref mut fresh71 = *((*mp).left).offset(i_1 as isize);
                        *fresh71 = *fresh70;
                        *((*mp).is).offset(i_1 as isize) = *fresh71;
                        (*mp).in_0 = enlist((*mp).in_0, (*mp).is, i_1);
                        current_block = 4644295000439058019;
                    }
                }
            }
        }
        match current_block {
            15284946753588281703 => {
                mp = allocmust(mp, 2 as libc::c_int as idx_t);
            }
            _ => {}
        }
        ri += 1;
        ri;
    }
    let mut dm: *mut dfamust = 0 as *mut dfamust;
    if *result != 0 {
        dm = xmalloc(
            (3 as libc::c_ulong)
                .wrapping_add(::std::mem::align_of::<dfamust>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (strlen(result)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                )
                & !(::std::mem::align_of::<dfamust>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) as *mut dfamust;
        (*dm).exact = exact;
        (*dm).begline = begline;
        (*dm).endline = endline;
        strcpy(((*dm).must).as_mut_ptr(), result);
    }
    while !mp.is_null() {
        let mut prev: *mut must = (*mp).prev;
        freemust(mp);
        mp = prev;
    }
    return dm;
}
pub unsafe extern "C" fn dfamustfree(mut dm: *mut dfamust) {
    rpl_free(dm as *mut libc::c_void);
}
pub unsafe extern "C" fn dfaalloc() -> *mut dfa {
    return xmalloc(::std::mem::size_of::<dfa>() as libc::c_ulong) as *mut dfa;
}
pub unsafe extern "C" fn dfasyntax(
    mut dfa: *mut dfa,
    mut linfo: *const localeinfo,
    mut bits: reg_syntax_t,
    mut dfaopts: libc::c_int,
) {
    memset(dfa as *mut libc::c_void, 0 as libc::c_int, 1040 as libc::c_ulong);
    (*dfa)
        .dfaexec = if (*linfo).multibyte as libc::c_int != 0 {
        Some(
            dfaexec_mb
                as unsafe extern "C" fn(
                    *mut dfa,
                    *const libc::c_char,
                    *mut libc::c_char,
                    bool,
                    *mut idx_t,
                    *mut bool,
                ) -> *mut libc::c_char,
        )
    } else {
        Some(
            dfaexec_sb
                as unsafe extern "C" fn(
                    *mut dfa,
                    *const libc::c_char,
                    *mut libc::c_char,
                    bool,
                    *mut idx_t,
                    *mut bool,
                ) -> *mut libc::c_char,
        )
    };
    (*dfa).localeinfo = *linfo;
    (*dfa).fast = !(*dfa).localeinfo.multibyte;
    (*dfa).canychar = -(1 as libc::c_int) as ptrdiff_t;
    (*dfa).syntax.syntax_bits_set = 1 as libc::c_int != 0;
    (*dfa)
        .syntax
        .case_fold = bits
        & ((((((((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
        != 0 as libc::c_int as libc::c_ulong;
    (*dfa)
        .syntax
        .eolbyte = (if dfaopts & DFA_EOL_NUL as libc::c_int != 0 {
        '\0' as i32
    } else {
        '\n' as i32
    }) as libc::c_uchar;
    (*dfa).syntax.syntax_bits = bits;
    (*dfa).syntax.dfaopts = dfaopts;
    let mut i: libc::c_int = -(127 as libc::c_int) - 1 as libc::c_int;
    while i <= 127 as libc::c_int {
        let mut uc: libc::c_uchar = i as libc::c_uchar;
        (*dfa).syntax.sbit[uc as usize] = char_context(dfa, uc) as libc::c_char;
        match (*dfa).syntax.sbit[uc as usize] as libc::c_int {
            2 => {
                setbit(uc as libc::c_uint, &mut (*dfa).syntax.letters);
            }
            4 => {
                setbit(uc as libc::c_uint, &mut (*dfa).syntax.newline);
            }
            _ => {}
        }
        (*dfa)
            .syntax
            .never_trail[uc
            as usize] = if (*dfa).localeinfo.using_utf8 as libc::c_int != 0 {
            (uc as libc::c_int & 0xc0 as libc::c_int != 0x80 as libc::c_int)
                as libc::c_int
        } else {
            (strchr(b"\n\r./\0" as *const u8 as *const libc::c_char, uc as libc::c_int)
                != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
        } != 0;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn dfacopysyntax(mut to: *mut dfa, mut from: *const dfa) {
    memset(to as *mut libc::c_void, 0 as libc::c_int, 448 as libc::c_ulong);
    (*to).canychar = -(1 as libc::c_int) as ptrdiff_t;
    (*to).fast = (*from).fast;
    (*to).syntax = (*from).syntax;
    (*to).dfaexec = (*from).dfaexec;
    (*to).localeinfo = (*from).localeinfo;
}
unsafe extern "C" fn run_static_initializers() {
    CHARCLASS_WORD_MASK = (((1 as libc::c_int as charclass_word)
        << CHARCLASS_WORD_BITS as libc::c_int - 1 as libc::c_int) << 1 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    utf8_classes = [
        {
            let mut init = charclass {
                w: [
                    ((0xffffffff as libc::c_uint as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0xffffffff as libc::c_uint as libc::c_ulong),
                    ((0xffffffff as libc::c_uint as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0xffffffff as libc::c_uint as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                ],
            };
            init
        },
        {
            let mut init = charclass {
                w: [
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0xfffffffc as libc::c_uint as libc::c_ulong),
                ],
            };
            init
        },
        {
            let mut init = charclass {
                w: [
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0xffffffff as libc::c_uint as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0xffffffff as libc::c_uint as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                ],
            };
            init
        },
        {
            let mut init = charclass {
                w: [
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0xffffffff as libc::c_uint as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                ],
            };
            init
        },
        {
            let mut init = charclass {
                w: [
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0xdffe as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                ],
            };
            init
        },
        {
            let mut init = charclass {
                w: [
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0xffffffff as libc::c_uint as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                ],
            };
            init
        },
        {
            let mut init = charclass {
                w: [
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0xffffffff as libc::c_uint as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0xffff0000 as libc::c_uint as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                ],
            };
            init
        },
        {
            let mut init = charclass {
                w: [
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0xe0000 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                ],
            };
            init
        },
        {
            let mut init = charclass {
                w: [
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0xffff as libc::c_int as libc::c_ulong),
                    ((0 as libc::c_int as charclass_word) << 32 as libc::c_int)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                ],
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
