use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn clock() -> clock_t;
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
    fn llabs(_: libc::c_longlong) -> libc::c_longlong;
    fn cd(a: K) -> K;
    static mut SYMBOLS: N;
    fn kerr(s: cS) -> K;
    fn newK(t: I, n: I) -> K;
    fn SC(a: S, b: S) -> I;
    fn setS(y: libc::c_int, z: I);
    fn wleft(x: N, y: I, z: I) -> I;
    fn wright(x: N, y: I, z: I) -> I;
    static mut feci: I;
    fn OOM_CD(g: I, _: ...) -> I;
}
pub type __clock_t = libc::c_long;
pub type clock_t = __clock_t;
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
pub struct node {
    pub k: V,
    pub b: I,
    pub c: [*mut node; 2],
}
pub type Node = node;
pub type N = *mut Node;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub i: I,
    pub f: F,
}
static mut gt: I = 0 as libc::c_int as I;
pub unsafe extern "C" fn FC(mut a: F, mut b: F) -> I {
    let mut x: C2RustUnnamed = C2RustUnnamed { i: 0 };
    let mut y: C2RustUnnamed = C2RustUnnamed { i: 0 };
    let mut xu: I = 0;
    let mut ad: I = 0;
    x.f = a;
    y.f = b;
    xu = x.i | y.i;
    if (0x7ff0000000000000 as libc::c_ulonglong)
        < 0x7fffffffffffffff as libc::c_ulonglong & x.i as libc::c_ulonglong
    {
        return (if (0x7ff0000000000000 as libc::c_ulonglong)
            < 0x7fffffffffffffff as libc::c_ulonglong & y.i as libc::c_ulonglong
        {
            0 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as I
    } else if x.i < 0 as libc::c_int as libc::c_longlong {
        x.i = -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong - x.i;
    }
    if (0x7ff0000000000000 as libc::c_ulonglong)
        < 0x7fffffffffffffff as libc::c_ulonglong & y.i as libc::c_ulonglong
    {
        return 1 as libc::c_int as I
    } else if y.i < 0 as libc::c_int as libc::c_longlong {
        y.i = -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong - y.i;
    }
    ad = llabs(x.i - y.i);
    if ad < 1023 as libc::c_int as libc::c_longlong {
        if 0x10000000000000 as libc::c_ulonglong
            > 0x7fffffffffffffff as libc::c_ulonglong & xu as libc::c_ulonglong
        {
            return (if x.i < y.i {
                -(1 as libc::c_int)
            } else if x.i != y.i {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as I;
        }
        if ad < 513 as libc::c_int as libc::c_longlong {
            return 0 as libc::c_int as I;
        }
        xu = 513 as libc::c_int as libc::c_longlong
            + ((255 as libc::c_int as libc::c_longlong & xu >> 44 as libc::c_int)
                << 1 as libc::c_int);
        if ad < xu {
            return 0 as libc::c_int as I;
        }
    }
    return (if x.i < y.i { -(1 as libc::c_int) } else { 1 as libc::c_int }) as I;
}
pub unsafe extern "C" fn KC(mut a: K, mut b: K) -> I {
    if b.is_null() {
        feci = 1 as libc::c_int as I;
        return kerr(b"type\0" as *const u8 as *const libc::c_char) as I;
    }
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    let mut A: I = if at < 0 as libc::c_int as libc::c_longlong { -at } else { at };
    if at < bt {
        return -(1 as libc::c_int) as I;
    }
    if at > bt {
        return 1 as libc::c_int as I;
    }
    if 3 as libc::c_int as libc::c_longlong != A {
        if an < bn {
            return -(1 as libc::c_int) as I;
        }
        if an > bn {
            return 1 as libc::c_int as I;
        }
    }
    let mut u: I = 0;
    let mut v: I = 0;
    let mut c: C = 0;
    let mut d: C = 0;
    if 7 as libc::c_int as libc::c_longlong == A {
        return 0 as libc::c_int as I
    } else if 6 as libc::c_int as libc::c_longlong == A {
        return 0 as libc::c_int as I
    } else if 5 as libc::c_int as libc::c_longlong == A {
        return 0 as libc::c_int as I
    } else if 4 as libc::c_int as libc::c_longlong == A {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = an;
        while i < _i {
            u = SC(
                *(((*a).k).as_mut_ptr() as *mut S).offset(i as isize),
                *(((*b).k).as_mut_ptr() as *mut S).offset(i as isize),
            );
            if u != 0 {
                return u;
            }
            i += 1;
            i;
        }
    } else if 3 as libc::c_int as libc::c_longlong == A {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = (if an < bn { an } else { bn })
            + 1 as libc::c_int as libc::c_longlong;
        while i_0 < _i_0 {
            c = *(((*a).k).as_mut_ptr() as *mut C).offset(i_0 as isize);
            d = *(((*b).k).as_mut_ptr() as *mut C).offset(i_0 as isize);
            if (c as libc::c_int) < d as libc::c_int {
                return -(1 as libc::c_int) as I;
            }
            if c as libc::c_int > d as libc::c_int {
                return 1 as libc::c_int as I;
            }
            i_0 += 1;
            i_0;
        }
    } else if 2 as libc::c_int as libc::c_longlong == A {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = an;
        while i_1 < _i_1 {
            u = FC(
                *(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize),
                *(((*b).k).as_mut_ptr() as *mut F).offset(i_1 as isize),
            );
            if u != 0 {
                return u;
            }
            i_1 += 1;
            i_1;
        }
    } else if 1 as libc::c_int as libc::c_longlong == A {
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = an;
        while i_2 < _i_2 {
            u = *(((*a).k).as_mut_ptr() as *mut I).offset(i_2 as isize);
            v = *(((*b).k).as_mut_ptr() as *mut I).offset(i_2 as isize);
            if u < v {
                return -(1 as libc::c_int) as I;
            }
            if u > v {
                return 1 as libc::c_int as I;
            }
            i_2 += 1;
            i_2;
        }
    } else if 0 as libc::c_int as libc::c_longlong == A {
        let mut i_3: I = 0 as libc::c_int as I;
        let mut _i_3: I = an;
        while i_3 < _i_3 {
            u = KC(
                *((*a).k).as_mut_ptr().offset(i_3 as isize),
                *((*b).k).as_mut_ptr().offset(i_3 as isize),
            );
            if u != 0 {
                return u;
            }
            i_3 += 1;
            i_3;
        }
    }
    if 3 as libc::c_int as libc::c_longlong == A && an != bn {
        return (if an < bn { -(1 as libc::c_int) } else { 1 as libc::c_int }) as I;
    }
    return 0 as libc::c_int as I;
}
pub unsafe extern "C" fn distributionGrade(
    mut a: K,
    mut r: I,
    mut u: uI,
    mut v: uI,
) -> K {
    if gt != 0 {
        printf(b"distributionGrade\0" as *const u8 as *const libc::c_char);
    }
    let mut n: I = (*a).n;
    let mut b: I = v.wrapping_sub(u).wrapping_add(1 as libc::c_int as libc::c_ulonglong)
        as I;
    let mut c: *mut I = 0 as *mut I;
    let mut d: K = newK(-(1 as libc::c_int) as I, b);
    if d.is_null() {
        return 0 as K;
    }
    c = ((*d).k).as_mut_ptr() as *mut I;
    let mut s: K = newK(-(1 as libc::c_int) as I, n);
    if !s.is_null() {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = n;
        while i < _i {
            let ref mut fresh0 = *c
                .offset(
                    (*(((*a).k).as_mut_ptr() as *mut uI).offset(i as isize))
                        .wrapping_sub(u) as isize,
                );
            *fresh0 += 1;
            *fresh0;
            i += 1;
            i;
        }
        if r == 0 {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = b - 1 as libc::c_int as libc::c_longlong;
            while i_0 < _i_0 {
                let ref mut fresh1 = *c
                    .offset((i_0 + 1 as libc::c_int as libc::c_longlong) as isize);
                *fresh1 += *c.offset(i_0 as isize);
                i_0 += 1;
                i_0;
            }
        } else {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = b - 1 as libc::c_int as libc::c_longlong;
            while i_1 < _i_1 {
                let ref mut fresh2 = *c
                    .offset(
                        (_i_1 - i_1 - 1 as libc::c_int as libc::c_longlong) as isize,
                    );
                *fresh2
                    += *c
                        .offset(
                            (_i_1 - i_1 - 0 as libc::c_int as libc::c_longlong) as isize,
                        );
                i_1 += 1;
                i_1;
            }
        }
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = n;
        while i_2 < _i_2 {
            let ref mut fresh3 = *c
                .offset(
                    (*(((*a).k).as_mut_ptr() as *mut uI)
                        .offset(
                            (n - i_2 - 1 as libc::c_int as libc::c_longlong) as isize,
                        ))
                        .wrapping_sub(u) as isize,
                );
            let fresh4 = *fresh3;
            *fresh3 = *fresh3 - 1;
            *(((*s).k).as_mut_ptr() as *mut I)
                .offset(
                    (-(1 as libc::c_int) as libc::c_longlong + fresh4) as isize,
                ) = n - i_2 - 1 as libc::c_int as libc::c_longlong;
            i_2 += 1;
            i_2;
        }
    }
    cd(d);
    return s;
}
pub unsafe extern "C" fn charGrade(mut a: K, mut r: I) -> K {
    let mut n: I = (*a).n;
    let mut c: [I; 256] = [0; 256];
    memset(
        c.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ((1 as libc::c_int + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int))
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong),
    );
    let mut s: K = newK(-(1 as libc::c_int) as I, n);
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = n;
    while i < _i {
        c[*(((*a).k).as_mut_ptr() as *mut C).offset(i as isize) as UC as usize] += 1;
        c[*(((*a).k).as_mut_ptr() as *mut C).offset(i as isize) as UC as usize];
        i += 1;
        i;
    }
    if r == 0 {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
            as I;
        while i_0 < _i_0 {
            c[(i_0 + 1 as libc::c_int as libc::c_longlong) as usize] += c[i_0 as usize];
            i_0 += 1;
            i_0;
        }
    } else {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
            as I;
        while i_1 < _i_1 {
            c[(_i_1 - i_1 - 1 as libc::c_int as libc::c_longlong) as usize]
                += c[(_i_1 - i_1 - 0 as libc::c_int as libc::c_longlong) as usize];
            i_1 += 1;
            i_1;
        }
    }
    let mut i_2: I = 0 as libc::c_int as I;
    let mut _i_2: I = n;
    while i_2 < _i_2 {
        let fresh5 = c[*(((*a).k).as_mut_ptr() as *mut C)
            .offset((n - i_2 - 1 as libc::c_int as libc::c_longlong) as isize) as UC
            as usize];
        c[*(((*a).k).as_mut_ptr() as *mut C)
            .offset((n - i_2 - 1 as libc::c_int as libc::c_longlong) as isize) as UC
            as usize] = c[*(((*a).k).as_mut_ptr() as *mut C)
            .offset((n - i_2 - 1 as libc::c_int as libc::c_longlong) as isize) as UC
            as usize] - 1;
        *(((*s).k).as_mut_ptr() as *mut I)
            .offset(
                (-(1 as libc::c_int) as libc::c_longlong + fresh5) as isize,
            ) = n - i_2 - 1 as libc::c_int as libc::c_longlong;
        i_2 += 1;
        i_2;
    }
    return s;
}
unsafe extern "C" fn mergerComparer(mut a: K, mut r: I, mut i: I, mut j: I) -> I {
    let mut t: I = (*a).t;
    if -(4 as libc::c_int) as libc::c_longlong == t
        && 0 as libc::c_int as libc::c_longlong == r
        && 1 as libc::c_int as libc::c_longlong
            > SC(
                *(((*a).k).as_mut_ptr() as *mut S).offset(i as isize),
                *(((*a).k).as_mut_ptr() as *mut S).offset(j as isize),
            )
    {
        return 1 as libc::c_int as I
    } else if -(4 as libc::c_int) as libc::c_longlong == t
        && 1 as libc::c_int as libc::c_longlong == r
        && (-(1 as libc::c_int) as libc::c_longlong)
            < SC(
                *(((*a).k).as_mut_ptr() as *mut S).offset(i as isize),
                *(((*a).k).as_mut_ptr() as *mut S).offset(j as isize),
            )
    {
        return 1 as libc::c_int as I
    } else if -(2 as libc::c_int) as libc::c_longlong == t
        && 0 as libc::c_int as libc::c_longlong == r
        && 1 as libc::c_int as libc::c_longlong
            > FC(
                *(((*a).k).as_mut_ptr() as *mut F).offset(i as isize),
                *(((*a).k).as_mut_ptr() as *mut F).offset(j as isize),
            )
    {
        return 1 as libc::c_int as I
    } else if -(2 as libc::c_int) as libc::c_longlong == t
        && 1 as libc::c_int as libc::c_longlong == r
        && (-(1 as libc::c_int) as libc::c_longlong)
            < FC(
                *(((*a).k).as_mut_ptr() as *mut F).offset(i as isize),
                *(((*a).k).as_mut_ptr() as *mut F).offset(j as isize),
            )
    {
        return 1 as libc::c_int as I
    } else if -(1 as libc::c_int) as libc::c_longlong == t
        && 0 as libc::c_int as libc::c_longlong == r
        && *(((*a).k).as_mut_ptr() as *mut I).offset(i as isize)
            <= *(((*a).k).as_mut_ptr() as *mut I).offset(j as isize)
    {
        return 1 as libc::c_int as I
    } else if -(1 as libc::c_int) as libc::c_longlong == t
        && 1 as libc::c_int as libc::c_longlong == r
        && *(((*a).k).as_mut_ptr() as *mut I).offset(i as isize)
            >= *(((*a).k).as_mut_ptr() as *mut I).offset(j as isize)
    {
        return 1 as libc::c_int as I
    } else if 0 as libc::c_int as libc::c_longlong == t
        && 0 as libc::c_int as libc::c_longlong == r
        && 1 as libc::c_int as libc::c_longlong
            > KC(
                *((*a).k).as_mut_ptr().offset(i as isize),
                *((*a).k).as_mut_ptr().offset(j as isize),
            )
    {
        return 1 as libc::c_int as I
    } else if 0 as libc::c_int as libc::c_longlong == t
        && 1 as libc::c_int as libc::c_longlong == r
        && (-(1 as libc::c_int) as libc::c_longlong)
            < KC(
                *((*a).k).as_mut_ptr().offset(i as isize),
                *((*a).k).as_mut_ptr().offset(j as isize),
            )
    {
        return 1 as libc::c_int as I
    }
    return 0 as libc::c_int as I;
}
unsafe extern "C" fn merger(
    mut a: K,
    mut r: I,
    mut x: K,
    mut y: K,
    mut s: I,
    mut t: I,
    mut m: I,
) {
    let mut i: I = 0;
    let mut j: I = 0;
    let mut k: I = 0;
    let mut c: *mut I = ((*x).k).as_mut_ptr() as *mut I;
    let mut d: *mut I = ((*y).k).as_mut_ptr() as *mut I;
    memcpy(
        d.offset(s as isize) as *mut libc::c_void,
        c.offset(s as isize) as *const libc::c_void,
        ((t - s + 1 as libc::c_int as libc::c_longlong) as libc::c_ulonglong)
            .wrapping_mul(
                ::std::mem::size_of::<I>() as libc::c_ulong as libc::c_ulonglong,
            ) as libc::c_ulong,
    );
    i = s;
    j = m + 1 as libc::c_int as libc::c_longlong;
    k = s;
    while i <= m && j <= t {
        if mergerComparer(a, r, *d.offset(i as isize), *d.offset(j as isize)) != 0 {
            let fresh6 = i;
            i = i + 1;
            let fresh7 = k;
            k = k + 1;
            *c.offset(fresh7 as isize) = *d.offset(fresh6 as isize);
        } else {
            let fresh8 = j;
            j = j + 1;
            let fresh9 = k;
            k = k + 1;
            *c.offset(fresh9 as isize) = *d.offset(fresh8 as isize);
        }
    }
    if i <= m {
        memcpy(
            c.offset(k as isize) as *mut libc::c_void,
            d.offset(i as isize) as *const libc::c_void,
            ((m - i + 1 as libc::c_int as libc::c_longlong) as libc::c_ulonglong)
                .wrapping_mul(
                    ::std::mem::size_of::<I>() as libc::c_ulong as libc::c_ulonglong,
                ) as libc::c_ulong,
        );
    }
}
unsafe extern "C" fn insertGrade(
    mut a: K,
    mut r: I,
    mut x: K,
    mut y: K,
    mut s: I,
    mut t: I,
) {
    let mut i: I = 0;
    let mut c: *mut I = ((*x).k).as_mut_ptr() as *mut I;
    i = s + 1 as libc::c_int as libc::c_longlong;
    while i <= t {
        let mut x_0: I = *c.offset(i as isize);
        let mut j: I = i;
        while s < j
            && mergerComparer(
                a,
                r,
                *c.offset((j - 1 as libc::c_int as libc::c_longlong) as isize),
                x_0,
            ) == 0
        {
            *c
                .offset(
                    j as isize,
                ) = *c.offset((j - 1 as libc::c_int as libc::c_longlong) as isize);
            j -= 1;
            j;
        }
        *c.offset(j as isize) = x_0;
        i += 1;
        i;
    }
}
unsafe extern "C" fn doMergeGrade(
    mut a: K,
    mut r: I,
    mut x: K,
    mut y: K,
    mut s: I,
    mut t: I,
) {
    if s >= t {
        return;
    }
    let mut m: I = s + (t - s) / 2 as libc::c_int as libc::c_longlong;
    if m - s < 7 as libc::c_int as libc::c_longlong {
        insertGrade(a, r, x, y, s, m);
    } else {
        doMergeGrade(a, r, x, y, s, m);
    }
    if t - (m + 1 as libc::c_int as libc::c_longlong)
        < 7 as libc::c_int as libc::c_longlong
    {
        insertGrade(a, r, x, y, m + 1 as libc::c_int as libc::c_longlong, t);
    } else {
        doMergeGrade(a, r, x, y, m + 1 as libc::c_int as libc::c_longlong, t);
    }
    merger(a, r, x, y, s, t, m);
}
unsafe extern "C" fn StoU(mut s: S, mut n: I, mut t: I) -> uI {
    let mut h: uI = 0 as libc::c_int as uI;
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = 8 as libc::c_int as I;
    while i < _i {
        h <<= 8 as libc::c_int;
        h = (h as libc::c_ulonglong)
            .wrapping_add(
                (if i < n {
                    *s.offset(i as isize) as UC as libc::c_int
                } else {
                    0 as libc::c_int
                }) as libc::c_ulonglong,
            ) as uI as uI;
        i += 1;
        i;
    }
    return h;
}
unsafe extern "C" fn strGrade(mut a: K, mut r: I) -> K {
    let mut h: uI = 0 as libc::c_int as uI;
    let mut k: I = 0;
    let mut s: I = 1 as libc::c_int as I;
    let mut z: K = 0 as K;
    let mut x: K = newK(-(1 as libc::c_int) as I, (*a).n);
    if OOM_CD(0 as libc::c_int as I, x, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = (*x).n;
    while i < _i {
        let mut y: K = *((*a).k).as_mut_ptr().offset(i as isize);
        if -(3 as libc::c_int) as libc::c_longlong != (*y).t
            || (*y).n > 8 as libc::c_int as libc::c_longlong
        {
            s = 0 as libc::c_int as I;
            break;
        } else {
            k = StoU(((*y).k).as_mut_ptr() as *mut C, (*y).n, (*y).t) as I;
            if k == 0 && (*y).n != 0 {
                s = 0 as libc::c_int as I;
                break;
            } else {
                *(((*x).k).as_mut_ptr() as *mut uI).offset(i as isize) = k as uI;
                h |= k as libc::c_ulonglong;
                i += 1;
                i;
            }
        }
    }
    if s != 0 {
        z = radixGrade(x, r, h);
    }
    cd(x);
    return z;
}
pub unsafe extern "C" fn mergeGrade(mut a: K, mut r: I) -> K {
    let mut x: K = 0 as K;
    let mut y: K = 0 as K;
    let mut n: I = (*a).n;
    if gt != 0 {
        printf(b"mergeGrade\0" as *const u8 as *const libc::c_char);
    }
    if 0 as libc::c_int as libc::c_longlong == (*a).t {
        x = strGrade(a, r);
        if !x.is_null() {
            return x;
        }
    }
    x = newK(-(1 as libc::c_int) as I, n);
    y = newK(-(1 as libc::c_int) as I, n);
    if OOM_CD(0 as libc::c_int as I, x, y, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = n;
    while i < _i {
        *(((*x).k).as_mut_ptr() as *mut I).offset(i as isize) = i;
        i += 1;
        i;
    }
    doMergeGrade(
        a,
        r,
        x,
        y,
        0 as libc::c_int as I,
        n - 1 as libc::c_int as libc::c_longlong,
    );
    cd(y);
    return x;
}
pub unsafe extern "C" fn insertGradeU(mut a: K, mut r: I) -> K {
    if gt != 0 {
        printf(b"insertGrade\0" as *const u8 as *const libc::c_char);
    }
    let mut u: *mut uI = ((*a).k).as_mut_ptr() as *mut uI;
    let mut n: I = (*a).n;
    let mut i: I = 0;
    let mut c: *mut I = 0 as *mut I;
    let mut x: K = newK(-(1 as libc::c_int) as I, n);
    if OOM_CD(0 as libc::c_int as I, x, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i: I = n;
    while i_0 < _i {
        *(((*x).k).as_mut_ptr() as *mut I).offset(i_0 as isize) = i_0;
        i_0 += 1;
        i_0;
    }
    c = ((*x).k).as_mut_ptr() as *mut I;
    if r == 0 {
        i = 1 as libc::c_int as I;
        while i <= n - 1 as libc::c_int as libc::c_longlong {
            let mut k: I = *c.offset(i as isize);
            let mut j: I = i;
            while (0 as libc::c_int as libc::c_longlong) < j
                && *u
                    .offset(
                        *c.offset((j - 1 as libc::c_int as libc::c_longlong) as isize)
                            as isize,
                    ) > *u.offset(k as isize)
            {
                *c
                    .offset(
                        j as isize,
                    ) = *c.offset((j - 1 as libc::c_int as libc::c_longlong) as isize);
                j -= 1;
                j;
            }
            *c.offset(j as isize) = k;
            i += 1;
            i;
        }
    } else {
        i = 1 as libc::c_int as I;
        while i <= n - 1 as libc::c_int as libc::c_longlong {
            let mut k_0: I = *c.offset(i as isize);
            let mut j_0: I = i;
            while (0 as libc::c_int as libc::c_longlong) < j_0
                && *u
                    .offset(
                        *c.offset((j_0 - 1 as libc::c_int as libc::c_longlong) as isize)
                            as isize,
                    ) < *u.offset(k_0 as isize)
            {
                *c
                    .offset(
                        j_0 as isize,
                    ) = *c.offset((j_0 - 1 as libc::c_int as libc::c_longlong) as isize);
                j_0 -= 1;
                j_0;
            }
            *c.offset(j_0 as isize) = k_0;
            i += 1;
            i;
        }
    }
    return x;
}
static mut t0: clock_t = 0;
pub unsafe extern "C" fn trst() {
    t0 = clock();
}
pub unsafe extern "C" fn elapsed(mut m: S) {
    let mut e: clock_t = clock() - t0;
    let mut ms: I = (1000.0f64 * e as libc::c_double
        / 1000000 as libc::c_int as __clock_t as libc::c_double) as I;
    if ms != 0 {
        printf(b"%s %lld\n\0" as *const u8 as *const libc::c_char, m, ms);
    }
    trst();
}
unsafe extern "C" fn dGU(
    mut a: *mut uI,
    mut r: I,
    mut x: *mut I,
    mut y: *mut I,
    mut n: I,
    mut c: *mut I,
    mut d: I,
) {
    let mut sa: I = 16 as libc::c_int as libc::c_longlong * d;
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = n;
    while i < _i {
        let ref mut fresh10 = *c
            .offset(
                (65535 as libc::c_int as libc::c_ulonglong & *a.offset(i as isize) >> sa)
                    as isize,
            );
        *fresh10 += 1;
        *fresh10;
        i += 1;
        i;
    }
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_0: I = 65535 as libc::c_int as I;
    while i_0 < _i_0 {
        let ref mut fresh11 = *c
            .offset((i_0 + 1 as libc::c_int as libc::c_longlong) as isize);
        *fresh11 += *c.offset(i_0 as isize);
        i_0 += 1;
        i_0;
    }
    let mut i_1: I = 0 as libc::c_int as I;
    let mut _i_1: I = n;
    while i_1 < _i_1 {
        let mut k: I = *x
            .offset((n - i_1 - 1 as libc::c_int as libc::c_longlong) as isize);
        let ref mut fresh12 = *c
            .offset(
                (65535 as libc::c_int as libc::c_ulonglong
                    & *a
                        .offset(
                            (n - i_1 - 1 as libc::c_int as libc::c_longlong) as isize,
                        ) >> sa) as isize,
            );
        let fresh13 = *fresh12;
        *fresh12 = *fresh12 - 1;
        *y.offset((-(1 as libc::c_int) as libc::c_longlong + fresh13) as isize) = k;
        i_1 += 1;
        i_1;
    }
}
unsafe extern "C" fn radixGradeI(
    mut a: *mut uI,
    mut w: *mut uI,
    mut r: I,
    mut u: *mut I,
    mut v: *mut I,
    mut c: *mut I,
    mut n: I,
    mut h: uI,
) {
    if r != 0 {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = n;
        while i < _i {
            *a.offset(i as isize) = !*a.offset(i as isize);
            i += 1;
            i;
        }
        r = 0 as libc::c_int as I;
    }
    dGU(a, r, u, v, n, c, 0 as libc::c_int as I);
    if !(0x10000 as libc::c_ulonglong > h) {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = n;
        while i_0 < _i_0 {
            *w.offset(i_0 as isize) = *a.offset(*v.offset(i_0 as isize) as isize);
            i_0 += 1;
            i_0;
        }
        c = c.offset((1 as libc::c_int + 65535 as libc::c_int) as isize);
        dGU(w, r, v, u, n, c, 1 as libc::c_int as I);
        if 0x100000000 as libc::c_ulonglong > h {
            return;
        }
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = n;
        while i_1 < _i_1 {
            *w.offset(i_1 as isize) = *a.offset(*u.offset(i_1 as isize) as isize);
            i_1 += 1;
            i_1;
        }
        c = c.offset((1 as libc::c_int + 65535 as libc::c_int) as isize);
        dGU(w, r, u, v, n, c, 2 as libc::c_int as I);
        if !(0x1000000000000 as libc::c_ulonglong > h) {
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = n;
            while i_2 < _i_2 {
                *w.offset(i_2 as isize) = *a.offset(*v.offset(i_2 as isize) as isize);
                i_2 += 1;
                i_2;
            }
            c = c.offset((1 as libc::c_int + 65535 as libc::c_int) as isize);
            dGU(w, r, v, u, n, c, 3 as libc::c_int as I);
            return;
        }
    }
    memcpy(
        u as *mut libc::c_void,
        v as *const libc::c_void,
        (n as libc::c_ulonglong)
            .wrapping_mul(
                ::std::mem::size_of::<I>() as libc::c_ulong as libc::c_ulonglong,
            ) as libc::c_ulong,
    );
}
pub unsafe extern "C" fn radixGrade(mut a: K, mut r: I, mut h: uI) -> K {
    if gt != 0 {
        printf(b"radixGrade\0" as *const u8 as *const libc::c_char);
    }
    let mut n: I = (*a).n;
    let mut x: K = newK(-(1 as libc::c_int) as I, n);
    let mut y: K = newK(-(1 as libc::c_int) as I, n);
    let mut z: K = newK(
        -(1 as libc::c_int) as I,
        (4 as libc::c_int * (1 as libc::c_int + 65535 as libc::c_int)) as I,
    );
    let mut w: K = newK(-(1 as libc::c_int) as I, n);
    if OOM_CD(0 as libc::c_int as I, x, y, z, w, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = n;
    while i < _i {
        *(((*x).k).as_mut_ptr() as *mut I).offset(i as isize) = i;
        i += 1;
        i;
    }
    radixGradeI(
        ((*a).k).as_mut_ptr() as *mut uI,
        ((*w).k).as_mut_ptr() as *mut uI,
        r,
        ((*x).k).as_mut_ptr() as *mut I,
        ((*y).k).as_mut_ptr() as *mut I,
        ((*z).k).as_mut_ptr() as *mut I,
        n,
        h,
    );
    cd(w);
    cd(z);
    cd(y);
    return x;
}
pub unsafe extern "C" fn symGrade(mut x: K, mut r: I) -> K {
    let mut z: K = newK(-(1 as libc::c_int) as I, (*x).n);
    if OOM_CD(0 as libc::c_int as I, x, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    setS(1 as libc::c_int, 0 as libc::c_int as I);
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = (*x).n;
    while i < _i {
        let mut s: S = *(((*x).k).as_mut_ptr() as *mut S).offset(i as isize);
        *(s as *mut I)
            .offset(
                -(1 as libc::c_int) as isize,
            ) = *(s as *mut I).offset(-(1 as libc::c_int) as isize)
            + 1 as libc::c_int as libc::c_longlong;
        i += 1;
        i;
    }
    if r == 0 {
        wleft(SYMBOLS, 1 as libc::c_int as I, 0 as libc::c_int as I);
    } else {
        wright(SYMBOLS, 1 as libc::c_int as I, 0 as libc::c_int as I);
    }
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_0: I = (*x).n;
    while i_0 < _i_0 {
        let mut s_0: S = *(((*x).k).as_mut_ptr() as *mut S).offset(i_0 as isize);
        let mut y: I = *(s_0 as *mut I).offset(-(1 as libc::c_int) as isize);
        let fresh14 = y;
        y = y + 1;
        *(((*z).k).as_mut_ptr() as *mut I).offset(fresh14 as isize) = i_0;
        *(s_0 as *mut I).offset(-(1 as libc::c_int) as isize) = y;
        i_0 += 1;
        i_0;
    }
    return z;
}
