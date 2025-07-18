use ::libc;
extern "C" {
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn cd(a: K) -> K;
    static mut LS: S;
    static mut NIL: K;
    fn kerr(s: cS) -> K;
    fn rc(x: K) -> I;
    fn mergeGrade(a: K, r: I) -> K;
    fn charGrade(a: K, r: I) -> K;
    fn newK(t: I, n: I) -> K;
    fn distributionGrade(a: K, r: I, u: uI, v: uI) -> K;
    fn FC(a: F, b: F) -> I;
    fn radixGrade(a: K, r: I, h: uI) -> K;
    fn insertGradeU(a: K, r: I) -> K;
    fn setS(y: libc::c_int, z: I);
    fn trst();
    fn Ks(x: S) -> K;
    fn Kc(x: C) -> K;
    fn Kf(x: F) -> K;
    fn Ki(x: I) -> K;
    fn _n() -> K;
    fn ci(a: K) -> K;
    fn OOM_CD(g: I, _: ...) -> I;
    fn cl2(v: I) -> I;
    fn matchI(a: K, b: K) -> I;
    fn promote(a: K) -> K;
    fn demote(x: K) -> K;
    fn itemAtIndex(a: K, i: I) -> K;
    fn symGrade(a: K, r: I) -> K;
    fn genrand64_int64() -> libc::c_ulonglong;
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type L = libc::c_longlong;
pub type UI = libc::c_ulonglong;
pub type V = *mut libc::c_void;
pub type I = libc::c_longlong;
pub type uI = libc::c_ulonglong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub f: F,
    pub i: I,
}
unsafe extern "C" fn FtoI(mut a: F) -> I {
    let mut u: C2RustUnnamed = C2RustUnnamed { f: 0. };
    if a.is_nan() as i32 != 0 {
        return -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong;
    }
    u.f = a;
    return if 0 as libc::c_int as libc::c_longlong > u.i {
        -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong - u.i
    } else {
        u.i
    };
}
unsafe extern "C" fn ItoU(mut a: I) -> uI {
    return 0x8000000000000000 as libc::c_ulonglong ^ a as uI;
}
pub unsafe extern "C" fn grade_updown(mut a: K, mut r: I) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    if (0 as libc::c_int as libc::c_longlong) < at {
        return kerr(b"rank\0" as *const u8 as *const libc::c_char);
    }
    if -(4 as libc::c_int) as libc::c_longlong == at {
        return symGrade(a, r);
    }
    if -(3 as libc::c_int) as libc::c_longlong == at {
        return charGrade(a, r);
    }
    if -(1 as libc::c_int) as libc::c_longlong == at
        || -(2 as libc::c_int) as libc::c_longlong == at
    {
        let mut z: K = 0 as *mut k0;
        if an < 2 as libc::c_int as libc::c_longlong {
            z = newK(-(1 as libc::c_int) as I, an);
            if OOM_CD(0 as libc::c_int as I, z, -(1 as libc::c_int) as V) == 0 {
                return 0 as K;
            }
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = an;
            while i < _i {
                *(((*z).k).as_mut_ptr() as *mut I).offset(i as isize) = i;
                i += 1;
                i;
            }
            return z;
        } else {
            let mut x: K = 0 as K;
            let mut y: uI = 0;
            let mut u: uI = -(1 as libc::c_int) as uI;
            let mut v: uI = 0 as libc::c_int as uI;
            let mut h: uI = 0 as libc::c_int as uI;
            let mut k: uI = 0;
            if -(2 as libc::c_int) as libc::c_longlong == at {
                x = newK(-(1 as libc::c_int) as I, an);
                if OOM_CD(0 as libc::c_int as I, x, -(1 as libc::c_int) as V) == 0 {
                    return 0 as K;
                }
            }
            if -(1 as libc::c_int) as libc::c_longlong == at {
                let mut i_0: I = 0 as libc::c_int as I;
                let mut _i_0: I = an;
                while i_0 < _i_0 {
                    y = *(((*a).k).as_mut_ptr() as *mut I).offset(i_0 as isize) as uI;
                    h |= y;
                    if y < u {
                        u = y;
                    }
                    if y > v {
                        v = y;
                    }
                    i_0 += 1;
                    i_0;
                }
            } else {
                let mut i_1: I = 0 as libc::c_int as I;
                let mut _i_1: I = an;
                while i_1 < _i_1 {
                    y = FtoI(*(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize))
                        as uI;
                    *(((*x).k).as_mut_ptr() as *mut uI).offset(i_1 as isize) = y;
                    h |= y;
                    if y < u {
                        u = y;
                    }
                    if y > v {
                        v = y;
                    }
                    i_1 += 1;
                    i_1;
                }
            }
            if r != 0 && -(1 as libc::c_int) as libc::c_longlong == at
                || u
                    & (-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong) as uI
                    != v
                        & (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong) as uI
            {
                u = -(1 as libc::c_int) as uI;
                v = 0 as libc::c_int as uI;
                h = 0 as libc::c_int as uI;
                if -(1 as libc::c_int) as libc::c_longlong == at {
                    x = newK(-(1 as libc::c_int) as I, an);
                    if OOM_CD(0 as libc::c_int as I, x, -(1 as libc::c_int) as V) == 0 {
                        return 0 as K;
                    }
                    let mut i_2: I = 0 as libc::c_int as I;
                    let mut _i_2: I = an;
                    while i_2 < _i_2 {
                        y = ItoU(
                            *(((*a).k).as_mut_ptr() as *mut I).offset(i_2 as isize),
                        );
                        *(((*x).k).as_mut_ptr() as *mut uI).offset(i_2 as isize) = y;
                        h |= y;
                        if y < u {
                            u = y;
                        }
                        if y > v {
                            v = y;
                        }
                        i_2 += 1;
                        i_2;
                    }
                } else {
                    let mut i_3: I = 0 as libc::c_int as I;
                    let mut _i_3: I = an;
                    while i_3 < _i_3 {
                        y = ItoU(
                            *(((*x).k).as_mut_ptr() as *mut I).offset(i_3 as isize),
                        );
                        *(((*x).k).as_mut_ptr() as *mut uI).offset(i_3 as isize) = y;
                        h |= y;
                        if y < u {
                            u = y;
                        }
                        if y > v {
                            v = y;
                        }
                        i_3 += 1;
                        i_3;
                    }
                }
            }
            k = v.wrapping_sub(u);
            if k == 0 {
                z = newK(-(1 as libc::c_int) as I, an);
                if OOM_CD(0 as libc::c_int as I, z, -(1 as libc::c_int) as V) == 0 {
                    return 0 as K;
                }
                let mut i_4: I = 0 as libc::c_int as I;
                let mut _i_4: I = an;
                while i_4 < _i_4 {
                    *(((*z).k).as_mut_ptr() as *mut I).offset(i_4 as isize) = i_4;
                    i_4 += 1;
                    i_4;
                }
            } else if an < 7 as libc::c_int as libc::c_longlong {
                z = insertGradeU(if !x.is_null() { x } else { a }, r);
            } else if k < ((1 as libc::c_int) << 26 as libc::c_int) as libc::c_ulonglong
                && (9 as libc::c_int as libc::c_longlong * an
                    + (1 as libc::c_int ^ 19 as libc::c_int) as libc::c_longlong)
                    as libc::c_ulonglong
                    > (2 as libc::c_int as libc::c_ulonglong).wrapping_mul(k)
            {
                z = distributionGrade(if !x.is_null() { x } else { a }, r, u, v);
            } else {
                z = radixGrade(if !x.is_null() { x } else { a }, r, h);
            }
            cd(x);
        }
        return z;
    }
    return mergeGrade(a, r);
}
pub unsafe extern "C" fn grade_up(mut a: K) -> K {
    return grade_updown(a, 0 as libc::c_int as I);
}
pub unsafe extern "C" fn grade_down(mut a: K) -> K {
    return grade_updown(a, 1 as libc::c_int as I);
}
pub unsafe extern "C" fn enlist(mut x: K) -> K {
    let mut t: I = if 1 as libc::c_int as libc::c_longlong <= (*x).t
        && (*x).t <= 4 as libc::c_int as libc::c_longlong
    {
        -(*x).t
    } else {
        0 as libc::c_int as libc::c_longlong
    };
    let mut z: K = newK(t, 1 as libc::c_int as I);
    if -(4 as libc::c_int) as libc::c_longlong == t {
        let ref mut fresh0 = *(((*z).k).as_mut_ptr() as *mut S);
        *fresh0 = *(((*x).k).as_mut_ptr() as *mut S);
    }
    if -(3 as libc::c_int) as libc::c_longlong == t {
        *(((*z).k).as_mut_ptr() as *mut C) = *(((*x).k).as_mut_ptr() as *mut C);
    }
    if -(2 as libc::c_int) as libc::c_longlong == t {
        *(((*z).k).as_mut_ptr() as *mut F) = *(((*x).k).as_mut_ptr() as *mut F);
    }
    if -(1 as libc::c_int) as libc::c_longlong == t {
        *(((*z).k).as_mut_ptr() as *mut I) = *(((*x).k).as_mut_ptr() as *mut I);
    }
    if 0 as libc::c_int as libc::c_longlong == t {
        let ref mut fresh1 = *((*z).k).as_mut_ptr();
        *fresh1 = ci(x);
    }
    return z;
}
unsafe extern "C" fn charRange(mut a: K) -> K {
    let mut n: I = (*a).n;
    let mut c: [I; 256] = [0; 256];
    let mut j: I = 0 as libc::c_int as I;
    memset(
        c.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ((1 as libc::c_int + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int))
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong),
    );
    let mut z: K = newK(-(3 as libc::c_int) as I, n);
    if OOM_CD(0 as libc::c_int as I, z, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = n;
    while i < _i {
        let mut x: UC = *(((*a).k).as_mut_ptr() as *mut C).offset(i as isize) as UC;
        if c[x as usize] == 0 {
            c[x as usize] = -(1 as libc::c_int) as I;
            let fresh2 = j;
            j = j + 1;
            *(((*z).k).as_mut_ptr() as *mut C)
                .offset(
                    fresh2 as isize,
                ) = *(((*a).k).as_mut_ptr() as *mut C).offset(i as isize);
        }
        i += 1;
        i;
    }
    if n == j {
        return z;
    }
    let mut y: K = newK(-(3 as libc::c_int) as I, j);
    if OOM_CD(0 as libc::c_int as I, z, y, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    memcpy(
        ((*y).k).as_mut_ptr() as *mut C as *mut libc::c_void,
        ((*z).k).as_mut_ptr() as *mut C as *const libc::c_void,
        (j as libc::c_ulonglong)
            .wrapping_mul(
                ::std::mem::size_of::<C>() as libc::c_ulong as libc::c_ulonglong,
            ) as libc::c_ulong,
    );
    cd(z);
    return y;
}
unsafe extern "C" fn symRange(mut x: K) -> K {
    let mut j: I = 0 as libc::c_int as I;
    let mut z: K = newK(-(4 as libc::c_int) as I, (*x).n);
    if OOM_CD(0 as libc::c_int as I, z, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    setS(2 as libc::c_int, 0 as libc::c_int as I);
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = (*x).n;
    while i < _i {
        let mut s: S = *(((*x).k).as_mut_ptr() as *mut S).offset(i as isize);
        if *(s as *mut I).offset(-(2 as libc::c_int) as isize) == 0 {
            *(s as *mut I)
                .offset(-(2 as libc::c_int) as isize) = -(1 as libc::c_int) as I;
            let fresh3 = j;
            j = j + 1;
            let ref mut fresh4 = *(((*z).k).as_mut_ptr() as *mut S)
                .offset(fresh3 as isize);
            *fresh4 = s;
        }
        i += 1;
        i;
    }
    if (*x).n == j {
        return z;
    }
    let mut y: K = newK(-(4 as libc::c_int) as I, j);
    if OOM_CD(0 as libc::c_int as I, z, y, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    memcpy(
        ((*y).k).as_mut_ptr() as *mut S as *mut libc::c_void,
        ((*z).k).as_mut_ptr() as *mut S as *const libc::c_void,
        (j as libc::c_ulonglong)
            .wrapping_mul(
                ::std::mem::size_of::<S>() as libc::c_ulong as libc::c_ulonglong,
            ) as libc::c_ulong,
    );
    cd(z);
    return y;
}
unsafe extern "C" fn newH(mut n: I) -> K {
    let mut m: I = ((1 as libc::c_int) << 1 as libc::c_int as libc::c_longlong + cl2(n))
        as I;
    let mut h: K = newK(-(1 as libc::c_int) as I, m);
    if OOM_CD(0 as libc::c_int as I, h, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    return h;
}
unsafe extern "C" fn hg(mut h: K, mut hk: uI, mut k: I, mut p: *mut uI) -> I {
    let mut n: I = (*h).n;
    let mut d: *mut I = ((*h).k).as_mut_ptr() as *mut I;
    let mut u: uI = hk & (n - 1 as libc::c_int as libc::c_longlong) as libc::c_ulonglong;
    while *d.offset(u as isize) != 0 {
        if k == *d.offset(u as isize) {
            *p = u;
            return k;
        }
        u = u.wrapping_add(1);
        if u == n as libc::c_ulonglong {
            u = 0 as libc::c_int as uI;
        }
    }
    *p = u;
    return 0 as libc::c_int as I;
}
static mut hcc: [uI; 8] = [
    0 as libc::c_int as uI,
    0 as libc::c_int as uI,
    0 as libc::c_int as uI,
    0 as libc::c_int as uI,
    0 as libc::c_int as uI,
    0 as libc::c_int as uI,
    0 as libc::c_int as uI,
    0 as libc::c_int as uI,
];
unsafe extern "C" fn hcinit() {
    if hcc[0 as libc::c_int as usize] == 0 {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = 8 as libc::c_int as I;
        while i < _i {
            hcc[i as usize] = genrand64_int64();
            i += 1;
            i;
        }
    }
}
unsafe extern "C" fn hc(mut u: uI) -> uint32_t {
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = 8 as libc::c_int as I;
    while i < _i {
        u ^= hcc[i as usize];
        u = (u as libc::c_ulonglong).wrapping_add(u >> 8 as libc::c_int) as uI as uI;
        i += 1;
        i;
    }
    return (u as uint32_t as libc::c_ulonglong ^ u >> 32 as libc::c_int) as uint32_t;
}
unsafe extern "C" fn intRange(mut x: K) -> K {
    let mut y: K = 0 as *mut k0;
    hcinit();
    let mut j: I = 0 as libc::c_int as I;
    let mut h0: I = 0 as libc::c_int as I;
    let mut sa: I = 0 as libc::c_int as I;
    let mut m: uI = 0 as libc::c_int as uI;
    let mut h: K = newH((*x).n);
    if OOM_CD(0 as libc::c_int as I, h, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut z: K = newK((*x).t, (*x).n);
    if OOM_CD(0 as libc::c_int as I, h, z, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = (*x).n;
    while i < _i {
        m |= *(((*x).k).as_mut_ptr() as *mut uI).offset(i as isize);
        i += 1;
        i;
    }
    if m != 0 {
        while m & 1 as libc::c_int as libc::c_ulonglong == 0 {
            m >>= 1 as libc::c_int;
            sa += 1;
            sa;
        }
    }
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*x).n;
    while i_0 < _i_0 {
        let mut v: uI = *(((*x).k).as_mut_ptr() as *mut uI).offset(i_0 as isize);
        if v == 0 {
            if h0 == 0 {
                h0 = 1 as libc::c_int as I;
                let fresh5 = j;
                j = j + 1;
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(fresh5 as isize) = 0 as libc::c_int as I;
            }
        } else {
            let mut vsa: uI = v >> sa;
            let mut u: uI = if m < (*h).n as libc::c_ulonglong {
                vsa
            } else {
                hc(vsa) as libc::c_ulonglong
            };
            let mut p: uI = 0;
            if hg(h, u, vsa as I, &mut p) == 0 {
                *(((*h).k).as_mut_ptr() as *mut I).offset(p as isize) = vsa as I;
                let fresh6 = j;
                j = j + 1;
                *(((*z).k).as_mut_ptr() as *mut I).offset(fresh6 as isize) = v as I;
            }
        }
        i_0 += 1;
        i_0;
    }
    if !((*x).n == j) {
        y = newK((*x).t, j);
        if !y.is_null() {
            memcpy(
                ((*y).k).as_mut_ptr() as *mut I as *mut libc::c_void,
                ((*z).k).as_mut_ptr() as *mut I as *const libc::c_void,
                (j as libc::c_ulonglong)
                    .wrapping_mul(
                        ::std::mem::size_of::<I>() as libc::c_ulong as libc::c_ulonglong,
                    ) as libc::c_ulong,
            );
            cd(z);
            z = y;
        }
    }
    cd(h);
    return z;
}
unsafe extern "C" fn KEQ(mut a: K, mut b: K) -> I {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    let mut A: I = if at < 0 as libc::c_int as libc::c_longlong { -at } else { at };
    if at != bt {
        return 0 as libc::c_int as I;
    }
    if an != bn {
        return 0 as libc::c_int as I;
    }
    if 7 as libc::c_int as libc::c_longlong == A {
        return 0 as libc::c_int as I
    } else if 6 as libc::c_int as libc::c_longlong == A {
        return 1 as libc::c_int as I
    } else if 5 as libc::c_int as libc::c_longlong == A {
        return 0 as libc::c_int as I
    } else if 4 as libc::c_int as libc::c_longlong == A {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = an;
        while i < _i {
            if *(((*a).k).as_mut_ptr() as *mut S).offset(i as isize)
                != *(((*b).k).as_mut_ptr() as *mut S).offset(i as isize)
            {
                return 0 as libc::c_int as I;
            }
            i += 1;
            i;
        }
    } else if 3 as libc::c_int as libc::c_longlong == A {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = an;
        while i_0 < _i_0 {
            if *(((*a).k).as_mut_ptr() as *mut C).offset(i_0 as isize) as libc::c_int
                != *(((*b).k).as_mut_ptr() as *mut C).offset(i_0 as isize) as libc::c_int
            {
                return 0 as libc::c_int as I;
            }
            i_0 += 1;
            i_0;
        }
    } else if 2 as libc::c_int as libc::c_longlong == A {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = an;
        while i_1 < _i_1 {
            if FC(
                *(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize),
                *(((*b).k).as_mut_ptr() as *mut F).offset(i_1 as isize),
            ) != 0
            {
                return 0 as libc::c_int as I;
            }
            i_1 += 1;
            i_1;
        }
    } else if 1 as libc::c_int as libc::c_longlong == A {
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = an;
        while i_2 < _i_2 {
            if *(((*a).k).as_mut_ptr() as *mut I).offset(i_2 as isize)
                != *(((*b).k).as_mut_ptr() as *mut I).offset(i_2 as isize)
            {
                return 0 as libc::c_int as I;
            }
            i_2 += 1;
            i_2;
        }
    } else if 0 as libc::c_int as libc::c_longlong == A {
        let mut i_3: I = 0 as libc::c_int as I;
        let mut _i_3: I = an;
        while i_3 < _i_3 {
            if KEQ(
                *((*a).k).as_mut_ptr().offset(i_3 as isize),
                *((*b).k).as_mut_ptr().offset(i_3 as isize),
            ) == 0
            {
                return 0 as libc::c_int as I;
            }
            i_3 += 1;
            i_3;
        }
    }
    return 1 as libc::c_int as I;
}
unsafe extern "C" fn shg(mut sh: K, mut hk: uI, mut k: K, mut p: *mut uI) -> K {
    let mut n: I = (*sh).n;
    let mut d: *mut K = ((*sh).k).as_mut_ptr();
    let mut u: uI = hk & (n - 1 as libc::c_int as libc::c_longlong) as libc::c_ulonglong;
    while !(*d.offset(u as isize)).is_null() {
        if KEQ(k, *d.offset(u as isize)) != 0 {
            *p = u;
            return k;
        }
        u = u.wrapping_add(1);
        if u == n as libc::c_ulonglong {
            u = 0 as libc::c_int as uI;
        }
    }
    *p = u;
    return 0 as K;
}
pub unsafe extern "C" fn fnv1a(mut x: *mut UC, mut n: I) -> uint32_t {
    let mut h: uint32_t = 2166136261 as libc::c_ulong as uint32_t;
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = n;
    while i < _i {
        h ^= *x.offset(i as isize) as libc::c_uint;
        h = (h as libc::c_ulong).wrapping_mul(16777619 as libc::c_ulong) as uint32_t
            as uint32_t;
        i += 1;
        i;
    }
    return h;
}
unsafe extern "C" fn hcode(mut x: K) -> UI {
    let mut t: I = if (*x).t < 0 as libc::c_int as libc::c_longlong {
        -(*x).t
    } else {
        (*x).t
    };
    let mut u: uI = 0 as libc::c_int as uI;
    match t {
        7 => return t as UI,
        6 => return &mut NIL as *mut K as UI,
        5 => return t as UI,
        4 => {
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = (*x).n;
            while i < _i {
                let mut v: S = *(((*x).k).as_mut_ptr() as *mut S).offset(i as isize);
                if *(v as *mut I).offset(-(1 as libc::c_int) as isize) == 0 {
                    *(v as *mut I)
                        .offset(
                            -(1 as libc::c_int) as isize,
                        ) = fnv1a(v as *mut UC, strlen(v as *const libc::c_char) as I)
                        as I;
                }
                u = (u as libc::c_ulonglong)
                    .wrapping_add(
                        *(v as *mut I).offset(-(1 as libc::c_int) as isize)
                            as libc::c_ulonglong,
                    ) as uI as uI;
                i += 1;
                i;
            }
            return ((*x).t as libc::c_ulonglong).wrapping_add(u);
        }
        3 => {
            return ((*x).t
                + fnv1a(((*x).k).as_mut_ptr() as *mut C as *mut UC, (*x).n)
                    as libc::c_longlong) as UI;
        }
        2 | 1 => {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = (*x).n;
            while i_0 < _i_0 {
                let mut v_0: uI = *(((*x).k).as_mut_ptr() as *mut I).offset(i_0 as isize)
                    as uI;
                u = (u as libc::c_ulonglong).wrapping_add(hc(v_0) as libc::c_ulonglong)
                    as uI as uI;
                i_0 += 1;
                i_0;
            }
            return ((*x).t as libc::c_ulonglong).wrapping_add(u);
        }
        0 => {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = (*x).n;
            while i_1 < _i_1 {
                let mut v_1: K = *((*x).k).as_mut_ptr().offset(i_1 as isize);
                u = (u as libc::c_ulonglong).wrapping_add(hcode(v_1)) as uI as uI;
                i_1 += 1;
                i_1;
            }
            return u;
        }
        _ => {}
    }
    return 0 as libc::c_int as UI;
}
unsafe extern "C" fn listRange(mut x: K) -> K {
    let mut y: K = 0 as *mut k0;
    hcinit();
    let mut j: I = 0 as libc::c_int as I;
    setS(1 as libc::c_int, 0 as libc::c_int as I);
    let mut sh: K = newH((*x).n);
    if OOM_CD(0 as libc::c_int as I, sh, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut z: K = newK((*x).t, (*x).n);
    if OOM_CD(0 as libc::c_int as I, sh, z, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = (*x).n;
    while i < _i {
        let mut p: uI = 0;
        let mut kv: K = *((*x).k).as_mut_ptr().offset(i as isize);
        let mut u: uI = hcode(kv);
        if (shg(sh, u, kv, &mut p)).is_null() {
            let ref mut fresh7 = *((*sh).k).as_mut_ptr().offset(p as isize);
            *fresh7 = kv;
            let fresh8 = j;
            j = j + 1;
            let ref mut fresh9 = *((*z).k).as_mut_ptr().offset(fresh8 as isize);
            *fresh9 = ci(kv);
        }
        i += 1;
        i;
    }
    if !((*x).n == j) {
        y = newK((*x).t, j);
        if !y.is_null() {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = j;
            while i_0 < _i_0 {
                let ref mut fresh10 = *((*y).k).as_mut_ptr().offset(i_0 as isize);
                *fresh10 = ci(*((*z).k).as_mut_ptr().offset(i_0 as isize));
                i_0 += 1;
                i_0;
            }
            cd(z);
            z = y;
        }
    }
    cd(sh);
    return z;
}
pub unsafe extern "C" fn range(mut a: K) -> K {
    let mut x: I = 0;
    let mut t: I = (*a).t;
    let mut n: I = (*a).n;
    let mut z: K = 0 as K;
    let mut g: K = 0 as K;
    let mut k: K = 0 as K;
    let mut u: I = n;
    let mut h: *mut I = 0 as *mut I;
    let mut m: *mut I = 0 as *mut I;
    if t > 0 as libc::c_int as libc::c_longlong {
        return kerr(b"rank\0" as *const u8 as *const libc::c_char);
    }
    match -t {
        0 => return listRange(a),
        1 | 2 => return intRange(a),
        3 => return charRange(a),
        4 => return symRange(a),
        _ => {}
    }
    g = grade_up(a);
    if !g.is_null() {
        h = ((*g).k).as_mut_ptr() as *mut I;
        k = newK(-(1 as libc::c_int) as I, n);
        if !k.is_null() {
            m = ((*k).k).as_mut_ptr() as *mut I;
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = n;
            while i < _i {
                *m.offset(*h.offset(i as isize) as isize) = i;
                i += 1;
                i;
            }
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = n - 1 as libc::c_int as libc::c_longlong;
            while i_0 < _i_0 {
                if matchI(
                    *((*a).k)
                        .as_mut_ptr()
                        .offset(
                            *h
                                .offset(
                                    (n - i_0 - 1 as libc::c_int as libc::c_longlong) as isize,
                                ) as isize,
                        ),
                    *((*a).k)
                        .as_mut_ptr()
                        .offset(
                            *h
                                .offset(
                                    (n - i_0 - 2 as libc::c_int as libc::c_longlong) as isize,
                                ) as isize,
                        ),
                ) != 0
                {
                    *h
                        .offset(
                            (n - i_0 - 1 as libc::c_int as libc::c_longlong) as isize,
                        ) = -(1 as libc::c_int) as I;
                    u -= 1;
                    u;
                }
                i_0 += 1;
                i_0;
            }
            z = newK(t, u);
            if !z.is_null() {
                x = 0 as libc::c_int as I;
                let mut i_1: I = 0 as libc::c_int as I;
                let mut _i_1: I = n;
                while i_1 < _i_1 {
                    if *h.offset(*m.offset(i_1 as isize) as isize)
                        > -(1 as libc::c_int) as libc::c_longlong
                    {
                        let fresh11 = x;
                        x = x + 1;
                        let ref mut fresh12 = *((*z).k)
                            .as_mut_ptr()
                            .offset(fresh11 as isize);
                        *fresh12 = ci(
                            *((*a).k)
                                .as_mut_ptr()
                                .offset(
                                    *h.offset(*m.offset(i_1 as isize) as isize) as isize,
                                ),
                        );
                    }
                    i_1 += 1;
                    i_1;
                }
            }
        }
    }
    cd(k);
    cd(g);
    return z;
}
unsafe extern "C" fn charGroup(mut x: K) -> K {
    trst();
    let mut h: [I; 256] = [0; 256];
    let mut c: [I; 256] = [0; 256];
    let mut j: I = 0 as libc::c_int as I;
    memset(
        h.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ((1 as libc::c_int + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int))
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong),
    );
    memset(
        c.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ((1 as libc::c_int + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int))
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong),
    );
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = (*x).n;
    while i < _i {
        let mut u: UC = *(((*x).k).as_mut_ptr() as *mut C).offset(i as isize) as UC;
        if h[u as usize] == 0 {
            j += 1;
            h[u as usize] = j;
        }
        let mut w: I = h[u as usize] - 1 as libc::c_int as libc::c_longlong;
        c[w as usize] += 1;
        c[w as usize];
        i += 1;
        i;
    }
    let mut y: K = newK(0 as libc::c_int as I, j);
    if OOM_CD(0 as libc::c_int as I, y, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_0: I = j;
    while i_0 < _i_0 {
        let mut z: K = newK(-(1 as libc::c_int) as I, c[i_0 as usize]);
        if OOM_CD(0 as libc::c_int as I, z, y, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        let ref mut fresh13 = *((*y).k).as_mut_ptr().offset(i_0 as isize);
        *fresh13 = z;
        c[i_0 as usize] = 0 as libc::c_int as I;
        i_0 += 1;
        i_0;
    }
    let mut i_1: I = 0 as libc::c_int as I;
    let mut _i_1: I = (*x).n;
    while i_1 < _i_1 {
        let mut u_0: UC = *(((*x).k).as_mut_ptr() as *mut C).offset(i_1 as isize) as UC;
        let mut w_0: I = h[u_0 as usize] - 1 as libc::c_int as libc::c_longlong;
        let mut z_0: K = *((*y).k).as_mut_ptr().offset(w_0 as isize);
        let fresh14 = c[w_0 as usize];
        c[w_0 as usize] = c[w_0 as usize] + 1;
        *(((*z_0).k).as_mut_ptr() as *mut I).offset(fresh14 as isize) = i_1;
        i_1 += 1;
        i_1;
    }
    return y;
}
unsafe extern "C" fn symGroup(mut x: K) -> K {
    let mut j: I = 0 as libc::c_int as I;
    let mut uk: K = newK(-(1 as libc::c_int) as I, (*x).n);
    if OOM_CD(0 as libc::c_int as I, uk, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut u: *mut I = ((*uk).k).as_mut_ptr() as *mut I;
    setS(1 as libc::c_int, 0 as libc::c_int as I);
    setS(2 as libc::c_int, 0 as libc::c_int as I);
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = (*x).n;
    while i < _i {
        let mut s: S = *(((*x).k).as_mut_ptr() as *mut S).offset(i as isize);
        if *(s as *mut I).offset(-(2 as libc::c_int) as isize) == 0 {
            *u.offset(j as isize) = s as L;
            j += 1;
            *(s as *mut I).offset(-(2 as libc::c_int) as isize) = j;
        }
        let ref mut fresh15 = *(s as *mut I).offset(-(1 as libc::c_int) as isize);
        *fresh15 += 1;
        *fresh15;
        i += 1;
        i;
    }
    let mut y: K = newK(0 as libc::c_int as I, j);
    if OOM_CD(0 as libc::c_int as I, y, uk, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_0: I = j;
    while i_0 < _i_0 {
        let mut s_0: S = *u.offset(i_0 as isize) as S;
        let mut z: K = newK(
            -(1 as libc::c_int) as I,
            *(s_0 as *mut I).offset(-(1 as libc::c_int) as isize),
        );
        if OOM_CD(0 as libc::c_int as I, z, y, uk, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        let ref mut fresh16 = *((*y).k).as_mut_ptr().offset(i_0 as isize);
        *fresh16 = z;
        *u.offset(i_0 as isize) = 0 as libc::c_int as I;
        i_0 += 1;
        i_0;
    }
    let mut i_1: I = 0 as libc::c_int as I;
    let mut _i_1: I = (*x).n;
    while i_1 < _i_1 {
        let mut s_1: S = *(((*x).k).as_mut_ptr() as *mut S).offset(i_1 as isize);
        let mut w: I = *(s_1 as *mut I).offset(-(2 as libc::c_int) as isize)
            - 1 as libc::c_int as libc::c_longlong;
        let mut z_0: K = *((*y).k).as_mut_ptr().offset(w as isize);
        let ref mut fresh17 = *u.offset(w as isize);
        let fresh18 = *fresh17;
        *fresh17 = *fresh17 + 1;
        *(((*z_0).k).as_mut_ptr() as *mut I).offset(fresh18 as isize) = i_1;
        i_1 += 1;
        i_1;
    }
    cd(uk);
    return y;
}
unsafe extern "C" fn groupI(mut x: K, mut y: K, mut n: I) -> K {
    let mut z: K = newK(0 as libc::c_int as I, n);
    if OOM_CD(0 as libc::c_int as I, z, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut c: *mut I = ((*y).k).as_mut_ptr() as *mut I;
    if n < 65537 as libc::c_int as libc::c_longlong {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = n;
        while i < _i {
            let mut v: K = newK(-(1 as libc::c_int) as I, *c.offset(i as isize));
            if OOM_CD(0 as libc::c_int as I, v, z, -(1 as libc::c_int) as V) == 0 {
                return 0 as K;
            }
            let ref mut fresh19 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh19 = v;
            *c.offset(i as isize) = 0 as libc::c_int as I;
            i += 1;
            i;
        }
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = (*x).n;
        while i_0 < _i_0 {
            let mut w: I = *(((*x).k).as_mut_ptr() as *mut I).offset(i_0 as isize);
            let mut v_0: K = *((*z).k).as_mut_ptr().offset(w as isize);
            let ref mut fresh20 = *c.offset(w as isize);
            let fresh21 = *fresh20;
            *fresh20 = *fresh20 + 1;
            *(((*v_0).k).as_mut_ptr() as *mut I).offset(fresh21 as isize) = i_0;
            i_0 += 1;
            i_0;
        }
    } else {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = n;
        while i_1 < _i_1 {
            let mut v_1: K = newK(-(1 as libc::c_int) as I, *c.offset(i_1 as isize));
            if OOM_CD(0 as libc::c_int as I, v_1, z, -(1 as libc::c_int) as V) == 0 {
                return 0 as K;
            }
            let ref mut fresh22 = *((*z).k).as_mut_ptr().offset(i_1 as isize);
            *fresh22 = v_1;
            (*v_1).n = 0 as libc::c_int as I;
            i_1 += 1;
            i_1;
        }
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = (*x).n;
        while i_2 < _i_2 {
            let mut w_0: I = *(((*x).k).as_mut_ptr() as *mut I).offset(i_2 as isize);
            let mut v_2: K = *((*z).k).as_mut_ptr().offset(w_0 as isize);
            let fresh23 = (*v_2).n;
            (*v_2).n = (*v_2).n + 1;
            *(((*v_2).k).as_mut_ptr() as *mut I).offset(fresh23 as isize) = i_2;
            i_2 += 1;
            i_2;
        }
    }
    cd(y);
    cd(x);
    return z;
}
unsafe extern "C" fn intGroup(mut x: K) -> K {
    hcinit();
    let mut j: I = 0 as libc::c_int as I;
    let mut h0: I = 0 as libc::c_int as I;
    let mut sa: I = 0 as libc::c_int as I;
    let mut m: uI = 0 as libc::c_int as uI;
    let mut h: K = newH((*x).n);
    if OOM_CD(0 as libc::c_int as I, h, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut ok: K = newK(-(1 as libc::c_int) as I, (*h).n);
    if OOM_CD(0 as libc::c_int as I, ok, h, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut o: *mut I = ((*ok).k).as_mut_ptr() as *mut I;
    let mut xok: K = newK(-(1 as libc::c_int) as I, (*x).n);
    if OOM_CD(0 as libc::c_int as I, xok, ok, h, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut xo: *mut I = ((*xok).k).as_mut_ptr() as *mut I;
    let mut ck: K = newK(-(1 as libc::c_int) as I, (*x).n);
    if OOM_CD(0 as libc::c_int as I, ck, xok, ok, h, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut c: *mut I = ((*ck).k).as_mut_ptr() as *mut I;
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = (*x).n;
    while i < _i {
        m |= *(((*x).k).as_mut_ptr() as *mut uI).offset(i as isize);
        i += 1;
        i;
    }
    if m != 0 {
        while m & 1 as libc::c_int as libc::c_ulonglong == 0 {
            m >>= 1 as libc::c_int;
            sa += 1;
            sa;
        }
    }
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*x).n;
    while i_0 < _i_0 {
        let mut v: uI = *(((*x).k).as_mut_ptr() as *mut uI).offset(i_0 as isize);
        if v == 0 {
            if h0 == 0 {
                j += 1;
                h0 = j;
            }
            *xo.offset(i_0 as isize) = h0 - 1 as libc::c_int as libc::c_longlong;
            let ref mut fresh24 = *c
                .offset((h0 - 1 as libc::c_int as libc::c_longlong) as isize);
            *fresh24 += 1;
            *fresh24;
        } else {
            v >>= sa;
            let mut u: uI = if m < (*h).n as libc::c_ulonglong {
                v
            } else {
                hc(v) as libc::c_ulonglong
            };
            let mut p: uI = 0;
            if hg(h, u, v as I, &mut p) == 0 {
                *(((*h).k).as_mut_ptr() as *mut I).offset(p as isize) = v as I;
                let fresh25 = j;
                j = j + 1;
                *o.offset(p as isize) = fresh25;
            }
            let mut w: I = *o.offset(p as isize);
            *xo.offset(i_0 as isize) = w;
            let ref mut fresh26 = *c.offset(w as isize);
            *fresh26 += 1;
            *fresh26;
        }
        i_0 += 1;
        i_0;
    }
    cd(ok);
    cd(h);
    let mut z: K = groupI(xok, ck, j);
    return z;
}
unsafe extern "C" fn listGroup(mut x: K) -> K {
    hcinit();
    let mut j: I = 0 as libc::c_int as I;
    let mut h: K = newH((*x).n);
    if OOM_CD(0 as libc::c_int as I, h, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut ok: K = newK(-(1 as libc::c_int) as I, (*h).n);
    if OOM_CD(0 as libc::c_int as I, ok, h, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut o: *mut I = ((*ok).k).as_mut_ptr() as *mut I;
    let mut xok: K = newK(-(1 as libc::c_int) as I, (*x).n);
    if OOM_CD(0 as libc::c_int as I, xok, ok, h, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut xo: *mut I = ((*xok).k).as_mut_ptr() as *mut I;
    let mut ck: K = newK(-(1 as libc::c_int) as I, (*x).n);
    if OOM_CD(0 as libc::c_int as I, ck, xok, ok, h, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut c: *mut I = ((*ck).k).as_mut_ptr() as *mut I;
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = (*x).n;
    while i < _i {
        let mut v: K = *((*x).k).as_mut_ptr().offset(i as isize);
        let mut u: uI = hcode(v);
        let mut p: uI = 0;
        if (shg(h, u, v, &mut p)).is_null() {
            let ref mut fresh27 = *((*h).k).as_mut_ptr().offset(p as isize);
            *fresh27 = v;
            let fresh28 = j;
            j = j + 1;
            *o.offset(p as isize) = fresh28;
        }
        let mut w: I = *o.offset(p as isize);
        *xo.offset(i as isize) = w;
        let ref mut fresh29 = *c.offset(w as isize);
        *fresh29 += 1;
        *fresh29;
        i += 1;
        i;
    }
    cd(ok);
    cd(h);
    let mut z: K = groupI(xok, ck, j);
    return z;
}
pub unsafe extern "C" fn group(mut x: K) -> K {
    let mut t: I = (*x).t;
    let mut n: I = (*x).n;
    if t > 0 as libc::c_int as libc::c_longlong {
        return kerr(b"rank\0" as *const u8 as *const libc::c_char);
    }
    let mut u: I = n;
    let mut g: *mut I = 0 as *mut I;
    let mut h: *mut I = 0 as *mut I;
    let mut z: K = 0 as K;
    let mut b: K = 0 as K;
    let mut c: K = 0 as K;
    match -t {
        0 => return listGroup(x),
        1 | 2 => return intGroup(x),
        3 => return charGroup(x),
        4 => return symGroup(x),
        _ => {}
    }
    b = grade_up(x);
    if OOM_CD(0 as libc::c_int as I, b, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    g = ((*b).k).as_mut_ptr() as *mut I;
    c = newK(-(1 as libc::c_int) as I, n);
    if OOM_CD(0 as libc::c_int as I, b, c, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    h = ((*c).k).as_mut_ptr() as *mut I;
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = n;
    while i < _i {
        *h.offset(*g.offset(i as isize) as isize) = i;
        i += 1;
        i;
    }
    if 0 as libc::c_int as libc::c_longlong == t {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = n - 1 as libc::c_int as libc::c_longlong;
        while i_0 < _i_0 {
            if matchI(
                *((*x).k)
                    .as_mut_ptr()
                    .offset(
                        *g
                            .offset(
                                (n - i_0 - 1 as libc::c_int as libc::c_longlong) as isize,
                            ) as isize,
                    ),
                *((*x).k)
                    .as_mut_ptr()
                    .offset(
                        *g
                            .offset(
                                (n - i_0 - 2 as libc::c_int as libc::c_longlong) as isize,
                            ) as isize,
                    ),
            ) != 0
            {
                u -= 1;
                u;
                let ref mut fresh30 = *g
                    .offset((n - i_0 - 1 as libc::c_int as libc::c_longlong) as isize);
                *fresh30 *= -(1 as libc::c_int) as libc::c_longlong;
            }
            i_0 += 1;
            i_0;
        }
    }
    z = newK(0 as libc::c_int as I, u);
    if OOM_CD(0 as libc::c_int as I, b, c, z, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut k: I = 0 as libc::c_int as I;
    let mut p: I = 0 as libc::c_int as I;
    let mut v: I = 0;
    while p < n && k < u {
        v = 1 as libc::c_int as I;
        while p + v < n
            && *g.offset((*h.offset(p as isize) + v) as isize)
                < 0 as libc::c_int as libc::c_longlong
        {
            v += 1;
            v;
        }
        let mut s: K = newK(-(1 as libc::c_int) as I, v);
        if OOM_CD(0 as libc::c_int as I, b, c, z, s, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = v;
        while i_1 < _i_1 {
            *(((*s).k).as_mut_ptr() as *mut I)
                .offset(
                    i_1 as isize,
                ) = if *g.offset((*h.offset(p as isize) + i_1) as isize)
                < 0 as libc::c_int as libc::c_longlong
            {
                -*g.offset((*h.offset(p as isize) + i_1) as isize)
            } else {
                *g.offset((*h.offset(p as isize) + i_1) as isize)
            };
            i_1 += 1;
            i_1;
        }
        let ref mut fresh31 = *((*z).k).as_mut_ptr().offset(k as isize);
        *fresh31 = s;
        loop {
            p += 1;
            if !(p < n
                && *g.offset(*h.offset(p as isize) as isize)
                    < 0 as libc::c_int as libc::c_longlong)
            {
                break;
            }
        }
        k += 1;
        k;
    }
    cd(b);
    cd(c);
    return z;
}
pub unsafe extern "C" fn VAT(mut i: I) -> I {
    return if 1 as libc::c_int as libc::c_longlong <= i
        && i <= 4 as libc::c_int as libc::c_longlong
    {
        i
    } else {
        0 as libc::c_int as libc::c_longlong
    };
}
pub unsafe extern "C" fn flip(mut a: K) -> K {
    let mut x: K = 0 as *mut k0;
    let mut i: I = 0;
    let mut p: I = (*a).n;
    let mut q: I = -(1 as libc::c_int) as I;
    if (*a).t != 0 || p == 0 {
        return ci(a);
    }
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i: I = p;
    while i_0 < _i {
        x = *((*a).k).as_mut_ptr().offset(i_0 as isize);
        if (*x).t < 1 as libc::c_int as libc::c_longlong {
            q = (*x).n;
        }
        i_0 += 1;
        i_0;
    }
    if -(1 as libc::c_int) as libc::c_longlong == q {
        return ci(a);
    }
    let mut i_1: I = 0 as libc::c_int as I;
    let mut _i_0: I = p;
    while i_1 < _i_0 {
        x = *((*a).k).as_mut_ptr().offset(i_1 as isize);
        if (*x).t < 1 as libc::c_int as libc::c_longlong && (*x).n != q {
            return kerr(b"length\0" as *const u8 as *const libc::c_char);
        }
        i_1 += 1;
        i_1;
    }
    let mut z: K = newK(0 as libc::c_int as I, q);
    i = 0 as libc::c_int as I;
    while i < q {
        let mut c: *mut K = ((*a).k).as_mut_ptr();
        let mut d: K = *c.offset(0 as libc::c_int as isize);
        let mut u: I = 0;
        let mut t: I = -if (if (*d).t != 0 {
            (*d).t
        } else {
            u = (**((*d).k).as_mut_ptr().offset(i as isize)).t;
            (if VAT(u) != 0 { u } else { 0 as libc::c_int as libc::c_longlong })
        }) < 0 as libc::c_int as libc::c_longlong
        {
            -if (*d).t != 0 {
                (*d).t
            } else {
                u = (**((*d).k).as_mut_ptr().offset(i as isize)).t;
                if VAT(u) != 0 { u } else { 0 as libc::c_int as libc::c_longlong }
            }
        } else if (*d).t != 0 {
            (*d).t
        } else {
            u = (**((*d).k).as_mut_ptr().offset(i as isize)).t;
            if VAT(u) != 0 { u } else { 0 as libc::c_int as libc::c_longlong }
        };
        let mut j: I = 0 as libc::c_int as I;
        let mut _j: I = p;
        while j < _j {
            d = *c.offset(j as isize);
            t = if t
                == -(if (if (*d).t != 0 {
                    (*d).t
                } else {
                    u = (**((*d).k).as_mut_ptr().offset(i as isize)).t;
                    (if VAT(u) != 0 { u } else { 0 as libc::c_int as libc::c_longlong })
                }) < 0 as libc::c_int as libc::c_longlong
                {
                    -(if (*d).t != 0 {
                        (*d).t
                    } else {
                        u = (**((*d).k).as_mut_ptr().offset(i as isize)).t;
                        (if VAT(u) != 0 {
                            u
                        } else {
                            0 as libc::c_int as libc::c_longlong
                        })
                    })
                } else {
                    (if (*d).t != 0 {
                        (*d).t
                    } else {
                        u = (**((*d).k).as_mut_ptr().offset(i as isize)).t;
                        (if VAT(u) != 0 {
                            u
                        } else {
                            0 as libc::c_int as libc::c_longlong
                        })
                    })
                })
            {
                t
            } else {
                0 as libc::c_int as libc::c_longlong
            };
            j += 1;
            j;
        }
        let ref mut fresh32 = *((*z).k).as_mut_ptr().offset(i as isize);
        *fresh32 = newK(t, p);
        let mut y: K = *fresh32;
        if -(4 as libc::c_int) as libc::c_longlong == t {
            let mut j_0: I = 0 as libc::c_int as I;
            let mut _j_0: I = p;
            while j_0 < _j_0 {
                d = *c.offset(j_0 as isize);
                let ref mut fresh33 = *(((*y).k).as_mut_ptr() as *mut S)
                    .offset(j_0 as isize);
                *fresh33 = if (*d).t != 0 {
                    *(((*d).k).as_mut_ptr() as *mut S).offset((i % (*d).n) as isize)
                } else {
                    *(((*(*((*d).k).as_mut_ptr().offset(i as isize) as K)).k)
                        .as_mut_ptr() as *mut S)
                };
                j_0 += 1;
                j_0;
            }
        } else if -(3 as libc::c_int) as libc::c_longlong == t {
            let mut j_1: I = 0 as libc::c_int as I;
            let mut _j_1: I = p;
            while j_1 < _j_1 {
                d = *c.offset(j_1 as isize);
                *(((*y).k).as_mut_ptr() as *mut C)
                    .offset(
                        j_1 as isize,
                    ) = (if (*d).t != 0 {
                    *(((*d).k).as_mut_ptr() as *mut C).offset((i % (*d).n) as isize)
                        as libc::c_int
                } else {
                    *(((*(*((*d).k).as_mut_ptr().offset(i as isize) as K)).k)
                        .as_mut_ptr() as *mut C) as libc::c_int
                }) as C;
                j_1 += 1;
                j_1;
            }
        } else if -(2 as libc::c_int) as libc::c_longlong == t {
            let mut j_2: I = 0 as libc::c_int as I;
            let mut _j_2: I = p;
            while j_2 < _j_2 {
                d = *c.offset(j_2 as isize);
                *(((*y).k).as_mut_ptr() as *mut F)
                    .offset(
                        j_2 as isize,
                    ) = if (*d).t != 0 {
                    *(((*d).k).as_mut_ptr() as *mut F).offset((i % (*d).n) as isize)
                } else {
                    *(((*(*((*d).k).as_mut_ptr().offset(i as isize) as K)).k)
                        .as_mut_ptr() as *mut F)
                };
                j_2 += 1;
                j_2;
            }
        } else if -(1 as libc::c_int) as libc::c_longlong == t {
            let mut j_3: I = 0 as libc::c_int as I;
            let mut _j_3: I = p;
            while j_3 < _j_3 {
                d = *c.offset(j_3 as isize);
                *(((*y).k).as_mut_ptr() as *mut I)
                    .offset(
                        j_3 as isize,
                    ) = if (*d).t != 0 {
                    *(((*d).k).as_mut_ptr() as *mut I).offset((i % (*d).n) as isize)
                } else {
                    *(((*(*((*d).k).as_mut_ptr().offset(i as isize) as K)).k)
                        .as_mut_ptr() as *mut I)
                };
                j_3 += 1;
                j_3;
            }
        } else if 0 as libc::c_int as libc::c_longlong == t {
            let mut j_4: I = 0 as libc::c_int as I;
            let mut _j_4: I = p;
            while j_4 < _j_4 {
                d = *c.offset(j_4 as isize);
                let ref mut fresh34 = *((*y).k).as_mut_ptr().offset(j_4 as isize);
                *fresh34 = itemAtIndex(d, i);
                j_4 += 1;
                j_4;
            }
        }
        i += 1;
        i;
    }
    return z;
}
pub unsafe extern "C" fn first(mut a: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    if -(4 as libc::c_int) as libc::c_longlong == at {
        return Ks(if an != 0 { *(((*a).k).as_mut_ptr() as *mut S) } else { LS });
    }
    if -(3 as libc::c_int) as libc::c_longlong == at {
        return Kc(
            (if an != 0 {
                *(((*a).k).as_mut_ptr() as *mut C) as libc::c_int
            } else {
                ' ' as i32
            }) as C,
        );
    }
    if -(2 as libc::c_int) as libc::c_longlong == at {
        return Kf(if an != 0 { *(((*a).k).as_mut_ptr() as *mut F) } else { 0.0f64 });
    }
    if -(1 as libc::c_int) as libc::c_longlong == at {
        return Ki(
            if an != 0 {
                *(((*a).k).as_mut_ptr() as *mut I)
            } else {
                0 as libc::c_int as libc::c_longlong
            },
        );
    }
    if 0 as libc::c_int as libc::c_longlong == at {
        return if an != 0 { ci(*((*a).k).as_mut_ptr()) } else { _n() };
    }
    return ci(a);
}
pub unsafe extern "C" fn last(mut a: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    if -(4 as libc::c_int) as libc::c_longlong == at {
        return Ks(
            if an != 0 {
                *(((*a).k).as_mut_ptr() as *mut S)
                    .offset((an - 1 as libc::c_int as libc::c_longlong) as isize)
            } else {
                LS
            },
        );
    }
    if -(3 as libc::c_int) as libc::c_longlong == at {
        return Kc(
            (if an != 0 {
                *(((*a).k).as_mut_ptr() as *mut C)
                    .offset((an - 1 as libc::c_int as libc::c_longlong) as isize)
                    as libc::c_int
            } else {
                ' ' as i32
            }) as C,
        );
    }
    if -(2 as libc::c_int) as libc::c_longlong == at {
        return Kf(
            if an != 0 {
                *(((*a).k).as_mut_ptr() as *mut F)
                    .offset((an - 1 as libc::c_int as libc::c_longlong) as isize)
            } else {
                0.0f64
            },
        );
    }
    if -(1 as libc::c_int) as libc::c_longlong == at {
        return Ki(
            if an != 0 {
                *(((*a).k).as_mut_ptr() as *mut I)
                    .offset((an - 1 as libc::c_int as libc::c_longlong) as isize)
            } else {
                0 as libc::c_int as libc::c_longlong
            },
        );
    }
    if 0 as libc::c_int as libc::c_longlong == at {
        return if an != 0 {
            ci(
                *((*a).k)
                    .as_mut_ptr()
                    .offset((an - 1 as libc::c_int as libc::c_longlong) as isize),
            )
        } else {
            _n()
        };
    }
    return ci(a);
}
unsafe extern "C" fn reshaper(
    mut a: K,
    mut b: K,
    mut d: I,
    mut f: I,
    mut p: *mut I,
) -> K {
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    let mut v: I = *(((*a).k).as_mut_ptr() as *mut I).offset(d as isize);
    let mut g: I = (if v == 0 || (*a).n == d + 1 as libc::c_int as libc::c_longlong {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as I;
    let mut t: I = if g != 0 && bt < 5 as libc::c_int as libc::c_longlong {
        -if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt }
    } else {
        0 as libc::c_int as libc::c_longlong
    };
    let mut n: I = if -(1 as libc::c_int) as libc::c_longlong == v { f } else { v };
    let mut z: K = newK(t, n);
    if z.is_null() {
        return 0 as K;
    }
    if g == 0 {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = n;
        while i < _i {
            let ref mut fresh35 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh35 = reshaper(a, b, d + 1 as libc::c_int as libc::c_longlong, f, p);
            i += 1;
            i;
        }
    } else if 4 as libc::c_int as libc::c_longlong
        == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = n;
        while i_0 < _i_0 {
            let ref mut fresh36 = *(((*z).k).as_mut_ptr() as *mut S)
                .offset(i_0 as isize);
            *fresh36 = if bn != 0 {
                *p += 1;
                *(((*b).k).as_mut_ptr() as *mut S).offset((*p % bn) as isize)
            } else {
                LS
            };
            i_0 += 1;
            i_0;
        }
    } else if 3 as libc::c_int as libc::c_longlong
        == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = n;
        while i_1 < _i_1 {
            *(((*z).k).as_mut_ptr() as *mut C)
                .offset(
                    i_1 as isize,
                ) = (if bn != 0 {
                *p += 1;
                *(((*b).k).as_mut_ptr() as *mut C).offset((*p % bn) as isize)
                    as libc::c_int
            } else {
                ' ' as i32
            }) as C;
            i_1 += 1;
            i_1;
        }
    } else if 2 as libc::c_int as libc::c_longlong
        == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = n;
        while i_2 < _i_2 {
            *(((*z).k).as_mut_ptr() as *mut F)
                .offset(
                    i_2 as isize,
                ) = if bn != 0 {
                *p += 1;
                *(((*b).k).as_mut_ptr() as *mut F).offset((*p % bn) as isize)
            } else {
                0.0f64
            };
            i_2 += 1;
            i_2;
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        let mut i_3: I = 0 as libc::c_int as I;
        let mut _i_3: I = n;
        while i_3 < _i_3 {
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    i_3 as isize,
                ) = if bn != 0 {
                *p += 1;
                *(((*b).k).as_mut_ptr() as *mut I).offset((*p % bn) as isize)
            } else {
                0 as libc::c_int as libc::c_longlong
            };
            i_3 += 1;
            i_3;
        }
    } else if 0 as libc::c_int as libc::c_longlong
        == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        let mut i_4: I = 0 as libc::c_int as I;
        let mut _i_4: I = n;
        while i_4 < _i_4 {
            let ref mut fresh37 = *((*z).k).as_mut_ptr().offset(i_4 as isize);
            *fresh37 = if bn != 0 {
                *p += 1;
                ci(*((*b).k).as_mut_ptr().offset((*p % bn) as isize))
            } else {
                _n()
            };
            i_4 += 1;
            i_4;
        }
    } else if 5 as libc::c_int as libc::c_longlong <= bt {
        let mut i_5: I = 0 as libc::c_int as I;
        let mut _i_5: I = n;
        while i_5 < _i_5 {
            let ref mut fresh38 = *((*z).k).as_mut_ptr().offset(i_5 as isize);
            *fresh38 = ci(b);
            i_5 += 1;
            i_5;
        }
    }
    return z;
}
pub unsafe extern "C" fn reshape(mut a: K, mut b: K) -> K {
    let mut an: I = (*a).n;
    let mut bn: I = (*b).n;
    if an == 0 {
        return first(b);
    }
    let mut ns: I = 0 as libc::c_int as I;
    let mut x: I = 0;
    let mut y: I = -(1 as libc::c_int) as I;
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = an;
    while i < _i {
        x = *(((*a).k).as_mut_ptr() as *mut I).offset(i as isize);
        if (50000000 as libc::c_int as libc::c_longlong) < x {
            return kerr(b"limit\0" as *const u8 as *const libc::c_char);
        }
        if 0 as libc::c_int as libc::c_longlong > x {
            ns -= x;
        }
        i += 1;
        i;
    }
    if ns < -(1 as libc::c_int) as libc::c_longlong {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    let mut p: I = 1 as libc::c_int as I;
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_0: I = an;
    while i_0 < _i_0 {
        p *= *(((*a).k).as_mut_ptr() as *mut I).offset(i_0 as isize);
        i_0 += 1;
        i_0;
    }
    if ns < 0 as libc::c_int as libc::c_longlong && (p == 0 || bn == 0 || bn % p != 0) {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    return reshaper(
        a,
        b,
        0 as libc::c_int as I,
        if p != 0 {
            if bn / p < 0 as libc::c_int as libc::c_longlong {
                -(bn / p)
            } else {
                bn / p
            }
        } else {
            0 as libc::c_int as libc::c_longlong
        },
        &mut y,
    );
}
pub unsafe extern "C" fn take(mut a: K, mut b: K) -> K {
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    let mut n: I = if *(((*a).k).as_mut_ptr() as *mut I)
        < 0 as libc::c_int as libc::c_longlong
    {
        -*(((*a).k).as_mut_ptr() as *mut I)
    } else {
        *(((*a).k).as_mut_ptr() as *mut I)
    };
    let mut m: I = if 1 as libc::c_int as libc::c_longlong > bn {
        1 as libc::c_int as libc::c_longlong
    } else {
        bn
    };
    let mut k: I = *(((*a).k).as_mut_ptr() as *mut I) % m;
    k = if k < 0 as libc::c_int as libc::c_longlong {
        bn + k
    } else {
        0 as libc::c_int as libc::c_longlong
    };
    let mut t: I = if bt < 5 as libc::c_int as libc::c_longlong {
        -if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt }
    } else {
        0 as libc::c_int as libc::c_longlong
    };
    let mut z: K = newK(t, n);
    if z.is_null() {
        return 0 as K;
    }
    if 4 as libc::c_int as libc::c_longlong
        == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = n;
        while i < _i {
            let ref mut fresh39 = *(((*z).k).as_mut_ptr() as *mut S).offset(i as isize);
            *fresh39 = if bn != 0 {
                *(((*b).k).as_mut_ptr() as *mut S).offset(((i + k) % m) as isize)
            } else {
                LS
            };
            i += 1;
            i;
        }
    } else if 3 as libc::c_int as libc::c_longlong
        == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = n;
        while i_0 < _i_0 {
            *(((*z).k).as_mut_ptr() as *mut C)
                .offset(
                    i_0 as isize,
                ) = (if bn != 0 {
                *(((*b).k).as_mut_ptr() as *mut C).offset(((i_0 + k) % m) as isize)
                    as libc::c_int
            } else {
                ' ' as i32
            }) as C;
            i_0 += 1;
            i_0;
        }
    } else if 2 as libc::c_int as libc::c_longlong
        == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = n;
        while i_1 < _i_1 {
            *(((*z).k).as_mut_ptr() as *mut F)
                .offset(
                    i_1 as isize,
                ) = if bn != 0 {
                *(((*b).k).as_mut_ptr() as *mut F).offset(((i_1 + k) % m) as isize)
            } else {
                0.0f64
            };
            i_1 += 1;
            i_1;
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = n;
        while i_2 < _i_2 {
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    i_2 as isize,
                ) = if bn != 0 {
                *(((*b).k).as_mut_ptr() as *mut I).offset(((i_2 + k) % m) as isize)
            } else {
                0 as libc::c_int as libc::c_longlong
            };
            i_2 += 1;
            i_2;
        }
    } else if 0 as libc::c_int as libc::c_longlong == bt {
        let mut i_3: I = 0 as libc::c_int as I;
        let mut _i_3: I = n;
        while i_3 < _i_3 {
            let ref mut fresh40 = *((*z).k).as_mut_ptr().offset(i_3 as isize);
            *fresh40 = if bn != 0 {
                ci(*((*b).k).as_mut_ptr().offset(((i_3 + k) % m) as isize))
            } else {
                _n()
            };
            i_3 += 1;
            i_3;
        }
    } else if 5 as libc::c_int as libc::c_longlong <= bt {
        let mut i_4: I = 0 as libc::c_int as I;
        let mut _i_4: I = n;
        while i_4 < _i_4 {
            let ref mut fresh41 = *((*z).k).as_mut_ptr().offset(i_4 as isize);
            *fresh41 = ci(b);
            i_4 += 1;
            i_4;
        }
    }
    return demote(z);
}
pub unsafe extern "C" fn take_reshape(mut a: K, mut b: K) -> K {
    if (*a).n != 0
        && 1 as libc::c_int as libc::c_longlong
            != (if (*a).t < 0 as libc::c_int as libc::c_longlong {
                -(*a).t
            } else {
                (*a).t
            })
    {
        return kerr(b"int\0" as *const u8 as *const libc::c_char);
    }
    return if (0 as libc::c_int as libc::c_longlong) < (*a).t {
        take(a, b)
    } else {
        reshape(a, b)
    };
}
unsafe extern "C" fn shapeCheck(mut a: K, mut p: K, mut d: I) {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    if at > 0 as libc::c_int as libc::c_longlong
        || an != *(((*p).k).as_mut_ptr() as *mut I).offset(d as isize)
    {
        *(((*p).k).as_mut_ptr() as *mut I).offset(d as isize) = -(1 as libc::c_int) as I;
    } else if at != 0 && d < (*p).n - 1 as libc::c_int as libc::c_longlong {
        *(((*p).k).as_mut_ptr() as *mut I)
            .offset(
                (d + 1 as libc::c_int as libc::c_longlong) as isize,
            ) = -(1 as libc::c_int) as I;
    } else if at == 0 && an != 0
        && *(((*p).k).as_mut_ptr() as *mut I).offset(d as isize)
            != -(1 as libc::c_int) as libc::c_longlong
        && d < (*p).n - 1 as libc::c_int as libc::c_longlong
    {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = an;
        while i < _i {
            shapeCheck(
                *((*a).k).as_mut_ptr().offset(i as isize),
                p,
                d + 1 as libc::c_int as libc::c_longlong,
            );
            i += 1;
            i;
        }
    }
}
unsafe extern "C" fn firstDepth(mut x: K) -> I {
    return if (*x).t == 0 && (*x).n != 0 {
        1 as libc::c_int as libc::c_longlong + firstDepth(*((*x).k).as_mut_ptr())
    } else {
        (if (*x).t > 0 as libc::c_int as libc::c_longlong {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        }) as libc::c_longlong
    };
}
pub unsafe extern "C" fn shape(mut a: K) -> K {
    let mut b: K = a;
    let mut p: K = newK(-(1 as libc::c_int) as I, firstDepth(a));
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = (*p).n;
    while i < _i {
        *(((*p).k).as_mut_ptr() as *mut I).offset(i as isize) = (*b).n;
        if i < _i - 1 as libc::c_int as libc::c_longlong {
            b = *((*b).k).as_mut_ptr();
        }
        i += 1;
        i;
    }
    shapeCheck(a, p, 0 as libc::c_int as I);
    let mut n: I = 0 as libc::c_int as I;
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*p).n;
    while i_0 < _i_0 {
        if *(((*p).k).as_mut_ptr() as *mut I).offset(i_0 as isize)
            == -(1 as libc::c_int) as libc::c_longlong
        {
            break;
        }
        n += 1;
        n;
        i_0 += 1;
        i_0;
    }
    let mut z: K = newK(-(1 as libc::c_int) as I, n);
    let mut i_1: I = 0 as libc::c_int as I;
    let mut _i_1: I = n;
    while i_1 < _i_1 {
        *(((*z).k).as_mut_ptr() as *mut I)
            .offset(
                i_1 as isize,
            ) = *(((*p).k).as_mut_ptr() as *mut I).offset(i_1 as isize);
        i_1 += 1;
        i_1;
    }
    cd(p);
    return z;
}
pub unsafe extern "C" fn rotate(mut a: K, mut b: K) -> K {
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    let mut r: I = *(((*a).k).as_mut_ptr() as *mut I)
        % (if 1 as libc::c_int as libc::c_longlong > bn {
            1 as libc::c_int as libc::c_longlong
        } else {
            bn
        });
    r = if r > 0 as libc::c_int as libc::c_longlong { r } else { bn + r };
    let mut z: K = newK(bt, bn);
    if z.is_null() {
        return 0 as K;
    }
    if -(4 as libc::c_int) as libc::c_longlong == bt {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = bn;
        while i < _i {
            let ref mut fresh42 = *(((*z).k).as_mut_ptr() as *mut S).offset(i as isize);
            *fresh42 = *(((*b).k).as_mut_ptr() as *mut S)
                .offset(((i + r) % bn) as isize);
            i += 1;
            i;
        }
    } else if -(3 as libc::c_int) as libc::c_longlong == bt {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = bn;
        while i_0 < _i_0 {
            *(((*z).k).as_mut_ptr() as *mut C)
                .offset(
                    i_0 as isize,
                ) = *(((*b).k).as_mut_ptr() as *mut C).offset(((i_0 + r) % bn) as isize);
            i_0 += 1;
            i_0;
        }
    } else if -(2 as libc::c_int) as libc::c_longlong == bt {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = bn;
        while i_1 < _i_1 {
            *(((*z).k).as_mut_ptr() as *mut F)
                .offset(
                    i_1 as isize,
                ) = *(((*b).k).as_mut_ptr() as *mut F).offset(((i_1 + r) % bn) as isize);
            i_1 += 1;
            i_1;
        }
    } else if -(1 as libc::c_int) as libc::c_longlong == bt {
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = bn;
        while i_2 < _i_2 {
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    i_2 as isize,
                ) = *(((*b).k).as_mut_ptr() as *mut I).offset(((i_2 + r) % bn) as isize);
            i_2 += 1;
            i_2;
        }
    } else if 0 as libc::c_int as libc::c_longlong == bt {
        let mut i_3: I = 0 as libc::c_int as I;
        let mut _i_3: I = bn;
        while i_3 < _i_3 {
            let ref mut fresh43 = *((*z).k).as_mut_ptr().offset(i_3 as isize);
            *fresh43 = ci(*((*b).k).as_mut_ptr().offset(((i_3 + r) % bn) as isize));
            i_3 += 1;
            i_3;
        }
    }
    return z;
}
#[export_name = "drop"]
pub unsafe extern "C" fn drop_0(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    if 1 as libc::c_int as libc::c_longlong != at {
        return kerr(b"int\0" as *const u8 as *const libc::c_char);
    }
    if bt > 0 as libc::c_int as libc::c_longlong {
        return ci(b);
    }
    let mut v: I = *(((*a).k).as_mut_ptr() as *mut I);
    let mut zn: I = if 0 as libc::c_int as libc::c_longlong
        > bn - (if v < 0 as libc::c_int as libc::c_longlong { -v } else { v })
    {
        0 as libc::c_int as libc::c_longlong
    } else {
        bn - (if v < 0 as libc::c_int as libc::c_longlong { -v } else { v })
    };
    let mut z: K = newK(bt, zn);
    if z.is_null() {
        return 0 as K;
    }
    let mut c: I = if v < 1 as libc::c_int as libc::c_longlong {
        0 as libc::c_int as libc::c_longlong
    } else if v < bn {
        v
    } else {
        bn
    };
    if -(4 as libc::c_int) as libc::c_longlong == bt {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = zn;
        while i < _i {
            let ref mut fresh44 = *(((*z).k).as_mut_ptr() as *mut S).offset(i as isize);
            *fresh44 = *(((*b).k).as_mut_ptr() as *mut S).offset((i + c) as isize);
            i += 1;
            i;
        }
    } else if -(3 as libc::c_int) as libc::c_longlong == bt {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = zn;
        while i_0 < _i_0 {
            *(((*z).k).as_mut_ptr() as *mut C)
                .offset(
                    i_0 as isize,
                ) = *(((*b).k).as_mut_ptr() as *mut C).offset((i_0 + c) as isize);
            i_0 += 1;
            i_0;
        }
    } else if -(2 as libc::c_int) as libc::c_longlong == bt {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = zn;
        while i_1 < _i_1 {
            *(((*z).k).as_mut_ptr() as *mut F)
                .offset(
                    i_1 as isize,
                ) = *(((*b).k).as_mut_ptr() as *mut F).offset((i_1 + c) as isize);
            i_1 += 1;
            i_1;
        }
    } else if -(1 as libc::c_int) as libc::c_longlong == bt {
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = zn;
        while i_2 < _i_2 {
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    i_2 as isize,
                ) = *(((*b).k).as_mut_ptr() as *mut I).offset((i_2 + c) as isize);
            i_2 += 1;
            i_2;
        }
    } else if 0 as libc::c_int as libc::c_longlong == bt {
        let mut i_3: I = 0 as libc::c_int as I;
        let mut _i_3: I = zn;
        while i_3 < _i_3 {
            let ref mut fresh45 = *((*z).k).as_mut_ptr().offset(i_3 as isize);
            *fresh45 = ci(*((*b).k).as_mut_ptr().offset((i_3 + c) as isize));
            i_3 += 1;
            i_3;
        }
    }
    return demote(z);
}
pub unsafe extern "C" fn cut(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    if -(1 as libc::c_int) as libc::c_longlong != at {
        return kerr(b"int\0" as *const u8 as *const libc::c_char);
    }
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = an;
    while i < _i {
        let mut x: I = *(((*a).k).as_mut_ptr() as *mut I).offset(i as isize);
        if x < 0 as libc::c_int as libc::c_longlong
            || x
                < *(((*a).k).as_mut_ptr() as *mut I)
                    .offset(
                        (if i > 0 as libc::c_int as libc::c_longlong {
                            i - 1 as libc::c_int as libc::c_longlong
                        } else {
                            0 as libc::c_int as libc::c_longlong
                        }) as isize,
                    )
        {
            return kerr(b"domain\0" as *const u8 as *const libc::c_char)
        } else if x > bn {
            return kerr(b"length\0" as *const u8 as *const libc::c_char)
        }
        i += 1;
        i;
    }
    let mut z: K = newK(0 as libc::c_int as I, an);
    if z.is_null() {
        return 0 as K;
    }
    let mut zn: I = (*z).n;
    if -(4 as libc::c_int) as libc::c_longlong == bt {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = zn;
        while i_0 < _i_0 {
            let mut x_0: I = *(((*a).k).as_mut_ptr() as *mut I).offset(i_0 as isize);
            let mut y: I = if i_0 == (*z).n - 1 as libc::c_int as libc::c_longlong {
                bn
            } else {
                *(((*a).k).as_mut_ptr() as *mut I)
                    .offset((i_0 + 1 as libc::c_int as libc::c_longlong) as isize)
            };
            let mut w: K = newK(-(4 as libc::c_int) as I, y - x_0);
            let mut j: I = 0 as libc::c_int as I;
            let mut _j: I = (*w).n;
            while j < _j {
                let ref mut fresh46 = *(((*w).k).as_mut_ptr() as *mut S)
                    .offset(j as isize);
                *fresh46 = *(((*b).k).as_mut_ptr() as *mut S).offset((x_0 + j) as isize);
                j += 1;
                j;
            }
            let ref mut fresh47 = *((*z).k).as_mut_ptr().offset(i_0 as isize);
            *fresh47 = w;
            i_0 += 1;
            i_0;
        }
    } else if -(3 as libc::c_int) as libc::c_longlong == bt {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = zn;
        while i_1 < _i_1 {
            let mut x_1: I = *(((*a).k).as_mut_ptr() as *mut I).offset(i_1 as isize);
            let mut y_0: I = if i_1 == (*z).n - 1 as libc::c_int as libc::c_longlong {
                bn
            } else {
                *(((*a).k).as_mut_ptr() as *mut I)
                    .offset((i_1 + 1 as libc::c_int as libc::c_longlong) as isize)
            };
            let mut w_0: K = newK(-(3 as libc::c_int) as I, y_0 - x_1);
            let mut j_0: I = 0 as libc::c_int as I;
            let mut _j_0: I = (*w_0).n;
            while j_0 < _j_0 {
                *(((*w_0).k).as_mut_ptr() as *mut C)
                    .offset(
                        j_0 as isize,
                    ) = *(((*b).k).as_mut_ptr() as *mut C).offset((x_1 + j_0) as isize);
                j_0 += 1;
                j_0;
            }
            let ref mut fresh48 = *((*z).k).as_mut_ptr().offset(i_1 as isize);
            *fresh48 = w_0;
            i_1 += 1;
            i_1;
        }
    } else if -(2 as libc::c_int) as libc::c_longlong == bt {
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = zn;
        while i_2 < _i_2 {
            let mut x_2: I = *(((*a).k).as_mut_ptr() as *mut I).offset(i_2 as isize);
            let mut y_1: I = if i_2 == (*z).n - 1 as libc::c_int as libc::c_longlong {
                bn
            } else {
                *(((*a).k).as_mut_ptr() as *mut I)
                    .offset((i_2 + 1 as libc::c_int as libc::c_longlong) as isize)
            };
            let mut w_1: K = newK(-(2 as libc::c_int) as I, y_1 - x_2);
            let mut j_1: I = 0 as libc::c_int as I;
            let mut _j_1: I = (*w_1).n;
            while j_1 < _j_1 {
                *(((*w_1).k).as_mut_ptr() as *mut F)
                    .offset(
                        j_1 as isize,
                    ) = *(((*b).k).as_mut_ptr() as *mut F).offset((x_2 + j_1) as isize);
                j_1 += 1;
                j_1;
            }
            let ref mut fresh49 = *((*z).k).as_mut_ptr().offset(i_2 as isize);
            *fresh49 = w_1;
            i_2 += 1;
            i_2;
        }
    } else if -(1 as libc::c_int) as libc::c_longlong == bt {
        let mut i_3: I = 0 as libc::c_int as I;
        let mut _i_3: I = zn;
        while i_3 < _i_3 {
            let mut x_3: I = *(((*a).k).as_mut_ptr() as *mut I).offset(i_3 as isize);
            let mut y_2: I = if i_3 == (*z).n - 1 as libc::c_int as libc::c_longlong {
                bn
            } else {
                *(((*a).k).as_mut_ptr() as *mut I)
                    .offset((i_3 + 1 as libc::c_int as libc::c_longlong) as isize)
            };
            let mut w_2: K = newK(-(1 as libc::c_int) as I, y_2 - x_3);
            let mut j_2: I = 0 as libc::c_int as I;
            let mut _j_2: I = (*w_2).n;
            while j_2 < _j_2 {
                *(((*w_2).k).as_mut_ptr() as *mut I)
                    .offset(
                        j_2 as isize,
                    ) = *(((*b).k).as_mut_ptr() as *mut I).offset((x_3 + j_2) as isize);
                j_2 += 1;
                j_2;
            }
            let ref mut fresh50 = *((*z).k).as_mut_ptr().offset(i_3 as isize);
            *fresh50 = w_2;
            i_3 += 1;
            i_3;
        }
    } else if 0 as libc::c_int as libc::c_longlong == bt {
        let mut i_4: I = 0;
        i_4 = 0 as libc::c_int as I;
        while i_4 < zn {
            let mut x_4: I = *(((*a).k).as_mut_ptr() as *mut I).offset(i_4 as isize);
            let mut y_3: I = if i_4 == (*z).n - 1 as libc::c_int as libc::c_longlong {
                bn
            } else {
                *(((*a).k).as_mut_ptr() as *mut I)
                    .offset((i_4 + 1 as libc::c_int as libc::c_longlong) as isize)
            };
            let mut sn: I = y_3 - x_4;
            let mut t: I = bt;
            if sn != 0 && x_4 < bn {
                t = (**((*b).k).as_mut_ptr().offset(x_4 as isize)).t;
            }
            let mut j_3: I = 0 as libc::c_int as I;
            let mut _j_3: I = sn;
            while j_3 < _j_3 {
                if t != (**((*b).k).as_mut_ptr().offset((x_4 + j_3) as isize)).t {
                    t = 0 as libc::c_int as I;
                    break;
                } else {
                    j_3 += 1;
                    j_3;
                }
            }
            t = -if 0 as libc::c_int as libc::c_longlong > t {
                0 as libc::c_int as libc::c_longlong
            } else {
                t
            };
            let mut s: K = newK(t, sn);
            if -(4 as libc::c_int) as libc::c_longlong == t {
                let mut j_4: I = 0 as libc::c_int as I;
                let mut _j_4: I = sn;
                while j_4 < _j_4 {
                    let ref mut fresh51 = *(((*s).k).as_mut_ptr() as *mut S)
                        .offset(j_4 as isize);
                    *fresh51 = *(((*(*((*b).k).as_mut_ptr().offset((x_4 + j_4) as isize)
                        as K))
                        .k)
                        .as_mut_ptr() as *mut S);
                    j_4 += 1;
                    j_4;
                }
            } else if -(3 as libc::c_int) as libc::c_longlong == t {
                let mut j_5: I = 0 as libc::c_int as I;
                let mut _j_5: I = sn;
                while j_5 < _j_5 {
                    *(((*s).k).as_mut_ptr() as *mut C)
                        .offset(
                            j_5 as isize,
                        ) = *(((*(*((*b).k).as_mut_ptr().offset((x_4 + j_5) as isize)
                        as K))
                        .k)
                        .as_mut_ptr() as *mut C);
                    j_5 += 1;
                    j_5;
                }
            } else if -(2 as libc::c_int) as libc::c_longlong == t {
                let mut j_6: I = 0 as libc::c_int as I;
                let mut _j_6: I = sn;
                while j_6 < _j_6 {
                    *(((*s).k).as_mut_ptr() as *mut F)
                        .offset(
                            j_6 as isize,
                        ) = *(((*(*((*b).k).as_mut_ptr().offset((x_4 + j_6) as isize)
                        as K))
                        .k)
                        .as_mut_ptr() as *mut F);
                    j_6 += 1;
                    j_6;
                }
            } else if -(1 as libc::c_int) as libc::c_longlong == t {
                let mut j_7: I = 0 as libc::c_int as I;
                let mut _j_7: I = sn;
                while j_7 < _j_7 {
                    *(((*s).k).as_mut_ptr() as *mut I)
                        .offset(
                            j_7 as isize,
                        ) = *(((*(*((*b).k).as_mut_ptr().offset((x_4 + j_7) as isize)
                        as K))
                        .k)
                        .as_mut_ptr() as *mut I);
                    j_7 += 1;
                    j_7;
                }
            } else if 0 as libc::c_int as libc::c_longlong == t {
                let mut j_8: I = 0 as libc::c_int as I;
                let mut _j_8: I = sn;
                while j_8 < _j_8 {
                    let ref mut fresh52 = *((*s).k).as_mut_ptr().offset(j_8 as isize);
                    *fresh52 = ci(*((*b).k).as_mut_ptr().offset((x_4 + j_8) as isize));
                    j_8 += 1;
                    j_8;
                }
            }
            let ref mut fresh53 = *((*z).k).as_mut_ptr().offset(i_4 as isize);
            *fresh53 = s;
            i_4 += 1;
            i_4;
        }
    }
    return z;
}
pub unsafe extern "C" fn drop_cut(mut a: K, mut b: K) -> K {
    if 1 as libc::c_int as libc::c_longlong
        != (if (*a).t < 0 as libc::c_int as libc::c_longlong { -(*a).t } else { (*a).t })
        || -(1 as libc::c_int) as libc::c_longlong == (*a).t
            && (0 as libc::c_int as libc::c_longlong) < (*b).t
    {
        return kerr(b"int\0" as *const u8 as *const libc::c_char);
    }
    return if 1 as libc::c_int as libc::c_longlong == (*a).t {
        drop_0(a, b)
    } else {
        cut(a, b)
    };
}
#[export_name = "where"]
pub unsafe extern "C" fn where_0(mut x: K) -> K {
    if (*x).n == 0 {
        return newK(-(1 as libc::c_int) as I, 0 as libc::c_int as I);
    }
    if 1 as libc::c_int as libc::c_longlong
        != (if (*x).t < 0 as libc::c_int as libc::c_longlong { -(*x).t } else { (*x).t })
    {
        return kerr(b"int\0" as *const u8 as *const libc::c_char);
    }
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = (*x).n;
    while i < _i {
        if *(((*x).k).as_mut_ptr() as *mut I).offset(i as isize)
            == 9223372036854775807 as libc::c_longlong
        {
            return kerr(b"int\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
    let mut zn: I = 0 as libc::c_int as I;
    let mut y: I = 0;
    let mut j: I = 0;
    let mut t: I = 0 as libc::c_int as I;
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*x).n;
    while i_0 < _i_0 {
        y = *(((*x).k).as_mut_ptr() as *mut I).offset(i_0 as isize);
        if !(y < 0 as libc::c_int as libc::c_longlong) {
            zn += y;
        }
        i_0 += 1;
        i_0;
    }
    let mut z: K = newK(-(1 as libc::c_int) as I, zn);
    if z.is_null() {
        return 0 as K;
    }
    let mut i_1: I = 0 as libc::c_int as I;
    let mut _i_1: I = (*x).n;
    while i_1 < _i_1 {
        j = 0 as libc::c_int as I;
        while j < *(((*x).k).as_mut_ptr() as *mut I).offset(i_1 as isize) {
            let fresh54 = t;
            t = t + 1;
            *(((*z).k).as_mut_ptr() as *mut I).offset(fresh54 as isize) = i_1;
            j += 1;
            j;
        }
        i_1 += 1;
        i_1;
    }
    return z;
}
pub unsafe extern "C" fn reverse(mut a: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    if (0 as libc::c_int as libc::c_longlong) < at {
        return ci(a);
    }
    let mut z: K = 0 as *mut k0;
    if 1 as libc::c_int as libc::c_longlong == rc(a) {
        let mut n: I = an >> 1 as libc::c_int;
        z = ci(a);
        if n == 0 {
            return z;
        }
        if -(4 as libc::c_int) as libc::c_longlong == at {
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = n;
            while i < _i {
                let mut _t: S = *(((*z).k).as_mut_ptr() as *mut S).offset(i as isize);
                let ref mut fresh55 = *(((*z).k).as_mut_ptr() as *mut S)
                    .offset(i as isize);
                *fresh55 = *(((*z).k).as_mut_ptr() as *mut S)
                    .offset((an - i - 1 as libc::c_int as libc::c_longlong) as isize);
                let ref mut fresh56 = *(((*z).k).as_mut_ptr() as *mut S)
                    .offset((an - i - 1 as libc::c_int as libc::c_longlong) as isize);
                *fresh56 = _t;
                i += 1;
                i;
            }
        } else if -(3 as libc::c_int) as libc::c_longlong == at {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = n;
            while i_0 < _i_0 {
                let mut _t_0: C = *(((*z).k).as_mut_ptr() as *mut C)
                    .offset(i_0 as isize);
                *(((*z).k).as_mut_ptr() as *mut C)
                    .offset(
                        i_0 as isize,
                    ) = *(((*z).k).as_mut_ptr() as *mut C)
                    .offset((an - i_0 - 1 as libc::c_int as libc::c_longlong) as isize);
                *(((*z).k).as_mut_ptr() as *mut C)
                    .offset(
                        (an - i_0 - 1 as libc::c_int as libc::c_longlong) as isize,
                    ) = _t_0;
                i_0 += 1;
                i_0;
            }
        } else if -(2 as libc::c_int) as libc::c_longlong == at {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = n;
            while i_1 < _i_1 {
                let mut _t_1: F = *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(i_1 as isize);
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_1 as isize,
                    ) = *(((*z).k).as_mut_ptr() as *mut F)
                    .offset((an - i_1 - 1 as libc::c_int as libc::c_longlong) as isize);
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        (an - i_1 - 1 as libc::c_int as libc::c_longlong) as isize,
                    ) = _t_1;
                i_1 += 1;
                i_1;
            }
        } else if -(1 as libc::c_int) as libc::c_longlong == at {
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = n;
            while i_2 < _i_2 {
                let mut _t_2: I = *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(i_2 as isize);
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_2 as isize,
                    ) = *(((*z).k).as_mut_ptr() as *mut I)
                    .offset((an - i_2 - 1 as libc::c_int as libc::c_longlong) as isize);
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        (an - i_2 - 1 as libc::c_int as libc::c_longlong) as isize,
                    ) = _t_2;
                i_2 += 1;
                i_2;
            }
        } else if 0 as libc::c_int as libc::c_longlong == at {
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = n;
            while i_3 < _i_3 {
                let mut _t_3: K = *((*z).k).as_mut_ptr().offset(i_3 as isize);
                let ref mut fresh57 = *((*z).k).as_mut_ptr().offset(i_3 as isize);
                *fresh57 = *((*z).k)
                    .as_mut_ptr()
                    .offset((an - i_3 - 1 as libc::c_int as libc::c_longlong) as isize);
                let ref mut fresh58 = *((*z).k)
                    .as_mut_ptr()
                    .offset((an - i_3 - 1 as libc::c_int as libc::c_longlong) as isize);
                *fresh58 = _t_3;
                i_3 += 1;
                i_3;
            }
        }
    } else {
        z = newK(at, an);
        if z.is_null() {
            return 0 as K;
        }
        if -(4 as libc::c_int) as libc::c_longlong == at {
            let mut i_4: I = 0 as libc::c_int as I;
            let mut _i_4: I = an;
            while i_4 < _i_4 {
                let ref mut fresh59 = *(((*z).k).as_mut_ptr() as *mut S)
                    .offset(i_4 as isize);
                *fresh59 = *(((*a).k).as_mut_ptr() as *mut S)
                    .offset((an - i_4 - 1 as libc::c_int as libc::c_longlong) as isize);
                i_4 += 1;
                i_4;
            }
        } else if -(3 as libc::c_int) as libc::c_longlong == at {
            let mut i_5: I = 0 as libc::c_int as I;
            let mut _i_5: I = an;
            while i_5 < _i_5 {
                *(((*z).k).as_mut_ptr() as *mut C)
                    .offset(
                        i_5 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut C)
                    .offset((an - i_5 - 1 as libc::c_int as libc::c_longlong) as isize);
                i_5 += 1;
                i_5;
            }
        } else if -(2 as libc::c_int) as libc::c_longlong == at {
            let mut i_6: I = 0 as libc::c_int as I;
            let mut _i_6: I = an;
            while i_6 < _i_6 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_6 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F)
                    .offset((an - i_6 - 1 as libc::c_int as libc::c_longlong) as isize);
                i_6 += 1;
                i_6;
            }
        } else if -(1 as libc::c_int) as libc::c_longlong == at {
            let mut i_7: I = 0 as libc::c_int as I;
            let mut _i_7: I = an;
            while i_7 < _i_7 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_7 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I)
                    .offset((an - i_7 - 1 as libc::c_int as libc::c_longlong) as isize);
                i_7 += 1;
                i_7;
            }
        } else if 0 as libc::c_int as libc::c_longlong == at {
            let mut i_8: I = 0 as libc::c_int as I;
            let mut _i_8: I = an;
            while i_8 < _i_8 {
                let ref mut fresh60 = *((*z).k).as_mut_ptr().offset(i_8 as isize);
                *fresh60 = ci(
                    *((*a).k)
                        .as_mut_ptr()
                        .offset(
                            (an - i_8 - 1 as libc::c_int as libc::c_longlong) as isize,
                        ),
                );
                i_8 += 1;
                i_8;
            }
        }
    }
    return z;
}
pub unsafe extern "C" fn countI(mut x: K) -> I {
    return if (*x).t > 0 as libc::c_int as libc::c_longlong {
        1 as libc::c_int as libc::c_longlong
    } else {
        (*x).n
    };
}
pub unsafe extern "C" fn count(mut x: K) -> K {
    return Ki(countI(x));
}
pub unsafe extern "C" fn join(mut x: K, mut y: K) -> K {
    let mut xk: I = countI(x);
    let mut yk: I = countI(y);
    let mut zt: I = 0 as libc::c_int as I;
    if (if (*x).t < 0 as libc::c_int as libc::c_longlong { -(*x).t } else { (*x).t })
        == (if (*y).t < 0 as libc::c_int as libc::c_longlong { -(*y).t } else { (*y).t })
    {
        zt = -if (*x).t < 0 as libc::c_int as libc::c_longlong {
            -(*x).t
        } else {
            (*x).t
        };
    }
    if xk == 0 {
        zt = -if (*y).t < 0 as libc::c_int as libc::c_longlong {
            -(*y).t
        } else {
            (*y).t
        };
    } else if yk == 0 {
        zt = -if (*x).t < 0 as libc::c_int as libc::c_longlong {
            -(*x).t
        } else {
            (*x).t
        };
    }
    if zt < -(4 as libc::c_int) as libc::c_longlong {
        zt = 0 as libc::c_int as I;
    }
    let mut zn: I = xk + yk;
    let mut z: K = newK(zt, zn);
    if z.is_null() {
        return 0 as K;
    }
    if -(4 as libc::c_int) as libc::c_longlong == zt {
        memcpy(
            ((*z).k).as_mut_ptr() as *mut S as *mut libc::c_void,
            ((*x).k).as_mut_ptr() as *mut S as *const libc::c_void,
            (xk as libc::c_ulonglong)
                .wrapping_mul(
                    ::std::mem::size_of::<S>() as libc::c_ulong as libc::c_ulonglong,
                ) as libc::c_ulong,
        );
        memcpy(
            (((*z).k).as_mut_ptr() as *mut S).offset(xk as isize) as *mut libc::c_void,
            ((*y).k).as_mut_ptr() as *mut S as *const libc::c_void,
            (yk as libc::c_ulonglong)
                .wrapping_mul(
                    ::std::mem::size_of::<S>() as libc::c_ulong as libc::c_ulonglong,
                ) as libc::c_ulong,
        );
    } else if -(3 as libc::c_int) as libc::c_longlong == zt {
        memcpy(
            ((*z).k).as_mut_ptr() as *mut C as *mut libc::c_void,
            ((*x).k).as_mut_ptr() as *mut C as *const libc::c_void,
            (xk as libc::c_ulonglong)
                .wrapping_mul(
                    ::std::mem::size_of::<C>() as libc::c_ulong as libc::c_ulonglong,
                ) as libc::c_ulong,
        );
        memcpy(
            (((*z).k).as_mut_ptr() as *mut C).offset(xk as isize) as *mut libc::c_void,
            ((*y).k).as_mut_ptr() as *mut C as *const libc::c_void,
            (yk as libc::c_ulonglong)
                .wrapping_mul(
                    ::std::mem::size_of::<C>() as libc::c_ulong as libc::c_ulonglong,
                ) as libc::c_ulong,
        );
    } else if -(2 as libc::c_int) as libc::c_longlong == zt {
        memcpy(
            ((*z).k).as_mut_ptr() as *mut F as *mut libc::c_void,
            ((*x).k).as_mut_ptr() as *mut F as *const libc::c_void,
            (xk as libc::c_ulonglong)
                .wrapping_mul(
                    ::std::mem::size_of::<F>() as libc::c_ulong as libc::c_ulonglong,
                ) as libc::c_ulong,
        );
        memcpy(
            (((*z).k).as_mut_ptr() as *mut F).offset(xk as isize) as *mut libc::c_void,
            ((*y).k).as_mut_ptr() as *mut F as *const libc::c_void,
            (yk as libc::c_ulonglong)
                .wrapping_mul(
                    ::std::mem::size_of::<F>() as libc::c_ulong as libc::c_ulonglong,
                ) as libc::c_ulong,
        );
    } else if -(1 as libc::c_int) as libc::c_longlong == zt {
        memcpy(
            ((*z).k).as_mut_ptr() as *mut I as *mut libc::c_void,
            ((*x).k).as_mut_ptr() as *mut I as *const libc::c_void,
            (xk as libc::c_ulonglong)
                .wrapping_mul(
                    ::std::mem::size_of::<I>() as libc::c_ulong as libc::c_ulonglong,
                ) as libc::c_ulong,
        );
        memcpy(
            (((*z).k).as_mut_ptr() as *mut I).offset(xk as isize) as *mut libc::c_void,
            ((*y).k).as_mut_ptr() as *mut I as *const libc::c_void,
            (yk as libc::c_ulonglong)
                .wrapping_mul(
                    ::std::mem::size_of::<I>() as libc::c_ulong as libc::c_ulonglong,
                ) as libc::c_ulong,
        );
    } else if 0 as libc::c_int as libc::c_longlong == zt {
        let mut c: K = promote(x);
        let mut d: K = promote(y);
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = xk;
        while i < _i {
            let ref mut fresh61 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh61 = ci(*((*c).k).as_mut_ptr().offset(i as isize));
            i += 1;
            i;
        }
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = yk;
        while i_0 < _i_0 {
            let ref mut fresh62 = *((*z).k).as_mut_ptr().offset((xk + i_0) as isize);
            *fresh62 = ci(*((*d).k).as_mut_ptr().offset(i_0 as isize));
            i_0 += 1;
            i_0;
        }
        cd(c);
        cd(d);
    }
    return z;
}
unsafe extern "C" fn _hg(mut h: K, mut k: uI, mut v: I, mut x: K, mut p: *mut uI) -> I {
    let mut n: I = (*h).n;
    let mut d: *mut I = ((*h).k).as_mut_ptr() as *mut I;
    let mut i: I = 0;
    let mut u: uI = k & (n - 1 as libc::c_int as libc::c_longlong) as libc::c_ulonglong;
    loop {
        i = *d.offset(u as isize);
        if !(-(1 as libc::c_int) as libc::c_longlong != i) {
            break;
        }
        if v == *(((*x).k).as_mut_ptr() as *mut I).offset(i as isize) {
            *p = u;
            return i;
        }
        u = u.wrapping_add(1);
        if u == n as libc::c_ulonglong {
            u = 0 as libc::c_int as uI;
        }
    }
    *p = u;
    return (*x).n;
}
unsafe extern "C" fn _hgk(mut h: K, mut k: uI, mut v: K, mut x: K, mut p: *mut uI) -> I {
    let mut n: I = (*h).n;
    let mut d: *mut I = ((*h).k).as_mut_ptr() as *mut I;
    let mut i: I = 0;
    let mut u: uI = k & (n - 1 as libc::c_int as libc::c_longlong) as libc::c_ulonglong;
    loop {
        i = *d.offset(u as isize);
        if !(-(1 as libc::c_int) as libc::c_longlong != i) {
            break;
        }
        if KEQ(v, *((*x).k).as_mut_ptr().offset(i as isize)) != 0 {
            *p = u;
            return i;
        }
        u = u.wrapping_add(1);
        if u == n as libc::c_ulonglong {
            u = 0 as libc::c_int as uI;
        }
    }
    *p = u;
    return (*x).n;
}
unsafe extern "C" fn _hgv(mut h: K, mut k: uI, mut v: V, mut x: K, mut p: *mut uI) -> I {
    let mut n: I = (*h).n;
    let mut d: *mut I = ((*h).k).as_mut_ptr() as *mut I;
    let mut i: I = 0;
    let mut u: uI = k & (n - 1 as libc::c_int as libc::c_longlong) as libc::c_ulonglong;
    loop {
        i = *d.offset(u as isize);
        if !(-(1 as libc::c_int) as libc::c_longlong != i) {
            break;
        }
        if v == *(((*x).k).as_mut_ptr() as *mut V).offset(i as isize) {
            *p = u;
            return i;
        }
        u = u.wrapping_add(1);
        if u == n as libc::c_ulonglong {
            u = 0 as libc::c_int as uI;
        }
    }
    *p = u;
    return (*x).n;
}
pub unsafe extern "C" fn _hash(mut x: K) -> K {
    if (*x).t > 0 as libc::c_int as libc::c_longlong {
        return kerr(b"rank\0" as *const u8 as *const libc::c_char);
    }
    let mut p: uI = 0;
    let mut y: K = if -(3 as libc::c_int) as libc::c_longlong == (*x).t {
        newK(
            -(1 as libc::c_int) as I,
            (1 as libc::c_int
                + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)) as I,
        )
    } else {
        newH((*x).n)
    };
    if OOM_CD(0 as libc::c_int as I, y, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    hcinit();
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = (*y).n;
    while i < _i {
        *(((*y).k).as_mut_ptr() as *mut I).offset(i as isize) = -(1 as libc::c_int) as I;
        i += 1;
        i;
    }
    match -(*x).t {
        0 => {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = (*x).n;
            while i_0 < _i_0 {
                let mut v: K = *((*x).k).as_mut_ptr().offset(i_0 as isize);
                if (*x).n == _hgk(y, hcode(v), v, x, &mut p) {
                    *(((*y).k).as_mut_ptr() as *mut I).offset(p as isize) = i_0;
                }
                i_0 += 1;
                i_0;
            }
        }
        1 | 2 => {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = (*x).n;
            while i_1 < _i_1 {
                let mut v_0: uI = *(((*x).k).as_mut_ptr() as *mut uI)
                    .offset(i_1 as isize);
                if (*x).n == _hg(y, hc(v_0) as uI, v_0 as I, x, &mut p) {
                    *(((*y).k).as_mut_ptr() as *mut I).offset(p as isize) = i_1;
                }
                i_1 += 1;
                i_1;
            }
        }
        3 => {
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = (*x).n;
            while i_2 < _i_2 {
                let mut k: uI = *(((*x).k).as_mut_ptr() as *mut C).offset(i_2 as isize)
                    as UC as uI;
                if (*x).n == *(((*y).k).as_mut_ptr() as *mut I).offset(k as isize) {
                    *(((*y).k).as_mut_ptr() as *mut I).offset(k as isize) = i_2;
                }
                i_2 += 1;
                i_2;
            }
        }
        4 => {
            setS(1 as libc::c_int, 0 as libc::c_int as I);
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = (*x).n;
            while i_3 < _i_3 {
                let mut v_1: S = *(((*x).k).as_mut_ptr() as *mut S).offset(i_3 as isize);
                if *(v_1 as *mut I).offset(-(1 as libc::c_int) as isize) == 0 {
                    *(v_1 as *mut I)
                        .offset(
                            -(1 as libc::c_int) as isize,
                        ) = fnv1a(
                        v_1 as *mut UC,
                        strlen(v_1 as *const libc::c_char) as I,
                    ) as I;
                }
                if (*x).n
                    == _hgv(
                        y,
                        *(v_1 as *mut I).offset(-(1 as libc::c_int) as isize) as uI,
                        v_1 as V,
                        x,
                        &mut p,
                    )
                {
                    *(((*y).k).as_mut_ptr() as *mut I).offset(p as isize) = i_3;
                }
                i_3 += 1;
                i_3;
            }
        }
        _ => {}
    }
    (*y).t = -(5 as libc::c_int) as I;
    return y;
}
pub unsafe extern "C" fn hash_find(mut a: K, mut b: K) -> K {
    let mut x: K = *((*a).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    let mut y: K = *((*a).k).as_mut_ptr().offset(1 as libc::c_int as isize);
    let mut k: uI = 0;
    let mut p: uI = 0;
    let mut i: I = 0;
    if (*x).t > 0 as libc::c_int as libc::c_longlong {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    if (*x).t != 0 && (*x).t + (*b).t != 0 {
        return Ki((*x).n);
    }
    hcinit();
    match -(*x).t {
        0 => {
            i = _hgk(y, hcode(b), b, x, &mut p);
        }
        1 | 2 => {
            let mut v: uI = *(((*b).k).as_mut_ptr() as *mut uI);
            i = _hg(y, hc(v) as uI, v as I, x, &mut p);
        }
        3 => {
            k = *(((*b).k).as_mut_ptr() as *mut C) as UC as uI;
            i = *(((*y).k).as_mut_ptr() as *mut I).offset(k as isize);
            if i < 0 as libc::c_int as libc::c_longlong {
                i = (*x).n;
            }
        }
        4 => {
            let mut v_0: S = *(((*b).k).as_mut_ptr() as *mut S);
            k = fnv1a(v_0 as *mut UC, strlen(v_0 as *const libc::c_char) as I) as uI;
            i = _hgv(y, k, v_0 as V, x, &mut p);
        }
        _ => {}
    }
    return Ki(i);
}
