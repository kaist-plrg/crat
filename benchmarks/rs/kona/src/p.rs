use ::libc;
extern "C" {
    fn atol(__nptr: *const libc::c_char) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn tolower(_: libc::c_int) -> libc::c_int;
    fn cd(a: K) -> K;
    fn lsz(k: I) -> I;
    fn show(a: K) -> K;
    fn adverbClass(p: V) -> I;
    fn sva(p: V) -> I;
    fn charsAdverb(c: C) -> L;
    fn charsVerb(c: C) -> L;
    fn isCharVerb(c: C) -> I;
    fn charpos(s: S, c: C) -> I;
    fn stringHasChar(s: S, c: C) -> I;
    static mut IFP: [S; 3];
    static mut offsetColon: V;
    #[link_name = "match"]
    fn match_0(a: K, b: K) -> K;
    fn bk(p: V) -> I;
    fn end() -> K;
    fn simpleString(a: S) -> I;
    fn maX(a: I, b: I) -> I;
    fn DT_OFFSET(v: V) -> L;
    static mut LS: S;
    static mut d_: S;
    static mut KTREE: K;
    fn kerr(s: cS) -> K;
    static mut DT_SIZE: L;
    static mut DT_ADVERB_OFFSET: L;
    static mut DT_VERB_OFFSET: L;
    static mut DT_SPECIAL_VERB_OFFSET: L;
    static mut DT: [TR; 0];
    fn newE(s: S, k: K) -> K;
    fn newEntry(s: S) -> K;
    fn Kv() -> K;
    fn Kd() -> K;
    fn pdafree(p: PDA);
    fn bottom(p: PDA) -> C;
    fn pop(p: PDA) -> C;
    fn peek(p: PDA) -> C;
    fn push(p: PDA, c: C) -> I;
    fn newPDA() -> PDA;
    fn kap(a: *mut K, v: V) -> K;
    fn _n() -> K;
    fn newK(t: I, n: I) -> K;
    fn ci(a: K) -> K;
    fn sz(t: I, n: I) -> I;
    fn OOM_CD(g: I, _: ...) -> I;
    static mut fbr: I;
    static mut stopDict: K;
    static mut fStop: I;
    fn alloc(sz_0: size_t) -> V;
    fn SC(a: S, b: S) -> I;
    fn denameS(dir_string: S, t: S, create: I) -> *mut K;
    fn EV(e: K) -> K;
    static mut vn_: [V; 0];
    static mut n_s: S;
    fn sp(k: S) -> S;
    fn formKfCS(s: S) -> K;
    fn formKiCS(s: S) -> K;
    fn strdupn(s: S, k: I) -> S;
    fn DE(d: K, b: S) -> K;
    fn EVP(e: K) -> *mut K;
    fn EAP(e: K) -> *mut K;
    fn DI(d: K, i: I) -> K;
    fn backslash(s: S, n: I, dict: *mut K) -> K;
    fn denameD(d: *mut K, t: S, create: I) -> *mut K;
    fn lookupEntryOrCreate(p: *mut K, k: S) -> K;
}
pub type size_t = libc::c_ulong;
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
pub type L = libc::c_longlong;
pub type V = *mut libc::c_void;
pub type I = libc::c_longlong;
pub type F = libc::c_double;
pub type C = libc::c_char;
pub type S = *mut C;
pub type cS = *const C;
pub type UC = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct k0 {
    pub _c: I,
    pub t: I,
    pub n: I,
    pub k: [*mut k0; 1],
}
pub type K = *mut k0;
pub type TYPE_SEVEN_MEMBERS = libc::c_uint;
pub const TYPE_SEVEN_SIZE: TYPE_SEVEN_MEMBERS = 8;
pub const CACHE_TREE: TYPE_SEVEN_MEMBERS = 7;
pub const CACHE_WD: TYPE_SEVEN_MEMBERS = 6;
pub const CONJ: TYPE_SEVEN_MEMBERS = 5;
pub const PARAMS: TYPE_SEVEN_MEMBERS = 4;
pub const LOCALS: TYPE_SEVEN_MEMBERS = 3;
pub const CODE: TYPE_SEVEN_MEMBERS = 2;
pub const DEPTH: TYPE_SEVEN_MEMBERS = 1;
pub const CONTeXT: TYPE_SEVEN_MEMBERS = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pda {
    pub i: I,
    pub s: I,
    pub n: I,
    pub c: S,
}
pub type Pda = pda;
pub type PDA = *mut Pda;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct af {
    pub verb_over: V,
    pub verb_scan: V,
    pub verb_eachpair: V,
}
pub type AF = af;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tr {
    pub adverbClass: I,
    pub arity: I,
    pub func: V,
    pub text: S,
    pub alt_funcs: AF,
}
pub type TR = tr;
pub const MARK_NUMBER: mark_members = 9;
pub const MARK_SYMBOL: mark_members = 7;
pub const MARK_END: mark_members = 3;
pub const MARK_QUOTE: mark_members = 6;
pub const MARK_BRACE: mark_members = 5;
pub const MARK_BRACKET: mark_members = 2;
pub const MARK_PAREN: mark_members = 4;
pub const MARK_NAME: mark_members = 8;
pub const MARK_ADVERB: mark_members = 11;
pub const MARK_VERB: mark_members = 10;
pub const MARK_CONDITIONAL: mark_members = 12;
pub const MARK_IGNORE: mark_members = 1;
pub const MARK_UNMARKED: mark_members = 0;
pub type mark_members = libc::c_uint;
pub const MARK_COUNT: mark_members = 13;
unsafe extern "C" fn formed_group(mut c: C) -> I {
    let mut s: S = b"\n \\/\"\0" as *const u8 as *const libc::c_char as S;
    return charpos(s, c);
}
pub static mut formed_dfa: S = b"023451021151021451033333044444556515555555\0"
    as *const u8 as *const libc::c_char as S;
pub static mut left: S = b"([{\0" as *const u8 as *const libc::c_char as S;
pub static mut right: S = b")]}\0" as *const u8 as *const libc::c_char as S;
pub static mut lineA: S = 0 as *const C as *mut C;
pub static mut lineB: S = 0 as *const C as *mut C;
pub static mut fdc: I = 1 as libc::c_int as I;
pub static mut fll: I = 0 as libc::c_int as I;
unsafe extern "C" fn flop(mut c: C) -> C {
    return (if c as libc::c_int == '(' as i32 {
        ')' as i32
    } else if c as libc::c_int == '[' as i32 {
        ']' as i32
    } else if c as libc::c_int == '{' as i32 {
        '}' as i32
    } else {
        c as libc::c_int
    }) as C;
}
pub unsafe extern "C" fn parsedepth(mut p: PDA) -> I {
    return if !p.is_null() {
        (*p).n
            + (if (*p).s >= 5 as libc::c_int as libc::c_longlong {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_longlong
    } else {
        0 as libc::c_int as libc::c_longlong
    };
}
pub unsafe extern "C" fn complete(
    mut a: S,
    mut n: I,
    mut q: *mut PDA,
    mut marks: *mut I,
) -> I {
    if (*q).is_null() {
        *q = newPDA();
    }
    let mut p: PDA = *q;
    if p.is_null() {
        return -(1 as libc::c_int) as I;
    }
    let mut t: C = 0;
    let mut r: I = formed_group('\0' as i32 as C) + 1 as libc::c_int as libc::c_longlong;
    while (*p).i < n {
        let mut before_pn: I = (*p).n;
        let mut before_sq: I = ((*p).s >= 5 as libc::c_int as libc::c_longlong)
            as libc::c_int as I;
        let mut before_sc: I = ((*p).s == 4 as libc::c_int as libc::c_longlong)
            as libc::c_int as I;
        t = *a.offset((*p).i as isize);
        if (*p).s < 4 as libc::c_int as libc::c_longlong && t as libc::c_int != 0 {
            if !(strchr(left as *const libc::c_char, t as libc::c_int)).is_null() {
                if push(p, flop(t)) != 0 {
                    return -(1 as libc::c_int) as I;
                }
                if (*p).n > 99 as libc::c_int as libc::c_longlong {
                    return 3 as libc::c_int as I;
                }
            } else if !(strchr(right as *const libc::c_char, t as libc::c_int)).is_null()
            {
                if peek(p) as libc::c_int != t as libc::c_int {
                    return 2 as libc::c_int as I
                } else {
                    pop(p);
                }
            }
        }
        (*p)
            .s = (*formed_dfa
            .offset((r * (*p).s + formed_group(*a.offset((*p).i as isize))) as isize)
            as libc::c_int - '0' as i32) as I;
        if !marks.is_null() {
            let mut bot: C = bottom(p);
            let mut m: I = if bot as libc::c_int == ')' as i32 {
                MARK_PAREN as libc::c_int as libc::c_longlong
            } else if bot as libc::c_int == ']' as i32 {
                MARK_BRACKET as libc::c_int as libc::c_longlong
            } else if bot as libc::c_int == '}' as i32 {
                MARK_BRACE as libc::c_int as libc::c_longlong
            } else if (*p).s == 4 as libc::c_int as libc::c_longlong {
                MARK_IGNORE as libc::c_int as libc::c_longlong
            } else if (*p).s >= 5 as libc::c_int as libc::c_longlong {
                MARK_QUOTE as libc::c_int as libc::c_longlong
            } else if before_pn != 0 || before_sq != 0 {
                if *marks
                    .offset(((*p).i - 1 as libc::c_int as libc::c_longlong) as isize)
                    < 0 as libc::c_int as libc::c_longlong
                {
                    -*marks
                        .offset(((*p).i - 1 as libc::c_int as libc::c_longlong) as isize)
                } else {
                    *marks
                        .offset(((*p).i - 1 as libc::c_int as libc::c_longlong) as isize)
                }
            } else {
                0 as libc::c_int as libc::c_longlong
            };
            if (*p).n != 0 && before_pn == 0
                || MARK_QUOTE as libc::c_int as libc::c_longlong == m && before_sq == 0
                || MARK_IGNORE as libc::c_int as libc::c_longlong == m && before_sc == 0
            {
                m *= -(1 as libc::c_int) as libc::c_longlong;
            }
            *marks.offset((*p).i as isize) = m;
        }
        (*p).i += 1;
        (*p).i;
    }
    return (if !((*p).s >= 5 as libc::c_int as libc::c_longlong) && (*p).n == 0 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    }) as I;
}
unsafe extern "C" fn mark_symbol(mut s: S, mut n: I, mut i: I, mut m: *mut I) -> I {
    if *m.offset(i as isize) != 0 || '`' as i32 != *s.offset(i as isize) as libc::c_int {
        return 0 as libc::c_int as I;
    }
    let mut adot: I = 0 as libc::c_int as I;
    let mut j: I = 0 as libc::c_int as I;
    let mut k: I = 0;
    while i + 2 as libc::c_int as libc::c_longlong + j < n
        && -(MARK_QUOTE as libc::c_int) as libc::c_longlong
            == *m.offset((i + 1 as libc::c_int as libc::c_longlong) as isize)
        && MARK_QUOTE as libc::c_int as libc::c_longlong
            == *m.offset((i + 2 as libc::c_int as libc::c_longlong + j) as isize)
    {
        j += 1;
        j;
    }
    if j != 0 {
        return j + 2 as libc::c_int as libc::c_longlong;
    }
    loop {
        k = i + 1 as libc::c_int as libc::c_longlong + j;
        if !(k < n) {
            break;
        }
        let mut c: C = *s.offset(k as isize);
        if j == 0
            && *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            break;
        }
        if adot != 0
            && (*(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                || '.' as i32 == c as libc::c_int)
        {
            break;
        }
        if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0
            && '.' as i32 != c as libc::c_int && '_' as i32 != c as libc::c_int
        {
            break;
        }
        adot = ('.' as i32 == c as libc::c_int) as libc::c_int as I;
        j += 1;
        j;
    }
    if 1 as libc::c_int as libc::c_longlong == j && adot != 0 {
        return 1 as libc::c_int as I;
    }
    return j + 1 as libc::c_int as libc::c_longlong;
}
unsafe extern "C" fn isalnum_(mut c: C) -> I {
    return (*(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
        || '_' as i32 == c as libc::c_int) as libc::c_int as I;
}
unsafe extern "C" fn isalnumdot_(mut c: C) -> I {
    return (isalnum_(c) != 0 || '.' as i32 == c as libc::c_int) as libc::c_int as I;
}
unsafe extern "C" fn mark_name(mut s: S, mut n: I, mut i: I, mut m: *mut I) -> I {
    let mut c: I = 0 as libc::c_int as I;
    if *m.offset(i as isize) != 0 {
        return 0 as libc::c_int as I;
    }
    if i != 0
        && isalnum_(*s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)) != 0
    {
        return 0 as libc::c_int as I;
    }
    if i < n - 1 as libc::c_int as libc::c_longlong
        && *s.offset(i as isize) as libc::c_int == '_' as i32
        && *(*__ctype_b_loc())
            .offset(
                *s.offset((i + 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int as isize,
            ) as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        && (i == n - 2 as libc::c_int as libc::c_longlong
            || *m.offset((i + 2 as libc::c_int as libc::c_longlong) as isize) != 0
            || isalnumdot_(
                *s.offset((i + 2 as libc::c_int as libc::c_longlong) as isize),
            ) == 0)
    {
        return 2 as libc::c_int as I;
    }
    while i + c < n && isalnumdot_(*s.offset((i + c) as isize)) != 0 {
        if i + c < n - 1 as libc::c_int as libc::c_longlong
            && '.' as i32 == *s.offset((i + c) as isize) as libc::c_int
            && *(*__ctype_b_loc())
                .offset(
                    *s.offset((i + c + 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int as isize,
                ) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            c += 2 as libc::c_int as libc::c_longlong;
        } else {
            if !(*(*__ctype_b_loc())
                .offset(*s.offset((i + c) as isize) as libc::c_int as isize)
                as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                != 0)
            {
                break;
            }
            c += 1;
            c;
        }
        if i + c >= n {
            break;
        }
        while i + c < n && isalnum_(*s.offset((i + c) as isize)) != 0 {
            c += 1;
            c;
        }
        if i + c >= n {
            break;
        }
        if '.' as i32 == *s.offset((i + c) as isize) as libc::c_int {
            c += 1;
            c;
        }
    }
    if (1 as libc::c_int as libc::c_longlong) < i
        && '.' as i32
            == *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                as libc::c_int
        && (0 as libc::c_int as libc::c_longlong
            == *m.offset((i - 2 as libc::c_int as libc::c_longlong) as isize)
            && '.' as i32
                != *s.offset((i - 2 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int)
    {
        c = 0 as libc::c_int as I;
    }
    return c;
}
unsafe extern "C" fn mark_number(mut s: S, mut n: I, mut i: I, mut m: *mut I) -> I {
    let mut c: I = 0 as libc::c_int as I;
    if *m.offset(i as isize) != 0 {
        return 0 as libc::c_int as I;
    }
    if i != 0 && '-' as i32 == *s.offset(i as isize) as libc::c_int
        && *(*__ctype_b_loc())
            .offset(
                *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int as isize,
            ) as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        match if *m.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
            < 0 as libc::c_int as libc::c_longlong
        {
            -*m.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
        } else {
            *m.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
        } {
            2 | 4 | 7 | 8 | 9 => return 0 as libc::c_int as I,
            _ => {}
        }
    }
    if '-' as i32 == *s.offset(i as isize) as libc::c_int {
        if i < n - 2 as libc::c_int as libc::c_longlong
            && '.' as i32
                == *s.offset((i + 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int
            && *(*__ctype_b_loc())
                .offset(
                    *s.offset((i + 2 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            c += 1;
            c;
        } else if i < n - 1 as libc::c_int as libc::c_longlong
            && *(*__ctype_b_loc())
                .offset(
                    *s.offset((i + 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            c += 1;
            c;
        } else {
            return 0 as libc::c_int as I
        }
    }
    while i + c < n && *m.offset((i + c) as isize) == 0
        && *(*__ctype_b_loc())
            .offset(*s.offset((i + c) as isize) as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        c += 1;
        c;
    }
    if i + c < n && '.' as i32 == *s.offset((i + c) as isize) as libc::c_int {
        if c != 0
            || i + c < n - 1 as libc::c_int as libc::c_longlong
                && *(*__ctype_b_loc())
                    .offset(
                        *s
                            .offset(
                                (i + c + 1 as libc::c_int as libc::c_longlong) as isize,
                            ) as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            c += 1;
            c;
        } else {
            return 0 as libc::c_int as I
        }
    }
    while i + c < n && *m.offset((i + c) as isize) == 0
        && *(*__ctype_b_loc())
            .offset(*s.offset((i + c) as isize) as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        c += 1;
        c;
    }
    if i + c < n && 'e' as i32 == tolower(*s.offset((i + c) as isize) as libc::c_int) {
        if c == 0 {
            return 0 as libc::c_int as I
        } else if i + c < n - 2 as libc::c_int as libc::c_longlong
            && '-' as i32
                == *s.offset((i + c + 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int
            && *(*__ctype_b_loc())
                .offset(
                    *s.offset((i + c + 2 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            c += 2 as libc::c_int as libc::c_longlong;
        } else if i + c < n - 1 as libc::c_int as libc::c_longlong
            && *(*__ctype_b_loc())
                .offset(
                    *s.offset((i + c + 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            c += 1;
            c;
        } else {
            return 0 as libc::c_int as I
        }
    }
    while i + c < n && *m.offset((i + c) as isize) == 0
        && *(*__ctype_b_loc())
            .offset(*s.offset((i + c) as isize) as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        c += 1;
        c;
    }
    if i + c < n
        && (1 as libc::c_int as libc::c_longlong == c
            && '0' as i32 == *s.offset(i as isize) as libc::c_int
            || 2 as libc::c_int as libc::c_longlong == c
                && '-' as i32 == *s.offset(i as isize) as libc::c_int
                && '0' as i32
                    == *s.offset((i + 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int)
        && stringHasChar(
            b"NIni\0" as *const u8 as *const libc::c_char as S,
            *s.offset((i + c) as isize),
        ) != 0
        && (i + c == n - 1 as libc::c_int as libc::c_longlong
            || *(*__ctype_b_loc())
                .offset(
                    *s.offset((i + c + 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int as isize,
                ) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int == 0)
    {
        c += 1;
        c;
    }
    if c != 0 {
        while i + c < n && *m.offset((i + c) as isize) == 0
            && *(*__ctype_b_loc())
                .offset(*s.offset((i + c) as isize) as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            c += 1;
            c;
        }
    }
    return c;
}
unsafe extern "C" fn mark_adverb(mut s: S, mut n: I, mut i: I, mut m: *mut I) -> I {
    if *m.offset(i as isize) != 0 {
        return 0 as libc::c_int as I;
    }
    let mut c: C = *s.offset(i as isize);
    if i == 0 as libc::c_int as libc::c_longlong {
        if c as libc::c_int == '\'' as i32 || c as libc::c_int == '\\' as i32 {
            if strlen(s as *const libc::c_char) > 3 as libc::c_int as libc::c_ulong {
                let mut j: I = 3 as libc::c_int as I;
                while *s.offset(j as isize) as libc::c_int != '\0' as i32 {
                    let fresh0 = j;
                    j = j + 1;
                    if *s.offset(fresh0 as isize) as libc::c_int == ')' as i32 {
                        return 0 as libc::c_int as I;
                    }
                }
            }
            return 1 as libc::c_int as I;
        }
        return 0 as libc::c_int as I;
    }
    if strcmp(s as *const libc::c_char, b";\\\\\0" as *const u8 as *const libc::c_char)
        == 0
    {
        return 1 as libc::c_int as I;
    }
    if c as libc::c_int == '\\' as i32
        && *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize) as libc::c_int
            == ';' as i32
    {
        return 0 as libc::c_int as I;
    }
    if *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize) as libc::c_int
        != ' ' as i32
        && (c as libc::c_int == '/' as i32 || c as libc::c_int == '\\' as i32
            || c as libc::c_int == '\'' as i32)
    {
        if i < n - 1 as libc::c_int as libc::c_longlong
            && *s.offset((i + 1 as libc::c_int as libc::c_longlong) as isize)
                as libc::c_int == ':' as i32
        {
            return 2 as libc::c_int as I;
        }
        return 1 as libc::c_int as I;
    }
    return 0 as libc::c_int as I;
}
unsafe extern "C" fn mark_verb(mut s: S, mut n: I, mut i: I, mut m: *mut I) -> I {
    let mut c: I = 0 as libc::c_int as I;
    while i + c < n - 1 as libc::c_int as libc::c_longlong
        && -(MARK_NUMBER as libc::c_int) as libc::c_longlong == *m.offset(i as isize)
        && {
            c += 1;
            MARK_NUMBER as libc::c_int as libc::c_longlong == *m.offset((c + i) as isize)
        }
    {}
    if c != 0 && ':' as i32 == *s.offset((i + c) as isize) as libc::c_int {
        c += 1;
        c;
        return c;
    }
    if *m.offset(i as isize) != 0 {
        return 0 as libc::c_int as I;
    }
    c = 0 as libc::c_int as I;
    if '_' as i32 == *s.offset(i as isize) as libc::c_int {
        while i + c < n
            && *(*__ctype_b_loc())
                .offset(
                    *s.offset((i + 1 as libc::c_int as libc::c_longlong + c) as isize)
                        as libc::c_int as isize,
                ) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            c += 1;
            c;
        }
    }
    if c > 1 as libc::c_int as libc::c_longlong {
        return 1 as libc::c_int as libc::c_longlong + c;
    }
    if *s.offset(i as isize) as libc::c_int == '\\' as i32
        && (*s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize) as libc::c_int
            == ' ' as i32
            || *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                as libc::c_int == ';' as i32
            || *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                as libc::c_int == '(' as i32)
    {
        return 1 as libc::c_int as I;
    }
    c = 0 as libc::c_int as I;
    if isCharVerb(*s.offset(i as isize)) != 0 {
        c += 1;
        c;
    }
    return c;
}
unsafe extern "C" fn mark_conditional(mut s: S, mut n: I, mut k: I, mut m: *mut I) -> I {
    let mut t: [S; 3] = [
        b"if\0" as *const u8 as *const libc::c_char as S,
        b"do\0" as *const u8 as *const libc::c_char as S,
        b"while\0" as *const u8 as *const libc::c_char as S,
    ];
    if *s.offset(k as isize) as libc::c_int == ':' as i32
        && *s.offset((k + 1 as libc::c_int as libc::c_longlong) as isize) as libc::c_int
            == '[' as i32
        && *s.offset((k + 2 as libc::c_int as libc::c_longlong) as isize) as libc::c_int
            != ';' as i32 && *m.offset(k as isize) == 0
    {
        return 1 as libc::c_int as I
    } else if *m.offset(k as isize) == -(MARK_NAME as libc::c_int) as libc::c_longlong {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = (::std::mem::size_of::<[S; 3]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<S>() as libc::c_ulong) as I;
        while i < _i {
            let mut c: I = strlen(t[i as usize] as *const libc::c_char) as I;
            if strncmp(
                s.offset(k as isize) as *const libc::c_char,
                t[i as usize] as *const libc::c_char,
                c as libc::c_ulong,
            ) == 0 && *s.offset((k + c) as isize) as libc::c_int == '[' as i32
            {
                return c;
            }
            i += 1;
            i;
        }
    }
    return 0 as libc::c_int as I;
}
unsafe extern "C" fn mark_end(mut s: S, mut n: I, mut i: I, mut m: *mut I) -> I {
    let mut c: C = *s.offset(i as isize);
    return (if *m.offset(i as isize) != 0 {
        0 as libc::c_int
    } else if c as libc::c_int == ';' as i32 || c as libc::c_int == '\n' as i32 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as I;
}
unsafe extern "C" fn mark_ignore(mut s: S, mut n: I, mut i: I, mut m: *mut I) -> I {
    let mut c: C = *s.offset(i as isize);
    return (if *m.offset(i as isize) != 0 {
        0 as libc::c_int
    } else if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as I;
}
unsafe extern "C" fn overcount(mut m: *mut I, mut n: I) -> I {
    let mut c: I = 0 as libc::c_int as I;
    let mut p: I = 0 as libc::c_int as I;
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = n;
    while i < _i {
        if *m.offset(i as isize) <= -(MARK_END as libc::c_int) as libc::c_longlong
            && !(*m.offset(i as isize) == p
                && (p == -(MARK_SYMBOL as libc::c_int) as libc::c_longlong
                    || p == -(MARK_NUMBER as libc::c_int) as libc::c_longlong))
        {
            p = *m.offset(i as isize);
            c += 1;
            c;
        }
        i += 1;
        i;
    }
    return c;
}
unsafe extern "C" fn syntaxChk(mut s: S) -> I {
    if *s.offset(0 as libc::c_int as isize) as libc::c_int == '\t' as i32
        || *s.offset(0 as libc::c_int as isize) as libc::c_int == '\u{c}' as i32
    {
        return 5 as libc::c_int as I;
    }
    let mut n: I = strlen(s as *const libc::c_char) as I;
    if n == 1 as libc::c_int as libc::c_longlong {
        if *s.offset(0 as libc::c_int as isize) as libc::c_int == '\'' as i32 {
            return 10 as libc::c_int as I
        } else {
            return 0 as libc::c_int as I
        }
    }
    let mut i: I = 0;
    let mut j: I = 0;
    let mut k: I = 0 as libc::c_int as I;
    i = 0 as libc::c_int as I;
    while i < n {
        if *s.offset(i as isize) as libc::c_int != ' ' as i32 {
            break;
        }
        i += 1;
        i;
    }
    if i >= n - 1 as libc::c_int as libc::c_longlong {
        return 0 as libc::c_int as I;
    }
    j = i + 1 as libc::c_int as libc::c_longlong;
    while j < n {
        if *s.offset(j as isize) as libc::c_int != ' ' as i32 {
            break;
        }
        j += 1;
        j;
    }
    if *s.offset(i as isize) as libc::c_int == '\\' as i32
        && *s.offset(j as isize) as libc::c_int == '\\' as i32
    {
        return 0 as libc::c_int as I;
    }
    if *s.offset(i as isize) as libc::c_int == '\'' as i32
        && *s.offset(j as isize) as libc::c_int != '"' as i32 || j == n
    {
        return 20 as libc::c_int as I;
    }
    i = 0 as libc::c_int as I;
    while i < n {
        if *s.offset(i as isize) as libc::c_int == '"' as i32 {
            break;
        }
        if i > 0 as libc::c_int as libc::c_longlong
            && (*s.offset(i as isize) as libc::c_int == '\u{b}' as i32
                || *s.offset(i as isize) as libc::c_int == '\u{c}' as i32
                || *s.offset(i as isize) as libc::c_int == '\'' as i32
                    && *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int == ';' as i32)
            || i > 1 as libc::c_int as libc::c_longlong
                && *s.offset(i as isize) as libc::c_int == '\'' as i32
                && *s.offset((i - 2 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '\\' as i32
        {
            return 30 as libc::c_int as I;
        }
        i += 1;
        i;
    }
    if n > 1 as libc::c_int as libc::c_longlong
        && *s.offset((n - 1 as libc::c_int as libc::c_longlong) as isize) as libc::c_int
            == '\'' as i32
        && *s.offset((n - 2 as libc::c_int as libc::c_longlong) as isize) as libc::c_int
            == ':' as i32
    {
        return 50 as libc::c_int as I;
    }
    if n > 1 as libc::c_int as libc::c_longlong {
        i = 1 as libc::c_int as I;
        while i < n {
            if *s.offset(i as isize) as libc::c_int == '"' as i32 {
                break;
            }
            if *s.offset(i as isize) as libc::c_int == ':' as i32
                && *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '`' as i32
            {
                return 55 as libc::c_int as I;
            }
            if *s.offset(i as isize) as libc::c_int == ',' as i32
                && (*s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '\\' as i32
                    || *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int == '_' as i32)
            {
                return 60 as libc::c_int as I;
            }
            if *s.offset(i as isize) as libc::c_int == '?' as i32
                && (*s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '-' as i32
                    || *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int == '\\' as i32)
            {
                return 70 as libc::c_int as I;
            }
            i += 1;
            i;
        }
    }
    if n > 2 as libc::c_int as libc::c_longlong {
        i = 2 as libc::c_int as I;
        while i < n {
            if *s.offset(i as isize) as libc::c_int == '\\' as i32
                && *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == ':' as i32
                && (*s.offset((i - 2 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int != '/' as i32
                    && *s.offset((i - 2 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int != '\\' as i32)
                && *s.offset((i + 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int != 'n' as i32
            {
                return 80 as libc::c_int as I;
            }
            if *s.offset(i as isize) as libc::c_int == '/' as i32
                && (*s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '+' as i32
                    || *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int == '\'' as i32
                    || *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int == '>' as i32
                    || *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int == '%' as i32
                    || *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int == '*' as i32
                    || *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int == '?' as i32
                    || *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int == '&' as i32
                    || *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int == '\\' as i32)
                && *s.offset((i - 2 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '/' as i32
            {
                return 90 as libc::c_int as I;
            }
            if *s.offset(i as isize) as libc::c_int == '/' as i32
                && *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '/' as i32
                && *s.offset((i - 2 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '-' as i32
            {
                return 100 as libc::c_int as I;
            }
            if *s.offset(i as isize) as libc::c_int == '/' as i32
                && *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '/' as i32
                && *s.offset((i + 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == ',' as i32
            {
                return 999 as libc::c_int as I;
            }
            if *s.offset(i as isize) as libc::c_int == '_' as i32
                && *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == ',' as i32
                && *s.offset((i - 2 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '~' as i32
            {
                return 110 as libc::c_int as I;
            }
            if *s.offset(i as isize) as libc::c_int == '/' as i32
                && *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '#' as i32
                && *s.offset((i - 2 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '0' as i32
            {
                return 120 as libc::c_int as I;
            }
            if *s.offset(i as isize) as libc::c_int == '\\' as i32
                && *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '\\' as i32
                && *s.offset((i - 2 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '<' as i32
            {
                return 123 as libc::c_int as I;
            }
            if *s.offset(i as isize) as libc::c_int == '$' as i32
                && *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == ',' as i32
                && *s.offset((i - 2 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '$' as i32
            {
                return 130 as libc::c_int as I;
            }
            if *s.offset(i as isize) as libc::c_int == ':' as i32
                && *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '0' as i32
                && *s.offset((i - 2 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '0' as i32
            {
                return 136 as libc::c_int as I;
            }
            if *s.offset(i as isize) as libc::c_int == '/' as i32
                && *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '/' as i32
                && *s.offset((i - 2 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '/' as i32
            {
                return 140 as libc::c_int as I;
            }
            i += 1;
            i;
        }
    }
    if n > 3 as libc::c_int as libc::c_longlong {
        i = 3 as libc::c_int as I;
        while i < n {
            if *s.offset(i as isize) as libc::c_int == '\\' as i32
                && *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == ' ' as i32
                && (*s.offset((i - 2 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '\\' as i32
                    || *s.offset((i - 2 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int == '/' as i32)
                && *s.offset((i - 3 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '\\' as i32
            {
                return 141 as libc::c_int as I;
            }
            if (*(*__ctype_b_loc()).offset(*s.offset(i as isize) as libc::c_int as isize)
                as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                != 0 || *s.offset(i as isize) as libc::c_int == '`' as i32)
                && *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == ':' as i32
                && *s.offset((i - 2 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == ':' as i32
                && *s.offset((i - 3 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == ':' as i32
            {
                return 142 as libc::c_int as I;
            }
            i += 1;
            i;
        }
    }
    if n > 3 as libc::c_int as libc::c_longlong {
        i = 2 as libc::c_int as I;
        while i < n - 1 as libc::c_int as libc::c_longlong {
            if *s.offset(i as isize) as libc::c_int == '/' as i32
                && *s.offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == ':' as i32
                && *s.offset((i - 2 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int == '/' as i32
                && *s.offset((i + 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int != ':' as i32
            {
                return 150 as libc::c_int as I;
            }
            i += 1;
            i;
        }
    }
    return k;
}
pub unsafe extern "C" fn mark(mut m: *mut I, mut k: I, mut t: I) -> I {
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = k;
    while i < _i {
        *m.offset(i as isize) = if i != 0 { t } else { -t };
        i += 1;
        i;
    }
    return k;
}
pub unsafe extern "C" fn wd(mut s: S, mut n: libc::c_int) -> K {
    lineA = s;
    fdc = 0 as libc::c_int as I;
    return wd_(s, n, denameD(&mut KTREE, d_, 1 as libc::c_int as I), 0 as K);
}
pub unsafe extern "C" fn wd_(
    mut s: S,
    mut n: libc::c_int,
    mut dict: *mut K,
    mut func: K,
) -> K {
    if s.is_null() {
        return 0 as K;
    }
    if !(strstr(s as *const libc::c_char, b":\\t\0" as *const u8 as *const libc::c_char))
        .is_null()
    {
        show(kerr(b"\\t  syntax\0" as *const u8 as *const libc::c_char));
        return 0 as K;
    }
    let mut z: I = syntaxChk(s);
    if z == 999 as libc::c_int as libc::c_longlong {
        return kerr(b"nonce\0" as *const u8 as *const libc::c_char);
    }
    if z != 0 {
        return kerr(b"syntax\0" as *const u8 as *const libc::c_char);
    }
    let mut i: I = 0 as libc::c_int as I;
    while i < n as libc::c_longlong
        && *(*__ctype_b_loc()).offset(*s.offset(i as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        i += 1;
        i;
    }
    if '\\' as i32 == *s.offset(i as isize) as libc::c_int {
        return backslash(s.offset(i as isize), n as I, dict);
    }
    let mut p: PDA = 0 as PDA;
    let mut km: K = newK(-(1 as libc::c_int) as I, (1 as libc::c_int + n) as I);
    if km.is_null() {
        return 0 as K;
    }
    let mut m: *mut I = ((*km).k).as_mut_ptr() as *mut I;
    let mut e: I = complete(s, n as I, &mut p, m);
    if !p.is_null() {
        pdafree(p);
        p = 0 as PDA;
    }
    lineB = s;
    if e != 0 {
        cd(km);
        return kerr(b"parse\0" as *const u8 as *const libc::c_char);
    }
    let mut v: K = Kv();
    if OOM_CD(0 as libc::c_int as I, v, km, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    (*v).n = 0 as libc::c_int as I;
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i: I = n as I;
    while i_0 < _i {
        i_0
            += maX(
                0 as libc::c_int as I,
                -(1 as libc::c_int) as libc::c_longlong
                    + mark(
                        m.offset(i_0 as isize),
                        mark_end(s, n as I, i_0, m),
                        MARK_END as libc::c_int as I,
                    ),
            );
        i_0 += 1;
        i_0;
    }
    let mut i_1: I = 0 as libc::c_int as I;
    let mut _i_0: I = n as I;
    while i_1 < _i_0 {
        i_1
            += maX(
                0 as libc::c_int as I,
                -(1 as libc::c_int) as libc::c_longlong
                    + mark(
                        m.offset(i_1 as isize),
                        mark_symbol(s, n as I, i_1, m),
                        MARK_SYMBOL as libc::c_int as I,
                    ),
            );
        i_1 += 1;
        i_1;
    }
    let mut i_2: I = 0 as libc::c_int as I;
    let mut _i_1: I = n as I;
    while i_2 < _i_1 {
        i_2
            += maX(
                0 as libc::c_int as I,
                -(1 as libc::c_int) as libc::c_longlong
                    + mark(
                        m.offset(i_2 as isize),
                        mark_name(s, n as I, i_2, m),
                        MARK_NAME as libc::c_int as I,
                    ),
            );
        i_2 += 1;
        i_2;
    }
    let mut i_3: I = 0 as libc::c_int as I;
    let mut _i_2: I = n as I;
    while i_3 < _i_2 {
        i_3
            += maX(
                0 as libc::c_int as I,
                -(1 as libc::c_int) as libc::c_longlong
                    + mark(
                        m.offset(i_3 as isize),
                        mark_number(s, n as I, i_3, m),
                        MARK_NUMBER as libc::c_int as I,
                    ),
            );
        i_3 += 1;
        i_3;
    }
    let mut i_4: I = 0 as libc::c_int as I;
    let mut _i_3: I = n as I;
    while i_4 < _i_3 {
        i_4
            += maX(
                0 as libc::c_int as I,
                -(1 as libc::c_int) as libc::c_longlong
                    + mark(
                        m.offset(i_4 as isize),
                        mark_adverb(s, n as I, i_4, m),
                        MARK_ADVERB as libc::c_int as I,
                    ),
            );
        i_4 += 1;
        i_4;
    }
    let mut i_5: I = 0 as libc::c_int as I;
    let mut _i_4: I = n as I;
    while i_5 < _i_4 {
        i_5
            += maX(
                0 as libc::c_int as I,
                -(1 as libc::c_int) as libc::c_longlong
                    + mark(
                        m.offset(i_5 as isize),
                        mark_conditional(s, n as I, i_5, m),
                        MARK_CONDITIONAL as libc::c_int as I,
                    ),
            );
        i_5 += 1;
        i_5;
    }
    let mut i_6: I = 0 as libc::c_int as I;
    let mut _i_5: I = n as I;
    while i_6 < _i_5 {
        i_6
            += maX(
                0 as libc::c_int as I,
                -(1 as libc::c_int) as libc::c_longlong
                    + mark(
                        m.offset(i_6 as isize),
                        mark_verb(s, n as I, i_6, m),
                        MARK_VERB as libc::c_int as I,
                    ),
            );
        i_6 += 1;
        i_6;
    }
    let mut i_7: I = 0 as libc::c_int as I;
    let mut _i_6: I = n as I;
    while i_7 < _i_6 {
        i_7
            += maX(
                0 as libc::c_int as I,
                -(1 as libc::c_int) as libc::c_longlong
                    + mark(
                        m.offset(i_7 as isize),
                        mark_ignore(s, n as I, i_7, m),
                        MARK_IGNORE as libc::c_int as I,
                    ),
            );
        i_7 += 1;
        i_7;
    }
    let mut i_8: I = 0 as libc::c_int as I;
    let mut _i_7: I = n as I;
    while i_8 < _i_7 {
        if *m.offset(i_8 as isize) == MARK_UNMARKED as libc::c_int as libc::c_longlong {
            cd(v);
            cd(km);
            return kerr(b"parse\0" as *const u8 as *const libc::c_char);
        }
        i_8 += 1;
        i_8;
    }
    let mut y: I = 0 as libc::c_int as I;
    let mut ks2: K = newK(-(3 as libc::c_int) as I, n as I);
    if OOM_CD(0 as libc::c_int as I, v, km, ks2, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut s2: S = ((*ks2).k).as_mut_ptr() as S;
    memcpy(s2 as *mut libc::c_void, s as *const libc::c_void, n as libc::c_ulong);
    let mut i_9: I = 0 as libc::c_int as I;
    let mut _i_8: I = n as I;
    while i_9 < _i_8 {
        if (if *m.offset(i_9 as isize) < 0 as libc::c_int as libc::c_longlong {
            -*m.offset(i_9 as isize)
        } else {
            *m.offset(i_9 as isize)
        }) > MARK_IGNORE as libc::c_int as libc::c_longlong
        {
            *m.offset(y as isize) = *m.offset(i_9 as isize);
            *s2.offset(y as isize) = *s2.offset(i_9 as isize);
            y += 1;
            y;
        }
        i_9 += 1;
        i_9;
    }
    let ref mut fresh1 = *m.offset(y as isize);
    *fresh1 = 0 as libc::c_int as I;
    *s2.offset(y as isize) = *fresh1 as C;
    let mut oc: I = overcount(m, n as I);
    let mut kw: K = newK(
        -(4 as libc::c_int) as I,
        1 as libc::c_int as libc::c_longlong + oc,
    );
    if OOM_CD(0 as libc::c_int as I, v, km, ks2, kw, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut w: *mut V = ((*kw).k).as_mut_ptr() as *mut V;
    let mut c: I = 0 as libc::c_int as I;
    let mut j: I = 0 as libc::c_int as I;
    if fll == 0 {
        fll = strlen(s2 as *const libc::c_char) as I;
    } else {
        fll = -(1 as libc::c_int) as I;
    }
    let mut i_10: I = 0 as libc::c_int as I;
    let mut _i_9: I = y;
    while i_10 < _i_9 {
        j = capture(
            s2,
            y,
            i_10,
            m,
            w,
            &mut c,
            (((*v).k).as_mut_ptr() as *mut V as *mut K)
                .offset(LOCALS as libc::c_int as isize),
            dict,
            func,
        );
        i_10 += -(1 as libc::c_int) as libc::c_longlong + j;
        if j == 0 {
            if OOM_CD(
                0 as libc::c_int as I,
                0 as libc::c_int,
                v,
                km,
                ks2,
                kw,
                -(1 as libc::c_int) as V,
            ) == 0
            {
                return 0 as K;
            }
        }
        i_10 += 1;
        i_10;
    }
    cd(km);
    cd(ks2);
    if oc > c
        && lsz(sz(0 as libc::c_int as I, 1 as libc::c_int as libc::c_longlong + oc))
            > lsz(sz(0 as libc::c_int as I, 1 as libc::c_int as libc::c_longlong + c))
    {
        let mut kw2: K = newK(
            -(4 as libc::c_int) as I,
            1 as libc::c_int as libc::c_longlong + c,
        );
        if OOM_CD(0 as libc::c_int as I, v, kw, kw2, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        memcpy(
            ((*kw2).k).as_mut_ptr() as *mut libc::c_void,
            ((*kw).k).as_mut_ptr() as *const libc::c_void,
            (::std::mem::size_of::<V>() as libc::c_ulong as libc::c_ulonglong)
                .wrapping_mul(c as libc::c_ulonglong) as libc::c_ulong,
        );
        cd(kw);
        kw = kw2;
    }
    let ref mut fresh2 = *(((*v).k).as_mut_ptr() as *mut V)
        .offset(CODE as libc::c_int as isize);
    *fresh2 = kw as V;
    let ref mut fresh3 = *(((*(*(((*v).k).as_mut_ptr() as *mut V)
        .offset(CODE as libc::c_int as isize) as K))
        .k)
        .as_mut_ptr() as *mut S as *mut V)
        .offset(c as isize);
    *fresh3 = 0 as V;
    return v;
}
unsafe extern "C" fn isodigit(mut c: C) -> I {
    return (*(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        && (c as libc::c_int) < '8' as i32) as libc::c_int as I;
}
unsafe extern "C" fn odigitlen3(mut s: S) -> I {
    let mut i: I = 0 as libc::c_int as I;
    while *s.offset(i as isize) as libc::c_int != 0
        && isodigit(*s.offset(i as isize)) != 0
        && i < 3 as libc::c_int as libc::c_longlong
    {
        i += 1;
        i;
    }
    return i;
}
unsafe extern "C" fn unescape(mut s: S, mut k: *mut I) -> C {
    *k = 1 as libc::c_int as I;
    let mut c: C = *s;
    if '\\' as i32 != c as libc::c_int {
        return c;
    }
    let mut y: I = odigitlen3(s.offset(1 as libc::c_int as isize));
    let mut a: I = 0 as libc::c_int as I;
    if y == 0 {
        *k = 2 as libc::c_int as I;
        c = *s.offset(1 as libc::c_int as isize);
        return (if 'b' as i32 == c as libc::c_int {
            '\u{8}' as i32
        } else {
            (if 'n' as i32 == c as libc::c_int {
                '\n' as i32
            } else {
                (if 'r' as i32 == c as libc::c_int {
                    '\r' as i32
                } else {
                    (if 't' as i32 == c as libc::c_int {
                        '\t' as i32
                    } else {
                        c as libc::c_int
                    })
                })
            })
        }) as C;
    }
    *k += y;
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = y;
    while i < _i {
        a = a * 8 as libc::c_int as libc::c_longlong
            + *s.offset((1 as libc::c_int as libc::c_longlong + i) as isize)
                as libc::c_longlong - '0' as i32 as libc::c_longlong;
        i += 1;
        i;
    }
    return (if a < 255 as libc::c_int as libc::c_longlong {
        a
    } else {
        255 as libc::c_int as libc::c_longlong
    }) as UC as C;
}
unsafe extern "C" fn unescaped_size(mut s: S, mut n: I) -> I {
    let mut k: I = 0 as libc::c_int as I;
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = n;
    while i < _i {
        k += 1;
        k;
        if '\\' as i32 == *s.offset(i as isize) as libc::c_int {
            i
                += maX(
                    1 as libc::c_int as I,
                    odigitlen3(s.offset(i as isize).offset(1 as libc::c_int as isize)),
                );
        }
        i += 1;
        i;
    }
    return k;
}
unsafe extern "C" fn unescaped_fill(mut d: S, mut s: S, mut n: I) -> I {
    let mut k: I = 0 as libc::c_int as I;
    let mut q: I = 0;
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = n;
    while i < _i {
        let fresh4 = k;
        k = k + 1;
        *d.offset(fresh4 as isize) = unescape(s.offset(i as isize), &mut q);
        i += q - 1 as libc::c_int as libc::c_longlong;
        i += 1;
        i;
    }
    return k;
}
pub static mut param_dfa: S = b"155045113245553245155355\0" as *const u8
    as *const libc::c_char as S;
unsafe extern "C" fn param_gp(mut c: C) -> I {
    return (if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
        & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        0 as libc::c_int
    } else if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        || '_' as i32 == c as libc::c_int
    {
        1 as libc::c_int
    } else if ';' as i32 == c as libc::c_int {
        2 as libc::c_int
    } else if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        3 as libc::c_int
    } else if ']' as i32 == c as libc::c_int {
        4 as libc::c_int
    } else {
        5 as libc::c_int
    }) as I;
}
unsafe extern "C" fn param_validate(mut s: S, mut n: I) -> I {
    let mut u: S = s.offset(n as isize);
    while s < u
        && *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        && '\n' as i32 != *s as libc::c_int
    {
        s = s.offset(1);
        s;
    }
    if s == u
        || {
            let fresh5 = s;
            s = s.offset(1);
            '[' as i32 != *fresh5 as libc::c_int
        }
    {
        return 0 as libc::c_int as I;
    }
    let mut p: I = 0 as libc::c_int as I;
    let mut r: I = param_gp('.' as i32 as C) + 1 as libc::c_int as libc::c_longlong;
    while s < u && 4 as libc::c_int as libc::c_longlong > p {
        let fresh6 = s;
        s = s.offset(1);
        p = (*param_dfa.offset((r * p + param_gp(*fresh6)) as isize) as libc::c_int
            - '0' as i32) as I;
    }
    return (if 4 as libc::c_int as libc::c_longlong == p {
        1 as libc::c_int
    } else {
        2 as libc::c_int
    }) as I;
}
unsafe extern "C" fn inKtreeR(mut p: *mut K, mut t: S, mut create: I) -> *mut K {
    if *t == 0 {
        return p;
    }
    if '.' as i32 == *t as libc::c_int {
        t = t.offset(1);
        t;
    }
    let mut c: I = 0 as libc::c_int as I;
    let mut a: I = (**p).t;
    while *t.offset(c as isize) as libc::c_int != 0
        && '.' as i32 != *t.offset(c as isize) as libc::c_int
    {
        c += 1;
        c;
    }
    let mut u: S = strdupn(t, c);
    let mut k: S = sp(u);
    free(u as *mut libc::c_void);
    t = t.offset(c as isize);
    if '_' as i32 == *k as libc::c_int {
        return kerr(b"reserved\0" as *const u8 as *const libc::c_char) as *mut K;
    }
    if !(6 as libc::c_int as libc::c_longlong == a
        || 5 as libc::c_int as libc::c_longlong == a)
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char) as *mut K;
    }
    let mut e: K = 0 as K;
    if create != 0 {
        e = lookupEntryOrCreate(p, k);
        if e.is_null() {
            return kerr(b"wsfull\0" as *const u8 as *const libc::c_char) as *mut K;
        }
    } else {
        let mut a_0: K = *p;
        if 5 as libc::c_int as libc::c_longlong == (*a_0).t {
            e = DE(a_0, k);
        }
        if e.is_null() {
            return 0 as *mut K;
        }
    }
    if '.' as i32 == *t as libc::c_int
        && (*t.offset(1 as libc::c_int as isize) == 0
            || '.' as i32 == *t.offset(1 as libc::c_int as isize) as libc::c_int)
    {
        t = t.offset(1);
        t;
        p = EAP(e);
    } else {
        p = EVP(e);
    }
    return inKtreeR(p, t, create);
}
pub unsafe extern "C" fn inKtree(mut d: *mut K, mut t: S, mut create: I) -> *mut K {
    if simpleString(t) == 0 {
        return 0 as *mut K;
    }
    return inKtreeR(
        if '.' as i32 == *t as libc::c_int || *t == 0 { &mut KTREE } else { d },
        t,
        create,
    );
}
pub unsafe extern "C" fn capture(
    mut s: S,
    mut n: I,
    mut k: I,
    mut m: *mut I,
    mut w: *mut V,
    mut d: *mut I,
    mut locals: *mut K,
    mut dict: *mut K,
    mut func: K,
) -> I {
    if fll != 0 && fll != n {
        fll = -(1 as libc::c_int) as I;
    }
    let mut z: V = 0 as V;
    let mut p: *mut V = w.offset(*d as isize);
    let mut r: I = 1 as libc::c_int as I;
    let mut v: I = 0 as libc::c_int as I;
    let mut y: I = 0 as libc::c_int as I;
    let mut a: I = 0;
    let mut b: I = 0 as libc::c_int as I;
    let mut c: I = 0;
    let mut l: I = 0;
    let mut frc: I = 0 as libc::c_int as I;
    let mut u: S = b"\0" as *const u8 as *const libc::c_char as S;
    let mut e: S = 0 as *mut C;
    let mut g: K = 0 as *mut k0;
    let mut h: K = 0 as K;
    let mut hh: K = 0 as K;
    if k >= n
        || !(*m.offset(k as isize) < -(MARK_IGNORE as libc::c_int) as libc::c_longlong)
    {
        return r;
    }
    let mut M: I = *m.offset(k as isize);
    while k + r < n && -M == *m.offset((k + r) as isize) {
        r += 1;
        r;
    }
    if M == -(MARK_SYMBOL as libc::c_int) as libc::c_longlong
        || M == -(MARK_NUMBER as libc::c_int) as libc::c_longlong
    {
        while k + v < n
            && -M
                == (if *m.offset((k + v) as isize) < 0 as libc::c_int as libc::c_longlong
                {
                    -*m.offset((k + v) as isize)
                } else {
                    *m.offset((k + v) as isize)
                })
        {
            let fresh7 = v;
            v = v + 1;
            if *m.offset((k + fresh7) as isize)
                <= -(MARK_END as libc::c_int) as libc::c_longlong
            {
                y += 1;
                y;
            }
        }
    }
    let mut ko: K = 0 as *mut k0;
    let mut o: *mut V = 0 as *mut V;
    let mut f: *mut K = 0 as *mut K;
    let mut zdict: *mut K = 0 as *mut K;
    let mut ydict: *mut K = 0 as *mut K;
    let mut j: K = 0 as *mut k0;
    let mut state: I = 0;
    let mut q_0: K = 0 as *mut k0;
    let mut grab: I = 0;
    let mut i_7: L = 0;
    let mut is_colon: I = 0;
    let mut name_bracket_assign: I = 0;
    let mut modifier_colon: I = 0;
    let mut y_present: I = 0;
    match -M {
        12 => {
            z = offsetColon;
        }
        4 => {
            fbr = 1 as libc::c_int as I;
            fdc = 1 as libc::c_int as I;
            z = wd_(
                s.offset(k as isize).offset(1 as libc::c_int as isize),
                (r - 2 as libc::c_int as libc::c_longlong) as libc::c_int,
                dict,
                func,
            ) as V;
            if z.is_null() {
                return kerr(b"parse\0" as *const u8 as *const libc::c_char) as L;
            }
        }
        2 => {
            fbr = 1 as libc::c_int as I;
            if *d == 0 || bk(*p.offset(-(1 as libc::c_int) as isize)) != 0 {
                if !func.is_null() && k == 0 {
                    return r
                } else {
                    return kerr(b"parse\0" as *const u8 as *const libc::c_char) as L
                }
            }
            a = 0 as libc::c_int as I;
            while a < -(1 as libc::c_int) as libc::c_longlong + *d
                && adverbClass(
                    *p.offset((-(1 as libc::c_int) as libc::c_longlong - a) as isize),
                ) != 0
            {
                a += 1;
                a;
            }
            g = Kv();
            ko = newK(
                -(4 as libc::c_int) as I,
                a + 2 as libc::c_int as libc::c_longlong,
            );
            if OOM_CD(0 as libc::c_int as I, g, ko, -(1 as libc::c_int) as V) == 0 {
                return 0 as libc::c_int as I;
            }
            (*g).n = 0 as libc::c_int as I;
            let ref mut fresh8 = *(((*g).k).as_mut_ptr() as *mut V)
                .offset(CODE as libc::c_int as isize);
            *fresh8 = ko as V;
            o = ((*(*(((*g).k).as_mut_ptr() as *mut V)
                .offset(CODE as libc::c_int as isize) as K))
                .k)
                .as_mut_ptr() as *mut S as *mut V;
            if *s
                .offset(k as isize)
                .offset(1 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as libc::c_int == 'x' as i32
                && *s
                    .offset(k as isize)
                    .offset(1 as libc::c_int as isize)
                    .offset(1 as libc::c_int as isize) as libc::c_int == ';' as i32
            {
                fbr = 0 as libc::c_int as I;
            }
            z = wd_(
                s.offset(k as isize).offset(1 as libc::c_int as isize),
                (r - 2 as libc::c_int as libc::c_longlong) as libc::c_int,
                dict,
                func,
            ) as V;
            if z.is_null() {
                cd(g);
                return kerr(b"parse\0" as *const u8 as *const libc::c_char) as L;
            }
            if MARK_CONDITIONAL as libc::c_int as libc::c_longlong
                == (if *m.offset((k - 1 as libc::c_int as libc::c_longlong) as isize)
                    < 0 as libc::c_int as libc::c_longlong
                {
                    -*m.offset((k - 1 as libc::c_int as libc::c_longlong) as isize)
                } else {
                    *m.offset((k - 1 as libc::c_int as libc::c_longlong) as isize)
                })
            {
                let mut zp: *mut V = ((*(*(((*(z as K)).k).as_mut_ptr() as *mut V)
                    .offset(CODE as libc::c_int as isize) as K))
                    .k)
                    .as_mut_ptr() as *mut S as *mut V;
                while !(*zp).is_null() && bk(*zp) == 0 {
                    zp = zp.offset(1);
                    zp;
                }
                match *s.offset((k - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int
                {
                    58 => {
                        b = 4 as libc::c_int as I;
                    }
                    102 => {
                        b = 5 as libc::c_int as I;
                    }
                    101 => {
                        b = 6 as libc::c_int as I;
                    }
                    111 => {
                        b = 7 as libc::c_int as I;
                    }
                    _ => {}
                }
                (*(z as K)).n = b;
                cd(g);
            } else {
                let mut i: I = 0 as libc::c_int as I;
                let mut _i: I = a + 1 as libc::c_int as libc::c_longlong;
                while i < _i {
                    let ref mut fresh9 = *o.offset(i as isize);
                    *fresh9 = *p
                        .offset(
                            (-(1 as libc::c_int) as libc::c_longlong - a + i) as isize,
                        );
                    i += 1;
                    i;
                }
                let ref mut fresh10 = *o
                    .offset((a + 1 as libc::c_int as libc::c_longlong) as isize);
                *fresh10 = 0 as V;
                f = *p.offset((-(1 as libc::c_int) as libc::c_longlong - a) as isize)
                    as *mut K;
                if sva(f as V) == 0 {
                    if !(MARK_ADVERB as libc::c_int as libc::c_longlong
                        == (if *m
                            .offset(
                                (k - 1 as libc::c_int as libc::c_longlong - a) as isize,
                            ) < 0 as libc::c_int as libc::c_longlong
                        {
                            -*m
                                .offset(
                                    (k - 1 as libc::c_int as libc::c_longlong - a) as isize,
                                )
                        } else {
                            *m
                                .offset(
                                    (k - 1 as libc::c_int as libc::c_longlong - a) as isize,
                                )
                        })
                        || MARK_ADVERB as libc::c_int as libc::c_longlong
                            == (if *m.offset((k - a) as isize)
                                < 0 as libc::c_int as libc::c_longlong
                            {
                                -*m.offset((k - a) as isize)
                            } else {
                                *m.offset((k - a) as isize)
                            }))
                    {
                        if MARK_NAME as libc::c_int as libc::c_longlong
                            != (if *m
                                .offset(
                                    (k - 1 as libc::c_int as libc::c_longlong - a) as isize,
                                ) < 0 as libc::c_int as libc::c_longlong
                            {
                                -*m
                                    .offset(
                                        (k - 1 as libc::c_int as libc::c_longlong - a) as isize,
                                    )
                            } else {
                                *m
                                    .offset(
                                        (k - 1 as libc::c_int as libc::c_longlong - a) as isize,
                                    )
                            })
                        {
                            let mut q: K = newE(LS, ci(*f));
                            kap(
                                (((*g).k).as_mut_ptr() as *mut V as *mut K)
                                    .offset(LOCALS as libc::c_int as isize),
                                &mut q as *mut K as V,
                            );
                            cd(q);
                            let mut temp: K = DI(
                                *locals,
                                (**locals).n - 1 as libc::c_int as libc::c_longlong,
                            );
                            if !temp.is_null() {
                                cd(
                                    *((*temp).k).as_mut_ptr().offset(1 as libc::c_int as isize),
                                );
                                let ref mut fresh11 = *((*temp).k)
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize);
                                *fresh11 = 0 as *mut k0;
                            }
                            *o = EVP(q) as V;
                        } else {
                            7 as libc::c_int as libc::c_longlong == (**f).t
                                && 3 as libc::c_int as libc::c_longlong == (**f).n;
                        }
                    }
                }
                let ref mut fresh12 = *(((*g).k).as_mut_ptr() as *mut V)
                    .offset(CONJ as libc::c_int as isize);
                *fresh12 = z;
                z = g as V;
            }
            *d -= 1 as libc::c_int as libc::c_longlong + a;
            p = w.offset(*d as isize);
        }
        5 => {
            fbr = 1 as libc::c_int as I;
            fdc = 1 as libc::c_int as I;
            z = Kv() as V;
            g = newK(-(3 as libc::c_int) as I, r - 2 as libc::c_int as libc::c_longlong);
            if OOM_CD(0 as libc::c_int as I, z, g, -(1 as libc::c_int) as V) == 0 {
                return 0 as libc::c_int as I;
            }
            let ref mut fresh13 = *(((*(z as K)).k).as_mut_ptr() as *mut V)
                .offset(CODE as libc::c_int as isize);
            *fresh13 = g as V;
            (*(z as K)).n = 3 as libc::c_int as I;
            memcpy(
                ((*g).k).as_mut_ptr() as *mut C as *mut libc::c_void,
                s.offset(k as isize).offset(1 as libc::c_int as isize)
                    as *const libc::c_void,
                (r - 2 as libc::c_int as libc::c_longlong) as libc::c_ulong,
            );
            let ref mut fresh14 = *(((*(z as K)).k).as_mut_ptr() as *mut V)
                .offset(CONTeXT as libc::c_int as isize);
            *fresh14 = if !func.is_null() {
                *(((*func).k).as_mut_ptr() as *mut V)
                    .offset(CONTeXT as libc::c_int as isize)
            } else {
                d_ as *mut libc::c_void
            };
            zdict = (((*(z as K)).k).as_mut_ptr() as *mut V as *mut K)
                .offset(PARAMS as libc::c_int as isize);
            ydict = (((*(z as K)).k).as_mut_ptr() as *mut V as *mut K)
                .offset(LOCALS as libc::c_int as isize);
            j = 0 as *mut k0;
            state = param_validate(
                s.offset(k as isize).offset(1 as libc::c_int as isize),
                r - 2 as libc::c_int as libc::c_longlong,
            );
            if state > 1 as libc::c_int as libc::c_longlong {
                return kerr(b"parameter\0" as *const u8 as *const libc::c_char) as L;
            }
            if state != 0 {
                let mut a_0: S = strchr(
                    s.offset(k as isize).offset(1 as libc::c_int as isize)
                        as *const libc::c_char,
                    '[' as i32,
                );
                let mut b_0: S = strchr(a_0 as *const libc::c_char, ']' as i32);
                j = wd_(
                    a_0.offset(1 as libc::c_int as isize),
                    (b_0.offset_from(a_0) as libc::c_long
                        - 1 as libc::c_int as libc::c_long) as libc::c_int,
                    zdict,
                    z as K,
                );
                if OOM_CD(0 as libc::c_int as I, z, j, -(1 as libc::c_int) as V) == 0 {
                    return 0 as libc::c_int as I;
                }
                cd(j);
            } else {
                let mut t: K = Kd();
                if OOM_CD(0 as libc::c_int as I, z, t, -(1 as libc::c_int) as V) == 0 {
                    return 0 as libc::c_int as I;
                }
                j = wd_(
                    s.offset(k as isize).offset(1 as libc::c_int as isize),
                    (r - 2 as libc::c_int as libc::c_longlong) as libc::c_int,
                    &mut t,
                    0 as K,
                );
                if OOM_CD(0 as libc::c_int as I, z, t, j, -(1 as libc::c_int) as V) == 0
                {
                    return 0 as libc::c_int as I;
                }
                let mut n_0: I = 0 as libc::c_int as I;
                let mut i_0: I = 0 as libc::c_int as I;
                let mut _i_0: I = 3 as libc::c_int as I;
                while i_0 < _i_0 {
                    if !(DE(
                        t,
                        IFP[(2 as libc::c_int as libc::c_longlong - i_0) as usize],
                    ))
                        .is_null()
                    {
                        n_0 = 3 as libc::c_int as libc::c_longlong - i_0;
                        break;
                    } else {
                        i_0 += 1;
                        i_0;
                    }
                }
                let mut i_1: I = 0 as libc::c_int as I;
                let mut _i_1: I = n_0;
                while i_1 < _i_1 {
                    denameD(zdict, IFP[i_1 as usize], 1 as libc::c_int as I);
                    i_1 += 1;
                    i_1;
                }
                cd(t);
                cd(j);
            }
            j = wd_(
                s.offset(k as isize).offset(1 as libc::c_int as isize),
                (r - 2 as libc::c_int as libc::c_longlong) as libc::c_int,
                ydict,
                z as K,
            );
            if OOM_CD(0 as libc::c_int as I, z, j, -(1 as libc::c_int) as V) == 0 {
                return 0 as libc::c_int as I;
            }
            cd(j);
        }
        9 => {
            r = v;
            a = 1 as libc::c_int as I;
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = r;
            while i_2 < _i_2 {
                if stringHasChar(
                    b".Eein\0" as *const u8 as *const libc::c_char as S,
                    *s.offset((k + i_2) as isize),
                ) != 0
                {
                    a = 2 as libc::c_int as I;
                    break;
                } else {
                    i_2 += 1;
                    i_2;
                }
            }
            z = newK(if 1 as libc::c_int as libc::c_longlong == y { a } else { -a }, y)
                as V;
            if z.is_null() {
                return 0 as libc::c_int as I;
            }
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = r;
            while i_3 < _i_3 {
                if !(*m.offset((k + i_3) as isize)
                    >= 0 as libc::c_int as libc::c_longlong)
                {
                    l = 1 as libc::c_int as I;
                    while *m.offset((l + k + i_3) as isize)
                        == MARK_NUMBER as libc::c_int as libc::c_longlong
                    {
                        l += 1;
                        l;
                    }
                    u = strdupn(s.offset(k as isize).offset(i_3 as isize), l);
                    if u.is_null() {
                        cd(z as K);
                        return kerr(b"wsfull\0" as *const u8 as *const libc::c_char)
                            as L;
                    }
                    g = if 1 as libc::c_int as libc::c_longlong == a {
                        formKiCS(u)
                    } else {
                        formKfCS(u)
                    };
                    free(u as *mut libc::c_void);
                    if OOM_CD(0 as libc::c_int as I, z, g, -(1 as libc::c_int) as V) == 0
                    {
                        return 0 as libc::c_int as I;
                    }
                    if 1 as libc::c_int as libc::c_longlong == a {
                        let fresh15 = b;
                        b = b + 1;
                        *(((*(z as K)).k).as_mut_ptr() as *mut I)
                            .offset(
                                fresh15 as isize,
                            ) = *(((*g).k).as_mut_ptr() as *mut I);
                    } else {
                        let fresh16 = b;
                        b = b + 1;
                        *(((*(z as K)).k).as_mut_ptr() as *mut F)
                            .offset(
                                fresh16 as isize,
                            ) = *(((*g).k).as_mut_ptr() as *mut F);
                    }
                    cd(g);
                }
                i_3 += 1;
                i_3;
            }
        }
        6 => {
            a = unescaped_size(
                s.offset(k as isize).offset(1 as libc::c_int as isize),
                r - 2 as libc::c_int as libc::c_longlong,
            );
            z = newK(
                (if 1 as libc::c_int as libc::c_longlong == a {
                    3 as libc::c_int
                } else {
                    -(3 as libc::c_int)
                }) as I,
                a,
            ) as V;
            unescaped_fill(
                ((*(z as K)).k).as_mut_ptr() as *mut C,
                s.offset(k as isize).offset(1 as libc::c_int as isize),
                r - 2 as libc::c_int as libc::c_longlong,
            );
        }
        7 => {
            r = v;
            z = newK(
                (if 1 as libc::c_int as libc::c_longlong == y {
                    4 as libc::c_int
                } else {
                    -(4 as libc::c_int)
                }) as I,
                y,
            ) as V;
            let mut i_4: I = 0 as libc::c_int as I;
            let mut _i_4: I = r;
            while i_4 < _i_4 {
                if !(*m.offset((k + i_4) as isize)
                    >= 0 as libc::c_int as libc::c_longlong)
                {
                    a = 0 as libc::c_int as I;
                    while *m
                        .offset(
                            (k + i_4 + 1 as libc::c_int as libc::c_longlong + a) as isize,
                        ) > 0 as libc::c_int as libc::c_longlong
                    {
                        a += 1;
                        a;
                    }
                    u = alloc((1 as libc::c_int as libc::c_longlong + a) as size_t) as S;
                    c = (if '"' as i32
                        == *s
                            .offset(
                                (k + i_4 + 1 as libc::c_int as libc::c_longlong) as isize,
                            ) as libc::c_int
                    {
                        2 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as I;
                    *u
                        .offset(
                            unescaped_fill(
                                u,
                                s
                                    .offset(k as isize)
                                    .offset(i_4 as isize)
                                    .offset(1 as libc::c_int as isize)
                                    .offset(
                                        (c / 2 as libc::c_int as libc::c_longlong) as isize,
                                    ),
                                a - c,
                            ) as isize,
                        ) = 0 as libc::c_int as C;
                    let fresh17 = b;
                    b = b + 1;
                    let ref mut fresh18 = *(((*(z as K)).k).as_mut_ptr() as *mut S)
                        .offset(fresh17 as isize);
                    *fresh18 = sp(u);
                    free(u as *mut libc::c_void);
                    i_4 += a;
                }
                i_4 += 1;
                i_4;
            }
        }
        8 => {
            e = strdupn(s.offset(k as isize), r);
            u = sp(e);
            free(e as *mut libc::c_void);
            if u.is_null() {
                return kerr(b"wsfull\0" as *const u8 as *const libc::c_char) as L;
            }
            q_0 = 0 as *mut k0;
            if 2 as libc::c_int as libc::c_longlong == r
                && '_' as i32 == *u as libc::c_int
                && stringHasChar(n_s, *u.offset(1 as libc::c_int as isize)) != 0
            {
                if 'f' as i32 == *u.offset(1 as libc::c_int as isize) as libc::c_int {
                    z = (if !func.is_null() { ci(func) } else { _n() }) as V;
                } else {
                    z = ::std::mem::transmute::<
                        _,
                        fn() -> K,
                    >(
                        (::std::mem::transmute::<
                            V,
                            Option::<unsafe extern "C" fn() -> K>,
                        >(
                            *vn_
                                .as_mut_ptr()
                                .offset(
                                    charpos(n_s, *u.offset(1 as libc::c_int as isize)) as isize,
                                ),
                        ))
                            .unwrap(),
                    )() as V;
                }
            } else if !func.is_null() {
                if *u.offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
                    && *u.offset(1 as libc::c_int as isize) as libc::c_int != 'n' as i32
                {
                    return 0 as libc::c_int as I;
                }
                h = *denameS(
                    b".k\0" as *const u8 as *const libc::c_char as S,
                    u,
                    0 as libc::c_int as I,
                );
                if 7 as libc::c_int as libc::c_longlong == (*h).t {
                    hh = match_0(
                        *(((*h).k).as_mut_ptr() as *mut V)
                            .offset(CODE as libc::c_int as isize) as K,
                        *(((*func).k).as_mut_ptr() as *mut V)
                            .offset(CODE as libc::c_int as isize) as K,
                    );
                }
                if dict
                    == (((*func).k).as_mut_ptr() as *mut V as *mut K)
                        .offset(PARAMS as libc::c_int as isize)
                {
                    let mut q_1: V = newEntry(u) as V;
                    if q_1.is_null() {
                        return 0 as libc::c_int as I;
                    }
                    if OOM_CD(
                        0 as libc::c_int as I,
                        q_1,
                        kap(dict, &mut q_1 as *mut V as V),
                        -(1 as libc::c_int) as V,
                    ) == 0
                    {
                        return 0 as libc::c_int as I;
                    }
                    z = EV(q_1 as K) as V;
                    cd(q_1 as K);
                } else {
                    q_0 = DE(*dict, u);
                    if !q_0.is_null() {
                        z = EVP(q_0) as V;
                    } else if -(MARK_VERB as libc::c_int) as libc::c_longlong
                        == *m.offset((k + r) as isize)
                        && ':' as i32
                            == *s
                                .offset(
                                    (k + r + 1 as libc::c_int as libc::c_longlong) as isize,
                                ) as libc::c_int
                        && -(MARK_VERB as libc::c_int) as libc::c_longlong
                            == *m
                                .offset(
                                    (k + r + 1 as libc::c_int as libc::c_longlong) as isize,
                                )
                    {
                        if ':' as i32 == *s.offset((k + r) as isize) as libc::c_int {
                            r += 1;
                            r;
                        }
                        z = denameS(
                            *(((*func).k).as_mut_ptr() as *mut V)
                                .offset(CONTeXT as libc::c_int as isize) as S,
                            u,
                            1 as libc::c_int as I,
                        ) as V;
                    } else if dict
                        == (((*func).k).as_mut_ptr() as *mut V as *mut K)
                            .offset(LOCALS as libc::c_int as isize)
                        && ':' as i32 == *s.offset((k + r) as isize) as libc::c_int
                        && -(MARK_VERB as libc::c_int) as libc::c_longlong
                            == *m.offset((k + r) as isize)
                    {
                        z = denameD(dict, u, 1 as libc::c_int as I) as V;
                    } else if 7 as libc::c_int as libc::c_longlong == (*h).t
                        && *(((*hh).k).as_mut_ptr() as *mut I) != 0
                    {
                        z = ci(func) as V;
                        frc = 1 as libc::c_int as I;
                    } else {
                        z = denameS(
                            *(((*func).k).as_mut_ptr() as *mut V)
                                .offset(CONTeXT as libc::c_int as isize) as S,
                            u,
                            (strstr(
                                s as *const libc::c_char,
                                b"do[\0" as *const u8 as *const libc::c_char,
                            ) != 0 as *mut libc::c_void as *mut libc::c_char)
                                as libc::c_int as I,
                        ) as V;
                    }
                }
            } else {
                if fll > 0 as libc::c_int as libc::c_longlong {
                    fdc = 0 as libc::c_int as I;
                }
                let mut i_5: I = 0;
                i_5 = k;
                while (i_5 as libc::c_ulonglong)
                    < strlen(s as *const libc::c_char) as libc::c_ulonglong
                {
                    if fbr == 0 && *s.offset(i_5 as isize) as libc::c_int == ';' as i32 {
                        break;
                    }
                    if *s.offset(i_5 as isize) as libc::c_int == ':' as i32
                        || fbr != 0
                            && (*s.offset(i_5 as isize) as libc::c_int == 'x' as i32
                                || *s.offset(i_5 as isize) as libc::c_int == 'y' as i32
                                || *s.offset(i_5 as isize) as libc::c_int == 'z' as i32)
                    {
                        fdc = 1 as libc::c_int as I;
                        break;
                    } else {
                        i_5 += 1;
                        i_5;
                    }
                }
                z = inKtree(dict, u, 0 as libc::c_int as I) as V;
                if fStop != 0 && z.is_null() {
                    z = denameD(&mut stopDict, u, 0 as libc::c_int as I) as V;
                }
                if fdc == 0 && z.is_null() {
                    let mut err: L = kerr(b"value\0" as *const u8 as *const libc::c_char)
                        as L;
                    return err;
                }
                if fStop == 0 && z.is_null() {
                    z = denameD(dict, u, (fll != 0 && fdc != 0) as libc::c_int as I)
                        as V;
                }
            }
        }
        10 => {
            if *s.offset(k as isize) as libc::c_int == '\\' as i32 {
                z = 0x7c as libc::c_int as V;
            } else if *s.offset(k as isize) as libc::c_int == ':' as i32
                && (k == 0 as libc::c_int as libc::c_longlong
                    && strlen(s as *const libc::c_char)
                        > 1 as libc::c_int as libc::c_ulong
                    && *s.offset((k + 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int != ':' as i32
                    || k > 0 as libc::c_int as libc::c_longlong
                        && k != n - 1 as libc::c_int as libc::c_longlong
                        && *s.offset((k - 1 as libc::c_int as libc::c_longlong) as isize)
                            as libc::c_int == ';' as i32
                        && *s.offset((k + 1 as libc::c_int as libc::c_longlong) as isize)
                            as libc::c_int != ';' as i32
                        && *s.offset((k + 1 as libc::c_int as libc::c_longlong) as isize)
                            as libc::c_int != '_' as i32
                        && *s.offset((k + 1 as libc::c_int as libc::c_longlong) as isize)
                            as libc::c_int != '[' as i32)
            {
                z = 0x7d as libc::c_int as V;
            } else if '_' as i32 == *s.offset(k as isize) as libc::c_int
                && r > 1 as libc::c_int as libc::c_longlong
            {
                if k + r < n && ':' as i32 == *s.offset((k + r) as isize) as libc::c_int
                    && -(MARK_VERB as libc::c_int) as libc::c_longlong
                        == *m.offset((k + r) as isize)
                {
                    return kerr(b"parse\0" as *const u8 as *const libc::c_char) as L;
                }
                u = strdupn(s.offset(k as isize), r);
                if u.is_null() {
                    return kerr(b"parse\0" as *const u8 as *const libc::c_char) as L;
                }
                let mut i_6: L = 0;
                i_6 = DT_SPECIAL_VERB_OFFSET;
                while i_6 < DT_SIZE
                    && (((*DT.as_mut_ptr().offset(i_6 as isize)).text).is_null()
                        || SC(u, (*DT.as_mut_ptr().offset(i_6 as isize)).text) != 0)
                {
                    i_6 += 1;
                    i_6;
                }
                if i_6 < DT_SIZE {
                    z = i_6 as V;
                }
                free(u as *mut libc::c_void);
                if z.is_null() {
                    return kerr(b"reserved\0" as *const u8 as *const libc::c_char) as L;
                }
            } else {
                grab = 0 as libc::c_int as I;
                i_7 = 0 as libc::c_int as L;
                is_colon = (':' as i32 == *s.offset(k as isize) as libc::c_int)
                    as libc::c_int as I;
                name_bracket_assign = 0 as libc::c_int as I;
                modifier_colon = (k + r < n
                    && ':' as i32 == *s.offset((k + r) as isize) as libc::c_int
                    && -(MARK_VERB as libc::c_int) as libc::c_longlong
                        == *m.offset((k + r) as isize)) as libc::c_int as I;
                if k - i_7 > 0 as libc::c_int as libc::c_longlong {
                    if is_colon != 0
                        && MARK_VERB as libc::c_int as libc::c_longlong
                            == (if *m
                                .offset(
                                    (k - i_7 - 1 as libc::c_int as libc::c_longlong) as isize,
                                ) < 0 as libc::c_int as libc::c_longlong
                            {
                                -*m
                                    .offset(
                                        (k - i_7 - 1 as libc::c_int as libc::c_longlong) as isize,
                                    )
                            } else {
                                *m
                                    .offset(
                                        (k - i_7 - 1 as libc::c_int as libc::c_longlong) as isize,
                                    )
                            })
                    {
                        i_7 += 1;
                        i_7;
                    }
                }
                if k - i_7 > 0 as libc::c_int as libc::c_longlong {
                    if MARK_BRACKET as libc::c_int as libc::c_longlong
                        == (if *m
                            .offset(
                                (k - i_7 - 1 as libc::c_int as libc::c_longlong) as isize,
                            ) < 0 as libc::c_int as libc::c_longlong
                        {
                            -*m
                                .offset(
                                    (k - i_7 - 1 as libc::c_int as libc::c_longlong) as isize,
                                )
                        } else {
                            *m
                                .offset(
                                    (k - i_7 - 1 as libc::c_int as libc::c_longlong) as isize,
                                )
                        })
                    {
                        while *m.offset((k - i_7) as isize)
                            != -(MARK_BRACKET as libc::c_int) as libc::c_longlong
                        {
                            i_7 += 1;
                            i_7;
                        }
                    }
                }
                if k - i_7 > 0 as libc::c_int as libc::c_longlong {
                    if MARK_NAME as libc::c_int as libc::c_longlong
                        == (if *m
                            .offset(
                                (k - i_7 - 1 as libc::c_int as libc::c_longlong) as isize,
                            ) < 0 as libc::c_int as libc::c_longlong
                        {
                            -*m
                                .offset(
                                    (k - i_7 - 1 as libc::c_int as libc::c_longlong) as isize,
                                )
                        } else {
                            *m
                                .offset(
                                    (k - i_7 - 1 as libc::c_int as libc::c_longlong) as isize,
                                )
                        })
                    {
                        name_bracket_assign = 1 as libc::c_int as I;
                    }
                }
                if is_colon == 0
                    && !((k + 1 as libc::c_int as libc::c_longlong) < n
                        && ':' as i32
                            == *s
                                .offset((k + 1 as libc::c_int as libc::c_longlong) as isize)
                                as libc::c_int
                        && -(MARK_VERB as libc::c_int) as libc::c_longlong
                            == *m
                                .offset(
                                    (k + 1 as libc::c_int as libc::c_longlong) as isize,
                                ))
                {
                    name_bracket_assign = 0 as libc::c_int as I;
                }
                if i_7 != 0 && is_colon != 0 && modifier_colon == 0
                    && name_bracket_assign == 0
                {
                    return kerr(b"parse\0" as *const u8 as *const libc::c_char) as L;
                }
                y_present = ((k + r + 1 as libc::c_int as libc::c_longlong) < n
                    && !(*s
                        .offset((k + r + 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int == ':' as i32
                        && -(MARK_VERB as libc::c_int) as libc::c_longlong
                            == *m
                                .offset(
                                    (k + r + 1 as libc::c_int as libc::c_longlong) as isize,
                                ))
                    && MARK_END as libc::c_int as libc::c_longlong
                        != (if *m
                            .offset(
                                (k + r + 1 as libc::c_int as libc::c_longlong) as isize,
                            ) < 0 as libc::c_int as libc::c_longlong
                        {
                            -*m
                                .offset(
                                    (k + r + 1 as libc::c_int as libc::c_longlong) as isize,
                                )
                        } else {
                            *m
                                .offset(
                                    (k + r + 1 as libc::c_int as libc::c_longlong) as isize,
                                )
                        })) as libc::c_int as I;
                a = (if (*d == 0
                    || MARK_END as libc::c_int as libc::c_longlong
                        == (if *m
                            .offset((k - 1 as libc::c_int as libc::c_longlong) as isize)
                            < 0 as libc::c_int as libc::c_longlong
                        {
                            -*m
                                .offset((k - 1 as libc::c_int as libc::c_longlong) as isize)
                        } else {
                            *m
                                .offset((k - 1 as libc::c_int as libc::c_longlong) as isize)
                        })
                    || MARK_ADVERB as libc::c_int as libc::c_longlong
                        == (if *m
                            .offset((k - 1 as libc::c_int as libc::c_longlong) as isize)
                            < 0 as libc::c_int as libc::c_longlong
                        {
                            -*m
                                .offset((k - 1 as libc::c_int as libc::c_longlong) as isize)
                        } else {
                            *m
                                .offset((k - 1 as libc::c_int as libc::c_longlong) as isize)
                        })
                    || MARK_VERB as libc::c_int as libc::c_longlong
                        == (if *m
                            .offset((k - 1 as libc::c_int as libc::c_longlong) as isize)
                            < 0 as libc::c_int as libc::c_longlong
                        {
                            -*m
                                .offset((k - 1 as libc::c_int as libc::c_longlong) as isize)
                        } else {
                            *m
                                .offset((k - 1 as libc::c_int as libc::c_longlong) as isize)
                        }))
                    && !(k + r >= n
                        || -(MARK_END as libc::c_int) as libc::c_longlong
                            == *m.offset((k + r) as isize)
                        || -(MARK_ADVERB as libc::c_int) as libc::c_longlong
                            == *m.offset((k + r) as isize)
                        || -(MARK_BRACKET as libc::c_int) as libc::c_longlong
                            == *m.offset((k + r) as isize))
                {
                    1 as libc::c_int
                } else {
                    2 as libc::c_int
                }) as I;
                if is_colon != 0 && modifier_colon == 0 {
                    a = 2 as libc::c_int as I;
                    if k > 0 as libc::c_int as libc::c_longlong
                        && -(MARK_END as libc::c_int) as libc::c_longlong
                            != *m
                                .offset((k - 1 as libc::c_int as libc::c_longlong) as isize)
                        && *s.offset((k + 1 as libc::c_int as libc::c_longlong) as isize)
                            == 0 && name_bracket_assign == 0
                    {
                        return kerr(b"parse\0" as *const u8 as *const libc::c_char) as L;
                    }
                } else if name_bracket_assign != 0 {
                    a = (if y_present != 0 {
                        2 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) as I;
                } else if modifier_colon != 0 {
                    let ref mut fresh19 = *m.offset((k + r) as isize);
                    *fresh19 *= -(1 as libc::c_int) as libc::c_longlong;
                    r += 1;
                    r;
                    a = 1 as libc::c_int as I;
                    grab = 1 as libc::c_int as I;
                }
                i_7 = 0 as libc::c_int as L;
                if r - grab == 1 as libc::c_int as libc::c_longlong {
                    z = (DT_VERB_OFFSET
                        + 2 as libc::c_int as libc::c_longlong
                            * charsVerb(*s.offset(k as isize))
                        + (if 1 as libc::c_int as libc::c_longlong == a {
                            0 as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) as libc::c_longlong) as V;
                    if z == 0x3d as libc::c_int as V
                        && *s.offset(k as isize) as libc::c_int != ':' as i32
                    {
                        z = 0x7c as libc::c_int as V;
                    }
                } else {
                    let mut jj: libc::c_long = atol(
                        s.offset(k as isize) as *const libc::c_char,
                    );
                    i_7 = DT_SPECIAL_VERB_OFFSET;
                    while i_7 < DT_SIZE
                        && (((*DT.as_mut_ptr().offset(i_7 as isize)).text).is_null()
                            || jj
                                != atol(
                                    (*DT.as_mut_ptr().offset(i_7 as isize)).text
                                        as *const libc::c_char,
                                ))
                    {
                        i_7 += 1;
                        i_7;
                    }
                    if i_7 < DT_SIZE {
                        z = (i_7
                            + (if 1 as libc::c_int as libc::c_longlong == a {
                                0 as libc::c_int
                            } else {
                                1 as libc::c_int
                            }) as libc::c_longlong) as V;
                    } else {
                        return kerr(b"parse\0" as *const u8 as *const libc::c_char) as L
                    }
                }
            }
        }
        11 => {
            z = (DT_ADVERB_OFFSET + charsAdverb(*s.offset(k as isize))
                + (if r > 1 as libc::c_int as libc::c_longlong {
                    3 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as libc::c_longlong) as V;
        }
        3 => {
            z = DT_OFFSET(
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> K>,
                    V,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn() -> K,
                            unsafe extern "C" fn() -> K,
                        >(end),
                    ),
                ),
            ) as V;
        }
        _ => {}
    }
    z.is_null();
    let mut current_block_302: u64;
    match -M {
        8 => {
            if '_' as i32 != *u as libc::c_int && frc == 0 {
                current_block_302 = 11603475171617447446;
            } else {
                current_block_302 = 3341827980619986622;
            }
        }
        4 | 2 | 5 | 9 | 6 | 7 => {
            current_block_302 = 3341827980619986622;
        }
        _ => {
            current_block_302 = 11603475171617447446;
        }
    }
    match current_block_302 {
        3341827980619986622 => {
            z = newE(LS, z as K) as V;
            if z.is_null() {
                return kerr(b"wsfull\0" as *const u8 as *const libc::c_char) as L;
            }
            kap(locals, &mut z as *mut V as V);
            cd(z as K);
            z = EVP(z as K) as V;
        }
        _ => {}
    }
    cd(hh);
    *p = z;
    *d += 1;
    *d;
    return r;
}
