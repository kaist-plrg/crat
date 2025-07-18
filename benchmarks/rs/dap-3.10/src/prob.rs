use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn random() -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn finite(_: libc::c_double) -> libc::c_int;
    fn lgamma(_: libc::c_double) -> libc::c_double;
    fn erfc(_: libc::c_double) -> libc::c_double;
    fn erf(_: libc::c_double) -> libc::c_double;
    static mut dap_prtol: libc::c_double;
    static mut dap_err: *mut FILE;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub unsafe extern "C" fn dap_simp(
    mut f: Option::<unsafe extern "C" fn() -> libc::c_double>,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut n: libc::c_int,
) -> libc::c_double {
    let mut h: libc::c_double = 0.;
    let mut val: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    h = (b - a) / n as libc::c_double;
    val = 0.0f64;
    i = 1 as libc::c_int;
    while i <= n - 1 as libc::c_int {
        val
            += 4.0f64
                * ::std::mem::transmute::<
                    _,
                    fn(_) -> libc::c_double,
                >((Some(f.unwrap())).unwrap())(a + i as libc::c_double * h);
        i += 2 as libc::c_int;
    }
    i = 2 as libc::c_int;
    while i <= n - 2 as libc::c_int {
        val
            += 2.0f64
                * ::std::mem::transmute::<
                    _,
                    fn(_) -> libc::c_double,
                >((Some(f.unwrap())).unwrap())(a + i as libc::c_double * h);
        i += 2 as libc::c_int;
    }
    val
        += ::std::mem::transmute::<
            _,
            fn(_) -> libc::c_double,
        >((Some(f.unwrap())).unwrap())(a)
            + ::std::mem::transmute::<
                _,
                fn(_) -> libc::c_double,
            >((Some(f.unwrap())).unwrap())(b);
    return val * h / 3.0f64;
}
static mut dddi: libc::c_double = 0.;
unsafe extern "C" fn Tfun(mut x: libc::c_double) -> libc::c_double {
    if dddi == 0.0f64 {
        return 1.0f64;
    }
    return pow(cos(x), dddi - 1.0f64);
}
pub unsafe extern "C" fn probt(
    mut t1: libc::c_double,
    mut di: libc::c_int,
) -> libc::c_double {
    let mut c: libc::c_double = 0.;
    let mut ddi: libc::c_double = 0.;
    if finite(t1) == 0 {
        return 0.0f64 / 0.0f64;
    }
    ddi = di as libc::c_double;
    dddi = ddi;
    c = 1.0f64;
    while ddi > 2.0f64 {
        c *= (ddi - 1.0f64) / (ddi - 2.0f64);
        ddi -= 2.0f64;
    }
    if ddi == 2.0f64 {
        c *= 0.5f64;
    } else {
        c /= 3.14159265358979323846f64;
    }
    return c
        * dap_simp(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                Option::<unsafe extern "C" fn() -> libc::c_double>,
            >(Some(Tfun as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
            atan(t1 / sqrt(dddi)),
            1.57079632679489661923f64,
            1024 as libc::c_int,
        );
}
pub unsafe extern "C" fn zpoint(mut p: libc::c_double) -> libc::c_double {
    static mut a: [libc::c_double; 4] = [
        2.5066282f64,
        -18.6150006f64,
        41.3911977f64,
        -25.4410605f64,
    ];
    static mut b: [libc::c_double; 4] = [
        -8.4735109f64,
        23.0833674f64,
        -21.0622410f64,
        3.1308291f64,
    ];
    static mut c: [libc::c_double; 4] = [
        -2.7871893f64,
        -2.2979648f64,
        4.8501413f64,
        2.3212128f64,
    ];
    static mut d: [libc::c_double; 2] = [3.5438892f64, 1.6370678f64];
    let mut q: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut x0: libc::c_double = 0.;
    q = p - 0.5f64;
    if fabs(q) > 0.42f64 {
        r = p;
        if q > 0.0f64 {
            r = 1.0f64 - p;
        }
        if r <= 0.0f64 {
            fputs(
                b"(zpoint) input not between 0 and 1\n\0" as *const u8
                    as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        r = sqrt(-log(r));
        x = ((c[3 as libc::c_int as usize] * r + c[2 as libc::c_int as usize]) * r
            + c[1 as libc::c_int as usize]) * r + c[0 as libc::c_int as usize];
        x0 = x
            / ((d[1 as libc::c_int as usize] * r + d[0 as libc::c_int as usize]) * r
                + 1.0f64);
        if q < 0.0f64 {
            x0 = -x0;
        }
    } else {
        r = q * q;
        x = q
            * (((a[3 as libc::c_int as usize] * r + a[2 as libc::c_int as usize]) * r
                + a[1 as libc::c_int as usize]) * r + a[0 as libc::c_int as usize]);
        x0 = x
            / ((((b[3 as libc::c_int as usize] * r + b[2 as libc::c_int as usize]) * r
                + b[1 as libc::c_int as usize]) * r + b[0 as libc::c_int as usize]) * r
                + 1.0f64);
    }
    return -x0;
}
pub unsafe extern "C" fn tpoint(
    mut p: libc::c_double,
    mut df: libc::c_int,
) -> libc::c_double {
    let mut pt0: libc::c_double = 0.;
    let mut pr0: libc::c_double = 0.;
    let mut pt1: libc::c_double = 0.;
    let mut pr1: libc::c_double = 0.;
    let mut pt2: libc::c_double = 0.;
    let mut pr2: libc::c_double = 0.;
    pt2 = 0.0f64;
    pt0 = zpoint(p);
    pr0 = probt(pt0, df);
    pt1 = 2.0f64 * pt0;
    pr2 = pr0;
    while fabs(pr2 - p) > dap_prtol {
        pr1 = probt(pt1, df);
        pt2 = pt0 + (pt1 - pt0) * (p - pr0) / (pr1 - pr0);
        if pt2 < 0.0f64 {
            if pt0 < pt1 {
                pt2 = 0.5f64 * pt0;
            } else {
                pt2 = 0.5f64 * pt1;
            }
        }
        pr2 = probt(pt2, df);
        if fabs(pr2 - pr0) < fabs(pr2 - pr1) {
            pr1 = pr2;
            pt1 = pt2;
        } else {
            pr0 = pr2;
            pt0 = pt2;
        }
    }
    return pt2;
}
static mut dmpln: libc::c_double = 0.;
static mut drat: libc::c_double = 0.;
static mut dnm1: libc::c_double = 0.;
unsafe extern "C" fn Ffun(mut x: libc::c_double) -> libc::c_double {
    if dnm1 > 0.0f64 {
        return pow(x, dnm1) / pow(1.0f64 + drat * x, dmpln)
    } else {
        return 1.0f64 / pow(1.0f64 + drat * x, dmpln)
    };
}
pub unsafe extern "C" fn probf(
    mut f0: libc::c_double,
    mut numdf: libc::c_int,
    mut dendf: libc::c_int,
) -> libc::c_double {
    let mut c: libc::c_double = 0.;
    let mut dm: libc::c_double = 0.;
    let mut dn: libc::c_double = 0.;
    let mut dmpn: libc::c_double = 0.;
    let mut ddn: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut s0: libc::c_double = 0.;
    let mut s1: libc::c_double = 0.;
    let mut s2: libc::c_double = 0.;
    if finite(f0) == 0 {
        return 0.0f64 / 0.0f64;
    }
    if numdf == 1 as libc::c_int {
        return 2.0f64 * probt(sqrt(f0), dendf);
    }
    dn = 0.5f64 * numdf as libc::c_double;
    dm = 0.5f64 * dendf as libc::c_double;
    dmpn = dn + dm;
    dmpln = dm + dn;
    dnm1 = dn - 1.0f64;
    drat = dn / dm;
    ddn = dn;
    c = 1.0f64;
    while dmpn >= 0.5f64 {
        dm -= 1.0f64;
        dn -= 1.0f64;
        dmpn -= 1.0f64;
        if dm > 0.0f64 {
            c /= dm;
        } else if dm == -0.5f64 {
            c /= 1.772453850905516027297f64;
        }
        if dn > 0.0f64 {
            c /= dn;
        } else if dn == -0.5f64 {
            c /= 1.772453850905516027297f64;
        }
        if dmpn > 0.0f64 {
            c *= dmpn;
        } else if dmpn == -0.5f64 {
            c *= 1.772453850905516027297f64;
        }
    }
    c *= pow(drat, ddn);
    b = 1.0f64;
    while f0 >= b {
        b *= 2.0f64;
    }
    s = dap_simp(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
            Option::<unsafe extern "C" fn() -> libc::c_double>,
        >(Some(Ffun as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
        f0,
        b,
        1024 as libc::c_int,
    );
    s0 = dap_simp(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
            Option::<unsafe extern "C" fn() -> libc::c_double>,
        >(Some(Ffun as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
        b,
        2.0f64 * b,
        1024 as libc::c_int,
    );
    b *= 2.0f64;
    s1 = dap_simp(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
            Option::<unsafe extern "C" fn() -> libc::c_double>,
        >(Some(Ffun as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
        b,
        2.0f64 * b,
        1024 as libc::c_int,
    );
    b *= 2.0f64;
    s2 = dap_simp(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
            Option::<unsafe extern "C" fn() -> libc::c_double>,
        >(Some(Ffun as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
        b,
        2.0f64 * b,
        1024 as libc::c_int,
    );
    s += s0 + s1 + s2;
    while s2 < s1 && s2 * s2 / (s1 - s2) > 5.0e-11f64 / c {
        b *= 2.0f64;
        s0 = s1;
        s1 = s2;
        s2 = dap_simp(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                Option::<unsafe extern "C" fn() -> libc::c_double>,
            >(Some(Ffun as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
            b,
            2.0f64 * b,
            1024 as libc::c_int,
        );
        s += s2;
    }
    return c * s;
}
pub unsafe extern "C" fn fpoint(
    mut p: libc::c_double,
    mut numdf: libc::c_int,
    mut dendf: libc::c_int,
) -> libc::c_double {
    let mut pt0: libc::c_double = 0.;
    let mut pr0: libc::c_double = 0.;
    let mut pt1: libc::c_double = 0.;
    let mut pr1: libc::c_double = 0.;
    let mut pt2: libc::c_double = 0.;
    let mut pr2: libc::c_double = 0.;
    pt2 = 0.0f64;
    pt0 = numdf as libc::c_double / dendf as libc::c_double;
    pr0 = probf(pt0, numdf, dendf);
    pt1 = 2.0f64 * pt0;
    pr2 = 1.0f64;
    while fabs(pr2 - p) > dap_prtol {
        pr1 = probf(pt1, numdf, dendf);
        pt2 = pt0 + (pt1 - pt0) * (p - pr0) / (pr1 - pr0);
        if pt2 < 0.0f64 {
            if pt0 < pt1 {
                pt2 = 0.5f64 * pt0;
            } else {
                pt2 = 0.5f64 * pt1;
            }
        }
        pr2 = probf(pt2, numdf, dendf);
        if fabs(pr2 - pr0) < fabs(pr2 - pr1) {
            pr1 = pr2;
            pt1 = pt2;
        } else {
            pr0 = pr2;
            pt0 = pt2;
        }
    }
    return pt2;
}
static mut randmax: libc::c_double = 2147483647.0f64;
pub unsafe extern "C" fn varnorm() -> libc::c_double {
    let mut u1: libc::c_double = 0.;
    let mut u2: libc::c_double = 0.;
    let mut v1: libc::c_double = 0.;
    let mut v2: libc::c_double = 0.;
    let mut w_0: libc::c_double = 0.;
    loop {
        u1 = 2.0f64 * random() as libc::c_double / randmax - 1.0f64;
        v1 = u1 * u1;
        u2 = 2.0f64 * random() as libc::c_double / randmax - 1.0f64;
        v2 = u2 * u2;
        w_0 = v1 + v2;
        if !(w_0 > 1.0f64) {
            break;
        }
    }
    w_0 = sqrt(-2.0f64 * log(w_0) / w_0);
    return u1 * w_0;
}
pub unsafe extern "C" fn varunif() -> libc::c_double {
    return random() as libc::c_double / randmax;
}
pub unsafe extern "C" fn probz(mut z: libc::c_double) -> libc::c_double {
    if finite(z) == 0 {
        return 0.0f64 / 0.0f64;
    }
    z *= 0.707106781186547524401f64;
    if z < -0.58f64 {
        return 0.5f64 * erfc(-z);
    }
    if z < 0.0f64 {
        return 0.5f64 * (1.0f64 - erf(-z));
    }
    if z < 0.58f64 {
        return 0.5f64 * (1.0f64 + erf(z));
    }
    return 1.0f64 - 0.5f64 * erfc(z);
}
pub unsafe extern "C" fn probchisq(
    mut c: libc::c_double,
    mut df: libc::c_int,
) -> libc::c_double {
    let mut ddf: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    if df < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(probchisq) non-positive df = %d\n\0" as *const u8 as *const libc::c_char,
            df,
        );
        exit(1 as libc::c_int);
    }
    if finite(c) == 0 {
        return 0.0f64 / 0.0f64;
    }
    match df {
        1 => return 2.0f64 * probz(-sqrt(c)),
        2 => return exp(-0.5f64 * c),
        _ => {
            ddf = df as libc::c_double;
            tmp = (0.5f64 * ddf - 1.0f64) * log(0.5f64 * c) - 0.5f64 * c
                - lgamma(0.5f64 * ddf);
            if finite(tmp) != 0 {
                return exp(tmp) + probchisq(c, df - 2 as libc::c_int);
            }
            return 0.0f64;
        }
    };
}
pub unsafe extern "C" fn chisqpoint(
    mut p: libc::c_double,
    mut df: libc::c_int,
) -> libc::c_double {
    let mut pt0: libc::c_double = 0.;
    let mut pr0: libc::c_double = 0.;
    let mut pt1: libc::c_double = 0.;
    let mut pr1: libc::c_double = 0.;
    let mut pt2: libc::c_double = 0.;
    let mut pr2: libc::c_double = 0.;
    pt2 = 0.0f64;
    pt0 = df as libc::c_double;
    pr0 = probchisq(pt0, df);
    pt1 = 2.0f64 * pt0;
    pr2 = 1.0f64;
    while fabs(pr2 - p) > dap_prtol {
        pr1 = probchisq(pt1, df);
        pt2 = pt0 + (pt1 - pt0) * (p - pr0) / (pr1 - pr0);
        if pt2 < 0.0f64 {
            if pt0 < pt1 {
                pt2 = 0.5f64 * pt0;
            } else {
                pt2 = 0.5f64 * pt1;
            }
        }
        pr2 = probchisq(pt2, df);
        if fabs(pr2 - pr0) < fabs(pr2 - pr1) {
            pr1 = pr2;
            pt1 = pt2;
        } else {
            pr0 = pr2;
            pt0 = pt2;
        }
    }
    return pt2;
}
static mut w: libc::c_double = 0.;
static mut numdfm1: libc::c_double = 0.;
static mut dendfm1: libc::c_double = 0.;
static mut dnumdf: libc::c_double = 0.;
static mut ddendf: libc::c_double = 0.;
static mut pt: libc::c_double = 0.;
pub unsafe extern "C" fn rangef1(mut x0: libc::c_double) -> libc::c_double {
    let mut diff: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    if x0 == -1.0f64 {
        return 0.0f64;
    }
    x1 = 1.0f64 + x0;
    x = x0 / x1;
    x2 = x1 * x1;
    diff = probz(x + w) - probz(x);
    if diff / x2 < 1.0e-16f64 {
        return 0.0f64;
    }
    tmp = -0.5f64 * x * x + numdfm1 * log(diff);
    if finite(tmp) != 0 {
        return exp(tmp) / (x1 * x1);
    }
    return 0.0f64;
}
pub unsafe extern "C" fn rangef2(mut x0: libc::c_double) -> libc::c_double {
    let mut diff: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    if x0 == 1.0f64 {
        return 0.0f64;
    }
    x1 = 1.0f64 - x0;
    x = x0 / x1;
    x2 = x1 * x1;
    diff = probz(x + w) - probz(x);
    if diff / x2 < 1.0e-16f64 {
        return 0.0f64;
    }
    tmp = -0.5f64 * x * x + numdfm1 * log(diff);
    if finite(tmp) != 0 {
        return exp(tmp) / (x1 * x1);
    }
    return 0.0f64;
}
unsafe extern "C" fn range(mut w0: libc::c_double) -> libc::c_double {
    w = w0;
    return 1.0f64
        - dnumdf * 0.398942280401432677940f64
            * (dap_simp(
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    Option::<unsafe extern "C" fn() -> libc::c_double>,
                >(
                    Some(
                        rangef1 as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                    ),
                ),
                -1.0f64,
                0.0f64,
                32 as libc::c_int,
            )
                + dap_simp(
                    ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                        Option::<unsafe extern "C" fn() -> libc::c_double>,
                    >(
                        Some(
                            rangef2
                                as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                        ),
                    ),
                    0.0f64,
                    1.0f64,
                    32 as libc::c_int,
                ));
}
unsafe extern "C" fn sturf(mut s0: libc::c_double) -> libc::c_double {
    let mut s: libc::c_double = 0.;
    let mut s1: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    if s0 == 1.0f64 {
        return 0.0f64;
    }
    s1 = 1.0f64 - s0;
    s = s0 / s1;
    if dendfm1 == 0.0f64 {
        return exp(-0.5f64 * s * s / 2.7182818284590450908f64)
            * range(pt * s * 0.60653065971263343973f64) / (s1 * s1);
    }
    tmp = dendfm1 * log(s) - 0.5f64 * ddendf * s * s / 2.7182818284590450908f64;
    if finite(tmp) != 0 {
        return exp(tmp) * range(pt * s * 0.60653065971263343973f64) / (s1 * s1);
    }
    return 0.0f64;
}
pub unsafe extern "C" fn dap_sr(
    mut numdf: libc::c_int,
    mut dendf: libc::c_int,
    mut pt0: libc::c_double,
) -> libc::c_double {
    let mut c: libc::c_double = 0.;
    let mut dn: libc::c_double = 0.;
    let mut dn1: libc::c_double = 0.;
    pt = pt0;
    dnumdf = numdf as libc::c_double;
    numdfm1 = dnumdf - 1 as libc::c_int as libc::c_double;
    ddendf = dendf as libc::c_double;
    dendfm1 = (dendf - 1 as libc::c_int) as libc::c_double;
    dn = 0.5f64 * ddendf;
    dn1 = dn * 0.36787944117144234115f64;
    c = 2.0f64;
    while dn > 1.0f64 {
        c *= dn1 / (dn - 1.0f64);
        dn -= 1.0f64;
    }
    if dn == 0.5f64 {
        c *= sqrt(dn1) / 1.772453850905516027297f64;
    } else {
        c *= dn1;
    }
    return c
        * dap_simp(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                Option::<unsafe extern "C" fn() -> libc::c_double>,
            >(Some(sturf as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
            0.0f64,
            1.0f64,
            64 as libc::c_int,
        );
}
pub unsafe extern "C" fn dap_srpt(
    mut numdf: libc::c_int,
    mut dendf: libc::c_int,
    mut pt0: libc::c_double,
    mut pr0: libc::c_double,
    mut alpha: libc::c_double,
) -> libc::c_double {
    let mut pt1: libc::c_double = 0.;
    let mut pr1: libc::c_double = 0.;
    let mut pt2: libc::c_double = 0.;
    let mut pr2: libc::c_double = 0.;
    pt2 = 0.0f64;
    if alpha < pr0 {
        pt1 = 2.0f64 * pt0;
    } else {
        pt1 = 0.5f64 * pt0;
    }
    pr2 = 1.0f64;
    while fabs(pr2 - alpha) > dap_prtol {
        pr1 = dap_sr(numdf, dendf, pt1);
        pt2 = pt0 + (pt1 - pt0) * (alpha - pr0) / (pr1 - pr0);
        if pt2 < 0.0f64 {
            if pt0 < pt1 {
                pt2 = 0.5f64 * pt0;
            } else {
                pt2 = 0.5f64 * pt1;
            }
        }
        pr2 = dap_sr(numdf, dendf, pt2);
        if fabs(pr2 - pr0) < fabs(pr2 - pr1) {
            pr1 = pr2;
            pt1 = pt2;
        } else {
            pr0 = pr2;
            pt0 = pt2;
        }
    }
    return pt2;
}
pub unsafe extern "C" fn maxdf1(mut x0: libc::c_double) -> libc::c_double {
    let mut diff: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    if x0 == -1.0f64 {
        return 0.0f64;
    }
    x1 = 1.0f64 + x0;
    x = x0 / x1;
    x2 = x1 * x1;
    diff = probz(x + w) - probz(x - w);
    if diff / x2 < 1.0e-16f64 {
        return 0.0f64;
    }
    return exp(-0.5f64 * x * x + dnumdf * log(diff)) / (x1 * x1);
}
pub unsafe extern "C" fn maxdf2(mut x0: libc::c_double) -> libc::c_double {
    let mut diff: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    if x0 == 1.0f64 {
        return 0.0f64;
    }
    x1 = 1.0f64 - x0;
    x = x0 / x1;
    x2 = x1 * x1;
    diff = probz(x + w) - probz(x - w);
    if diff / x2 < 1.0e-16f64 {
        return 0.0f64;
    }
    return exp(-0.5f64 * x * x + dnumdf * log(diff)) / (x1 * x1);
}
unsafe extern "C" fn maxdf(mut w0: libc::c_double) -> libc::c_double {
    w = w0;
    return 1.0f64
        - 0.398942280401432677940f64
            * (dap_simp(
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                    Option::<unsafe extern "C" fn() -> libc::c_double>,
                >(
                    Some(
                        maxdf1 as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                    ),
                ),
                -1.0f64,
                0.0f64,
                32 as libc::c_int,
            )
                + dap_simp(
                    ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                        Option::<unsafe extern "C" fn() -> libc::c_double>,
                    >(
                        Some(
                            maxdf2
                                as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
                        ),
                    ),
                    0.0f64,
                    1.0f64,
                    32 as libc::c_int,
                ));
}
unsafe extern "C" fn maxdiff(mut s0: libc::c_double) -> libc::c_double {
    let mut s: libc::c_double = 0.;
    let mut s1: libc::c_double = 0.;
    if s0 == 1.0f64 {
        return 0.0f64;
    }
    s1 = 1.0f64 - s0;
    s = s0 / s1;
    if dendfm1 == 0.0f64 {
        return exp(-0.5f64 * s * s / 2.7182818284590450908f64)
            * maxdf(pt * s * 0.60653065971263343973f64) / (s1 * s1);
    }
    return exp(dendfm1 * log(s) - 0.5f64 * ddendf * s * s / 2.7182818284590450908f64)
        * maxdf(pt * s * 0.60653065971263343973f64) / (s1 * s1);
}
pub unsafe extern "C" fn dap_md(
    mut numdf: libc::c_int,
    mut dendf: libc::c_int,
    mut pt0: libc::c_double,
) -> libc::c_double {
    let mut c: libc::c_double = 0.;
    let mut dn: libc::c_double = 0.;
    let mut dn1: libc::c_double = 0.;
    pt = pt0;
    dnumdf = numdf as libc::c_double;
    numdfm1 = dnumdf - 1 as libc::c_int as libc::c_double;
    ddendf = dendf as libc::c_double;
    dendfm1 = (dendf - 1 as libc::c_int) as libc::c_double;
    dn = 0.5f64 * ddendf;
    dn1 = dn * 0.36787944117144234115f64;
    c = 2.0f64;
    while dn > 1.0f64 {
        c *= dn1 / (dn - 1.0f64);
        dn -= 1.0f64;
    }
    if dn == 0.5f64 {
        c *= sqrt(dn1) / 1.772453850905516027297f64;
    } else {
        c *= dn1;
    }
    return c
        * dap_simp(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
                Option::<unsafe extern "C" fn() -> libc::c_double>,
            >(Some(maxdiff as unsafe extern "C" fn(libc::c_double) -> libc::c_double)),
            0.0f64,
            1.0f64,
            256 as libc::c_int,
        );
}
pub unsafe extern "C" fn dap_mdpt(
    mut numdf: libc::c_int,
    mut dendf: libc::c_int,
    mut pt0: libc::c_double,
    mut pr0: libc::c_double,
    mut alpha: libc::c_double,
) -> libc::c_double {
    let mut pt1: libc::c_double = 0.;
    let mut pr1: libc::c_double = 0.;
    let mut pt2: libc::c_double = 0.;
    let mut pr2: libc::c_double = 0.;
    pt2 = 0.0f64;
    if alpha < pr0 {
        pt1 = 2.0f64 * pt0;
    } else {
        pt1 = 0.5f64 * pt0;
    }
    pr2 = 1.0f64;
    while fabs(pr2 - alpha) > dap_prtol {
        pr1 = dap_md(numdf, dendf, pt1);
        pt2 = pt0 + (pt1 - pt0) * (alpha - pr0) / (pr1 - pr0);
        if pt2 < 0.0f64 {
            if pt0 < pt1 {
                pt2 = 0.5f64 * pt0;
            } else {
                pt2 = 0.5f64 * pt1;
            }
        }
        pr2 = dap_md(numdf, dendf, pt2);
        if fabs(pr2 - pr0) < fabs(pr2 - pr1) {
            pr1 = pr2;
            pt1 = pt2;
        } else {
            pr0 = pr2;
            pt0 = pt2;
        }
    }
    return pt2;
}
