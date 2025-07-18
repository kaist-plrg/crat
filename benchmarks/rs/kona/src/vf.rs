use ::libc;
extern "C" {
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn cd(a: K) -> K;
    fn adverbClass(p: V) -> I;
    fn sva(p: V) -> I;
    fn charpos(s: S, c: C) -> I;
    fn enlist(x: K) -> K;
    fn FF(f: F) -> F;
    static mut PP: I;
    fn kerr(s: cS) -> K;
    static mut DT_SIZE: L;
    static mut DT_SPECIAL_VERB_OFFSET: L;
    static mut DT: [TR; 0];
    fn Ks(x: S) -> K;
    fn Kf(x: F) -> K;
    fn Ki(x: I) -> K;
    fn newK(t: I, n: I) -> K;
    fn ci(a: K) -> K;
    fn OOM_CD(g: I, _: ...) -> I;
    fn demote(a: K) -> K;
    fn promote(a: K) -> K;
    fn FC(a: F, b: F) -> I;
    fn kcloneI(a: K, f: *const libc::c_char, n: libc::c_int) -> K;
    fn sp(k: S) -> S;
    fn verbsChar(p: V) -> C;
    fn adverbsChar(p: V) -> C;
}
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
pub unsafe extern "C" fn CSK(mut x: K) -> S {
    return if x.is_null() {
        0 as S
    } else if 4 as libc::c_int as libc::c_longlong == (*x).t {
        *(((*x).k).as_mut_ptr() as *mut S)
    } else if 3 as libc::c_int as libc::c_longlong
        == (if (*x).t < 0 as libc::c_int as libc::c_longlong { -(*x).t } else { (*x).t })
    {
        ((*x).k).as_mut_ptr() as *mut C
    } else {
        0 as *mut C
    };
}
unsafe extern "C" fn formKsCS(mut s: S) -> K {
    let mut t: S = sp(s);
    if t.is_null() {
        return 0 as K;
    }
    let mut z: K = Ks(t);
    if z.is_null() {
        return 0 as K;
    }
    return z;
}
pub unsafe extern "C" fn formKiCS(mut s: S) -> K {
    let mut p: *mut C = 0 as *mut C;
    let mut q: C = 0 as libc::c_int as C;
    let mut r: I = -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong;
    let mut w: I = parseNI(s, strlen(s as *const libc::c_char) as I);
    if w != 0 {
        r = *NI.as_mut_ptr().offset(w as isize);
    } else if *s != 0 {
        r = strtoll(s as *const libc::c_char, &mut p, 10 as libc::c_int);
        *__errno_location() = 0 as libc::c_int;
        q = *p;
        if -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong == r {
            r = -(9223372036854775807 as libc::c_longlong);
        }
    }
    if q as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(q as libc::c_int as isize) as libc::c_int
            & _ISblank as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return 0 as K;
    }
    return Ki(r);
}
pub unsafe extern "C" fn formKfCS(mut s: S) -> K {
    let mut p: *mut C = 0 as *mut C;
    let mut q: C = 0 as libc::c_int as C;
    let mut r: F = 0 as libc::c_int as libc::c_double / 0.0f64;
    let mut w: I = parseNI(s, strlen(s as *const libc::c_char) as I);
    if w != 0 {
        r = *ni.as_mut_ptr().offset(w as isize);
    } else if *s != 0 {
        r = strtod(s as *const libc::c_char, &mut p);
        *__errno_location() = 0 as libc::c_int;
        q = *p;
        if r.is_nan() as i32 != 0 {
            r = -(1 as libc::c_int as libc::c_double / 0.0f64);
        }
    }
    if q as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(q as libc::c_int as isize) as libc::c_int
            & _ISblank as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return 0 as K;
    }
    return Kf(r);
}
unsafe extern "C" fn formatFn(mut a: K) -> K {
    let mut v: *mut V = ((*(*(((*a).k).as_mut_ptr() as *mut V)
        .offset(CODE as libc::c_int as isize) as K))
        .k)
        .as_mut_ptr() as *mut S as *mut V;
    let mut p: V = 0 as *mut libc::c_void;
    let mut i: I = 0;
    let mut k: I = 0;
    let mut n: I = 0;
    let mut r: I = 0 as libc::c_int as I;
    let mut z: K = 0 as K;
    let mut t: [C; 256] = *::std::mem::transmute::<
        &[u8; 256],
        &mut [C; 256],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut s: S = t.as_mut_ptr();
    match (*a).n {
        1 => {
            i = 0 as libc::c_int as I;
            loop {
                p = *v.offset(i as isize);
                if p.is_null() {
                    break;
                }
                let mut q: L = p as L;
                if q < DT_SIZE && q >= DT_SPECIAL_VERB_OFFSET {
                    let mut u: S = (*DT.as_mut_ptr().offset(q as isize)).text;
                    n = strlen(u as *const libc::c_char) as I;
                    strcpy(s.offset(r as isize), u as *const libc::c_char);
                    r += n;
                } else {
                    k = adverbClass(p);
                    if k != 0 {
                        t[r as usize] = adverbsChar(p);
                        if k != 1 as libc::c_int as libc::c_longlong {
                            t[(r + 1 as libc::c_int as libc::c_longlong)
                                as usize] = ':' as i32 as C;
                        }
                        r += 1;
                        r;
                    } else {
                        k = sva(p);
                        if k != 0 {
                            t[r as usize] = verbsChar(p);
                            if k != 2 as libc::c_int as libc::c_longlong {
                                t[(r + 1 as libc::c_int as libc::c_longlong)
                                    as usize] = ':' as i32 as C;
                            }
                            r += 1;
                            r;
                        }
                    }
                }
                i += 1;
                i;
            }
            n = strlen(s as *const libc::c_char) as I;
            z = newK(-(3 as libc::c_int) as I, n);
            memcpy(
                ((*z).k).as_mut_ptr() as *mut C as *mut libc::c_void,
                s as *const libc::c_void,
                (n + 1 as libc::c_int as libc::c_longlong) as libc::c_ulong,
            );
        }
        3 => {
            let mut f: S = ((*(*(((*a).k).as_mut_ptr() as *mut V)
                .offset(CODE as libc::c_int as isize) as K))
                .k)
                .as_mut_ptr() as *mut C;
            let mut n_0: I = strlen(f as *const libc::c_char) as I;
            z = newK(
                -(3 as libc::c_int) as I,
                n_0 + 2 as libc::c_int as libc::c_longlong,
            );
            *(((*z).k).as_mut_ptr() as *mut C)
                .offset(0 as libc::c_int as isize) = '{' as i32 as C;
            memcpy(
                (((*z).k).as_mut_ptr() as *mut C).offset(1 as libc::c_int as isize)
                    as *mut libc::c_void,
                f as *const libc::c_void,
                n_0 as libc::c_ulong,
            );
            *(((*z).k).as_mut_ptr() as *mut C)
                .offset(
                    (n_0 + 1 as libc::c_int as libc::c_longlong) as isize,
                ) = '}' as i32 as C;
            *(((*z).k).as_mut_ptr() as *mut C)
                .offset(
                    (n_0 + 2 as libc::c_int as libc::c_longlong) as isize,
                ) = 0 as libc::c_int as C;
        }
        2 | _ => {}
    }
    return z;
}
unsafe extern "C" fn formatS(mut x: S) -> K {
    let mut n: I = strlen(x as *const libc::c_char) as I;
    let mut z: K = newK(-(3 as libc::c_int) as I, n);
    if !z.is_null() {
        sprintf(
            ((*z).k).as_mut_ptr() as *mut C,
            b"%s\0" as *const u8 as *const libc::c_char,
            x,
        );
    }
    return z;
}
unsafe extern "C" fn formatF(mut x: F, mut y: I, mut c: I) -> K {
    static mut buf: [C; 32] = [0; 32];
    let mut k: libc::c_int = y as libc::c_int;
    let mut b: S = (if 0 as libc::c_int as libc::c_longlong == c {
        b"%.*g\0" as *const u8 as *const libc::c_char
    } else if 1 as libc::c_int as libc::c_longlong == c {
        b"%.*f\0" as *const u8 as *const libc::c_char
    } else {
        b"%.*e\0" as *const u8 as *const libc::c_char
    }) as S;
    sprintf(buf.as_mut_ptr(), b as *const libc::c_char, k, x);
    let mut n: I = strlen(buf.as_mut_ptr()) as I;
    let mut z: K = newK(-(3 as libc::c_int) as I, n);
    if !z.is_null() {
        memcpy(
            ((*z).k).as_mut_ptr() as *mut C as *mut libc::c_void,
            buf.as_mut_ptr() as *const libc::c_void,
            n as libc::c_ulong,
        );
    }
    return z;
}
unsafe extern "C" fn formatI(mut x: I) -> K {
    static mut buf: [C; 72] = [0; 72];
    sprintf(buf.as_mut_ptr(), b"%lld\0" as *const u8 as *const libc::c_char, x);
    let mut n: I = strlen(buf.as_mut_ptr()) as I;
    let mut z: K = newK(-(3 as libc::c_int) as I, n);
    if !z.is_null() {
        memcpy(
            ((*z).k).as_mut_ptr() as *mut C as *mut libc::c_void,
            buf.as_mut_ptr() as *const libc::c_void,
            n as libc::c_ulong,
        );
    }
    return z;
}
pub unsafe extern "C" fn format(mut a: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut z: K = 0 as *mut k0;
    if 3 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
    {
        z = kcloneI(
            a,
            b"src/vf.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
        );
        (*z).t = -(3 as libc::c_int) as I;
        return z;
    } else if 7 as libc::c_int as libc::c_longlong == at {
        return formatFn(a)
    } else if 6 as libc::c_int as libc::c_longlong == at {
        return newK(-(3 as libc::c_int) as I, 0 as libc::c_int as I)
    } else if 5 as libc::c_int as libc::c_longlong == at {
        return formatS(sp(b".(..)\0" as *const u8 as *const libc::c_char as S))
    } else if 4 as libc::c_int as libc::c_longlong == at {
        return formatS(*(((*a).k).as_mut_ptr() as *mut S))
    } else if 2 as libc::c_int as libc::c_longlong == at {
        return formatF(*(((*a).k).as_mut_ptr() as *mut F), PP, 0 as libc::c_int as I)
    } else if 1 as libc::c_int as libc::c_longlong == at {
        return formatI(*(((*a).k).as_mut_ptr() as *mut I))
    }
    z = newK(0 as libc::c_int as I, an);
    if 0 as libc::c_int as libc::c_longlong == at {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = an;
        while i < _i {
            let ref mut fresh0 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh0 = format(*((*a).k).as_mut_ptr().offset(i as isize));
            i += 1;
            i;
        }
    } else if -(1 as libc::c_int) as libc::c_longlong == at {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = an;
        while i_0 < _i_0 {
            let ref mut fresh1 = *((*z).k).as_mut_ptr().offset(i_0 as isize);
            *fresh1 = formatI(*(((*a).k).as_mut_ptr() as *mut I).offset(i_0 as isize));
            i_0 += 1;
            i_0;
        }
    } else if -(2 as libc::c_int) as libc::c_longlong == at {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = an;
        while i_1 < _i_1 {
            let ref mut fresh2 = *((*z).k).as_mut_ptr().offset(i_1 as isize);
            *fresh2 = formatF(
                *(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize),
                PP,
                0 as libc::c_int as I,
            );
            i_1 += 1;
            i_1;
        }
    } else if -(4 as libc::c_int) as libc::c_longlong == at {
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = an;
        while i_2 < _i_2 {
            let ref mut fresh3 = *((*z).k).as_mut_ptr().offset(i_2 as isize);
            *fresh3 = formatS(*(((*a).k).as_mut_ptr() as *mut S).offset(i_2 as isize));
            i_2 += 1;
            i_2;
        }
    }
    return z;
}
pub static mut NI: [I; 7] = [
    0 as libc::c_int as I,
    -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong,
    -(9223372036854775807 as libc::c_longlong),
    9223372036854775807 as libc::c_longlong,
    9223372036854775807 as libc::c_longlong,
    -(9223372036854775807 as libc::c_longlong),
    9223372036854775807 as libc::c_longlong,
];
pub static mut ni: [F; 7] = [
    0 as libc::c_int as F,
    0 as libc::c_int as libc::c_double / 0.0f64,
    -(1 as libc::c_int as libc::c_double / 0.0f64),
    1 as libc::c_int as libc::c_double / 0.0f64,
    0 as libc::c_int as libc::c_double / 0.0f64,
    -(1 as libc::c_int as libc::c_double / 0.0f64),
    1 as libc::c_int as libc::c_double / 0.0f64,
];
unsafe extern "C" fn TNI(mut p: I, mut h: C) -> I {
    let mut c: I = if *(*__ctype_b_loc()).offset(h as libc::c_int as isize)
        as libc::c_int & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        0 as libc::c_int as libc::c_longlong
    } else {
        charpos(b" -0NIni\0" as *const u8 as *const libc::c_char as S, h)
    };
    if 0 as libc::c_int as libc::c_longlong == c
        && 7 as libc::c_int as libc::c_longlong >= p
    {
        return p;
    }
    if 1 as libc::c_int as libc::c_longlong == c
        && (0 as libc::c_int as libc::c_longlong == p
            || 7 as libc::c_int as libc::c_longlong == p)
    {
        return 7 as libc::c_int as libc::c_longlong - p;
    }
    if 2 as libc::c_int as libc::c_longlong == c
        && 0 as libc::c_int as libc::c_longlong == p
    {
        return 9 as libc::c_int as I;
    }
    if 2 as libc::c_int as libc::c_longlong == c
        && 7 as libc::c_int as libc::c_longlong == p
    {
        return 8 as libc::c_int as I;
    }
    if 3 as libc::c_int as libc::c_longlong == c
        && (8 as libc::c_int as libc::c_longlong == p
            || 9 as libc::c_int as libc::c_longlong == p)
    {
        return 1 as libc::c_int as I;
    }
    if 4 as libc::c_int as libc::c_longlong == c
        && (8 as libc::c_int as libc::c_longlong == p
            || 9 as libc::c_int as libc::c_longlong == p)
    {
        return p - 6 as libc::c_int as libc::c_longlong;
    }
    if 5 as libc::c_int as libc::c_longlong == c
        && (8 as libc::c_int as libc::c_longlong == p
            || 9 as libc::c_int as libc::c_longlong == p)
    {
        return 4 as libc::c_int as I;
    }
    if 6 as libc::c_int as libc::c_longlong == c
        && (8 as libc::c_int as libc::c_longlong == p
            || 9 as libc::c_int as libc::c_longlong == p)
    {
        return p - 3 as libc::c_int as libc::c_longlong;
    }
    return 10 as libc::c_int as I;
}
unsafe extern "C" fn parseNI(mut s: S, mut n: I) -> I {
    let mut i: I = 0 as libc::c_int as I;
    let mut p: I = 0 as libc::c_int as I;
    while i < n && *s as libc::c_int != 0 {
        let fresh4 = s;
        s = s.offset(1);
        p = TNI(p, *fresh4);
    }
    return if p < 7 as libc::c_int as libc::c_longlong {
        p
    } else {
        0 as libc::c_int as libc::c_longlong
    };
}
unsafe extern "C" fn tround(mut f: F) -> F {
    let mut d: F = FF(f);
    return if d > 0 as libc::c_int as libc::c_double && FC(d, 1 as libc::c_int as F) == 0
        || d < 0 as libc::c_int as libc::c_double && FC(d, 0 as libc::c_int as F) == 0
    {
        ceil(f)
    } else {
        floor(f)
    };
}
pub unsafe extern "C" fn dollar(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    let mut z: K = 0 as K;
    let mut x: I = (at <= 0 as libc::c_int as libc::c_longlong
        && -(3 as libc::c_int) as libc::c_longlong != at) as libc::c_int as I;
    let mut y: I = (bt <= 0 as libc::c_int as libc::c_longlong
        && -(3 as libc::c_int) as libc::c_longlong != bt) as libc::c_int as I;
    if x != 0 && y != 0 && an != bn {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    if x != 0 || y != 0 {
        a = if x != 0 { promote(a) } else { ci(a) };
        b = if y != 0 { promote(b) } else { ci(b) };
        z = if !a.is_null() && !b.is_null() {
            newK(0 as libc::c_int as I, if x != 0 { (*a).n } else { (*b).n })
        } else {
            0 as K
        };
        if !z.is_null() {
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = (*z).n;
            while i < _i {
                let mut q: K = dollar(
                    if x != 0 { *((*a).k).as_mut_ptr().offset(i as isize) } else { a },
                    if y != 0 { *((*b).k).as_mut_ptr().offset(i as isize) } else { b },
                );
                if OOM_CD(0 as libc::c_int as I, q, z, a, b, -(1 as libc::c_int) as V)
                    == 0
                {
                    return 0 as K;
                }
                let ref mut fresh5 = *((*z).k).as_mut_ptr().offset(i as isize);
                *fresh5 = q;
                i += 1;
                i;
            }
        }
        cd(a);
        cd(b);
        return demote(z);
    }
    if 1 as libc::c_int as libc::c_longlong == at
        && *(((*a).k).as_mut_ptr() as *mut I) != 0
    {
        let mut c: K = 0 as *mut k0;
        c = format(b);
        if c.is_null() {
            return 0 as K;
        }
        let mut m: I = *(((*a).k).as_mut_ptr() as *mut I);
        z = newK(
            -(3 as libc::c_int) as I,
            if m < 0 as libc::c_int as libc::c_longlong { -m } else { m },
        );
        if OOM_CD(0 as libc::c_int as I, c, z, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        if (*z).n < (*c).n {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = (*z).n;
            while i_0 < _i_0 {
                *(((*z).k).as_mut_ptr() as *mut C)
                    .offset(i_0 as isize) = '*' as i32 as C;
                i_0 += 1;
                i_0;
            }
        } else {
            let mut k: I = if m > 0 as libc::c_int as libc::c_longlong {
                m - (*c).n
            } else {
                0 as libc::c_int as libc::c_longlong
            };
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = (*z).n;
            while i_1 < _i_1 {
                *(((*z).k).as_mut_ptr() as *mut C)
                    .offset(i_1 as isize) = ' ' as i32 as C;
                i_1 += 1;
                i_1;
            }
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = (*c).n;
            while i_2 < _i_2 {
                *(((*z).k).as_mut_ptr() as *mut C)
                    .offset(
                        (i_2 + k) as isize,
                    ) = *(((*c).k).as_mut_ptr() as *mut C).offset(i_2 as isize);
                i_2 += 1;
                i_2;
            }
        }
        cd(c);
        return z;
    }
    if 2 as libc::c_int as libc::c_longlong == at {
        let mut f: F = *(((*a).k).as_mut_ptr() as *mut F);
        if 2 as libc::c_int as libc::c_longlong == bt
            || 1 as libc::c_int as libc::c_longlong == bt
        {
            let mut c_0: K = 0 as *mut k0;
            let mut d: K = 0 as *mut k0;
            c_0 = Ki(f as I);
            if c_0.is_null() {
                return 0 as K;
            }
            d = formatF(
                if 2 as libc::c_int as libc::c_longlong == bt {
                    *(((*b).k).as_mut_ptr() as *mut F)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut I) as libc::c_double
                },
                tround(fabs(f) * 10 as libc::c_int as libc::c_double) as I
                    % 10 as libc::c_int as libc::c_longlong,
                (if f.is_sign_negative() as libc::c_int != 0 {
                    2 as libc::c_int
                } else {
                    1 as libc::c_int
                }) as I,
            );
            if !d.is_null() {
                z = dollar(c_0, d);
            }
            cd(c_0);
            cd(d);
            return z;
        }
    }
    if 3 as libc::c_int as libc::c_longlong
        == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if 3 as libc::c_int as libc::c_longlong == bt {
            b = enlist(b);
        }
        if 4 as libc::c_int as libc::c_longlong == at
            && strlen(*(((*a).k).as_mut_ptr() as *mut S) as *const libc::c_char) == 0
        {
            return formKsCS(CSK(b));
        }
        if 3 as libc::c_int as libc::c_longlong
            == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        {
            return ci(b);
        }
        if 2 as libc::c_int as libc::c_longlong == at {
            return formKfCS(CSK(b));
        }
        if 1 as libc::c_int as libc::c_longlong == at
            && *(((*a).k).as_mut_ptr() as *mut I) == 0
        {
            return formKiCS(CSK(b));
        }
        return 0 as K;
    }
    return kerr(b"type\0" as *const u8 as *const libc::c_char);
}
