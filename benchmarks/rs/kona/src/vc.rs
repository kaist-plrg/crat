use ::libc;
extern "C" {
    fn cd(a: K) -> K;
    fn VA(p: V) -> I;
    fn kerr(s: cS) -> K;
    fn Ki(x: I) -> K;
    fn newK(t: I, n: I) -> K;
    fn OOM_CD(g: I, _: ...) -> I;
    fn promote(a: K) -> K;
    fn SC(a: S, b: S) -> I;
    fn FC(a: F, b: F) -> I;
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
pub unsafe extern "C" fn FC_IF(mut a: I, mut b: F) -> I {
    return FC(
        if 9223372036854775807 as libc::c_longlong == a {
            1 as libc::c_int as libc::c_double / 0.0f64
        } else if -(9223372036854775807 as libc::c_longlong) == a {
            -(1 as libc::c_int as libc::c_double / 0.0f64)
        } else if -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong == a
        {
            0 as libc::c_int as libc::c_double / 0.0f64
        } else {
            a as libc::c_double
        },
        b,
    );
}
pub unsafe extern "C" fn FC_FI(mut a: F, mut b: I) -> I {
    return FC(
        a,
        if 9223372036854775807 as libc::c_longlong == b {
            1 as libc::c_int as libc::c_double / 0.0f64
        } else if -(9223372036854775807 as libc::c_longlong) == b {
            -(1 as libc::c_int as libc::c_double / 0.0f64)
        } else if -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong == b
        {
            0 as libc::c_int as libc::c_double / 0.0f64
        } else {
            b as libc::c_double
        },
    );
}
pub unsafe extern "C" fn dp(
    mut z: *mut K,
    mut f: Option::<unsafe extern "C" fn(K, K) -> K>,
    mut x: K,
    mut y: K,
) -> K {
    x = promote(x);
    y = promote(y);
    if OOM_CD(0 as libc::c_int as I, x, y, *z, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = (**z).n;
    while i < _i {
        let ref mut fresh0 = *((**z).k).as_mut_ptr().offset(i as isize);
        *fresh0 = f
            .unwrap()(
            *((*x).k).as_mut_ptr().offset((i % (*x).n) as isize),
            *((*y).k).as_mut_ptr().offset((i % (*y).n) as isize),
        );
        if (*fresh0).is_null() {
            cd(*z);
            *z = kerr(b"type\0" as *const u8 as *const libc::c_char);
            break;
        } else {
            i += 1;
            i;
        }
    }
    cd(x);
    cd(y);
    return 0 as K;
}
pub unsafe extern "C" fn equals(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    if at <= 0 as libc::c_int as libc::c_longlong
        && bt <= 0 as libc::c_int as libc::c_longlong && an != bn
    {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    let mut AT: I = if at < 0 as libc::c_int as libc::c_longlong { -at } else { at };
    let mut BT: I = if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt };
    if (4 as libc::c_int as libc::c_longlong) < AT
        || (4 as libc::c_int as libc::c_longlong) < BT
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    if at != 0 && bt != 0
        && !(2 as libc::c_int as libc::c_longlong >= AT
            && 2 as libc::c_int as libc::c_longlong >= BT)
        && !(3 as libc::c_int as libc::c_longlong == AT
            && 3 as libc::c_int as libc::c_longlong == BT)
        && !(4 as libc::c_int as libc::c_longlong == AT
            && 4 as libc::c_int as libc::c_longlong == BT)
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut t: I = (if at == 0 || bt == 0 {
        0 as libc::c_int
    } else if (if at < bt { at } else { bt }) < 0 as libc::c_int as libc::c_longlong {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    }) as I;
    let mut zn: I = if at > 0 as libc::c_int as libc::c_longlong { bn } else { an };
    let mut z: K = newK(t, zn);
    if 2 as libc::c_int as libc::c_longlong == AT
        && 2 as libc::c_int as libc::c_longlong == BT
    {
        if an == bn {
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = zn;
            while i < _i {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i as isize,
                    ) = (if FC(
                    *(((*a).k).as_mut_ptr() as *mut F).offset(i as isize),
                    *(((*b).k).as_mut_ptr() as *mut F).offset(i as isize),
                ) != 0
                {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                }) as I;
                i += 1;
                i;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = zn;
            while i_0 < _i_0 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_0 as isize,
                    ) = (if FC(
                    *(((*a).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize),
                    *(((*b).k).as_mut_ptr() as *mut F).offset(i_0 as isize),
                ) != 0
                {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                }) as I;
                i_0 += 1;
                i_0;
            }
        } else {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = zn;
            while i_1 < _i_1 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_1 as isize,
                    ) = (if FC(
                    *(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize),
                    *(((*b).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize),
                ) != 0
                {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                }) as I;
                i_1 += 1;
                i_1;
            }
        }
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = zn;
        while i_2 < _i_2 {
            if *(((*a).k).as_mut_ptr() as *mut F).offset(i_2 as isize)
                != *(((*a).k).as_mut_ptr() as *mut F).offset(i_2 as isize)
                && *(((*b).k).as_mut_ptr() as *mut F).offset(i_2 as isize)
                    != *(((*b).k).as_mut_ptr() as *mut F).offset(i_2 as isize)
            {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(i_2 as isize) = 1 as libc::c_int as I;
            }
            i_2 += 1;
            i_2;
        }
    } else if 2 as libc::c_int as libc::c_longlong == AT
        && 1 as libc::c_int as libc::c_longlong == BT
    {
        if an == bn {
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = zn;
            while i_3 < _i_3 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_3 as isize,
                    ) = (if FC_FI(
                    *(((*a).k).as_mut_ptr() as *mut F).offset(i_3 as isize),
                    *(((*b).k).as_mut_ptr() as *mut I).offset(i_3 as isize),
                ) != 0
                {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                }) as I;
                i_3 += 1;
                i_3;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_4: I = 0 as libc::c_int as I;
            let mut _i_4: I = zn;
            while i_4 < _i_4 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_4 as isize,
                    ) = (if FC_FI(
                    *(((*a).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize),
                    *(((*b).k).as_mut_ptr() as *mut I).offset(i_4 as isize),
                ) != 0
                {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                }) as I;
                i_4 += 1;
                i_4;
            }
        } else {
            let mut i_5: I = 0 as libc::c_int as I;
            let mut _i_5: I = zn;
            while i_5 < _i_5 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_5 as isize,
                    ) = (if FC_FI(
                    *(((*a).k).as_mut_ptr() as *mut F).offset(i_5 as isize),
                    *(((*b).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize),
                ) != 0
                {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                }) as I;
                i_5 += 1;
                i_5;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong == AT
        && 2 as libc::c_int as libc::c_longlong == BT
    {
        if an == bn {
            let mut i_6: I = 0 as libc::c_int as I;
            let mut _i_6: I = zn;
            while i_6 < _i_6 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_6 as isize,
                    ) = (if FC_IF(
                    *(((*a).k).as_mut_ptr() as *mut I).offset(i_6 as isize),
                    *(((*b).k).as_mut_ptr() as *mut F).offset(i_6 as isize),
                ) != 0
                {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                }) as I;
                i_6 += 1;
                i_6;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_7: I = 0 as libc::c_int as I;
            let mut _i_7: I = zn;
            while i_7 < _i_7 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_7 as isize,
                    ) = (if FC_IF(
                    *(((*a).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize),
                    *(((*b).k).as_mut_ptr() as *mut F).offset(i_7 as isize),
                ) != 0
                {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                }) as I;
                i_7 += 1;
                i_7;
            }
        } else {
            let mut i_8: I = 0 as libc::c_int as I;
            let mut _i_8: I = zn;
            while i_8 < _i_8 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_8 as isize,
                    ) = (if FC_IF(
                    *(((*a).k).as_mut_ptr() as *mut I).offset(i_8 as isize),
                    *(((*b).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize),
                ) != 0
                {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                }) as I;
                i_8 += 1;
                i_8;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong == AT
        && 1 as libc::c_int as libc::c_longlong == BT
    {
        if an == bn {
            let mut i_9: I = 0 as libc::c_int as I;
            let mut _i_9: I = zn;
            while i_9 < _i_9 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_9 as isize,
                    ) = (*(((*a).k).as_mut_ptr() as *mut I).offset(i_9 as isize)
                    == *(((*b).k).as_mut_ptr() as *mut I).offset(i_9 as isize))
                    as libc::c_int as I;
                i_9 += 1;
                i_9;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_10: I = 0 as libc::c_int as I;
            let mut _i_10: I = zn;
            while i_10 < _i_10 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_10 as isize,
                    ) = (*(((*a).k).as_mut_ptr() as *mut I)
                    .offset(0 as libc::c_int as isize)
                    == *(((*b).k).as_mut_ptr() as *mut I).offset(i_10 as isize))
                    as libc::c_int as I;
                i_10 += 1;
                i_10;
            }
        } else {
            let mut i_11: I = 0 as libc::c_int as I;
            let mut _i_11: I = zn;
            while i_11 < _i_11 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_11 as isize,
                    ) = (*(((*a).k).as_mut_ptr() as *mut I).offset(i_11 as isize)
                    == *(((*b).k).as_mut_ptr() as *mut I)
                        .offset(0 as libc::c_int as isize)) as libc::c_int as I;
                i_11 += 1;
                i_11;
            }
        }
    } else if 3 as libc::c_int as libc::c_longlong == AT
        && 3 as libc::c_int as libc::c_longlong == BT
    {
        if an == bn {
            let mut i_12: I = 0 as libc::c_int as I;
            let mut _i_12: I = zn;
            while i_12 < _i_12 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_12 as isize,
                    ) = (*(((*a).k).as_mut_ptr() as *mut C).offset(i_12 as isize)
                    as libc::c_int
                    == *(((*b).k).as_mut_ptr() as *mut C).offset(i_12 as isize)
                        as libc::c_int) as libc::c_int as I;
                i_12 += 1;
                i_12;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_13: I = 0 as libc::c_int as I;
            let mut _i_13: I = zn;
            while i_13 < _i_13 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_13 as isize,
                    ) = (*(((*a).k).as_mut_ptr() as *mut C)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    == *(((*b).k).as_mut_ptr() as *mut C).offset(i_13 as isize)
                        as libc::c_int) as libc::c_int as I;
                i_13 += 1;
                i_13;
            }
        } else {
            let mut i_14: I = 0 as libc::c_int as I;
            let mut _i_14: I = zn;
            while i_14 < _i_14 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_14 as isize,
                    ) = (*(((*a).k).as_mut_ptr() as *mut C).offset(i_14 as isize)
                    as libc::c_int
                    == *(((*b).k).as_mut_ptr() as *mut C)
                        .offset(0 as libc::c_int as isize) as libc::c_int) as libc::c_int
                    as I;
                i_14 += 1;
                i_14;
            }
        }
    } else if 4 as libc::c_int as libc::c_longlong == AT
        && 4 as libc::c_int as libc::c_longlong == BT
    {
        if an == bn {
            let mut i_15: I = 0 as libc::c_int as I;
            let mut _i_15: I = zn;
            while i_15 < _i_15 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_15 as isize,
                    ) = (*(((*a).k).as_mut_ptr() as *mut S).offset(i_15 as isize)
                    == *(((*b).k).as_mut_ptr() as *mut S).offset(i_15 as isize))
                    as libc::c_int as I;
                i_15 += 1;
                i_15;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_16: I = 0 as libc::c_int as I;
            let mut _i_16: I = zn;
            while i_16 < _i_16 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_16 as isize,
                    ) = (*(((*a).k).as_mut_ptr() as *mut S)
                    .offset(0 as libc::c_int as isize)
                    == *(((*b).k).as_mut_ptr() as *mut S).offset(i_16 as isize))
                    as libc::c_int as I;
                i_16 += 1;
                i_16;
            }
        } else {
            let mut i_17: I = 0 as libc::c_int as I;
            let mut _i_17: I = zn;
            while i_17 < _i_17 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_17 as isize,
                    ) = (*(((*a).k).as_mut_ptr() as *mut S).offset(i_17 as isize)
                    == *(((*b).k).as_mut_ptr() as *mut S)
                        .offset(0 as libc::c_int as isize)) as libc::c_int as I;
                i_17 += 1;
                i_17;
            }
        }
    } else if 0 as libc::c_int as libc::c_longlong == at
        || 0 as libc::c_int as libc::c_longlong == bt
    {
        dp(&mut z, Some(equals as unsafe extern "C" fn(K, K) -> K), a, b);
    }
    return z;
}
pub unsafe extern "C" fn matchI(mut a: K, mut b: K) -> I {
    if a.is_null() || b.is_null() {
        return 0 as libc::c_int as I;
    }
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    let mut AT: I = if at < 0 as libc::c_int as libc::c_longlong { -at } else { at };
    let mut BT: I = if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt };
    let mut c: *mut K = 0 as *mut K;
    let mut d: *mut K = 0 as *mut K;
    if an != bn || at != bt {
        return 0 as libc::c_int as I;
    }
    if 4 as libc::c_int as libc::c_longlong == AT {
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
    }
    if 3 as libc::c_int as libc::c_longlong == AT {
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
    }
    if 2 as libc::c_int as libc::c_longlong == AT
        && 2 as libc::c_int as libc::c_longlong == BT
    {
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
    }
    if 1 as libc::c_int as libc::c_longlong == AT
        && 1 as libc::c_int as libc::c_longlong == BT
    {
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
    }
    if 0 as libc::c_int as libc::c_longlong == AT
        || 5 as libc::c_int as libc::c_longlong == AT
    {
        let mut i_3: I = 0 as libc::c_int as I;
        let mut _i_3: I = an;
        while i_3 < _i_3 {
            if matchI(
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
    if 7 as libc::c_int as libc::c_longlong == AT {
        if (*a).n != (*b).n {
            return 0 as libc::c_int as I;
        }
        match (*a).n {
            1 => {
                an = (*(*(((*a).k).as_mut_ptr() as *mut V)
                    .offset(CODE as libc::c_int as isize) as K))
                    .n - 1 as libc::c_int as libc::c_longlong;
                bn = (*(*(((*b).k).as_mut_ptr() as *mut V)
                    .offset(CODE as libc::c_int as isize) as K))
                    .n - 1 as libc::c_int as libc::c_longlong;
                if an != bn {
                    return 0 as libc::c_int as I;
                }
                let mut i_4: I = 0 as libc::c_int as I;
                let mut _i_4: I = an;
                while i_4 < _i_4 {
                    c = *(((*(*(((*a).k).as_mut_ptr() as *mut V)
                        .offset(CODE as libc::c_int as isize) as K))
                        .k)
                        .as_mut_ptr() as *mut S as *mut V)
                        .offset(i_4 as isize) as *mut K;
                    d = *(((*(*(((*b).k).as_mut_ptr() as *mut V)
                        .offset(CODE as libc::c_int as isize) as K))
                        .k)
                        .as_mut_ptr() as *mut S as *mut V)
                        .offset(i_4 as isize) as *mut K;
                    if VA(c as V) != 0 || VA(d as V) != 0 {
                        if c != d {
                            return 0 as libc::c_int as I;
                        }
                    } else if matchI(*c, *d) == 0 {
                        return 0 as libc::c_int as I
                    }
                    i_4 += 1;
                    i_4;
                }
            }
            3 => {
                if *(((*a).k).as_mut_ptr() as *mut V)
                    .offset(CONTeXT as libc::c_int as isize)
                    != *(((*b).k).as_mut_ptr() as *mut V)
                        .offset(CONTeXT as libc::c_int as isize)
                {
                    return 0 as libc::c_int as I;
                }
                return matchI(
                    *(((*a).k).as_mut_ptr() as *mut V)
                        .offset(CODE as libc::c_int as isize) as K,
                    *(((*b).k).as_mut_ptr() as *mut V)
                        .offset(CODE as libc::c_int as isize) as K,
                );
            }
            2 | _ => {}
        }
    }
    return 1 as libc::c_int as I;
}
#[export_name = "match"]
pub unsafe extern "C" fn match_0(mut a: K, mut b: K) -> K {
    return Ki(matchI(a, b));
}
unsafe extern "C" fn lessmore(mut a: K, mut b: K, mut x: I) -> K {
    if x == 0 && 0 as libc::c_int as libc::c_longlong != (*b).t {
        let mut c: K = a;
        a = b;
        b = c;
    }
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    if at <= 0 as libc::c_int as libc::c_longlong
        && bt <= 0 as libc::c_int as libc::c_longlong && an != bn
    {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    let mut AT: I = if at < 0 as libc::c_int as libc::c_longlong { -at } else { at };
    let mut BT: I = if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt };
    if (4 as libc::c_int as libc::c_longlong) < AT
        || (4 as libc::c_int as libc::c_longlong) < BT
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    if at != 0 && bt != 0
        && !(2 as libc::c_int as libc::c_longlong >= AT
            && 2 as libc::c_int as libc::c_longlong >= BT)
        && !(3 as libc::c_int as libc::c_longlong == AT
            && 3 as libc::c_int as libc::c_longlong == BT)
        && !(4 as libc::c_int as libc::c_longlong == AT
            && 4 as libc::c_int as libc::c_longlong == BT)
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut t: I = (if at == 0 || bt == 0 {
        0 as libc::c_int
    } else if (if at < bt { at } else { bt }) < 0 as libc::c_int as libc::c_longlong {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    }) as I;
    let mut zn: I = if at > 0 as libc::c_int as libc::c_longlong { bn } else { an };
    let mut z: K = newK(t, zn);
    if z.is_null() {
        return 0 as K;
    }
    let mut h: *mut I = ((*z).k).as_mut_ptr() as *mut I;
    if 0 as libc::c_int as libc::c_longlong == at
        || 0 as libc::c_int as libc::c_longlong == bt
    {
        a = promote(a);
        b = promote(b);
        if OOM_CD(0 as libc::c_int as I, a, b, z, -(1 as libc::c_int) as V) == 0 {
            return 0 as K;
        }
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = zn;
        while i < _i {
            let ref mut fresh1 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh1 = lessmore(
                *((*a).k).as_mut_ptr().offset((i % an) as isize),
                *((*b).k).as_mut_ptr().offset((i % (*b).n) as isize),
                x,
            );
            if (*fresh1).is_null() {
                cd(z);
                z = kerr(b"wsfull\0" as *const u8 as *const libc::c_char);
                break;
            } else {
                i += 1;
                i;
            }
        }
        cd(a);
        cd(b);
    } else if 2 as libc::c_int as libc::c_longlong == AT
        && 2 as libc::c_int as libc::c_longlong == BT
    {
        if an == bn {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = zn;
            while i_0 < _i_0 {
                *h
                    .offset(
                        i_0 as isize,
                    ) = (FC(
                    *(((*a).k).as_mut_ptr() as *mut F).offset(i_0 as isize),
                    *(((*b).k).as_mut_ptr() as *mut F).offset(i_0 as isize),
                ) > 0 as libc::c_int as libc::c_longlong) as libc::c_int as I;
                i_0 += 1;
                i_0;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = zn;
            while i_1 < _i_1 {
                *h
                    .offset(
                        i_1 as isize,
                    ) = (FC(
                    *(((*a).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize),
                    *(((*b).k).as_mut_ptr() as *mut F).offset(i_1 as isize),
                ) > 0 as libc::c_int as libc::c_longlong) as libc::c_int as I;
                i_1 += 1;
                i_1;
            }
        } else {
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = zn;
            while i_2 < _i_2 {
                *h
                    .offset(
                        i_2 as isize,
                    ) = (FC(
                    *(((*a).k).as_mut_ptr() as *mut F).offset(i_2 as isize),
                    *(((*b).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize),
                ) > 0 as libc::c_int as libc::c_longlong) as libc::c_int as I;
                i_2 += 1;
                i_2;
            }
        }
    } else if 2 as libc::c_int as libc::c_longlong == AT
        && 1 as libc::c_int as libc::c_longlong == BT
    {
        if an == bn {
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = zn;
            while i_3 < _i_3 {
                *h
                    .offset(
                        i_3 as isize,
                    ) = (FC_FI(
                    *(((*a).k).as_mut_ptr() as *mut F).offset(i_3 as isize),
                    *(((*b).k).as_mut_ptr() as *mut I).offset(i_3 as isize),
                ) > 0 as libc::c_int as libc::c_longlong) as libc::c_int as I;
                i_3 += 1;
                i_3;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_4: I = 0 as libc::c_int as I;
            let mut _i_4: I = zn;
            while i_4 < _i_4 {
                *h
                    .offset(
                        i_4 as isize,
                    ) = (FC_FI(
                    *(((*a).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize),
                    *(((*b).k).as_mut_ptr() as *mut I).offset(i_4 as isize),
                ) > 0 as libc::c_int as libc::c_longlong) as libc::c_int as I;
                i_4 += 1;
                i_4;
            }
        } else {
            let mut i_5: I = 0 as libc::c_int as I;
            let mut _i_5: I = zn;
            while i_5 < _i_5 {
                *h
                    .offset(
                        i_5 as isize,
                    ) = (FC_FI(
                    *(((*a).k).as_mut_ptr() as *mut F).offset(i_5 as isize),
                    *(((*b).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize),
                ) > 0 as libc::c_int as libc::c_longlong) as libc::c_int as I;
                i_5 += 1;
                i_5;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong == AT
        && 2 as libc::c_int as libc::c_longlong == BT
    {
        if an == bn {
            let mut i_6: I = 0 as libc::c_int as I;
            let mut _i_6: I = zn;
            while i_6 < _i_6 {
                *h
                    .offset(
                        i_6 as isize,
                    ) = (FC_IF(
                    *(((*a).k).as_mut_ptr() as *mut I).offset(i_6 as isize),
                    *(((*b).k).as_mut_ptr() as *mut F).offset(i_6 as isize),
                ) > 0 as libc::c_int as libc::c_longlong) as libc::c_int as I;
                i_6 += 1;
                i_6;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_7: I = 0 as libc::c_int as I;
            let mut _i_7: I = zn;
            while i_7 < _i_7 {
                *h
                    .offset(
                        i_7 as isize,
                    ) = (FC_IF(
                    *(((*a).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize),
                    *(((*b).k).as_mut_ptr() as *mut F).offset(i_7 as isize),
                ) > 0 as libc::c_int as libc::c_longlong) as libc::c_int as I;
                i_7 += 1;
                i_7;
            }
        } else {
            let mut i_8: I = 0 as libc::c_int as I;
            let mut _i_8: I = zn;
            while i_8 < _i_8 {
                *h
                    .offset(
                        i_8 as isize,
                    ) = (FC_IF(
                    *(((*a).k).as_mut_ptr() as *mut I).offset(i_8 as isize),
                    *(((*b).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize),
                ) > 0 as libc::c_int as libc::c_longlong) as libc::c_int as I;
                i_8 += 1;
                i_8;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong == AT
        && 1 as libc::c_int as libc::c_longlong == BT
    {
        if an == bn {
            let mut i_9: I = 0 as libc::c_int as I;
            let mut _i_9: I = zn;
            while i_9 < _i_9 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_9 as isize,
                    ) = (*(((*a).k).as_mut_ptr() as *mut I).offset(i_9 as isize)
                    > *(((*b).k).as_mut_ptr() as *mut I).offset(i_9 as isize))
                    as libc::c_int as I;
                i_9 += 1;
                i_9;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_10: I = 0 as libc::c_int as I;
            let mut _i_10: I = zn;
            while i_10 < _i_10 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_10 as isize,
                    ) = (*(((*a).k).as_mut_ptr() as *mut I)
                    .offset(0 as libc::c_int as isize)
                    > *(((*b).k).as_mut_ptr() as *mut I).offset(i_10 as isize))
                    as libc::c_int as I;
                i_10 += 1;
                i_10;
            }
        } else {
            let mut i_11: I = 0 as libc::c_int as I;
            let mut _i_11: I = zn;
            while i_11 < _i_11 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_11 as isize,
                    ) = (*(((*a).k).as_mut_ptr() as *mut I).offset(i_11 as isize)
                    > *(((*b).k).as_mut_ptr() as *mut I)
                        .offset(0 as libc::c_int as isize)) as libc::c_int as I;
                i_11 += 1;
                i_11;
            }
        }
    } else if 3 as libc::c_int as libc::c_longlong == AT
        && 3 as libc::c_int as libc::c_longlong == BT
    {
        if an == bn {
            let mut i_12: I = 0 as libc::c_int as I;
            let mut _i_12: I = zn;
            while i_12 < _i_12 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_12 as isize,
                    ) = (*(((*a).k).as_mut_ptr() as *mut C).offset(i_12 as isize)
                    as libc::c_int
                    > *(((*b).k).as_mut_ptr() as *mut C).offset(i_12 as isize)
                        as libc::c_int) as libc::c_int as I;
                i_12 += 1;
                i_12;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_13: I = 0 as libc::c_int as I;
            let mut _i_13: I = zn;
            while i_13 < _i_13 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_13 as isize,
                    ) = (*(((*a).k).as_mut_ptr() as *mut C)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    > *(((*b).k).as_mut_ptr() as *mut C).offset(i_13 as isize)
                        as libc::c_int) as libc::c_int as I;
                i_13 += 1;
                i_13;
            }
        } else {
            let mut i_14: I = 0 as libc::c_int as I;
            let mut _i_14: I = zn;
            while i_14 < _i_14 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_14 as isize,
                    ) = (*(((*a).k).as_mut_ptr() as *mut C).offset(i_14 as isize)
                    as libc::c_int
                    > *(((*b).k).as_mut_ptr() as *mut C)
                        .offset(0 as libc::c_int as isize) as libc::c_int) as libc::c_int
                    as I;
                i_14 += 1;
                i_14;
            }
        }
    } else if 4 as libc::c_int as libc::c_longlong == AT
        && 4 as libc::c_int as libc::c_longlong == BT
    {
        if an == bn {
            let mut i_15: I = 0 as libc::c_int as I;
            let mut _i_15: I = zn;
            while i_15 < _i_15 {
                *h
                    .offset(
                        i_15 as isize,
                    ) = (SC(
                    *(((*a).k).as_mut_ptr() as *mut S).offset(i_15 as isize),
                    *(((*b).k).as_mut_ptr() as *mut S).offset(i_15 as isize),
                ) > 0 as libc::c_int as libc::c_longlong) as libc::c_int as I;
                i_15 += 1;
                i_15;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_16: I = 0 as libc::c_int as I;
            let mut _i_16: I = zn;
            while i_16 < _i_16 {
                *h
                    .offset(
                        i_16 as isize,
                    ) = (SC(
                    *(((*a).k).as_mut_ptr() as *mut S).offset(0 as libc::c_int as isize),
                    *(((*b).k).as_mut_ptr() as *mut S).offset(i_16 as isize),
                ) > 0 as libc::c_int as libc::c_longlong) as libc::c_int as I;
                i_16 += 1;
                i_16;
            }
        } else {
            let mut i_17: I = 0 as libc::c_int as I;
            let mut _i_17: I = zn;
            while i_17 < _i_17 {
                *h
                    .offset(
                        i_17 as isize,
                    ) = (SC(
                    *(((*a).k).as_mut_ptr() as *mut S).offset(i_17 as isize),
                    *(((*b).k).as_mut_ptr() as *mut S).offset(0 as libc::c_int as isize),
                ) > 0 as libc::c_int as libc::c_longlong) as libc::c_int as I;
                i_17 += 1;
                i_17;
            }
        }
    }
    return z;
}
pub unsafe extern "C" fn less(mut a: K, mut b: K) -> K {
    return lessmore(a, b, 0 as libc::c_int as I);
}
pub unsafe extern "C" fn more(mut a: K, mut b: K) -> K {
    return lessmore(a, b, 1 as libc::c_int as I);
}
