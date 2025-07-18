use ::libc;
extern "C" {
    fn _ssr(a: K, b: K, c: K) -> K;
    fn free(__ptr: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vprintf(_: *const libc::c_char, _: ::std::ffi::VaList) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn modf(_: libc::c_double, _: *mut libc::c_double) -> libc::c_double;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut KFIXED: K;
    fn cd(a: K) -> K;
    static mut fer: I;
    static mut fnci: I;
    static mut prnt: K;
    fn lsz(k: I) -> I;
    fn repool(v: V, r: I) -> I;
    fn check() -> I;
    fn kapn(a: *mut K, v: V, n: I) -> K;
    fn _6d(a: K, b: K) -> K;
    fn _5d(x: K, y: K) -> K;
    fn _4d(x: K, y: K) -> K;
    fn _3d(x: K, y: K) -> K;
    fn _2d(a: K, b: K) -> K;
    fn _1d(x: K, y: K) -> K;
    fn _0d(a: K, b: K) -> K;
    fn _6m(x: K) -> K;
    fn _5m(x: K) -> K;
    fn _4m(x: K) -> K;
    fn _3m(x: K) -> K;
    fn _2m(a: K) -> K;
    fn _1m(x: K) -> K;
    fn _0m(a: K) -> K;
    fn colon_dyadic(a: K, b: K) -> K;
    fn dot(a: K, b: K) -> K;
    fn dollar(a: K, b: K) -> K;
    fn take_reshape(a: K, b: K) -> K;
    fn join(a: K, b: K) -> K;
    fn drop_cut(a: K, b: K) -> K;
    fn what(x: K, y: K) -> K;
    fn at(x: K, y: K) -> K;
    #[link_name = "match"]
    fn match_0(a: K, b: K) -> K;
    fn equals(a: K, b: K) -> K;
    fn more(a: K, b: K) -> K;
    fn less(a: K, b: K) -> K;
    fn rotate_mod(a: K, b: K) -> K;
    fn power(a: K, b: K) -> K;
    fn min_and(a: K, b: K) -> K;
    fn max_or(a: K, b: K) -> K;
    fn divide(a: K, b: K) -> K;
    fn times(a: K, b: K) -> K;
    fn minus(a: K, b: K) -> K;
    fn plus(a: K, b: K) -> K;
    fn colon_monadic(a: K) -> K;
    fn dot_monadic(x: K) -> K;
    fn format(a: K) -> K;
    fn count(a: K) -> K;
    fn enlist(x: K) -> K;
    fn floor_verb(a: K) -> K;
    fn range(a: K) -> K;
    fn atom(a: K) -> K;
    fn not_attribute(a: K) -> K;
    fn group(x: K) -> K;
    fn grade_down(a: K) -> K;
    fn grade_up(a: K) -> K;
    fn enumerate(a: K) -> K;
    fn shape(a: K) -> K;
    #[link_name = "where"]
    fn where_0(x: K) -> K;
    fn reverse(a: K) -> K;
    fn reciprocal(x: K) -> K;
    fn first(a: K) -> K;
    fn negate(x: K) -> K;
    fn flip(a: K) -> K;
    fn CSK(x: K) -> S;
    fn wd(s: S, n: libc::c_int) -> K;
    fn ex(a: K) -> K;
    fn nfinish();
    fn rc(x: K) -> I;
    fn _hash(x: K) -> K;
    fn _kona_exit(x: K) -> K;
    fn _ss(a: K, b: K) -> K;
    fn _sm(a: K, b: K) -> K;
    fn _setenv(a: K, b: K) -> K;
    fn _lsq(a: K, b: K) -> K;
    fn _draw(a: K, b: K) -> K;
    fn _bin(x: K, y: K) -> K;
    fn Kf(x: F) -> K;
    fn _size(a: K) -> K;
    fn _ltime(a: K) -> K;
    fn _lt(a: K) -> K;
    fn _jd(a: K) -> K;
    fn _ic(a: K) -> K;
    fn Ki(x: I) -> K;
    fn _host(a: K) -> K;
    fn _getenv(a: K) -> K;
    fn _dj(a: K) -> K;
    fn _db(x: K) -> K;
    fn _ci(a: K) -> K;
    fn _ceiling(a: K) -> K;
    fn _bd(x: K) -> K;
    fn _abs(a: K) -> K;
    fn ci(a: K) -> K;
    fn newK(t: I, n: I) -> K;
    fn _acos(x: K) -> K;
    fn _asin(x: K) -> K;
    fn _atan(x: K) -> K;
    fn _ceil(x: K) -> K;
    fn _cos(x: K) -> K;
    fn _cosh(x: K) -> K;
    fn _exp(x: K) -> K;
    fn _floor(x: K) -> K;
    fn _log(x: K) -> K;
    fn _sin(x: K) -> K;
    fn _sinh(x: K) -> K;
    fn _sqr(x: K) -> K;
    fn _sqrt(x: K) -> K;
    fn _tan(x: K) -> K;
    fn _tanh(x: K) -> K;
    fn _gtime(x: K) -> K;
    fn _inv(x: K) -> K;
    fn _binl(x: K, y: K) -> K;
    fn _di(x: K, y: K) -> K;
    fn _dot(x: K, y: K) -> K;
    fn _dv(x: K, y: K) -> K;
    fn _dvl(x: K, y: K) -> K;
    fn _hat(x: K, y: K) -> K;
    fn _in(x: K, y: K) -> K;
    fn _lin(x: K, y: K) -> K;
    fn _mul(x: K, y: K) -> K;
    fn _sv(x: K, y: K) -> K;
    fn _vs(x: K, y: K) -> K;
    fn _vsx(x: K, y: K) -> K;
    static mut lineA: S;
    static mut fBreak: S;
    fn OOM_CD(g: I, _: ...) -> I;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
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
pub type UI = libc::c_ulonglong;
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
pub struct node {
    pub k: V,
    pub b: I,
    pub c: [*mut node; 2],
}
pub type Node = node;
pub type N = *mut Node;
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
pub static mut stopDict: K = 0 as *const k0 as K;
pub static mut fStop: I = 0 as libc::c_int as I;
pub static mut errmsg: [C; 256] = [0; 256];
pub unsafe extern "C" fn kerr(mut s: cS) -> K {
    if strcmp(s, b"(nil)\0" as *const u8 as *const libc::c_char) != 0 {
        fer = 2 as libc::c_int as I;
    }
    snprintf(
        errmsg.as_mut_ptr(),
        256 as libc::c_int as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        s,
    );
    return 0 as K;
}
pub static mut SYMBOLS: N = 0 as *const Node as *mut Node;
pub static mut KTREE: K = 0 as *const k0 as *mut k0;
pub static mut SEED: I = 0;
pub static mut d_: S = 0 as *const C as *mut C;
pub static mut HOST_IFACE: S = 0 as *const C as *mut C;
pub static mut IPC_PORT: S = 0 as *const C as *mut C;
pub static mut HTTP_PORT: S = 0 as *const C as *mut C;
pub static mut NIL: K = 0 as *const k0 as *mut k0;
pub static mut LS: S = 0 as *const C as *mut C;
pub static mut PP: I = 7 as libc::c_int as I;
pub static mut PPMAX: I = 19 as libc::c_int as I;
pub static mut PPON: C = 1 as libc::c_int as C;
pub unsafe extern "C" fn maX(mut a: I, mut b: I) -> I {
    return if a > b { a } else { b };
}
pub unsafe extern "C" fn miN(mut a: I, mut b: I) -> I {
    return if a < b { a } else { b };
}
pub unsafe extern "C" fn X(mut s: S) -> K {
    kerr(b"(nil)\0" as *const u8 as *const libc::c_char);
    fnci = 0 as libc::c_int as I;
    return XN(s, strlen(s as *const libc::c_char) as I);
}
unsafe extern "C" fn XN(mut s: S, mut n: I) -> K {
    return ex(wd(s, n as libc::c_int));
}
pub unsafe extern "C" fn KX(mut x: K) -> K {
    return XN(CSK(x), (*x).n);
}
pub unsafe extern "C" fn FF(mut f: F) -> F {
    let mut F: F = 0.;
    return modf(f, &mut F);
}
pub unsafe extern "C" fn simpleString(mut a: S) -> I {
    let mut n: I = strlen(a as *const libc::c_char) as I;
    if n != 0
        && *(*__ctype_b_loc()).offset(*a as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        return 0 as libc::c_int as I;
    }
    if 1 as libc::c_int as libc::c_longlong == n && *a as libc::c_int == '.' as i32 {
        return 0 as libc::c_int as I;
    }
    if n > 1 as libc::c_int as libc::c_longlong
        && *a.offset((n - 1 as libc::c_int as libc::c_longlong) as isize) as libc::c_int
            == '.' as i32
        && *a.offset((n - 2 as libc::c_int as libc::c_longlong) as isize) as libc::c_int
            == '.' as i32
    {
        return 0 as libc::c_int as I;
    }
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = n;
    while i < _i {
        if *(*__ctype_b_loc()).offset(*a.offset(i as isize) as libc::c_int as isize)
            as libc::c_int & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
            == 0 && *a.offset(i as isize) as libc::c_int != '_' as i32
            && *a.offset(i as isize) as libc::c_int != '.' as i32
        {
            return 0 as libc::c_int as I;
        }
        i += 1;
        i;
    }
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_0: I = n - 1 as libc::c_int as libc::c_longlong;
    while i_0 < _i_0 {
        if *a.offset(i_0 as isize) as libc::c_int == '.' as i32
            && *(*__ctype_b_loc())
                .offset(
                    *a.offset((i_0 + 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            return 0 as libc::c_int as I;
        }
        i_0 += 1;
        i_0;
    }
    let mut i_1: I = 0 as libc::c_int as I;
    let mut _i_1: I = n - 2 as libc::c_int as libc::c_longlong;
    while i_1 < _i_1 {
        if *a.offset(i_1 as isize) as libc::c_int == '.' as i32
            && *a.offset((i_1 + 1 as libc::c_int as libc::c_longlong) as isize)
                as libc::c_int == '.' as i32
            && *a.offset((i_1 + 2 as libc::c_int as libc::c_longlong) as isize)
                as libc::c_int == '.' as i32
        {
            return 0 as libc::c_int as I;
        }
        i_1 += 1;
        i_1;
    }
    return 1 as libc::c_int as I;
}
pub unsafe extern "C" fn end() -> K {
    return 0 as K;
}
pub unsafe extern "C" fn bk(mut p: V) -> I {
    return (p as L == DT_END_OFFSET) as libc::c_int as I;
}
pub static mut ac: [C; 4] = unsafe {
    *::std::mem::transmute::<&[u8; 4], &mut [C; 4]>(b"/\\'\0")
};
pub unsafe extern "C" fn over() -> K {
    return 0 as K;
}
pub unsafe extern "C" fn scan() -> K {
    return 0 as K;
}
pub unsafe extern "C" fn each() -> K {
    return 0 as K;
}
pub unsafe extern "C" fn tr_st(mut x: K) -> K {
    if 0 as libc::c_int
        == strcmp(
            fBreak as *const libc::c_char,
            b"t\0" as *const u8 as *const libc::c_char,
        )
    {
        show(x);
    }
    if 0 as libc::c_int
        == strcmp(
            fBreak as *const libc::c_char,
            b"s\0" as *const u8 as *const libc::c_char,
        )
    {
        printf(b"stop\n\0" as *const u8 as *const libc::c_char);
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, lineA);
        stopDict = ci(
            *(((*prnt).k).as_mut_ptr() as *mut V).offset(7 as libc::c_int as isize) as K,
        );
        fStop = 1 as libc::c_int as I;
        check();
        cd(stopDict);
        stopDict = 0 as K;
        fStop = 0 as libc::c_int as I;
    }
    return ci(x);
}
pub unsafe extern "C" fn rtrn(mut x: K) -> K {
    fer = 1 as libc::c_int as I;
    return ci(x);
}
pub unsafe extern "C" fn eachright() -> K {
    return 0 as K;
}
pub unsafe extern "C" fn eachleft() -> K {
    return 0 as K;
}
pub unsafe extern "C" fn eachpair() -> K {
    return 0 as K;
}
pub unsafe extern "C" fn resume(mut x: K) -> K {
    return 0 as K;
}
pub static mut vc: [C; 21] = unsafe {
    *::std::mem::transmute::<&[u8; 21], &mut [C; 21]>(b"+-*%|&^!<>=~@?_,#$.:\0")
};
pub static mut offsetSSR: V = 0 as *const libc::c_void as *mut libc::c_void;
pub static mut offsetWhat: V = 0 as *const libc::c_void as *mut libc::c_void;
pub static mut offsetAt: V = 0 as *const libc::c_void as *mut libc::c_void;
pub static mut offsetDot: V = 0 as *const libc::c_void as *mut libc::c_void;
pub static mut offsetColon: V = 0 as *const libc::c_void as *mut libc::c_void;
pub static mut offsetJoin: V = 0 as *const libc::c_void as *mut libc::c_void;
pub static mut offset3m: V = 0 as *const libc::c_void as *mut libc::c_void;
pub static mut offsetOver: L = 0;
pub static mut offsetScan: L = 0;
pub static mut offsetEach: L = 0;
pub static mut offsetEachright: L = 0;
pub static mut offsetEachleft: L = 0;
pub static mut offsetEachpair: L = 0;
pub static mut IFS: [S; 3] = [
    b"x\0" as *const u8 as *const libc::c_char as S,
    b"y\0" as *const u8 as *const libc::c_char as S,
    b"z\0" as *const u8 as *const libc::c_char as S,
];
pub static mut IFP: [S; 3] = [0 as *const C as *mut C; 3];
pub unsafe extern "C" fn stringHasChar(mut s: S, mut c: C) -> I {
    let mut i: I = 0 as libc::c_int as I;
    while *s.offset(i as isize) != 0 {
        let fresh0 = i;
        i = i + 1;
        if c as libc::c_int == *s.offset(fresh0 as isize) as libc::c_int {
            return 1 as libc::c_int as I;
        }
    }
    return 0 as libc::c_int as I;
}
pub unsafe extern "C" fn charpos(mut s: S, mut c: C) -> I {
    let mut i: I = 0 as libc::c_int as I;
    while *s.offset(i as isize) as libc::c_int != 0
        && c as libc::c_int != *s.offset(i as isize) as libc::c_int
    {
        i += 1;
        i;
    }
    return i;
}
pub unsafe extern "C" fn isCharVerb(mut c: C) -> I {
    return stringHasChar(vc.as_mut_ptr(), c);
}
pub unsafe extern "C" fn charsVerb(mut c: C) -> L {
    return charpos(vc.as_mut_ptr(), c);
}
pub unsafe extern "C" fn verbsChar(mut p: V) -> C {
    return (if p as L >= DT_VERB_OFFSET && (p as L) < DT_SPECIAL_VERB_OFFSET {
        vc[((p as L - DT_VERB_OFFSET) / 2 as libc::c_int as libc::c_longlong) as usize]
            as libc::c_int
    } else {
        '\0' as i32
    }) as C;
}
pub unsafe extern "C" fn adverbsChar(mut p: V) -> C {
    return (if p as L >= DT_ADVERB_OFFSET {
        ac[((p as L - DT_ADVERB_OFFSET) % 3 as libc::c_int as libc::c_longlong) as usize]
            as libc::c_int
    } else {
        '\0' as i32
    }) as C;
}
pub unsafe extern "C" fn charsAdverb(mut c: C) -> L {
    return charpos(ac.as_mut_ptr(), c);
}
pub unsafe extern "C" fn sva(mut p: V) -> I {
    let mut q: UI = p as UI;
    if q < DT_SIZE as libc::c_ulonglong {
        return (*DT.as_mut_ptr().offset(q as isize)).arity;
    }
    return 0 as libc::c_int as I;
}
pub unsafe extern "C" fn adverbClass(mut p: V) -> I {
    let mut q: UI = p as UI;
    if q < DT_SIZE as libc::c_ulonglong {
        return (*DT.as_mut_ptr().offset(q as isize)).adverbClass;
    }
    return 0 as libc::c_int as I;
}
pub unsafe extern "C" fn kdefClass(mut n: I) -> I {
    return (n == 98 as libc::c_int as libc::c_longlong
        || n == 101 as libc::c_int as libc::c_longlong
        || n == 107 as libc::c_int as libc::c_longlong
        || n == 108 as libc::c_int as libc::c_longlong
        || n == 111 as libc::c_int as libc::c_longlong
        || n == 112 as libc::c_int as libc::c_longlong
        || n == 113 as libc::c_int as libc::c_longlong
        || n == 114 as libc::c_int as libc::c_longlong
        || n == 115 as libc::c_int as libc::c_longlong
        || n == 117 as libc::c_int as libc::c_longlong
        || n == 121 as libc::c_int as libc::c_longlong
        || n == 123 as libc::c_int as libc::c_longlong) as libc::c_int as I;
}
unsafe extern "C" fn specialValence(mut p: V) -> I {
    return (if p == offsetSSR || p == offsetWhat {
        3 as libc::c_int
    } else if p == offsetAt || p == offsetDot {
        4 as libc::c_int
    } else {
        0 as libc::c_int
    }) as I;
}
pub unsafe extern "C" fn valence(mut p: V) -> I {
    let mut a: I = 0;
    let mut i: I = 0;
    a = specialValence(p);
    a = if a != 0 { a } else { sva(p) };
    if a != 0 {
        return a;
    }
    if adverbClass(p) != 0 {
        return 0 as libc::c_int as I;
    }
    let mut v: K = *(p as *mut K);
    if v.is_null() || (*v).t != 7 as libc::c_int as libc::c_longlong {
        return 0 as libc::c_int as I;
    }
    let mut w: *mut V = ((*(*(((*v).k).as_mut_ptr() as *mut V)
        .offset(CODE as libc::c_int as isize) as K))
        .k)
        .as_mut_ptr() as *mut S as *mut V;
    let mut t: I = (*v).n;
    let mut b: K = *(((*v).k).as_mut_ptr() as *mut V)
        .offset(CONJ as libc::c_int as isize) as K;
    let mut c: I = 0 as libc::c_int as I;
    if !b.is_null() {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i: I = (*b).n;
        while i_0 < _i {
            if !(*((*b).k).as_mut_ptr().offset(i_0 as isize)).is_null() {
                c += 1;
                c;
            }
            i_0 += 1;
            i_0;
        }
        return (*b).n - c;
    }
    if 1 as libc::c_int as libc::c_longlong == t {
        i = (*(*(((*v).k).as_mut_ptr() as *mut V).offset(CODE as libc::c_int as isize)
            as K))
            .n - 1 as libc::c_int as libc::c_longlong;
        let mut k: *mut V = *(((*(*(((*v).k).as_mut_ptr() as *mut V)
            .offset(CODE as libc::c_int as isize) as K))
            .k)
            .as_mut_ptr() as *mut S as *mut V)
            .offset((i - 1 as libc::c_int as libc::c_longlong) as isize) as *mut V;
        if k as L == offsetEachright || k as L == offsetEachleft {
            return 2 as libc::c_int as I;
        }
        if i > 1 as libc::c_int as libc::c_longlong && k as L == offsetEach
            || k as L == offsetOver || k as L == offsetScan
        {
            let mut q: *mut V = 0 as *mut V;
            let mut j: I = 0 as libc::c_int as I;
            let mut s: I = 0;
            loop {
                let fresh1 = j;
                j = j + 1;
                q = *(((*(*(((*v).k).as_mut_ptr() as *mut V)
                    .offset(CODE as libc::c_int as isize) as K))
                    .k)
                    .as_mut_ptr() as *mut S as *mut V)
                    .offset((i - 2 as libc::c_int as libc::c_longlong - fresh1) as isize)
                    as *mut V;
                if !(q as L == offsetEach || q as L == offsetOver
                    || q as L == offsetScan)
                {
                    break;
                }
            }
            s = sva(q as V);
            if s != 0 && specialValence(q as V) == 0 {
                return s
                    - (if i - 2 as libc::c_int as libc::c_longlong - j
                        >= 0 as libc::c_int as libc::c_longlong
                    {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as libc::c_longlong;
            }
            if j < i - 2 as libc::c_int as libc::c_longlong {
                return valence(q as V) - 1 as libc::c_int as libc::c_longlong
            } else {
                return valence(q as V)
            }
        }
        if adverbClass(k as V) != 0 {
            return 2 as libc::c_int as I;
        }
        if sva(k as V) > 1 as libc::c_int as libc::c_longlong
            && i > 1 as libc::c_int as libc::c_longlong
            && VA(
                *(((*(*(((*v).k).as_mut_ptr() as *mut V)
                    .offset(CODE as libc::c_int as isize) as K))
                    .k)
                    .as_mut_ptr() as *mut S as *mut V)
                    .offset((i - 2 as libc::c_int as libc::c_longlong) as isize),
            ) == 0
        {
            return valence(k as V) - 1 as libc::c_int as libc::c_longlong;
        }
        return valence(k as V);
    }
    if 2 as libc::c_int as libc::c_longlong == t {
        return *w.offset(0 as libc::c_int as isize) as L;
    }
    if 3 as libc::c_int as libc::c_longlong == t {
        return (*(*(((*v).k).as_mut_ptr() as *mut V)
            .offset(PARAMS as libc::c_int as isize) as K))
            .n;
    }
    return 0 as libc::c_int as I;
}
pub unsafe extern "C" fn VA(mut p: V) -> I {
    return (sva(p) != 0 || adverbClass(p) != 0) as libc::c_int as I;
}
unsafe extern "C" fn isescape(mut c: UC) -> I {
    return (c as libc::c_int == '"' as i32 || c as libc::c_int == '\\' as i32
        || c as libc::c_int == '\u{8}' as i32 || c as libc::c_int == '\n' as i32
        || c as libc::c_int == '\r' as i32 || c as libc::c_int == '\t' as i32)
        as libc::c_int as I;
}
unsafe extern "C" fn needspt0(mut f: F) -> I {
    if f.is_nan() as i32 != 0 || -(1 as libc::c_int as libc::c_double / 0.0f64) == f
        || 1 as libc::c_int as libc::c_double / 0.0f64 == f
    {
        return 0 as libc::c_int as I;
    }
    static mut b: [C; 512] = [0; 512];
    snprintf(
        b.as_mut_ptr(),
        512 as libc::c_int as libc::c_ulong,
        b"%.*g\0" as *const u8 as *const libc::c_char,
        PP as libc::c_int,
        f,
    );
    return (stringHasChar(b.as_mut_ptr(), '.' as i32 as C) == 0
        && stringHasChar(b.as_mut_ptr(), 'e' as i32 as C) == 0) as libc::c_int as I;
}
unsafe extern "C" fn splitprint(
    mut u: V,
    mut s: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    static mut b: [C; 512] = [0; 512];
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    if u.is_null() {
        vprintf(s, args_0.as_va_list());
    } else {
        let mut n: I = vsnprintf(
            b.as_mut_ptr(),
            512 as libc::c_int as libc::c_ulong,
            s,
            args_0.as_va_list(),
        ) as I;
        (kapn(u as *mut K, b.as_mut_ptr() as V, n)).is_null();
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn printAtDepth(
    mut u: V,
    mut a: K,
    mut d: I,
    mut x: I,
    mut vdep: I,
    mut b: I,
) {
    if a.is_null() {
        return;
    }
    let mut t: I = (*a).t;
    if x != 0 {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = d;
        while i < _i {
            splitprint(u, b" \0" as *const u8 as *const libc::c_char);
            i += 1;
            i;
        }
    }
    if 5 as libc::c_int as libc::c_longlong == t {
        splitprint(u, b".\0" as *const u8 as *const libc::c_char);
        d += 1 as libc::c_int as libc::c_longlong;
        t = 0 as libc::c_int as I;
    }
    if t <= 0 as libc::c_int as libc::c_longlong
        && (*a).n == 1 as libc::c_int as libc::c_longlong
    {
        splitprint(u, b",\0" as *const u8 as *const libc::c_char);
    }
    let mut m: I = 0 as libc::c_int as I;
    let mut s: K = 0 as *mut k0;
    if 0 as libc::c_int as libc::c_longlong == t && b == 0 {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = (*a).n;
        while i_0 < _i_0 {
            s = *((*a).k).as_mut_ptr().offset(i_0 as isize);
            if !s.is_null() && (*s).t <= 0 as libc::c_int as libc::c_longlong
                && ((*s).n != 0 || -(3 as libc::c_int) as libc::c_longlong == (*s).t)
            {
                m = 1 as libc::c_int as I;
                break;
            } else if !s.is_null() && (*s).t == 5 as libc::c_int as libc::c_longlong {
                m = 1 as libc::c_int as I;
                break;
            } else {
                i_0 += 1;
                i_0;
            }
        }
    }
    let mut enclose: I = (0 as libc::c_int as libc::c_longlong == t
        && (*a).n != 1 as libc::c_int as libc::c_longlong
        || t == 7 as libc::c_int as libc::c_longlong && vdep != 0) as libc::c_int as I;
    if enclose != 0 {
        splitprint(
            u,
            if b != 0 {
                b"[\0" as *const u8 as *const libc::c_char
            } else {
                b"(\0" as *const u8 as *const libc::c_char
            },
        );
    }
    let mut f: I = 0;
    let mut g: F = 0.;
    if 0 as libc::c_int as libc::c_longlong == t {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = (*a).n;
        while i_1 < _i_1 {
            printAtDepth(
                u,
                *((*a).k).as_mut_ptr().offset(i_1 as isize),
                d + 1 as libc::c_int as libc::c_longlong,
                i_1 * m,
                0 as libc::c_int as I,
                0 as libc::c_int as I,
            );
            splitprint(
                u,
                if i_1 < _i_1 - 1 as libc::c_int as libc::c_longlong {
                    if m != 0 {
                        b"\n\0" as *const u8 as *const libc::c_char
                    } else {
                        b";\0" as *const u8 as *const libc::c_char
                    }
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
            i_1 += 1;
            i_1;
        }
    }
    if 1 as libc::c_int as libc::c_longlong
        == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        if (*a).n == 0 {
            splitprint(u, b"!0\0" as *const u8 as *const libc::c_char);
        } else {
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = (*a).n;
            while i_2 < _i_2 {
                f = *(((*a).k).as_mut_ptr() as *mut I).offset(i_2 as isize);
                if f
                    == -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
                {
                    splitprint(u, b"0N\0" as *const u8 as *const libc::c_char);
                } else {
                    if f == -(9223372036854775807 as libc::c_longlong) {
                        splitprint(u, b"-0I\0" as *const u8 as *const libc::c_char);
                    } else {
                        if f == 9223372036854775807 as libc::c_longlong {
                            splitprint(u, b"0I\0" as *const u8 as *const libc::c_char);
                        } else {
                            splitprint(
                                u,
                                b"%lld\0" as *const u8 as *const libc::c_char,
                                f,
                            );
                        };
                    };
                };
                if i_2 < _i_2 - 1 as libc::c_int as libc::c_longlong {
                    splitprint(u, b" \0" as *const u8 as *const libc::c_char);
                }
                i_2 += 1;
                i_2;
            }
        }
    }
    if 2 as libc::c_int as libc::c_longlong
        == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        if (*a).n == 0 {
            splitprint(u, b"0#0.0\0" as *const u8 as *const libc::c_char);
        } else {
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = (*a).n;
            while i_3 < _i_3 {
                g = *(((*a).k).as_mut_ptr() as *mut F).offset(i_3 as isize);
                if g.is_nan() as i32 != 0 {
                    splitprint(u, b"0n\0" as *const u8 as *const libc::c_char);
                } else {
                    if g == -(1 as libc::c_int as libc::c_double / 0.0f64) {
                        splitprint(u, b"-0i\0" as *const u8 as *const libc::c_char);
                    } else {
                        if g == 1 as libc::c_int as libc::c_double / 0.0f64 {
                            splitprint(u, b"0i\0" as *const u8 as *const libc::c_char);
                        } else {
                            splitprint(
                                u,
                                b"%.*g\0" as *const u8 as *const libc::c_char,
                                PP as libc::c_int,
                                g,
                            );
                        };
                    };
                };
                if i_3 < _i_3 - 1 as libc::c_int as libc::c_longlong {
                    splitprint(u, b" \0" as *const u8 as *const libc::c_char);
                } else if needspt0(g) != 0 {
                    splitprint(u, b".0\0" as *const u8 as *const libc::c_char);
                }
                i_3 += 1;
                i_3;
            }
        }
    }
    if 3 as libc::c_int as libc::c_longlong
        == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        splitprint(u, b"\"\0" as *const u8 as *const libc::c_char);
        let mut i_4: I = 0 as libc::c_int as I;
        let mut _i_4: I = (*a).n;
        while i_4 < _i_4 {
            let mut c: UC = *(((*a).k).as_mut_ptr() as *mut C).offset(i_4 as isize)
                as UC;
            if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
                && isescape(c) == 0
            {
                splitprint(
                    u,
                    b"%c\0" as *const u8 as *const libc::c_char,
                    c as libc::c_int,
                );
            } else if isescape(c) != 0 {
                match c as libc::c_int {
                    34 => {
                        splitprint(u, b"\\\"\0" as *const u8 as *const libc::c_char);
                    }
                    92 => {
                        splitprint(u, b"\\\\\0" as *const u8 as *const libc::c_char);
                    }
                    8 => {
                        splitprint(u, b"\\b\0" as *const u8 as *const libc::c_char);
                    }
                    10 => {
                        splitprint(u, b"\\n\0" as *const u8 as *const libc::c_char);
                    }
                    13 => {
                        splitprint(u, b"\\r\0" as *const u8 as *const libc::c_char);
                    }
                    9 => {
                        splitprint(u, b"\\t\0" as *const u8 as *const libc::c_char);
                    }
                    _ => {}
                }
            } else {
                splitprint(
                    u,
                    b"\\%.3o\0" as *const u8 as *const libc::c_char,
                    c as libc::c_int,
                );
            }
            i_4 += 1;
            i_4;
        }
        splitprint(u, b"\"\0" as *const u8 as *const libc::c_char);
    }
    if 4 as libc::c_int as libc::c_longlong
        == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        if (*a).n == 0 {
            splitprint(u, b"0#`\0" as *const u8 as *const libc::c_char);
        } else {
            let mut ss: I = 0 as libc::c_int as I;
            let mut sl: I = 0;
            let mut str: S = 0 as *mut C;
            let mut i_5: I = 0 as libc::c_int as I;
            let mut _i_5: I = (*a).n;
            while i_5 < _i_5 {
                str = *(((*a).k).as_mut_ptr() as *mut S).offset(i_5 as isize);
                if (str as L as libc::c_double) < -2e9f64 || DT_SIZE < str as L {
                    sl = strlen(str as *const libc::c_char) as I;
                    ss = simpleString(str);
                    splitprint(u, b"`\0" as *const u8 as *const libc::c_char);
                    if ss == 0 {
                        splitprint(u, b"\"\0" as *const u8 as *const libc::c_char);
                    }
                    let mut j: I = 0 as libc::c_int as I;
                    let mut _j: I = sl;
                    while j < _j {
                        splitprint(
                            u,
                            b"%c\0" as *const u8 as *const libc::c_char,
                            *str.offset(j as isize) as libc::c_int,
                        );
                        j += 1;
                        j;
                    }
                    if ss == 0 {
                        splitprint(u, b"\"\0" as *const u8 as *const libc::c_char);
                    }
                    splitprint(
                        u,
                        if i_5 < _i_5 - 1 as libc::c_int as libc::c_longlong {
                            b" \0" as *const u8 as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                    );
                } else if -(4 as libc::c_int) as libc::c_longlong == t {
                    if (str as L) < 0 as libc::c_int as libc::c_longlong {
                        printf(b"\n%p \0" as *const u8 as *const libc::c_char, str);
                    } else {
                        printf(b"%p \0" as *const u8 as *const libc::c_char, str);
                        if str.is_null() {
                            printf(b" \0" as *const u8 as *const libc::c_char);
                        } else if i_5 < (*a).n
                            && *(((*a).k).as_mut_ptr() as *mut S)
                                .offset(
                                    (i_5 + 1 as libc::c_int as libc::c_longlong) as isize,
                                ) < DT_SIZE as S
                        {
                            printf(b"\n\0" as *const u8 as *const libc::c_char);
                        }
                    }
                    if (str as L) < 0 as libc::c_int as libc::c_longlong {
                        printf(
                            b" %p \0" as *const u8 as *const libc::c_char,
                            *(str as *mut V),
                        );
                        if !((0 as libc::c_int as libc::c_longlong)
                            < *(str as *mut V) as L && (*(str as *mut V) as L) < DT_SIZE)
                        {
                            printf(
                                b"%p\n\0" as *const u8 as *const libc::c_char,
                                *(str as *mut K),
                            );
                        }
                    }
                }
                i_5 += 1;
                i_5;
            }
        }
    }
    if 7 as libc::c_int as libc::c_longlong == t {
        if 1 as libc::c_int as libc::c_longlong == (*a).n {
            let mut i_6: I = 0;
            let mut k: I = 0;
            let mut s_0: S = 0 as *mut C;
            let mut v: *mut V = ((*(*(((*a).k).as_mut_ptr() as *mut V)
                .offset(CODE as libc::c_int as isize) as K))
                .k)
                .as_mut_ptr() as *mut S as *mut V;
            let mut p: *mut V = 0 as *mut V;
            i_6 = 0 as libc::c_int as I;
            loop {
                p = *v.offset(i_6 as isize) as *mut V;
                if p.is_null() {
                    break;
                }
                let mut q: I = p as L;
                if q < DT_SIZE && q >= DT_SPECIAL_VERB_OFFSET {
                    s_0 = (*DT.as_mut_ptr().offset(q as isize)).text;
                    k = strlen(s_0 as *const libc::c_char) as I;
                    let mut i_7: I = 0 as libc::c_int as I;
                    let mut _i_6: I = k;
                    while i_7 < _i_6 {
                        splitprint(
                            u,
                            b"%c\0" as *const u8 as *const libc::c_char,
                            *s_0.offset(i_7 as isize) as libc::c_int,
                        );
                        i_7 += 1;
                        i_7;
                    }
                    if *s_0.offset((k - 1 as libc::c_int as libc::c_longlong) as isize)
                        as libc::c_int == ':' as i32
                        && 1 as libc::c_int as libc::c_longlong
                            == (*DT.as_mut_ptr().offset(q as isize)).arity
                    {
                        splitprint(
                            u,
                            b"%c\0" as *const u8 as *const libc::c_char,
                            ':' as i32,
                        );
                    }
                } else {
                    k = sva(p as V);
                    if k != 0 {
                        splitprint(
                            u,
                            if 2 as libc::c_int as libc::c_longlong == k {
                                b"%c\0" as *const u8 as *const libc::c_char
                            } else {
                                b"%c:\0" as *const u8 as *const libc::c_char
                            },
                            verbsChar(p as V) as libc::c_int,
                        );
                    } else {
                        k = adverbClass(p as V);
                        if k != 0 {
                            splitprint(
                                u,
                                if 1 as libc::c_int as libc::c_longlong == k {
                                    b"%c\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"%c:\0" as *const u8 as *const libc::c_char
                                },
                                adverbsChar(p as V) as libc::c_int,
                            );
                        } else {
                            printAtDepth(
                                u,
                                *(p as *mut K),
                                d + 1 as libc::c_int as libc::c_longlong,
                                0 as libc::c_int as I,
                                1 as libc::c_int as libc::c_longlong + vdep,
                                0 as libc::c_int as I,
                            );
                        }
                    }
                }
                i_6 += 1;
                i_6;
            }
        } else if 2 as libc::c_int as libc::c_longlong == (*a).n {
            return
        } else if 3 as libc::c_int as libc::c_longlong == (*a).n {
            splitprint(
                u,
                b"{%s}\0" as *const u8 as *const libc::c_char,
                ((*(*(((*a).k).as_mut_ptr() as *mut V)
                    .offset(CODE as libc::c_int as isize) as K))
                    .k)
                    .as_mut_ptr() as *mut C,
            );
        }
        if !(*(((*a).k).as_mut_ptr() as *mut V).offset(CONJ as libc::c_int as isize))
            .is_null()
        {
            printAtDepth(
                u,
                *(((*a).k).as_mut_ptr() as *mut V).offset(CONJ as libc::c_int as isize)
                    as K,
                d + 1 as libc::c_int as libc::c_longlong,
                0 as libc::c_int as I,
                0 as libc::c_int as I,
                1 as libc::c_int as I,
            );
        }
    }
    if enclose != 0 {
        splitprint(
            u,
            if b != 0 {
                b"]\0" as *const u8 as *const libc::c_char
            } else {
                b")\0" as *const u8 as *const libc::c_char
            },
        );
    }
}
pub unsafe extern "C" fn show(mut a: K) -> K {
    printAtDepth(
        0 as V,
        a,
        0 as libc::c_int as I,
        0 as libc::c_int as I,
        0 as libc::c_int as I,
        0 as libc::c_int as I,
    );
    if !a.is_null() && (*a).t != 6 as libc::c_int as libc::c_longlong {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    return a;
}
pub unsafe extern "C" fn minus_eachpair(mut x: K, mut y: K) -> K {
    if (*y).n < 2 as libc::c_int as libc::c_longlong || (*y).t == 0
        || (if (*y).t < 0 as libc::c_int as libc::c_longlong { -(*y).t } else { (*y).t })
            > 2 as libc::c_int as libc::c_longlong
    {
        return 0 as K;
    }
    let mut z: K = newK((*y).t, (*y).n - 1 as libc::c_int as libc::c_longlong);
    if -(2 as libc::c_int) as libc::c_longlong == (*y).t {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = (*y).n - 1 as libc::c_int as libc::c_longlong;
        while i < _i {
            *(((*z).k).as_mut_ptr() as *mut F)
                .offset(
                    i as isize,
                ) = *(((*y).k).as_mut_ptr() as *mut F)
                .offset((i + 1 as libc::c_int as libc::c_longlong) as isize)
                - *(((*y).k).as_mut_ptr() as *mut F).offset(i as isize);
            i += 1;
            i;
        }
    } else if -(1 as libc::c_int) as libc::c_longlong == (*y).t {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = (*y).n - 1 as libc::c_int as libc::c_longlong;
        while i_0 < _i_0 {
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    i_0 as isize,
                ) = *(((*y).k).as_mut_ptr() as *mut I)
                .offset((i_0 + 1 as libc::c_int as libc::c_longlong) as isize)
                - *(((*y).k).as_mut_ptr() as *mut I).offset(i_0 as isize);
            i_0 += 1;
            i_0;
        }
    }
    if !x.is_null() {
        let mut u: K = 0 as *mut k0;
        let mut v: K = 0 as *mut k0;
        u = enlist(x);
        if OOM_CD(0 as libc::c_int as I, u, z, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        v = join(u, z);
        cd(u);
        cd(z);
        return v;
    }
    return z;
}
pub unsafe extern "C" fn plus_scan(mut x: K, mut y: K) -> K {
    if !x.is_null() && (*x).t != 1 as libc::c_int as libc::c_longlong
        && (*x).t != 2 as libc::c_int as libc::c_longlong
    {
        return 0 as K;
    }
    if (*y).n < 2 as libc::c_int as libc::c_longlong || (*y).t == 0
        || (if (*y).t < 0 as libc::c_int as libc::c_longlong { -(*y).t } else { (*y).t })
            > 2 as libc::c_int as libc::c_longlong
    {
        return 0 as K;
    }
    let mut t: I = -if (*y).t < 0 as libc::c_int as libc::c_longlong {
        -(*y).t
    } else {
        (*y).t
    };
    if !x.is_null() {
        t = -if (if (*x).t < 0 as libc::c_int as libc::c_longlong {
            -(*x).t
        } else {
            (*x).t
        }) > (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
        {
            if (*x).t < 0 as libc::c_int as libc::c_longlong { -(*x).t } else { (*x).t }
        } else if t < 0 as libc::c_int as libc::c_longlong {
            -t
        } else {
            t
        };
    }
    let mut n: I = (*y).n
        + (if !x.is_null() { 1 as libc::c_int } else { 0 as libc::c_int })
            as libc::c_longlong;
    let mut z: K = 0 as *mut k0;
    if x.is_null() && 1 as libc::c_int as libc::c_longlong == rc(y)
        && ((*y).t == t
            || ::std::mem::size_of::<I>() as libc::c_ulong
                == ::std::mem::size_of::<F>() as libc::c_ulong)
    {
        z = ci(y);
    } else {
        z = newK(t, n);
    }
    if z.is_null() {
        return 0 as K;
    }
    let mut j: I = 0 as libc::c_int as I;
    if !x.is_null() {
        j = 1 as libc::c_int as I;
        if -(2 as libc::c_int) as libc::c_longlong == t
            && 2 as libc::c_int as libc::c_longlong == (*x).t
        {
            *(((*z).k).as_mut_ptr() as *mut F) = *(((*x).k).as_mut_ptr() as *mut F);
        } else if -(2 as libc::c_int) as libc::c_longlong == t
            && 1 as libc::c_int as libc::c_longlong == (*x).t
        {
            *(((*z).k).as_mut_ptr() as *mut F) = *(((*x).k).as_mut_ptr() as *mut I) as F;
        } else if -(1 as libc::c_int) as libc::c_longlong == t
            && 1 as libc::c_int as libc::c_longlong == (*x).t
        {
            *(((*z).k).as_mut_ptr() as *mut I) = *(((*x).k).as_mut_ptr() as *mut I);
        }
    }
    if -(2 as libc::c_int) as libc::c_longlong == t
        && -(2 as libc::c_int) as libc::c_longlong == (*y).t
    {
        *(((*z).k).as_mut_ptr() as *mut F)
            .offset(
                j as isize,
            ) = *(((*y).k).as_mut_ptr() as *mut F)
            + (if j != 0 {
                *(((*z).k).as_mut_ptr() as *mut F)
            } else {
                0 as libc::c_int as libc::c_double
            });
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = (*y).n - 1 as libc::c_int as libc::c_longlong;
        while i < _i {
            *(((*z).k).as_mut_ptr() as *mut F)
                .offset(
                    (i + j + 1 as libc::c_int as libc::c_longlong) as isize,
                ) = *(((*z).k).as_mut_ptr() as *mut F).offset((i + j) as isize)
                + *(((*y).k).as_mut_ptr() as *mut F)
                    .offset((i + 1 as libc::c_int as libc::c_longlong) as isize);
            i += 1;
            i;
        }
    } else if -(2 as libc::c_int) as libc::c_longlong == t
        && -(1 as libc::c_int) as libc::c_longlong == (*y).t
    {
        *(((*z).k).as_mut_ptr() as *mut F)
            .offset(
                j as isize,
            ) = *(((*y).k).as_mut_ptr() as *mut I) as libc::c_double
            + (if j != 0 {
                *(((*z).k).as_mut_ptr() as *mut F)
            } else {
                0 as libc::c_int as libc::c_double
            });
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = (*y).n - 1 as libc::c_int as libc::c_longlong;
        while i_0 < _i_0 {
            *(((*z).k).as_mut_ptr() as *mut F)
                .offset(
                    (i_0 + j + 1 as libc::c_int as libc::c_longlong) as isize,
                ) = *(((*z).k).as_mut_ptr() as *mut F).offset((i_0 + j) as isize)
                + *(((*y).k).as_mut_ptr() as *mut I)
                    .offset((i_0 + 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_double;
            i_0 += 1;
            i_0;
        }
    } else if -(1 as libc::c_int) as libc::c_longlong == t
        && -(1 as libc::c_int) as libc::c_longlong == (*y).t
    {
        *(((*z).k).as_mut_ptr() as *mut I)
            .offset(
                j as isize,
            ) = *(((*y).k).as_mut_ptr() as *mut I)
            + (if j != 0 {
                *(((*z).k).as_mut_ptr() as *mut I)
            } else {
                0 as libc::c_int as libc::c_longlong
            });
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = (*y).n - 1 as libc::c_int as libc::c_longlong;
        while i_1 < _i_1 {
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    (i_1 + j + 1 as libc::c_int as libc::c_longlong) as isize,
                ) = *(((*z).k).as_mut_ptr() as *mut I).offset((i_1 + j) as isize)
                + *(((*y).k).as_mut_ptr() as *mut I)
                    .offset((i_1 + 1 as libc::c_int as libc::c_longlong) as isize);
            i_1 += 1;
            i_1;
        }
    }
    return z;
}
pub unsafe extern "C" fn plus_over(mut x: K, mut y: K) -> K {
    let mut accI: I = 0 as libc::c_int as I;
    let mut accF: F = 0 as libc::c_int as F;
    if (*y).t == 0 && (*y).n != 0
        || (if (*y).t < 0 as libc::c_int as libc::c_longlong { -(*y).t } else { (*y).t })
            > 2 as libc::c_int as libc::c_longlong
        || !x.is_null() && (*x).t != 1 as libc::c_int as libc::c_longlong
            && (*x).t != 2 as libc::c_int as libc::c_longlong
    {
        return 0 as K;
    }
    let mut z: K = 0 as *mut k0;
    let mut r: K = 0 as *mut k0;
    match if (*y).t < 0 as libc::c_int as libc::c_longlong { -(*y).t } else { (*y).t } {
        2 => {
            if (*y).n != 0 {
                accF = *(((*y).k).as_mut_ptr() as *mut F);
            }
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = (*y).n - 1 as libc::c_int as libc::c_longlong;
            while i < _i {
                accF = accF
                    + *(((*y).k).as_mut_ptr() as *mut F)
                        .offset((i + 1 as libc::c_int as libc::c_longlong) as isize);
                i += 1;
                i;
            }
            z = Kf(accF);
        }
        _ => {
            if (*y).n != 0 {
                accI = *(((*y).k).as_mut_ptr() as *mut I);
            }
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = (*y).n - 1 as libc::c_int as libc::c_longlong;
            while i_0 < _i_0 {
                accI = accI
                    + *(((*y).k).as_mut_ptr() as *mut I)
                        .offset((i_0 + 1 as libc::c_int as libc::c_longlong) as isize);
                i_0 += 1;
                i_0;
            }
            z = Ki(accI);
        }
    }
    if z.is_null() {
        return 0 as K;
    }
    if !x.is_null() {
        r = plus(x, z);
        cd(z);
        z = r;
    }
    return z;
}
pub unsafe extern "C" fn times_over(mut x: K, mut y: K) -> K {
    let mut accI: I = 1 as libc::c_int as I;
    let mut accF: F = 1 as libc::c_int as F;
    if (*y).t == 0 && (*y).n != 0
        || (if (*y).t < 0 as libc::c_int as libc::c_longlong { -(*y).t } else { (*y).t })
            > 2 as libc::c_int as libc::c_longlong
        || !x.is_null() && (*x).t != 1 as libc::c_int as libc::c_longlong
            && (*x).t != 2 as libc::c_int as libc::c_longlong
    {
        return 0 as K;
    }
    let mut z: K = 0 as *mut k0;
    let mut r: K = 0 as *mut k0;
    match if (*y).t < 0 as libc::c_int as libc::c_longlong { -(*y).t } else { (*y).t } {
        2 => {
            if (*y).n != 0 {
                accF = *(((*y).k).as_mut_ptr() as *mut F);
            }
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = (*y).n - 1 as libc::c_int as libc::c_longlong;
            while i < _i {
                accF = accF
                    * *(((*y).k).as_mut_ptr() as *mut F)
                        .offset((i + 1 as libc::c_int as libc::c_longlong) as isize);
                i += 1;
                i;
            }
            z = Kf(accF);
        }
        _ => {
            if (*y).n != 0 {
                accI = *(((*y).k).as_mut_ptr() as *mut I);
            }
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = (*y).n - 1 as libc::c_int as libc::c_longlong;
            while i_0 < _i_0 {
                accI = accI
                    * *(((*y).k).as_mut_ptr() as *mut I)
                        .offset((i_0 + 1 as libc::c_int as libc::c_longlong) as isize);
                i_0 += 1;
                i_0;
            }
            z = Ki(accI);
        }
    }
    if z.is_null() {
        return 0 as K;
    }
    if !x.is_null() {
        r = times(x, z);
        cd(z);
        z = r;
    }
    return z;
}
pub unsafe extern "C" fn max_or_over(mut x: K, mut y: K) -> K {
    let mut accI: I = 0 as libc::c_int as I;
    let mut accF: F = -(1 as libc::c_int as libc::c_double / 0.0f64);
    if (*y).t == 0 && (*y).n != 0
        || (if (*y).t < 0 as libc::c_int as libc::c_longlong { -(*y).t } else { (*y).t })
            > 2 as libc::c_int as libc::c_longlong
        || !x.is_null() && (*x).t != 1 as libc::c_int as libc::c_longlong
            && (*x).t != 2 as libc::c_int as libc::c_longlong
    {
        return 0 as K;
    }
    let mut z: K = 0 as *mut k0;
    let mut r: K = 0 as *mut k0;
    match if (*y).t < 0 as libc::c_int as libc::c_longlong { -(*y).t } else { (*y).t } {
        2 => {
            if (*y).n != 0 {
                accF = *(((*y).k).as_mut_ptr() as *mut F);
            }
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = (*y).n - 1 as libc::c_int as libc::c_longlong;
            while i < _i {
                accF = if accF
                    > *(((*y).k).as_mut_ptr() as *mut F)
                        .offset((i + 1 as libc::c_int as libc::c_longlong) as isize)
                {
                    accF
                } else {
                    *(((*y).k).as_mut_ptr() as *mut F)
                        .offset((i + 1 as libc::c_int as libc::c_longlong) as isize)
                };
                i += 1;
                i;
            }
            z = Kf(accF);
        }
        _ => {
            if (*y).n != 0 {
                accI = *(((*y).k).as_mut_ptr() as *mut I);
            }
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = (*y).n - 1 as libc::c_int as libc::c_longlong;
            while i_0 < _i_0 {
                accI = if accI
                    > *(((*y).k).as_mut_ptr() as *mut I)
                        .offset((i_0 + 1 as libc::c_int as libc::c_longlong) as isize)
                {
                    accI
                } else {
                    *(((*y).k).as_mut_ptr() as *mut I)
                        .offset((i_0 + 1 as libc::c_int as libc::c_longlong) as isize)
                };
                i_0 += 1;
                i_0;
            }
            z = Ki(accI);
        }
    }
    if z.is_null() {
        return 0 as K;
    }
    if !x.is_null() {
        r = max_or(x, z);
        cd(z);
        z = r;
    }
    return z;
}
pub unsafe extern "C" fn min_and_over(mut x: K, mut y: K) -> K {
    let mut accI: I = 1 as libc::c_int as I;
    let mut accF: F = 1 as libc::c_int as libc::c_double / 0.0f64;
    if (*y).t == 0 && (*y).n != 0
        || (if (*y).t < 0 as libc::c_int as libc::c_longlong { -(*y).t } else { (*y).t })
            > 2 as libc::c_int as libc::c_longlong
        || !x.is_null() && (*x).t != 1 as libc::c_int as libc::c_longlong
            && (*x).t != 2 as libc::c_int as libc::c_longlong
    {
        return 0 as K;
    }
    let mut z: K = 0 as *mut k0;
    let mut r: K = 0 as *mut k0;
    match if (*y).t < 0 as libc::c_int as libc::c_longlong { -(*y).t } else { (*y).t } {
        2 => {
            if (*y).n != 0 {
                accF = *(((*y).k).as_mut_ptr() as *mut F);
            }
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = (*y).n - 1 as libc::c_int as libc::c_longlong;
            while i < _i {
                accF = if accF
                    < *(((*y).k).as_mut_ptr() as *mut F)
                        .offset((i + 1 as libc::c_int as libc::c_longlong) as isize)
                {
                    accF
                } else {
                    *(((*y).k).as_mut_ptr() as *mut F)
                        .offset((i + 1 as libc::c_int as libc::c_longlong) as isize)
                };
                i += 1;
                i;
            }
            z = Kf(accF);
        }
        _ => {
            if (*y).n != 0 {
                accI = *(((*y).k).as_mut_ptr() as *mut I);
            }
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = (*y).n - 1 as libc::c_int as libc::c_longlong;
            while i_0 < _i_0 {
                accI = if accI
                    < *(((*y).k).as_mut_ptr() as *mut I)
                        .offset((i_0 + 1 as libc::c_int as libc::c_longlong) as isize)
                {
                    accI
                } else {
                    *(((*y).k).as_mut_ptr() as *mut I)
                        .offset((i_0 + 1 as libc::c_int as libc::c_longlong) as isize)
                };
                i_0 += 1;
                i_0;
            }
            z = Ki(accI);
        }
    }
    if z.is_null() {
        return 0 as K;
    }
    if !x.is_null() {
        r = min_and(x, z);
        cd(z);
        z = r;
    }
    return z;
}
pub unsafe extern "C" fn join_over(mut x: K, mut y: K) -> K {
    let mut i: I = 0;
    let mut j: I = 0 as libc::c_int as I;
    let mut k: I = 0;
    i = 0 as libc::c_int as I;
    while i < (*y).n {
        j += (**((*y).k).as_mut_ptr().offset(i as isize)).n;
        i += 1;
        i;
    }
    let mut z: K = newK(0 as libc::c_int as I, j);
    if z.is_null() {
        return 0 as K;
    }
    i = 0 as libc::c_int as I;
    j = 0 as libc::c_int as I;
    while j < (*y).n {
        k = 0 as libc::c_int as I;
        while k < (**((*y).k).as_mut_ptr().offset(j as isize)).n {
            let ref mut fresh2 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh2 = ci(
                *((*(*((*y).k).as_mut_ptr().offset(j as isize) as K)).k)
                    .as_mut_ptr()
                    .offset(k as isize),
            );
            i += 1;
            i;
            k += 1;
            k;
        }
        j += 1;
        j;
    }
    return z;
}
pub static mut DT: [TR; 130] = unsafe {
    [
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: 0 as *const libc::c_void as V,
                text: 0 as *const C as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: ::std::mem::transmute::<
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
                text: 0 as *const C as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: 0 as *const libc::c_void as V,
                text: 0 as *const C as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: 0 as *const libc::c_void as V,
                text: 0 as *const C as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: 0 as *const libc::c_void as V,
                text: 0 as *const C as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: 0 as *const libc::c_void as V,
                text: 0 as *const C as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: 0 as *const libc::c_void as V,
                text: 0 as *const C as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 1 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> K>,
                    V,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn() -> K,
                            unsafe extern "C" fn() -> K,
                        >(over),
                    ),
                ),
                text: b"/\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 1 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> K>,
                    V,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn() -> K,
                            unsafe extern "C" fn() -> K,
                        >(scan),
                    ),
                ),
                text: b"\\\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 1 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> K>,
                    V,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn() -> K,
                            unsafe extern "C" fn() -> K,
                        >(each),
                    ),
                ),
                text: b"'\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 2 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> K>,
                    V,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn() -> K,
                            unsafe extern "C" fn() -> K,
                        >(eachright),
                    ),
                ),
                text: b"/:\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 2 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> K>,
                    V,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn() -> K,
                            unsafe extern "C" fn() -> K,
                        >(eachleft),
                    ),
                ),
                text: b"\\:\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 2 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> K>,
                    V,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn() -> K,
                            unsafe extern "C" fn() -> K,
                        >(eachpair),
                    ),
                ),
                text: b"':\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: 0 as *const libc::c_void as V,
                text: 0 as *const C as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: 0 as *const libc::c_void as V,
                text: 0 as *const C as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: 0 as *const libc::c_void as V,
                text: 0 as *const C as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: 0 as *const libc::c_void as V,
                text: 0 as *const C as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: 0 as *const libc::c_void as V,
                text: 0 as *const C as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: 0 as *const libc::c_void as V,
                text: 0 as *const C as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: 0 as *const libc::c_void as V,
                text: 0 as *const C as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 0 as libc::c_int as I,
                func: 0 as *const libc::c_void as V,
                text: 0 as *const C as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(flip as unsafe extern "C" fn(K) -> K)),
                text: b"+\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(plus as unsafe extern "C" fn(K, K) -> K)),
                text: b"+\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: ::std::mem::transmute::<
                            Option::<unsafe extern "C" fn(K, K) -> K>,
                            V,
                        >(Some(plus_over as unsafe extern "C" fn(K, K) -> K)),
                        verb_scan: ::std::mem::transmute::<
                            Option::<unsafe extern "C" fn(K, K) -> K>,
                            V,
                        >(Some(plus_scan as unsafe extern "C" fn(K, K) -> K)),
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(negate as unsafe extern "C" fn(K) -> K)),
                text: b"-\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(minus as unsafe extern "C" fn(K, K) -> K)),
                text: b"-\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as V,
                        verb_eachpair: ::std::mem::transmute::<
                            Option::<unsafe extern "C" fn(K, K) -> K>,
                            V,
                        >(Some(minus_eachpair as unsafe extern "C" fn(K, K) -> K)),
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(first as unsafe extern "C" fn(K) -> K)),
                text: b"*\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(times as unsafe extern "C" fn(K, K) -> K)),
                text: b"*\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: ::std::mem::transmute::<
                            Option::<unsafe extern "C" fn(K, K) -> K>,
                            V,
                        >(Some(times_over as unsafe extern "C" fn(K, K) -> K)),
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(reciprocal as unsafe extern "C" fn(K) -> K)),
                text: b"%%\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(divide as unsafe extern "C" fn(K, K) -> K)),
                text: b"%%\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(reverse as unsafe extern "C" fn(K) -> K)),
                text: b"|\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(max_or as unsafe extern "C" fn(K, K) -> K)),
                text: b"|\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: ::std::mem::transmute::<
                            Option::<unsafe extern "C" fn(K, K) -> K>,
                            V,
                        >(Some(max_or_over as unsafe extern "C" fn(K, K) -> K)),
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(where_0 as unsafe extern "C" fn(K) -> K)),
                text: b"&\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(min_and as unsafe extern "C" fn(K, K) -> K)),
                text: b"&\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: ::std::mem::transmute::<
                            Option::<unsafe extern "C" fn(K, K) -> K>,
                            V,
                        >(Some(min_and_over as unsafe extern "C" fn(K, K) -> K)),
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(shape as unsafe extern "C" fn(K) -> K)),
                text: b"^\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(power as unsafe extern "C" fn(K, K) -> K)),
                text: b"^\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(enumerate as unsafe extern "C" fn(K) -> K)),
                text: b"!\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(rotate_mod as unsafe extern "C" fn(K, K) -> K)),
                text: b"!\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(grade_up as unsafe extern "C" fn(K) -> K)),
                text: b"<\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(less as unsafe extern "C" fn(K, K) -> K)),
                text: b"<\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(grade_down as unsafe extern "C" fn(K) -> K)),
                text: b">\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(more as unsafe extern "C" fn(K, K) -> K)),
                text: b">\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(group as unsafe extern "C" fn(K) -> K)),
                text: b"=\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(equals as unsafe extern "C" fn(K, K) -> K)),
                text: b"=\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(not_attribute as unsafe extern "C" fn(K) -> K)),
                text: b"~\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(match_0 as unsafe extern "C" fn(K, K) -> K)),
                text: b"~\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(atom as unsafe extern "C" fn(K) -> K)),
                text: b"@\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(at as unsafe extern "C" fn(K, K) -> K)),
                text: b"@\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(range as unsafe extern "C" fn(K) -> K)),
                text: b"?\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(what as unsafe extern "C" fn(K, K) -> K)),
                text: b"?\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(floor_verb as unsafe extern "C" fn(K) -> K)),
                text: b"_\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(drop_cut as unsafe extern "C" fn(K, K) -> K)),
                text: b"_\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(enlist as unsafe extern "C" fn(K) -> K)),
                text: b",\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(join as unsafe extern "C" fn(K, K) -> K)),
                text: b",\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: ::std::mem::transmute::<
                            Option::<unsafe extern "C" fn(K, K) -> K>,
                            V,
                        >(Some(join_over as unsafe extern "C" fn(K, K) -> K)),
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(count as unsafe extern "C" fn(K) -> K)),
                text: b"#\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(take_reshape as unsafe extern "C" fn(K, K) -> K)),
                text: b"#\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(format as unsafe extern "C" fn(K) -> K)),
                text: b"$\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(dollar as unsafe extern "C" fn(K, K) -> K)),
                text: b"$\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(dot_monadic as unsafe extern "C" fn(K) -> K)),
                text: b".\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(dot as unsafe extern "C" fn(K, K) -> K)),
                text: b".\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(colon_monadic as unsafe extern "C" fn(K) -> K)),
                text: b":\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(colon_dyadic as unsafe extern "C" fn(K, K) -> K)),
                text: b":\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_0m as unsafe extern "C" fn(K) -> K)),
                text: b"0:\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_0d as unsafe extern "C" fn(K, K) -> K)),
                text: b"0:\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_1m as unsafe extern "C" fn(K) -> K)),
                text: b"1:\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_1d as unsafe extern "C" fn(K, K) -> K)),
                text: b"1:\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_2m as unsafe extern "C" fn(K) -> K)),
                text: b"2:\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_2d as unsafe extern "C" fn(K, K) -> K)),
                text: b"2:\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_3m as unsafe extern "C" fn(K) -> K)),
                text: b"3:\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_3d as unsafe extern "C" fn(K, K) -> K)),
                text: b"3:\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_4m as unsafe extern "C" fn(K) -> K)),
                text: b"4:\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_4d as unsafe extern "C" fn(K, K) -> K)),
                text: b"4:\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_5m as unsafe extern "C" fn(K) -> K)),
                text: b"5:\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_5d as unsafe extern "C" fn(K, K) -> K)),
                text: b"5:\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_6m as unsafe extern "C" fn(K) -> K)),
                text: b"6:\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_6d as unsafe extern "C" fn(K, K) -> K)),
                text: b"6:\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_acos as unsafe extern "C" fn(K) -> K)),
                text: b"_acos\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_asin as unsafe extern "C" fn(K) -> K)),
                text: b"_asin\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_atan as unsafe extern "C" fn(K) -> K)),
                text: b"_atan\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_ceil as unsafe extern "C" fn(K) -> K)),
                text: b"_ceil\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_cos as unsafe extern "C" fn(K) -> K)),
                text: b"_cos\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_cosh as unsafe extern "C" fn(K) -> K)),
                text: b"_cosh\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_exp as unsafe extern "C" fn(K) -> K)),
                text: b"_exp\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_floor as unsafe extern "C" fn(K) -> K)),
                text: b"_floor\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_log as unsafe extern "C" fn(K) -> K)),
                text: b"_log\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_sin as unsafe extern "C" fn(K) -> K)),
                text: b"_sin\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_sinh as unsafe extern "C" fn(K) -> K)),
                text: b"_sinh\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_sqr as unsafe extern "C" fn(K) -> K)),
                text: b"_sqr\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_sqrt as unsafe extern "C" fn(K) -> K)),
                text: b"_sqrt\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_tan as unsafe extern "C" fn(K) -> K)),
                text: b"_tan\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_tanh as unsafe extern "C" fn(K) -> K)),
                text: b"_tanh\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_abs as unsafe extern "C" fn(K) -> K)),
                text: b"_abs\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_bd as unsafe extern "C" fn(K) -> K)),
                text: b"_bd\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_ceiling as unsafe extern "C" fn(K) -> K)),
                text: b"_ceiling\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_ci as unsafe extern "C" fn(K) -> K)),
                text: b"_ci\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_db as unsafe extern "C" fn(K) -> K)),
                text: b"_db\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_dj as unsafe extern "C" fn(K) -> K)),
                text: b"_dj\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_kona_exit as unsafe extern "C" fn(K) -> K)),
                text: b"_exit\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_getenv as unsafe extern "C" fn(K) -> K)),
                text: b"_getenv\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_gtime as unsafe extern "C" fn(K) -> K)),
                text: b"_gtime\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_host as unsafe extern "C" fn(K) -> K)),
                text: b"_host\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_ic as unsafe extern "C" fn(K) -> K)),
                text: b"_ic\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_inv as unsafe extern "C" fn(K) -> K)),
                text: b"_inv\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_jd as unsafe extern "C" fn(K) -> K)),
                text: b"_jd\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_lt as unsafe extern "C" fn(K) -> K)),
                text: b"_lt\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_ltime as unsafe extern "C" fn(K) -> K)),
                text: b"_ltime\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_size as unsafe extern "C" fn(K) -> K)),
                text: b"_size\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_bin as unsafe extern "C" fn(K, K) -> K)),
                text: b"_bin\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_binl as unsafe extern "C" fn(K, K) -> K)),
                text: b"_binl\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_di as unsafe extern "C" fn(K, K) -> K)),
                text: b"_di\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_dot as unsafe extern "C" fn(K, K) -> K)),
                text: b"_dot\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_draw as unsafe extern "C" fn(K, K) -> K)),
                text: b"_draw\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_dv as unsafe extern "C" fn(K, K) -> K)),
                text: b"_dv\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_dvl as unsafe extern "C" fn(K, K) -> K)),
                text: b"_dvl\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_hat as unsafe extern "C" fn(K, K) -> K)),
                text: b"_hat\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_in as unsafe extern "C" fn(K, K) -> K)),
                text: b"_in\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_lin as unsafe extern "C" fn(K, K) -> K)),
                text: b"_lin\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_lsq as unsafe extern "C" fn(K, K) -> K)),
                text: b"_lsq\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_mul as unsafe extern "C" fn(K, K) -> K)),
                text: b"_mul\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_setenv as unsafe extern "C" fn(K, K) -> K)),
                text: b"_setenv\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_sm as unsafe extern "C" fn(K, K) -> K)),
                text: b"_sm\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_ss as unsafe extern "C" fn(K, K) -> K)),
                text: b"_ss\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_sv as unsafe extern "C" fn(K, K) -> K)),
                text: b"_sv\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_vsx as unsafe extern "C" fn(K, K) -> K)),
                text: b"_vsx\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 3 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K, K) -> K>,
                    V,
                >(Some(_ssr as unsafe extern "C" fn(K, K, K) -> K)),
                text: b"_ssr\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(tr_st as unsafe extern "C" fn(K) -> K)),
                text: b" \\\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(rtrn as unsafe extern "C" fn(K) -> K)),
                text: b":\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 2 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                    V,
                >(Some(_vs as unsafe extern "C" fn(K, K) -> K)),
                text: b"_vs\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(_hash as unsafe extern "C" fn(K) -> K)),
                text: b"_hash\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: 0 as libc::c_int as I,
                arity: 1 as libc::c_int as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(K) -> K>,
                    V,
                >(Some(resume as unsafe extern "C" fn(K) -> K)),
                text: b":\0" as *const u8 as *const libc::c_char as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
        {
            let mut init = tr {
                adverbClass: -(1 as libc::c_int) as I,
                arity: -(1 as libc::c_int) as I,
                func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> K>,
                    V,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn() -> K,
                            unsafe extern "C" fn() -> K,
                        >(TABLE_END),
                    ),
                ),
                text: 0 as *const C as S,
                alt_funcs: {
                    let mut init = af {
                        verb_over: 0 as *const libc::c_void as V,
                        verb_scan: 0 as *const libc::c_void as *mut libc::c_void,
                        verb_eachpair: 0 as *const libc::c_void as *mut libc::c_void,
                    };
                    init
                },
            };
            init
        },
    ]
};
pub unsafe extern "C" fn TABLE_END() -> K {
    return 0 as K;
}
pub static mut DT_SIZE: L = 0 as libc::c_int as L;
pub static mut DT_END_OFFSET: L = 0;
pub static mut DT_ADVERB_OFFSET: L = 0;
pub static mut DT_VERB_OFFSET: L = 0;
pub static mut DT_SPECIAL_VERB_OFFSET: L = 0;
pub unsafe extern "C" fn DT_OFFSET(mut v: V) -> L {
    let mut i: I = 0 as libc::c_int as I;
    while v != DT[i as usize].func {
        i += 1;
        i;
    }
    return i;
}
pub static mut kreci: I = 0 as libc::c_int as I;
pub unsafe extern "C" fn tf(mut n: N) {
    if n.is_null() {
        return;
    }
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = 2 as libc::c_int as I;
    while i < _i {
        tf((*n).c[i as usize]);
        i += 1;
        i;
    }
    if !((*n).k).is_null() {
        free(
            ((*n).k)
                .offset(
                    -((2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong)
                        as isize),
                ),
        );
    }
    repool(n as V, lsz(::std::mem::size_of::<Node>() as libc::c_ulong as I));
}
pub static mut krec: [V; 1000000] = [0 as *const libc::c_void
    as *mut libc::c_void; 1000000];
pub unsafe extern "C" fn finally() {
    nfinish();
    tf(SYMBOLS);
    cd(KTREE);
    cd(KFIXED);
}
