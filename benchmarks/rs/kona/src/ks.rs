use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    static mut SYMBOLS: N;
    fn kerr(s: cS) -> K;
    fn newN() -> N;
    fn alloc(sz: size_t) -> V;
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub k: V,
    pub b: I,
    pub c: [*mut node; 2],
}
pub type Node = node;
pub type N = *mut Node;
static mut ns: I = 0 as libc::c_int as I;
static mut sdd: I = 0 as libc::c_int as I;
unsafe extern "C" fn sdupI(mut s: S) -> S {
    let mut k: I = 0;
    k = strlen(s as *const libc::c_char) as I;
    let mut d: S = alloc(
        ((2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong)
            as libc::c_ulonglong)
            .wrapping_add(k as libc::c_ulonglong)
            .wrapping_add(1 as libc::c_int as libc::c_ulonglong) as size_t,
    ) as S;
    if d.is_null() {
        return 0 as S;
    }
    ns += 1;
    ns;
    sdd = 1 as libc::c_int as I;
    d = d
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong) as isize,
        );
    *d.offset(k as isize) = 0 as libc::c_int as C;
    return memcpy(d as *mut libc::c_void, s as *const libc::c_void, k as libc::c_ulong)
        as S;
}
pub unsafe extern "C" fn strdupn(mut s: S, mut k: I) -> S {
    let mut d: S = alloc((k + 1 as libc::c_int as libc::c_longlong) as size_t) as S;
    if d.is_null() {
        return 0 as S;
    }
    *d.offset(k as isize) = 0 as libc::c_int as C;
    return memcpy(d as *mut libc::c_void, s as *const libc::c_void, k as libc::c_ulong)
        as S;
}
pub unsafe extern "C" fn strlenn(mut s: S, mut k: I) -> I {
    let mut t: S = memchr(s as *const libc::c_void, '\0' as i32, k as libc::c_ulong)
        as S;
    return if !t.is_null() {
        t.offset_from(s) as libc::c_long as libc::c_longlong
    } else {
        k
    };
}
pub unsafe extern "C" fn StoI(mut s: S, mut n: *mut I) -> I {
    let mut t: S = 0 as *mut C;
    *__errno_location() = 0 as libc::c_int;
    *n = strtol(s as *const libc::c_char, &mut t, 10 as libc::c_int) as I;
    return !(*__errno_location() != 0 as libc::c_int || t == s
        || *t as libc::c_int != 0 as libc::c_int) as libc::c_int as I;
}
pub unsafe extern "C" fn SC(mut a: S, mut b: S) -> I {
    let mut x: I = strcmp(a as *const libc::c_char, b as *const libc::c_char) as I;
    return (if x < 0 as libc::c_int as libc::c_longlong {
        -(1 as libc::c_int)
    } else if x > 0 as libc::c_int as libc::c_longlong {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as I;
}
pub unsafe extern "C" fn sp(mut k: S) -> S {
    if k.is_null() {
        return 0 as S;
    }
    let mut t: N = SYMBOLS;
    let mut s: N = (*t).c[1 as libc::c_int as usize];
    let mut p: N = s;
    let mut q: N = p;
    let mut r: N = 0 as *mut Node;
    let mut a: I = 0;
    let mut x: I = 0;
    if s.is_null() {
        (*t).c[1 as libc::c_int as usize] = newN();
        s = (*t).c[1 as libc::c_int as usize];
        if s.is_null() {
            return kerr(b"wsfull\0" as *const u8 as *const libc::c_char) as S;
        }
        (*s).k = sdupI(k) as V;
        if ((*s).k).is_null() {
            free(s as *mut libc::c_void);
            (*t).c[1 as libc::c_int as usize] = 0 as *mut node;
            kerr(b"wsfull\0" as *const u8 as *const libc::c_char);
        }
        return (*s).k as S;
    }
    while !q.is_null() {
        a = SC(k, (*p).k as S);
        if a == 0 {
            return (*p).k as S;
        }
        q = (*p)
            .c[((a + 1 as libc::c_int as libc::c_longlong)
            / 2 as libc::c_int as libc::c_longlong) as usize];
        if q.is_null() {
            q = newN();
            if q.is_null() {
                return kerr(b"wsfull\0" as *const u8 as *const libc::c_char) as S;
            }
            (*q).k = sdupI(k) as V;
            if ((*q).k).is_null() {
                free(q as *mut libc::c_void);
                kerr(b"wsfull\0" as *const u8 as *const libc::c_char);
                return 0 as S;
            }
            (*p)
                .c[((a + 1 as libc::c_int as libc::c_longlong)
                / 2 as libc::c_int as libc::c_longlong) as usize] = q;
            break;
        } else {
            if (*q).b != 0 {
                t = p;
                s = q;
            }
            p = q;
        }
    }
    a = (if 0 as libc::c_int as libc::c_longlong > SC(k, (*s).k as S) {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    }) as I;
    p = (*s)
        .c[((a + 1 as libc::c_int as libc::c_longlong)
        / 2 as libc::c_int as libc::c_longlong) as usize];
    r = p;
    while p != q {
        x = SC(k, (*p).k as S);
        (*p).b = x;
        p = (*p)
            .c[((x + 1 as libc::c_int as libc::c_longlong)
            / 2 as libc::c_int as libc::c_longlong) as usize];
    }
    if (*s).b == 0 {
        (*s).b = a;
        return (*p).k as S;
    } else if (*s).b == -a {
        (*s).b = 0 as libc::c_int as I;
        return (*p).k as S;
    }
    if (*r).b == a {
        p = r;
        (*s)
            .c[((a + 1 as libc::c_int as libc::c_longlong)
            / 2 as libc::c_int as libc::c_longlong)
            as usize] = (*r)
            .c[((-a + 1 as libc::c_int as libc::c_longlong)
            / 2 as libc::c_int as libc::c_longlong) as usize];
        (*r)
            .c[((-a + 1 as libc::c_int as libc::c_longlong)
            / 2 as libc::c_int as libc::c_longlong) as usize] = s;
        (*r).b = 0 as libc::c_int as I;
        (*s).b = (*r).b;
    } else if (*r).b == -a {
        p = (*r)
            .c[((-a + 1 as libc::c_int as libc::c_longlong)
            / 2 as libc::c_int as libc::c_longlong) as usize];
        (*r)
            .c[((-a + 1 as libc::c_int as libc::c_longlong)
            / 2 as libc::c_int as libc::c_longlong)
            as usize] = (*p)
            .c[((a + 1 as libc::c_int as libc::c_longlong)
            / 2 as libc::c_int as libc::c_longlong) as usize];
        (*p)
            .c[((a + 1 as libc::c_int as libc::c_longlong)
            / 2 as libc::c_int as libc::c_longlong) as usize] = r;
        (*s)
            .c[((a + 1 as libc::c_int as libc::c_longlong)
            / 2 as libc::c_int as libc::c_longlong)
            as usize] = (*p)
            .c[((-a + 1 as libc::c_int as libc::c_longlong)
            / 2 as libc::c_int as libc::c_longlong) as usize];
        (*p)
            .c[((-a + 1 as libc::c_int as libc::c_longlong)
            / 2 as libc::c_int as libc::c_longlong) as usize] = s;
        if (*p).b == a {
            (*s).b = -a;
            (*r).b = 0 as libc::c_int as I;
        } else if (*p).b == 0 as libc::c_int as libc::c_longlong {
            (*s).b = 0 as libc::c_int as I;
            (*r).b = 0 as libc::c_int as I;
        } else if (*p).b == -a {
            (*s).b = 0 as libc::c_int as I;
            (*r).b = a;
        }
        (*p).b = 0 as libc::c_int as I;
    }
    (*t)
        .c[(if s == (*t).c[1 as libc::c_int as usize] {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as usize] = p;
    return (*q).k as S;
}
pub unsafe extern "C" fn spn(mut s: S, mut n: I) -> S {
    let mut k: I = 0 as libc::c_int as I;
    while k < n && *s.offset(k as isize) as libc::c_int != 0 {
        k += 1;
        k;
    }
    let mut u: S = strdupn(s, k);
    if u.is_null() {
        return 0 as S;
    }
    let mut v: S = sp(u);
    free(u as *mut libc::c_void);
    return v;
}
pub unsafe extern "C" fn wleft(mut x: N, mut y: I, mut z: I) -> I {
    if x.is_null() {
        return z;
    }
    z = wleft((*x).c[0 as libc::c_int as usize], y, z);
    if !((*x).k).is_null() && *((*x).k as *mut I).offset(-y as isize) != 0 {
        let mut o: I = *((*x).k as *mut I).offset(-y as isize);
        *((*x).k as *mut I).offset(-y as isize) = z;
        z += o;
    }
    return wleft((*x).c[1 as libc::c_int as usize], y, z);
}
pub unsafe extern "C" fn wright(mut x: N, mut y: I, mut z: I) -> I {
    if x.is_null() {
        return z;
    }
    z = wright((*x).c[1 as libc::c_int as usize], y, z);
    if !((*x).k).is_null() && *((*x).k as *mut I).offset(-y as isize) != 0 {
        let mut o: I = *((*x).k as *mut I).offset(-y as isize);
        *((*x).k as *mut I).offset(-y as isize) = z;
        z += o;
    }
    return wright((*x).c[0 as libc::c_int as usize], y, z);
}
unsafe extern "C" fn ssI(mut x: N, mut y: libc::c_int, mut z: I) {
    if !x.is_null() {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = 2 as libc::c_int as I;
        while i < _i {
            ssI((*x).c[i as usize], y, z);
            i += 1;
            i;
        }
        if !((*x).k).is_null() {
            *((*x).k as *mut I).offset(-y as isize) = z;
        }
    }
}
pub unsafe extern "C" fn setS(mut y: libc::c_int, mut z: I) {
    ssI(SYMBOLS, y, z);
}
pub unsafe extern "C" fn OS(mut x: N, mut y: I) {
    if x.is_null() {
        return;
    }
    OS((*x).c[0 as libc::c_int as usize], y);
    if !((*x).k).is_null() && *((*x).k as *mut I).offset(-y as isize) != 0 {
        printf(
            b"%p: %lld\n\0" as *const u8 as *const libc::c_char,
            (*x).k,
            *((*x).k as *mut I).offset(-y as isize),
        );
    }
    OS((*x).c[1 as libc::c_int as usize], y);
}
