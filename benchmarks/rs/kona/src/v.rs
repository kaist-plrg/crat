use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn cd(a: K) -> K;
    static mut offsetDot: V;
    static mut offsetColon: V;
    fn dot(a: K, b: K) -> K;
    fn enlist(x: K) -> K;
    fn simpleString(a: S) -> I;
    fn ex(a: K) -> K;
    static mut LS: S;
    static mut NIL: K;
    static mut d_: S;
    static mut KTREE: K;
    fn kerr(s: cS) -> K;
    fn newEntry(s: S) -> K;
    fn Ks(x: S) -> K;
    fn Kc(x: C) -> K;
    fn Kf(x: F) -> K;
    fn Ki(x: I) -> K;
    fn kap(a: *mut K, v: V) -> K;
    fn _n() -> K;
    fn newK(t: I, n: I) -> K;
    fn ci(a: K) -> K;
    fn OOM_CD(g: I, _: ...) -> I;
    fn popen_charvec(cmd: *mut C) -> K;
    #[link_name = "mod"]
    fn mod_0(a: K, b: K) -> K;
    fn sp(k: S) -> S;
    fn strdupn(s: S, k: I) -> S;
    fn alloc(sz: size_t) -> V;
    fn rotate(x: K, y: K) -> K;
    fn dot_tetradic(x: K, y: K, z: K, w: K) -> K;
    fn collapse(x: K) -> K;
    fn of(x: K, y: K) -> K;
    fn wd_(s: S, n: libc::c_int, dict: *mut K, func: K) -> K;
    fn demote(a: K) -> K;
    fn promote(a: K) -> K;
    fn vf_ex(q: V, g: K) -> K;
    fn dv_ex(a: K, p: *mut V, b: K) -> K;
}
pub type size_t = libc::c_ulong;
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
pub unsafe extern "C" fn itemAtIndex(mut a: K, mut i: I) -> K {
    let mut at_0: I = (*a).t;
    if (0 as libc::c_int as libc::c_longlong) < at_0 {
        return ci(a);
    }
    if -(4 as libc::c_int) as libc::c_longlong == at_0 {
        return Ks(*(((*a).k).as_mut_ptr() as *mut S).offset(i as isize));
    }
    if -(3 as libc::c_int) as libc::c_longlong == at_0 {
        return Kc(*(((*a).k).as_mut_ptr() as *mut C).offset(i as isize));
    }
    if -(2 as libc::c_int) as libc::c_longlong == at_0 {
        return Kf(*(((*a).k).as_mut_ptr() as *mut F).offset(i as isize));
    }
    if -(1 as libc::c_int) as libc::c_longlong == at_0 {
        return Ki(*(((*a).k).as_mut_ptr() as *mut I).offset(i as isize));
    }
    return ci(*((*a).k).as_mut_ptr().offset(i as isize));
}
pub unsafe extern "C" fn glueSS(mut c: S, mut d: S) -> S {
    let mut x: I = strlen(c as *const libc::c_char) as I;
    let mut y: I = strlen(d as *const libc::c_char) as I;
    let mut m: S = alloc((x + y + 2 as libc::c_int as libc::c_longlong) as size_t) as S;
    sprintf(m, b"%s.%s\0" as *const u8 as *const libc::c_char, c, d);
    return m;
}
pub unsafe extern "C" fn glue(mut a: K, mut b: K) -> K {
    return Ks(
        sp(
            glueSS(
                *(((*a).k).as_mut_ptr() as *mut S),
                *(((*b).k).as_mut_ptr() as *mut S),
            ),
        ),
    );
}
pub unsafe extern "C" fn DI(mut d: K, mut i: I) -> K {
    return *((*d).k).as_mut_ptr().offset(i as isize);
}
pub unsafe extern "C" fn ES(mut d: K) -> S {
    return *(((*(*((*d).k).as_mut_ptr().offset(0 as libc::c_int as isize) as K)).k)
        .as_mut_ptr() as *mut S);
}
pub unsafe extern "C" fn DE(mut d: K, mut b: S) -> K {
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = (*d).n;
    while i < _i {
        let mut x: K = DI(d, i);
        if b == ES(x) {
            return x;
        }
        i += 1;
        i;
    }
    return 0 as K;
}
unsafe extern "C" fn EIA(mut a: K, mut i: I) -> *mut K {
    return ((*a).k).as_mut_ptr().offset(i as isize);
}
pub unsafe extern "C" fn EVP(mut e: K) -> *mut K {
    return EIA(e, 1 as libc::c_int as I);
}
pub unsafe extern "C" fn EAP(mut e: K) -> *mut K {
    return EIA(e, 2 as libc::c_int as I);
}
pub unsafe extern "C" fn EV(mut e: K) -> K {
    return *EVP(e);
}
pub unsafe extern "C" fn lookupEntryOrCreate(mut p: *mut K, mut k: S) -> K {
    let mut a: K = *p;
    let mut x: K = 0 as *mut k0;
    if 5 as libc::c_int as libc::c_longlong == (*a).t {
        x = DE(a, k);
        if !x.is_null() {
            return x;
        }
    }
    if strlen(k as *const libc::c_char) == 0 {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    if !(strchr(k as *const libc::c_char, '.' as i32)).is_null() {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    x = newEntry(k);
    if 6 as libc::c_int as libc::c_longlong == (*a).t {
        cd(*p);
        *p = newK(5 as libc::c_int as I, 0 as libc::c_int as I);
    }
    kap(p, &mut x as *mut K as V);
    cd(x);
    return x;
}
unsafe extern "C" fn denameRecurse(mut p: *mut K, mut t: S, mut create: I) -> *mut K {
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
            return &mut NIL;
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
    return denameRecurse(p, t, create);
}
pub unsafe extern "C" fn denameD(mut d: *mut K, mut t: S, mut create: I) -> *mut K {
    if simpleString(t) == 0 {
        return 0 as *mut K;
    }
    return denameRecurse(
        if '.' as i32 == *t as libc::c_int || *t == 0 { &mut KTREE } else { d },
        t,
        create,
    );
}
pub unsafe extern "C" fn denameS(mut dir_string: S, mut t: S, mut create: I) -> *mut K {
    return denameD(
        if '.' as i32 == *t as libc::c_int || *t == 0 {
            &mut KTREE
        } else {
            denameD(&mut KTREE, dir_string, create)
        },
        t,
        create,
    );
}
pub unsafe extern "C" fn lookupEVOrCreate(mut p: *mut K, mut k: S) -> *mut K {
    let mut x: K = lookupEntryOrCreate(p, k);
    return if !x.is_null() { EVP(x) } else { 0 as *mut K };
}
pub unsafe extern "C" fn lookup(mut a: K, mut b: S) -> K {
    let mut x: K = DE(a, b);
    return if !x.is_null() { EV(x) } else { _n() };
}
unsafe extern "C" fn isVerbDyadic(mut x: K, mut v: V) -> I {
    return ((*x).t == 7 as libc::c_int as libc::c_longlong
        && *(((*(*(((*x).k).as_mut_ptr() as *mut V).offset(CODE as libc::c_int as isize)
            as K))
            .k)
            .as_mut_ptr() as *mut S as *mut V)
            .offset(0 as libc::c_int as isize) == v
        && (*(((*(*(((*x).k).as_mut_ptr() as *mut V).offset(CODE as libc::c_int as isize)
            as K))
            .k)
            .as_mut_ptr() as *mut S as *mut V)
            .offset(1 as libc::c_int as isize))
            .is_null()) as libc::c_int as I;
}
pub unsafe extern "C" fn isColonDyadic(mut x: K) -> I {
    return isVerbDyadic(x, offsetColon);
}
unsafe extern "C" fn isDotDyadic(mut x: K) -> I {
    return isVerbDyadic(x, offsetDot);
}
pub unsafe extern "C" fn at_verb(mut a: K, mut b: K) -> K {
    if b.is_null() {
        return b;
    }
    if 0 as libc::c_int as libc::c_longlong == (*b).t
        && 0 as libc::c_int as libc::c_longlong == (*b).n
    {
        return newK(0 as libc::c_int as I, 0 as libc::c_int as I);
    }
    let mut at_0: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    let mut z: K = 0 as *mut k0;
    if at_0 == 6 as libc::c_int as libc::c_longlong {
        if 1 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
        {
            return ci(b);
        }
        if 6 as libc::c_int as libc::c_longlong == bt
            || 0 as libc::c_int as libc::c_longlong >= bt
                && 0 as libc::c_int as libc::c_longlong == bn
        {
            return newK(0 as libc::c_int as I, 0 as libc::c_int as I);
        }
        if 4 as libc::c_int as libc::c_longlong == bt {
            return _n();
        }
        if -(4 as libc::c_int) as libc::c_longlong == bt {
            z = newK(0 as libc::c_int as I, bn);
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = bn;
            while i < _i {
                let ref mut fresh0 = *((*z).k).as_mut_ptr().offset(i as isize);
                *fresh0 = _n();
                i += 1;
                i;
            }
            return z;
        }
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    if 1 as libc::c_int as libc::c_longlong
        == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if at_0 == 7 as libc::c_int as libc::c_longlong
            && an == 3 as libc::c_int as libc::c_longlong
        {
            let mut p: *mut K = &mut a;
            return dv_ex(0 as K, &mut p as *mut *mut K as *mut V, b);
        }
        if (0 as libc::c_int as libc::c_longlong) < at_0 {
            return kerr(b"type\0" as *const u8 as *const libc::c_char);
        }
        let mut x: I = 0;
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = bn;
        while i_0 < _i_0 {
            x = *(((*b).k).as_mut_ptr() as *mut I).offset(i_0 as isize);
            if x >= an || x < 0 as libc::c_int as libc::c_longlong {
                return kerr(b"index\0" as *const u8 as *const libc::c_char);
            }
            i_0 += 1;
            i_0;
        }
        z = newK(at_0 * -bt, bn);
        if -(4 as libc::c_int) as libc::c_longlong == at_0 {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = bn;
            while i_1 < _i_1 {
                let ref mut fresh1 = *(((*z).k).as_mut_ptr() as *mut S)
                    .offset(i_1 as isize);
                *fresh1 = *(((*a).k).as_mut_ptr() as *mut S)
                    .offset(
                        *(((*b).k).as_mut_ptr() as *mut I).offset(i_1 as isize) as isize,
                    );
                i_1 += 1;
                i_1;
            }
        } else if -(3 as libc::c_int) as libc::c_longlong == at_0 {
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = bn;
            while i_2 < _i_2 {
                *(((*z).k).as_mut_ptr() as *mut C)
                    .offset(
                        i_2 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut C)
                    .offset(
                        *(((*b).k).as_mut_ptr() as *mut I).offset(i_2 as isize) as isize,
                    );
                i_2 += 1;
                i_2;
            }
        } else if -(2 as libc::c_int) as libc::c_longlong == at_0 {
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = bn;
            while i_3 < _i_3 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_3 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F)
                    .offset(
                        *(((*b).k).as_mut_ptr() as *mut I).offset(i_3 as isize) as isize,
                    );
                i_3 += 1;
                i_3;
            }
        } else if -(1 as libc::c_int) as libc::c_longlong == at_0 {
            let mut i_4: I = 0 as libc::c_int as I;
            let mut _i_4: I = bn;
            while i_4 < _i_4 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_4 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I)
                    .offset(
                        *(((*b).k).as_mut_ptr() as *mut I).offset(i_4 as isize) as isize,
                    );
                i_4 += 1;
                i_4;
            }
        } else if 0 as libc::c_int as libc::c_longlong == at_0 {
            let mut i_5: I = 0 as libc::c_int as I;
            let mut _i_5: I = bn;
            while i_5 < _i_5 {
                let ref mut fresh2 = *((*z).k).as_mut_ptr().offset(i_5 as isize);
                *fresh2 = ci(
                    *((*a).k)
                        .as_mut_ptr()
                        .offset(
                            *(((*b).k).as_mut_ptr() as *mut I).offset(i_5 as isize)
                                as isize,
                        ),
                );
                i_5 += 1;
                i_5;
            }
            if bt == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
                || bn != 1 as libc::c_int as libc::c_longlong
            {
                z = collapse(z);
            }
        }
    } else if 3 as libc::c_int as libc::c_longlong
        == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if 5 as libc::c_int as libc::c_longlong != at_0 {
            return kerr(b"type\0" as *const u8 as *const libc::c_char);
        }
        z = ex(wd_(((*b).k).as_mut_ptr() as *mut C, bn as libc::c_int, &mut a, 0 as K));
    } else if 4 as libc::c_int as libc::c_longlong
        == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if 5 as libc::c_int as libc::c_longlong != at_0 {
            return kerr(b"type\0" as *const u8 as *const libc::c_char);
        }
        z = newK(0 as libc::c_int as I, bn);
        let mut i_6: I = 0 as libc::c_int as I;
        let mut _i_6: I = bn;
        while i_6 < _i_6 {
            let ref mut fresh3 = *((*z).k).as_mut_ptr().offset(i_6 as isize);
            *fresh3 = ci(
                lookup(a, *(((*b).k).as_mut_ptr() as *mut S).offset(i_6 as isize)),
            );
            i_6 += 1;
            i_6;
        }
        if !(bt < 0 as libc::c_int as libc::c_longlong
            && bn == 1 as libc::c_int as libc::c_longlong)
        {
            z = collapse(z);
        }
        if (*z).t == 0 as libc::c_int as libc::c_longlong
            && (*z).n == 1 as libc::c_int as libc::c_longlong
            && (**((*z).k).as_mut_ptr().offset(0 as libc::c_int as isize)).t
                == 1 as libc::c_int as libc::c_longlong
        {
            let mut zz: K = enlist(
                *((*z).k).as_mut_ptr().offset(0 as libc::c_int as isize),
            );
            cd(z);
            return zz;
        }
    } else if 6 as libc::c_int as libc::c_longlong == bt {
        if 0 as libc::c_int as libc::c_longlong >= at_0 {
            z = ci(a);
        } else if 5 as libc::c_int as libc::c_longlong == at_0 {
            z = newK(0 as libc::c_int as I, an);
            let mut i_7: I = 0 as libc::c_int as I;
            let mut _i_7: I = an;
            while i_7 < _i_7 {
                let ref mut fresh4 = *((*z).k).as_mut_ptr().offset(i_7 as isize);
                *fresh4 = ci(EV(DI(a, i_7)));
                i_7 += 1;
                i_7;
            }
            z = collapse(z);
        } else {
            return kerr(b"type\0" as *const u8 as *const libc::c_char)
        }
    } else if 0 as libc::c_int as libc::c_longlong == bt {
        z = newK(0 as libc::c_int as I, bn);
        if z.is_null() {
            return 0 as K;
        }
        let mut i_8: I = 0 as libc::c_int as I;
        let mut _i_8: I = bn;
        while i_8 < _i_8 {
            let ref mut fresh5 = *((*z).k).as_mut_ptr().offset(i_8 as isize);
            *fresh5 = at_verb(a, *((*b).k).as_mut_ptr().offset(i_8 as isize));
            if OOM_CD(0 as libc::c_int as I, z, *fresh5, -(1 as libc::c_int) as V) == 0 {
                return 0 as K;
            }
            i_8 += 1;
            i_8;
        }
    } else if isDotDyadic(b) != 0 && at_0 == 5 as libc::c_int as libc::c_longlong {
        z = newK(0 as libc::c_int as I, an);
        let mut i_9: I = 0 as libc::c_int as I;
        let mut _i_9: I = an;
        while i_9 < _i_9 {
            let ref mut fresh6 = *((*z).k).as_mut_ptr().offset(i_9 as isize);
            *fresh6 = ci(*EAP(DI(a, i_9)));
            i_9 += 1;
            i_9;
        }
    } else {
        return kerr(b"type\0" as *const u8 as *const libc::c_char)
    }
    return z;
}
pub unsafe extern "C" fn at(mut x: K, mut y: K) -> K {
    let mut a: K = 0 as *mut k0;
    let mut z: K = 0 as *mut k0;
    if (*x).t == 4 as libc::c_int as libc::c_longlong {
        if 1 as libc::c_int as libc::c_longlong
            == (if (*y).t < 0 as libc::c_int as libc::c_longlong {
                -(*y).t
            } else {
                (*y).t
            })
        {
            let mut s: [C; 256] = [0; 256];
            strcpy(s.as_mut_ptr(), d_ as *const libc::c_char);
            strcat(s.as_mut_ptr(), b".\0" as *const u8 as *const libc::c_char);
            strcat(
                s.as_mut_ptr(),
                *(((*x).k).as_mut_ptr() as *mut S) as *const libc::c_char,
            );
            let mut xx: *mut K = denameD(
                &mut KTREE,
                sp(s.as_mut_ptr()),
                1 as libc::c_int as I,
            );
            if 6 as libc::c_int as libc::c_longlong == (**xx).t {
                return ci(y)
            } else {
                return of(*xx, y)
            }
        } else {
            return kerr(b"nyi\0" as *const u8 as *const libc::c_char)
        }
    }
    if 7 as libc::c_int as libc::c_longlong != (*x).t {
        return at_verb(x, y);
    }
    a = enlist(y);
    if OOM_CD(0 as libc::c_int as I, a, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    z = dot(x, a);
    cd(a);
    return z;
}
unsafe extern "C" fn updateIndex(mut p: *mut K, mut x: I, mut r: K) -> I {
    let mut pt: I = (**p).t;
    let mut rt: I = (*r).t;
    if 0 as libc::c_int as libc::c_longlong == pt {
        cd(*((**p).k).as_mut_ptr().offset(x as isize));
        let ref mut fresh7 = *((**p).k).as_mut_ptr().offset(x as isize);
        *fresh7 = ci(r);
        *p = demote(*p);
    } else if pt != -rt {
        let mut t: K = promote(*p);
        cd(*p);
        *p = t;
        cd(*((**p).k).as_mut_ptr().offset(x as isize));
        let ref mut fresh8 = *((**p).k).as_mut_ptr().offset(x as isize);
        *fresh8 = ci(r);
    } else {
        if -(4 as libc::c_int) as libc::c_longlong == pt {
            let ref mut fresh9 = *(((**p).k).as_mut_ptr() as *mut S).offset(x as isize);
            *fresh9 = *(((*r).k).as_mut_ptr() as *mut S);
        }
        if -(3 as libc::c_int) as libc::c_longlong == pt {
            *(((**p).k).as_mut_ptr() as *mut C)
                .offset(x as isize) = *(((*r).k).as_mut_ptr() as *mut C);
        }
        if -(2 as libc::c_int) as libc::c_longlong == pt {
            *(((**p).k).as_mut_ptr() as *mut F)
                .offset(x as isize) = *(((*r).k).as_mut_ptr() as *mut F);
        }
        if -(1 as libc::c_int) as libc::c_longlong == pt {
            *(((**p).k).as_mut_ptr() as *mut I)
                .offset(x as isize) = *(((*r).k).as_mut_ptr() as *mut I);
        }
    }
    return 0 as libc::c_int as I;
}
pub unsafe extern "C" fn specialAmendDot(mut c: K, mut args: K) -> K {
    if isColonDyadic(c) != 0
        && (*(((*c).k).as_mut_ptr() as *mut V).offset(CONJ as libc::c_int as isize))
            .is_null()
    {
        return if 2 as libc::c_int as libc::c_longlong == (*args).n {
            ci(*((*args).k).as_mut_ptr().offset(1 as libc::c_int as isize))
        } else {
            _n()
        };
    }
    return vf_ex(&mut c as *mut K as V, args);
}
pub unsafe extern "C" fn atomI(mut a: K) -> I {
    return (if (*a).t > 0 as libc::c_int as libc::c_longlong {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as I;
}
pub unsafe extern "C" fn atom(mut a: K) -> K {
    return Ki(atomI(a));
}
pub unsafe extern "C" fn at_ref(mut p: *mut K, mut b: K, mut c: K, mut y: K) -> K {
    let mut pt: I = (**p).t;
    let mut pn: I = (**p).n;
    if pt > 0 as libc::c_int as libc::c_longlong
        && pt != 5 as libc::c_int as libc::c_longlong
        && pt != 6 as libc::c_int as libc::c_longlong
    {
        return kerr(b"rank\0" as *const u8 as *const libc::c_char);
    }
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    if 0 as libc::c_int as libc::c_longlong == bn
        && (-(1 as libc::c_int) as libc::c_longlong == bt
            || 0 as libc::c_int as libc::c_longlong == bt
            || 6 as libc::c_int as libc::c_longlong == pt)
    {
        return 0 as K;
    }
    if 0 as libc::c_int as libc::c_longlong == bn
        && bt <= 0 as libc::c_int as libc::c_longlong
    {
        return kerr(b"int\0" as *const u8 as *const libc::c_char);
    }
    if !y.is_null() && atomI(b) == 0 && atomI(y) == 0 && bn != (*y).n {
        return 0 as K;
    }
    let mut n: I = if !y.is_null() && atomI(b) != 0 { (*y).n } else { bn };
    let mut argc: I = (if !y.is_null() { 2 as libc::c_int } else { 1 as libc::c_int })
        as I;
    if 1 as libc::c_int as libc::c_longlong
        == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if 5 as libc::c_int as libc::c_longlong == pt
            || 6 as libc::c_int as libc::c_longlong == pt
        {
            return kerr(b"type\0" as *const u8 as *const libc::c_char);
        }
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = bn;
        while i < _i {
            let mut x: I = *(((*b).k).as_mut_ptr() as *mut I).offset(i as isize);
            if x < 0 as libc::c_int as libc::c_longlong || x >= pn {
                return kerr(b"index\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
            i;
        }
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = if atomI(b) != 0 {
            1 as libc::c_int as libc::c_longlong
        } else {
            n
        };
        while i_0 < _i_0 {
            let mut args: K = newK(0 as libc::c_int as I, argc);
            if args.is_null() {
                return 0 as K;
            }
            let ref mut fresh10 = *((*args).k)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize);
            *fresh10 = itemAtIndex(
                *p,
                *(((*b).k).as_mut_ptr() as *mut I).offset((i_0 % bn) as isize),
            );
            if argc > 1 as libc::c_int as libc::c_longlong {
                let ref mut fresh11 = *((*args).k)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize);
                *fresh11 = if atomI(b) != 0 {
                    ci(y)
                } else {
                    itemAtIndex(y, i_0 % (*y).n)
                };
            }
            let mut r: K = specialAmendDot(c, args);
            if OOM_CD(0 as libc::c_int as I, r, args, -(1 as libc::c_int) as V) == 0 {
                return 0 as K;
            }
            updateIndex(
                p,
                *(((*b).k).as_mut_ptr() as *mut I).offset((i_0 % bn) as isize),
                r,
            );
            cd(r);
            cd(args);
            i_0 += 1;
            i_0;
        }
    } else if 4 as libc::c_int as libc::c_longlong
        == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if 5 as libc::c_int as libc::c_longlong != pt
            && 6 as libc::c_int as libc::c_longlong != pt
        {
            return kerr(b"type\0" as *const u8 as *const libc::c_char);
        }
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = if atomI(b) != 0 {
            1 as libc::c_int as libc::c_longlong
        } else {
            n
        };
        while i_1 < _i_1 {
            let mut args_0: K = newK(0 as libc::c_int as I, argc);
            if args_0.is_null() {
                return 0 as K;
            }
            let mut u: S = *(((*b).k).as_mut_ptr() as *mut S)
                .offset((i_1 % bn) as isize);
            if strlen(u as *const libc::c_char) == 0 {
                return kerr(b"domain\0" as *const u8 as *const libc::c_char);
            }
            let ref mut fresh12 = *((*args_0).k)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize);
            *fresh12 = ci(*lookupEVOrCreate(p, u));
            if argc > 1 as libc::c_int as libc::c_longlong {
                let ref mut fresh13 = *((*args_0).k)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize);
                *fresh13 = if atomI(b) != 0 {
                    ci(y)
                } else {
                    itemAtIndex(y, i_1 % (*y).n)
                };
            }
            let mut r_0: K = specialAmendDot(c, args_0);
            if OOM_CD(0 as libc::c_int as I, r_0, args_0, -(1 as libc::c_int) as V) == 0
            {
                return 0 as K;
            }
            let mut v: *mut K = EVP(DE(*p, u));
            cd(*v);
            *v = r_0;
            cd(args_0);
            i_1 += 1;
            i_1;
        }
    } else if 6 as libc::c_int as libc::c_longlong == bt {
        if !y.is_null() && atomI(y) == 0 && (*y).n != pn {
            return kerr(b"length\0" as *const u8 as *const libc::c_char);
        }
        if 6 as libc::c_int as libc::c_longlong == pt {
            return 0 as K;
        }
        let mut k: K = if 5 as libc::c_int as libc::c_longlong == pt {
            Ks(LS)
        } else {
            Ki(0 as libc::c_int as I)
        };
        if k.is_null() {
            return 0 as K;
        }
        if !y.is_null() {
            y = promote(y);
            if OOM_CD(0 as libc::c_int as I, k, y, -(1 as libc::c_int) as V) == 0 {
                return 0 as K;
            }
        }
        if 5 as libc::c_int as libc::c_longlong == pt {
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = pn;
            while i_2 < _i_2 {
                let ref mut fresh14 = *(((*k).k).as_mut_ptr() as *mut S);
                *fresh14 = ES(DI(*p, i_2));
                at_ref(
                    p,
                    k,
                    c,
                    if !y.is_null() {
                        *((*y).k).as_mut_ptr().offset((i_2 % (*y).n) as isize)
                    } else {
                        0 as *mut k0
                    },
                );
                i_2 += 1;
                i_2;
            }
        } else {
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = pn;
            while i_3 < _i_3 {
                *(((*k).k).as_mut_ptr() as *mut I) = i_3;
                at_ref(
                    p,
                    k,
                    c,
                    if !y.is_null() {
                        *((*y).k).as_mut_ptr().offset((i_3 % (*y).n) as isize)
                    } else {
                        0 as *mut k0
                    },
                );
                i_3 += 1;
                i_3;
            }
        }
        cd(k);
        cd(y);
    } else if 0 as libc::c_int as libc::c_longlong == bt {
        let mut i_4: I = 0 as libc::c_int as I;
        let mut _i_4: I = n;
        while i_4 < _i_4 {
            let mut e: K = 0 as K;
            if !y.is_null() {
                e = itemAtIndex(y, i_4 % (*y).n);
                if e.is_null() {
                    return 0 as K;
                }
            }
            at_ref(p, *((*b).k).as_mut_ptr().offset((i_4 % bn) as isize), c, e);
            cd(e);
            i_4 += 1;
            i_4;
        }
    } else {
        return kerr(b"type\0" as *const u8 as *const libc::c_char)
    }
    return 0 as K;
}
pub unsafe extern "C" fn at_tetradic(mut a: K, mut b: K, mut c: K, mut y: K) -> K {
    let mut d: K = enlist(b);
    if d.is_null() {
        return 0 as K;
    }
    let mut e: K = dot_tetradic(a, d, c, y);
    cd(d);
    return e;
}
pub unsafe extern "C" fn colon_monadic(mut a: K) -> K {
    return ci(a);
}
pub unsafe extern "C" fn colon_dyadic(mut a: K, mut b: K) -> K {
    return ci(b);
}
unsafe extern "C" fn notsp(mut a: S) -> S {
    let mut b: I = strlen(a as *const libc::c_char) as I;
    let mut c: S = strcpy(
        alloc((b + 2 as libc::c_int as libc::c_longlong) as size_t) as *mut libc::c_char,
        a as *const libc::c_char,
    );
    if c.is_null() {
        return 0 as S;
    }
    *c.offset(b as isize) = '.' as i32 as C;
    *c.offset((b + 1 as libc::c_int as libc::c_longlong) as isize) = '\0' as i32 as C;
    let mut d: S = sp(c);
    free(c as *mut libc::c_void);
    return d;
}
pub unsafe extern "C" fn not_attribute(mut a: K) -> K {
    let mut t: I = (*a).t;
    let mut n: I = (*a).n;
    let mut z: K = 0 as *mut k0;
    if 4 as libc::c_int as libc::c_longlong
        == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        z = newK(t, n);
        if z.is_null() {
            return 0 as K;
        }
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = n;
        while i < _i {
            let ref mut fresh15 = *(((*z).k).as_mut_ptr() as *mut S).offset(i as isize);
            *fresh15 = notsp(*(((*a).k).as_mut_ptr() as *mut S).offset(i as isize));
            if (*fresh15).is_null() {
                cd(z);
                return 0 as K;
            }
            i += 1;
            i;
        }
    } else if 2 as libc::c_int as libc::c_longlong
        == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        z = newK(t / 2 as libc::c_int as libc::c_longlong, n);
        if z.is_null() {
            return 0 as K;
        }
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = n;
        while i_0 < _i_0 {
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    i_0 as isize,
                ) = (if 0 as libc::c_int as libc::c_double
                == *(((*a).k).as_mut_ptr() as *mut F).offset(i_0 as isize)
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as I;
            i_0 += 1;
            i_0;
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        z = newK(t, n);
        if z.is_null() {
            return 0 as K;
        }
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = n;
        while i_1 < _i_1 {
            *(((*z).k).as_mut_ptr() as *mut I)
                .offset(
                    i_1 as isize,
                ) = (if 0 as libc::c_int as libc::c_longlong
                == *(((*a).k).as_mut_ptr() as *mut I).offset(i_1 as isize)
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as I;
            i_1 += 1;
            i_1;
        }
    } else if 0 as libc::c_int as libc::c_longlong
        == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        z = newK(t, n);
        if z.is_null() {
            return 0 as K;
        }
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = n;
        while i_2 < _i_2 {
            let ref mut fresh16 = *((*z).k).as_mut_ptr().offset(i_2 as isize);
            *fresh16 = not_attribute(*((*a).k).as_mut_ptr().offset(i_2 as isize));
            if (*fresh16).is_null() {
                cd(z);
                return 0 as K;
            }
            i_2 += 1;
            i_2;
        }
    } else {
        return kerr(b"type\0" as *const u8 as *const libc::c_char)
    }
    return z;
}
unsafe extern "C" fn excl_mkdict(mut a: K, mut b: K) -> K {
    let mut n: I = (*a).n;
    let mut k: K = 0 as *mut k0;
    let mut v: K = 0 as *mut k0;
    let mut t: K = 0 as *mut k0;
    let mut z: K = 0 as *mut k0;
    z = newK(5 as libc::c_int as I, n);
    if z.is_null() {
        return 0 as K;
    }
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = n;
    while i < _i {
        k = Ks(*(((*a).k).as_mut_ptr() as *mut S).offset(i as isize));
        t = newK(0 as libc::c_int as I, 3 as libc::c_int as I);
        v = ci(*((*b).k).as_mut_ptr().offset(i as isize));
        if OOM_CD(0 as libc::c_int as I, z, k, t, v, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        let ref mut fresh17 = *((*t).k).as_mut_ptr().offset(0 as libc::c_int as isize);
        *fresh17 = k;
        let ref mut fresh18 = *((*t).k).as_mut_ptr().offset(1 as libc::c_int as isize);
        *fresh18 = v;
        let ref mut fresh19 = *((*t).k).as_mut_ptr().offset(2 as libc::c_int as isize);
        *fresh19 = _n();
        let ref mut fresh20 = *((*z).k).as_mut_ptr().offset(i as isize);
        *fresh20 = t;
        i += 1;
        i;
    }
    return z;
}
pub unsafe extern "C" fn rotate_mod(mut a: K, mut b: K) -> K {
    if (*b).t > 2 as libc::c_int as libc::c_longlong {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    if 4 as libc::c_int as libc::c_longlong
        == (if (*a).t < 0 as libc::c_int as libc::c_longlong { -(*a).t } else { (*a).t })
        && 0 as libc::c_int as libc::c_longlong == (*b).t && (*a).n == (*b).n
    {
        return excl_mkdict(a, b);
    }
    if !(1 as libc::c_int as libc::c_longlong == (*a).t
        || (*b).t > 0 as libc::c_int as libc::c_longlong)
    {
        return kerr(b"int\0" as *const u8 as *const libc::c_char);
    }
    return if (*b).t < 1 as libc::c_int as libc::c_longlong {
        rotate(a, b)
    } else {
        mod_0(a, b)
    };
}
unsafe extern "C" fn enumerate_charvec(mut pth: *mut C) -> K {
    let mut z: K = 0 as *mut k0;
    let mut len: I = (strlen(pth)).wrapping_add(3 as libc::c_int as libc::c_ulong) as I;
    let mut p: K = newK(-(3 as libc::c_int) as I, len);
    snprintf(
        ((*p).k).as_mut_ptr() as *mut C,
        len as libc::c_ulong,
        b"ls %s\0" as *const u8 as *const libc::c_char,
        pth,
    );
    z = popen_charvec(((*p).k).as_mut_ptr() as *mut C);
    cd(p);
    return z;
}
pub unsafe extern "C" fn enumerate(mut a: K) -> K {
    let mut t: I = (*a).t;
    let mut z: K = 0 as *mut k0;
    if 6 as libc::c_int as libc::c_longlong == t {
        z = newK(-(4 as libc::c_int) as I, 0 as libc::c_int as I);
    } else if 5 as libc::c_int as libc::c_longlong == t {
        let mut n: I = (*a).n;
        z = newK(-(4 as libc::c_int) as I, n);
        if z.is_null() {
            return 0 as K;
        }
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = n;
        while i < _i {
            let ref mut fresh21 = *(((*z).k).as_mut_ptr() as *mut S).offset(i as isize);
            *fresh21 = ES(DI(a, i));
            i += 1;
            i;
        }
    } else if -(3 as libc::c_int) as libc::c_longlong == t
        || 3 as libc::c_int as libc::c_longlong == t
    {
        return enumerate_charvec(((*a).k).as_mut_ptr() as *mut C)
    } else if 4 as libc::c_int as libc::c_longlong == t {
        return kerr(b"nyi\0" as *const u8 as *const libc::c_char)
    } else if -(1 as libc::c_int) as libc::c_longlong == t {
        let mut n_0: I = (*a).n;
        let mut x: I = 0;
        let mut p: I = 1 as libc::c_int as I;
        let mut e: K = 0 as *mut k0;
        let mut r: K = 0 as *mut k0;
        let mut s: K = 0 as *mut k0;
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = n_0;
        while i_0 < _i_0 {
            x = *(((*a).k).as_mut_ptr() as *mut I).offset(i_0 as isize);
            p *= x;
            if x < 0 as libc::c_int as libc::c_longlong
                || p < 0 as libc::c_int as libc::c_longlong
            {
                return kerr(b"int\0" as *const u8 as *const libc::c_char);
            }
            i_0 += 1;
            i_0;
        }
        if n_0 == 0 as libc::c_int as libc::c_longlong {
            p = 0 as libc::c_int as I;
        }
        z = newK(0 as libc::c_int as I, p);
        if z.is_null() {
            return 0 as K;
        }
        if p > 0 as libc::c_int as libc::c_longlong {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = p;
            while i_1 < _i_1 {
                e = newK(-(1 as libc::c_int) as I, (*a).n);
                if OOM_CD(0 as libc::c_int as I, e, z, -(1 as libc::c_int) as V) == 0 {
                    return 0 as K;
                }
                let ref mut fresh22 = *((*z).k).as_mut_ptr().offset(i_1 as isize);
                *fresh22 = e;
                i_1 += 1;
                i_1;
            }
            r = *((*z).k).as_mut_ptr().offset(0 as libc::c_int as isize);
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = (*r).n;
            while i_2 < _i_2 {
                *(((*r).k).as_mut_ptr() as *mut I)
                    .offset(i_2 as isize) = 0 as libc::c_int as I;
                i_2 += 1;
                i_2;
            }
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = p - 1 as libc::c_int as libc::c_longlong;
            while i_3 < _i_3 {
                r = *((*z).k).as_mut_ptr().offset(i_3 as isize);
                s = *((*z).k)
                    .as_mut_ptr()
                    .offset((i_3 + 1 as libc::c_int as libc::c_longlong) as isize);
                let mut carry: I = 1 as libc::c_int as I;
                let mut j: I = 0 as libc::c_int as I;
                let mut _j: I = (*s).n;
                while j < _j {
                    x = -(1 as libc::c_int) as libc::c_longlong + (*s).n - j;
                    *(((*s).k).as_mut_ptr() as *mut I)
                        .offset(
                            x as isize,
                        ) = *(((*r).k).as_mut_ptr() as *mut I).offset(x as isize);
                    if carry != 0 {
                        let ref mut fresh23 = *(((*s).k).as_mut_ptr() as *mut I)
                            .offset(x as isize);
                        *fresh23 += 1;
                        *fresh23;
                        carry = 0 as libc::c_int as I;
                    }
                    if *(((*s).k).as_mut_ptr() as *mut I).offset(x as isize)
                        >= *(((*a).k).as_mut_ptr() as *mut I).offset(x as isize)
                    {
                        *(((*s).k).as_mut_ptr() as *mut I)
                            .offset(x as isize) = 0 as libc::c_int as I;
                        carry = 1 as libc::c_int as I;
                    }
                    j += 1;
                    j;
                }
                i_3 += 1;
                i_3;
            }
        }
        return z;
    } else if 1 as libc::c_int as libc::c_longlong == t
        || 2 as libc::c_int as libc::c_longlong == t
    {
        let mut n_1: I = if t == 1 as libc::c_int as libc::c_longlong {
            *(((*a).k).as_mut_ptr() as *mut I)
        } else {
            *(((*a).k).as_mut_ptr() as *mut F) as I
        };
        if n_1 < 0 as libc::c_int as libc::c_longlong {
            return kerr(b"domain\0" as *const u8 as *const libc::c_char);
        }
        z = newK(-(1 as libc::c_int) as I, n_1);
        if z.is_null() {
            return 0 as K;
        }
        let mut i_4: I = 0 as libc::c_int as I;
        let mut _i_4: I = n_1;
        while i_4 < _i_4 {
            *(((*z).k).as_mut_ptr() as *mut I).offset(i_4 as isize) = i_4;
            i_4 += 1;
            i_4;
        }
    } else {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char)
    }
    return z;
}
