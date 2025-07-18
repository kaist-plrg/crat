use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn cd(a: K) -> K;
    static mut fer: I;
    static mut fnci: I;
    static mut offsetDot: V;
    fn KX(x: K) -> K;
    static mut d_: S;
    static mut KTREE: K;
    fn kerr(s: cS) -> K;
    static mut DT: [TR; 0];
    fn Ks(x: S) -> K;
    fn Ki(x: I) -> K;
    fn _n() -> K;
    fn newK(t: I, n: I) -> K;
    fn ci(a: K) -> K;
    fn OOM_CD(g: I, _: ...) -> I;
    fn denameS(dir_string: S, t: S, create: I) -> *mut K;
    fn sp(k: S) -> S;
    fn EVP(e: K) -> *mut K;
    fn DI(d: K, i: I) -> K;
    fn denameD(d: *mut K, t: S, create: I) -> *mut K;
    fn demote(a: K) -> K;
    fn promote(a: K) -> K;
    fn vf_ex(q: V, g: K) -> K;
    static mut KONA_GSET: K;
    static mut KONA_IDX: K;
    fn collapse(x: K) -> K;
    fn kcloneI(a: K, f: *const libc::c_char, n: libc::c_int) -> K;
    fn inKtree(d: *mut K, t: S, create: I) -> *mut K;
    static mut errmsg: [C; 256];
    fn isColonDyadic(x: K) -> I;
    fn lookupEVOrCreate(p: *mut K, k: S) -> *mut K;
    fn atomI(a: K) -> I;
    fn at_ref(p: *mut K, b: K, c: K, y: K) -> K;
    fn specialAmendDot(c: K, args: K) -> K;
    fn countI(a: K) -> I;
    fn lookup(a: K, b: S) -> K;
    fn at_verb(a: K, b: K) -> K;
    static mut fnc: S;
    static mut fncp: [V; 128];
}
pub type L = libc::c_longlong;
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
unsafe extern "C" fn of2(mut d: K, mut x: *mut K, mut y: *mut K, mut s: I) -> K {
    let mut f: K = *x;
    if f.is_null() {
        return kerr(b"nyi\0" as *const u8 as *const libc::c_char);
    }
    let mut dt: I = (*d).t;
    let mut dn: I = (*d).n;
    let mut ft: I = (*f).t;
    let mut fn_0: I = (*f).n;
    if 0 as libc::c_int as libc::c_longlong >= s {
        return at_verb(d, f);
    }
    let mut z: K = 0 as *mut k0;
    if 0 as libc::c_int as libc::c_longlong == ft {
        z = newK(0 as libc::c_int as I, fn_0);
        if z.is_null() {
            return 0 as K;
        }
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = fn_0;
        while i < _i {
            let ref mut fresh0 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh0 = of2(d, &mut *((*f).k).as_mut_ptr().offset(i as isize), y, s);
            if OOM_CD(0 as libc::c_int as I, z, *fresh0, -(1 as libc::c_int) as V) == 0 {
                return 0 as K;
            }
            i += 1;
            i;
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if ft < 0 as libc::c_int as libc::c_longlong { -ft } else { ft })
    {
        if dt != 0 as libc::c_int as libc::c_longlong {
            return 0 as K;
        }
        let mut k: I = 0;
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = fn_0;
        while i_0 < _i_0 {
            k = *(((*f).k).as_mut_ptr() as *mut I).offset(i_0 as isize);
            if k < 0 as libc::c_int as libc::c_longlong || k >= dn {
                return kerr(b"index\0" as *const u8 as *const libc::c_char);
            }
            i_0 += 1;
            i_0;
        }
        if 1 as libc::c_int as libc::c_longlong == ft {
            return of2(
                *((*d).k)
                    .as_mut_ptr()
                    .offset(*(((*f).k).as_mut_ptr() as *mut I) as isize),
                y,
                y.offset(1 as libc::c_int as isize),
                s - 1 as libc::c_int as libc::c_longlong,
            );
        }
        z = newK(0 as libc::c_int as I, fn_0);
        if z.is_null() {
            return 0 as K;
        }
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = fn_0;
        while i_1 < _i_1 {
            let ref mut fresh1 = *((*z).k).as_mut_ptr().offset(i_1 as isize);
            *fresh1 = of2(
                *((*d).k)
                    .as_mut_ptr()
                    .offset(
                        *(((*f).k).as_mut_ptr() as *mut I).offset(i_1 as isize) as isize,
                    ),
                y,
                y.offset(1 as libc::c_int as isize),
                s - 1 as libc::c_int as libc::c_longlong,
            );
            if OOM_CD(0 as libc::c_int as I, z, *fresh1, -(1 as libc::c_int) as V) == 0 {
                return 0 as K;
            }
            i_1 += 1;
            i_1;
        }
    } else if 4 as libc::c_int as libc::c_longlong
        == (if ft < 0 as libc::c_int as libc::c_longlong { -ft } else { ft })
    {
        if dt != 5 as libc::c_int as libc::c_longlong {
            return 0 as K;
        }
        if 4 as libc::c_int as libc::c_longlong == ft {
            return of2(
                lookup(d, *(((*f).k).as_mut_ptr() as *mut S)),
                y,
                y.offset(1 as libc::c_int as isize),
                s - 1 as libc::c_int as libc::c_longlong,
            );
        }
        z = newK(0 as libc::c_int as I, fn_0);
        if z.is_null() {
            return 0 as K;
        }
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = fn_0;
        while i_2 < _i_2 {
            let ref mut fresh2 = *((*z).k).as_mut_ptr().offset(i_2 as isize);
            *fresh2 = of2(
                lookup(d, *(((*f).k).as_mut_ptr() as *mut S).offset(i_2 as isize)),
                y,
                y.offset(1 as libc::c_int as isize),
                s - 1 as libc::c_int as libc::c_longlong,
            );
            if OOM_CD(0 as libc::c_int as I, z, *fresh2, -(1 as libc::c_int) as V) == 0 {
                return 0 as K;
            }
            i_2 += 1;
            i_2;
        }
    } else if 6 as libc::c_int as libc::c_longlong == ft {
        if 0 as libc::c_int as libc::c_longlong == dt {
            z = newK(0 as libc::c_int as I, dn);
            if z.is_null() {
                return 0 as K;
            }
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = dn;
            while i_3 < _i_3 {
                let ref mut fresh3 = *((*z).k).as_mut_ptr().offset(i_3 as isize);
                *fresh3 = of2(
                    *((*d).k).as_mut_ptr().offset(i_3 as isize),
                    y,
                    y.offset(1 as libc::c_int as isize),
                    s - 1 as libc::c_int as libc::c_longlong,
                );
                if OOM_CD(0 as libc::c_int as I, z, *fresh3, -(1 as libc::c_int) as V)
                    == 0
                {
                    return 0 as K;
                }
                i_3 += 1;
                i_3;
            }
        } else if 5 as libc::c_int as libc::c_longlong == dt {
            z = newK(0 as libc::c_int as I, dn);
            if z.is_null() {
                return 0 as K;
            }
            let mut i_4: I = 0 as libc::c_int as I;
            let mut _i_4: I = dn;
            while i_4 < _i_4 {
                let ref mut fresh4 = *((*z).k).as_mut_ptr().offset(i_4 as isize);
                *fresh4 = of2(
                    *((*(*((*d).k).as_mut_ptr().offset(i_4 as isize) as K)).k)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize),
                    y,
                    y.offset(1 as libc::c_int as isize),
                    s - 1 as libc::c_int as libc::c_longlong,
                );
                if OOM_CD(0 as libc::c_int as I, z, *fresh4, -(1 as libc::c_int) as V)
                    == 0
                {
                    return 0 as K;
                }
                i_4 += 1;
                i_4;
            }
        } else {
            return kerr(b"rank\0" as *const u8 as *const libc::c_char)
        }
    } else {
        return kerr(b"type\0" as *const u8 as *const libc::c_char)
    }
    if !z.is_null() {
        z = demote(z);
    }
    return z;
}
pub unsafe extern "C" fn of(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    if 0 as libc::c_int as libc::c_longlong == (*b).t
        && 0 as libc::c_int as libc::c_longlong == (*b).n
    {
        return ci(a);
    }
    let mut z: K = 0 as K;
    if at == 4 as libc::c_int as libc::c_longlong
        && bt == 0 as libc::c_int as libc::c_longlong
    {
        let mut s: [C; 256] = [0; 256];
        strcpy(s.as_mut_ptr(), d_ as *const libc::c_char);
        strcat(s.as_mut_ptr(), b".\0" as *const u8 as *const libc::c_char);
        strcat(
            s.as_mut_ptr(),
            *(((*a).k).as_mut_ptr() as *mut S) as *const libc::c_char,
        );
        let mut ss: S = *(((*a).k).as_mut_ptr() as *mut S);
        let mut i: I = 0;
        i = 0 as libc::c_int as I;
        while (i as libc::c_ulonglong)
            < strlen(ss as *const libc::c_char) as libc::c_ulonglong
        {
            if *ss.offset(i as isize) as libc::c_int == '_' as i32
                || *ss.offset(i as isize) as libc::c_int == '\u{1a}' as i32
            {
                return kerr(b"domain\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
            i;
        }
        let mut aa: *mut K = denameD(
            &mut KTREE,
            sp(s.as_mut_ptr()),
            1 as libc::c_int as I,
        );
        let mut f: *mut K = &mut *((*b).k).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut *mut k0;
        if aa.is_null() {
            return kerr(b"domain\0" as *const u8 as *const libc::c_char);
        }
        return of2(
            *aa,
            f,
            if bn > 0 as libc::c_int as libc::c_longlong {
                f.offset(1 as libc::c_int as isize)
            } else {
                0 as *mut K
            },
            bn - 1 as libc::c_int as libc::c_longlong,
        );
    }
    if at == 4 as libc::c_int as libc::c_longlong
        && bt == 1 as libc::c_int as libc::c_longlong
    {
        let mut s_0: [C; 256] = [0; 256];
        strcpy(s_0.as_mut_ptr(), d_ as *const libc::c_char);
        strcat(s_0.as_mut_ptr(), b".\0" as *const u8 as *const libc::c_char);
        strcat(
            s_0.as_mut_ptr(),
            *(((*a).k).as_mut_ptr() as *mut S) as *const libc::c_char,
        );
        let mut aa_0: *mut K = denameD(
            &mut KTREE,
            sp(s_0.as_mut_ptr()),
            1 as libc::c_int as I,
        );
        return of(*aa_0, b);
    }
    if (0 as libc::c_int as libc::c_longlong) < at
        && at < 5 as libc::c_int as libc::c_longlong
        && 6 as libc::c_int as libc::c_longlong != bt
    {
        return kerr(b"rank\0" as *const u8 as *const libc::c_char);
    }
    if 6 as libc::c_int as libc::c_longlong == at {
        if 1 as libc::c_int as libc::c_longlong == bt {
            z = ci(b);
        } else if 4 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
        {
            z = _n();
        } else if 0 as libc::c_int as libc::c_longlong == bn
            && (-(1 as libc::c_int) as libc::c_longlong == bt
                || -(2 as libc::c_int) as libc::c_longlong == bt)
        {
            z = _n();
        } else if 0 as libc::c_int as libc::c_longlong == bt
            && 1 as libc::c_int as libc::c_longlong == bn
        {
            z = ci(*((*b).k).as_mut_ptr());
        } else if 0 as libc::c_int as libc::c_longlong == bt {
            z = demote(ci(b));
        } else if 6 as libc::c_int as libc::c_longlong == bt
            || -(3 as libc::c_int) as libc::c_longlong == bt
                && 0 as libc::c_int as libc::c_longlong == bn
        {
            z = newK(0 as libc::c_int as I, 0 as libc::c_int as I);
        } else {
            return kerr(b"type\0" as *const u8 as *const libc::c_char)
        }
    } else if 6 as libc::c_int as libc::c_longlong == bt {
        if 5 as libc::c_int as libc::c_longlong == at {
            z = newK(0 as libc::c_int as I, an);
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i: I = an;
            while i_0 < _i {
                let ref mut fresh5 = *((*z).k).as_mut_ptr().offset(i_0 as isize);
                *fresh5 = ci(
                    *((*(*((*a).k).as_mut_ptr().offset(i_0 as isize) as K)).k)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize),
                );
                i_0 += 1;
                i_0;
            }
            z = demote(z);
        } else if 0 as libc::c_int as libc::c_longlong >= at {
            z = ci(a);
        } else {
            return kerr(b"rank\0" as *const u8 as *const libc::c_char)
        }
    } else if 0 as libc::c_int as libc::c_longlong > bt
        && 0 as libc::c_int as libc::c_longlong == bn
        && -(3 as libc::c_int) as libc::c_longlong != bt
    {
        z = ci(a);
    } else if 5 as libc::c_int as libc::c_longlong == at
        || 0 as libc::c_int as libc::c_longlong == at
    {
        if 0 as libc::c_int as libc::c_longlong == bt {
            let mut f_0: *mut K = &mut *((*b).k)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut *mut k0;
            z = of2(
                a,
                f_0,
                if bn > 0 as libc::c_int as libc::c_longlong {
                    f_0.offset(1 as libc::c_int as isize)
                } else {
                    0 as *mut K
                },
                bn - 1 as libc::c_int as libc::c_longlong,
            );
        } else if -(1 as libc::c_int) as libc::c_longlong == bt
            || -(4 as libc::c_int) as libc::c_longlong == bt
        {
            let mut k: K = promote(b);
            let mut f_1: *mut K = &mut *((*k).k)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut *mut k0;
            z = of2(
                a,
                f_1,
                f_1.offset(1 as libc::c_int as isize),
                bn - 1 as libc::c_int as libc::c_longlong,
            );
            cd(k);
        } else {
            z = at_verb(a, b);
        }
    } else if 0 as libc::c_int as libc::c_longlong > at {
        if -(1 as libc::c_int) as libc::c_longlong == bt
            && 1 as libc::c_int as libc::c_longlong == bn
        {
            let mut f_2: K = newK(1 as libc::c_int as I, 1 as libc::c_int as I);
            *(((*f_2).k).as_mut_ptr() as *mut I) = *(((*b).k).as_mut_ptr() as *mut I);
            z = at_verb(a, f_2);
            cd(f_2);
        } else if 1 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
        {
            z = at_verb(a, b);
        } else if 0 as libc::c_int as libc::c_longlong == bt {
            let mut k_0: K = 0 as *mut k0;
            z = newK(0 as libc::c_int as I, bn);
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_0: I = bn;
            while i_1 < _i_0 {
                k_0 = at_verb(a, *((*b).k).as_mut_ptr().offset(i_1 as isize));
                if OOM_CD(0 as libc::c_int as I, k_0, z, -(1 as libc::c_int) as V) == 0 {
                    return 0 as K;
                }
                let ref mut fresh6 = *((*z).k).as_mut_ptr().offset(i_1 as isize);
                *fresh6 = k_0;
                i_1 += 1;
                i_1;
            }
            z = collapse(z);
        } else {
            return kerr(b"type\0" as *const u8 as *const libc::c_char)
        }
    }
    return z;
}
pub unsafe extern "C" fn dot(mut a: K, mut b: K) -> K {
    if 4 as libc::c_int as libc::c_longlong == (*a).t
        && 4 as libc::c_int as libc::c_longlong == (*b).t
    {
        let mut s: S = malloc(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    strlen(*(((*a).k).as_mut_ptr() as *mut S) as *const libc::c_char),
                )
                .wrapping_add(
                    strlen(*(((*b).k).as_mut_ptr() as *mut S) as *const libc::c_char),
                ),
        ) as S;
        s = strcpy(s, *(((*a).k).as_mut_ptr() as *mut S) as *const libc::c_char);
        strcat(s, b".\0" as *const u8 as *const libc::c_char);
        strcat(s, *(((*b).k).as_mut_ptr() as *mut S) as *const libc::c_char);
        return ci(
            *inKtree(
                &mut *((*(*((*KTREE).k).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as K))
                    .k)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize),
                s,
                0 as libc::c_int as I,
            ),
        );
    }
    if 7 as libc::c_int as libc::c_longlong == (*a).t {
        return vf_ex(&mut a as *mut K as V, b);
    }
    return of(a, b);
}
unsafe extern "C" fn dot_ref(
    mut p: *mut K,
    mut x: *mut K,
    mut z: *mut K,
    mut s: I,
    mut c: K,
    mut y: K,
) -> K {
    let mut d: K = *p;
    let mut f: K = if !x.is_null() { *x } else { 0 as K };
    let mut dt: I = (*d).t;
    let mut dn: I = countI(d);
    let mut ft: I = 999 as libc::c_int as I;
    let mut fn_0: I = 0;
    let mut yn0: I = 0 as libc::c_int as I;
    if !f.is_null() {
        ft = (*f).t;
        fn_0 = countI(f);
    } else {
        return kerr(b"nyi\0" as *const u8 as *const libc::c_char)
    }
    if !y.is_null() {
        yn0 = countI(y);
    }
    if -(1 as libc::c_int) as libc::c_longlong == s
        && 0 as libc::c_int as libc::c_longlong == fn_0
        && -(3 as libc::c_int) as libc::c_longlong != ft
    {
        let mut argc: I = (if !y.is_null() {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        }) as I;
        let mut args: K = newK(0 as libc::c_int as I, argc);
        if args.is_null() {
            return 0 as K;
        }
        let ref mut fresh7 = *((*args).k).as_mut_ptr().offset(0 as libc::c_int as isize);
        *fresh7 = ci(*p);
        if argc > 1 as libc::c_int as libc::c_longlong {
            let ref mut fresh8 = *((*args).k)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize);
            *fresh8 = ci(y);
        }
        let mut r: K = specialAmendDot(c, args);
        cd(args);
        if r.is_null() {
            return 0 as K;
        }
        cd(*p);
        if 5 as libc::c_int as libc::c_longlong == (*r).t
            || 0 as libc::c_int as libc::c_longlong == (*r).t
        {
            *p = kcloneI(
                r,
                b"src/vd.c\0" as *const u8 as *const libc::c_char,
                157 as libc::c_int,
            );
            cd(r);
        } else {
            *p = r;
        }
        return 0 as K;
    }
    if 1 as libc::c_int as libc::c_longlong <= dt
        && dt <= 4 as libc::c_int as libc::c_longlong
        || 7 as libc::c_int as libc::c_longlong == dt
        || 7 as libc::c_int as libc::c_longlong == ft
    {
        return kerr(b"rank\0" as *const u8 as *const libc::c_char)
    } else if 6 as libc::c_int as libc::c_longlong == dt
        && 0 as libc::c_int as libc::c_longlong >= ft
        && -(4 as libc::c_int) as libc::c_longlong != ft
    {
        return kerr(b"index\0" as *const u8 as *const libc::c_char)
    } else if 6 as libc::c_int as libc::c_longlong == dt
        && 6 as libc::c_int as libc::c_longlong != ft
        && 4 as libc::c_int as libc::c_longlong
            != (if ft < 0 as libc::c_int as libc::c_longlong { -ft } else { ft })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char)
    }
    if 5 as libc::c_int as libc::c_longlong == dt
        && 123 as libc::c_int as libc::c_longlong == ft
    {
        return 0 as K;
    }
    if 0 as libc::c_int as libc::c_longlong >= s {
        at_ref(p, f, c, y);
    } else if 0 as libc::c_int as libc::c_longlong == ft {
        if atomI(f) == 0 && !y.is_null() && atomI(y) == 0 && fn_0 != yn0 {
            return kerr(b"length\0" as *const u8 as *const libc::c_char);
        }
        let mut n: I = if atomI(f) != 0 && !y.is_null() { yn0 } else { fn_0 };
        if !y.is_null() {
            y = promote(y);
            if y.is_null() {
                return 0 as K;
            }
        }
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = n;
        while i < _i {
            dot_ref(
                p,
                ((*f).k).as_mut_ptr().offset((i % fn_0) as isize),
                z,
                s,
                c,
                *((*y).k).as_mut_ptr().offset((i % yn0) as isize),
            );
            i += 1;
            i;
        }
        cd(y);
    } else if 1 as libc::c_int as libc::c_longlong
        == (if ft < 0 as libc::c_int as libc::c_longlong { -ft } else { ft })
    {
        if !f.is_null() && atomI(f) == 0 && !y.is_null() && atomI(y) == 0 && fn_0 != yn0
        {
            return kerr(b"length\0" as *const u8 as *const libc::c_char);
        }
        if 1 as libc::c_int as libc::c_longlong == ft
            && dt > 0 as libc::c_int as libc::c_longlong
        {
            return kerr(b"type\0" as *const u8 as *const libc::c_char);
        }
        if !y.is_null() && (*y).t != 0 as libc::c_int as libc::c_longlong && !f.is_null()
            && atomI(f) == 0
        {
            y = promote(y);
            if y.is_null() {
                return 0 as K;
            }
        } else {
            ci(y);
        }
        if dt != 0 as libc::c_int as libc::c_longlong {
            return kerr(b"rank\0" as *const u8 as *const libc::c_char);
        }
        if !f.is_null() {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = fn_0;
            while i_0 < _i_0 {
                let mut e: I = *(((*f).k).as_mut_ptr() as *mut I).offset(i_0 as isize);
                if e < 0 as libc::c_int as libc::c_longlong || dn <= e {
                    return kerr(b"index\0" as *const u8 as *const libc::c_char);
                }
                i_0 += 1;
                i_0;
            }
        }
        if !f.is_null() {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = fn_0;
            while i_1 < _i_1 {
                let mut py: K = 0 as K;
                if !y.is_null() {
                    py = if atomI(f) != 0 {
                        y
                    } else {
                        *((*y).k).as_mut_ptr().offset((i_1 % yn0) as isize)
                    };
                }
                dot_ref(
                    ((*d).k)
                        .as_mut_ptr()
                        .offset(
                            *(((*f).k).as_mut_ptr() as *mut I).offset(i_1 as isize)
                                as isize,
                        ),
                    z,
                    z.offset(1 as libc::c_int as isize),
                    s - 1 as libc::c_int as libc::c_longlong,
                    c,
                    py,
                );
                i_1 += 1;
                i_1;
            }
        }
        cd(y);
    } else if 4 as libc::c_int as libc::c_longlong
        == (if ft < 0 as libc::c_int as libc::c_longlong { -ft } else { ft })
    {
        if atomI(f) == 0 && !y.is_null() && atomI(y) == 0 && fn_0 != yn0 {
            return kerr(b"length\0" as *const u8 as *const libc::c_char);
        }
        if 4 as libc::c_int as libc::c_longlong == ft
            && 0 as libc::c_int as libc::c_longlong >= dt
        {
            return kerr(b"type\0" as *const u8 as *const libc::c_char);
        }
        if -(4 as libc::c_int) as libc::c_longlong == ft
            && 0 as libc::c_int as libc::c_longlong >= dt
        {
            return kerr(b"int\0" as *const u8 as *const libc::c_char);
        }
        if !y.is_null() && (*y).t != 0 as libc::c_int as libc::c_longlong
            && atomI(f) == 0
        {
            y = promote(y);
            if y.is_null() {
                return 0 as K;
            }
        } else {
            ci(y);
        }
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = fn_0;
        while i_2 < _i_2 {
            let mut py_0: K = 0 as K;
            if !y.is_null() {
                py_0 = if atomI(f) != 0 {
                    y
                } else {
                    *((*y).k).as_mut_ptr().offset((i_2 % yn0) as isize)
                };
            }
            let mut u: S = *(((*f).k).as_mut_ptr() as *mut S).offset(i_2 as isize);
            dot_ref(
                lookupEVOrCreate(p, u),
                z,
                z.offset(1 as libc::c_int as isize),
                s - 1 as libc::c_int as libc::c_longlong,
                c,
                py_0,
            );
            i_2 += 1;
            i_2;
        }
        cd(y);
    } else if 6 as libc::c_int as libc::c_longlong == ft {
        if 6 as libc::c_int as libc::c_longlong == dt {
            return 0 as K;
        }
        if !y.is_null() && atomI(y) == 0 && yn0 != (*d).n {
            return kerr(b"length\0" as *const u8 as *const libc::c_char);
        }
        if !y.is_null() {
            y = promote(y);
            if y.is_null() {
                return 0 as K;
            }
        }
        if 5 as libc::c_int as libc::c_longlong == dt {
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = (*d).n;
            while i_3 < _i_3 {
                dot_ref(
                    EVP(DI(d, i_3)),
                    z,
                    z.offset(1 as libc::c_int as isize),
                    s - 1 as libc::c_int as libc::c_longlong,
                    c,
                    if !y.is_null() {
                        *((*y).k).as_mut_ptr().offset((i_3 % yn0) as isize)
                    } else {
                        0 as *mut k0
                    },
                );
                i_3 += 1;
                i_3;
            }
        }
        if 0 as libc::c_int as libc::c_longlong >= dt {
            let mut k: K = Ki(0 as libc::c_int as I);
            if OOM_CD(
                0 as libc::c_int as I,
                k,
                if !y.is_null() { y } else { k },
                -(1 as libc::c_int) as V,
            ) == 0
            {
                return 0 as K;
            }
            let mut i_4: I = 0 as libc::c_int as I;
            let mut _i_4: I = countI(d);
            while i_4 < _i_4 {
                *(((*k).k).as_mut_ptr() as *mut I) = i_4;
                dot_ref(
                    p,
                    &mut k,
                    z,
                    s,
                    c,
                    if !y.is_null() {
                        *((*y).k).as_mut_ptr().offset((i_4 % yn0) as isize)
                    } else {
                        0 as *mut k0
                    },
                );
                i_4 += 1;
                i_4;
            }
            cd(k);
        }
        cd(y);
    }
    return 0 as K;
}
pub unsafe extern "C" fn dot_tetradic_2(
    mut g: *mut K,
    mut b: K,
    mut c: K,
    mut y: K,
) -> K {
    if (*c).t == 7 as libc::c_int as libc::c_longlong
        && (**((*c).k).as_mut_ptr().offset(CODE as libc::c_int as isize)).t
            == -(4 as libc::c_int) as libc::c_longlong
    {
        let mut q: V = *(((*(*(((*c).k).as_mut_ptr() as *mut S)
            .offset(CODE as libc::c_int as isize) as K))
            .k)
            .as_mut_ptr() as *mut V)
            .offset(0 as libc::c_int as isize);
        if q > 500 as libc::c_int as V {
            return kerr(b"syntax\0" as *const u8 as *const libc::c_char);
        }
        fnc = (*DT.as_mut_ptr().offset(q as L as isize)).text;
        if fnci < 127 as libc::c_int as libc::c_longlong {
            fncp[fnci as usize] = q;
            fnci += 1;
            fnci;
        }
    }
    let mut bt: I = (*b).t;
    let mut bn: I = countI(b);
    if 0 as libc::c_int as libc::c_longlong == bn
        || 6 as libc::c_int as libc::c_longlong == bt
    {
        dot_ref(g, &mut b, 0 as *mut K, bn - 1 as libc::c_int as libc::c_longlong, c, y);
    } else if 0 as libc::c_int as libc::c_longlong == bt
        || 1 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
        || 4 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        b = promote(b);
        bt = 0 as libc::c_int as I;
        bn = countI(b);
        let mut f: *mut K = ((*b).k).as_mut_ptr();
        dot_ref(
            g,
            f,
            if bn > 0 as libc::c_int as libc::c_longlong {
                f.offset(1 as libc::c_int as isize)
            } else {
                0 as *mut K
            },
            bn - 1 as libc::c_int as libc::c_longlong,
            c,
            y,
        );
        cd(b);
    } else {
        return kerr(b"type\0" as *const u8 as *const libc::c_char)
    }
    return *g;
}
pub unsafe extern "C" fn dot_tetradic(mut a: K, mut b: K, mut c: K, mut y: K) -> K {
    if isColonDyadic(c) != 0 && y.is_null()
        && (*(((*c).k).as_mut_ptr() as *mut V).offset(CONJ as libc::c_int as isize))
            .is_null()
    {
        let mut d: K = newK(0 as libc::c_int as I, 2 as libc::c_int as I);
        let mut i: K = Ki(0 as libc::c_int as I);
        if OOM_CD(0 as libc::c_int as I, d, i, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        let ref mut fresh9 = *((*d).k).as_mut_ptr().offset(0 as libc::c_int as isize);
        *fresh9 = i;
        let mut z: K = vf_ex(&mut a as *mut K as V, b);
        let ref mut fresh10 = *((*d).k).as_mut_ptr().offset(1 as libc::c_int as isize);
        *fresh10 = z;
        if z.is_null() {
            *(((*i).k).as_mut_ptr() as *mut I) = 1 as libc::c_int as I;
            let mut e: K = newK(
                -(3 as libc::c_int) as I,
                strlen(errmsg.as_mut_ptr()) as I,
            );
            if OOM_CD(0 as libc::c_int as I, d, e, -(1 as libc::c_int) as V) == 0 {
                return 0 as K;
            }
            strcpy(((*e).k).as_mut_ptr() as *mut C, errmsg.as_mut_ptr());
            let ref mut fresh11 = *((*d).k)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize);
            *fresh11 = e;
        }
        fer = -(1 as libc::c_int) as I;
        return demote(d);
    }
    if !KONA_GSET.is_null() && a != KONA_GSET {
        ci(a);
        cd(KONA_GSET);
        KONA_GSET = a;
    }
    if !KONA_IDX.is_null() && b != KONA_IDX {
        ci(b);
        cd(KONA_IDX);
        KONA_IDX = b;
    }
    let mut q: K = 0 as K;
    let mut p: *mut K = 0 as *mut K;
    if (*a).t == 4 as libc::c_int as libc::c_longlong {
        p = denameS(d_, *(((*a).k).as_mut_ptr() as *mut S), 1 as libc::c_int as I);
        if p.is_null() {
            return 0 as K;
        }
    } else {
        q = kcloneI(
            a,
            b"src/vd.c\0" as *const u8 as *const libc::c_char,
            295 as libc::c_int,
        );
    }
    let mut g: *mut K = if !q.is_null() { &mut q } else { p };
    if (dot_tetradic_2(g, b, c, y)).is_null() {
        return 0 as K;
    }
    return if !q.is_null() { q } else { ci(a) };
}
pub unsafe extern "C" fn make(mut a: K) -> K {
    if (makeable(a)).is_null() {
        return kerr(b"rank\0" as *const u8 as *const libc::c_char);
    }
    let mut n: I = (*a).n;
    let mut x: K = 0 as *mut k0;
    let mut y: K = 0 as *mut k0;
    let mut z: K = newK(5 as libc::c_int as I, n);
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = n;
    while i < _i {
        let ref mut fresh12 = *((*z).k).as_mut_ptr().offset(i as isize);
        *fresh12 = newK(0 as libc::c_int as I, 3 as libc::c_int as I);
        i += 1;
        i;
    }
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_0: I = n;
    while i_0 < _i_0 {
        x = *((*z).k).as_mut_ptr().offset(i_0 as isize);
        y = *((*a).k).as_mut_ptr().offset(i_0 as isize);
        let mut j: I = 0 as libc::c_int as I;
        let mut _j: I = (*y).n;
        while j < _j {
            let ref mut fresh13 = *((*x).k).as_mut_ptr().offset(j as isize);
            *fresh13 = if (*y).t != 0 {
                Ks(*(((*y).k).as_mut_ptr() as *mut S).offset(j as isize))
            } else {
                ci(*((*y).k).as_mut_ptr().offset(j as isize))
            };
            j += 1;
            j;
        }
        if (*y).n < 3 as libc::c_int as libc::c_longlong {
            let ref mut fresh14 = *((*x).k)
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize);
            *fresh14 = _n();
        }
        i_0 += 1;
        i_0;
    }
    return z;
}
unsafe extern "C" fn unmake(mut a: K) -> K {
    let mut z: K = kcloneI(
        a,
        b"src/vd.c\0" as *const u8 as *const libc::c_char,
        321 as libc::c_int,
    );
    (*z).t = 0 as libc::c_int as I;
    return z;
}
unsafe extern "C" fn makeable(mut a: K) -> K {
    let mut t: I = (*a).t;
    let mut n: I = (*a).n;
    if 0 as libc::c_int as libc::c_longlong != t {
        return 0 as K;
    }
    let mut x: K = 0 as *mut k0;
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = n;
    while i < _i {
        x = *((*a).k).as_mut_ptr().offset(i as isize);
        if 0 as libc::c_int as libc::c_longlong != (*x).t
            && -(4 as libc::c_int) as libc::c_longlong != (*x).t
            || (*x).n < 2 as libc::c_int as libc::c_longlong
            || (3 as libc::c_int as libc::c_longlong) < (*x).n
            || -(4 as libc::c_int) as libc::c_longlong == (*x).t
                && (*x).n != 2 as libc::c_int as libc::c_longlong
        {
            return 0 as K;
        }
        i += 1;
        i;
    }
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_0: I = n;
    while i_0 < _i_0 {
        x = *((*a).k).as_mut_ptr().offset(i_0 as isize);
        if 0 as libc::c_int as libc::c_longlong == (*x).t {
            if 4 as libc::c_int as libc::c_longlong
                != (**((*x).k).as_mut_ptr().offset(0 as libc::c_int as isize)).t
                || 3 as libc::c_int as libc::c_longlong == (*x).n
                    && 5 as libc::c_int as libc::c_longlong
                        != (**((*x).k).as_mut_ptr().offset(2 as libc::c_int as isize)).t
                    && 6 as libc::c_int as libc::c_longlong
                        != (**((*x).k).as_mut_ptr().offset(2 as libc::c_int as isize)).t
            {
                return 0 as K;
            }
        }
        i_0 += 1;
        i_0;
    }
    return 1 as libc::c_int as K;
}
pub unsafe extern "C" fn dot_monadic(mut x: K) -> K {
    if (*x).t == 0 as libc::c_int as libc::c_longlong
        && (*x).n == 1 as libc::c_int as libc::c_longlong
        && (**((*x).k).as_mut_ptr().offset(0 as libc::c_int as isize)).t
            == -(3 as libc::c_int) as libc::c_longlong
    {
        return kerr(b"valence\0" as *const u8 as *const libc::c_char);
    }
    if 3 as libc::c_int as libc::c_longlong
        == (if (*x).t < 0 as libc::c_int as libc::c_longlong { -(*x).t } else { (*x).t })
    {
        return KX(x);
    }
    if 4 as libc::c_int as libc::c_longlong == (*x).t {
        let mut p: *mut K = denameS(
            d_,
            *(((*x).k).as_mut_ptr() as *mut S),
            0 as libc::c_int as I,
        );
        if p.is_null() {
            return kerr(b"domain\0" as *const u8 as *const libc::c_char);
        }
        return ci(*p);
    }
    if 5 as libc::c_int as libc::c_longlong == (*x).t {
        return unmake(x);
    }
    if !(makeable(x)).is_null() {
        return make(x);
    }
    return vf_ex(offsetDot, x);
}
