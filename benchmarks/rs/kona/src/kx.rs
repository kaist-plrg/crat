use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn _ssr(a: K, b: K, c: K) -> K;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn cd(a: K) -> K;
    fn show(a: K) -> K;
    fn VA(p: V) -> I;
    fn valence(p: V) -> I;
    fn adverbClass(p: V) -> I;
    fn sva(p: V) -> I;
    static mut offsetSSR: V;
    static mut offsetWhat: V;
    static mut offsetAt: V;
    static mut offsetDot: V;
    static mut offsetColon: V;
    fn dot(a: K, b: K) -> K;
    fn join(a: K, b: K) -> K;
    fn what(x: K, y: K) -> K;
    fn dot_monadic(x: K) -> K;
    fn enlist(x: K) -> K;
    fn enumerate(a: K) -> K;
    fn first(a: K) -> K;
    fn bk(p: V) -> I;
    static mut LS: S;
    static mut KTREE: K;
    fn kerr(s: cS) -> K;
    static mut DT_SIZE: L;
    static mut DT_END_OFFSET: L;
    static mut DT_ADVERB_OFFSET: L;
    static mut DT_VERB_OFFSET: L;
    static mut DT: [TR; 0];
    static mut offsetOver: L;
    static mut offsetScan: L;
    static mut offsetEach: L;
    static mut offsetEachright: L;
    static mut offsetEachleft: L;
    static mut offsetEachpair: L;
    fn rc(x: K) -> I;
    fn newE(s: S, k: K) -> K;
    fn _n() -> K;
    static mut errmsg: [C; 256];
    static mut fll: I;
    fn sp(k: S) -> S;
    static mut offsetJoin: V;
    static mut offset3m: V;
    fn kap(a: *mut K, v: V) -> K;
    fn newK(t: I, n: I) -> K;
    fn promote(a: K) -> K;
    fn demote(a: K) -> K;
    fn ci(a: K) -> K;
    fn collapse(x: K) -> K;
    fn delist(x: K) -> K;
    fn EVP(e: K) -> *mut K;
    fn Kv() -> K;
    fn kcloneI(a: K, f: *const libc::c_char, n: libc::c_int) -> K;
    fn of(a: K, b: K) -> K;
    fn dot_tetradic_2(g: *mut K, b: K, c: K, y: K) -> K;
    static mut fBreak: S;
    static mut fCheck: I;
    fn itemAtIndex(a: K, i: I) -> K;
    fn wd_(s: S, n: libc::c_int, dict: *mut K, func: K) -> K;
    fn DI(d: K, i: I) -> K;
    fn dot_tetradic(a: K, b: K, c: K, y: K) -> K;
    fn at_tetradic(a: K, b: K, c: K, y: K) -> K;
    fn what_triadic(a: K, b: K, c: K) -> K;
    #[link_name = "drop"]
    fn drop_0(a: K, b: K) -> K;
    fn atomI(a: K) -> I;
    fn matchI(a: K, b: K) -> I;
    fn bp(t: I) -> I;
    fn Ki(x: I) -> K;
    fn last(a: K) -> K;
    fn kdefClass(n: I) -> I;
    fn OOM_CD(g: I, _: ...) -> I;
    static mut interrupted: sig_atomic_t;
    fn kdef(v: I) -> K;
}
pub type __sig_atomic_t = libc::c_int;
pub type L = libc::c_longlong;
pub type UI = libc::c_ulonglong;
pub type V = *mut libc::c_void;
pub type I = libc::c_longlong;
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
pub type sig_atomic_t = __sig_atomic_t;
pub static mut fer: I = 0 as libc::c_int as I;
pub static mut fer1: I = 0 as libc::c_int as I;
pub static mut fwh: I = 0 as libc::c_int as I;
pub static mut stk: I = 0 as libc::c_int as I;
pub static mut stk1: I = 0 as libc::c_int as I;
pub static mut prj: I = 0 as libc::c_int as I;
pub static mut prj2: I = 0 as libc::c_int as I;
pub static mut prnt: K = 0 as *const k0 as K;
pub static mut fsf: I = 0 as libc::c_int as I;
pub static mut grnt: K = 0 as *const k0 as K;
pub static mut cls: K = 0 as *const k0 as K;
pub static mut encf: K = 0 as *const k0 as K;
pub static mut encp: I = 0 as libc::c_int as I;
pub static mut frg: I = 0 as libc::c_int as I;
pub static mut fnc: S = 0 as *const C as S;
pub static mut fncp: [V; 128] = [0 as *const libc::c_void as *mut libc::c_void; 128];
pub static mut fnci: I = 0 as libc::c_int as I;
pub static mut fom: I = 0 as libc::c_int as I;
pub static mut fam: I = 1 as libc::c_int as I;
pub static mut ft3: I = 0 as libc::c_int as I;
pub static mut cdp: [C; 11] = unsafe {
    *::std::mem::transmute::<&[u8; 11], &mut [C; 11]>(b"aaaaaaaaaa\0")
};
pub static mut calf: I = -(1 as libc::c_int) as I;
pub static mut alf: *mut C = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ\0"
    as *const u8 as *const libc::c_char as *mut C;
pub unsafe extern "C" fn sd_(mut x: K, mut f: I) -> K {
    if !x.is_null() {
        if bk(x as V) == 0 {
            if (*x).t == 4 as libc::c_int as libc::c_longlong {
                printf(
                    b"     %p %p %p  %lld-%lld %lld %lld   \0" as *const u8
                        as *const libc::c_char,
                    x,
                    ((*x).k).as_mut_ptr(),
                    *((*x).k).as_mut_ptr(),
                    (*x)._c >> 8 as libc::c_int,
                    (*x)._c << 56 as libc::c_int >> 56 as libc::c_int,
                    (*x).t,
                    (*x).n,
                );
            } else {
                printf(
                    b"     %p %p            %lld-%lld %lld %lld   \0" as *const u8
                        as *const libc::c_char,
                    x,
                    ((*x).k).as_mut_ptr(),
                    (*x)._c >> 8 as libc::c_int,
                    (*x)._c << 56 as libc::c_int >> 56 as libc::c_int,
                    (*x).t,
                    (*x).n,
                );
            }
            if f > 0 as libc::c_int as libc::c_longlong
                && ((*x).t == 0 as libc::c_int as libc::c_longlong
                    || (*x).t == 5 as libc::c_int as libc::c_longlong)
            {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            if (*x).t != 6 as libc::c_int as libc::c_longlong
                && f > 0 as libc::c_int as libc::c_longlong
            {
                show(x);
            } else {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
        } else {
            printf(b" is ; or \\n\n\0" as *const u8 as *const libc::c_char);
            return x;
        }
    } else {
        printf(b"     \0" as *const u8 as *const libc::c_char);
        show(x);
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        return x;
    }
    if f < 2 as libc::c_int as libc::c_longlong {
        return 0 as K;
    }
    match (*x).t {
        7 => {
            calf += 1;
            calf;
            printf(
                b"     %c0:    %p     %s\n\0" as *const u8 as *const libc::c_char,
                *alf.offset(calf as isize) as libc::c_int,
                &mut *(((*x).k).as_mut_ptr() as *mut S)
                    .offset(CONTeXT as libc::c_int as isize) as *mut S,
                *(((*x).k).as_mut_ptr() as *mut S)
                    .offset(CONTeXT as libc::c_int as isize),
            );
            printf(
                b"     %c1:    %p     %p\n\0" as *const u8 as *const libc::c_char,
                *alf.offset(calf as isize) as libc::c_int,
                &mut *(((*x).k).as_mut_ptr() as *mut V)
                    .offset(DEPTH as libc::c_int as isize) as *mut V,
                *(((*x).k).as_mut_ptr() as *mut V).offset(DEPTH as libc::c_int as isize),
            );
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = (-(2 as libc::c_int) + TYPE_SEVEN_SIZE as libc::c_int) as I;
            while i < _i {
                printf(
                    b"     %c%lld:   \0" as *const u8 as *const libc::c_char,
                    *alf.offset(calf as isize) as libc::c_int,
                    2 as libc::c_int as libc::c_longlong + i,
                );
                printf(
                    b" %p\0" as *const u8 as *const libc::c_char,
                    &mut *(((*x).k).as_mut_ptr() as *mut V)
                        .offset((2 as libc::c_int as libc::c_longlong + i) as isize)
                        as *mut V,
                );
                sd_(
                    *(((*x).k).as_mut_ptr() as *mut V)
                        .offset((2 as libc::c_int as libc::c_longlong + i) as isize)
                        as K,
                    3 as libc::c_int as I,
                );
                i += 1;
                i;
            }
            calf -= 1;
            calf;
        }
        -4 => {
            let mut v: *mut V = ((*x).k).as_mut_ptr() as *mut V;
            if (*v.offset(0 as libc::c_int as isize) > 0x10 as libc::c_int as V)
                as libc::c_int
                & (*v.offset(0 as libc::c_int as isize) < 0x5000000 as libc::c_int as V)
                    as libc::c_int != 0
            {
                return 0 as K;
            }
            let mut ii: I = 0;
            ii = 0 as libc::c_int as I;
            while !(*v.offset(ii as isize)).is_null() {
                printf(
                    b"     .2%c[%lld]: %p\0" as *const u8 as *const libc::c_char,
                    *alf.offset(calf as isize) as libc::c_int,
                    ii,
                    *v.offset(ii as isize),
                );
                if *v.offset(ii as isize) > DT_SIZE as V {
                    if calf < 1 as libc::c_int as libc::c_longlong {
                        sd_(*(*v.offset(ii as isize) as *mut K), 2 as libc::c_int as I);
                    } else {
                        sd_(*(*v.offset(ii as isize) as *mut K), 1 as libc::c_int as I);
                    }
                } else {
                    printf(b"\n\0" as *const u8 as *const libc::c_char);
                }
                ii += 1;
                ii;
            }
        }
        5 | 0 => {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = (*x).n;
            while i_0 < _i_0 {
                printf(
                    b" %p\0" as *const u8 as *const libc::c_char,
                    &mut *((*x).k).as_mut_ptr().offset(i_0 as isize) as *mut *mut k0,
                );
                sd_(*((*x).k).as_mut_ptr().offset(i_0 as isize), 2 as libc::c_int as I);
                i_0 += 1;
                i_0;
            }
        }
        _ => {}
    }
    return 0 as K;
}
pub unsafe extern "C" fn sd(mut x: K) -> K {
    return sd_(x, 1 as libc::c_int as I);
}
unsafe extern "C" fn cjoin(mut x: K, mut y: K) -> K {
    if 3 as libc::c_int as libc::c_longlong != (*x).t {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    if 3 as libc::c_int as libc::c_longlong
        == (if (*y).t < 0 as libc::c_int as libc::c_longlong { -(*y).t } else { (*y).t })
    {
        return ci(y);
    }
    if (*y).t != 0 {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    if (*y).n == 0 {
        return newK(-(3 as libc::c_int) as I, 0 as libc::c_int as I);
    }
    let mut zn: I = 0 as libc::c_int as I;
    let mut v: K = 0 as *mut k0;
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = (*y).n;
    while i < _i {
        v = *((*y).k).as_mut_ptr().offset(i as isize);
        if -(3 as libc::c_int) as libc::c_longlong != (*v).t {
            return kerr(b"type\0" as *const u8 as *const libc::c_char);
        }
        zn += (*v).n;
        i += 1;
        i;
    }
    zn
        += if (*y).n != 0 {
            ((*y).n - 1 as libc::c_int as libc::c_longlong) * (*x).n
        } else {
            0 as libc::c_int as libc::c_longlong
        };
    let mut z: K = newK(-(3 as libc::c_int) as I, zn);
    if OOM_CD(0 as libc::c_int as I, z, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut p: S = ((*z).k).as_mut_ptr() as *mut C;
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*y).n - 1 as libc::c_int as libc::c_longlong;
    while i_0 < _i_0 {
        v = *((*y).k).as_mut_ptr().offset(i_0 as isize);
        memcpy(
            p as *mut libc::c_void,
            ((*v).k).as_mut_ptr() as *mut C as *const libc::c_void,
            (*v).n as libc::c_ulong,
        );
        p = p.offset((*v).n as isize);
        memcpy(
            p as *mut libc::c_void,
            ((*x).k).as_mut_ptr() as *mut C as *const libc::c_void,
            (*x).n as libc::c_ulong,
        );
        p = p.offset((*x).n as isize);
        i_0 += 1;
        i_0;
    }
    v = *((*y).k)
        .as_mut_ptr()
        .offset(((*y).n - 1 as libc::c_int as libc::c_longlong) as isize);
    memcpy(
        p as *mut libc::c_void,
        ((*v).k).as_mut_ptr() as *mut C as *const libc::c_void,
        (*v).n as libc::c_ulong,
    );
    return z;
}
unsafe extern "C" fn csplit(mut x: K, mut y: K) -> K {
    if 3 as libc::c_int as libc::c_longlong != (*x).t {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    if 3 as libc::c_int as libc::c_longlong
        != (if (*y).t < 0 as libc::c_int as libc::c_longlong { -(*y).t } else { (*y).t })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut delim: libc::c_int = *(((*x).k).as_mut_ptr() as *mut C) as libc::c_int;
    let mut s: S = ((*y).k).as_mut_ptr() as *mut C;
    let mut p0: I = 0;
    let mut p1: I = 0;
    let mut zn: I = 0 as libc::c_int as I;
    let mut i: I = 0 as libc::c_int as I;
    while i < (*y).n {
        let mut j: I = i;
        let mut n: I = 0 as libc::c_int as I;
        while i < (*y).n && delim != *s.offset(i as isize) as libc::c_int {
            i += 1;
            i;
            n += 1;
            n;
        }
        p0 = j;
        p1 = n;
        zn += 1;
        zn;
        if i < (*y).n && delim == *s.offset(i as isize) as libc::c_int {
            i += 1;
            i;
        }
    }
    if (*y).n != 0
        && delim
            == *s.offset(((*y).n - 1 as libc::c_int as libc::c_longlong) as isize)
                as libc::c_int
    {
        zn += 1;
        zn;
    }
    if zn == 0 {
        return newK(0 as libc::c_int as I, 0 as libc::c_int as I)
    } else if 1 as libc::c_int as libc::c_longlong == zn {
        if (*y).n == p1 {
            return enlist(y);
        }
        let mut z: K = newK(-(3 as libc::c_int) as I, p1);
        if OOM_CD(0 as libc::c_int as I, z, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        memcpy(
            ((*z).k).as_mut_ptr() as *mut C as *mut libc::c_void,
            s.offset(p0 as isize) as *const libc::c_void,
            p1 as libc::c_ulong,
        );
        y = enlist(z);
        cd(z);
        return y;
    }
    let mut j_0: I = 0 as libc::c_int as I;
    let mut z_0: K = newK(0 as libc::c_int as I, zn);
    if OOM_CD(0 as libc::c_int as I, z_0, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i: I = zn;
    while i_0 < _i {
        p0 = j_0;
        p1 = 0 as libc::c_int as I;
        while j_0 < (*y).n && delim != *s.offset(j_0 as isize) as libc::c_int {
            j_0 += 1;
            j_0;
            p1 += 1;
            p1;
        }
        let mut d: K = newK(-(3 as libc::c_int) as I, p1);
        if OOM_CD(0 as libc::c_int as I, d, z_0, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        memcpy(
            ((*d).k).as_mut_ptr() as *mut C as *mut libc::c_void,
            s.offset(p0 as isize) as *const libc::c_void,
            p1 as libc::c_ulong,
        );
        let ref mut fresh0 = *((*z_0).k).as_mut_ptr().offset(i_0 as isize);
        *fresh0 = d;
        if j_0 < (*y).n && delim == *s.offset(j_0 as isize) as libc::c_int {
            j_0 += 1;
            j_0;
        }
        i_0 += 1;
        i_0;
    }
    return z_0;
}
pub unsafe extern "C" fn overDyad(mut a: K, mut p: *mut V, mut b: K) -> K {
    let mut c: K = 0 as *mut k0;
    let mut d: K = 0 as *mut k0;
    let mut current_block: u64;
    let mut o: *mut V = p.offset(-(1 as libc::c_int as isize));
    let mut f: Option::<unsafe extern "C" fn(K, K) -> K> = None;
    let mut k: K = 0 as K;
    let mut i: I = 0 as libc::c_int as I;
    if !a.is_null() && *o == offsetJoin && (*b).t == 0 && (*b).n == 0 {
        return if (0 as libc::c_int as libc::c_longlong) < (*a).t {
            enlist(a)
        } else {
            ci(a)
        };
    }
    if (*b).t == 0 as libc::c_int as libc::c_longlong {
        while i < (*b).n && (**((*b).k).as_mut_ptr().offset(i as isize)).t == 0 {
            i += 1;
            i;
        }
    }
    if *o != offsetJoin || *o == offsetJoin && i == (*b).n {
        if VA(*o) != 0
            && {
                f = ::std::mem::transmute::<
                    V,
                    Option::<unsafe extern "C" fn(K, K) -> K>,
                >((*DT.as_mut_ptr().offset(*o as L as isize)).alt_funcs.verb_over);
                f.is_some()
            }
        {
            k = f.unwrap()(a, b);
        }
    }
    if !k.is_null() {
        return k;
    }
    let mut g: K = 0 as K;
    let mut u: K = 0 as K;
    let mut v: K = 0 as K;
    let mut z: K = 0 as K;
    let mut y: K = if !a.is_null() {
        u = enlist(a);
        v = join(u, b);
        v
    } else {
        b
    };
    if (*y).t > 0 as libc::c_int as libc::c_longlong {
        z = ci(y);
    } else if *o == 0x2a as libc::c_int as V && a.is_null()
        && (*y).t == -(1 as libc::c_int) as libc::c_longlong
        && (*y).n == 0 as libc::c_int as libc::c_longlong
    {
        z = Ki(1 as libc::c_int as I);
    } else if (*y).n == 0 as libc::c_int as libc::c_longlong {
        if VA(*o) != 0 {
            z = kerr(b"length\0" as *const u8 as *const libc::c_char);
        }
    } else {
        c = first(y);
        d = 0 as *mut k0;
        if 0 as libc::c_int as libc::c_longlong > (*y).t {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i: I = (*y).n - 1 as libc::c_int as libc::c_longlong;
            loop {
                if !(i_0 < _i) {
                    current_block = 652864300344834934;
                    break;
                }
                d = c;
                if g.is_null() {
                    g = newK(
                        if (*y).t < 0 as libc::c_int as libc::c_longlong {
                            -(*y).t
                        } else {
                            (*y).t
                        },
                        1 as libc::c_int as I,
                    );
                }
                memcpy(
                    ((*g).k).as_mut_ptr() as *mut libc::c_void,
                    (((*y).k).as_mut_ptr() as V)
                        .offset(
                            ((i_0 + 1 as libc::c_int as libc::c_longlong) * bp((*y).t))
                                as isize,
                        ) as *const libc::c_void,
                    bp((*y).t) as libc::c_ulong,
                );
                c = dv_ex(d, p.offset(-(1 as libc::c_int as isize)), g);
                if 2 as libc::c_int as libc::c_longlong == rc(g) {
                    cd(g);
                    g = 0 as K;
                }
                cd(d);
                if c.is_null() {
                    current_block = 5454593693674954355;
                    break;
                }
                i_0 += 1;
                i_0;
            }
        } else {
            current_block = 652864300344834934;
        }
        match current_block {
            5454593693674954355 => {}
            _ => {
                if 0 as libc::c_int as libc::c_longlong == (*y).t {
                    let mut i_1: I = 0 as libc::c_int as I;
                    let mut _i_0: I = (*y).n - 1 as libc::c_int as libc::c_longlong;
                    loop {
                        if !(i_1 < _i_0) {
                            current_block = 980989089337379490;
                            break;
                        }
                        d = c;
                        c = dv_ex(
                            d,
                            p.offset(-(1 as libc::c_int as isize)),
                            *((*y).k)
                                .as_mut_ptr()
                                .offset(
                                    (i_1 + 1 as libc::c_int as libc::c_longlong) as isize,
                                ),
                        );
                        cd(d);
                        if c.is_null() {
                            current_block = 5454593693674954355;
                            break;
                        }
                        i_1 += 1;
                        i_1;
                    }
                } else {
                    current_block = 980989089337379490;
                }
                match current_block {
                    5454593693674954355 => {}
                    _ => {
                        z = c;
                    }
                }
            }
        }
    }
    if !g.is_null() {
        cd(g);
    }
    if !u.is_null() {
        cd(u);
    }
    if !v.is_null() {
        cd(v);
    }
    return z;
}
unsafe extern "C" fn scanDyad(mut a: K, mut p: *mut V, mut b: K) -> K {
    let mut o: *mut V = p.offset(-(1 as libc::c_int as isize));
    let mut f: Option::<unsafe extern "C" fn(K, K) -> K> = None;
    let mut k: K = 0 as K;
    if VA(*o) != 0
        && {
            f = ::std::mem::transmute::<
                V,
                Option::<unsafe extern "C" fn(K, K) -> K>,
            >((*DT.as_mut_ptr().offset(*o as L as isize)).alt_funcs.verb_scan);
            f.is_some()
        }
    {
        k = f.unwrap()(a, b);
    }
    if !k.is_null() {
        return k;
    }
    if a.is_null()
        && !(*o < DT_SIZE as V
            || 7 as libc::c_int as libc::c_longlong == (**(*o as *mut K)).t)
        && 3 as libc::c_int as libc::c_longlong == (**(*o as *mut K)).t
    {
        return csplit(*(*o as *mut K), b);
    }
    let mut u: K = 0 as K;
    let mut y: K = if !a.is_null() {
        u = enlist(a);
        join(u, b)
    } else {
        ci(b)
    };
    cd(u);
    if (*y).t > 0 as libc::c_int as libc::c_longlong
        || (*y).n == 0 as libc::c_int as libc::c_longlong
    {
        return y;
    }
    let mut z: K = newK(0 as libc::c_int as I, (*y).n);
    let mut c: K = 0 as *mut k0;
    let mut d: K = 0 as *mut k0;
    let mut g: K = 0 as *mut k0;
    let ref mut fresh1 = *((*z).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh1 = first(y);
    if 0 as libc::c_int as libc::c_longlong > (*y).t {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = (*y).n - 1 as libc::c_int as libc::c_longlong;
        while i < _i {
            d = *((*z).k).as_mut_ptr().offset(i as isize);
            g = newK(
                if (*y).t < 0 as libc::c_int as libc::c_longlong {
                    -(*y).t
                } else {
                    (*y).t
                },
                1 as libc::c_int as I,
            );
            memcpy(
                ((*g).k).as_mut_ptr() as *mut libc::c_void,
                (((*y).k).as_mut_ptr() as V)
                    .offset(
                        ((i + 1 as libc::c_int as libc::c_longlong) * bp((*y).t))
                            as isize,
                    ) as *const libc::c_void,
                bp((*y).t) as libc::c_ulong,
            );
            c = dv_ex(d, p.offset(-(1 as libc::c_int as isize)), g);
            cd(g);
            if c.is_null() {
                return 0 as K;
            }
            let ref mut fresh2 = *((*z).k)
                .as_mut_ptr()
                .offset((i + 1 as libc::c_int as libc::c_longlong) as isize);
            *fresh2 = c;
            i += 1;
            i;
        }
    }
    if 0 as libc::c_int as libc::c_longlong == (*y).t {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = (*y).n - 1 as libc::c_int as libc::c_longlong;
        while i_0 < _i_0 {
            d = *((*z).k).as_mut_ptr().offset(i_0 as isize);
            c = dv_ex(
                d,
                p.offset(-(1 as libc::c_int as isize)),
                *((*y).k)
                    .as_mut_ptr()
                    .offset((i_0 + 1 as libc::c_int as libc::c_longlong) as isize),
            );
            if c.is_null() {
                return 0 as K;
            }
            let ref mut fresh3 = *((*z).k)
                .as_mut_ptr()
                .offset((i_0 + 1 as libc::c_int as libc::c_longlong) as isize);
            *fresh3 = c;
            i_0 += 1;
            i_0;
        }
    }
    cd(y);
    if !a.is_null() && atomI(b) != 0 {
        y = z;
        u = Ki(1 as libc::c_int as I);
        if OOM_CD(0 as libc::c_int as I, z, u, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        z = drop_0(u, z);
        if OOM_CD(0 as libc::c_int as I, y, u, z, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        cd(y);
        cd(u);
    }
    return collapse(z);
}
unsafe extern "C" fn overMonad(mut a: K, mut p: *mut V, mut b: K) -> K {
    let mut u: K = b;
    let mut c: K = 0 as K;
    let mut flag: I = 0 as libc::c_int as I;
    let mut useN: I = 0 as libc::c_int as I;
    let mut n: I = 0 as libc::c_int as I;
    let mut useB: I = 0 as libc::c_int as I;
    fsf = 0 as libc::c_int as I;
    if !a.is_null() {
        if 1 as libc::c_int as libc::c_longlong == (*a).t {
            useN = 1 as libc::c_int as I;
            n = *(((*a).k).as_mut_ptr() as *mut I);
        } else if 7 as libc::c_int as libc::c_longlong == (*a).t
            || 6 as libc::c_int as libc::c_longlong == (*a).t
        {
            useB = 1 as libc::c_int as I;
        }
    }
    if n < 0 as libc::c_int as libc::c_longlong {
        return kerr(b"int\0" as *const u8 as *const libc::c_char);
    }
    if useN != 0 {
        let mut f: I = 0 as libc::c_int as I;
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = n;
        while i < _i {
            c = dv_ex(0 as K, p.offset(-(1 as libc::c_int as isize)), u);
            if b != u {
                cd(u);
            }
            if f != 0 && b == c {
                cd(c);
            }
            f = 1 as libc::c_int as I;
            u = c;
            if u.is_null() {
                return 0 as K;
            }
            i += 1;
            i;
        }
        c = if !c.is_null() { c } else { ci(b) };
    } else if useB != 0 {
        let mut t: I = 0;
        loop {
            let mut aa: *mut K = &mut a;
            let mut g: K = dv_ex(0 as K, &mut aa as *mut *mut K as V as *mut V, u);
            if g.is_null() {
                return 0 as K;
            }
            t = ((*g).t == 1 as libc::c_int as libc::c_longlong
                && *(((*g).k).as_mut_ptr() as *mut I) != 0) as libc::c_int as I;
            cd(g);
            if t == 0 {
                break;
            }
            c = dv_ex(0 as K, p.offset(-(1 as libc::c_int as isize)), u);
            if b != u {
                cd(u);
            }
            u = c;
            if u.is_null() {
                return 0 as K;
            }
        }
        c = if !c.is_null() { c } else { ci(b) };
    } else {
        let mut o: *mut V = p.offset(-(1 as libc::c_int as isize));
        if *o == offsetOver as V {
            loop {
                if matchI(b, c) != 0 || u != b && matchI(u, c) != 0 {
                    flag = 1 as libc::c_int as I;
                }
                if u != b {
                    cd(u);
                }
                if flag != 0 {
                    break;
                }
                u = if !c.is_null() { c } else { u };
                c = dv_ex(0 as K, p.offset(-(1 as libc::c_int as isize)), u);
                if c.is_null() {
                    return 0 as K;
                }
                if 1 as libc::c_int as libc::c_longlong
                    == (if (*b).t < 0 as libc::c_int as libc::c_longlong {
                        -(*b).t
                    } else {
                        (*b).t
                    })
                    && 3 as libc::c_int as libc::c_longlong
                        == (if (*c).t < 0 as libc::c_int as libc::c_longlong {
                            -(*c).t
                        } else {
                            (*c).t
                        })
                {
                    flag = 1 as libc::c_int as I;
                }
            }
        } else if *o < DT_SIZE as V
            || 7 as libc::c_int as libc::c_longlong == (**(*o as *mut K)).t
                && 3 as libc::c_int as libc::c_longlong == (**(*o as *mut K)).n
        {
            loop {
                if matchI(b, c) != 0 || u != b && matchI(u, c) != 0 {
                    flag = 1 as libc::c_int as I;
                }
                if flag != 0 {
                    break;
                }
                if u != b {
                    cd(u);
                }
                u = if !c.is_null() { c } else { u };
                c = dv_ex(0 as K, o, u);
                if c.is_null() {
                    return 0 as K;
                }
                if 1 as libc::c_int as libc::c_longlong
                    == (if (*b).t < 0 as libc::c_int as libc::c_longlong {
                        -(*b).t
                    } else {
                        (*b).t
                    })
                    && 3 as libc::c_int as libc::c_longlong
                        == (if (*c).t < 0 as libc::c_int as libc::c_longlong {
                            -(*c).t
                        } else {
                            (*c).t
                        })
                {
                    flag = 1 as libc::c_int as I;
                }
            }
            cd(c);
            return u;
        } else {
            a = *(*o as *mut K);
            if 3 as libc::c_int as libc::c_longlong == (*a).t {
                return cjoin(a, b);
            }
            loop {
                if matchI(b, c) != 0 || u != b && matchI(u, c) != 0 {
                    flag = 1 as libc::c_int as I;
                }
                if u != b {
                    cd(u);
                }
                if flag != 0 {
                    break;
                }
                u = if !c.is_null() { c } else { u };
                c = dv_ex(0 as K, o, u);
                if c.is_null() {
                    return 0 as K;
                }
                if 1 as libc::c_int as libc::c_longlong
                    == (if (*b).t < 0 as libc::c_int as libc::c_longlong {
                        -(*b).t
                    } else {
                        (*b).t
                    })
                    && 3 as libc::c_int as libc::c_longlong
                        == (if (*c).t < 0 as libc::c_int as libc::c_longlong {
                            -(*c).t
                        } else {
                            (*c).t
                        })
                {
                    flag = 1 as libc::c_int as I;
                }
            }
        }
    }
    return c;
}
unsafe extern "C" fn scanMonad(mut a: K, mut p: *mut V, mut b: K) -> K {
    let mut u: K = enlist(b);
    let mut v: K = 0 as *mut k0;
    let mut w: K = 0 as *mut k0;
    let mut c: K = 0 as K;
    let mut d: K = 0 as *mut k0;
    let mut flag: I = 0 as libc::c_int as I;
    if u.is_null() {
        return 0 as K;
    }
    let mut useN: I = 0 as libc::c_int as I;
    let mut n: I = 0 as libc::c_int as I;
    let mut useB: I = 0 as libc::c_int as I;
    if !a.is_null() {
        if 1 as libc::c_int as libc::c_longlong == (*a).t {
            useN = 1 as libc::c_int as I;
            n = *(((*a).k).as_mut_ptr() as *mut I);
        } else if 7 as libc::c_int as libc::c_longlong == (*a).t {
            useB = 1 as libc::c_int as I;
        }
    }
    if n < 0 as libc::c_int as libc::c_longlong {
        return kerr(b"int\0" as *const u8 as *const libc::c_char);
    }
    if useN != 0 {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = n;
        while i < _i {
            d = last(u);
            if d.is_null() {
                return 0 as K;
            }
            c = dv_ex(0 as K, p.offset(-(1 as libc::c_int as isize)), d);
            cd(d);
            if c.is_null() {
                return 0 as K;
            }
            v = enlist(c);
            if v.is_null() {
                return 0 as K;
            }
            cd(c);
            w = u;
            u = join(w, v);
            cd(w);
            cd(v);
            if u.is_null() {
                return 0 as K;
            }
            i += 1;
            i;
        }
    } else if useB != 0 {
        let mut t: I = 0;
        loop {
            d = last(u);
            if d.is_null() {
                return 0 as K;
            }
            let mut aa: *mut K = &mut a;
            let mut g: K = dv_ex(0 as K, &mut aa as *mut *mut K as V as *mut V, d);
            if g.is_null() {
                return 0 as K;
            }
            t = (1 as libc::c_int as libc::c_longlong == (*g).t
                && *(((*g).k).as_mut_ptr() as *mut I) != 0) as libc::c_int as I;
            cd(g);
            if t == 0 {
                cd(d);
                break;
            } else {
                c = dv_ex(0 as K, p.offset(-(1 as libc::c_int as isize)), d);
                cd(d);
                if c.is_null() {
                    return 0 as K;
                }
                v = enlist(c);
                if v.is_null() {
                    return 0 as K;
                }
                cd(c);
                w = u;
                u = join(w, v);
                cd(w);
                cd(v);
                if u.is_null() {
                    return 0 as K;
                }
            }
        }
    } else {
        loop {
            d = last(u);
            if d.is_null() {
                return 0 as K;
            }
            if matchI(b, c) != 0 || matchI(c, d) != 0 {
                flag = 1 as libc::c_int as I;
            }
            if flag == 0 && !c.is_null() {
                v = u;
                w = enlist(c);
                u = join(v, w);
                cd(v);
                cd(w);
                cd(d);
                d = c;
            }
            if interrupted != 0 {
                ::std::ptr::write_volatile(
                    &mut interrupted as *mut sig_atomic_t,
                    0 as libc::c_int,
                );
                return kerr(b"break\0" as *const u8 as *const libc::c_char);
            }
            if flag != 0 {
                cd(c);
                cd(d);
                break;
            } else {
                c = dv_ex(0 as K, p.offset(-(1 as libc::c_int as isize)), d);
                cd(d);
                if c.is_null() {
                    cd(u);
                    return c;
                }
            }
        }
    }
    return u;
}
unsafe extern "C" fn each2(mut a: K, mut p: *mut V, mut b: K) -> K {
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    let mut prnt0: K = 0 as K;
    let mut grnt0: K = 0 as K;
    let mut d: K = 0 as K;
    if bt > 0 as libc::c_int as libc::c_longlong {
        if !a.is_null() && (*a).n > 0 as libc::c_int as libc::c_longlong {
            let mut z: K = newK(0 as libc::c_int as I, (*a).n);
            if z.is_null() {
                return 0 as K;
            }
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = (*a).n;
            while i < _i {
                d = dv_ex(
                    *((*a).k).as_mut_ptr().offset(i as isize),
                    p.offset(-(1 as libc::c_int as isize)),
                    b,
                );
                if OOM_CD(0 as libc::c_int as I, d, z, -(1 as libc::c_int) as V) == 0 {
                    return 0 as K;
                }
                let ref mut fresh4 = *((*z).k).as_mut_ptr().offset(i as isize);
                *fresh4 = d;
                i += 1;
                i;
            }
            z = demote(z);
            if (*z).t == 1 as libc::c_int as libc::c_longlong {
                (*z).t = -(1 as libc::c_int) as I;
            }
            return z;
        } else {
            d = dv_ex(a, p.offset(-(1 as libc::c_int as isize)), b);
            return d;
        }
    } else {
        let mut z_0: K = newK(0 as libc::c_int as I, bn);
        if z_0.is_null() {
            return 0 as K;
        }
        let mut g: K = 0 as *mut k0;
        let mut f: I = (*p == offsetEach as V
            && (*p.offset(-(1 as libc::c_int as isize)) == offsetEach as V
                || *p.offset(-(1 as libc::c_int as isize)) == offsetOver as V
                || *p.offset(-(1 as libc::c_int as isize)) == offsetScan as V)
            && *p.offset(-(2 as libc::c_int as isize)) < DT_SIZE as V) as libc::c_int
            as I;
        if 0 as libc::c_int as libc::c_longlong > bt {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = bn;
            while i_0 < _i_0 {
                g = newK(
                    if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt },
                    1 as libc::c_int as I,
                );
                if OOM_CD(0 as libc::c_int as I, g, z_0, -(1 as libc::c_int) as V) == 0 {
                    return 0 as K;
                }
                memcpy(
                    ((*g).k).as_mut_ptr() as *mut libc::c_void,
                    (((*b).k).as_mut_ptr() as V).offset((i_0 * bp(bt)) as isize)
                        as *const libc::c_void,
                    bp(bt) as libc::c_ulong,
                );
                if f != 0 {
                    d = dv_ex(a, p.offset(-(1 as libc::c_int as isize)), g);
                } else {
                    d = dv_ex(0 as K, p.offset(-(1 as libc::c_int as isize)), g);
                }
                cd(g);
                if OOM_CD(0 as libc::c_int as I, d, z_0, -(1 as libc::c_int) as V) == 0 {
                    return 0 as K;
                }
                let ref mut fresh5 = *((*z_0).k).as_mut_ptr().offset(i_0 as isize);
                *fresh5 = d;
                i_0 += 1;
                i_0;
            }
        }
        if 0 as libc::c_int as libc::c_longlong == bt {
            if !prnt.is_null() {
                prnt0 = ci(prnt);
            }
            if !grnt.is_null() {
                grnt0 = ci(grnt);
            }
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = bn;
            while i_1 < _i_1 {
                if f != 0 {
                    if !a.is_null() && (*a).n > 1 as libc::c_int as libc::c_longlong {
                        d = dv_ex(
                            *((*a).k).as_mut_ptr().offset(i_1 as isize),
                            p.offset(-(1 as libc::c_int as isize)),
                            *((*b).k).as_mut_ptr().offset(i_1 as isize),
                        );
                    } else {
                        d = dv_ex(
                            a,
                            p.offset(-(1 as libc::c_int as isize)),
                            *((*b).k).as_mut_ptr().offset(i_1 as isize),
                        );
                    }
                } else {
                    if !prnt0.is_null() {
                        cd(prnt);
                        prnt = ci(prnt0);
                    }
                    if !grnt0.is_null() {
                        cd(grnt);
                        grnt = ci(grnt0);
                    }
                    d = dv_ex(
                        0 as K,
                        p.offset(-(1 as libc::c_int as isize)),
                        *((*b).k).as_mut_ptr().offset(i_1 as isize),
                    );
                }
                if d.is_null() || z_0.is_null() {
                    if !prnt0.is_null() {
                        cd(prnt0);
                        prnt0 = 0 as K;
                    }
                    if !grnt0.is_null() {
                        cd(grnt0);
                        grnt0 = 0 as K;
                    }
                }
                if !grnt.is_null() && prnt.is_null() {
                    prnt = ci(grnt);
                }
                if OOM_CD(0 as libc::c_int as I, d, z_0, -(1 as libc::c_int) as V) == 0 {
                    return 0 as K;
                }
                let ref mut fresh6 = *((*z_0).k).as_mut_ptr().offset(i_1 as isize);
                *fresh6 = d;
                i_1 += 1;
                i_1;
            }
        }
        z_0 = demote(z_0);
        if (*z_0).t == 1 as libc::c_int as libc::c_longlong {
            (*z_0).t = -(1 as libc::c_int) as I;
        }
        if !prnt0.is_null() {
            cd(prnt0);
            prnt0 = 0 as K;
        }
        if !grnt0.is_null() {
            cd(grnt0);
            grnt0 = 0 as K;
        }
        return z_0;
    };
}
unsafe extern "C" fn eachright2(mut a: K, mut p: *mut V, mut b: K) -> K {
    if ft3 != 0 && a.is_null() {
        return kerr(b"valence\0" as *const u8 as *const libc::c_char);
    }
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    if bt > 0 as libc::c_int as libc::c_longlong {
        return dv_ex(a, p.offset(-(1 as libc::c_int as isize)), b);
    }
    let mut z: K = newK(0 as libc::c_int as I, bn);
    let mut d: K = 0 as *mut k0;
    let mut g: K = 0 as *mut k0;
    if 0 as libc::c_int as libc::c_longlong > bt {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = bn;
        while i < _i {
            g = newK(
                if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt },
                1 as libc::c_int as I,
            );
            memcpy(
                ((*g).k).as_mut_ptr() as *mut libc::c_void,
                (((*b).k).as_mut_ptr() as V).offset((i * bp(bt)) as isize)
                    as *const libc::c_void,
                bp(bt) as libc::c_ulong,
            );
            d = dv_ex(a, p.offset(-(1 as libc::c_int as isize)), g);
            cd(g);
            if d.is_null() {
                return 0 as K;
            }
            let ref mut fresh7 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh7 = d;
            i += 1;
            i;
        }
    }
    if 0 as libc::c_int as libc::c_longlong == bt {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = bn;
        while i_0 < _i_0 {
            d = dv_ex(
                a,
                p.offset(-(1 as libc::c_int as isize)),
                *((*b).k).as_mut_ptr().offset(i_0 as isize),
            );
            if d.is_null() {
                return 0 as K;
            }
            let ref mut fresh8 = *((*z).k).as_mut_ptr().offset(i_0 as isize);
            *fresh8 = d;
            i_0 += 1;
            i_0;
        }
    }
    return demote(z);
}
unsafe extern "C" fn eachleft2(mut a: K, mut p: *mut V, mut b: K) -> K {
    if a.is_null() {
        return kerr(b"valence\0" as *const u8 as *const libc::c_char);
    }
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    if at > 0 as libc::c_int as libc::c_longlong {
        return dv_ex(a, p.offset(-(1 as libc::c_int as isize)), b);
    }
    let mut z: K = newK(0 as libc::c_int as I, an);
    let mut d: K = 0 as *mut k0;
    let mut g: K = 0 as *mut k0;
    if 0 as libc::c_int as libc::c_longlong > at {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = an;
        while i < _i {
            g = newK(
                if at < 0 as libc::c_int as libc::c_longlong { -at } else { at },
                1 as libc::c_int as I,
            );
            memcpy(
                ((*g).k).as_mut_ptr() as *mut libc::c_void,
                (((*a).k).as_mut_ptr() as V).offset((i * bp(at)) as isize)
                    as *const libc::c_void,
                bp(at) as libc::c_ulong,
            );
            d = dv_ex(g, p.offset(-(1 as libc::c_int as isize)), b);
            cd(g);
            if d.is_null() {
                return 0 as K;
            }
            let ref mut fresh9 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh9 = d;
            i += 1;
            i;
        }
    }
    if 0 as libc::c_int as libc::c_longlong == at {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = an;
        while i_0 < _i_0 {
            d = dv_ex(
                *((*a).k).as_mut_ptr().offset(i_0 as isize),
                p.offset(-(1 as libc::c_int as isize)),
                b,
            );
            if d.is_null() {
                return 0 as K;
            }
            let ref mut fresh10 = *((*z).k).as_mut_ptr().offset(i_0 as isize);
            *fresh10 = d;
            i_0 += 1;
            i_0;
        }
    }
    return demote(z);
}
unsafe extern "C" fn eachpair2(mut a: K, mut p: *mut V, mut b: K) -> K {
    let mut o: *mut V = p.offset(-(1 as libc::c_int as isize));
    let mut f: Option::<unsafe extern "C" fn(K, K) -> K> = None;
    let mut k: K = 0 as K;
    if VA(*o) != 0
        && {
            f = ::std::mem::transmute::<
                V,
                Option::<unsafe extern "C" fn(K, K) -> K>,
            >((*DT.as_mut_ptr().offset(*o as L as isize)).alt_funcs.verb_eachpair);
            f.is_some()
        }
    {
        k = f.unwrap()(a, b);
    }
    if !k.is_null() {
        return k;
    }
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    if !a.is_null() && bt > 0 as libc::c_int as libc::c_longlong
        && bn > 1 as libc::c_int as libc::c_longlong
    {
        let mut u: K = 0 as *mut k0;
        let mut v: K = 0 as *mut k0;
        u = enlist(a);
        if OOM_CD(0 as libc::c_int as I, u, b, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        v = join(u, b);
        cd(u);
        return v;
    }
    if bt <= 0 as libc::c_int as libc::c_longlong {
        if bn == 0 as libc::c_int as libc::c_longlong && a.is_null() {
            return kerr(b"length\0" as *const u8 as *const libc::c_char)
        } else if bn == 0 as libc::c_int as libc::c_longlong && !a.is_null() {
            return newK(0 as libc::c_int as I, 0 as libc::c_int as I)
        } else if bn < 2 as libc::c_int as libc::c_longlong {
            return newK(0 as libc::c_int as I, 0 as libc::c_int as I)
        }
    }
    let mut z: K = newK(
        0 as libc::c_int as I,
        bn - 1 as libc::c_int as libc::c_longlong,
    );
    let mut d: K = 0 as K;
    if z.is_null() {
        return 0 as K;
    }
    let mut g: K = 0 as *mut k0;
    let mut h: K = 0 as *mut k0;
    if bt < 0 as libc::c_int as libc::c_longlong {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = bn - 1 as libc::c_int as libc::c_longlong;
        while i < _i {
            h = newK(
                if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt },
                1 as libc::c_int as I,
            );
            g = newK(
                if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt },
                1 as libc::c_int as I,
            );
            memcpy(
                ((*h).k).as_mut_ptr() as *mut libc::c_void,
                (((*b).k).as_mut_ptr() as V).offset((i * bp(bt)) as isize)
                    as *const libc::c_void,
                bp(bt) as libc::c_ulong,
            );
            memcpy(
                ((*g).k).as_mut_ptr() as *mut libc::c_void,
                (((*b).k).as_mut_ptr() as V)
                    .offset(
                        ((i + 1 as libc::c_int as libc::c_longlong) * bp(bt)) as isize,
                    ) as *const libc::c_void,
                bp(bt) as libc::c_ulong,
            );
            d = dv_ex(g, p.offset(-(1 as libc::c_int as isize)), h);
            cd(g);
            cd(h);
            if d.is_null() {
                return 0 as K;
            }
            let ref mut fresh11 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh11 = d;
            i += 1;
            i;
        }
    }
    if bt == 0 as libc::c_int as libc::c_longlong {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = bn - 1 as libc::c_int as libc::c_longlong;
        while i_0 < _i_0 {
            d = dv_ex(
                *((*b).k)
                    .as_mut_ptr()
                    .offset((i_0 + 1 as libc::c_int as libc::c_longlong) as isize),
                p.offset(-(1 as libc::c_int as isize)),
                *((*b).k).as_mut_ptr().offset(i_0 as isize),
            );
            if d.is_null() {
                return 0 as K;
            }
            let ref mut fresh12 = *((*z).k).as_mut_ptr().offset(i_0 as isize);
            *fresh12 = d;
            i_0 += 1;
            i_0;
        }
    }
    if bt > 0 as libc::c_int as libc::c_longlong && a.is_null() {
        h = newK(
            if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt },
            1 as libc::c_int as I,
        );
        g = newK(
            if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt },
            1 as libc::c_int as I,
        );
        memcpy(
            ((*h).k).as_mut_ptr() as *mut libc::c_void,
            (((*b).k).as_mut_ptr() as V)
                .offset((0 as libc::c_int as libc::c_longlong * bp(bt)) as isize)
                as *const libc::c_void,
            bp(bt) as libc::c_ulong,
        );
        memcpy(
            ((*g).k).as_mut_ptr() as *mut libc::c_void,
            (((*b).k).as_mut_ptr() as V)
                .offset((0 as libc::c_int as libc::c_longlong * bp(bt)) as isize)
                as *const libc::c_void,
            bp(bt) as libc::c_ulong,
        );
        d = dv_ex(g, p.offset(-(1 as libc::c_int as isize)), h);
        cd(g);
        cd(h);
        cd(z);
        if d.is_null() {
            return 0 as K;
        }
        let ref mut fresh13 = *((*z).k).as_mut_ptr().offset(0 as libc::c_int as isize);
        *fresh13 = d;
        return d;
    }
    z = demote(z);
    if !a.is_null() {
        if bn == 1 as libc::c_int as libc::c_longlong {
            cd(z);
            return ci(a);
        }
        let mut u_0: K = 0 as *mut k0;
        let mut v_0: K = 0 as *mut k0;
        u_0 = enlist(a);
        if OOM_CD(0 as libc::c_int as I, u_0, z, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        v_0 = join(u_0, z);
        cd(u_0);
        cd(z);
        return v_0;
    } else if (*z).t == 1 as libc::c_int as libc::c_longlong {
        let mut u_1: K = 0 as *mut k0;
        u_1 = enlist(z);
        cd(z);
        return u_1;
    }
    return z;
}
pub unsafe extern "C" fn dv_ex(mut a: K, mut p: *mut V, mut b: K) -> K {
    if p.is_null() || (*p).is_null() {
        return 0 as K;
    }
    if b.is_null() {
        return 0 as K;
    }
    let mut o: *mut V = p.offset(-(1 as libc::c_int as isize));
    let mut k: I = 0 as libc::c_int as I;
    let mut w: K = 0 as *mut k0;
    if *p == offsetScan as V && *o > DT_SIZE as V {
        w = *(*o as *mut K);
        if 7 as libc::c_int as libc::c_longlong == (*w).t
            && 3 as libc::c_int as libc::c_longlong
                == (*(*(((*w).k).as_mut_ptr() as *mut V)
                    .offset(CODE as libc::c_int as isize) as K))
                    .n
            && *(((*(*(((*w).k).as_mut_ptr() as *mut V)
                .offset(CODE as libc::c_int as isize) as K))
                .k)
                .as_mut_ptr() as *mut V)
                .offset(0 as libc::c_int as isize) == 0x16 as libc::c_int as V
            && *(((*(*(((*w).k).as_mut_ptr() as *mut V)
                .offset(CODE as libc::c_int as isize) as K))
                .k)
                .as_mut_ptr() as *mut V)
                .offset(1 as libc::c_int as isize) == offsetScan as V
        {
            k = 1 as libc::c_int as I;
        }
    }
    if k == 0 as libc::c_int as libc::c_longlong {
        k = if adverbClass(*p) != 0 {
            if adverbClass(*o) != 0 {
                1 as libc::c_int as libc::c_longlong
            } else {
                valence(*o)
            }
        } else {
            valence(*p)
        };
    }
    let mut adverb: V = *p;
    if k > 2 as libc::c_int as libc::c_longlong {
        k = 2 as libc::c_int as I;
    }
    if *p == offsetEach as V && k == 1 as libc::c_int as libc::c_longlong && !a.is_null()
        && !b.is_null()
        && ((*a).t > 0 as libc::c_int as libc::c_longlong
            && (*a).t < 5 as libc::c_int as libc::c_longlong
            && (*b).t > 0 as libc::c_int as libc::c_longlong
            && (*b).t < 5 as libc::c_int as libc::c_longlong
            || (*a).t == -(1 as libc::c_int) as libc::c_longlong
                && (*b).t == -(1 as libc::c_int) as libc::c_longlong)
    {
        k = 2 as libc::c_int as I;
    }
    if 2 as libc::c_int as libc::c_longlong == k
        || k == 0 as libc::c_int as libc::c_longlong
            && adverb as UI == offsetScan as libc::c_ulonglong
            && (0 as libc::c_int as libc::c_longlong
                == (**(*p.offset(-(1 as libc::c_int) as isize) as *mut K)).t
                || 3 as libc::c_int as libc::c_longlong
                    == (**(*p.offset(-(1 as libc::c_int) as isize) as *mut K)).t)
    {
        if adverb as UI == offsetOver as libc::c_ulonglong {
            return overDyad(a, p, b);
        }
        if adverb as UI == offsetScan as libc::c_ulonglong {
            return scanDyad(a, p, b);
        }
        if adverb as UI == offsetEach as libc::c_ulonglong {
            if a.is_null() {
                adverb = offsetEachright as V;
            } else if (*a).t <= 0 as libc::c_int as libc::c_longlong
                && (*b).t <= 0 as libc::c_int as libc::c_longlong && (*a).n != (*b).n
            {
                return kerr(b"length\0" as *const u8 as *const libc::c_char)
            } else if (*a).t > 0 as libc::c_int as libc::c_longlong
                && (*b).t > 0 as libc::c_int as libc::c_longlong
            {
                return dv_ex(a, p.offset(-(1 as libc::c_int as isize)), b)
            } else if (*a).t > 0 as libc::c_int as libc::c_longlong {
                adverb = offsetEachright as V;
            } else if (*b).t > 0 as libc::c_int as libc::c_longlong {
                adverb = offsetEachleft as V;
            } else {
                a = promote(a);
                b = promote(b);
                if OOM_CD(0 as libc::c_int as I, a, b, -(1 as libc::c_int) as V) == 0 {
                    return 0 as K;
                }
                let mut z: K = newK(0 as libc::c_int as I, (*a).n);
                let mut k_0: K = 0 as *mut k0;
                if OOM_CD(0 as libc::c_int as I, z, a, b, -(1 as libc::c_int) as V) == 0
                {
                    return 0 as K;
                }
                let mut i: I = 0 as libc::c_int as I;
                let mut _i: I = (*a).n;
                while i < _i {
                    k_0 = dv_ex(
                        *((*a).k).as_mut_ptr().offset(i as isize),
                        p.offset(-(1 as libc::c_int as isize)),
                        *((*b).k).as_mut_ptr().offset(i as isize),
                    );
                    if OOM_CD(
                        0 as libc::c_int as I,
                        k_0,
                        z,
                        a,
                        b,
                        -(1 as libc::c_int) as V,
                    ) == 0
                    {
                        return 0 as K;
                    }
                    let ref mut fresh14 = *((*z).k).as_mut_ptr().offset(i as isize);
                    *fresh14 = k_0;
                    i += 1;
                    i;
                }
                cd(a);
                cd(b);
                return demote(z);
            }
        }
    } else if 2 as libc::c_int as libc::c_longlong > k {
        if adverb as UI == offsetOver as libc::c_ulonglong {
            if fom == 0 {
                return overMonad(a, p, b)
            } else {
                return overMonad(
                    *((*b).k).as_mut_ptr().offset(0 as libc::c_int as isize),
                    p,
                    *((*b).k).as_mut_ptr().offset(1 as libc::c_int as isize),
                )
            }
        }
        if adverb as UI == offsetScan as libc::c_ulonglong {
            return scanMonad(a, p, b);
        }
        if adverb as UI == offsetEach as libc::c_ulonglong {
            return each2(a, p, b);
        }
    }
    if adverb as UI == offsetEachright as libc::c_ulonglong {
        return eachright2(a, p, b);
    }
    if adverb as UI == offsetEachleft as libc::c_ulonglong {
        return eachleft2(a, p, b);
    }
    if adverb as UI == offsetEachpair as libc::c_ulonglong {
        return eachpair2(a, p, b);
    }
    let mut gn: I = 0 as libc::c_int as I;
    if valence(*p) >= 2 as libc::c_int as libc::c_longlong && !a.is_null()
        && !b.is_null()
    {
        gn = 2 as libc::c_int as I;
    } else if !a.is_null() {
        let mut q: [V; 4] = [0 as *mut libc::c_void; 4];
        q[0 as libc::c_int as usize] = &mut a as *mut K as V;
        q[1 as libc::c_int as usize] = 1 as libc::c_int as V;
        q[2 as libc::c_int as usize] = &mut b as *mut K as V;
        q[3 as libc::c_int as usize] = 0 as V;
        let mut u: K = ex0(
            &mut *q.as_mut_ptr().offset(0 as libc::c_int as isize),
            0 as K,
            2 as libc::c_int as I,
        );
        q[0 as libc::c_int as usize] = *p;
        q[1 as libc::c_int as usize] = 0 as V;
        let mut v: K = ex0(
            &mut *q.as_mut_ptr().offset(0 as libc::c_int as isize),
            u,
            1 as libc::c_int as I,
        );
        cd(u);
        return v;
    } else if !b.is_null() {
        gn = 1 as libc::c_int as I;
    }
    let mut g: K = newK(0 as libc::c_int as I, gn);
    if g.is_null() {
        return 0 as K;
    }
    if gn > 1 as libc::c_int as libc::c_longlong {
        let ref mut fresh15 = *((*g).k).as_mut_ptr().offset(1 as libc::c_int as isize);
        *fresh15 = b;
    }
    if gn > 0 as libc::c_int as libc::c_longlong {
        let ref mut fresh16 = *((*g).k).as_mut_ptr().offset(0 as libc::c_int as isize);
        *fresh16 = if !a.is_null() { a } else { b };
    }
    let mut tmp: K = 0 as *mut k0;
    let mut flag: I = 0 as libc::c_int as I;
    if *p as UI > DT_SIZE as libc::c_ulonglong && (*b).n != 0 {
        let mut p1: *mut V = *p as *mut V;
        if *p1 as UI > DT_SIZE as libc::c_ulonglong {
            let mut p2: K = *p1 as K;
            if 7 as libc::c_int as libc::c_longlong != (*p2).t
                && -(1 as libc::c_int) as libc::c_longlong != (*p2).t
                && 5 as libc::c_int as libc::c_longlong != (*p2).t
            {
                flag = 1 as libc::c_int as I;
            }
        }
    }
    if flag != 0 {
        tmp = vf_ex(*p, b);
    } else {
        if stk as libc::c_double > 2e6f64 {
            return kerr(b"stack\0" as *const u8 as *const libc::c_char);
        }
        stk += 1;
        stk;
        tmp = vf_ex(*p, g);
        stk -= 1;
        stk;
        if !grnt.is_null() && prnt.is_null() {
            prnt = ci(grnt);
        }
    }
    memset(
        ((*g).k).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ((*g).n as libc::c_ulonglong)
            .wrapping_mul(
                ::std::mem::size_of::<K>() as libc::c_ulong as libc::c_ulonglong,
            ) as libc::c_ulong,
    );
    cd(g);
    return tmp;
}
pub unsafe extern "C" fn vf_ex(mut q: V, mut g: K) -> K {
    let mut m_0: K = 0 as *mut k0;
    let mut q_1: *mut K = 0 as *mut K;
    let mut a_0: [K; 7] = [0 as *mut k0; 7];
    let mut argc: I = 0;
    let mut a: K = 0 as *mut k0;
    let mut b: K = 0 as *mut k0;
    let mut c: K = 0 as *mut k0;
    let mut d: K = 0 as *mut k0;
    let mut f: K = 0 as *mut k0;
    let mut ft: I = 0;
    let mut t: I = 0;
    let mut o: K = 0 as *mut k0;
    let mut p: K = 0 as *mut k0;
    let mut s: K = 0 as *mut k0;
    let mut r: K = 0 as *mut k0;
    let mut special: I = 0;
    let mut v_0: V = 0 as *mut libc::c_void;
    let mut tree: K = 0 as *mut k0;
    let mut current_block: u64;
    let mut tc: K = 0 as K;
    if interrupted != 0 {
        ::std::ptr::write_volatile(
            &mut interrupted as *mut sig_atomic_t,
            0 as libc::c_int,
        );
        return kerr(b"break\0" as *const u8 as *const libc::c_char);
    }
    if g.is_null() {
        return 0 as K;
    }
    let mut z: K = 0 as K;
    g = promote(g);
    if g.is_null() {
        return 0 as K;
    }
    let mut gn: I = (*g).n;
    let mut k: I = sva(q);
    let mut n: I = -(1 as libc::c_int) as I;
    let mut j: I = 0 as libc::c_int as I;
    if k == 0 && (*(q as *mut V)).is_null() {
        cd(g);
        return 0 as K;
    }
    n = valence(q);
    let mut ee: I = 0 as libc::c_int as I;
    if q > DT_SIZE as V {
        let mut h: K = *(q as *mut K);
        if (*h).t == 7 as libc::c_int as libc::c_longlong {
            if (*h).n == 1 as libc::c_int as libc::c_longlong {
                if !(*((*h).k).as_mut_ptr().offset(CODE as libc::c_int as isize))
                    .is_null()
                    && *((*(*((*h).k).as_mut_ptr().offset(CODE as libc::c_int as isize)
                        as K))
                        .k)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as UI
                        > DT_SIZE as libc::c_ulonglong
                {
                    if (**((*h).k).as_mut_ptr().offset(CODE as libc::c_int as isize)).n
                        == 3 as libc::c_int as libc::c_longlong
                        && (**(*(((*(*((*h).k)
                            .as_mut_ptr()
                            .offset(CODE as libc::c_int as isize) as K))
                            .k)
                            .as_mut_ptr() as *mut S)
                            .offset(0 as libc::c_int as isize) as *mut K))
                            .t == 0 as libc::c_int as libc::c_longlong
                    {
                        z = dot(
                            *(*(((*(*((*h).k)
                                .as_mut_ptr()
                                .offset(CODE as libc::c_int as isize) as K))
                                .k)
                                .as_mut_ptr() as *mut S)
                                .offset(0 as libc::c_int as isize) as *mut K),
                            g,
                        );
                        current_block = 15299809402464954699;
                    } else if *((*(*((*h).k)
                        .as_mut_ptr()
                        .offset(CODE as libc::c_int as isize) as K))
                        .k)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize) as UI
                        == 0x3a as libc::c_int as libc::c_ulonglong
                        && (*g).t == 0 as libc::c_int as libc::c_longlong
                    {
                        z = dot(
                            *(*(((*(*((*h).k)
                                .as_mut_ptr()
                                .offset(CODE as libc::c_int as isize) as K))
                                .k)
                                .as_mut_ptr() as *mut S)
                                .offset(0 as libc::c_int as isize) as *mut K),
                            *((*g).k).as_mut_ptr().offset(0 as libc::c_int as isize),
                        );
                        current_block = 15299809402464954699;
                    } else {
                        current_block = 5948590327928692120;
                    }
                } else {
                    current_block = 5948590327928692120;
                }
                match current_block {
                    15299809402464954699 => {}
                    _ => {
                        if *(((*(*((*h).k)
                            .as_mut_ptr()
                            .offset(CODE as libc::c_int as isize) as K))
                            .k)
                            .as_mut_ptr() as *mut S)
                            .offset(0 as libc::c_int as isize) as V > DT_SIZE as V
                            && (**(*(((*(*((*h).k)
                                .as_mut_ptr()
                                .offset(CODE as libc::c_int as isize) as K))
                                .k)
                                .as_mut_ptr() as *mut S)
                                .offset(0 as libc::c_int as isize) as *mut K))
                                .t == 7 as libc::c_int as libc::c_longlong
                        {
                            n = 2 as libc::c_int as I;
                            ee = 1 as libc::c_int as I;
                        }
                        current_block = 11042950489265723346;
                    }
                }
            } else {
                current_block = 11042950489265723346;
            }
            match current_block {
                15299809402464954699 => {}
                _ => {
                    if -(4 as libc::c_int) as libc::c_longlong
                        == (**((*h).k).as_mut_ptr().offset(2 as libc::c_int as isize)).t
                        && 2 as libc::c_int as libc::c_longlong
                            == (**((*h).k)
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as isize))
                                .n
                        && *(((*(*((*h).k).as_mut_ptr().offset(2 as libc::c_int as isize)
                            as K))
                            .k)
                            .as_mut_ptr() as *mut V)
                            .offset(0 as libc::c_int as isize)
                            == 0x3a as libc::c_int as V
                        && 0 as libc::c_int as libc::c_longlong == (*g).t
                    {
                        if (*(((*g).k).as_mut_ptr() as *mut V)
                            .offset(0 as libc::c_int as isize))
                            .is_null()
                        {
                            z = ci(*(q as *mut K));
                            let ref mut fresh17 = *(((*z).k).as_mut_ptr() as *mut V)
                                .offset(5 as libc::c_int as isize);
                            *fresh17 = ci(g) as V;
                            current_block = 15299809402464954699;
                        } else if 6 as libc::c_int as libc::c_longlong
                            == (**((*g).k)
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize))
                                .t
                        {
                            z = ci(*(q as *mut K));
                            let ref mut fresh18 = *(((*z).k).as_mut_ptr() as *mut V)
                                .offset(5 as libc::c_int as isize);
                            *fresh18 = ci(
                                *((*z).k).as_mut_ptr().offset(4 as libc::c_int as isize),
                            ) as V;
                            *(*((*z).k).as_mut_ptr().offset(2 as libc::c_int as isize))
                                .offset(
                                    0 as libc::c_int as isize,
                                ) = *(*((*z).k)
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as isize))
                                .offset(1 as libc::c_int as isize);
                            ci(*((*z).k).as_mut_ptr().offset(2 as libc::c_int as isize));
                            current_block = 15299809402464954699;
                        } else {
                            current_block = 15897653523371991391;
                        }
                    } else {
                        current_block = 15897653523371991391;
                    }
                }
            }
        } else {
            current_block = 15897653523371991391;
        }
    } else {
        current_block = 15897653523371991391;
    }
    match current_block {
        15897653523371991391 => {
            if ee != 0
                && (*(((*g).k).as_mut_ptr() as *mut V).offset(0 as libc::c_int as isize))
                    .is_null()
                && !(*(((*g).k).as_mut_ptr() as *mut V)
                    .offset(1 as libc::c_int as isize))
                    .is_null()
            {
                fom = 1 as libc::c_int as I;
            }
            if (k != 0 || (**(q as *mut K)).t == 7 as libc::c_int as libc::c_longlong)
                && (((q as UI) < DT_SIZE as libc::c_ulonglong
                    || !(*(q as *mut V)).is_null()) && gn > n
                    && !(n == 0 && 1 as libc::c_int as libc::c_longlong >= gn))
                || ee != 0
                    && !(*(((*g).k).as_mut_ptr() as *mut V)
                        .offset(0 as libc::c_int as isize))
                        .is_null()
                    && !(*(((*g).k).as_mut_ptr() as *mut V)
                        .offset(1 as libc::c_int as isize))
                        .is_null()
            {
                if (*((*g).k).as_mut_ptr().offset(0 as libc::c_int as isize)).is_null() {
                    kerr(b"valence\0" as *const u8 as *const libc::c_char);
                    current_block = 15299809402464954699;
                } else if 3 as libc::c_int as libc::c_longlong
                    != (**((*g).k).as_mut_ptr().offset(0 as libc::c_int as isize)).t
                    || 1 as libc::c_int as libc::c_longlong == (**(q as *mut K)).n
                    || (*((*g).k).as_mut_ptr().offset(1 as libc::c_int as isize))
                        .is_null()
                {
                    if (*g).t == 0 as libc::c_int as libc::c_longlong
                        && gn == 2 as libc::c_int as libc::c_longlong
                        && (**((**(q as *mut K)).k)
                            .as_mut_ptr()
                            .offset(CODE as libc::c_int as isize))
                            .t == -(4 as libc::c_int) as libc::c_longlong
                        && *(((*(*((**(q as *mut K)).k)
                            .as_mut_ptr()
                            .offset(CODE as libc::c_int as isize) as K))
                            .k)
                            .as_mut_ptr() as *mut S)
                            .offset(0 as libc::c_int as isize) as V > DT_SIZE as V
                        && (**(*(((*(*((**(q as *mut K)).k)
                            .as_mut_ptr()
                            .offset(CODE as libc::c_int as isize) as K))
                            .k)
                            .as_mut_ptr() as *mut S)
                            .offset(0 as libc::c_int as isize) as *mut K))
                            .t == 7 as libc::c_int as libc::c_longlong
                    {
                        let mut w: [V; 2] = [0 as *mut libc::c_void; 2];
                        w[0 as libc::c_int
                            as usize] = *(((*(*((**(q as *mut K)).k)
                            .as_mut_ptr()
                            .offset(CODE as libc::c_int as isize) as K))
                            .k)
                            .as_mut_ptr() as *mut S)
                            .offset(0 as libc::c_int as isize) as V;
                        w[1 as libc::c_int as usize] = offsetOver as V;
                        z = overMonad(
                            *((*g).k).as_mut_ptr().offset(0 as libc::c_int as isize),
                            &mut *w.as_mut_ptr().offset(1 as libc::c_int as isize),
                            *((*g).k).as_mut_ptr().offset(1 as libc::c_int as isize),
                        );
                    } else {
                        kerr(b"valence\0" as *const u8 as *const libc::c_char);
                    }
                    current_block = 15299809402464954699;
                } else {
                    g = enlist(collapse(g));
                    gn = (*g).n;
                    cd(*((*g).k).as_mut_ptr().offset(0 as libc::c_int as isize));
                    current_block = 11626999923138678822;
                }
            } else {
                current_block = 11626999923138678822;
            }
            match current_block {
                15299809402464954699 => {}
                _ => {
                    argc = 0 as libc::c_int as I;
                    let mut i: I = 0 as libc::c_int as I;
                    let mut _i: I = gn;
                    while i < _i {
                        if !(*((*g).k).as_mut_ptr().offset(i as isize)).is_null() {
                            argc += 1;
                            argc;
                        }
                        i += 1;
                        i;
                    }
                    a = 0 as K;
                    b = 0 as K;
                    c = 0 as K;
                    d = 0 as K;
                    if gn > 0 as libc::c_int as libc::c_longlong {
                        a = *((*g).k).as_mut_ptr().offset(0 as libc::c_int as isize);
                    }
                    if gn > 1 as libc::c_int as libc::c_longlong {
                        b = *((*g).k).as_mut_ptr().offset(1 as libc::c_int as isize);
                    }
                    if gn > 2 as libc::c_int as libc::c_longlong {
                        c = *((*g).k).as_mut_ptr().offset(2 as libc::c_int as isize);
                    }
                    if gn > 3 as libc::c_int as libc::c_longlong {
                        d = *((*g).k).as_mut_ptr().offset(3 as libc::c_int as isize);
                    }
                    if gn > 2 as libc::c_int as libc::c_longlong
                        && (q == offsetWhat || q == offsetSSR)
                    {
                        z = if q == offsetWhat {
                            Some(what_triadic as unsafe extern "C" fn(K, K, K) -> K)
                        } else {
                            Some(_ssr as unsafe extern "C" fn(K, K, K) -> K)
                        }
                            .unwrap()(a, b, c);
                    } else if gn > 2 as libc::c_int as libc::c_longlong
                        && (q == offsetAt || q == offsetDot)
                    {
                        z = if q == offsetAt {
                            Some(at_tetradic as unsafe extern "C" fn(K, K, K, K) -> K)
                        } else {
                            Some(dot_tetradic as unsafe extern "C" fn(K, K, K, K) -> K)
                        }
                            .unwrap()(a, b, c, d);
                    } else if 2 as libc::c_int as libc::c_longlong == k && !a.is_null()
                        && !b.is_null()
                    {
                        fnc = (*DT.as_mut_ptr().offset(q as L as isize)).text;
                        if fnci < 127 as libc::c_int as libc::c_longlong {
                            fncp[fnci as usize] = q;
                            fnci += 1;
                            fnci;
                        }
                        if !cls.is_null()
                            && (*a).t == 6 as libc::c_int as libc::c_longlong
                        {
                            z = (::std::mem::transmute::<
                                V,
                                Option::<unsafe extern "C" fn(K, K) -> K>,
                            >((*DT.as_mut_ptr().offset(q as L as isize)).func))
                                .unwrap()(cls, b);
                        } else {
                            z = (::std::mem::transmute::<
                                V,
                                Option::<unsafe extern "C" fn(K, K) -> K>,
                            >((*DT.as_mut_ptr().offset(q as L as isize)).func))
                                .unwrap()(a, b);
                        }
                    } else if 2 as libc::c_int as libc::c_longlong == k && a.is_null() {
                        kerr(b"valence\0" as *const u8 as *const libc::c_char);
                    } else if (2 as libc::c_int as libc::c_longlong == k
                        || q == offsetSSR) && b.is_null()
                    {
                        let mut v: K = Kv();
                        let mut kb: K = newK(
                            -(4 as libc::c_int) as I,
                            2 as libc::c_int as I,
                        );
                        if OOM_CD(0 as libc::c_int as I, v, kb, -(1 as libc::c_int) as V)
                            == 0
                        {
                            return 0 as K;
                        }
                        let ref mut fresh19 = *((*kb).k)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize);
                        *fresh19 = q as *mut k0;
                        let ref mut fresh20 = *((*kb).k)
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize);
                        *fresh20 = 0 as *mut k0;
                        let ref mut fresh21 = *(((*v).k).as_mut_ptr() as *mut V)
                            .offset(CODE as libc::c_int as isize);
                        *fresh21 = kb as V;
                        z = vf_ex(&mut v as *mut K as V, g);
                        cd(v);
                    } else if 1 as libc::c_int as libc::c_longlong == k && !a.is_null() {
                        z = (::std::mem::transmute::<
                            V,
                            Option::<unsafe extern "C" fn(K) -> K>,
                        >((*DT.as_mut_ptr().offset(q as L as isize)).func))
                            .unwrap()(a);
                    } else if !(1 as libc::c_int as libc::c_longlong == k && a.is_null())
                    {
                        f = *(q as *mut V) as K;
                        ft = (*f).t;
                        if ft != 7 as libc::c_int as libc::c_longlong {
                            z = if !g.is_null() { dot(f, g) } else { f };
                        } else {
                            t = (*f).n;
                            if -(1 as libc::c_int) as libc::c_longlong == n {
                                n = valence(f as V);
                            }
                            o = *(((*f).k).as_mut_ptr() as *mut V)
                                .offset(CODE as libc::c_int as isize) as K;
                            p = *(((*f).k).as_mut_ptr() as *mut V)
                                .offset(PARAMS as libc::c_int as isize) as K;
                            s = *(((*f).k).as_mut_ptr() as *mut V)
                                .offset(LOCALS as libc::c_int as isize) as K;
                            r = *(((*f).k).as_mut_ptr() as *mut V)
                                .offset(CONJ as libc::c_int as isize) as K;
                            special = (1 as libc::c_int as libc::c_longlong == t
                                && r.is_null()
                                && (offsetAt
                                    == *(((*(*(((*f).k).as_mut_ptr() as *mut V)
                                        .offset(CODE as libc::c_int as isize) as K))
                                        .k)
                                        .as_mut_ptr() as *mut S as *mut V)
                                    || offsetDot
                                        == *(((*(*(((*f).k).as_mut_ptr() as *mut V)
                                            .offset(CODE as libc::c_int as isize) as K))
                                            .k)
                                            .as_mut_ptr() as *mut S as *mut V)
                                    || offsetWhat
                                        == *(((*(*(((*f).k).as_mut_ptr() as *mut V)
                                            .offset(CODE as libc::c_int as isize) as K))
                                            .k)
                                            .as_mut_ptr() as *mut S as *mut V))) as libc::c_int as I;
                            if (*o).t != -(3 as libc::c_int) as libc::c_longlong {
                                let mut ii: I = (*o).n
                                    - 2 as libc::c_int as libc::c_longlong;
                                let mut u: *mut V = (((*o).k).as_mut_ptr() as *mut V)
                                    .offset(ii as isize);
                                if 2 as libc::c_int as libc::c_longlong == n
                                    && 1 as libc::c_int as libc::c_longlong == adverbClass(*u)
                                {
                                    n = gn;
                                }
                            }
                            if (**((**(q as *mut K)).k)
                                .as_mut_ptr()
                                .offset(CODE as libc::c_int as isize))
                                .n == 3 as libc::c_int as libc::c_longlong
                                && offsetWhat
                                    == *(((*(*((**(q as *mut K)).k)
                                        .as_mut_ptr()
                                        .offset(CODE as libc::c_int as isize) as K))
                                        .k)
                                        .as_mut_ptr() as *mut V)
                                        .offset(1 as libc::c_int as isize)
                            {
                                z = what(
                                    *(*(((*(*((**(q as *mut K)).k)
                                        .as_mut_ptr()
                                        .offset(CODE as libc::c_int as isize) as K))
                                        .k)
                                        .as_mut_ptr() as *mut V)
                                        .offset(0 as libc::c_int as isize) as *mut K),
                                    *(((*g).k).as_mut_ptr() as *mut V as *mut K),
                                );
                            } else if n != 0
                                && (argc < gn
                                    || gn < n
                                        && (special == 0
                                            || gn <= 1 as libc::c_int as libc::c_longlong))
                            {
                                z = kcloneI(
                                    f,
                                    b"src/kx.c\0" as *const u8 as *const libc::c_char,
                                    482 as libc::c_int,
                                );
                                if !z.is_null() {
                                    let mut ae: I = 0 as libc::c_int as I;
                                    let mut m: *mut K = (((*z).k).as_mut_ptr() as *mut V
                                        as *mut K)
                                        .offset(CONJ as libc::c_int as isize);
                                    if special != 0
                                        && gn != 4 as libc::c_int as libc::c_longlong
                                    {
                                        n = 2 as libc::c_int as I;
                                    }
                                    if (3 as libc::c_int as libc::c_longlong)
                                        < (**((*z).k)
                                            .as_mut_ptr()
                                            .offset(CODE as libc::c_int as isize))
                                            .n
                                        && *((*(*((*z).k)
                                            .as_mut_ptr()
                                            .offset(CODE as libc::c_int as isize) as K))
                                            .k)
                                            .as_mut_ptr()
                                            .offset(1 as libc::c_int as isize) as *mut V
                                            == offsetAt as *mut V
                                        && *((*(*((*z).k)
                                            .as_mut_ptr()
                                            .offset(CODE as libc::c_int as isize) as K))
                                            .k)
                                            .as_mut_ptr()
                                            .offset(2 as libc::c_int as isize) as *mut V
                                            == offsetEach as V as *mut V
                                    {
                                        ae = 1 as libc::c_int as I;
                                        n = 1 as libc::c_int as I;
                                    }
                                    if (*m).is_null() {
                                        *m = newK(0 as libc::c_int as I, n);
                                    }
                                    if (*m).is_null() {
                                        cd(z);
                                    } else {
                                        let mut q_0: *mut K = ((**m).k).as_mut_ptr();
                                        let mut i_0: I = 0 as libc::c_int as I;
                                        let mut _i_0: I = (**m).n;
                                        while i_0 < _i_0 {
                                            if (*q_0.offset(i_0 as isize)).is_null() && j < gn {
                                                let fresh22 = j;
                                                j = j + 1;
                                                let ref mut fresh23 = *q_0.offset(i_0 as isize);
                                                *fresh23 = ci(
                                                    *((*g).k).as_mut_ptr().offset(fresh22 as isize),
                                                );
                                            }
                                            i_0 += 1;
                                            i_0;
                                        }
                                        if ae != 0 {
                                            let mut w_0: [V; 5] = [0 as *mut libc::c_void; 5];
                                            w_0[0 as libc::c_int
                                                as usize] = *(((*(*((*z).k)
                                                .as_mut_ptr()
                                                .offset(CODE as libc::c_int as isize) as K))
                                                .k)
                                                .as_mut_ptr() as *mut S)
                                                .offset(0 as libc::c_int as isize) as V;
                                            w_0[1 as libc::c_int as usize] = offsetAt;
                                            w_0[2 as libc::c_int as usize] = offsetEach as V;
                                            w_0[3 as libc::c_int
                                                as usize] = ((*(*((*z).k)
                                                .as_mut_ptr()
                                                .offset(CONJ as libc::c_int as isize) as K))
                                                .k)
                                                .as_mut_ptr() as V;
                                            w_0[4 as libc::c_int as usize] = 0 as V;
                                            let mut zz: K = ex2(
                                                &mut *w_0.as_mut_ptr().offset(0 as libc::c_int as isize),
                                                0 as K,
                                            );
                                            cd(g);
                                            cd(z);
                                            return zz;
                                        }
                                    }
                                }
                            } else {
                                v_0 = 0 as *mut libc::c_void;
                                tree = 0 as *mut k0;
                                match t {
                                    1 => {
                                        if r.is_null() {
                                            z = ex2(
                                                ((*(*(((*f).k).as_mut_ptr() as *mut V)
                                                    .offset(CODE as libc::c_int as isize) as K))
                                                    .k)
                                                    .as_mut_ptr() as *mut S as *mut V,
                                                g,
                                            );
                                            current_block = 15299809402464954699;
                                        } else {
                                            m_0 = newK(0 as libc::c_int as I, (*r).n);
                                            if m_0.is_null() {
                                                current_block = 15299809402464954699;
                                            } else {
                                                q_1 = ((*m_0).k).as_mut_ptr();
                                                let mut i_1: I = 0 as libc::c_int as I;
                                                let mut _i_1: I = (*m_0).n;
                                                while i_1 < _i_1 {
                                                    let ref mut fresh24 = *q_1.offset(i_1 as isize);
                                                    *fresh24 = ci(*((*r).k).as_mut_ptr().offset(i_1 as isize));
                                                    if (*q_1.offset(i_1 as isize)).is_null() && j < gn {
                                                        let fresh25 = j;
                                                        j = j + 1;
                                                        let ref mut fresh26 = *q_1.offset(i_1 as isize);
                                                        *fresh26 = ci(
                                                            *((*g).k).as_mut_ptr().offset(fresh25 as isize),
                                                        );
                                                    }
                                                    i_1 += 1;
                                                    i_1;
                                                }
                                                if prj != 0 {
                                                    let mut w_1: *mut V = &mut *(((*(*(((*f).k).as_mut_ptr()
                                                        as *mut V)
                                                        .offset(CODE as libc::c_int as isize) as K))
                                                        .k)
                                                        .as_mut_ptr() as *mut S as *mut V)
                                                        .offset(1 as libc::c_int as isize) as *mut V;
                                                    z = bv_ex(w_1, m_0);
                                                } else {
                                                    z = ex2(
                                                        ((*(*(((*f).k).as_mut_ptr() as *mut V)
                                                            .offset(CODE as libc::c_int as isize) as K))
                                                            .k)
                                                            .as_mut_ptr() as *mut S as *mut V,
                                                        m_0,
                                                    );
                                                }
                                                cd(m_0);
                                                current_block = 3304481414499905106;
                                            }
                                        }
                                    }
                                    2 => {
                                        v_0 = *(((*(*(((*f).k).as_mut_ptr() as *mut V)
                                            .offset(CODE as libc::c_int as isize) as K))
                                            .k)
                                            .as_mut_ptr() as *mut S as *mut V)
                                            .offset(1 as libc::c_int as isize);
                                        a_0 = [0 as *mut k0; 7];
                                        let mut i_2: I = 0 as libc::c_int as I;
                                        let mut _i_2: I = 7 as libc::c_int as I;
                                        while i_2 < _i_2 {
                                            a_0[i_2 as usize] = 0 as K;
                                            i_2 += 1;
                                            i_2;
                                        }
                                        if !r.is_null() {
                                            memcpy(
                                                a_0.as_mut_ptr() as *mut libc::c_void,
                                                ((*r).k).as_mut_ptr() as *const libc::c_void,
                                                ((if (*r).n < 7 as libc::c_int as libc::c_longlong {
                                                    (*r).n
                                                } else {
                                                    7 as libc::c_int as libc::c_longlong
                                                }) as libc::c_ulonglong)
                                                    .wrapping_mul(
                                                        ::std::mem::size_of::<V>() as libc::c_ulong
                                                            as libc::c_ulonglong,
                                                    ) as libc::c_ulong,
                                            );
                                        }
                                        let mut i_3: I = 0 as libc::c_int as I;
                                        let mut _i_3: I = 7 as libc::c_int as I;
                                        while i_3 < _i_3 {
                                            if (a_0[i_3 as usize]).is_null() && j < gn {
                                                let fresh27 = j;
                                                j = j + 1;
                                                a_0[i_3
                                                    as usize] = *((*g).k).as_mut_ptr().offset(fresh27 as isize);
                                            }
                                            i_3 += 1;
                                            i_3;
                                        }
                                        match n {
                                            0 => {
                                                z = ::std::mem::transmute::<
                                                    _,
                                                    fn() -> K,
                                                >(
                                                    (::std::mem::transmute::<
                                                        V,
                                                        Option::<unsafe extern "C" fn() -> K>,
                                                    >(v_0))
                                                        .unwrap(),
                                                )();
                                            }
                                            1 => {
                                                z = (::std::mem::transmute::<
                                                    V,
                                                    Option::<unsafe extern "C" fn(K) -> K>,
                                                >(v_0))
                                                    .unwrap()(a_0[0 as libc::c_int as usize]);
                                            }
                                            2 => {
                                                z = (::std::mem::transmute::<
                                                    V,
                                                    Option::<unsafe extern "C" fn(K, K) -> K>,
                                                >(v_0))
                                                    .unwrap()(
                                                    a_0[0 as libc::c_int as usize],
                                                    a_0[1 as libc::c_int as usize],
                                                );
                                            }
                                            3 => {
                                                z = (::std::mem::transmute::<
                                                    V,
                                                    Option::<unsafe extern "C" fn(K, K, K) -> K>,
                                                >(v_0))
                                                    .unwrap()(
                                                    a_0[0 as libc::c_int as usize],
                                                    a_0[1 as libc::c_int as usize],
                                                    a_0[2 as libc::c_int as usize],
                                                );
                                            }
                                            4 => {
                                                z = (::std::mem::transmute::<
                                                    V,
                                                    Option::<unsafe extern "C" fn(K, K, K, K) -> K>,
                                                >(v_0))
                                                    .unwrap()(
                                                    a_0[0 as libc::c_int as usize],
                                                    a_0[1 as libc::c_int as usize],
                                                    a_0[2 as libc::c_int as usize],
                                                    a_0[3 as libc::c_int as usize],
                                                );
                                            }
                                            5 => {
                                                z = (::std::mem::transmute::<
                                                    V,
                                                    Option::<unsafe extern "C" fn(K, K, K, K, K) -> K>,
                                                >(v_0))
                                                    .unwrap()(
                                                    a_0[0 as libc::c_int as usize],
                                                    a_0[1 as libc::c_int as usize],
                                                    a_0[2 as libc::c_int as usize],
                                                    a_0[3 as libc::c_int as usize],
                                                    a_0[4 as libc::c_int as usize],
                                                );
                                            }
                                            6 => {
                                                z = (::std::mem::transmute::<
                                                    V,
                                                    Option::<unsafe extern "C" fn(K, K, K, K, K, K) -> K>,
                                                >(v_0))
                                                    .unwrap()(
                                                    a_0[0 as libc::c_int as usize],
                                                    a_0[1 as libc::c_int as usize],
                                                    a_0[2 as libc::c_int as usize],
                                                    a_0[3 as libc::c_int as usize],
                                                    a_0[4 as libc::c_int as usize],
                                                    a_0[5 as libc::c_int as usize],
                                                );
                                            }
                                            7 => {
                                                z = (::std::mem::transmute::<
                                                    V,
                                                    Option::<unsafe extern "C" fn(K, K, K, K, K, K, K) -> K>,
                                                >(v_0))
                                                    .unwrap()(
                                                    a_0[0 as libc::c_int as usize],
                                                    a_0[1 as libc::c_int as usize],
                                                    a_0[2 as libc::c_int as usize],
                                                    a_0[3 as libc::c_int as usize],
                                                    a_0[4 as libc::c_int as usize],
                                                    a_0[5 as libc::c_int as usize],
                                                    a_0[6 as libc::c_int as usize],
                                                );
                                            }
                                            _ => {}
                                        }
                                        current_block = 3304481414499905106;
                                    }
                                    3 => {
                                        if *(((*f).k).as_mut_ptr() as *mut V)
                                            .offset(DEPTH as libc::c_int as isize) as L
                                            > 500 as libc::c_int as libc::c_longlong
                                        {
                                            kerr(b"stack\0" as *const u8 as *const libc::c_char);
                                            current_block = 15299809402464954699;
                                        } else if stk as libc::c_double > 2e6f64 {
                                            kerr(b"stack\0" as *const u8 as *const libc::c_char);
                                            current_block = 15299809402464954699;
                                        } else {
                                            stk += 1;
                                            stk;
                                            let mut j_0: I = 0 as libc::c_int as I;
                                            let mut e: *mut K = 0 as *mut K;
                                            let mut fw: K = 0 as *mut k0;
                                            tree = *(((*f).k).as_mut_ptr() as *mut V)
                                                .offset(CACHE_TREE as libc::c_int as isize) as K;
                                            if tree.is_null() {
                                                tree = newK(5 as libc::c_int as I, (*p).n + (*s).n);
                                                if tree.is_null() {
                                                    stk -= 1;
                                                    stk;
                                                    current_block = 15299809402464954699;
                                                } else {
                                                    let mut i_4: I = 0 as libc::c_int as I;
                                                    let mut _i_4: I = (*tree).n;
                                                    loop {
                                                        if !(i_4 < _i_4) {
                                                            current_block = 9763808675153183319;
                                                            break;
                                                        }
                                                        let ref mut fresh28 = *((*tree).k)
                                                            .as_mut_ptr()
                                                            .offset(i_4 as isize);
                                                        *fresh28 = newK(
                                                            0 as libc::c_int as I,
                                                            3 as libc::c_int as I,
                                                        );
                                                        if (*fresh28).is_null() {
                                                            cd(tree);
                                                            stk -= 1;
                                                            stk;
                                                            current_block = 15299809402464954699;
                                                            break;
                                                        } else {
                                                            i_4 += 1;
                                                            i_4;
                                                        }
                                                    }
                                                    match current_block {
                                                        15299809402464954699 => {}
                                                        _ => {
                                                            let mut i_5: I = 0 as libc::c_int as I;
                                                            let mut _i_5: I = (*tree).n;
                                                            while i_5 < _i_5 {
                                                                let mut j_1: I = 0 as libc::c_int as I;
                                                                let mut _j: I = 3 as libc::c_int as I;
                                                                while j_1 < _j {
                                                                    let ref mut fresh29 = *((*DI(tree, i_5)).k)
                                                                        .as_mut_ptr()
                                                                        .offset(j_1 as isize);
                                                                    *fresh29 = ci(
                                                                        *((*if i_5 < (*p).n {
                                                                            DI(p, i_5)
                                                                        } else {
                                                                            DI(s, i_5 - (*p).n)
                                                                        })
                                                                            .k)
                                                                            .as_mut_ptr()
                                                                            .offset(j_1 as isize),
                                                                    );
                                                                    j_1 += 1;
                                                                    j_1;
                                                                }
                                                                i_5 += 1;
                                                                i_5;
                                                            }
                                                            let ref mut fresh30 = *(((*f).k).as_mut_ptr() as *mut V)
                                                                .offset(CACHE_TREE as libc::c_int as isize);
                                                            *fresh30 = tree as V;
                                                            current_block = 12151070351325546249;
                                                        }
                                                    }
                                                }
                                            } else {
                                                current_block = 12151070351325546249;
                                            }
                                            match current_block {
                                                15299809402464954699 => {}
                                                _ => {
                                                    if fsf != 0 {
                                                        let mut j0: K = dot_monadic(
                                                            *(((*prnt).k).as_mut_ptr() as *mut V)
                                                                .offset(LOCALS as libc::c_int as isize) as K,
                                                        );
                                                        let mut j1: K = dot_monadic(
                                                            *(((*prnt).k).as_mut_ptr() as *mut V)
                                                                .offset(CACHE_TREE as libc::c_int as isize) as K,
                                                        );
                                                        let mut j2: K = join(ci(j0), j1);
                                                        cd(j0);
                                                        cd(
                                                            *(((*prnt).k).as_mut_ptr() as *mut V)
                                                                .offset(CACHE_TREE as libc::c_int as isize) as K,
                                                        );
                                                        let ref mut fresh31 = *(((*prnt).k).as_mut_ptr() as *mut V)
                                                            .offset(CACHE_TREE as libc::c_int as isize);
                                                        *fresh31 = dot_monadic(j2) as V;
                                                        cd(j0);
                                                        cd(j1);
                                                        cd(j2);
                                                        tree = *(((*prnt).k).as_mut_ptr() as *mut V)
                                                            .offset(CACHE_TREE as libc::c_int as isize) as K;
                                                        cd(
                                                            *(((*prnt).k).as_mut_ptr() as *mut V)
                                                                .offset(CACHE_WD as libc::c_int as isize) as K,
                                                        );
                                                        let ref mut fresh32 = *(((*prnt).k).as_mut_ptr() as *mut V)
                                                            .offset(CACHE_WD as libc::c_int as isize);
                                                        *fresh32 = 0 as V;
                                                    }
                                                    let mut i_6: I = 0 as libc::c_int as I;
                                                    let mut _i_6: I = (*p).n;
                                                    while i_6 < _i_6 {
                                                        e = EVP(DI(tree, i_6));
                                                        cd(*e);
                                                        *e = 0 as K;
                                                        if !r.is_null() && i_6 < (*r).n {
                                                            *e = ci(*((*r).k).as_mut_ptr().offset(i_6 as isize));
                                                        }
                                                        if (*e).is_null() && j_0 < (*g).n {
                                                            let fresh33 = j_0;
                                                            j_0 = j_0 + 1;
                                                            *e = ci(*((*g).k).as_mut_ptr().offset(fresh33 as isize));
                                                        }
                                                        i_6 += 1;
                                                        i_6;
                                                    }
                                                    fw = *(((*f).k).as_mut_ptr() as *mut V)
                                                        .offset(CACHE_WD as libc::c_int as isize) as K;
                                                    let mut t_0: I = 0 as libc::c_int as I;
                                                    if fw.is_null()
                                                        || {
                                                            t_0 = (*(((*(*((*fw).k)
                                                                .as_mut_ptr()
                                                                .offset(CODE as libc::c_int as isize) as K))
                                                                .k)
                                                                .as_mut_ptr() as *mut S)
                                                                .offset(0 as libc::c_int as isize) as UI
                                                                > DT_SIZE as libc::c_ulonglong
                                                                || *(((*(*((*fw).k)
                                                                    .as_mut_ptr()
                                                                    .offset(CODE as libc::c_int as isize) as K))
                                                                    .k)
                                                                    .as_mut_ptr() as *mut S)
                                                                    .offset(1 as libc::c_int as isize) as UI
                                                                    > DT_SIZE as libc::c_ulonglong) as libc::c_int as I;
                                                            t_0 != 0
                                                        }
                                                    {
                                                        if t_0 != 0 {
                                                            cd(
                                                                *(((*f).k).as_mut_ptr() as *mut V)
                                                                    .offset(CACHE_WD as libc::c_int as isize) as K,
                                                            );
                                                        }
                                                        let mut fc: K = kcloneI(
                                                            f,
                                                            b"src/kx.c\0" as *const u8 as *const libc::c_char,
                                                            539 as libc::c_int,
                                                        );
                                                        cd(
                                                            *(((*fc).k).as_mut_ptr() as *mut V)
                                                                .offset(CONJ as libc::c_int as isize) as K,
                                                        );
                                                        let ref mut fresh34 = *(((*fc).k).as_mut_ptr() as *mut V)
                                                            .offset(CONJ as libc::c_int as isize);
                                                        *fresh34 = 0 as V;
                                                        let ref mut fresh35 = *(((*fc).k).as_mut_ptr() as *mut V)
                                                            .offset(DEPTH as libc::c_int as isize);
                                                        *fresh35 = (*fresh35).offset(1);
                                                        *fresh35;
                                                        let mut tt: I = 0 as libc::c_int as I;
                                                        let mut ttt: I = 0 as libc::c_int as I;
                                                        let mut i_7: I = 0 as libc::c_int as I;
                                                        i_7 = 0 as libc::c_int as I;
                                                        while i_7 < (*o).n - 3 as libc::c_int as libc::c_longlong {
                                                            if *(((*o).k).as_mut_ptr() as *mut C).offset(i_7 as isize)
                                                                as libc::c_int == '{' as i32
                                                            {
                                                                tt = 1 as libc::c_int as I;
                                                                if *(((*o).k).as_mut_ptr() as *mut C)
                                                                    .offset(
                                                                        (i_7 + 1 as libc::c_int as libc::c_longlong) as isize,
                                                                    ) as libc::c_int == ':' as i32
                                                                {
                                                                    ttt = 1 as libc::c_int as I;
                                                                    break;
                                                                }
                                                            }
                                                            i_7 += 1;
                                                            i_7;
                                                        }
                                                        if ttt == 0
                                                            && (grnt.is_null() || tt != 0
                                                                || *(((*o).k).as_mut_ptr() as *mut C)
                                                                    .offset(0 as libc::c_int as isize) as libc::c_int
                                                                    == '[' as i32)
                                                        {
                                                            fw = wd_(
                                                                ((*o).k).as_mut_ptr() as *mut C,
                                                                (*o).n as libc::c_int,
                                                                &mut tree,
                                                                fc,
                                                            );
                                                        } else {
                                                            tc = kcloneI(
                                                                tree,
                                                                b"src/kx.c\0" as *const u8 as *const libc::c_char,
                                                                547 as libc::c_int,
                                                            );
                                                            fw = wd_(
                                                                ((*o).k).as_mut_ptr() as *mut C,
                                                                (*o).n as libc::c_int,
                                                                &mut tc,
                                                                fc,
                                                            );
                                                        }
                                                        let ref mut fresh36 = *(((*f).k).as_mut_ptr() as *mut V)
                                                            .offset(CACHE_WD as libc::c_int as isize);
                                                        *fresh36 = fw as V;
                                                        cd(fc);
                                                    }
                                                    if stk1 as libc::c_double > 1e3f64 {
                                                        cd(g);
                                                        kerr(b"stack\0" as *const u8 as *const libc::c_char);
                                                        return _n();
                                                    }
                                                    ci(fw);
                                                    stk1 += 1;
                                                    stk1;
                                                    z = ex(fw);
                                                    stk1 -= 1;
                                                    stk1;
                                                    let mut i_8: I = 0 as libc::c_int as I;
                                                    let mut _i_7: I = (*p).n;
                                                    while i_8 < _i_7 {
                                                        e = EVP(DI(tree, i_8));
                                                        cd(*e);
                                                        *e = 0 as K;
                                                        i_8 += 1;
                                                        i_8;
                                                    }
                                                    stk -= 1;
                                                    stk;
                                                    current_block = 3304481414499905106;
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        current_block = 3304481414499905106;
                                    }
                                }
                                match current_block {
                                    15299809402464954699 => {}
                                    _ => {
                                        if encp == 2 as libc::c_int as libc::c_longlong {
                                            let mut ff: I = 0 as libc::c_int as I;
                                            if !z.is_null()
                                                && (*z).t == 7 as libc::c_int as libc::c_longlong
                                                && (*z).n == 3 as libc::c_int as libc::c_longlong
                                                && !(*(((*z).k).as_mut_ptr() as *mut V)
                                                    .offset(CODE as libc::c_int as isize))
                                                    .is_null()
                                                && !(strchr(
                                                    ((*(*((*z).k)
                                                        .as_mut_ptr()
                                                        .offset(CODE as libc::c_int as isize) as K))
                                                        .k)
                                                        .as_mut_ptr() as *mut C,
                                                    (*::std::mem::transmute::<
                                                        &[u8; 2],
                                                        &[libc::c_char; 2],
                                                    >(b"z\0"))[0 as libc::c_int as usize] as libc::c_int,
                                                ))
                                                    .is_null()
                                                && !(*(((*z).k).as_mut_ptr() as *mut V)
                                                    .offset(PARAMS as libc::c_int as isize))
                                                    .is_null()
                                                && (**((*z).k)
                                                    .as_mut_ptr()
                                                    .offset(PARAMS as libc::c_int as isize))
                                                    .n != 0
                                            {
                                                ff = 1 as libc::c_int as I;
                                                let mut i_9: I = 0 as libc::c_int as I;
                                                let mut _i_8: I = (**((*z).k)
                                                    .as_mut_ptr()
                                                    .offset(PARAMS as libc::c_int as isize))
                                                    .n;
                                                while i_9 < _i_8 {
                                                    if strcmp(
                                                        *(((*(*((*(*((*(*((*z).k)
                                                            .as_mut_ptr()
                                                            .offset(PARAMS as libc::c_int as isize) as K))
                                                            .k)
                                                            .as_mut_ptr()
                                                            .offset(i_9 as isize) as K))
                                                            .k)
                                                            .as_mut_ptr()
                                                            .offset(0 as libc::c_int as isize) as K))
                                                            .k)
                                                            .as_mut_ptr() as *mut S) as *const libc::c_char,
                                                        b"z\0" as *const u8 as *const libc::c_char,
                                                    ) == 0
                                                    {
                                                        ff = 0 as libc::c_int as I;
                                                        break;
                                                    } else {
                                                        i_9 += 1;
                                                        i_9;
                                                    }
                                                }
                                            }
                                            if ff != 0 {
                                                let mut d_0: K = *((*(*((*KTREE).k)
                                                    .as_mut_ptr()
                                                    .offset(0 as libc::c_int as isize) as K))
                                                    .k)
                                                    .as_mut_ptr()
                                                    .offset(1 as libc::c_int as isize);
                                                let mut w_2: K = 0 as K;
                                                let mut i_10: I = 0 as libc::c_int as I;
                                                let mut _i_9: I = (*d_0).n;
                                                while i_10 < _i_9 {
                                                    if strcmp(
                                                        *(((*(*((*(*((*d_0).k).as_mut_ptr().offset(i_10 as isize)
                                                            as K))
                                                            .k)
                                                            .as_mut_ptr()
                                                            .offset(0 as libc::c_int as isize) as K))
                                                            .k)
                                                            .as_mut_ptr() as *mut S) as *const libc::c_char,
                                                        b"z\0" as *const u8 as *const libc::c_char,
                                                    ) == 0
                                                    {
                                                        w_2 = kcloneI(
                                                            *((*d_0).k).as_mut_ptr().offset(i_10 as isize),
                                                            b"src/kx.c\0" as *const u8 as *const libc::c_char,
                                                            559 as libc::c_int,
                                                        );
                                                        break;
                                                    } else {
                                                        i_10 += 1;
                                                        i_10;
                                                    }
                                                }
                                                if !w_2.is_null() {
                                                    let mut p_0: K = *((*g).k)
                                                        .as_mut_ptr()
                                                        .offset(0 as libc::c_int as isize);
                                                    cd(
                                                        *((*w_2).k).as_mut_ptr().offset(1 as libc::c_int as isize),
                                                    );
                                                    let ref mut fresh37 = *((*w_2).k)
                                                        .as_mut_ptr()
                                                        .offset(1 as libc::c_int as isize);
                                                    *fresh37 = kcloneI(
                                                        p_0,
                                                        b"src/kx.c\0" as *const u8 as *const libc::c_char,
                                                        561 as libc::c_int,
                                                    );
                                                    let mut we: K = enlist(w_2);
                                                    let mut j0_0: K = dot_monadic(
                                                        *((*z).k)
                                                            .as_mut_ptr()
                                                            .offset(CACHE_TREE as libc::c_int as isize),
                                                    );
                                                    let mut j2_0: K = join(ci(j0_0), we);
                                                    cd(j0_0);
                                                    cd(
                                                        *((*z).k)
                                                            .as_mut_ptr()
                                                            .offset(CACHE_TREE as libc::c_int as isize),
                                                    );
                                                    let ref mut fresh38 = *((*z).k)
                                                        .as_mut_ptr()
                                                        .offset(CACHE_TREE as libc::c_int as isize);
                                                    *fresh38 = dot_monadic(j2_0);
                                                    cd(w_2);
                                                    cd(we);
                                                    cd(j0_0);
                                                    cd(j2_0);
                                                    encp = 3 as libc::c_int as I;
                                                }
                                            }
                                        }
                                        if encp == 1 as libc::c_int as libc::c_longlong {
                                            let mut ff_0: I = 0 as libc::c_int as I;
                                            if !z.is_null()
                                                && (*z).t == 7 as libc::c_int as libc::c_longlong
                                                && (*z).n == 3 as libc::c_int as libc::c_longlong
                                                && !(*(((*z).k).as_mut_ptr() as *mut V)
                                                    .offset(CODE as libc::c_int as isize))
                                                    .is_null()
                                                && !(strchr(
                                                    ((*(*((*z).k)
                                                        .as_mut_ptr()
                                                        .offset(CODE as libc::c_int as isize) as K))
                                                        .k)
                                                        .as_mut_ptr() as *mut C,
                                                    (*::std::mem::transmute::<
                                                        &[u8; 2],
                                                        &[libc::c_char; 2],
                                                    >(b"y\0"))[0 as libc::c_int as usize] as libc::c_int,
                                                ))
                                                    .is_null()
                                                && !(*(((*z).k).as_mut_ptr() as *mut V)
                                                    .offset(PARAMS as libc::c_int as isize))
                                                    .is_null()
                                                && (**((*z).k)
                                                    .as_mut_ptr()
                                                    .offset(PARAMS as libc::c_int as isize))
                                                    .n != 0
                                            {
                                                ff_0 = 1 as libc::c_int as I;
                                                let mut i_11: I = 0 as libc::c_int as I;
                                                let mut _i_10: I = (**((*z).k)
                                                    .as_mut_ptr()
                                                    .offset(PARAMS as libc::c_int as isize))
                                                    .n;
                                                while i_11 < _i_10 {
                                                    if strcmp(
                                                        *(((*(*((*(*((*(*((*z).k)
                                                            .as_mut_ptr()
                                                            .offset(PARAMS as libc::c_int as isize) as K))
                                                            .k)
                                                            .as_mut_ptr()
                                                            .offset(i_11 as isize) as K))
                                                            .k)
                                                            .as_mut_ptr()
                                                            .offset(0 as libc::c_int as isize) as K))
                                                            .k)
                                                            .as_mut_ptr() as *mut S) as *const libc::c_char,
                                                        b"y\0" as *const u8 as *const libc::c_char,
                                                    ) == 0
                                                    {
                                                        ff_0 = 0 as libc::c_int as I;
                                                        break;
                                                    } else {
                                                        i_11 += 1;
                                                        i_11;
                                                    }
                                                }
                                            }
                                            if ff_0 != 0 {
                                                let mut d_1: K = *((*(*((*KTREE).k)
                                                    .as_mut_ptr()
                                                    .offset(0 as libc::c_int as isize) as K))
                                                    .k)
                                                    .as_mut_ptr()
                                                    .offset(1 as libc::c_int as isize);
                                                let mut y: K = 0 as K;
                                                if 6 as libc::c_int as libc::c_longlong != (*d_1).t
                                                    && !(5 as libc::c_int as libc::c_longlong == (*d_1).t
                                                        && 6 as libc::c_int as libc::c_longlong
                                                            == (**((*(*((*d_1).k)
                                                                .as_mut_ptr()
                                                                .offset(0 as libc::c_int as isize) as K))
                                                                .k)
                                                                .as_mut_ptr()
                                                                .offset(1 as libc::c_int as isize))
                                                                .t)
                                                {
                                                    let mut i_12: I = 0 as libc::c_int as I;
                                                    let mut _i_11: I = (*d_1).n;
                                                    while i_12 < _i_11 {
                                                        if strcmp(
                                                            *(((*(*((*(*((*d_1).k).as_mut_ptr().offset(i_12 as isize)
                                                                as K))
                                                                .k)
                                                                .as_mut_ptr()
                                                                .offset(0 as libc::c_int as isize) as K))
                                                                .k)
                                                                .as_mut_ptr() as *mut S) as *const libc::c_char,
                                                            b"y\0" as *const u8 as *const libc::c_char,
                                                        ) == 0
                                                        {
                                                            y = kcloneI(
                                                                *((*d_1).k).as_mut_ptr().offset(i_12 as isize),
                                                                b"src/kx.c\0" as *const u8 as *const libc::c_char,
                                                                571 as libc::c_int,
                                                            );
                                                            break;
                                                        } else {
                                                            i_12 += 1;
                                                            i_12;
                                                        }
                                                    }
                                                } else {
                                                    return kerr(b"nyi\0" as *const u8 as *const libc::c_char)
                                                }
                                                if !y.is_null() {
                                                    let mut p_1: K = *((*g).k)
                                                        .as_mut_ptr()
                                                        .offset(0 as libc::c_int as isize);
                                                    cd(
                                                        *((*y).k).as_mut_ptr().offset(1 as libc::c_int as isize),
                                                    );
                                                    let ref mut fresh39 = *((*y).k)
                                                        .as_mut_ptr()
                                                        .offset(1 as libc::c_int as isize);
                                                    *fresh39 = kcloneI(
                                                        p_1,
                                                        b"src/kx.c\0" as *const u8 as *const libc::c_char,
                                                        574 as libc::c_int,
                                                    );
                                                    let mut ye: K = enlist(y);
                                                    let mut j0_1: K = dot_monadic(
                                                        *((*z).k)
                                                            .as_mut_ptr()
                                                            .offset(CACHE_TREE as libc::c_int as isize),
                                                    );
                                                    let mut j2_1: K = join(ci(j0_1), ye);
                                                    cd(j0_1);
                                                    cd(
                                                        *((*z).k)
                                                            .as_mut_ptr()
                                                            .offset(CACHE_TREE as libc::c_int as isize),
                                                    );
                                                    let ref mut fresh40 = *((*z).k)
                                                        .as_mut_ptr()
                                                        .offset(CACHE_TREE as libc::c_int as isize);
                                                    *fresh40 = dot_monadic(j2_1);
                                                    cd(y);
                                                    cd(ye);
                                                    cd(j0_1);
                                                    cd(j2_1);
                                                    encp = 2 as libc::c_int as I;
                                                }
                                            }
                                        }
                                        if encp == 0 as libc::c_int as libc::c_longlong {
                                            let mut ff_1: I = 0 as libc::c_int as I;
                                            if !z.is_null()
                                                && (*z).t == 7 as libc::c_int as libc::c_longlong
                                                && (*z).n == 3 as libc::c_int as libc::c_longlong
                                                && !(*(((*z).k).as_mut_ptr() as *mut V)
                                                    .offset(CODE as libc::c_int as isize))
                                                    .is_null()
                                                && !(strchr(
                                                    ((*(*((*z).k)
                                                        .as_mut_ptr()
                                                        .offset(CODE as libc::c_int as isize) as K))
                                                        .k)
                                                        .as_mut_ptr() as *mut C,
                                                    (*::std::mem::transmute::<
                                                        &[u8; 2],
                                                        &[libc::c_char; 2],
                                                    >(b"x\0"))[0 as libc::c_int as usize] as libc::c_int,
                                                ))
                                                    .is_null()
                                                && !(*(((*z).k).as_mut_ptr() as *mut V)
                                                    .offset(PARAMS as libc::c_int as isize))
                                                    .is_null()
                                                && (**((*z).k)
                                                    .as_mut_ptr()
                                                    .offset(PARAMS as libc::c_int as isize))
                                                    .n != 0
                                            {
                                                ff_1 = 1 as libc::c_int as I;
                                                let mut i_13: I = 0 as libc::c_int as I;
                                                let mut _i_12: I = (**((*z).k)
                                                    .as_mut_ptr()
                                                    .offset(PARAMS as libc::c_int as isize))
                                                    .n;
                                                while i_13 < _i_12 {
                                                    if strcmp(
                                                        *(((*(*((*(*((*(*((*z).k)
                                                            .as_mut_ptr()
                                                            .offset(PARAMS as libc::c_int as isize) as K))
                                                            .k)
                                                            .as_mut_ptr()
                                                            .offset(i_13 as isize) as K))
                                                            .k)
                                                            .as_mut_ptr()
                                                            .offset(0 as libc::c_int as isize) as K))
                                                            .k)
                                                            .as_mut_ptr() as *mut S) as *const libc::c_char,
                                                        b"x\0" as *const u8 as *const libc::c_char,
                                                    ) == 0
                                                    {
                                                        ff_1 = 0 as libc::c_int as I;
                                                        break;
                                                    } else {
                                                        i_13 += 1;
                                                        i_13;
                                                    }
                                                }
                                            }
                                            if ff_1 != 0 {
                                                let mut xx: K = newK(
                                                    4 as libc::c_int as I,
                                                    1 as libc::c_int as I,
                                                );
                                                let ref mut fresh41 = *((*xx).k).as_mut_ptr();
                                                *fresh41 = sp(
                                                    b"x\0" as *const u8 as *const libc::c_char as S,
                                                ) as V as *mut k0;
                                                let mut x: K = newK(
                                                    0 as libc::c_int as I,
                                                    3 as libc::c_int as I,
                                                );
                                                let ref mut fresh42 = *((*x).k)
                                                    .as_mut_ptr()
                                                    .offset(0 as libc::c_int as isize);
                                                *fresh42 = xx;
                                                let ref mut fresh43 = *((*x).k)
                                                    .as_mut_ptr()
                                                    .offset(1 as libc::c_int as isize);
                                                *fresh43 = _n();
                                                let ref mut fresh44 = *((*x).k)
                                                    .as_mut_ptr()
                                                    .offset(2 as libc::c_int as isize);
                                                *fresh44 = _n();
                                                let mut p_2: K = *((*g).k)
                                                    .as_mut_ptr()
                                                    .offset(0 as libc::c_int as isize);
                                                cd(
                                                    *((*x).k).as_mut_ptr().offset(1 as libc::c_int as isize),
                                                );
                                                let ref mut fresh45 = *((*x).k)
                                                    .as_mut_ptr()
                                                    .offset(1 as libc::c_int as isize);
                                                *fresh45 = kcloneI(
                                                    p_2,
                                                    b"src/kx.c\0" as *const u8 as *const libc::c_char,
                                                    584 as libc::c_int,
                                                );
                                                let mut xe: K = enlist(x);
                                                let mut j0_2: K = dot_monadic(
                                                    *((*z).k)
                                                        .as_mut_ptr()
                                                        .offset(CACHE_TREE as libc::c_int as isize),
                                                );
                                                let mut j2_2: K = join(ci(j0_2), xe);
                                                cd(j0_2);
                                                cd(
                                                    *((*z).k)
                                                        .as_mut_ptr()
                                                        .offset(CACHE_TREE as libc::c_int as isize),
                                                );
                                                let ref mut fresh46 = *((*z).k)
                                                    .as_mut_ptr()
                                                    .offset(CACHE_TREE as libc::c_int as isize);
                                                *fresh46 = dot_monadic(j2_2);
                                                cd(x);
                                                cd(xe);
                                                cd(j0_2);
                                                cd(j2_2);
                                                encp = 1 as libc::c_int as I;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    cd(g);
    cd(tc);
    return z;
}
unsafe extern "C" fn ex_(mut a: V, mut r: I) -> V {
    let mut x: K = 0 as *mut k0;
    let mut y: K = 0 as K;
    let mut z: K = 0 as *mut k0;
    let mut tmp: K = 0 as *mut k0;
    if VA(a) != 0 {
        return a;
    }
    x = *(a as *mut K);
    if x.is_null() || 7 as libc::c_int as libc::c_longlong != (*x).t
        || (0 as libc::c_int as libc::c_longlong) < (*x).n
            && (*x).n < 4 as libc::c_int as libc::c_longlong
    {
        return ci(x) as V;
    }
    r = if (*x).n < 4 as libc::c_int as libc::c_longlong { r } else { (*x).n };
    if !(*(((*x).k).as_mut_ptr() as *mut V).offset(CONJ as libc::c_int as isize))
        .is_null()
    {
        tmp = *((((*x).k).as_mut_ptr() as *mut V).offset(CONJ as libc::c_int as isize)
            as *mut K);
        if !tmp.is_null() {
            if offsetColon
                == *(((*(*(((*tmp).k).as_mut_ptr() as *mut V)
                    .offset(CODE as libc::c_int as isize) as K))
                    .k)
                    .as_mut_ptr() as *mut S as *mut V)
                && *(((*(*(((*tmp).k).as_mut_ptr() as *mut V)
                    .offset(CODE as libc::c_int as isize) as K))
                    .k)
                    .as_mut_ptr() as *mut S as *mut V)
                    .offset(1 as libc::c_int as isize) as UI
                    > DT_SIZE as libc::c_ulonglong
            {
                fer = 1 as libc::c_int as I;
            }
        }
        y = ex_(
            (((*x).k).as_mut_ptr() as *mut V).offset(CONJ as libc::c_int as isize) as V,
            2 as libc::c_int as I,
        ) as K;
        if y.is_null() {
            return 0 as V;
        }
        if (*y).t == 0 as libc::c_int as libc::c_longlong
            && (*y).n == 0 as libc::c_int as libc::c_longlong
        {
            cd(y);
            y = _n();
        }
        if fer > 0 as libc::c_int as libc::c_longlong && fCheck == 0 {
            return y as V;
        }
    }
    z = ex0(
        ((*(*(((*x).k).as_mut_ptr() as *mut V).offset(CODE as libc::c_int as isize)
            as K))
            .k)
            .as_mut_ptr() as *mut S as *mut V,
        y,
        r,
    );
    cd(y);
    return z as V;
}
pub unsafe extern "C" fn ex(mut a: K) -> K {
    if a.is_null() {
        return 0 as K;
    }
    if (*a).t == 7 as libc::c_int as libc::c_longlong
        && *(((*a).k).as_mut_ptr() as *mut V).offset(CODE as libc::c_int as isize) as K
            > DT_SIZE as K
        && 7 as libc::c_int as libc::c_longlong
            == (*(*(((*a).k).as_mut_ptr() as *mut V).offset(CODE as libc::c_int as isize)
                as K))
                .t
        && 6 as libc::c_int as libc::c_longlong
            == (*(*(((*a).k).as_mut_ptr() as *mut V).offset(CODE as libc::c_int as isize)
                as K))
                .n
    {
        fwh = 1 as libc::c_int as I;
    }
    if (*a).t == 7 as libc::c_int as libc::c_longlong {
        if prnt.is_null() {
            if *(((*(*(((*a).k).as_mut_ptr() as *mut V)
                .offset(CODE as libc::c_int as isize) as K))
                .k)
                .as_mut_ptr() as *mut S as *mut V)
                .offset(1 as libc::c_int as isize) == offsetColon
                && *(((*(*(((*a).k).as_mut_ptr() as *mut V)
                    .offset(CODE as libc::c_int as isize) as K))
                    .k)
                    .as_mut_ptr() as *mut S as *mut V)
                    .offset(2 as libc::c_int as isize) != offset3m
            {
                fam = 0 as libc::c_int as I;
            }
            if (*(*(((*a).k).as_mut_ptr() as *mut V).offset(CODE as libc::c_int as isize)
                as K))
                .n > 3 as libc::c_int as libc::c_longlong
            {
                let mut i: I = 3 as libc::c_int as I;
                while !(*(((*(*(((*a).k).as_mut_ptr() as *mut V)
                    .offset(CODE as libc::c_int as isize) as K))
                    .k)
                    .as_mut_ptr() as *mut S as *mut V)
                    .offset(i as isize))
                    .is_null()
                {
                    let fresh47 = i;
                    i = i + 1;
                    if *(((*(*(((*a).k).as_mut_ptr() as *mut V)
                        .offset(CODE as libc::c_int as isize) as K))
                        .k)
                        .as_mut_ptr() as *mut S as *mut V)
                        .offset(fresh47 as isize) == 0x1 as libc::c_int as V
                    {
                        fam = 1 as libc::c_int as I;
                    }
                    if *(((*(*(((*a).k).as_mut_ptr() as *mut V)
                        .offset(CODE as libc::c_int as isize) as K))
                        .k)
                        .as_mut_ptr() as *mut S as *mut V)
                        .offset((i - 1 as libc::c_int as libc::c_longlong) as isize)
                        == offsetColon
                        && *(((*(*(((*a).k).as_mut_ptr() as *mut V)
                            .offset(CODE as libc::c_int as isize) as K))
                            .k)
                            .as_mut_ptr() as *mut S as *mut V)
                            .offset(i as isize) != offset3m
                    {
                        fam = 0 as libc::c_int as I;
                    }
                }
                if fCheck == 0 && i > 2 as libc::c_int as libc::c_longlong {
                    let mut j: I = 0;
                    let mut k: I = 0 as libc::c_int as I;
                    j = i - 1 as libc::c_int as libc::c_longlong;
                    while j > 0 as libc::c_int as libc::c_longlong
                        && k < 10 as libc::c_int as libc::c_longlong
                    {
                        if *(((*(*(((*a).k).as_mut_ptr() as *mut V)
                            .offset(CODE as libc::c_int as isize) as K))
                            .k)
                            .as_mut_ptr() as *mut S as *mut V)
                            .offset(j as isize) < DT_SIZE as V
                            && *(((*(*(((*a).k).as_mut_ptr() as *mut V)
                                .offset(CODE as libc::c_int as isize) as K))
                                .k)
                                .as_mut_ptr() as *mut S as *mut V)
                                .offset(j as isize) > 1 as libc::c_int as V
                        {
                            cdp[k
                                as usize] = *(*DT
                                .as_mut_ptr()
                                .offset(
                                    *(((*(*(((*a).k).as_mut_ptr() as *mut V)
                                        .offset(CODE as libc::c_int as isize) as K))
                                        .k)
                                        .as_mut_ptr() as *mut S as *mut V)
                                        .offset(j as isize) as I as isize,
                                ))
                                .text;
                            k += 1;
                            k;
                        }
                        j -= 1;
                        j;
                    }
                }
            }
        }
    } else {
        fam = 1 as libc::c_int as I;
    }
    let mut z: K = ex_(&mut a as *mut K as V, 0 as libc::c_int as I) as K;
    cd(a);
    if fer == 1 as libc::c_int as libc::c_longlong {
        fer1 = 0 as libc::c_int as I;
        fer = fer1;
    }
    fsf = 0 as libc::c_int as I;
    prj2 = fsf;
    prj = prj2;
    stk1 = prj;
    stk = stk1;
    fwh = stk;
    if !prnt.is_null() {
        cd(prnt);
    }
    prnt = 0 as K;
    return z;
}
unsafe extern "C" fn ex0(mut v: *mut V, mut k: K, mut r: I) -> K {
    let mut n: I = 0 as libc::c_int as I;
    let mut e: I = 1 as libc::c_int as I;
    let mut i: I = 0;
    let mut a: I = 0;
    let mut b: I = 0;
    while !(*v.offset(n as isize)).is_null() {
        let fresh48 = n;
        n = n + 1;
        if bk(*v.offset(fresh48 as isize)) != 0 {
            e += 1;
            e;
        }
    }
    b = (e > 1 as libc::c_int as libc::c_longlong) as libc::c_int as I;
    let mut z: K = 0 as K;
    let mut x: K = 0 as *mut k0;
    match r {
        0 => {
            i = -(1 as libc::c_int) as I;
            while i < n {
                if -(1 as libc::c_int) as libc::c_longlong == i
                    || bk(*v.offset(i as isize)) != 0
                {
                    cd(z);
                    frg += 1;
                    frg;
                    x = ex1(
                        v.offset(1 as libc::c_int as isize).offset(i as isize),
                        0 as K,
                        &mut i,
                        n,
                        1 as libc::c_int as I,
                    );
                    frg -= 1;
                    frg;
                    if frg == 0 {
                        encp = 0 as libc::c_int as I;
                        if !encf.is_null() {
                            cd(encf);
                            encf = 0 as K;
                        }
                        if !grnt.is_null() {
                            cd(grnt);
                            grnt = 0 as K;
                        }
                    }
                    if x.is_null() {
                        return 0 as K;
                    }
                    z = if bk(x as V) != 0 { _n() } else { x };
                    if fer > 0 as libc::c_int as libc::c_longlong && fCheck == 0 {
                        return z;
                    }
                    if !grnt.is_null()
                        && (prnt.is_null()
                            || rc(prnt) == 2 as libc::c_int as libc::c_longlong)
                    {
                        if !prnt.is_null() {
                            cd(prnt);
                        }
                        prnt = ci(grnt);
                    }
                }
                i += 1;
                i;
            }
        }
        4 => {
            i = -(1 as libc::c_int) as I;
            while i < n {
                if -(1 as libc::c_int) as libc::c_longlong == i
                    || bk(*v.offset(i as isize)) != 0
                {
                    x = ex1(
                        v.offset(1 as libc::c_int as isize).offset(i as isize),
                        0 as K,
                        &mut i,
                        n,
                        1 as libc::c_int as I,
                    );
                    if x.is_null() {
                        return 0 as K;
                    }
                    if fer > 0 as libc::c_int as libc::c_longlong && fCheck == 0 {
                        return x;
                    }
                    x = if bk(x as V) != 0 { _n() } else { x };
                    loop {
                        i += 1;
                        if !(i < n && bk(*v.offset(i as isize)) == 0) {
                            break;
                        }
                    }
                    if i == n {
                        return x;
                    }
                    z = delist(x);
                    if (if (*z).t < 0 as libc::c_int as libc::c_longlong {
                        -(*z).t
                    } else {
                        (*z).t
                    }) != 1 as libc::c_int as libc::c_longlong
                        || (*z).n != 1 as libc::c_int as libc::c_longlong
                    {
                        cd(z);
                        return kerr(b"type\0" as *const u8 as *const libc::c_char);
                    }
                    a = *(((*z).k).as_mut_ptr() as *mut I);
                    cd(z);
                    if a != 0 {
                        x = ex1(
                            v.offset(i as isize).offset(1 as libc::c_int as isize),
                            0 as K,
                            &mut i,
                            n,
                            1 as libc::c_int as I,
                        );
                        x = if bk(x as V) != 0 { _n() } else { x };
                        return x;
                    } else {
                        while i < n && bk(*v.offset(i as isize)) == 0 {
                            i += 1;
                            i;
                        }
                    }
                }
                i += 1;
                i;
            }
            return _n();
        }
        5 | 6 | 7 => {
            loop {
                let mut i_0: I = 0 as libc::c_int as I;
                x = ex1(
                    v,
                    0 as K,
                    &mut i_0,
                    0 as libc::c_int as I,
                    1 as libc::c_int as I,
                );
                if x.is_null() {
                    return 0 as K;
                }
                if fer > 0 as libc::c_int as libc::c_longlong {
                    return x;
                }
                x = if bk(x as V) != 0 { _n() } else { x };
                z = delist(x);
                if (if (*z).t < 0 as libc::c_int as libc::c_longlong {
                    -(*z).t
                } else {
                    (*z).t
                }) != 1 as libc::c_int as libc::c_longlong
                    || (*z).n != 1 as libc::c_int as libc::c_longlong
                {
                    cd(z);
                    return kerr(b"type\0" as *const u8 as *const libc::c_char);
                }
                a = *(((*z).k).as_mut_ptr() as *mut I);
                cd(z);
                i_0 = 0 as libc::c_int as I;
                if b != 0 {
                    loop {
                        i_0 += 1;
                        if !(i_0 < n && bk(*v.offset(i_0 as isize)) == 0) {
                            break;
                        }
                    }
                    if i_0 >= n {
                        break;
                    }
                }
                match r {
                    5 | 6 => {
                        if a != 0 && b != 0 {
                            x = ex0(
                                v.offset(i_0 as isize).offset(1 as libc::c_int as isize),
                                0 as K,
                                0 as libc::c_int as I,
                            );
                            if fer > 0 as libc::c_int as libc::c_longlong {
                                return x;
                            }
                            cd(x);
                        }
                    }
                    7 => {
                        let mut j: I = 0 as libc::c_int as I;
                        let mut _j: I = a;
                        while j < _j {
                            x = ex0(
                                v.offset(i_0 as isize).offset(1 as libc::c_int as isize),
                                0 as K,
                                0 as libc::c_int as I,
                            );
                            if fer > 0 as libc::c_int as libc::c_longlong {
                                return x;
                            }
                            cd(x);
                            j += 1;
                            j;
                        }
                    }
                    _ => {}
                }
                if !(6 as libc::c_int as libc::c_longlong == r && a != 0) {
                    break;
                }
            }
            return _n();
        }
        _ => {
            z = newK(
                0 as libc::c_int as I,
                if n != 0 { e } else { 0 as libc::c_int as libc::c_longlong },
            );
            if n != 0 {
                i = n - 1 as libc::c_int as libc::c_longlong;
                while i >= -(1 as libc::c_int) as libc::c_longlong {
                    if -(1 as libc::c_int) as libc::c_longlong == i
                        || bk(*v.offset(i as isize)) != 0
                    {
                        if offsetColon
                            == *v
                                .offset(1 as libc::c_int as isize)
                                .offset(i as isize)
                                .offset(0 as libc::c_int as isize)
                            && *v
                                .offset(1 as libc::c_int as isize)
                                .offset(i as isize)
                                .offset(1 as libc::c_int as isize) as UI
                                > DT_SIZE as libc::c_ulonglong
                        {
                            fer = 1 as libc::c_int as I;
                        }
                        x = ex1(
                            v.offset(1 as libc::c_int as isize).offset(i as isize),
                            0 as K,
                            &mut i,
                            n,
                            0 as libc::c_int as I,
                        );
                        if fer1 != 0
                            || fer > 0 as libc::c_int as libc::c_longlong
                                && (*v.offset(0 as libc::c_int as isize) == offsetColon
                                    || *v.offset(2 as libc::c_int as isize)
                                        == 1 as libc::c_int as V) && fCheck == 0
                        {
                            cd(z);
                            fer1 = 1 as libc::c_int as I;
                            return x;
                        }
                        if OOM_CD(0 as libc::c_int as I, x, z, -(1 as libc::c_int) as V)
                            == 0
                        {
                            return 0 as K;
                        }
                        e -= 1;
                        let ref mut fresh49 = *((*z).k).as_mut_ptr().offset(e as isize);
                        *fresh49 = if bk(x as V) != 0 {
                            if 2 as libc::c_int as libc::c_longlong == r {
                                0 as K
                            } else {
                                _n()
                            }
                        } else {
                            x
                        };
                    }
                    i -= 1;
                    i;
                }
            }
        }
    }
    if 1 as libc::c_int as libc::c_longlong == r
        || *v.offset(0 as libc::c_int as isize) == 0x7d as libc::c_int as V
    {
        z = collapse(z);
    }
    if !k.is_null() {
        let mut j_0: I = valence(&mut z as *mut K as V);
        if j_0 == 0 && 0 as libc::c_int as libc::c_longlong == (*k).t {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i: I = (*k).n;
            while i_1 < _i {
                if (*((*k).k).as_mut_ptr().offset(i_1 as isize)).is_null() {
                    let ref mut fresh50 = *((*k).k).as_mut_ptr().offset(i_1 as isize);
                    *fresh50 = _n();
                }
                i_1 += 1;
                i_1;
            }
        }
        if (*z).t != 7 as libc::c_int as libc::c_longlong
            || (*z).n != 1 as libc::c_int as libc::c_longlong
            || j_0 < (*k).n
                && !(0 as libc::c_int as libc::c_longlong == j_0
                    && (*k).n == 1 as libc::c_int as libc::c_longlong)
        {
            if !encf.is_null() && (DT_SIZE as libc::c_ulonglong) < &mut z as *mut K as UI
            {
                x = vf_ex(&mut encf as *mut K as V, k);
            } else {
                x = vf_ex(&mut z as *mut K as V, k);
            }
            if encp != 3 as libc::c_int as libc::c_longlong {
                cd(z);
            }
            z = x;
            return z;
        } else {
            let mut p: K = *(((*z).k).as_mut_ptr() as *mut V)
                .offset(CODE as libc::c_int as isize) as K;
            let mut i_2: I = (*p).n - 2 as libc::c_int as libc::c_longlong;
            let mut q: *mut V = (((*p).k).as_mut_ptr() as *mut V).offset(i_2 as isize);
            let mut t: K = 0 as K;
            if (*k).n == 1 as libc::c_int as libc::c_longlong {
                t = first(k);
            }
            if ((*k).n > 1 as libc::c_int as libc::c_longlong
                || !t.is_null() && (*t).n == 1 as libc::c_int as libc::c_longlong)
                && sva(*q) == 0 && adverbClass(*q) != 0
            {
                if (*k).n == 1 as libc::c_int as libc::c_longlong && prj2 == 0 {
                    (*k).n = 2 as libc::c_int as I;
                }
                prj2 = 1 as libc::c_int as I;
                let mut i_3: I = 0 as libc::c_int as I;
                let mut _i_0: I = (*k).n;
                while i_3 < _i_0 {
                    if (*((*k).k).as_mut_ptr().offset(i_3 as isize)).is_null() {
                        prj = 1 as libc::c_int as I;
                    }
                    i_3 += 1;
                    i_3;
                }
                if prj == 0 {
                    x = bv_ex(q, k);
                    cd(z);
                    return x;
                }
            }
            cd(t);
            if (*z).t == 7 as libc::c_int as libc::c_longlong
                && (*z).n == 1 as libc::c_int as libc::c_longlong
                && *((*(*((*z).k).as_mut_ptr().offset(CODE as libc::c_int as isize)
                    as K))
                    .k)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) == offsetSSR as *mut k0
                && (*k).t == 0 as libc::c_int as libc::c_longlong
                && (*k).n == 3 as libc::c_int as libc::c_longlong
                && (if (**((*k).k).as_mut_ptr().offset(2 as libc::c_int as isize)).t
                    < 0 as libc::c_int as libc::c_longlong
                {
                    -(**((*k).k).as_mut_ptr().offset(2 as libc::c_int as isize)).t
                } else {
                    (**((*k).k).as_mut_ptr().offset(2 as libc::c_int as isize)).t
                }) == -(3 as libc::c_int) as libc::c_longlong
            {
                let ref mut fresh51 = *((*k).k)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize);
                *fresh51 = enlist(
                    *((*k).k).as_mut_ptr().offset(2 as libc::c_int as isize),
                );
                cd(x);
            }
            x = vf_ex(&mut z as *mut K as V, k);
            cd(z);
            z = x;
        }
    }
    return z;
}
unsafe extern "C" fn bv_ex(mut p: *mut V, mut k: K) -> K {
    let mut q: V = *p;
    let mut x: K = 0 as *mut k0;
    let mut n: I = 0 as libc::c_int as I;
    if adverbClass(*p) == 0 && valence(*p) < 3 as libc::c_int as libc::c_longlong {
        if (*k).n < 2 as libc::c_int as libc::c_longlong {
            return kerr(b"valence\0" as *const u8 as *const libc::c_char);
        }
        return dv_ex(
            *((*k).k).as_mut_ptr().offset(0 as libc::c_int as isize),
            p,
            *((*k).k).as_mut_ptr().offset(1 as libc::c_int as isize),
        );
    }
    if offsetOver == q as L {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = (*k).n - 1 as libc::c_int as libc::c_longlong;
        while i < _i {
            x = *((*k).k)
                .as_mut_ptr()
                .offset((i + 1 as libc::c_int as libc::c_longlong) as isize);
            if (*x).n == 0 {
                return ci(*((*k).k).as_mut_ptr());
            }
            if atomI(x) == 0 {
                if n != 0 && n != (*x).n {
                    return kerr(b"length\0" as *const u8 as *const libc::c_char)
                } else {
                    n = (*x).n;
                }
            }
            i += 1;
            i;
        }
        n = if 1 as libc::c_int as libc::c_longlong > n {
            1 as libc::c_int as libc::c_longlong
        } else {
            n
        };
        let mut z: K = ci(*((*k).k).as_mut_ptr());
        let mut g: K = newK(0 as libc::c_int as I, (*k).n);
        if OOM_CD(0 as libc::c_int as I, z, g, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = n;
        while i_0 < _i_0 {
            let ref mut fresh52 = *((*g).k).as_mut_ptr();
            *fresh52 = z;
            let mut j: I = 0 as libc::c_int as I;
            let mut _j: I = (*g).n - 1 as libc::c_int as libc::c_longlong;
            while j < _j {
                x = itemAtIndex(
                    *((*k).k)
                        .as_mut_ptr()
                        .offset((j + 1 as libc::c_int as libc::c_longlong) as isize),
                    i_0,
                );
                if OOM_CD(0 as libc::c_int as I, x, g, -(1 as libc::c_int) as V) == 0 {
                    return 0 as K;
                }
                let ref mut fresh53 = *((*g).k)
                    .as_mut_ptr()
                    .offset((j + 1 as libc::c_int as libc::c_longlong) as isize);
                *fresh53 = x;
                j += 1;
                j;
            }
            x = bv_ex(p.offset(-(1 as libc::c_int as isize)), g);
            if OOM_CD(0 as libc::c_int as I, x, g, -(1 as libc::c_int) as V) == 0 {
                return 0 as K;
            }
            let mut j_0: I = 0 as libc::c_int as I;
            let mut _j_0: I = (*g).n;
            while j_0 < _j_0 {
                cd(*((*g).k).as_mut_ptr().offset(j_0 as isize));
                let ref mut fresh54 = *((*g).k).as_mut_ptr().offset(j_0 as isize);
                *fresh54 = 0 as *mut k0;
                j_0 += 1;
                j_0;
            }
            z = x;
            i_0 += 1;
            i_0;
        }
        cd(g);
        return z;
    }
    if offsetScan == q as L {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = (*k).n - 1 as libc::c_int as libc::c_longlong;
        while i_1 < _i_1 {
            x = *((*k).k)
                .as_mut_ptr()
                .offset((i_1 + 1 as libc::c_int as libc::c_longlong) as isize);
            if !x.is_null() {
                if (*x).n == 0 {
                    return ci(*((*k).k).as_mut_ptr());
                }
                if atomI(x) == 0 {
                    if n != 0 && n != (*x).n {
                        return kerr(b"length\0" as *const u8 as *const libc::c_char)
                    } else {
                        n = (*x).n;
                    }
                }
            }
            i_1 += 1;
            i_1;
        }
        if n == 0 {
            return bv_ex(p.offset(-(1 as libc::c_int as isize)), k);
        }
        n = if 1 as libc::c_int as libc::c_longlong > n {
            1 as libc::c_int as libc::c_longlong
        } else {
            n
        };
        let mut z_0: K = newK(0 as libc::c_int as I, 1 as libc::c_int as I);
        let mut g_0: K = newK(0 as libc::c_int as I, (*k).n);
        if OOM_CD(0 as libc::c_int as I, z_0, g_0, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        let ref mut fresh55 = *((*z_0).k).as_mut_ptr().offset(0 as libc::c_int as isize);
        *fresh55 = ci(*((*k).k).as_mut_ptr());
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = n;
        while i_2 < _i_2 {
            let ref mut fresh56 = *((*g_0).k).as_mut_ptr();
            *fresh56 = ci(
                *((*z_0).k)
                    .as_mut_ptr()
                    .offset(((*z_0).n - 1 as libc::c_int as libc::c_longlong) as isize),
            );
            let mut j_1: I = 0 as libc::c_int as I;
            let mut _j_1: I = (*g_0).n - 1 as libc::c_int as libc::c_longlong;
            while j_1 < _j_1 {
                x = itemAtIndex(
                    *((*k).k)
                        .as_mut_ptr()
                        .offset((j_1 + 1 as libc::c_int as libc::c_longlong) as isize),
                    i_2,
                );
                if OOM_CD(0 as libc::c_int as I, x, z_0, g_0, -(1 as libc::c_int) as V)
                    == 0
                {
                    return 0 as K;
                }
                let ref mut fresh57 = *((*g_0).k)
                    .as_mut_ptr()
                    .offset((j_1 + 1 as libc::c_int as libc::c_longlong) as isize);
                *fresh57 = x;
                j_1 += 1;
                j_1;
            }
            x = bv_ex(p.offset(-(1 as libc::c_int as isize)), g_0);
            if OOM_CD(0 as libc::c_int as I, x, z_0, g_0, -(1 as libc::c_int) as V) == 0
            {
                return 0 as K;
            }
            let mut j_2: I = 0 as libc::c_int as I;
            let mut _j_2: I = (*g_0).n;
            while j_2 < _j_2 {
                cd(*((*g_0).k).as_mut_ptr().offset(j_2 as isize));
                let ref mut fresh58 = *((*g_0).k).as_mut_ptr().offset(j_2 as isize);
                *fresh58 = 0 as *mut k0;
                j_2 += 1;
                j_2;
            }
            kap(&mut z_0, &mut x as *mut K as V);
            cd(x);
            i_2 += 1;
            i_2;
        }
        cd(g_0);
        z_0 = collapse(z_0);
        return z_0;
    }
    if offsetEach == q as L {
        let mut i_3: I = 0 as libc::c_int as I;
        let mut _i_3: I = (*k).n;
        while i_3 < _i_3 {
            x = *((*k).k).as_mut_ptr().offset(i_3 as isize);
            if !x.is_null() {
                if (*x).n == 0 {
                    return newK(0 as libc::c_int as I, 0 as libc::c_int as I);
                }
                if atomI(x) == 0 {
                    if n != 0 && n != (*x).n {
                        return kerr(b"length\0" as *const u8 as *const libc::c_char)
                    } else {
                        n = (*x).n;
                    }
                }
            }
            i_3 += 1;
            i_3;
        }
        let mut c: I = (n == 0) as libc::c_int as I;
        n = if 1 as libc::c_int as libc::c_longlong > n {
            1 as libc::c_int as libc::c_longlong
        } else {
            n
        };
        let mut z_1: K = newK(0 as libc::c_int as I, n);
        let mut g_1: K = newK(0 as libc::c_int as I, (*k).n);
        if OOM_CD(0 as libc::c_int as I, g_1, z_1, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        let mut i_4: I = 0 as libc::c_int as I;
        let mut _i_4: I = n;
        while i_4 < _i_4 {
            let mut x_0: K = 0 as *mut k0;
            let mut j_3: I = 0 as libc::c_int as I;
            let mut _j_3: I = (*k).n;
            while j_3 < _j_3 {
                x_0 = itemAtIndex(*((*k).k).as_mut_ptr().offset(j_3 as isize), i_4);
                if OOM_CD(0 as libc::c_int as I, x_0, g_1, z_1, -(1 as libc::c_int) as V)
                    == 0
                {
                    return 0 as K;
                }
                let ref mut fresh59 = *((*g_1).k).as_mut_ptr().offset(j_3 as isize);
                *fresh59 = x_0;
                j_3 += 1;
                j_3;
            }
            x_0 = bv_ex(p.offset(-(1 as libc::c_int as isize)), g_1);
            if OOM_CD(0 as libc::c_int as I, x_0, z_1, g_1, -(1 as libc::c_int) as V)
                == 0
            {
                return 0 as K;
            }
            let ref mut fresh60 = *((*z_1).k).as_mut_ptr().offset(i_4 as isize);
            *fresh60 = x_0;
            let mut j_4: I = 0 as libc::c_int as I;
            let mut _j_4: I = (*k).n;
            while j_4 < _j_4 {
                cd(*((*g_1).k).as_mut_ptr().offset(j_4 as isize));
                let ref mut fresh61 = *((*g_1).k).as_mut_ptr().offset(j_4 as isize);
                *fresh61 = 0 as *mut k0;
                j_4 += 1;
                j_4;
            }
            i_4 += 1;
            i_4;
        }
        cd(g_1);
        if c != 0 {
            z_1 = collapse(z_1);
        } else {
            z_1 = demote(z_1);
        }
        return z_1;
    }
    if offsetEachright == q as L {
        if (*k).n != 2 as libc::c_int as libc::c_longlong {
            return kerr(b"valence\0" as *const u8 as *const libc::c_char);
        }
        let mut a: K = *((*k).k).as_mut_ptr().offset(0 as libc::c_int as isize);
        let mut b: K = *((*k).k).as_mut_ptr().offset(1 as libc::c_int as isize);
        return eachright2(a, p, b);
    }
    if offsetEachleft == q as L {
        if (*k).n != 2 as libc::c_int as libc::c_longlong {
            return kerr(b"valence\0" as *const u8 as *const libc::c_char);
        }
        let mut a_0: K = *((*k).k).as_mut_ptr().offset(0 as libc::c_int as isize);
        let mut b_0: K = *((*k).k).as_mut_ptr().offset(1 as libc::c_int as isize);
        return eachleft2(a_0, p, b_0);
    }
    if offsetEachpair == q as L {
        if (*k).n != 2 as libc::c_int as libc::c_longlong {
            return kerr(b"valence\0" as *const u8 as *const libc::c_char);
        }
        let mut a_1: K = *((*k).k).as_mut_ptr().offset(0 as libc::c_int as isize);
        let mut b_1: K = *((*k).k).as_mut_ptr().offset(1 as libc::c_int as isize);
        return eachpair2(a_1, p, b_1);
    }
    return vf_ex(*p, k);
}
pub unsafe extern "C" fn ex1(
    mut w: *mut V,
    mut k: K,
    mut i: *mut I,
    mut n: I,
    mut f: I,
) -> K {
    if offsetColon == *w.offset(0 as libc::c_int as isize)
        && *w.offset(1 as libc::c_int as isize) as UI > DT_SIZE as libc::c_ulonglong
        && *w.offset(2 as libc::c_int as isize) as UI > DT_SIZE as libc::c_ulonglong
        && fwh == 0 as libc::c_int as libc::c_longlong
    {
        fer = 1 as libc::c_int as I;
        if f != 0 {
            *i = n;
        } else {
            *i = -(1 as libc::c_int) as I;
        }
        let mut tmp: K = *(*w.offset(1 as libc::c_int as isize) as *mut K);
        return ci(tmp);
    }
    if DT_ADVERB_OFFSET <= *w as L && (*w as L) < DT_VERB_OFFSET
        && offsetScan != *w.offset(1 as libc::c_int as isize) as L
        && !(*w.offset(1 as libc::c_int as isize)).is_null()
    {
        if offsetScan == *w as L {
            if 0 as libc::c_int
                == strcmp(
                    fBreak as *const libc::c_char,
                    b"n\0" as *const u8 as *const libc::c_char,
                )
            {
                return ex2(w.offset(1 as libc::c_int as isize), k);
            }
            if 0 as libc::c_int
                == strcmp(
                    fBreak as *const libc::c_char,
                    b"t\0" as *const u8 as *const libc::c_char,
                )
            {
                return ex2(w.offset(1 as libc::c_int as isize), k);
            }
            if 0 as libc::c_int
                == strcmp(
                    fBreak as *const libc::c_char,
                    b"s\0" as *const u8 as *const libc::c_char,
                )
            {
                fer = 1 as libc::c_int as I;
                return ex2(w.offset(1 as libc::c_int as isize), k);
            }
        } else if offsetEach as V == *w {
            if 3 as libc::c_int as libc::c_longlong
                == (if (**(*w.offset(1 as libc::c_int as isize) as *mut K)).t
                    < 0 as libc::c_int as libc::c_longlong
                {
                    -(**(*w.offset(1 as libc::c_int as isize) as *mut K)).t
                } else {
                    (**(*w.offset(1 as libc::c_int as isize) as *mut K)).t
                })
            {
                return ci(*(*w.offset(1 as libc::c_int as isize) as *mut K))
            } else {
                return _n()
            }
        } else {
            return kerr(b"nyi\0" as *const u8 as *const libc::c_char)
        }
    }
    let mut c: I = 0 as libc::c_int as I;
    while !(*w.offset(c as isize)).is_null() && bk(*w.offset(c as isize)) == 0 {
        c += 1;
        c;
        if offsetColon == *w.offset((c - 1 as libc::c_int as libc::c_longlong) as isize)
        {
            break;
        }
    }
    if c == 0 || VA(*w.offset((c - 1 as libc::c_int as libc::c_longlong) as isize)) == 0
        || c > 1 as libc::c_int as libc::c_longlong
            && offsetColon
                == *w.offset((c - 1 as libc::c_int as libc::c_longlong) as isize)
    {
        return ex2(w, k);
    }
    if *w.offset(0 as libc::c_int as isize) == offsetColon
        && *w.offset(1 as libc::c_int as isize) as UI > DT_SIZE as libc::c_ulonglong
    {
        let mut d: I = 0 as libc::c_int as I;
        while !(*w.offset(d as isize)).is_null() && bk(*w.offset(d as isize)) == 0 {
            d += 1;
            d;
        }
        let mut a: K = Kv();
        (*a).n = 0 as libc::c_int as I;
        let mut kb: K = newK(-(4 as libc::c_int) as I, d);
        if OOM_CD(0 as libc::c_int as I, a, kb, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        let mut b: *mut V = ((*kb).k).as_mut_ptr() as *mut V;
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i: I = d - 1 as libc::c_int as libc::c_longlong;
        while i_0 < _i {
            let ref mut fresh62 = *b.offset(i_0 as isize);
            *fresh62 = *w.offset((i_0 + 1 as libc::c_int as libc::c_longlong) as isize);
            i_0 += 1;
            i_0;
        }
        let ref mut fresh63 = *b
            .offset((d - 1 as libc::c_int as libc::c_longlong) as isize);
        *fresh63 = 0 as V;
        let ref mut fresh64 = *(((*a).k).as_mut_ptr() as *mut V)
            .offset(CODE as libc::c_int as isize);
        *fresh64 = kb as V;
        let mut x: V = ex_(&mut a as *mut K as V, 0 as libc::c_int as I);
        cd(a);
        if *w.offset(-(1 as libc::c_int) as isize) != offsetColon {
            fer = 1 as libc::c_int as I;
        }
        return x as K;
    }
    let mut a_0: K = Kv();
    let mut kb_0: K = newK(
        -(4 as libc::c_int) as I,
        1 as libc::c_int as libc::c_longlong + c,
    );
    if OOM_CD(0 as libc::c_int as I, a_0, kb_0, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut b_0: *mut V = ((*kb_0).k).as_mut_ptr() as *mut V;
    let ref mut fresh65 = *b_0.offset(c as isize);
    *fresh65 = 0 as V;
    let mut i_1: I = 0 as libc::c_int as I;
    let mut _i_0: I = c;
    while i_1 < _i_0 {
        let mut j: I = c - i_1 - 1 as libc::c_int as libc::c_longlong;
        let ref mut fresh66 = *b_0.offset(j as isize);
        *fresh66 = *w.offset(j as isize);
        if !(VA(*b_0.offset(j as isize)) != 0) {
            let mut r: K = ex_(*w.offset(j as isize), 1 as libc::c_int as I) as K;
            let mut q: V = newE(LS, r) as V;
            kap(
                (((*a_0).k).as_mut_ptr() as *mut V as *mut K)
                    .offset(LOCALS as libc::c_int as isize),
                &mut q as *mut V as V,
            );
            cd(q as K);
            q = EVP(q as K) as V;
            let ref mut fresh67 = *b_0.offset(j as isize);
            *fresh67 = q;
        }
        i_1 += 1;
        i_1;
    }
    let ref mut fresh68 = *(((*a_0).k).as_mut_ptr() as *mut V)
        .offset(CODE as libc::c_int as isize);
    *fresh68 = kb_0 as V;
    if fll > 0 as libc::c_int as libc::c_longlong
        && 2 as libc::c_int as libc::c_longlong == (*kb_0).n
        && kdefClass(
            *(((*kb_0).k).as_mut_ptr() as *mut V).offset(0 as libc::c_int as isize) as I,
        ) != 0
    {
        let mut z: K = kdef(
            *(((*kb_0).k).as_mut_ptr() as *mut V).offset(0 as libc::c_int as isize) as I,
        );
        cd(a_0);
        return z;
    }
    return a_0;
}
unsafe extern "C" fn ex2(mut v: *mut V, mut k: K) -> K {
    let mut t0: K = 0 as *mut k0;
    let mut t2: K = 0 as *mut k0;
    let mut t3: K = 0 as *mut k0;
    let mut e: K = 0 as *mut k0;
    let mut u: K = 0 as *mut k0;
    let mut i: I = 0 as libc::c_int as I;
    ft3 = 0 as libc::c_int as I;
    if v.is_null() || (*v).is_null() {
        return if !k.is_null() {
            if 1 as libc::c_int as libc::c_longlong == (*k).n {
                ci(*((*k).k).as_mut_ptr().offset(0 as libc::c_int as isize))
            } else {
                ci(k)
            }
        } else {
            DT_END_OFFSET as K
        };
    }
    if bk(*v) != 0 {
        return *v as K;
    }
    if (*v.offset(1 as libc::c_int as isize)).is_null() && k.is_null() {
        let mut z: K = ex_(*v, 1 as libc::c_int as I) as K;
        if z > DT_SIZE as K && (*z).t == 7 as libc::c_int as libc::c_longlong
            && (*z).n == 3 as libc::c_int as libc::c_longlong
        {
            if !prnt.is_null()
                && !(*(((*z).k).as_mut_ptr() as *mut V)
                    .offset(PARAMS as libc::c_int as isize))
                    .is_null()
                && !(*(((*prnt).k).as_mut_ptr() as *mut V)
                    .offset(CACHE_TREE as libc::c_int as isize))
                    .is_null()
                && (*(((*z).k).as_mut_ptr() as *mut V)
                    .offset(CACHE_TREE as libc::c_int as isize))
                    .is_null()
                && (**((*z).k).as_mut_ptr().offset(LOCALS as libc::c_int as isize)).n
                    == 0
            {
                let mut j0: K = dot_monadic(
                    *(((*z).k).as_mut_ptr() as *mut V)
                        .offset(PARAMS as libc::c_int as isize) as K,
                );
                let mut j1: K = dot_monadic(
                    *(((*prnt).k).as_mut_ptr() as *mut V)
                        .offset(CACHE_TREE as libc::c_int as isize) as K,
                );
                let mut j2: K = join(ci(j0), j1);
                cd(j0);
                if encp == 0 as libc::c_int as libc::c_longlong {
                    let ref mut fresh69 = *(((*z).k).as_mut_ptr() as *mut V)
                        .offset(CACHE_TREE as libc::c_int as isize);
                    *fresh69 = dot_monadic(j2) as V;
                }
                if encp == 1 as libc::c_int as libc::c_longlong {
                    let ref mut fresh70 = *(((*z).k).as_mut_ptr() as *mut V)
                        .offset(CACHE_TREE as libc::c_int as isize);
                    *fresh70 = dot_monadic(j1) as V;
                }
                cd(j0);
                cd(j1);
                cd(j2);
                cd(*((*prnt).k).as_mut_ptr().offset(CACHE_WD as libc::c_int as isize));
                let ref mut fresh71 = *(((*prnt).k).as_mut_ptr() as *mut V)
                    .offset(CACHE_WD as libc::c_int as isize);
                *fresh71 = 0 as V;
            }
            if !prnt.is_null()
                && !(*(((*prnt).k).as_mut_ptr() as *mut V)
                    .offset(CODE as libc::c_int as isize))
                    .is_null()
                && (**((*prnt).k).as_mut_ptr().offset(CODE as libc::c_int as isize)).t
                    == -(3 as libc::c_int) as libc::c_longlong
                && *(((*(*((*prnt).k).as_mut_ptr().offset(CODE as libc::c_int as isize)
                    as K))
                    .k)
                    .as_mut_ptr() as *mut C)
                    .offset(0 as libc::c_int as isize) as libc::c_int == '{' as i32
                && *(((*(*((*prnt).k).as_mut_ptr().offset(CODE as libc::c_int as isize)
                    as K))
                    .k)
                    .as_mut_ptr() as *mut C)
                    .offset(
                        ((**((*prnt).k)
                            .as_mut_ptr()
                            .offset(CODE as libc::c_int as isize))
                            .n - 1 as libc::c_int as libc::c_longlong) as isize,
                    ) as libc::c_int
                    == (*::std::mem::transmute::<
                        &[u8; 2],
                        &[libc::c_char; 2],
                    >(b"}\0"))[0 as libc::c_int as usize] as libc::c_int
                && !(strchr(
                    ((*(*((*prnt).k).as_mut_ptr().offset(CODE as libc::c_int as isize)
                        as K))
                        .k)
                        .as_mut_ptr() as *mut C,
                    (*::std::mem::transmute::<
                        &[u8; 2],
                        &[libc::c_char; 2],
                    >(b"y\0"))[0 as libc::c_int as usize] as libc::c_int,
                ))
                    .is_null()
            {
                if !encf.is_null() {
                    cd(encf);
                }
                encf = ci(prnt);
            }
            if encp != 2 as libc::c_int as libc::c_longlong || prnt.is_null() {
                if !prnt.is_null() {
                    if !grnt.is_null() {
                        cd(grnt);
                    }
                    grnt = prnt;
                }
                prnt = ci(z);
            } else {
                cd(z);
                return prnt;
            }
        }
        return z;
    }
    if (*v.offset(1 as libc::c_int as isize)).is_null() && sva(*v) != 0 {
        return vf_ex(*v, k);
    }
    if bk(*v.offset(1 as libc::c_int as isize)) != 0 {
        let mut z_0: K = ex_(*v, 1 as libc::c_int as I) as K;
        if fer == 2 as libc::c_int as libc::c_longlong && fCheck == 0 {
            return 0 as K;
        }
        if !prnt.is_null() && !z_0.is_null()
            && (*z_0).t == 7 as libc::c_int as libc::c_longlong
        {
            if !(*(((*prnt).k).as_mut_ptr() as *mut V)
                .offset(PARAMS as libc::c_int as isize))
                .is_null()
                && (**((*prnt).k).as_mut_ptr().offset(PARAMS as libc::c_int as isize)).n
                    == 0
                && !(*(((*z_0).k).as_mut_ptr() as *mut V)
                    .offset(LOCALS as libc::c_int as isize))
                    .is_null()
                && (**((*z_0).k).as_mut_ptr().offset(LOCALS as libc::c_int as isize)).n
                    == 0
                && !(*(((*prnt).k).as_mut_ptr() as *mut V)
                    .offset(LOCALS as libc::c_int as isize))
                    .is_null()
                && (**((*prnt).k).as_mut_ptr().offset(LOCALS as libc::c_int as isize)).n
                    != 0
            {
                let ref mut fresh72 = *(((*z_0).k).as_mut_ptr() as *mut V)
                    .offset(CACHE_TREE as libc::c_int as isize);
                *fresh72 = kcloneI(
                    *((*prnt).k).as_mut_ptr().offset(CACHE_TREE as libc::c_int as isize),
                    b"src/kx.c\0" as *const u8 as *const libc::c_char,
                    847 as libc::c_int,
                ) as V;
                if !prnt.is_null() {
                    cd(prnt);
                }
                prnt = ci(z_0);
            } else if !(*(((*prnt).k).as_mut_ptr() as *mut V)
                .offset(LOCALS as libc::c_int as isize))
                .is_null()
                && (**((*prnt).k).as_mut_ptr().offset(LOCALS as libc::c_int as isize)).n
                    != 0
                && !(*(((*z_0).k).as_mut_ptr() as *mut V)
                    .offset(PARAMS as libc::c_int as isize))
                    .is_null()
                && (**((*z_0).k).as_mut_ptr().offset(PARAMS as libc::c_int as isize)).n
                    != 0
            {
                if !(*(((*prnt).k).as_mut_ptr() as *mut V)
                    .offset(CACHE_TREE as libc::c_int as isize))
                    .is_null()
                {
                    let mut j0_0: K = dot_monadic(
                        *(((*z_0).k).as_mut_ptr() as *mut V)
                            .offset(PARAMS as libc::c_int as isize) as K,
                    );
                    let mut j1_0: K = dot_monadic(
                        *(((*prnt).k).as_mut_ptr() as *mut V)
                            .offset(CACHE_TREE as libc::c_int as isize) as K,
                    );
                    let mut j2_0: K = join(ci(j0_0), j1_0);
                    cd(j0_0);
                    let ref mut fresh73 = *(((*z_0).k).as_mut_ptr() as *mut V)
                        .offset(CACHE_TREE as libc::c_int as isize);
                    *fresh73 = dot_monadic(j2_0) as V;
                    cd(j0_0);
                    cd(j1_0);
                    cd(j2_0);
                } else {
                    let ref mut fresh74 = *(((*z_0).k).as_mut_ptr() as *mut V)
                        .offset(CACHE_TREE as libc::c_int as isize);
                    *fresh74 = kcloneI(
                        *(((*z_0).k).as_mut_ptr() as *mut V)
                            .offset(PARAMS as libc::c_int as isize) as K,
                        b"src/kx.c\0" as *const u8 as *const libc::c_char,
                        852 as libc::c_int,
                    ) as V;
                }
            }
        }
        return z_0;
    }
    if VA(*v) == 0
        && (offsetColon == *v.offset(1 as libc::c_int as isize)
            || VA(*v.offset(1 as libc::c_int as isize)) != 0
                && offsetColon == *v.offset(2 as libc::c_int as isize))
    {
        if adverbClass(*v.offset(1 as libc::c_int as isize)) != 0 {
            return kerr(b"syntax\0" as *const u8 as *const libc::c_char);
        }
        let mut a: K = 0 as K;
        let mut b: K = 0 as K;
        let mut c: K = 0 as K;
        let mut d: K = 0 as K;
        let mut p: K = 0 as K;
        let mut w: *mut K = *v as *mut K;
        a = *w;
        if a.is_null() {
            return 0 as K;
        }
        if 7 as libc::c_int as libc::c_longlong == (*a).t
            && 0 as libc::c_int as libc::c_longlong == (*a).n
            && {
                b = *(((*a).k).as_mut_ptr() as *mut V)
                    .offset(CONJ as libc::c_int as isize) as K;
                !b.is_null()
            } && 7 as libc::c_int as libc::c_longlong == (*b).t
            && 0 as libc::c_int as libc::c_longlong == (*b).n
        {
            b = ex_(
                (((*a).k).as_mut_ptr() as *mut V).offset(CONJ as libc::c_int as isize)
                    as V,
                (if *(((*(*(((*b).k).as_mut_ptr() as *mut V)
                    .offset(CODE as libc::c_int as isize) as K))
                    .k)
                    .as_mut_ptr() as *mut S as *mut V) as L
                    == 1 as libc::c_int as libc::c_longlong
                    || *(((*(*(((*b).k).as_mut_ptr() as *mut V)
                        .offset(CODE as libc::c_int as isize) as K))
                        .k)
                        .as_mut_ptr() as *mut S as *mut V)
                        .offset(1 as libc::c_int as isize) as L
                        == 1 as libc::c_int as libc::c_longlong
                {
                    1 as libc::c_int
                } else {
                    2 as libc::c_int
                }) as I,
            ) as K;
            if b.is_null() {
                return 0 as K;
            }
            w = *(((*(*(((*a).k).as_mut_ptr() as *mut V)
                .offset(CODE as libc::c_int as isize) as K))
                .k)
                .as_mut_ptr() as *mut S as *mut V) as *mut K;
            if (*b).t == 0 as libc::c_int as libc::c_longlong
                && (*b).n == 0 as libc::c_int as libc::c_longlong
            {
                if 1e6f64 < w as UI as libc::c_double {
                    let mut r: K = *w;
                    if (*r).t == 5 as libc::c_int as libc::c_longlong {
                        p = enumerate(r);
                        cd(b);
                        b = enlist(p);
                        cd(p);
                    }
                }
            }
        }
        if b.is_null() {
            b = newK(0 as libc::c_int as I, 0 as libc::c_int as I);
            if b.is_null() {
                return 0 as K;
            }
        }
        c = Kv();
        let mut kc: K = newK(-(4 as libc::c_int) as I, 2 as libc::c_int as I);
        if OOM_CD(0 as libc::c_int as I, b, c, kc, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        let ref mut fresh75 = *(((*c).k).as_mut_ptr() as *mut V)
            .offset(CODE as libc::c_int as isize);
        *fresh75 = kc as V;
        let ref mut fresh76 = *(((*(*(((*c).k).as_mut_ptr() as *mut V)
            .offset(CODE as libc::c_int as isize) as K))
            .k)
            .as_mut_ptr() as *mut S as *mut V);
        *fresh76 = *v.offset(1 as libc::c_int as isize);
        if 1 as libc::c_int as libc::c_longlong
            != sva(*v.offset(1 as libc::c_int as isize))
        {
            d = ex1(
                v
                    .offset(
                        (if offsetColon == *v.offset(1 as libc::c_int as isize) {
                            2 as libc::c_int
                        } else {
                            3 as libc::c_int
                        }) as isize,
                    ),
                k,
                0 as *mut I,
                0 as libc::c_int as I,
                1 as libc::c_int as I,
            );
        }
        d = if bk(d as V) != 0 { 0 as K } else { d };
        if fer > 0 as libc::c_int as libc::c_longlong {
            cd(c);
            cd(d);
            cd(b);
            return _n();
        }
        if (*w).is_null() {
            return kerr(b"parse\0" as *const u8 as *const libc::c_char);
        }
        if cirRef(*w, d) != 0
            || (**w).t == 6 as libc::c_int as libc::c_longlong && !d.is_null()
                && ((*d).t == 0 as libc::c_int as libc::c_longlong
                    || (*d).t == 5 as libc::c_int as libc::c_longlong
                    || (if (*d).t < 0 as libc::c_int as libc::c_longlong {
                        -(*d).t
                    } else {
                        (*d).t
                    }) != (*d).t)
        {
            let mut x: K = d;
            if rc(x) != 0 {
                d = kcloneI(
                    x,
                    b"src/kx.c\0" as *const u8 as *const libc::c_char,
                    871 as libc::c_int,
                );
                cd(x);
            }
        } else if (**w).t != 6 as libc::c_int as libc::c_longlong {
            let mut x_0: K = *w;
            if rc(x_0) > 1 as libc::c_int as libc::c_longlong {
                *w = kcloneI(
                    x_0,
                    b"src/kx.c\0" as *const u8 as *const libc::c_char,
                    872 as libc::c_int,
                );
                cd(x_0);
            }
        }
        let mut h: K = dot_tetradic_2(w, b, c, d);
        cd(c);
        cd(d);
        if OOM_CD(0 as libc::c_int as I, b, h, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        let mut j: K = of(h, b);
        cd(b);
        return j;
    }
    while !(*v.offset(1 as libc::c_int as isize)).is_null()
        && adverbClass(*v.offset((2 as libc::c_int as libc::c_longlong + i) as isize))
            != 0
    {
        i += 1;
        i;
    }
    if sva(*v.offset(0 as libc::c_int as isize)) == 0
        && (i != 0
            || 2 as libc::c_int as libc::c_longlong
                == sva(*v.offset(1 as libc::c_int as isize)))
    {
        t2 = ex2(v.offset(2 as libc::c_int as isize).offset(i as isize), k);
        if fer > 0 as libc::c_int as libc::c_longlong
            && strcmp(
                errmsg.as_mut_ptr(),
                b"(nil)\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            return t2;
        }
        t3 = ex_(*v.offset(1 as libc::c_int as isize), 1 as libc::c_int as I) as K;
        if t3 > DT_SIZE as K && (*t3).t == 7 as libc::c_int as libc::c_longlong
            && (*t3).n == 3 as libc::c_int as libc::c_longlong
        {
            if !prnt.is_null()
                && !(*(((*prnt).k).as_mut_ptr() as *mut V)
                    .offset(CACHE_TREE as libc::c_int as isize))
                    .is_null()
                && !(*(((*prnt).k).as_mut_ptr() as *mut V)
                    .offset(CACHE_WD as libc::c_int as isize))
                    .is_null()
                && (**((*t3).k).as_mut_ptr().offset(LOCALS as libc::c_int as isize)).n
                    == 0
            {
                let mut j0_1: K = dot_monadic(
                    *(((*t3).k).as_mut_ptr() as *mut V)
                        .offset(PARAMS as libc::c_int as isize) as K,
                );
                let mut j1_1: K = dot_monadic(
                    *(((*prnt).k).as_mut_ptr() as *mut V)
                        .offset(CACHE_TREE as libc::c_int as isize) as K,
                );
                let mut j2_1: K = join(ci(j0_1), j1_1);
                cd(j0_1);
                cd(*((*t3).k).as_mut_ptr().offset(CACHE_TREE as libc::c_int as isize));
                let ref mut fresh77 = *(((*t3).k).as_mut_ptr() as *mut V)
                    .offset(CACHE_TREE as libc::c_int as isize);
                *fresh77 = dot_monadic(j2_1) as V;
                cd(j0_1);
                cd(j1_1);
                cd(j2_1);
                if (**((*prnt).k)
                    .as_mut_ptr()
                    .offset(CACHE_TREE as libc::c_int as isize))
                    .n != 0
                {
                    fsf = 1 as libc::c_int as I;
                }
            }
            if !prnt.is_null() {
                cd(prnt);
            }
            prnt = ci(t3);
        }
        u = *v.offset(1 as libc::c_int as isize) as K;
        let ref mut fresh78 = *v.offset(1 as libc::c_int as isize);
        *fresh78 = if VA(t3 as V) != 0 {
            t3 as *mut libc::c_void
        } else {
            &mut t3 as *mut K as V
        };
        t0 = ex_(*v, 1 as libc::c_int as I) as K;
        if fer > 0 as libc::c_int as libc::c_longlong
            && strcmp(
                errmsg.as_mut_ptr(),
                b"(nil)\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            cd(t2);
            return t0;
        }
        if t0 > DT_SIZE as K && (*t0).t == 7 as libc::c_int as libc::c_longlong
            && (*t0).n == 3 as libc::c_int as libc::c_longlong
        {
            if !prnt.is_null()
                && !(*(((*prnt).k).as_mut_ptr() as *mut V)
                    .offset(CACHE_TREE as libc::c_int as isize))
                    .is_null()
                && !(*(((*prnt).k).as_mut_ptr() as *mut V)
                    .offset(CACHE_WD as libc::c_int as isize))
                    .is_null()
                && (**((*t0).k).as_mut_ptr().offset(LOCALS as libc::c_int as isize)).n
                    == 0
            {
                let mut j0_2: K = dot_monadic(
                    *(((*t0).k).as_mut_ptr() as *mut V)
                        .offset(PARAMS as libc::c_int as isize) as K,
                );
                let mut j1_2: K = dot_monadic(
                    *(((*prnt).k).as_mut_ptr() as *mut V)
                        .offset(CACHE_TREE as libc::c_int as isize) as K,
                );
                let mut j2_2: K = join(ci(j0_2), j1_2);
                cd(j0_2);
                cd(*((*t0).k).as_mut_ptr().offset(CACHE_TREE as libc::c_int as isize));
                let ref mut fresh79 = *(((*t0).k).as_mut_ptr() as *mut V)
                    .offset(CACHE_TREE as libc::c_int as isize);
                *fresh79 = dot_monadic(j2_2) as V;
                cd(j0_2);
                cd(j1_2);
                cd(j2_2);
                if (**((*prnt).k)
                    .as_mut_ptr()
                    .offset(CACHE_TREE as libc::c_int as isize))
                    .n != 0
                {
                    fsf = 1 as libc::c_int as I;
                }
            }
            if !prnt.is_null() {
                cd(prnt);
            }
            prnt = ci(t0);
        }
        if prnt.is_null() && (*t0).t == 7 as libc::c_int as libc::c_longlong
            && (*t0).n == 3 as libc::c_int as libc::c_longlong
        {
            prnt = ci(t0);
        }
        if *v.offset(1 as libc::c_int as isize).offset(i as isize) == offsetDot
            && (*t0).t == 7 as libc::c_int as libc::c_longlong
            && (*t0).n == 1 as libc::c_int as libc::c_longlong
            && (*((*(*((*t0).k).as_mut_ptr().offset(CODE as libc::c_int as isize) as K))
                .k)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize) == offsetEach as V as *mut k0
                || *((*(*((*t0).k).as_mut_ptr().offset(CODE as libc::c_int as isize)
                    as K))
                    .k)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) == offsetEachright as V as *mut k0
                || *((*(*((*t0).k).as_mut_ptr().offset(CODE as libc::c_int as isize)
                    as K))
                    .k)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) == offsetEachleft as V as *mut k0
                || *((*(*((*t0).k).as_mut_ptr().offset(CODE as libc::c_int as isize)
                    as K))
                    .k)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) == offsetEachpair as V as *mut k0)
        {
            let mut p_0: K = *(((*t0).k).as_mut_ptr() as *mut V)
                .offset(CODE as libc::c_int as isize) as K;
            let mut i_0: I = (*p_0).n - 2 as libc::c_int as libc::c_longlong;
            let mut q: *mut V = (((*p_0).k).as_mut_ptr() as *mut V).offset(i_0 as isize);
            e = bv_ex(q, t2);
        } else {
            e = dv_ex(t0, v.offset(1 as libc::c_int as isize).offset(i as isize), t2);
            let ref mut fresh80 = *v.offset(1 as libc::c_int as isize);
            *fresh80 = u as V;
        }
        cd(t0);
        cd(t2);
        if VA(t3 as V) == 0 {
            cd(t3);
        }
        return e;
    }
    i = 0 as libc::c_int as I;
    while adverbClass(*v.offset((1 as libc::c_int as libc::c_longlong + i) as isize))
        != 0
    {
        i += 1;
        i;
    }
    t2 = ex2(v.offset(1 as libc::c_int as isize).offset(i as isize), k);
    t3 = ex_(*v, 1 as libc::c_int as I) as K;
    if t3 < DT_SIZE as K {
        ft3 = 1 as libc::c_int as I;
    }
    if t3 > DT_SIZE as K && (*t3).t == 7 as libc::c_int as libc::c_longlong
        && (*t3).n == 3 as libc::c_int as libc::c_longlong
    {
        if ((*t3).k).as_mut_ptr() as *mut V == ((*grnt).k).as_mut_ptr() as *mut V {
            if !cls.is_null() {
                cd(cls);
            }
            cls = ci(
                *((*(*((*(*((*prnt).k).as_mut_ptr().offset(7 as libc::c_int as isize)
                    as K))
                    .k)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as K))
                    .k)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize),
            );
        }
        if !prnt.is_null() {
            if !(*(((*prnt).k).as_mut_ptr() as *mut V)
                .offset(CACHE_WD as libc::c_int as isize))
                .is_null()
                && (**((*t3).k).as_mut_ptr().offset(LOCALS as libc::c_int as isize)).n
                    == 0
            {
                if (**((*prnt).k).as_mut_ptr().offset(LOCALS as libc::c_int as isize)).n
                    != 0
                {
                    if !(*(((*t3).k).as_mut_ptr() as *mut V)
                        .offset(CACHE_WD as libc::c_int as isize))
                        .is_null()
                        && (*(((*t3).k).as_mut_ptr() as *mut V)
                            .offset(CACHE_TREE as libc::c_int as isize))
                            .is_null()
                    {
                        let ref mut fresh81 = *((*t3).k)
                            .as_mut_ptr()
                            .offset(CACHE_TREE as libc::c_int as isize);
                        *fresh81 = *((*prnt).k)
                            .as_mut_ptr()
                            .offset(CACHE_TREE as libc::c_int as isize);
                        ci(
                            *((*t3).k)
                                .as_mut_ptr()
                                .offset(CACHE_TREE as libc::c_int as isize),
                        );
                    } else if (**((*t3).k)
                        .as_mut_ptr()
                        .offset(PARAMS as libc::c_int as isize))
                        .n != 0 || !grnt.is_null()
                    {
                        let mut j0_3: K = dot_monadic(
                            *(((*t3).k).as_mut_ptr() as *mut V)
                                .offset(PARAMS as libc::c_int as isize) as K,
                        );
                        let mut j1_3: K = dot_monadic(
                            *(((*prnt).k).as_mut_ptr() as *mut V)
                                .offset(CACHE_TREE as libc::c_int as isize) as K,
                        );
                        let mut j2_3: K = join(ci(j0_3), j1_3);
                        cd(j0_3);
                        if !(*(((*t3).k).as_mut_ptr() as *mut V)
                            .offset(CACHE_TREE as libc::c_int as isize))
                            .is_null()
                            && (**((*t3).k)
                                .as_mut_ptr()
                                .offset(CACHE_TREE as libc::c_int as isize))
                                .n != 0
                        {
                            cd(
                                *((*t3).k)
                                    .as_mut_ptr()
                                    .offset(CACHE_TREE as libc::c_int as isize),
                            );
                        }
                        let ref mut fresh82 = *(((*t3).k).as_mut_ptr() as *mut V)
                            .offset(CACHE_TREE as libc::c_int as isize);
                        *fresh82 = dot_monadic(j2_3) as V;
                        cd(j0_3);
                        cd(j1_3);
                        cd(j2_3);
                    }
                } else if !(*(((*(*((*prnt).k)
                    .as_mut_ptr()
                    .offset(CACHE_WD as libc::c_int as isize) as K))
                    .k)
                    .as_mut_ptr() as *mut V)
                    .offset(LOCALS as libc::c_int as isize))
                    .is_null()
                    && (**((*(*((*prnt).k)
                        .as_mut_ptr()
                        .offset(CACHE_WD as libc::c_int as isize) as K))
                        .k)
                        .as_mut_ptr()
                        .offset(LOCALS as libc::c_int as isize))
                        .n != 0
                    && !(*(((*prnt).k).as_mut_ptr() as *mut V)
                        .offset(CACHE_TREE as libc::c_int as isize))
                        .is_null()
                    && (**((*prnt).k)
                        .as_mut_ptr()
                        .offset(CACHE_TREE as libc::c_int as isize))
                        .n != 0
                    && ((*(((*(*((*(*((*(*((*prnt).k)
                        .as_mut_ptr()
                        .offset(CACHE_WD as libc::c_int as isize) as K))
                        .k)
                        .as_mut_ptr()
                        .offset(LOCALS as libc::c_int as isize) as K))
                        .k)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as K))
                        .k)
                        .as_mut_ptr() as *mut V)
                        .offset(1 as libc::c_int as isize))
                        .is_null()
                        || (*(((*(*((*(*((*(*((*(*((*prnt).k)
                            .as_mut_ptr()
                            .offset(CACHE_WD as libc::c_int as isize) as K))
                            .k)
                            .as_mut_ptr()
                            .offset(LOCALS as libc::c_int as isize) as K))
                            .k)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as K))
                            .k)
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize) as K))
                            .k)
                            .as_mut_ptr() as *mut V)
                            .offset(CONJ as libc::c_int as isize))
                            .is_null())
                {
                    let mut j0_4: K = dot_monadic(
                        *(((*t3).k).as_mut_ptr() as *mut V)
                            .offset(PARAMS as libc::c_int as isize) as K,
                    );
                    let mut j1_4: K = dot_monadic(
                        *(((*prnt).k).as_mut_ptr() as *mut V)
                            .offset(CACHE_TREE as libc::c_int as isize) as K,
                    );
                    let mut j2_4: K = join(ci(j0_4), j1_4);
                    cd(j0_4);
                    let ref mut fresh83 = *(((*t3).k).as_mut_ptr() as *mut V)
                        .offset(CACHE_TREE as libc::c_int as isize);
                    *fresh83 = dot_monadic(j2_4) as V;
                    cd(j0_4);
                    cd(j1_4);
                    cd(j2_4);
                }
            } else if !(*(((*prnt).k).as_mut_ptr() as *mut V)
                .offset(CACHE_TREE as libc::c_int as isize))
                .is_null()
                && 1 as libc::c_int as libc::c_longlong
                    == (**((*prnt).k)
                        .as_mut_ptr()
                        .offset(CACHE_TREE as libc::c_int as isize))
                        .n
                && (*(((*prnt).k).as_mut_ptr() as *mut V)
                    .offset(CACHE_WD as libc::c_int as isize))
                    .is_null()
                && (*(((*t3).k).as_mut_ptr() as *mut V)
                    .offset(CACHE_TREE as libc::c_int as isize))
                    .is_null()
            {
                let mut j0_5: K = dot_monadic(
                    *(((*t3).k).as_mut_ptr() as *mut V)
                        .offset(PARAMS as libc::c_int as isize) as K,
                );
                let mut j1_5: K = dot_monadic(
                    *(((*prnt).k).as_mut_ptr() as *mut V)
                        .offset(CACHE_TREE as libc::c_int as isize) as K,
                );
                let mut j2_5: K = join(ci(j0_5), j1_5);
                cd(j0_5);
                let ref mut fresh84 = *(((*t3).k).as_mut_ptr() as *mut V)
                    .offset(CACHE_TREE as libc::c_int as isize);
                *fresh84 = dot_monadic(j2_5) as V;
                cd(j0_5);
                cd(j1_5);
                cd(j2_5);
            } else if !(*(((*t3).k).as_mut_ptr() as *mut V)
                .offset(PARAMS as libc::c_int as isize))
                .is_null()
                && (**((*t3).k).as_mut_ptr().offset(PARAMS as libc::c_int as isize)).n
                    != 0
                && !(*(((*prnt).k).as_mut_ptr() as *mut V)
                    .offset(CACHE_TREE as libc::c_int as isize))
                    .is_null()
                && (**((*prnt).k)
                    .as_mut_ptr()
                    .offset(CACHE_TREE as libc::c_int as isize))
                    .n == 1 as libc::c_int as libc::c_longlong
                && ((*((*prnt).k).as_mut_ptr().offset(CACHE_WD as libc::c_int as isize))
                    .is_null()
                    || !(*((*prnt).k)
                        .as_mut_ptr()
                        .offset(CACHE_WD as libc::c_int as isize))
                        .is_null()
                        && (**((*prnt).k)
                            .as_mut_ptr()
                            .offset(CACHE_WD as libc::c_int as isize))
                            .n != 0)
            {
                let mut j0_6: K = dot_monadic(
                    *(((*t3).k).as_mut_ptr() as *mut V)
                        .offset(PARAMS as libc::c_int as isize) as K,
                );
                let mut j1_6: K = dot_monadic(
                    *(((*prnt).k).as_mut_ptr() as *mut V)
                        .offset(CACHE_TREE as libc::c_int as isize) as K,
                );
                let mut j2_6: K = join(ci(j0_6), j1_6);
                cd(j0_6);
                if !(*(((*t3).k).as_mut_ptr() as *mut V)
                    .offset(CACHE_TREE as libc::c_int as isize))
                    .is_null()
                    && (**((*t3).k)
                        .as_mut_ptr()
                        .offset(CACHE_TREE as libc::c_int as isize))
                        .n != 0
                {
                    cd(
                        *((*t3).k)
                            .as_mut_ptr()
                            .offset(CACHE_TREE as libc::c_int as isize),
                    );
                }
                let ref mut fresh85 = *(((*t3).k).as_mut_ptr() as *mut V)
                    .offset(CACHE_TREE as libc::c_int as isize);
                *fresh85 = dot_monadic(j2_6) as V;
                cd(j0_6);
                cd(j1_6);
                cd(j2_6);
            }
            if !grnt.is_null() {
                cd(prnt);
            } else {
                grnt = prnt;
            }
        }
        prnt = ci(t3);
    }
    u = *v as K;
    *v = if VA(t3 as V) != 0 { t3 as *mut libc::c_void } else { &mut t3 as *mut K as V };
    if *v.offset(i as isize) == offsetEach as V && grnt.is_null() {
        grnt = ci(prnt);
    }
    e = dv_ex(0 as K, v.offset(i as isize), t2);
    *v = u as V;
    if *v.offset(i as isize) == offsetEach as V && prnt == grnt {
        cd(grnt);
        grnt = 0 as K;
    }
    cd(t2);
    if VA(t3 as V) == 0
        && (encp != 3 as libc::c_int as libc::c_longlong
            || encp == 3 as libc::c_int as libc::c_longlong
                && !(*(((*t3).k).as_mut_ptr() as *mut V)
                    .offset(CACHE_WD as libc::c_int as isize))
                    .is_null())
    {
        cd(t3);
    }
    return e;
}
pub unsafe extern "C" fn cirRef(mut x: K, mut y: K) -> I {
    if !x.is_null() && x == y {
        return 1 as libc::c_int as I;
    }
    let mut f: I = 0 as libc::c_int as I;
    if (*x).t == 6 as libc::c_int as libc::c_longlong || y.is_null()
        || (*y).t != 0 as libc::c_int as libc::c_longlong
            && (*y).t != 5 as libc::c_int as libc::c_longlong
        || (x as UI) < DT_SIZE as libc::c_ulonglong
    {
        return 0 as libc::c_int as I;
    }
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = (*y).n;
    while i < _i {
        f = cirRef_(
            x,
            *((*y).k)
                .as_mut_ptr()
                .offset(((*y).n - i - 1 as libc::c_int as libc::c_longlong) as isize),
            f,
        );
        i += 1;
        i;
    }
    return f;
}
pub unsafe extern "C" fn cirRef_(mut x: K, mut y: K, mut f: I) -> I {
    if x == y {
        f = 1 as libc::c_int as I;
    }
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = (*y).n;
    while i < _i {
        if f == 0
            && ((*y).t == 0 as libc::c_int as libc::c_longlong
                || (*y).t == 5 as libc::c_int as libc::c_longlong)
        {
            f = cirRef_(
                x,
                *((*y).k)
                    .as_mut_ptr()
                    .offset(
                        ((*y).n - i - 1 as libc::c_int as libc::c_longlong) as isize,
                    ),
                f,
            );
        }
        i += 1;
        i;
    }
    return f;
}
