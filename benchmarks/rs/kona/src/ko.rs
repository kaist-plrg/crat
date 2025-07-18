use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn cd(a: K) -> K;
    fn VA(p: V) -> I;
    static mut LS: S;
    fn newE(s: S, k: K) -> K;
    fn Kv() -> K;
    fn kap(a: *mut K, v: V) -> K;
    fn newK(t: I, n: I) -> K;
    fn ci(a: K) -> K;
    fn OOM_CD(g: I, _: ...) -> I;
    fn EVP(e: K) -> *mut K;
}
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
static mut w: I = 0 as libc::c_int as I;
pub unsafe extern "C" fn kcloneI(mut a: K, mut f: cS, mut n: libc::c_int) -> K {
    if w != 0 {
        printf(b"kclone %s:%d \0" as *const u8 as *const libc::c_char, f, n);
    }
    return _kclone(a);
}
pub unsafe extern "C" fn _kclone(mut a: K) -> K {
    if a.is_null() {
        return 0 as K;
    }
    let mut t: I = (*a).t;
    let mut n: I = (*a).n;
    let mut z: K = if 7 as libc::c_int as libc::c_longlong == t {
        Kv()
    } else {
        newK(
            if -(5 as libc::c_int) as libc::c_longlong == t {
                -(1 as libc::c_int) as libc::c_longlong
            } else {
                t
            },
            n,
        )
    };
    (*z).t = t;
    if 4 as libc::c_int as libc::c_longlong
        == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = n;
        while i < _i {
            let ref mut fresh0 = *(((*z).k).as_mut_ptr() as *mut S).offset(i as isize);
            *fresh0 = *(((*a).k).as_mut_ptr() as *mut S).offset(i as isize);
            i += 1;
            i;
        }
    } else if 3 as libc::c_int as libc::c_longlong
        == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = n;
        while i_0 < _i_0 {
            *(((*z).k).as_mut_ptr() as *mut C)
                .offset(
                    i_0 as isize,
                ) = *(((*a).k).as_mut_ptr() as *mut C).offset(i_0 as isize);
            i_0 += 1;
            i_0;
        }
    } else if 2 as libc::c_int as libc::c_longlong
        == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = n;
        while i_1 < _i_1 {
            *(((*z).k).as_mut_ptr() as *mut F)
                .offset(
                    i_1 as isize,
                ) = *(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize);
            i_1 += 1;
            i_1;
        }
    } else if -(5 as libc::c_int) as libc::c_longlong == t
        || 1 as libc::c_int as libc::c_longlong
            == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = n;
        while i_2 < _i_2 {
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    i_2 as isize,
                ) = *(((*a).k).as_mut_ptr() as *mut I).offset(i_2 as isize);
            i_2 += 1;
            i_2;
        }
    } else if 0 as libc::c_int as libc::c_longlong == t {
        let mut i_3: I = 0 as libc::c_int as I;
        let mut _i_3: I = n;
        while i_3 < _i_3 {
            let ref mut fresh1 = *((*z).k).as_mut_ptr().offset(i_3 as isize);
            *fresh1 = _kclone(*((*a).k).as_mut_ptr().offset(i_3 as isize));
            i_3 += 1;
            i_3;
        }
    } else if 5 as libc::c_int as libc::c_longlong == t {
        let mut i_4: I = 0 as libc::c_int as I;
        let mut _i_4: I = n;
        while i_4 < _i_4 {
            let ref mut fresh2 = *((*z).k).as_mut_ptr().offset(i_4 as isize);
            *fresh2 = _kclone(*((*a).k).as_mut_ptr().offset(i_4 as isize));
            i_4 += 1;
            i_4;
        }
    } else if 7 as libc::c_int as libc::c_longlong == t {
        let mut k: I = 0 as libc::c_int as I;
        (*z).t = (*a).t;
        (*z).n = (*a).n;
        let mut vt: I = (*z).n;
        let mut kv: K = 0 as K;
        let mut v: *mut V = 0 as *mut V;
        match vt {
            1 => {
                k = (*(*(((*a).k).as_mut_ptr() as *mut V)
                    .offset(CODE as libc::c_int as isize) as K))
                    .n - 1 as libc::c_int as libc::c_longlong;
                kv = newK(
                    -(4 as libc::c_int) as I,
                    k + 1 as libc::c_int as libc::c_longlong,
                );
                if OOM_CD(0 as libc::c_int as I, z, kv, -(1 as libc::c_int) as V) == 0 {
                    return 0 as K;
                }
                v = ((*kv).k).as_mut_ptr() as *mut V;
                let mut i_5: I = 0 as libc::c_int as I;
                let mut _i_5: I = k;
                while i_5 < _i_5 {
                    let mut w_0: V = *(((*(*(((*a).k).as_mut_ptr() as *mut V)
                        .offset(CODE as libc::c_int as isize) as K))
                        .k)
                        .as_mut_ptr() as *mut S as *mut V)
                        .offset(i_5 as isize);
                    if VA(w_0) != 0 {
                        let ref mut fresh3 = *v.offset(i_5 as isize);
                        *fresh3 = w_0;
                    } else {
                        let mut r: K = _kclone(*(w_0 as *mut K));
                        let mut q: V = newE(LS, r) as V;
                        kap(
                            (((*z).k).as_mut_ptr() as *mut V as *mut K)
                                .offset(LOCALS as libc::c_int as isize),
                            &mut q as *mut V as V,
                        );
                        cd(q as K);
                        q = EVP(q as K) as V;
                        let ref mut fresh4 = *v.offset(i_5 as isize);
                        *fresh4 = q;
                    }
                    i_5 += 1;
                    i_5;
                }
            }
            2 => {
                kv = newK(-(4 as libc::c_int) as I, 3 as libc::c_int as I);
                if OOM_CD(0 as libc::c_int as I, z, kv, -(1 as libc::c_int) as V) == 0 {
                    return 0 as K;
                }
                v = ((*kv).k).as_mut_ptr() as *mut V;
                memcpy(
                    v as *mut libc::c_void,
                    ((*(*(((*a).k).as_mut_ptr() as *mut V)
                        .offset(CODE as libc::c_int as isize) as K))
                        .k)
                        .as_mut_ptr() as *mut S as *mut V as *const libc::c_void,
                    ::std::mem::size_of::<V>() as libc::c_ulong,
                );
            }
            3 => {
                kv = _kclone(
                    *(((*a).k).as_mut_ptr() as *mut V)
                        .offset(CODE as libc::c_int as isize) as K,
                );
                if OOM_CD(0 as libc::c_int as I, z, kv, -(1 as libc::c_int) as V) == 0 {
                    return 0 as K;
                }
            }
            _ => {}
        }
        let ref mut fresh5 = *(((*z).k).as_mut_ptr() as *mut V)
            .offset(CODE as libc::c_int as isize);
        *fresh5 = kv as V;
        let ref mut fresh6 = *(((*z).k).as_mut_ptr() as *mut V)
            .offset(DEPTH as libc::c_int as isize);
        *fresh6 = *(((*a).k).as_mut_ptr() as *mut V)
            .offset(DEPTH as libc::c_int as isize);
        let ref mut fresh7 = *(((*z).k).as_mut_ptr() as *mut V)
            .offset(CONTeXT as libc::c_int as isize);
        *fresh7 = *(((*a).k).as_mut_ptr() as *mut V)
            .offset(CONTeXT as libc::c_int as isize);
        cd(
            *(((*z).k).as_mut_ptr() as *mut V).offset(PARAMS as libc::c_int as isize)
                as K,
        );
        let ref mut fresh8 = *(((*z).k).as_mut_ptr() as *mut V)
            .offset(PARAMS as libc::c_int as isize);
        *fresh8 = _kclone(
            *(((*a).k).as_mut_ptr() as *mut V).offset(PARAMS as libc::c_int as isize)
                as K,
        ) as V;
        cd(
            *(((*z).k).as_mut_ptr() as *mut V).offset(LOCALS as libc::c_int as isize)
                as K,
        );
        let ref mut fresh9 = *(((*z).k).as_mut_ptr() as *mut V)
            .offset(LOCALS as libc::c_int as isize);
        *fresh9 = _kclone(
            *(((*a).k).as_mut_ptr() as *mut V).offset(LOCALS as libc::c_int as isize)
                as K,
        ) as V;
        let ref mut fresh10 = *(((*z).k).as_mut_ptr() as *mut V)
            .offset(CONJ as libc::c_int as isize);
        *fresh10 = _kclone(
            *(((*a).k).as_mut_ptr() as *mut V).offset(CONJ as libc::c_int as isize) as K,
        ) as V;
    }
    return z;
}
pub unsafe extern "C" fn collapse(mut x: K) -> K {
    if (*x).t == 1 as libc::c_int as libc::c_longlong
        && (*x).n == 1 as libc::c_int as libc::c_longlong
    {
        return x;
    }
    if (*x).t < 0 as libc::c_int as libc::c_longlong
        && (*x).n == 1 as libc::c_int as libc::c_longlong
    {
        (*x)
            .t = if (*x).t < 0 as libc::c_int as libc::c_longlong {
            -(*x).t
        } else {
            (*x).t
        };
        return x;
    }
    let mut z: K = 0 as *mut k0;
    if 1 as libc::c_int as libc::c_longlong == (*x).n {
        z = ci(*((*x).k).as_mut_ptr());
        cd(x);
    } else {
        z = demote(x);
    }
    return z;
}
pub unsafe extern "C" fn delist(mut x: K) -> K {
    let mut z: K = 0 as K;
    let mut t: K = x;
    while (*t).t == 0 as libc::c_int as libc::c_longlong
        && (*t).n == 1 as libc::c_int as libc::c_longlong
    {
        z = *((*t).k).as_mut_ptr();
        t = z;
    }
    z = ci(t);
    cd(x);
    return z;
}
pub unsafe extern "C" fn demote(mut a: K) -> K {
    if a.is_null() {
        return a;
    }
    let mut t: I = (*a).t;
    let mut n: I = (*a).n;
    if 0 as libc::c_int as libc::c_longlong != t
        || 1 as libc::c_int as libc::c_longlong > n
    {
        return a;
    }
    let mut p: I = (**((*a).k).as_mut_ptr().offset(0 as libc::c_int as isize)).t;
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = n;
    while i < _i {
        if p != (**((*a).k).as_mut_ptr().offset(i as isize)).t {
            p = 0 as libc::c_int as I;
        }
        i += 1;
        i;
    }
    if !(1 as libc::c_int as libc::c_longlong <= p
        && p <= 4 as libc::c_int as libc::c_longlong)
    {
        return a;
    }
    let mut z: K = newK(-p, n);
    if OOM_CD(0 as libc::c_int as I, a, z, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    if 4 as libc::c_int as libc::c_longlong == p {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = n;
        while i_0 < _i_0 {
            let ref mut fresh11 = *(((*z).k).as_mut_ptr() as *mut S)
                .offset(i_0 as isize);
            *fresh11 = *(((*(*((*a).k).as_mut_ptr().offset(i_0 as isize) as K)).k)
                .as_mut_ptr() as *mut S);
            i_0 += 1;
            i_0;
        }
    } else if 3 as libc::c_int as libc::c_longlong == p {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = n;
        while i_1 < _i_1 {
            *(((*z).k).as_mut_ptr() as *mut C)
                .offset(
                    i_1 as isize,
                ) = *(((*(*((*a).k).as_mut_ptr().offset(i_1 as isize) as K)).k)
                .as_mut_ptr() as *mut C);
            i_1 += 1;
            i_1;
        }
    } else if 2 as libc::c_int as libc::c_longlong == p {
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = n;
        while i_2 < _i_2 {
            *(((*z).k).as_mut_ptr() as *mut F)
                .offset(
                    i_2 as isize,
                ) = *(((*(*((*a).k).as_mut_ptr().offset(i_2 as isize) as K)).k)
                .as_mut_ptr() as *mut F);
            i_2 += 1;
            i_2;
        }
    } else if 1 as libc::c_int as libc::c_longlong == p {
        let mut i_3: I = 0 as libc::c_int as I;
        let mut _i_3: I = n;
        while i_3 < _i_3 {
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    i_3 as isize,
                ) = *(((*(*((*a).k).as_mut_ptr().offset(i_3 as isize) as K)).k)
                .as_mut_ptr() as *mut I);
            i_3 += 1;
            i_3;
        }
    }
    cd(a);
    if (*z).t == -(1 as libc::c_int) as libc::c_longlong
        && (*z).n == 1 as libc::c_int as libc::c_longlong
    {
        (*z).t = 1 as libc::c_int as I;
    }
    return z;
}
pub unsafe extern "C" fn promote(mut a: K) -> K {
    let mut at: I = (*a).t;
    if 0 as libc::c_int as libc::c_longlong == at {
        return ci(a);
    }
    if (4 as libc::c_int as libc::c_longlong) < at {
        let mut z: K = newK(0 as libc::c_int as I, 1 as libc::c_int as I);
        if z.is_null() {
            return 0 as K;
        }
        let ref mut fresh12 = *((*z).k).as_mut_ptr();
        *fresh12 = ci(a);
        return z;
    }
    let mut z_0: K = newK(0 as libc::c_int as I, (*a).n);
    if z_0.is_null() {
        return 0 as K;
    }
    let mut x: K = 0 as *mut k0;
    let mut v: I = if at < 0 as libc::c_int as libc::c_longlong { -at } else { at };
    if 4 as libc::c_int as libc::c_longlong == v {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = (*a).n;
        while i < _i {
            x = newK(v, 1 as libc::c_int as I);
            if OOM_CD(0 as libc::c_int as I, x, z_0, -(1 as libc::c_int) as V) == 0 {
                return 0 as K;
            }
            let ref mut fresh13 = *(((*x).k).as_mut_ptr() as *mut S);
            *fresh13 = *(((*a).k).as_mut_ptr() as *mut S).offset(i as isize);
            let ref mut fresh14 = *((*z_0).k).as_mut_ptr().offset(i as isize);
            *fresh14 = x;
            i += 1;
            i;
        }
    } else if 3 as libc::c_int as libc::c_longlong == v {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = (*a).n;
        while i_0 < _i_0 {
            x = newK(v, 1 as libc::c_int as I);
            if OOM_CD(0 as libc::c_int as I, x, z_0, -(1 as libc::c_int) as V) == 0 {
                return 0 as K;
            }
            *(((*x).k).as_mut_ptr()
                as *mut C) = *(((*a).k).as_mut_ptr() as *mut C).offset(i_0 as isize);
            let ref mut fresh15 = *((*z_0).k).as_mut_ptr().offset(i_0 as isize);
            *fresh15 = x;
            i_0 += 1;
            i_0;
        }
    } else if 2 as libc::c_int as libc::c_longlong == v {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = (*a).n;
        while i_1 < _i_1 {
            x = newK(v, 1 as libc::c_int as I);
            if OOM_CD(0 as libc::c_int as I, x, z_0, -(1 as libc::c_int) as V) == 0 {
                return 0 as K;
            }
            *(((*x).k).as_mut_ptr()
                as *mut F) = *(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize);
            let ref mut fresh16 = *((*z_0).k).as_mut_ptr().offset(i_1 as isize);
            *fresh16 = x;
            i_1 += 1;
            i_1;
        }
    } else if 1 as libc::c_int as libc::c_longlong == v {
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = (*a).n;
        while i_2 < _i_2 {
            x = newK(v, 1 as libc::c_int as I);
            if OOM_CD(0 as libc::c_int as I, x, z_0, -(1 as libc::c_int) as V) == 0 {
                return 0 as K;
            }
            *(((*x).k).as_mut_ptr()
                as *mut I) = *(((*a).k).as_mut_ptr() as *mut I).offset(i_2 as isize);
            let ref mut fresh17 = *((*z_0).k).as_mut_ptr().offset(i_2 as isize);
            *fresh17 = x;
            i_2 += 1;
            i_2;
        }
    }
    return z_0;
}
