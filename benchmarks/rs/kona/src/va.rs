use ::libc;
extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn cd(a: K) -> K;
    fn FF(f: F) -> F;
    fn kerr(s: cS) -> K;
    fn FC(a: F, b: F) -> I;
    fn Kf(x: F) -> K;
    fn Ki(x: I) -> K;
    fn ci(a: K) -> K;
    fn newK(t: I, n: I) -> K;
    fn dp(z: *mut K, f: Option::<unsafe extern "C" fn(K, K) -> K>, x: K, y: K) -> K;
    fn overDyad(a: K, p: *mut V, b: K) -> K;
    static mut errmsg: [C; 256];
}
pub type V = *mut libc::c_void;
pub type I = libc::c_longlong;
pub type F = libc::c_double;
pub type C = libc::c_char;
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
pub unsafe extern "C" fn power(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    let mut type_0: I = if (if at < 0 as libc::c_int as libc::c_longlong {
        -at
    } else {
        at
    }) > (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if at < 0 as libc::c_int as libc::c_longlong { -at } else { at }
    } else if bt < 0 as libc::c_int as libc::c_longlong {
        -bt
    } else {
        bt
    };
    if at <= 0 as libc::c_int as libc::c_longlong
        && bt <= 0 as libc::c_int as libc::c_longlong && an != bn
    {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    if type_0 > 2 as libc::c_int as libc::c_longlong {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut zt: I = type_0;
    if (if at < bt { at } else { bt }) < 1 as libc::c_int as libc::c_longlong {
        zt = -zt;
    }
    if at == 0 || bt == 0 {
        zt = 0 as libc::c_int as I;
    }
    if 1 as libc::c_int as libc::c_longlong == zt * zt {
        zt *= 2 as libc::c_int as libc::c_longlong;
    }
    let mut zn: I = if at > 0 as libc::c_int as libc::c_longlong { bn } else { an };
    let mut z: K = newK(zt, zn);
    if z.is_null() {
        return 0 as K;
    }
    let mut x: F = 0.;
    let mut y: F = 0.;
    if 2 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 2 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = zn;
            while i < _i {
                x = *(((*a).k).as_mut_ptr() as *mut F).offset(i as isize);
                y = *(((*b).k).as_mut_ptr() as *mut F).offset(i as isize);
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i as isize,
                    ) = if 0 as libc::c_int as libc::c_double == y {
                    1 as libc::c_int as libc::c_double
                } else if 0 as libc::c_int as libc::c_double == x {
                    0 as libc::c_int as libc::c_double
                } else {
                    pow(x, y)
                };
                i += 1;
                i;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            x = *(((*a).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize);
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = zn;
            while i_0 < _i_0 {
                y = *(((*b).k).as_mut_ptr() as *mut F).offset(i_0 as isize);
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_0 as isize,
                    ) = if 0 as libc::c_int as libc::c_double == y {
                    1 as libc::c_int as libc::c_double
                } else if 0 as libc::c_int as libc::c_double == x {
                    0 as libc::c_int as libc::c_double
                } else {
                    pow(x, y)
                };
                i_0 += 1;
                i_0;
            }
        } else {
            y = *(((*b).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize);
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = zn;
            while i_1 < _i_1 {
                x = *(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize);
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_1 as isize,
                    ) = if 0 as libc::c_int as libc::c_double == y {
                    1 as libc::c_int as libc::c_double
                } else if 0 as libc::c_int as libc::c_double == x {
                    0 as libc::c_int as libc::c_double
                } else {
                    pow(x, y)
                };
                i_1 += 1;
                i_1;
            }
        }
    } else if 2 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 1 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = zn;
            while i_2 < _i_2 {
                x = *(((*a).k).as_mut_ptr() as *mut F).offset(i_2 as isize);
                y = *(((*b).k).as_mut_ptr() as *mut I).offset(i_2 as isize) as F;
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_2 as isize,
                    ) = if 0 as libc::c_int as libc::c_double == y {
                    1 as libc::c_int as libc::c_double
                } else if 0 as libc::c_int as libc::c_double == x {
                    0 as libc::c_int as libc::c_double
                } else {
                    pow(x, y)
                };
                i_2 += 1;
                i_2;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            x = *(((*a).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize);
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = zn;
            while i_3 < _i_3 {
                y = *(((*b).k).as_mut_ptr() as *mut I).offset(i_3 as isize) as F;
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_3 as isize,
                    ) = if 0 as libc::c_int as libc::c_double == y {
                    1 as libc::c_int as libc::c_double
                } else if 0 as libc::c_int as libc::c_double == x {
                    0 as libc::c_int as libc::c_double
                } else {
                    pow(x, y)
                };
                i_3 += 1;
                i_3;
            }
        } else {
            y = *(((*b).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize)
                as F;
            let mut i_4: I = 0 as libc::c_int as I;
            let mut _i_4: I = zn;
            while i_4 < _i_4 {
                x = *(((*a).k).as_mut_ptr() as *mut F).offset(i_4 as isize);
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_4 as isize,
                    ) = if 0 as libc::c_int as libc::c_double == y {
                    1 as libc::c_int as libc::c_double
                } else if 0 as libc::c_int as libc::c_double == x {
                    0 as libc::c_int as libc::c_double
                } else {
                    pow(x, y)
                };
                i_4 += 1;
                i_4;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 2 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_5: I = 0 as libc::c_int as I;
            let mut _i_5: I = zn;
            while i_5 < _i_5 {
                x = *(((*a).k).as_mut_ptr() as *mut I).offset(i_5 as isize) as F;
                y = *(((*b).k).as_mut_ptr() as *mut F).offset(i_5 as isize);
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_5 as isize,
                    ) = if 0 as libc::c_int as libc::c_double == y {
                    1 as libc::c_int as libc::c_double
                } else if 0 as libc::c_int as libc::c_double == x {
                    0 as libc::c_int as libc::c_double
                } else {
                    pow(x, y)
                };
                i_5 += 1;
                i_5;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            x = *(((*a).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize)
                as F;
            let mut i_6: I = 0 as libc::c_int as I;
            let mut _i_6: I = zn;
            while i_6 < _i_6 {
                y = *(((*b).k).as_mut_ptr() as *mut F).offset(i_6 as isize);
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_6 as isize,
                    ) = if 0 as libc::c_int as libc::c_double == y {
                    1 as libc::c_int as libc::c_double
                } else if 0 as libc::c_int as libc::c_double == x {
                    0 as libc::c_int as libc::c_double
                } else {
                    pow(x, y)
                };
                i_6 += 1;
                i_6;
            }
        } else {
            y = *(((*b).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize);
            let mut i_7: I = 0 as libc::c_int as I;
            let mut _i_7: I = zn;
            while i_7 < _i_7 {
                x = *(((*a).k).as_mut_ptr() as *mut I).offset(i_7 as isize) as F;
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_7 as isize,
                    ) = if 0 as libc::c_int as libc::c_double == y {
                    1 as libc::c_int as libc::c_double
                } else if 0 as libc::c_int as libc::c_double == x {
                    0 as libc::c_int as libc::c_double
                } else {
                    pow(x, y)
                };
                i_7 += 1;
                i_7;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 1 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_8: I = 0 as libc::c_int as I;
            let mut _i_8: I = zn;
            while i_8 < _i_8 {
                x = *(((*a).k).as_mut_ptr() as *mut I).offset(i_8 as isize) as F;
                y = *(((*b).k).as_mut_ptr() as *mut I).offset(i_8 as isize) as F;
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_8 as isize,
                    ) = if 0 as libc::c_int as libc::c_double == y {
                    1 as libc::c_int as libc::c_double
                } else if 0 as libc::c_int as libc::c_double == x {
                    0 as libc::c_int as libc::c_double
                } else {
                    pow(x, y)
                };
                i_8 += 1;
                i_8;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            x = *(((*a).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize)
                as F;
            let mut i_9: I = 0 as libc::c_int as I;
            let mut _i_9: I = zn;
            while i_9 < _i_9 {
                y = *(((*b).k).as_mut_ptr() as *mut I).offset(i_9 as isize) as F;
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_9 as isize,
                    ) = if 0 as libc::c_int as libc::c_double == y {
                    1 as libc::c_int as libc::c_double
                } else if 0 as libc::c_int as libc::c_double == x {
                    0 as libc::c_int as libc::c_double
                } else {
                    pow(x, y)
                };
                i_9 += 1;
                i_9;
            }
        } else {
            y = *(((*b).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize)
                as F;
            let mut i_10: I = 0 as libc::c_int as I;
            let mut _i_10: I = zn;
            while i_10 < _i_10 {
                x = *(((*a).k).as_mut_ptr() as *mut I).offset(i_10 as isize) as F;
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_10 as isize,
                    ) = if 0 as libc::c_int as libc::c_double == y {
                    1 as libc::c_int as libc::c_double
                } else if 0 as libc::c_int as libc::c_double == x {
                    0 as libc::c_int as libc::c_double
                } else {
                    pow(x, y)
                };
                i_10 += 1;
                i_10;
            }
        }
    } else if 0 as libc::c_int as libc::c_longlong == at
        || 0 as libc::c_int as libc::c_longlong == bt
    {
        dp(&mut z, Some(power as unsafe extern "C" fn(K, K) -> K), a, b);
    }
    return z;
}
pub unsafe extern "C" fn plus(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    let mut type_0: I = if (if at < 0 as libc::c_int as libc::c_longlong {
        -at
    } else {
        at
    }) > (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if at < 0 as libc::c_int as libc::c_longlong { -at } else { at }
    } else if bt < 0 as libc::c_int as libc::c_longlong {
        -bt
    } else {
        bt
    };
    if at <= 0 as libc::c_int as libc::c_longlong
        && bt <= 0 as libc::c_int as libc::c_longlong && an != bn
    {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    if type_0 > 2 as libc::c_int as libc::c_longlong {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut zt: I = type_0;
    if (if at < bt { at } else { bt }) < 1 as libc::c_int as libc::c_longlong {
        zt = -zt;
    }
    if at == 0 || bt == 0 {
        zt = 0 as libc::c_int as I;
    }
    let mut zn: I = if at > 0 as libc::c_int as libc::c_longlong { bn } else { an };
    let mut z: K = newK(zt, zn);
    if z.is_null() {
        return 0 as K;
    }
    if 2 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 2 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = zn;
            while i < _i {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F).offset(i as isize)
                    + *(((*b).k).as_mut_ptr() as *mut F).offset(i as isize);
                i += 1;
                i;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = zn;
            while i_0 < _i_0 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_0 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F)
                    .offset(0 as libc::c_int as isize)
                    + *(((*b).k).as_mut_ptr() as *mut F).offset(i_0 as isize);
                i_0 += 1;
                i_0;
            }
        } else {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = zn;
            while i_1 < _i_1 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_1 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize)
                    + *(((*b).k).as_mut_ptr() as *mut F)
                        .offset(0 as libc::c_int as isize);
                i_1 += 1;
                i_1;
            }
        }
    } else if 2 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 1 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = zn;
            while i_2 < _i_2 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_2 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F).offset(i_2 as isize)
                    + *(((*b).k).as_mut_ptr() as *mut I).offset(i_2 as isize)
                        as libc::c_double;
                i_2 += 1;
                i_2;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = zn;
            while i_3 < _i_3 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_3 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F)
                    .offset(0 as libc::c_int as isize)
                    + *(((*b).k).as_mut_ptr() as *mut I).offset(i_3 as isize)
                        as libc::c_double;
                i_3 += 1;
                i_3;
            }
        } else {
            let mut i_4: I = 0 as libc::c_int as I;
            let mut _i_4: I = zn;
            while i_4 < _i_4 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_4 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F).offset(i_4 as isize)
                    + *(((*b).k).as_mut_ptr() as *mut I)
                        .offset(0 as libc::c_int as isize) as libc::c_double;
                i_4 += 1;
                i_4;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 2 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_5: I = 0 as libc::c_int as I;
            let mut _i_5: I = zn;
            while i_5 < _i_5 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_5 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I).offset(i_5 as isize)
                    as libc::c_double
                    + *(((*b).k).as_mut_ptr() as *mut F).offset(i_5 as isize);
                i_5 += 1;
                i_5;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_6: I = 0 as libc::c_int as I;
            let mut _i_6: I = zn;
            while i_6 < _i_6 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_6 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I)
                    .offset(0 as libc::c_int as isize) as libc::c_double
                    + *(((*b).k).as_mut_ptr() as *mut F).offset(i_6 as isize);
                i_6 += 1;
                i_6;
            }
        } else {
            let mut i_7: I = 0 as libc::c_int as I;
            let mut _i_7: I = zn;
            while i_7 < _i_7 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_7 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I).offset(i_7 as isize)
                    as libc::c_double
                    + *(((*b).k).as_mut_ptr() as *mut F)
                        .offset(0 as libc::c_int as isize);
                i_7 += 1;
                i_7;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 1 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_8: I = 0 as libc::c_int as I;
            let mut _i_8: I = zn;
            while i_8 < _i_8 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_8 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I).offset(i_8 as isize)
                    + *(((*b).k).as_mut_ptr() as *mut I).offset(i_8 as isize);
                i_8 += 1;
                i_8;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_9: I = 0 as libc::c_int as I;
            let mut _i_9: I = zn;
            while i_9 < _i_9 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_9 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I)
                    .offset(0 as libc::c_int as isize)
                    + *(((*b).k).as_mut_ptr() as *mut I).offset(i_9 as isize);
                i_9 += 1;
                i_9;
            }
        } else {
            let mut i_10: I = 0 as libc::c_int as I;
            let mut _i_10: I = zn;
            while i_10 < _i_10 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_10 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I).offset(i_10 as isize)
                    + *(((*b).k).as_mut_ptr() as *mut I)
                        .offset(0 as libc::c_int as isize);
                i_10 += 1;
                i_10;
            }
        }
    } else if 0 as libc::c_int as libc::c_longlong == at
        || 0 as libc::c_int as libc::c_longlong == bt
    {
        dp(&mut z, Some(plus as unsafe extern "C" fn(K, K) -> K), a, b);
    }
    return z;
}
pub unsafe extern "C" fn times(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    let mut type_0: I = if (if at < 0 as libc::c_int as libc::c_longlong {
        -at
    } else {
        at
    }) > (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if at < 0 as libc::c_int as libc::c_longlong { -at } else { at }
    } else if bt < 0 as libc::c_int as libc::c_longlong {
        -bt
    } else {
        bt
    };
    if at <= 0 as libc::c_int as libc::c_longlong
        && bt <= 0 as libc::c_int as libc::c_longlong && an != bn
    {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    if type_0 > 2 as libc::c_int as libc::c_longlong {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut zt: I = type_0;
    if (if at < bt { at } else { bt }) < 1 as libc::c_int as libc::c_longlong {
        zt = -zt;
    }
    if at == 0 || bt == 0 {
        zt = 0 as libc::c_int as I;
    }
    let mut zn: I = if at > 0 as libc::c_int as libc::c_longlong { bn } else { an };
    let mut z: K = newK(zt, zn);
    if z.is_null() {
        return 0 as K;
    }
    if 2 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 2 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = zn;
            while i < _i {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F).offset(i as isize)
                    * *(((*b).k).as_mut_ptr() as *mut F).offset(i as isize);
                i += 1;
                i;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = zn;
            while i_0 < _i_0 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_0 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F)
                    .offset(0 as libc::c_int as isize)
                    * *(((*b).k).as_mut_ptr() as *mut F).offset(i_0 as isize);
                i_0 += 1;
                i_0;
            }
        } else {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = zn;
            while i_1 < _i_1 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_1 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize)
                    * *(((*b).k).as_mut_ptr() as *mut F)
                        .offset(0 as libc::c_int as isize);
                i_1 += 1;
                i_1;
            }
        }
    } else if 2 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 1 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = zn;
            while i_2 < _i_2 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_2 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F).offset(i_2 as isize)
                    * *(((*b).k).as_mut_ptr() as *mut I).offset(i_2 as isize)
                        as libc::c_double;
                i_2 += 1;
                i_2;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = zn;
            while i_3 < _i_3 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_3 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F)
                    .offset(0 as libc::c_int as isize)
                    * *(((*b).k).as_mut_ptr() as *mut I).offset(i_3 as isize)
                        as libc::c_double;
                i_3 += 1;
                i_3;
            }
        } else {
            let mut i_4: I = 0 as libc::c_int as I;
            let mut _i_4: I = zn;
            while i_4 < _i_4 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_4 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F).offset(i_4 as isize)
                    * *(((*b).k).as_mut_ptr() as *mut I)
                        .offset(0 as libc::c_int as isize) as libc::c_double;
                i_4 += 1;
                i_4;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 2 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_5: I = 0 as libc::c_int as I;
            let mut _i_5: I = zn;
            while i_5 < _i_5 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_5 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I).offset(i_5 as isize)
                    as libc::c_double
                    * *(((*b).k).as_mut_ptr() as *mut F).offset(i_5 as isize);
                i_5 += 1;
                i_5;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_6: I = 0 as libc::c_int as I;
            let mut _i_6: I = zn;
            while i_6 < _i_6 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_6 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I)
                    .offset(0 as libc::c_int as isize) as libc::c_double
                    * *(((*b).k).as_mut_ptr() as *mut F).offset(i_6 as isize);
                i_6 += 1;
                i_6;
            }
        } else {
            let mut i_7: I = 0 as libc::c_int as I;
            let mut _i_7: I = zn;
            while i_7 < _i_7 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_7 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I).offset(i_7 as isize)
                    as libc::c_double
                    * *(((*b).k).as_mut_ptr() as *mut F)
                        .offset(0 as libc::c_int as isize);
                i_7 += 1;
                i_7;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 1 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_8: I = 0 as libc::c_int as I;
            let mut _i_8: I = zn;
            while i_8 < _i_8 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_8 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I).offset(i_8 as isize)
                    * *(((*b).k).as_mut_ptr() as *mut I).offset(i_8 as isize);
                i_8 += 1;
                i_8;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_9: I = 0 as libc::c_int as I;
            let mut _i_9: I = zn;
            while i_9 < _i_9 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_9 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I)
                    .offset(0 as libc::c_int as isize)
                    * *(((*b).k).as_mut_ptr() as *mut I).offset(i_9 as isize);
                i_9 += 1;
                i_9;
            }
        } else {
            let mut i_10: I = 0 as libc::c_int as I;
            let mut _i_10: I = zn;
            while i_10 < _i_10 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_10 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I).offset(i_10 as isize)
                    * *(((*b).k).as_mut_ptr() as *mut I)
                        .offset(0 as libc::c_int as isize);
                i_10 += 1;
                i_10;
            }
        }
    } else if 0 as libc::c_int as libc::c_longlong == at
        || 0 as libc::c_int as libc::c_longlong == bt
    {
        dp(&mut z, Some(times as unsafe extern "C" fn(K, K) -> K), a, b);
    }
    return z;
}
pub unsafe extern "C" fn _dot(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    let mut type_0: I = if (if at < 0 as libc::c_int as libc::c_longlong {
        -at
    } else {
        at
    }) > (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if at < 0 as libc::c_int as libc::c_longlong { -at } else { at }
    } else if bt < 0 as libc::c_int as libc::c_longlong {
        -bt
    } else {
        bt
    };
    if at <= 0 as libc::c_int as libc::c_longlong
        && bt <= 0 as libc::c_int as libc::c_longlong && an != bn
    {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    if type_0 > 2 as libc::c_int as libc::c_longlong {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut zt: I = type_0;
    if (if at < bt { at } else { bt }) < 1 as libc::c_int as libc::c_longlong {
        zt = -zt;
    }
    if at == 0 || bt == 0 {
        zt = 0 as libc::c_int as I;
    }
    let mut zn: I = if at > 0 as libc::c_int as libc::c_longlong { bn } else { an };
    let mut A: I = if at < 0 as libc::c_int as libc::c_longlong { -at } else { at };
    let mut B: I = if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt };
    let mut accI: I = 0 as libc::c_int as I;
    let mut accF: F = 0.0f64;
    if 2 as libc::c_int as libc::c_longlong == A
        && 2 as libc::c_int as libc::c_longlong == B
    {
        let mut x: F = 0.;
        let mut y: F = 0.;
        if an == bn {
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = zn;
            while i < _i {
                x = *(((*a).k).as_mut_ptr() as *mut F).offset(i as isize);
                y = *(((*b).k).as_mut_ptr() as *mut F).offset(i as isize);
                accF += x * y;
                i += 1;
                i;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            x = *(((*a).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize);
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = zn;
            while i_0 < _i_0 {
                y = *(((*b).k).as_mut_ptr() as *mut F).offset(i_0 as isize);
                accF += x * y;
                i_0 += 1;
                i_0;
            }
        } else {
            y = *(((*b).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize);
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = zn;
            while i_1 < _i_1 {
                x = *(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize);
                accF += x * y;
                i_1 += 1;
                i_1;
            }
        }
    } else if 2 as libc::c_int as libc::c_longlong == A
        && 1 as libc::c_int as libc::c_longlong == B
    {
        let mut x_0: F = 0.;
        let mut y_0: I = 0;
        if an == bn {
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = zn;
            while i_2 < _i_2 {
                x_0 = *(((*a).k).as_mut_ptr() as *mut F).offset(i_2 as isize);
                y_0 = *(((*b).k).as_mut_ptr() as *mut I).offset(i_2 as isize);
                accF
                    += x_0
                        * (if 9223372036854775807 as libc::c_longlong == y_0 {
                            1 as libc::c_int as libc::c_double / 0.0f64
                        } else {
                            (if -(9223372036854775807 as libc::c_longlong) == y_0 {
                                -(1 as libc::c_int as libc::c_double / 0.0f64)
                            } else {
                                (if -(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong == y_0
                                {
                                    0 as libc::c_int as libc::c_double / 0.0f64
                                } else {
                                    y_0 as libc::c_double
                                })
                            })
                        });
                i_2 += 1;
                i_2;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            x_0 = *(((*a).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize);
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = zn;
            while i_3 < _i_3 {
                y_0 = *(((*b).k).as_mut_ptr() as *mut I).offset(i_3 as isize);
                accF
                    += x_0
                        * (if 9223372036854775807 as libc::c_longlong == y_0 {
                            1 as libc::c_int as libc::c_double / 0.0f64
                        } else {
                            (if -(9223372036854775807 as libc::c_longlong) == y_0 {
                                -(1 as libc::c_int as libc::c_double / 0.0f64)
                            } else {
                                (if -(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong == y_0
                                {
                                    0 as libc::c_int as libc::c_double / 0.0f64
                                } else {
                                    y_0 as libc::c_double
                                })
                            })
                        });
                i_3 += 1;
                i_3;
            }
        } else {
            y_0 = *(((*b).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize);
            let mut i_4: I = 0 as libc::c_int as I;
            let mut _i_4: I = zn;
            while i_4 < _i_4 {
                x_0 = *(((*a).k).as_mut_ptr() as *mut F).offset(i_4 as isize);
                accF
                    += x_0
                        * (if 9223372036854775807 as libc::c_longlong == y_0 {
                            1 as libc::c_int as libc::c_double / 0.0f64
                        } else {
                            (if -(9223372036854775807 as libc::c_longlong) == y_0 {
                                -(1 as libc::c_int as libc::c_double / 0.0f64)
                            } else {
                                (if -(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong == y_0
                                {
                                    0 as libc::c_int as libc::c_double / 0.0f64
                                } else {
                                    y_0 as libc::c_double
                                })
                            })
                        });
                i_4 += 1;
                i_4;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong == A
        && 2 as libc::c_int as libc::c_longlong == B
    {
        let mut x_1: I = 0;
        let mut y_1: F = 0.;
        if an == bn {
            let mut i_5: I = 0 as libc::c_int as I;
            let mut _i_5: I = zn;
            while i_5 < _i_5 {
                x_1 = *(((*a).k).as_mut_ptr() as *mut I).offset(i_5 as isize);
                y_1 = *(((*b).k).as_mut_ptr() as *mut F).offset(i_5 as isize);
                accF
                    += (if 9223372036854775807 as libc::c_longlong == x_1 {
                        1 as libc::c_int as libc::c_double / 0.0f64
                    } else {
                        (if -(9223372036854775807 as libc::c_longlong) == x_1 {
                            -(1 as libc::c_int as libc::c_double / 0.0f64)
                        } else {
                            (if -(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong == x_1
                            {
                                0 as libc::c_int as libc::c_double / 0.0f64
                            } else {
                                x_1 as libc::c_double
                            })
                        })
                    }) * y_1;
                i_5 += 1;
                i_5;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            x_1 = *(((*a).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize);
            let mut i_6: I = 0 as libc::c_int as I;
            let mut _i_6: I = zn;
            while i_6 < _i_6 {
                y_1 = *(((*b).k).as_mut_ptr() as *mut F).offset(i_6 as isize);
                accF
                    += (if 9223372036854775807 as libc::c_longlong == x_1 {
                        1 as libc::c_int as libc::c_double / 0.0f64
                    } else {
                        (if -(9223372036854775807 as libc::c_longlong) == x_1 {
                            -(1 as libc::c_int as libc::c_double / 0.0f64)
                        } else {
                            (if -(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong == x_1
                            {
                                0 as libc::c_int as libc::c_double / 0.0f64
                            } else {
                                x_1 as libc::c_double
                            })
                        })
                    }) * y_1;
                i_6 += 1;
                i_6;
            }
        } else {
            y_1 = *(((*b).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize);
            let mut i_7: I = 0 as libc::c_int as I;
            let mut _i_7: I = zn;
            while i_7 < _i_7 {
                x_1 = *(((*a).k).as_mut_ptr() as *mut I).offset(i_7 as isize);
                accF
                    += (if 9223372036854775807 as libc::c_longlong == x_1 {
                        1 as libc::c_int as libc::c_double / 0.0f64
                    } else {
                        (if -(9223372036854775807 as libc::c_longlong) == x_1 {
                            -(1 as libc::c_int as libc::c_double / 0.0f64)
                        } else {
                            (if -(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong == x_1
                            {
                                0 as libc::c_int as libc::c_double / 0.0f64
                            } else {
                                x_1 as libc::c_double
                            })
                        })
                    }) * y_1;
                i_7 += 1;
                i_7;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong == A
        && 1 as libc::c_int as libc::c_longlong == B
    {
        let mut x_2: I = 0;
        let mut y_2: I = 0;
        if an == bn {
            let mut i_8: I = 0 as libc::c_int as I;
            let mut _i_8: I = zn;
            while i_8 < _i_8 {
                x_2 = *(((*a).k).as_mut_ptr() as *mut I).offset(i_8 as isize);
                y_2 = *(((*b).k).as_mut_ptr() as *mut I).offset(i_8 as isize);
                accI += x_2 * y_2;
                i_8 += 1;
                i_8;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            x_2 = *(((*a).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize);
            let mut i_9: I = 0 as libc::c_int as I;
            let mut _i_9: I = zn;
            while i_9 < _i_9 {
                y_2 = *(((*b).k).as_mut_ptr() as *mut I).offset(i_9 as isize);
                accI += x_2 * y_2;
                i_9 += 1;
                i_9;
            }
        } else {
            y_2 = *(((*b).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize);
            let mut i_10: I = 0 as libc::c_int as I;
            let mut _i_10: I = zn;
            while i_10 < _i_10 {
                x_2 = *(((*a).k).as_mut_ptr() as *mut I).offset(i_10 as isize);
                accI += x_2 * y_2;
                i_10 += 1;
                i_10;
            }
        }
    } else if 0 as libc::c_int as libc::c_longlong == A
        || 0 as libc::c_int as libc::c_longlong == B
    {
        let mut p: [V; 2] = [0 as V, 0x16 as libc::c_int as V];
        let mut x_3: K = 0 as *mut k0;
        x_3 = times(a, b);
        let mut y_3: K = overDyad(
            0 as K,
            p.as_mut_ptr().offset(2 as libc::c_int as isize),
            x_3,
        );
        cd(x_3);
        return y_3;
    }
    return if 1 as libc::c_int as libc::c_longlong
        == (if zt < 0 as libc::c_int as libc::c_longlong { -zt } else { zt })
    {
        Ki(accI)
    } else {
        Kf(accF)
    };
}
#[export_name = "mod"]
pub unsafe extern "C" fn mod_0(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    if (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        > 2 as libc::c_int as libc::c_longlong
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut t: I = if 0 as libc::c_int as libc::c_longlong == at {
        0 as libc::c_int as libc::c_longlong
    } else {
        (if (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
            > (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
        {
            (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        } else {
            (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
        })
            * (if at > 0 as libc::c_int as libc::c_longlong {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            }) as libc::c_longlong
    };
    let mut z: K = newK(t, an);
    if z.is_null() {
        return 0 as K;
    }
    let mut c: I = 0;
    let mut d: I = 0;
    let mut e: I = 0;
    let mut f: F = 0.;
    let mut g: F = 0.;
    let mut h: F = 0.;
    let mut ct: F = 1e-13f64;
    if 2 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 2 as libc::c_int as libc::c_longlong == bt
    {
        g = *(((*b).k).as_mut_ptr() as *mut F);
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = an;
        while i < _i {
            f = *(((*a).k).as_mut_ptr() as *mut F).offset(i as isize);
            h = if g != 0. { f - g * floor(ct + f / g) } else { f };
            *(((*z).k).as_mut_ptr() as *mut F)
                .offset(
                    i as isize,
                ) = if (if h < 0 as libc::c_int as libc::c_double { -h } else { h }) > ct
            {
                h
            } else {
                0 as libc::c_int as libc::c_double
            };
            i += 1;
            i;
        }
    } else if 2 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 1 as libc::c_int as libc::c_longlong == bt
    {
        g = *(((*b).k).as_mut_ptr() as *mut I) as F;
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = an;
        while i_0 < _i_0 {
            f = *(((*a).k).as_mut_ptr() as *mut F).offset(i_0 as isize);
            h = if g != 0. { f - g * floor(ct + f / g) } else { f };
            *(((*z).k).as_mut_ptr() as *mut F)
                .offset(
                    i_0 as isize,
                ) = if (if h < 0 as libc::c_int as libc::c_double { -h } else { h }) > ct
            {
                h
            } else {
                0 as libc::c_int as libc::c_double
            };
            i_0 += 1;
            i_0;
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 2 as libc::c_int as libc::c_longlong == bt
    {
        g = *(((*b).k).as_mut_ptr() as *mut F);
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = an;
        while i_1 < _i_1 {
            f = *(((*a).k).as_mut_ptr() as *mut I).offset(i_1 as isize) as F;
            h = if g != 0. { f - g * floor(ct + f / g) } else { f };
            *(((*z).k).as_mut_ptr() as *mut F)
                .offset(
                    i_1 as isize,
                ) = if (if h < 0 as libc::c_int as libc::c_double { -h } else { h }) > ct
            {
                h
            } else {
                0 as libc::c_int as libc::c_double
            };
            i_1 += 1;
            i_1;
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 1 as libc::c_int as libc::c_longlong == bt
    {
        d = *(((*b).k).as_mut_ptr() as *mut I);
        g = d as F;
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = an;
        while i_2 < _i_2 {
            c = *(((*a).k).as_mut_ptr() as *mut I).offset(i_2 as isize);
            e = (if d != 0 {
                c as libc::c_double
                    - d as libc::c_double * floor(c as libc::c_double / g)
            } else {
                c as libc::c_double
            }) as I;
            *(((*z).k).as_mut_ptr() as *mut I).offset(i_2 as isize) = e;
            i_2 += 1;
            i_2;
        }
    } else if 0 as libc::c_int as libc::c_longlong == at {
        let mut i_3: I = 0 as libc::c_int as I;
        let mut _i_3: I = an;
        while i_3 < _i_3 {
            let ref mut fresh0 = *((*z).k).as_mut_ptr().offset(i_3 as isize);
            *fresh0 = mod_0(*((*a).k).as_mut_ptr().offset(i_3 as isize), b);
            if (*fresh0).is_null() {
                cd(z);
                return 0 as K;
            }
            i_3 += 1;
            i_3;
        }
    }
    return z;
}
pub unsafe extern "C" fn minus(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    let mut type_0: I = if (if at < 0 as libc::c_int as libc::c_longlong {
        -at
    } else {
        at
    }) > (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if at < 0 as libc::c_int as libc::c_longlong { -at } else { at }
    } else if bt < 0 as libc::c_int as libc::c_longlong {
        -bt
    } else {
        bt
    };
    if at <= 0 as libc::c_int as libc::c_longlong
        && bt <= 0 as libc::c_int as libc::c_longlong && an != bn
    {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    if type_0 > 2 as libc::c_int as libc::c_longlong {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut zt: I = type_0;
    if (if at < bt { at } else { bt }) < 1 as libc::c_int as libc::c_longlong {
        zt = -zt;
    }
    if at == 0 || bt == 0 {
        zt = 0 as libc::c_int as I;
    }
    let mut zn: I = if at > 0 as libc::c_int as libc::c_longlong { bn } else { an };
    let mut z: K = newK(zt, zn);
    if z.is_null() {
        return 0 as K;
    }
    if 2 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 2 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = zn;
            while i < _i {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F).offset(i as isize)
                    - *(((*b).k).as_mut_ptr() as *mut F).offset(i as isize);
                i += 1;
                i;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = zn;
            while i_0 < _i_0 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_0 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F)
                    .offset(0 as libc::c_int as isize)
                    - *(((*b).k).as_mut_ptr() as *mut F).offset(i_0 as isize);
                i_0 += 1;
                i_0;
            }
        } else {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = zn;
            while i_1 < _i_1 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_1 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize)
                    - *(((*b).k).as_mut_ptr() as *mut F)
                        .offset(0 as libc::c_int as isize);
                i_1 += 1;
                i_1;
            }
        }
    } else if 2 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 1 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = zn;
            while i_2 < _i_2 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_2 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F).offset(i_2 as isize)
                    - *(((*b).k).as_mut_ptr() as *mut I).offset(i_2 as isize)
                        as libc::c_double;
                i_2 += 1;
                i_2;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = zn;
            while i_3 < _i_3 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_3 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F)
                    .offset(0 as libc::c_int as isize)
                    - *(((*b).k).as_mut_ptr() as *mut I).offset(i_3 as isize)
                        as libc::c_double;
                i_3 += 1;
                i_3;
            }
        } else {
            let mut i_4: I = 0 as libc::c_int as I;
            let mut _i_4: I = zn;
            while i_4 < _i_4 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_4 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut F).offset(i_4 as isize)
                    - *(((*b).k).as_mut_ptr() as *mut I)
                        .offset(0 as libc::c_int as isize) as libc::c_double;
                i_4 += 1;
                i_4;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 2 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_5: I = 0 as libc::c_int as I;
            let mut _i_5: I = zn;
            while i_5 < _i_5 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_5 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I).offset(i_5 as isize)
                    as libc::c_double
                    - *(((*b).k).as_mut_ptr() as *mut F).offset(i_5 as isize);
                i_5 += 1;
                i_5;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_6: I = 0 as libc::c_int as I;
            let mut _i_6: I = zn;
            while i_6 < _i_6 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_6 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I)
                    .offset(0 as libc::c_int as isize) as libc::c_double
                    - *(((*b).k).as_mut_ptr() as *mut F).offset(i_6 as isize);
                i_6 += 1;
                i_6;
            }
        } else {
            let mut i_7: I = 0 as libc::c_int as I;
            let mut _i_7: I = zn;
            while i_7 < _i_7 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_7 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I).offset(i_7 as isize)
                    as libc::c_double
                    - *(((*b).k).as_mut_ptr() as *mut F)
                        .offset(0 as libc::c_int as isize);
                i_7 += 1;
                i_7;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 1 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_8: I = 0 as libc::c_int as I;
            let mut _i_8: I = zn;
            while i_8 < _i_8 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_8 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I).offset(i_8 as isize)
                    - *(((*b).k).as_mut_ptr() as *mut I).offset(i_8 as isize);
                i_8 += 1;
                i_8;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_9: I = 0 as libc::c_int as I;
            let mut _i_9: I = zn;
            while i_9 < _i_9 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_9 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I)
                    .offset(0 as libc::c_int as isize)
                    - *(((*b).k).as_mut_ptr() as *mut I).offset(i_9 as isize);
                i_9 += 1;
                i_9;
            }
        } else {
            let mut i_10: I = 0 as libc::c_int as I;
            let mut _i_10: I = zn;
            while i_10 < _i_10 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_10 as isize,
                    ) = *(((*a).k).as_mut_ptr() as *mut I).offset(i_10 as isize)
                    - *(((*b).k).as_mut_ptr() as *mut I)
                        .offset(0 as libc::c_int as isize);
                i_10 += 1;
                i_10;
            }
        }
    } else if 0 as libc::c_int as libc::c_longlong == at
        || 0 as libc::c_int as libc::c_longlong == bt
    {
        dp(&mut z, Some(minus as unsafe extern "C" fn(K, K) -> K), a, b);
    }
    return z;
}
pub unsafe extern "C" fn negate(mut x: K) -> K {
    let mut y: K = 0 as *mut k0;
    let mut z: K = 0 as *mut k0;
    y = Ki(0 as libc::c_int as I);
    if y.is_null() {
        return 0 as K;
    }
    z = minus(y, x);
    cd(y);
    return z;
}
pub unsafe extern "C" fn divide(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    let mut type_0: I = if (if at < 0 as libc::c_int as libc::c_longlong {
        -at
    } else {
        at
    }) > (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if at < 0 as libc::c_int as libc::c_longlong { -at } else { at }
    } else if bt < 0 as libc::c_int as libc::c_longlong {
        -bt
    } else {
        bt
    };
    if at <= 0 as libc::c_int as libc::c_longlong
        && bt <= 0 as libc::c_int as libc::c_longlong && an != bn
    {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    if type_0 > 2 as libc::c_int as libc::c_longlong {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut zt: I = type_0;
    if (if at < bt { at } else { bt }) < 1 as libc::c_int as libc::c_longlong {
        zt = -zt;
    }
    if at == 0 || bt == 0 {
        zt = 0 as libc::c_int as I;
    }
    let mut zn: I = if at > 0 as libc::c_int as libc::c_longlong { bn } else { an };
    let mut z: K = newK(zt, zn);
    if z.is_null() {
        return 0 as K;
    }
    let mut u: F = 0.;
    let mut d: F = 0.;
    let mut y: F = 1 as libc::c_int as libc::c_double / 0.0f64;
    let mut s: I = 0;
    let mut t: I = 0;
    let mut w: I = 9223372036854775807 as libc::c_longlong;
    if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 1 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = zn;
            while i < _i {
                s = *(((*a).k).as_mut_ptr() as *mut I).offset(i as isize);
                t = *(((*b).k).as_mut_ptr() as *mut I).offset(i as isize);
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i as isize,
                    ) = if t == 0 {
                    if s == 0 {
                        -(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong
                    } else if s > 0 as libc::c_int as libc::c_longlong {
                        w
                    } else {
                        -w
                    }
                } else {
                    s / t
                };
                i += 1;
                i;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = zn;
            while i_0 < _i_0 {
                s = *(((*a).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize);
                t = *(((*b).k).as_mut_ptr() as *mut I).offset(i_0 as isize);
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_0 as isize,
                    ) = if t == 0 {
                    if s == 0 {
                        -(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong
                    } else if s > 0 as libc::c_int as libc::c_longlong {
                        w
                    } else {
                        -w
                    }
                } else {
                    s / t
                };
                i_0 += 1;
                i_0;
            }
        } else {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = zn;
            while i_1 < _i_1 {
                s = *(((*a).k).as_mut_ptr() as *mut I).offset(i_1 as isize);
                t = *(((*b).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize);
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_1 as isize,
                    ) = if t == 0 {
                    if s == 0 {
                        -(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong
                    } else if s > 0 as libc::c_int as libc::c_longlong {
                        w
                    } else {
                        -w
                    }
                } else {
                    s / t
                };
                i_1 += 1;
                i_1;
            }
        }
        return z;
    }
    if 2 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 2 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = zn;
            while i_2 < _i_2 {
                u = *(((*a).k).as_mut_ptr() as *mut F).offset(i_2 as isize);
                d = *(((*b).k).as_mut_ptr() as *mut F).offset(i_2 as isize);
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_2 as isize,
                    ) = if d == 0. {
                    if u == 0. {
                        0 as libc::c_int as libc::c_double / 0.0f64
                    } else if u > 0 as libc::c_int as libc::c_double {
                        y
                    } else {
                        -y
                    }
                } else {
                    u / d
                };
                i_2 += 1;
                i_2;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            u = *(((*a).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize);
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = zn;
            while i_3 < _i_3 {
                d = *(((*b).k).as_mut_ptr() as *mut F).offset(i_3 as isize);
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_3 as isize,
                    ) = if d == 0. {
                    if u == 0. {
                        0 as libc::c_int as libc::c_double / 0.0f64
                    } else if u > 0 as libc::c_int as libc::c_double {
                        y
                    } else {
                        -y
                    }
                } else {
                    u / d
                };
                i_3 += 1;
                i_3;
            }
        } else {
            d = *(((*b).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize);
            let mut i_4: I = 0 as libc::c_int as I;
            let mut _i_4: I = zn;
            while i_4 < _i_4 {
                u = *(((*a).k).as_mut_ptr() as *mut F).offset(i_4 as isize);
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_4 as isize,
                    ) = if d == 0. {
                    if u == 0. {
                        0 as libc::c_int as libc::c_double / 0.0f64
                    } else if u > 0 as libc::c_int as libc::c_double {
                        y
                    } else {
                        -y
                    }
                } else {
                    u / d
                };
                i_4 += 1;
                i_4;
            }
        }
    } else if 2 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 1 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_5: I = 0 as libc::c_int as I;
            let mut _i_5: I = zn;
            while i_5 < _i_5 {
                u = *(((*a).k).as_mut_ptr() as *mut F).offset(i_5 as isize);
                d = *(((*b).k).as_mut_ptr() as *mut I).offset(i_5 as isize) as F;
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_5 as isize,
                    ) = if d == 0. {
                    if u == 0. {
                        0 as libc::c_int as libc::c_double / 0.0f64
                    } else if u > 0 as libc::c_int as libc::c_double {
                        y
                    } else {
                        -y
                    }
                } else {
                    u / d
                };
                i_5 += 1;
                i_5;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            u = *(((*a).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize);
            let mut i_6: I = 0 as libc::c_int as I;
            let mut _i_6: I = zn;
            while i_6 < _i_6 {
                d = *(((*b).k).as_mut_ptr() as *mut I).offset(i_6 as isize) as F;
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_6 as isize,
                    ) = if d == 0. {
                    if u == 0. {
                        0 as libc::c_int as libc::c_double / 0.0f64
                    } else if u > 0 as libc::c_int as libc::c_double {
                        y
                    } else {
                        -y
                    }
                } else {
                    u / d
                };
                i_6 += 1;
                i_6;
            }
        } else {
            d = *(((*b).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize)
                as F;
            let mut i_7: I = 0 as libc::c_int as I;
            let mut _i_7: I = zn;
            while i_7 < _i_7 {
                u = *(((*a).k).as_mut_ptr() as *mut F).offset(i_7 as isize);
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_7 as isize,
                    ) = if d == 0. {
                    if u == 0. {
                        0 as libc::c_int as libc::c_double / 0.0f64
                    } else if u > 0 as libc::c_int as libc::c_double {
                        y
                    } else {
                        -y
                    }
                } else {
                    u / d
                };
                i_7 += 1;
                i_7;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 2 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_8: I = 0 as libc::c_int as I;
            let mut _i_8: I = zn;
            while i_8 < _i_8 {
                u = *(((*a).k).as_mut_ptr() as *mut I).offset(i_8 as isize) as F;
                d = *(((*b).k).as_mut_ptr() as *mut F).offset(i_8 as isize);
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_8 as isize,
                    ) = if d == 0. {
                    if u == 0. {
                        0 as libc::c_int as libc::c_double / 0.0f64
                    } else if u > 0 as libc::c_int as libc::c_double {
                        y
                    } else {
                        -y
                    }
                } else {
                    u / d
                };
                i_8 += 1;
                i_8;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            u = *(((*a).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize)
                as F;
            let mut i_9: I = 0 as libc::c_int as I;
            let mut _i_9: I = zn;
            while i_9 < _i_9 {
                d = *(((*b).k).as_mut_ptr() as *mut F).offset(i_9 as isize);
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_9 as isize,
                    ) = if d == 0. {
                    if u == 0. {
                        0 as libc::c_int as libc::c_double / 0.0f64
                    } else if u > 0 as libc::c_int as libc::c_double {
                        y
                    } else {
                        -y
                    }
                } else {
                    u / d
                };
                i_9 += 1;
                i_9;
            }
        } else {
            d = *(((*b).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize);
            let mut i_10: I = 0 as libc::c_int as I;
            let mut _i_10: I = zn;
            while i_10 < _i_10 {
                u = *(((*a).k).as_mut_ptr() as *mut I).offset(i_10 as isize) as F;
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_10 as isize,
                    ) = if d == 0. {
                    if u == 0. {
                        0 as libc::c_int as libc::c_double / 0.0f64
                    } else if u > 0 as libc::c_int as libc::c_double {
                        y
                    } else {
                        -y
                    }
                } else {
                    u / d
                };
                i_10 += 1;
                i_10;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 1 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_11: I = 0 as libc::c_int as I;
            let mut _i_11: I = zn;
            while i_11 < _i_11 {
                u = *(((*a).k).as_mut_ptr() as *mut I).offset(i_11 as isize) as F;
                d = *(((*b).k).as_mut_ptr() as *mut I).offset(i_11 as isize) as F;
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_11 as isize,
                    ) = if d == 0. {
                    if u == 0. {
                        0 as libc::c_int as libc::c_double / 0.0f64
                    } else if u > 0 as libc::c_int as libc::c_double {
                        y
                    } else {
                        -y
                    }
                } else {
                    u / d
                };
                i_11 += 1;
                i_11;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            u = *(((*a).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize)
                as F;
            let mut i_12: I = 0 as libc::c_int as I;
            let mut _i_12: I = zn;
            while i_12 < _i_12 {
                d = *(((*b).k).as_mut_ptr() as *mut I).offset(i_12 as isize) as F;
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_12 as isize,
                    ) = if d == 0. {
                    if u == 0. {
                        0 as libc::c_int as libc::c_double / 0.0f64
                    } else if u > 0 as libc::c_int as libc::c_double {
                        y
                    } else {
                        -y
                    }
                } else {
                    u / d
                };
                i_12 += 1;
                i_12;
            }
        } else {
            d = *(((*b).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize)
                as F;
            let mut i_13: I = 0 as libc::c_int as I;
            let mut _i_13: I = zn;
            while i_13 < _i_13 {
                u = *(((*a).k).as_mut_ptr() as *mut I).offset(i_13 as isize) as F;
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_13 as isize,
                    ) = if d == 0. {
                    if u == 0. {
                        0 as libc::c_int as libc::c_double / 0.0f64
                    } else if u > 0 as libc::c_int as libc::c_double {
                        y
                    } else {
                        -y
                    }
                } else {
                    u / d
                };
                i_13 += 1;
                i_13;
            }
        }
    } else if 0 as libc::c_int as libc::c_longlong == at
        || 0 as libc::c_int as libc::c_longlong == bt
    {
        dp(&mut z, Some(divide as unsafe extern "C" fn(K, K) -> K), a, b);
    }
    return z;
}
pub unsafe extern "C" fn reciprocal(mut x: K) -> K {
    let mut y: K = 0 as *mut k0;
    let mut z: K = 0 as *mut k0;
    y = Kf(1 as libc::c_int as F);
    if y.is_null() {
        return 0 as K;
    }
    z = divide(y, x);
    cd(y);
    return z;
}
pub unsafe extern "C" fn min_and(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    let mut type_0: I = if (if at < 0 as libc::c_int as libc::c_longlong {
        -at
    } else {
        at
    }) > (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if at < 0 as libc::c_int as libc::c_longlong { -at } else { at }
    } else if bt < 0 as libc::c_int as libc::c_longlong {
        -bt
    } else {
        bt
    };
    if at <= 0 as libc::c_int as libc::c_longlong
        && bt <= 0 as libc::c_int as libc::c_longlong && an != bn
    {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    if type_0 > 2 as libc::c_int as libc::c_longlong {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut zt: I = type_0;
    if (if at < bt { at } else { bt }) < 1 as libc::c_int as libc::c_longlong {
        zt = -zt;
    }
    if at == 0 || bt == 0 {
        zt = 0 as libc::c_int as I;
    }
    let mut zn: I = if at > 0 as libc::c_int as libc::c_longlong { bn } else { an };
    let mut z: K = newK(zt, zn);
    if z.is_null() {
        return 0 as K;
    }
    if 2 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 2 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = zn;
            while i < _i {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut F).offset(i as isize)
                    < *(((*b).k).as_mut_ptr() as *mut F).offset(i as isize)
                {
                    *(((*a).k).as_mut_ptr() as *mut F).offset(i as isize)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut F).offset(i as isize)
                };
                i += 1;
                i;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = zn;
            while i_0 < _i_0 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_0 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut F)
                    .offset(0 as libc::c_int as isize)
                    < *(((*b).k).as_mut_ptr() as *mut F).offset(i_0 as isize)
                {
                    *(((*a).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut F).offset(i_0 as isize)
                };
                i_0 += 1;
                i_0;
            }
        } else {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = zn;
            while i_1 < _i_1 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_1 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize)
                    < *(((*b).k).as_mut_ptr() as *mut F)
                        .offset(0 as libc::c_int as isize)
                {
                    *(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize)
                };
                i_1 += 1;
                i_1;
            }
        }
    } else if 2 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 1 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = zn;
            while i_2 < _i_2 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_2 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut F).offset(i_2 as isize)
                    < *(((*b).k).as_mut_ptr() as *mut I).offset(i_2 as isize)
                        as libc::c_double
                {
                    *(((*a).k).as_mut_ptr() as *mut F).offset(i_2 as isize)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut I).offset(i_2 as isize)
                        as libc::c_double
                };
                i_2 += 1;
                i_2;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = zn;
            while i_3 < _i_3 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_3 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut F)
                    .offset(0 as libc::c_int as isize)
                    < *(((*b).k).as_mut_ptr() as *mut I).offset(i_3 as isize)
                        as libc::c_double
                {
                    *(((*a).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut I).offset(i_3 as isize)
                        as libc::c_double
                };
                i_3 += 1;
                i_3;
            }
        } else {
            let mut i_4: I = 0 as libc::c_int as I;
            let mut _i_4: I = zn;
            while i_4 < _i_4 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_4 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut F).offset(i_4 as isize)
                    < *(((*b).k).as_mut_ptr() as *mut I)
                        .offset(0 as libc::c_int as isize) as libc::c_double
                {
                    *(((*a).k).as_mut_ptr() as *mut F).offset(i_4 as isize)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize)
                        as libc::c_double
                };
                i_4 += 1;
                i_4;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 2 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_5: I = 0 as libc::c_int as I;
            let mut _i_5: I = zn;
            while i_5 < _i_5 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_5 as isize,
                    ) = if (*(((*a).k).as_mut_ptr() as *mut I).offset(i_5 as isize)
                    as libc::c_double)
                    < *(((*b).k).as_mut_ptr() as *mut F).offset(i_5 as isize)
                {
                    *(((*a).k).as_mut_ptr() as *mut I).offset(i_5 as isize)
                        as libc::c_double
                } else {
                    *(((*b).k).as_mut_ptr() as *mut F).offset(i_5 as isize)
                };
                i_5 += 1;
                i_5;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_6: I = 0 as libc::c_int as I;
            let mut _i_6: I = zn;
            while i_6 < _i_6 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_6 as isize,
                    ) = if (*(((*a).k).as_mut_ptr() as *mut I)
                    .offset(0 as libc::c_int as isize) as libc::c_double)
                    < *(((*b).k).as_mut_ptr() as *mut F).offset(i_6 as isize)
                {
                    *(((*a).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize)
                        as libc::c_double
                } else {
                    *(((*b).k).as_mut_ptr() as *mut F).offset(i_6 as isize)
                };
                i_6 += 1;
                i_6;
            }
        } else {
            let mut i_7: I = 0 as libc::c_int as I;
            let mut _i_7: I = zn;
            while i_7 < _i_7 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_7 as isize,
                    ) = if (*(((*a).k).as_mut_ptr() as *mut I).offset(i_7 as isize)
                    as libc::c_double)
                    < *(((*b).k).as_mut_ptr() as *mut F)
                        .offset(0 as libc::c_int as isize)
                {
                    *(((*a).k).as_mut_ptr() as *mut I).offset(i_7 as isize)
                        as libc::c_double
                } else {
                    *(((*b).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize)
                };
                i_7 += 1;
                i_7;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 1 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_8: I = 0 as libc::c_int as I;
            let mut _i_8: I = zn;
            while i_8 < _i_8 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_8 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut I).offset(i_8 as isize)
                    < *(((*b).k).as_mut_ptr() as *mut I).offset(i_8 as isize)
                {
                    *(((*a).k).as_mut_ptr() as *mut I).offset(i_8 as isize)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut I).offset(i_8 as isize)
                };
                i_8 += 1;
                i_8;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_9: I = 0 as libc::c_int as I;
            let mut _i_9: I = zn;
            while i_9 < _i_9 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_9 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut I)
                    .offset(0 as libc::c_int as isize)
                    < *(((*b).k).as_mut_ptr() as *mut I).offset(i_9 as isize)
                {
                    *(((*a).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut I).offset(i_9 as isize)
                };
                i_9 += 1;
                i_9;
            }
        } else {
            let mut i_10: I = 0 as libc::c_int as I;
            let mut _i_10: I = zn;
            while i_10 < _i_10 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_10 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut I).offset(i_10 as isize)
                    < *(((*b).k).as_mut_ptr() as *mut I)
                        .offset(0 as libc::c_int as isize)
                {
                    *(((*a).k).as_mut_ptr() as *mut I).offset(i_10 as isize)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize)
                };
                i_10 += 1;
                i_10;
            }
        }
    } else if 0 as libc::c_int as libc::c_longlong == at
        || 0 as libc::c_int as libc::c_longlong == bt
    {
        dp(&mut z, Some(min_and as unsafe extern "C" fn(K, K) -> K), a, b);
    }
    return z;
}
pub unsafe extern "C" fn max_or(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    let mut type_0: I = if (if at < 0 as libc::c_int as libc::c_longlong {
        -at
    } else {
        at
    }) > (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if at < 0 as libc::c_int as libc::c_longlong { -at } else { at }
    } else if bt < 0 as libc::c_int as libc::c_longlong {
        -bt
    } else {
        bt
    };
    if at <= 0 as libc::c_int as libc::c_longlong
        && bt <= 0 as libc::c_int as libc::c_longlong && an != bn
    {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    if type_0 > 2 as libc::c_int as libc::c_longlong {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut zt: I = type_0;
    if (if at < bt { at } else { bt }) < 1 as libc::c_int as libc::c_longlong {
        zt = -zt;
    }
    if at == 0 || bt == 0 {
        zt = 0 as libc::c_int as I;
    }
    let mut zn: I = if at > 0 as libc::c_int as libc::c_longlong { bn } else { an };
    let mut z: K = newK(zt, zn);
    if z.is_null() {
        return 0 as K;
    }
    if 2 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 2 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = zn;
            while i < _i {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut F).offset(i as isize)
                    > *(((*b).k).as_mut_ptr() as *mut F).offset(i as isize)
                {
                    *(((*a).k).as_mut_ptr() as *mut F).offset(i as isize)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut F).offset(i as isize)
                };
                i += 1;
                i;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = zn;
            while i_0 < _i_0 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_0 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut F)
                    .offset(0 as libc::c_int as isize)
                    > *(((*b).k).as_mut_ptr() as *mut F).offset(i_0 as isize)
                {
                    *(((*a).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut F).offset(i_0 as isize)
                };
                i_0 += 1;
                i_0;
            }
        } else {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = zn;
            while i_1 < _i_1 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_1 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize)
                    > *(((*b).k).as_mut_ptr() as *mut F)
                        .offset(0 as libc::c_int as isize)
                {
                    *(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize)
                };
                i_1 += 1;
                i_1;
            }
        }
    } else if 2 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 1 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = zn;
            while i_2 < _i_2 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_2 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut F).offset(i_2 as isize)
                    > *(((*b).k).as_mut_ptr() as *mut I).offset(i_2 as isize)
                        as libc::c_double
                {
                    *(((*a).k).as_mut_ptr() as *mut F).offset(i_2 as isize)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut I).offset(i_2 as isize)
                        as libc::c_double
                };
                i_2 += 1;
                i_2;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_3: I = 0 as libc::c_int as I;
            let mut _i_3: I = zn;
            while i_3 < _i_3 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_3 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut F)
                    .offset(0 as libc::c_int as isize)
                    > *(((*b).k).as_mut_ptr() as *mut I).offset(i_3 as isize)
                        as libc::c_double
                {
                    *(((*a).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut I).offset(i_3 as isize)
                        as libc::c_double
                };
                i_3 += 1;
                i_3;
            }
        } else {
            let mut i_4: I = 0 as libc::c_int as I;
            let mut _i_4: I = zn;
            while i_4 < _i_4 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_4 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut F).offset(i_4 as isize)
                    > *(((*b).k).as_mut_ptr() as *mut I)
                        .offset(0 as libc::c_int as isize) as libc::c_double
                {
                    *(((*a).k).as_mut_ptr() as *mut F).offset(i_4 as isize)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize)
                        as libc::c_double
                };
                i_4 += 1;
                i_4;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 2 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_5: I = 0 as libc::c_int as I;
            let mut _i_5: I = zn;
            while i_5 < _i_5 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_5 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut I).offset(i_5 as isize)
                    as libc::c_double
                    > *(((*b).k).as_mut_ptr() as *mut F).offset(i_5 as isize)
                {
                    *(((*a).k).as_mut_ptr() as *mut I).offset(i_5 as isize)
                        as libc::c_double
                } else {
                    *(((*b).k).as_mut_ptr() as *mut F).offset(i_5 as isize)
                };
                i_5 += 1;
                i_5;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_6: I = 0 as libc::c_int as I;
            let mut _i_6: I = zn;
            while i_6 < _i_6 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_6 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut I)
                    .offset(0 as libc::c_int as isize) as libc::c_double
                    > *(((*b).k).as_mut_ptr() as *mut F).offset(i_6 as isize)
                {
                    *(((*a).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize)
                        as libc::c_double
                } else {
                    *(((*b).k).as_mut_ptr() as *mut F).offset(i_6 as isize)
                };
                i_6 += 1;
                i_6;
            }
        } else {
            let mut i_7: I = 0 as libc::c_int as I;
            let mut _i_7: I = zn;
            while i_7 < _i_7 {
                *(((*z).k).as_mut_ptr() as *mut F)
                    .offset(
                        i_7 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut I).offset(i_7 as isize)
                    as libc::c_double
                    > *(((*b).k).as_mut_ptr() as *mut F)
                        .offset(0 as libc::c_int as isize)
                {
                    *(((*a).k).as_mut_ptr() as *mut I).offset(i_7 as isize)
                        as libc::c_double
                } else {
                    *(((*b).k).as_mut_ptr() as *mut F).offset(0 as libc::c_int as isize)
                };
                i_7 += 1;
                i_7;
            }
        }
    } else if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        && 1 as libc::c_int as libc::c_longlong
            == (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
    {
        if an == bn {
            let mut i_8: I = 0 as libc::c_int as I;
            let mut _i_8: I = zn;
            while i_8 < _i_8 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_8 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut I).offset(i_8 as isize)
                    > *(((*b).k).as_mut_ptr() as *mut I).offset(i_8 as isize)
                {
                    *(((*a).k).as_mut_ptr() as *mut I).offset(i_8 as isize)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut I).offset(i_8 as isize)
                };
                i_8 += 1;
                i_8;
            }
        } else if an == 1 as libc::c_int as libc::c_longlong {
            let mut i_9: I = 0 as libc::c_int as I;
            let mut _i_9: I = zn;
            while i_9 < _i_9 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_9 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut I)
                    .offset(0 as libc::c_int as isize)
                    > *(((*b).k).as_mut_ptr() as *mut I).offset(i_9 as isize)
                {
                    *(((*a).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut I).offset(i_9 as isize)
                };
                i_9 += 1;
                i_9;
            }
        } else {
            let mut i_10: I = 0 as libc::c_int as I;
            let mut _i_10: I = zn;
            while i_10 < _i_10 {
                *(((*z).k).as_mut_ptr() as *mut I)
                    .offset(
                        i_10 as isize,
                    ) = if *(((*a).k).as_mut_ptr() as *mut I).offset(i_10 as isize)
                    > *(((*b).k).as_mut_ptr() as *mut I)
                        .offset(0 as libc::c_int as isize)
                {
                    *(((*a).k).as_mut_ptr() as *mut I).offset(i_10 as isize)
                } else {
                    *(((*b).k).as_mut_ptr() as *mut I).offset(0 as libc::c_int as isize)
                };
                i_10 += 1;
                i_10;
            }
        }
    } else if 0 as libc::c_int as libc::c_longlong == at
        || 0 as libc::c_int as libc::c_longlong == bt
    {
        dp(&mut z, Some(max_or as unsafe extern "C" fn(K, K) -> K), a, b);
    }
    return z;
}
pub unsafe extern "C" fn floor_ceil(
    mut a: K,
    mut g: Option::<unsafe extern "C" fn(F) -> F>,
) -> K {
    if strcmp(errmsg.as_mut_ptr(), b"(nil)\0" as *const u8 as *const libc::c_char) != 0 {
        return 0 as K;
    }
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut h: Option::<unsafe extern "C" fn(F) -> F> = if g
        == Some(ceil as unsafe extern "C" fn(libc::c_double) -> libc::c_double)
    {
        Some(floor as unsafe extern "C" fn(libc::c_double) -> libc::c_double)
    } else {
        Some(ceil as unsafe extern "C" fn(libc::c_double) -> libc::c_double)
    };
    if (2 as libc::c_int as libc::c_longlong)
        < (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        || 0 as libc::c_int as libc::c_longlong == at
            && 7 as libc::c_int as libc::c_longlong
                == (**((*a).k).as_mut_ptr().offset(0 as libc::c_int as isize)).t
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    if 1 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
    {
        return ci(a);
    }
    let mut z: K = newK(
        (if at != 0 {
            if at < 0 as libc::c_int as libc::c_longlong {
                -(1 as libc::c_int)
            } else {
                1 as libc::c_int
            }
        } else {
            0 as libc::c_int
        }) as I,
        an,
    );
    let mut e: F = 0.;
    let mut f: F = 0.;
    let mut r: I = 0;
    if 2 as libc::c_int as libc::c_longlong
        == (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
    {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = an;
        while i < _i {
            e = *(((*a).k).as_mut_ptr() as *mut F).offset(i as isize);
            if e.is_nan() as i32 != 0 {
                r = -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong;
            } else if if e.is_infinite() {
                if e.is_sign_positive() { 1 } else { -1 }
            } else {
                0
            } != 0 || e <= -(9223372036854775807 as libc::c_longlong) as libc::c_double
                || e >= 9223372036854775807 as libc::c_longlong as libc::c_double
            {
                r = if e < 0 as libc::c_int as libc::c_double {
                    -(9223372036854775807 as libc::c_longlong)
                } else {
                    9223372036854775807 as libc::c_longlong
                };
            } else {
                f = FF(e);
                r = (if f > 0 as libc::c_int as libc::c_double
                    && FC(f, 1 as libc::c_int as F) == 0
                    || f < 0 as libc::c_int as libc::c_double
                        && FC(f, 0 as libc::c_int as F) == 0
                {
                    h.unwrap()(e)
                } else {
                    g.unwrap()(e)
                }) as I;
            }
            *(((*z).k).as_mut_ptr() as *mut I).offset(i as isize) = r;
            i += 1;
            i;
        }
    } else if at == 0 {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = an;
        while i_0 < _i_0 {
            let ref mut fresh1 = *((*z).k).as_mut_ptr().offset(i_0 as isize);
            *fresh1 = floor_ceil(*((*a).k).as_mut_ptr().offset(i_0 as isize), g);
            i_0 += 1;
            i_0;
        }
    }
    return z;
}
pub unsafe extern "C" fn floor_verb(mut a: K) -> K {
    return floor_ceil(
        a,
        Some(floor as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
    );
}
