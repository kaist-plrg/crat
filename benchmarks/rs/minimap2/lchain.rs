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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn radix_sort_128x(beg: *mut mm128_t, end: *mut mm128_t);
    fn krealloc(
        km: *mut libc::c_void,
        ptr: *mut libc::c_void,
        size: size_t,
    ) -> *mut libc::c_void;
    fn kmalloc(km: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn kcalloc(km: *mut libc::c_void, count: size_t, size: size_t) -> *mut libc::c_void;
    fn kfree(km: *mut libc::c_void, ptr: *mut libc::c_void);
    fn km_init2(km_par: *mut libc::c_void, min_core_size: size_t) -> *mut libc::c_void;
    fn km_destroy(km: *mut libc::c_void);
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm128_t {
    pub x: uint64_t,
    pub y: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub f: libc::c_float,
    pub i: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub p: [*mut lc_elem_s; 2],
    pub s: *mut lc_elem_s,
    pub balance: libc::c_schar,
    pub size: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lc_elem_s {
    pub y: int32_t,
    pub i: int64_t,
    pub pri: libc::c_double,
    pub head: C2RustUnnamed_0,
}
pub type lc_elem_t = lc_elem_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct krmq_itr_lc_elem {
    pub stack: [*const lc_elem_t; 64],
    pub top: *mut *const lc_elem_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmp_rmq_t {
    pub cnt: size_t,
    pub n: size_t,
    pub max: size_t,
    pub buf: *mut *mut lc_elem_t,
    pub km: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn mg_log2(mut x: libc::c_float) -> libc::c_float {
    let mut z: C2RustUnnamed = C2RustUnnamed { f: x };
    let mut log_2: libc::c_float = (z.i >> 23 as libc::c_int
        & 255 as libc::c_int as libc::c_uint)
        .wrapping_sub(128 as libc::c_int as libc::c_uint) as libc::c_float;
    z.i &= !((255 as libc::c_int) << 23 as libc::c_int) as libc::c_uint;
    z
        .i = (z.i as libc::c_uint)
        .wrapping_add(((127 as libc::c_int) << 23 as libc::c_int) as libc::c_uint)
        as uint32_t as uint32_t;
    log_2 += (-0.34484843f32 * z.f + 2.02466578f32) * z.f - 0.67487759f32;
    return log_2;
}
unsafe extern "C" fn mg_chain_bk_end(
    mut max_drop: int32_t,
    mut z: *const mm128_t,
    mut f: *const int32_t,
    mut p: *const int64_t,
    mut t: *mut int32_t,
    mut k: int64_t,
) -> int64_t {
    let mut i: int64_t = (*z.offset(k as isize)).y as int64_t;
    let mut end_i: int64_t = -(1 as libc::c_int) as int64_t;
    let mut max_i: int64_t = i;
    let mut max_s: int32_t = 0 as libc::c_int;
    if i < 0 as libc::c_int as libc::c_long || *t.offset(i as isize) != 0 as libc::c_int
    {
        return i;
    }
    loop {
        let mut s: int32_t = 0;
        *t.offset(i as isize) = 2 as libc::c_int;
        i = *p.offset(i as isize);
        end_i = i;
        s = (if i < 0 as libc::c_int as libc::c_long {
            (*z.offset(k as isize)).x
        } else {
            ((*z.offset(k as isize)).x as int32_t - *f.offset(i as isize))
                as libc::c_ulong
        }) as int32_t;
        if s > max_s {
            max_s = s;
            max_i = i;
        } else if max_s - s > max_drop {
            break;
        }
        if !(i >= 0 as libc::c_int as libc::c_long
            && *t.offset(i as isize) == 0 as libc::c_int)
        {
            break;
        }
    }
    i = (*z.offset(k as isize)).y as int64_t;
    while i >= 0 as libc::c_int as libc::c_long && i != end_i {
        *t.offset(i as isize) = 0 as libc::c_int;
        i = *p.offset(i as isize);
    }
    return max_i;
}
pub unsafe extern "C" fn mg_chain_backtrack(
    mut km: *mut libc::c_void,
    mut n: int64_t,
    mut f: *const int32_t,
    mut p: *const int64_t,
    mut v: *mut int32_t,
    mut t: *mut int32_t,
    mut min_cnt: int32_t,
    mut min_sc: int32_t,
    mut max_drop: int32_t,
    mut n_u_: *mut int32_t,
    mut n_v_: *mut int32_t,
) -> *mut uint64_t {
    let mut z: *mut mm128_t = 0 as *mut mm128_t;
    let mut u: *mut uint64_t = 0 as *mut uint64_t;
    let mut i: int64_t = 0;
    let mut k: int64_t = 0;
    let mut n_z: int64_t = 0;
    let mut n_v: int64_t = 0;
    let mut n_u: int32_t = 0;
    *n_v_ = 0 as libc::c_int;
    *n_u_ = *n_v_;
    i = 0 as libc::c_int as int64_t;
    n_z = 0 as libc::c_int as int64_t;
    while i < n {
        if *f.offset(i as isize) >= min_sc {
            n_z += 1;
            n_z;
        }
        i += 1;
        i;
    }
    if n_z == 0 as libc::c_int as libc::c_long {
        return 0 as *mut uint64_t;
    }
    z = kmalloc(
        km,
        (n_z as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
    ) as *mut mm128_t;
    i = 0 as libc::c_int as int64_t;
    k = 0 as libc::c_int as int64_t;
    while i < n {
        if *f.offset(i as isize) >= min_sc {
            (*z.offset(k as isize)).x = *f.offset(i as isize) as uint64_t;
            let fresh0 = k;
            k = k + 1;
            (*z.offset(fresh0 as isize)).y = i as uint64_t;
        }
        i += 1;
        i;
    }
    radix_sort_128x(z, z.offset(n_z as isize));
    memset(
        t as *mut libc::c_void,
        0 as libc::c_int,
        (n * 4 as libc::c_int as libc::c_long) as libc::c_ulong,
    );
    k = n_z - 1 as libc::c_int as libc::c_long;
    n_u = 0 as libc::c_int;
    n_v = n_u as int64_t;
    while k >= 0 as libc::c_int as libc::c_long {
        if *t.offset((*z.offset(k as isize)).y as isize) == 0 as libc::c_int {
            let mut n_v0: int64_t = n_v;
            let mut end_i: int64_t = 0;
            let mut sc: int32_t = 0;
            end_i = mg_chain_bk_end(max_drop, z, f, p, t, k);
            i = (*z.offset(k as isize)).y as int64_t;
            while i != end_i {
                n_v += 1;
                n_v;
                *t.offset(i as isize) = 1 as libc::c_int;
                i = *p.offset(i as isize);
            }
            sc = (if i < 0 as libc::c_int as libc::c_long {
                (*z.offset(k as isize)).x
            } else {
                ((*z.offset(k as isize)).x as int32_t - *f.offset(i as isize))
                    as libc::c_ulong
            }) as int32_t;
            if sc >= min_sc && n_v > n_v0 && n_v - n_v0 >= min_cnt as libc::c_long {
                n_u += 1;
                n_u;
            } else {
                n_v = n_v0;
            }
        }
        k -= 1;
        k;
    }
    u = kmalloc(
        km,
        (n_u as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
    ) as *mut uint64_t;
    memset(
        t as *mut libc::c_void,
        0 as libc::c_int,
        (n * 4 as libc::c_int as libc::c_long) as libc::c_ulong,
    );
    k = n_z - 1 as libc::c_int as libc::c_long;
    n_u = 0 as libc::c_int;
    n_v = n_u as int64_t;
    while k >= 0 as libc::c_int as libc::c_long {
        if *t.offset((*z.offset(k as isize)).y as isize) == 0 as libc::c_int {
            let mut n_v0_0: int64_t = n_v;
            let mut end_i_0: int64_t = 0;
            let mut sc_0: int32_t = 0;
            end_i_0 = mg_chain_bk_end(max_drop, z, f, p, t, k);
            i = (*z.offset(k as isize)).y as int64_t;
            while i != end_i_0 {
                let fresh1 = n_v;
                n_v = n_v + 1;
                *v.offset(fresh1 as isize) = i as int32_t;
                *t.offset(i as isize) = 1 as libc::c_int;
                i = *p.offset(i as isize);
            }
            sc_0 = (if i < 0 as libc::c_int as libc::c_long {
                (*z.offset(k as isize)).x
            } else {
                ((*z.offset(k as isize)).x as int32_t - *f.offset(i as isize))
                    as libc::c_ulong
            }) as int32_t;
            if sc_0 >= min_sc && n_v > n_v0_0 && n_v - n_v0_0 >= min_cnt as libc::c_long
            {
                let fresh2 = n_u;
                n_u = n_u + 1;
                *u
                    .offset(
                        fresh2 as isize,
                    ) = (sc_0 as uint64_t) << 32 as libc::c_int
                    | (n_v - n_v0_0) as libc::c_ulong;
            } else {
                n_v = n_v0_0;
            }
        }
        k -= 1;
        k;
    }
    kfree(km, z as *mut libc::c_void);
    if n_v < 2147483647 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"n_v < INT32_MAX\0" as *const u8 as *const libc::c_char,
            b"lchain.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 151],
                &[libc::c_char; 151],
            >(
                b"uint64_t *mg_chain_backtrack(void *, int64_t, const int32_t *, const int64_t *, int32_t *, int32_t *, int32_t, int32_t, int32_t, int32_t *, int32_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3969: {
        if n_v < 2147483647 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"n_v < INT32_MAX\0" as *const u8 as *const libc::c_char,
                b"lchain.c\0" as *const u8 as *const libc::c_char,
                73 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 151],
                    &[libc::c_char; 151],
                >(
                    b"uint64_t *mg_chain_backtrack(void *, int64_t, const int32_t *, const int64_t *, int32_t *, int32_t *, int32_t, int32_t, int32_t, int32_t *, int32_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    *n_u_ = n_u;
    *n_v_ = n_v as int32_t;
    return u;
}
unsafe extern "C" fn compact_a(
    mut km: *mut libc::c_void,
    mut n_u: int32_t,
    mut u: *mut uint64_t,
    mut n_v: int32_t,
    mut v: *mut int32_t,
    mut a: *mut mm128_t,
) -> *mut mm128_t {
    let mut b: *mut mm128_t = 0 as *mut mm128_t;
    let mut w: *mut mm128_t = 0 as *mut mm128_t;
    let mut u2: *mut uint64_t = 0 as *mut uint64_t;
    let mut i: int64_t = 0;
    let mut j: int64_t = 0;
    let mut k: int64_t = 0;
    b = kmalloc(
        km,
        (n_v as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
    ) as *mut mm128_t;
    i = 0 as libc::c_int as int64_t;
    k = 0 as libc::c_int as int64_t;
    while i < n_u as libc::c_long {
        let mut k0: int32_t = k as int32_t;
        let mut ni: int32_t = *u.offset(i as isize) as int32_t;
        j = 0 as libc::c_int as int64_t;
        while j < ni as libc::c_long {
            let fresh3 = k;
            k = k + 1;
            *b
                .offset(
                    fresh3 as isize,
                ) = *a
                .offset(
                    *v
                        .offset(
                            (k0 as libc::c_long
                                + (ni as libc::c_long - j
                                    - 1 as libc::c_int as libc::c_long)) as isize,
                        ) as isize,
                );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    kfree(km, v as *mut libc::c_void);
    w = kmalloc(
        km,
        (n_u as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
    ) as *mut mm128_t;
    k = 0 as libc::c_int as int64_t;
    i = k;
    while i < n_u as libc::c_long {
        (*w.offset(i as isize)).x = (*b.offset(k as isize)).x;
        (*w.offset(i as isize))
            .y = (k as uint64_t) << 32 as libc::c_int | i as libc::c_ulong;
        k += *u.offset(i as isize) as int32_t as libc::c_long;
        i += 1;
        i;
    }
    radix_sort_128x(w, w.offset(n_u as isize));
    u2 = kmalloc(
        km,
        (n_u as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
    ) as *mut uint64_t;
    k = 0 as libc::c_int as int64_t;
    i = k;
    while i < n_u as libc::c_long {
        let mut j_0: int32_t = (*w.offset(i as isize)).y as int32_t;
        let mut n: int32_t = *u.offset(j_0 as isize) as int32_t;
        *u2.offset(i as isize) = *u.offset(j_0 as isize);
        memcpy(
            &mut *a.offset(k as isize) as *mut mm128_t as *mut libc::c_void,
            &mut *b.offset(((*w.offset(i as isize)).y >> 32 as libc::c_int) as isize)
                as *mut mm128_t as *const libc::c_void,
            (n as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
        );
        k += n as libc::c_long;
        i += 1;
        i;
    }
    memcpy(
        u as *mut libc::c_void,
        u2 as *const libc::c_void,
        (n_u * 8 as libc::c_int) as libc::c_ulong,
    );
    memcpy(
        b as *mut libc::c_void,
        a as *const libc::c_void,
        (k as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
    );
    kfree(km, a as *mut libc::c_void);
    kfree(km, w as *mut libc::c_void);
    kfree(km, u2 as *mut libc::c_void);
    return b;
}
#[inline]
unsafe extern "C" fn comput_sc(
    mut ai: *const mm128_t,
    mut aj: *const mm128_t,
    mut max_dist_x: int32_t,
    mut max_dist_y: int32_t,
    mut bw: int32_t,
    mut chn_pen_gap: libc::c_float,
    mut chn_pen_skip: libc::c_float,
    mut is_cdna: libc::c_int,
    mut n_seg: libc::c_int,
) -> int32_t {
    let mut dq: int32_t = (*ai).y as int32_t - (*aj).y as int32_t;
    let mut dr: int32_t = 0;
    let mut dd: int32_t = 0;
    let mut dg: int32_t = 0;
    let mut q_span: int32_t = 0;
    let mut sc: int32_t = 0;
    let mut sidi: int32_t = (((*ai).y as libc::c_ulonglong
        & (0xff as libc::c_ulonglong) << 48 as libc::c_int) >> 48 as libc::c_int)
        as int32_t;
    let mut sidj: int32_t = (((*aj).y as libc::c_ulonglong
        & (0xff as libc::c_ulonglong) << 48 as libc::c_int) >> 48 as libc::c_int)
        as int32_t;
    if dq <= 0 as libc::c_int || dq > max_dist_x {
        return -(2147483647 as libc::c_int) - 1 as libc::c_int;
    }
    dr = ((*ai).x).wrapping_sub((*aj).x) as int32_t;
    if sidi == sidj && (dr == 0 as libc::c_int || dq > max_dist_y) {
        return -(2147483647 as libc::c_int) - 1 as libc::c_int;
    }
    dd = if dr > dq { dr - dq } else { dq - dr };
    if sidi == sidj && dd > bw {
        return -(2147483647 as libc::c_int) - 1 as libc::c_int;
    }
    if n_seg > 1 as libc::c_int && is_cdna == 0 && sidi == sidj && dr > max_dist_y {
        return -(2147483647 as libc::c_int) - 1 as libc::c_int;
    }
    dg = if dr < dq { dr } else { dq };
    q_span = ((*aj).y >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as int32_t;
    sc = if q_span < dg { q_span } else { dg };
    if dd != 0 || dg > q_span {
        let mut lin_pen: libc::c_float = 0.;
        let mut log_pen: libc::c_float = 0.;
        lin_pen = chn_pen_gap * dd as libc::c_float + chn_pen_skip * dg as libc::c_float;
        log_pen = if dd >= 1 as libc::c_int {
            mg_log2((dd + 1 as libc::c_int) as libc::c_float)
        } else {
            0.0f32
        };
        if is_cdna != 0 || sidi != sidj {
            if sidi != sidj && dr == 0 as libc::c_int {
                sc += 1;
                sc;
            } else if dr > dq || sidi != sidj {
                sc -= (if lin_pen < log_pen { lin_pen } else { log_pen }) as libc::c_int;
            } else {
                sc -= (lin_pen + 0.5f32 * log_pen) as libc::c_int;
            }
        } else {
            sc -= (lin_pen + 0.5f32 * log_pen) as libc::c_int;
        }
    }
    return sc;
}
pub unsafe extern "C" fn mg_lchain_dp(
    mut max_dist_x: libc::c_int,
    mut max_dist_y: libc::c_int,
    mut bw: libc::c_int,
    mut max_skip: libc::c_int,
    mut max_iter: libc::c_int,
    mut min_cnt: libc::c_int,
    mut min_sc: libc::c_int,
    mut chn_pen_gap: libc::c_float,
    mut chn_pen_skip: libc::c_float,
    mut is_cdna: libc::c_int,
    mut n_seg: libc::c_int,
    mut n: int64_t,
    mut a: *mut mm128_t,
    mut n_u_: *mut libc::c_int,
    mut _u: *mut *mut uint64_t,
    mut km: *mut libc::c_void,
) -> *mut mm128_t {
    let mut f: *mut int32_t = 0 as *mut int32_t;
    let mut t: *mut int32_t = 0 as *mut int32_t;
    let mut v: *mut int32_t = 0 as *mut int32_t;
    let mut n_u: int32_t = 0;
    let mut n_v: int32_t = 0;
    let mut mmax_f: int32_t = 0 as libc::c_int;
    let mut max_drop: int32_t = bw;
    let mut p: *mut int64_t = 0 as *mut int64_t;
    let mut i: int64_t = 0;
    let mut j: int64_t = 0;
    let mut max_ii: int64_t = 0;
    let mut st: int64_t = 0 as libc::c_int as int64_t;
    let mut n_iter: int64_t = 0 as libc::c_int as int64_t;
    let mut u: *mut uint64_t = 0 as *mut uint64_t;
    if !_u.is_null() {
        *_u = 0 as *mut uint64_t;
        *n_u_ = 0 as libc::c_int;
    }
    if n == 0 as libc::c_int as libc::c_long || a.is_null() {
        kfree(km, a as *mut libc::c_void);
        return 0 as *mut mm128_t;
    }
    if max_dist_x < bw {
        max_dist_x = bw;
    }
    if max_dist_y < bw && is_cdna == 0 {
        max_dist_y = bw;
    }
    if is_cdna != 0 {
        max_drop = 2147483647 as libc::c_int;
    }
    p = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int64_t>() as libc::c_ulong),
    ) as *mut int64_t;
    f = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
    ) as *mut int32_t;
    v = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
    ) as *mut int32_t;
    t = kcalloc(km, n as size_t, ::std::mem::size_of::<int32_t>() as libc::c_ulong)
        as *mut int32_t;
    i = 0 as libc::c_int as int64_t;
    max_ii = -(1 as libc::c_int) as int64_t;
    while i < n {
        let mut max_j: int64_t = -(1 as libc::c_int) as int64_t;
        let mut end_j: int64_t = 0;
        let mut max_f: int32_t = ((*a.offset(i as isize)).y >> 32 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as int32_t;
        let mut n_skip: int32_t = 0 as libc::c_int;
        while st < i
            && ((*a.offset(i as isize)).x >> 32 as libc::c_int
                != (*a.offset(st as isize)).x >> 32 as libc::c_int
                || (*a.offset(i as isize)).x
                    > ((*a.offset(st as isize)).x)
                        .wrapping_add(max_dist_x as libc::c_ulong))
        {
            st += 1;
            st;
        }
        if i - st > max_iter as libc::c_long {
            st = i - max_iter as libc::c_long;
        }
        j = i - 1 as libc::c_int as libc::c_long;
        while j >= st {
            let mut sc: int32_t = 0;
            sc = comput_sc(
                &mut *a.offset(i as isize),
                &mut *a.offset(j as isize),
                max_dist_x,
                max_dist_y,
                bw,
                chn_pen_gap,
                chn_pen_skip,
                is_cdna,
                n_seg,
            );
            n_iter += 1;
            n_iter;
            if !(sc == -(2147483647 as libc::c_int) - 1 as libc::c_int) {
                sc += *f.offset(j as isize);
                if sc > max_f {
                    max_f = sc;
                    max_j = j;
                    if n_skip > 0 as libc::c_int {
                        n_skip -= 1;
                        n_skip;
                    }
                } else if *t.offset(j as isize) == i as int32_t {
                    n_skip += 1;
                    if n_skip > max_skip {
                        break;
                    }
                }
                if *p.offset(j as isize) >= 0 as libc::c_int as libc::c_long {
                    *t.offset(*p.offset(j as isize) as isize) = i as int32_t;
                }
            }
            j -= 1;
            j;
        }
        end_j = j;
        if max_ii < 0 as libc::c_int as libc::c_long
            || ((*a.offset(i as isize)).x).wrapping_sub((*a.offset(max_ii as isize)).x)
                > max_dist_x as int64_t as libc::c_ulong
        {
            let mut max: int32_t = -(2147483647 as libc::c_int) - 1 as libc::c_int;
            max_ii = -(1 as libc::c_int) as int64_t;
            j = i - 1 as libc::c_int as libc::c_long;
            while j >= st {
                if max < *f.offset(j as isize) {
                    max = *f.offset(j as isize);
                    max_ii = j;
                }
                j -= 1;
                j;
            }
        }
        if max_ii >= 0 as libc::c_int as libc::c_long && max_ii < end_j {
            let mut tmp: int32_t = 0;
            tmp = comput_sc(
                &mut *a.offset(i as isize),
                &mut *a.offset(max_ii as isize),
                max_dist_x,
                max_dist_y,
                bw,
                chn_pen_gap,
                chn_pen_skip,
                is_cdna,
                n_seg,
            );
            if tmp != -(2147483647 as libc::c_int) - 1 as libc::c_int
                && max_f < tmp + *f.offset(max_ii as isize)
            {
                max_f = tmp + *f.offset(max_ii as isize);
                max_j = max_ii;
            }
        }
        *f.offset(i as isize) = max_f;
        *p.offset(i as isize) = max_j;
        *v
            .offset(
                i as isize,
            ) = if max_j >= 0 as libc::c_int as libc::c_long
            && *v.offset(max_j as isize) > max_f
        {
            *v.offset(max_j as isize)
        } else {
            max_f
        };
        if max_ii < 0 as libc::c_int as libc::c_long
            || ((*a.offset(i as isize)).x).wrapping_sub((*a.offset(max_ii as isize)).x)
                <= max_dist_x as int64_t as libc::c_ulong
                && *f.offset(max_ii as isize) < *f.offset(i as isize)
        {
            max_ii = i;
        }
        if mmax_f < max_f {
            mmax_f = max_f;
        }
        i += 1;
        i;
    }
    u = mg_chain_backtrack(
        km,
        n,
        f,
        p,
        v,
        t,
        min_cnt,
        min_sc,
        max_drop,
        &mut n_u,
        &mut n_v,
    );
    *n_u_ = n_u;
    *_u = u;
    kfree(km, p as *mut libc::c_void);
    kfree(km, f as *mut libc::c_void);
    kfree(km, t as *mut libc::c_void);
    if n_u == 0 as libc::c_int {
        kfree(km, a as *mut libc::c_void);
        kfree(km, v as *mut libc::c_void);
        return 0 as *mut mm128_t;
    }
    return compact_a(km, n_u, u, n_v, v, a);
}
#[inline]
unsafe extern "C" fn krmq_rotate2_lc_elem(
    mut p: *mut lc_elem_t,
    mut dir: libc::c_int,
) -> *mut lc_elem_t {
    let mut b1: libc::c_int = 0;
    let mut opp: libc::c_int = 1 as libc::c_int - dir;
    let mut q: *mut lc_elem_t = (*p).head.p[opp as usize];
    let mut r: *mut lc_elem_t = (*q).head.p[dir as usize];
    let mut s: *mut lc_elem_t = (*p).head.s;
    let mut size_x_dir: libc::c_uint = if !((*r).head.p[dir as usize]).is_null() {
        (*(*r).head.p[dir as usize]).head.size
    } else {
        0 as libc::c_int as libc::c_uint
    };
    (*r).head.size = (*p).head.size;
    (*p)
        .head
        .size = ((*p).head.size).wrapping_sub(((*q).head.size).wrapping_sub(size_x_dir));
    (*q)
        .head
        .size = ((*q).head.size)
        .wrapping_sub(size_x_dir.wrapping_add(1 as libc::c_int as libc::c_uint));
    krmq_update_min_lc_elem(p, (*p).head.p[dir as usize], (*r).head.p[dir as usize]);
    krmq_update_min_lc_elem(q, (*q).head.p[opp as usize], (*r).head.p[opp as usize]);
    (*r).head.s = s;
    (*p).head.p[opp as usize] = (*r).head.p[dir as usize];
    (*r).head.p[dir as usize] = p;
    (*q).head.p[dir as usize] = (*r).head.p[opp as usize];
    (*r).head.p[opp as usize] = q;
    b1 = if dir == 0 as libc::c_int { 1 as libc::c_int } else { -(1 as libc::c_int) };
    if (*r).head.balance as libc::c_int == b1 {
        (*q).head.balance = 0 as libc::c_int as libc::c_schar;
        (*p).head.balance = -b1 as libc::c_schar;
    } else if (*r).head.balance as libc::c_int == 0 as libc::c_int {
        (*p).head.balance = 0 as libc::c_int as libc::c_schar;
        (*q).head.balance = (*p).head.balance;
    } else {
        (*q).head.balance = b1 as libc::c_schar;
        (*p).head.balance = 0 as libc::c_int as libc::c_schar;
    }
    (*r).head.balance = 0 as libc::c_int as libc::c_schar;
    return r;
}
pub unsafe extern "C" fn krmq_itr_next_bidir_lc_elem(
    mut itr: *mut krmq_itr_lc_elem,
    mut dir: libc::c_int,
) -> libc::c_int {
    let mut p: *const lc_elem_t = 0 as *const lc_elem_t;
    if (*itr).top < ((*itr).stack).as_mut_ptr() {
        return 0 as libc::c_int;
    }
    dir = (dir != 0) as libc::c_int;
    p = (**(*itr).top).head.p[dir as usize];
    if !p.is_null() {
        while !p.is_null() {
            (*itr).top = ((*itr).top).offset(1);
            *(*itr).top = p;
            p = (*p).head.p[(dir == 0) as libc::c_int as usize];
        }
        return 1 as libc::c_int;
    } else {
        loop {
            let fresh4 = (*itr).top;
            (*itr).top = ((*itr).top).offset(-1);
            p = *fresh4;
            if !((*itr).top >= ((*itr).stack).as_mut_ptr()
                && p == (**(*itr).top).head.p[dir as usize] as *const lc_elem_t)
            {
                break;
            }
        }
        return if (*itr).top < ((*itr).stack).as_mut_ptr() {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        };
    };
}
#[inline]
unsafe extern "C" fn krmq_update_min_lc_elem(
    mut p: *mut lc_elem_t,
    mut q: *const lc_elem_t,
    mut r: *const lc_elem_t,
) {
    (*p)
        .head
        .s = if q.is_null() || (*p).pri < (*(*q).head.s).pri { p } else { (*q).head.s };
    (*p)
        .head
        .s = if r.is_null() || (*(*p).head.s).pri < (*(*r).head.s).pri {
        (*p).head.s
    } else {
        (*r).head.s
    };
}
#[inline]
unsafe extern "C" fn krmq_rotate1_lc_elem(
    mut p: *mut lc_elem_t,
    mut dir: libc::c_int,
) -> *mut lc_elem_t {
    let mut opp: libc::c_int = 1 as libc::c_int - dir;
    let mut q: *mut lc_elem_t = (*p).head.p[opp as usize];
    let mut s: *mut lc_elem_t = (*p).head.s;
    let mut size_p: libc::c_uint = (*p).head.size;
    (*p)
        .head
        .size = ((*p).head.size)
        .wrapping_sub(
            ((*q).head.size)
                .wrapping_sub(
                    (if !((*q).head.p[dir as usize]).is_null() {
                        (*(*q).head.p[dir as usize]).head.size
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                ),
        );
    (*q).head.size = size_p;
    krmq_update_min_lc_elem(p, (*p).head.p[dir as usize], (*q).head.p[dir as usize]);
    (*q).head.s = s;
    (*p).head.p[opp as usize] = (*q).head.p[dir as usize];
    (*q).head.p[dir as usize] = p;
    return q;
}
pub unsafe extern "C" fn krmq_itr_find_lc_elem(
    mut root: *const lc_elem_t,
    mut x: *const lc_elem_t,
    mut itr: *mut krmq_itr_lc_elem,
) -> libc::c_int {
    let mut p: *const lc_elem_t = root;
    (*itr).top = ((*itr).stack).as_mut_ptr().offset(-(1 as libc::c_int as isize));
    while !p.is_null() {
        let mut cmp: libc::c_int = 0;
        (*itr).top = ((*itr).top).offset(1);
        *(*itr).top = p;
        cmp = if (*x).y < (*p).y {
            -(1 as libc::c_int)
        } else if (*x).y > (*p).y {
            1 as libc::c_int
        } else {
            ((*x).i > (*p).i) as libc::c_int - ((*x).i < (*p).i) as libc::c_int
        };
        if cmp < 0 as libc::c_int {
            p = (*p).head.p[0 as libc::c_int as usize];
        } else {
            if !(cmp > 0 as libc::c_int) {
                break;
            }
            p = (*p).head.p[1 as libc::c_int as usize];
        }
    }
    return if !p.is_null() { 1 as libc::c_int } else { 0 as libc::c_int };
}
pub unsafe extern "C" fn krmq_itr_first_lc_elem(
    mut root: *const lc_elem_t,
    mut itr: *mut krmq_itr_lc_elem,
) {
    let mut p: *const lc_elem_t = 0 as *const lc_elem_t;
    (*itr).top = ((*itr).stack).as_mut_ptr().offset(-(1 as libc::c_int as isize));
    p = root;
    while !p.is_null() {
        (*itr).top = ((*itr).top).offset(1);
        *(*itr).top = p;
        p = (*p).head.p[0 as libc::c_int as usize];
    }
}
pub unsafe extern "C" fn krmq_insert_lc_elem(
    mut root_: *mut *mut lc_elem_t,
    mut x: *mut lc_elem_t,
    mut cnt_: *mut libc::c_uint,
) -> *mut lc_elem_t {
    let mut stack: [libc::c_uchar; 64] = [0; 64];
    let mut path: [*mut lc_elem_t; 64] = [0 as *mut lc_elem_t; 64];
    let mut bp: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut bq: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut p: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut q: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut r: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut i: libc::c_int = 0;
    let mut which: libc::c_int = 0 as libc::c_int;
    let mut top: libc::c_int = 0;
    let mut b1: libc::c_int = 0;
    let mut path_len: libc::c_int = 0;
    let mut cnt: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    bp = *root_;
    bq = 0 as *mut lc_elem_t;
    p = bp;
    q = bq;
    path_len = 0 as libc::c_int;
    top = path_len;
    while !p.is_null() {
        let mut cmp: libc::c_int = 0;
        cmp = if (*x).y < (*p).y {
            -(1 as libc::c_int)
        } else if (*x).y > (*p).y {
            1 as libc::c_int
        } else {
            ((*x).i > (*p).i) as libc::c_int - ((*x).i < (*p).i) as libc::c_int
        };
        if cmp >= 0 as libc::c_int {
            cnt = cnt
                .wrapping_add(
                    (if !((*p).head.p[0 as libc::c_int as usize]).is_null() {
                        (*(*p).head.p[0 as libc::c_int as usize]).head.size
                    } else {
                        0 as libc::c_int as libc::c_uint
                    })
                        .wrapping_add(1 as libc::c_int as libc::c_uint),
                );
        }
        if cmp == 0 as libc::c_int {
            if !cnt_.is_null() {
                *cnt_ = cnt;
            }
            return p;
        }
        if (*p).head.balance as libc::c_int != 0 as libc::c_int {
            bq = q;
            bp = p;
            top = 0 as libc::c_int;
        }
        which = (cmp > 0 as libc::c_int) as libc::c_int;
        let fresh5 = top;
        top = top + 1;
        stack[fresh5 as usize] = which as libc::c_uchar;
        let fresh6 = path_len;
        path_len = path_len + 1;
        path[fresh6 as usize] = p;
        q = p;
        p = (*p).head.p[which as usize];
    }
    if !cnt_.is_null() {
        *cnt_ = cnt;
    }
    (*x).head.balance = 0 as libc::c_int as libc::c_schar;
    (*x).head.size = 1 as libc::c_int as libc::c_uint;
    (*x).head.p[1 as libc::c_int as usize] = 0 as *mut lc_elem_s;
    (*x).head.p[0 as libc::c_int as usize] = (*x).head.p[1 as libc::c_int as usize];
    (*x).head.s = x;
    if q.is_null() {
        *root_ = x;
    } else {
        (*q).head.p[which as usize] = x;
    }
    if bp.is_null() {
        return x;
    }
    i = 0 as libc::c_int;
    while i < path_len {
        (*path[i as usize]).head.size = ((*path[i as usize]).head.size).wrapping_add(1);
        (*path[i as usize]).head.size;
        i += 1;
        i;
    }
    i = path_len - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        krmq_update_min_lc_elem(
            path[i as usize],
            (*path[i as usize]).head.p[0 as libc::c_int as usize],
            (*path[i as usize]).head.p[1 as libc::c_int as usize],
        );
        if (*path[i as usize]).head.s != x {
            break;
        }
        i -= 1;
        i;
    }
    p = bp;
    top = 0 as libc::c_int;
    while p != x {
        if stack[top as usize] as libc::c_int == 0 as libc::c_int {
            (*p).head.balance -= 1;
            (*p).head.balance;
        } else {
            (*p).head.balance += 1;
            (*p).head.balance;
        }
        p = (*p).head.p[stack[top as usize] as usize];
        top += 1;
        top;
    }
    if (*bp).head.balance as libc::c_int > -(2 as libc::c_int)
        && ((*bp).head.balance as libc::c_int) < 2 as libc::c_int
    {
        return x;
    }
    which = (((*bp).head.balance as libc::c_int) < 0 as libc::c_int) as libc::c_int;
    b1 = if which == 0 as libc::c_int { 1 as libc::c_int } else { -(1 as libc::c_int) };
    q = (*bp).head.p[(1 as libc::c_int - which) as usize];
    if (*q).head.balance as libc::c_int == b1 {
        r = krmq_rotate1_lc_elem(bp, which);
        (*bp).head.balance = 0 as libc::c_int as libc::c_schar;
        (*q).head.balance = (*bp).head.balance;
    } else {
        r = krmq_rotate2_lc_elem(bp, which);
    }
    if bq.is_null() {
        *root_ = r;
    } else {
        (*bq)
            .head
            .p[(bp != (*bq).head.p[0 as libc::c_int as usize]) as libc::c_int
            as usize] = r;
    }
    return x;
}
pub unsafe extern "C" fn krmq_find_lc_elem(
    mut root: *const lc_elem_t,
    mut x: *const lc_elem_t,
    mut cnt_: *mut libc::c_uint,
) -> *mut lc_elem_t {
    let mut p: *const lc_elem_t = root;
    let mut cnt: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while !p.is_null() {
        let mut cmp: libc::c_int = 0;
        cmp = if (*x).y < (*p).y {
            -(1 as libc::c_int)
        } else if (*x).y > (*p).y {
            1 as libc::c_int
        } else {
            ((*x).i > (*p).i) as libc::c_int - ((*x).i < (*p).i) as libc::c_int
        };
        if cmp >= 0 as libc::c_int {
            cnt = cnt
                .wrapping_add(
                    (if !((*p).head.p[0 as libc::c_int as usize]).is_null() {
                        (*(*p).head.p[0 as libc::c_int as usize]).head.size
                    } else {
                        0 as libc::c_int as libc::c_uint
                    })
                        .wrapping_add(1 as libc::c_int as libc::c_uint),
                );
        }
        if cmp < 0 as libc::c_int {
            p = (*p).head.p[0 as libc::c_int as usize];
        } else {
            if !(cmp > 0 as libc::c_int) {
                break;
            }
            p = (*p).head.p[1 as libc::c_int as usize];
        }
    }
    if !cnt_.is_null() {
        *cnt_ = cnt;
    }
    return p as *mut lc_elem_t;
}
pub unsafe extern "C" fn krmq_erase_lc_elem(
    mut root_: *mut *mut lc_elem_t,
    mut x: *const lc_elem_t,
    mut cnt_: *mut libc::c_uint,
) -> *mut lc_elem_t {
    let mut p: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut path: [*mut lc_elem_t; 64] = [0 as *mut lc_elem_t; 64];
    let mut fake: lc_elem_t = lc_elem_t {
        y: 0,
        i: 0,
        pri: 0.,
        head: C2RustUnnamed_0 {
            p: [0 as *mut lc_elem_s; 2],
            s: 0 as *mut lc_elem_s,
            balance: 0,
            size: 0,
        },
    };
    let mut dir: [libc::c_uchar; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut d: libc::c_int = 0 as libc::c_int;
    let mut cmp: libc::c_int = 0;
    let mut cnt: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    fake = **root_;
    fake.head.p[0 as libc::c_int as usize] = *root_;
    fake.head.p[1 as libc::c_int as usize] = 0 as *mut lc_elem_s;
    if !cnt_.is_null() {
        *cnt_ = 0 as libc::c_int as libc::c_uint;
    }
    if !x.is_null() {
        cmp = -(1 as libc::c_int);
        p = &mut fake;
        while cmp != 0 {
            let mut which: libc::c_int = (cmp > 0 as libc::c_int) as libc::c_int;
            if cmp > 0 as libc::c_int {
                cnt = cnt
                    .wrapping_add(
                        (if !((*p).head.p[0 as libc::c_int as usize]).is_null() {
                            (*(*p).head.p[0 as libc::c_int as usize]).head.size
                        } else {
                            0 as libc::c_int as libc::c_uint
                        })
                            .wrapping_add(1 as libc::c_int as libc::c_uint),
                    );
            }
            dir[d as usize] = which as libc::c_uchar;
            let fresh7 = d;
            d = d + 1;
            path[fresh7 as usize] = p;
            p = (*p).head.p[which as usize];
            if p.is_null() {
                if !cnt_.is_null() {
                    *cnt_ = 0 as libc::c_int as libc::c_uint;
                }
                return 0 as *mut lc_elem_t;
            }
            cmp = if (*x).y < (*p).y {
                -(1 as libc::c_int)
            } else if (*x).y > (*p).y {
                1 as libc::c_int
            } else {
                ((*x).i > (*p).i) as libc::c_int - ((*x).i < (*p).i) as libc::c_int
            };
        }
        cnt = cnt
            .wrapping_add(
                (if !((*p).head.p[0 as libc::c_int as usize]).is_null() {
                    (*(*p).head.p[0 as libc::c_int as usize]).head.size
                } else {
                    0 as libc::c_int as libc::c_uint
                })
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
            );
    } else {
        p = &mut fake;
        cnt = 1 as libc::c_int as libc::c_uint;
        while !p.is_null() {
            dir[d as usize] = 0 as libc::c_int as libc::c_uchar;
            let fresh8 = d;
            d = d + 1;
            path[fresh8 as usize] = p;
            p = (*p).head.p[0 as libc::c_int as usize];
        }
        d -= 1;
        p = path[d as usize];
    }
    if !cnt_.is_null() {
        *cnt_ = cnt;
    }
    i = 1 as libc::c_int;
    while i < d {
        (*path[i as usize]).head.size = ((*path[i as usize]).head.size).wrapping_sub(1);
        (*path[i as usize]).head.size;
        i += 1;
        i;
    }
    if ((*p).head.p[1 as libc::c_int as usize]).is_null() {
        (*path[(d - 1 as libc::c_int) as usize])
            .head
            .p[dir[(d - 1 as libc::c_int) as usize]
            as usize] = (*p).head.p[0 as libc::c_int as usize];
    } else {
        let mut q: *mut lc_elem_t = (*p).head.p[1 as libc::c_int as usize];
        if ((*q).head.p[0 as libc::c_int as usize]).is_null() {
            (*q)
                .head
                .p[0 as libc::c_int as usize] = (*p).head.p[0 as libc::c_int as usize];
            (*q).head.balance = (*p).head.balance;
            (*path[(d - 1 as libc::c_int) as usize])
                .head
                .p[dir[(d - 1 as libc::c_int) as usize] as usize] = q;
            path[d as usize] = q;
            let fresh9 = d;
            d = d + 1;
            dir[fresh9 as usize] = 1 as libc::c_int as libc::c_uchar;
            (*q)
                .head
                .size = ((*p).head.size).wrapping_sub(1 as libc::c_int as libc::c_uint);
        } else {
            let mut r: *mut lc_elem_t = 0 as *mut lc_elem_t;
            let fresh10 = d;
            d = d + 1;
            let mut e: libc::c_int = fresh10;
            loop {
                dir[d as usize] = 0 as libc::c_int as libc::c_uchar;
                let fresh11 = d;
                d = d + 1;
                path[fresh11 as usize] = q;
                r = (*q).head.p[0 as libc::c_int as usize];
                if ((*r).head.p[0 as libc::c_int as usize]).is_null() {
                    break;
                }
                q = r;
            }
            (*r)
                .head
                .p[0 as libc::c_int as usize] = (*p).head.p[0 as libc::c_int as usize];
            (*q)
                .head
                .p[0 as libc::c_int as usize] = (*r).head.p[1 as libc::c_int as usize];
            (*r)
                .head
                .p[1 as libc::c_int as usize] = (*p).head.p[1 as libc::c_int as usize];
            (*r).head.balance = (*p).head.balance;
            (*path[(e - 1 as libc::c_int) as usize])
                .head
                .p[dir[(e - 1 as libc::c_int) as usize] as usize] = r;
            path[e as usize] = r;
            dir[e as usize] = 1 as libc::c_int as libc::c_uchar;
            i = e + 1 as libc::c_int;
            while i < d {
                (*path[i as usize])
                    .head
                    .size = ((*path[i as usize]).head.size).wrapping_sub(1);
                (*path[i as usize]).head.size;
                i += 1;
                i;
            }
            (*r)
                .head
                .size = ((*p).head.size).wrapping_sub(1 as libc::c_int as libc::c_uint);
        }
    }
    i = d - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        krmq_update_min_lc_elem(
            path[i as usize],
            (*path[i as usize]).head.p[0 as libc::c_int as usize],
            (*path[i as usize]).head.p[1 as libc::c_int as usize],
        );
        i -= 1;
        i;
    }
    loop {
        d -= 1;
        if !(d > 0 as libc::c_int) {
            break;
        }
        let mut q_0: *mut lc_elem_t = path[d as usize];
        let mut which_0: libc::c_int = 0;
        let mut other: libc::c_int = 0;
        let mut b1: libc::c_int = 1 as libc::c_int;
        let mut b2: libc::c_int = 2 as libc::c_int;
        which_0 = dir[d as usize] as libc::c_int;
        other = 1 as libc::c_int - which_0;
        if which_0 != 0 {
            b1 = -b1;
            b2 = -b2;
        }
        (*q_0).head.balance = ((*q_0).head.balance as libc::c_int + b1) as libc::c_schar;
        if (*q_0).head.balance as libc::c_int == b1 {
            break;
        }
        if !((*q_0).head.balance as libc::c_int == b2) {
            continue;
        }
        let mut r_0: *mut lc_elem_t = (*q_0).head.p[other as usize];
        if (*r_0).head.balance as libc::c_int == -b1 {
            (*path[(d - 1 as libc::c_int) as usize])
                .head
                .p[dir[(d - 1 as libc::c_int) as usize]
                as usize] = krmq_rotate2_lc_elem(q_0, which_0);
        } else {
            (*path[(d - 1 as libc::c_int) as usize])
                .head
                .p[dir[(d - 1 as libc::c_int) as usize]
                as usize] = krmq_rotate1_lc_elem(q_0, which_0);
            if (*r_0).head.balance as libc::c_int == 0 as libc::c_int {
                (*r_0).head.balance = -b1 as libc::c_schar;
                (*q_0).head.balance = b1 as libc::c_schar;
                break;
            } else {
                (*q_0).head.balance = 0 as libc::c_int as libc::c_schar;
                (*r_0).head.balance = (*q_0).head.balance;
            }
        }
    }
    *root_ = fake.head.p[0 as libc::c_int as usize];
    return p;
}
pub unsafe extern "C" fn krmq_rmq_lc_elem(
    mut root: *const lc_elem_t,
    mut lo: *const lc_elem_t,
    mut up: *const lc_elem_t,
) -> *mut lc_elem_t {
    let mut p: *const lc_elem_t = root;
    let mut path: [[*const lc_elem_t; 64]; 2] = [[0 as *const lc_elem_t; 64]; 2];
    let mut min: *const lc_elem_t = 0 as *const lc_elem_t;
    let mut plen: [libc::c_int; 2] = [0 as libc::c_int, 0 as libc::c_int];
    let mut pcmp: [[libc::c_int; 64]; 2] = [[0; 64]; 2];
    let mut i: libc::c_int = 0;
    let mut cmp: libc::c_int = 0;
    let mut lca: libc::c_int = 0;
    if root.is_null() {
        return 0 as *mut lc_elem_t;
    }
    while !p.is_null() {
        cmp = if (*lo).y < (*p).y {
            -(1 as libc::c_int)
        } else if (*lo).y > (*p).y {
            1 as libc::c_int
        } else {
            ((*lo).i > (*p).i) as libc::c_int - ((*lo).i < (*p).i) as libc::c_int
        };
        path[0 as libc::c_int as usize][plen[0 as libc::c_int as usize] as usize] = p;
        let fresh12 = plen[0 as libc::c_int as usize];
        plen[0 as libc::c_int as usize] = plen[0 as libc::c_int as usize] + 1;
        pcmp[0 as libc::c_int as usize][fresh12 as usize] = cmp;
        if cmp < 0 as libc::c_int {
            p = (*p).head.p[0 as libc::c_int as usize];
        } else {
            if !(cmp > 0 as libc::c_int) {
                break;
            }
            p = (*p).head.p[1 as libc::c_int as usize];
        }
    }
    p = root;
    while !p.is_null() {
        cmp = if (*up).y < (*p).y {
            -(1 as libc::c_int)
        } else if (*up).y > (*p).y {
            1 as libc::c_int
        } else {
            ((*up).i > (*p).i) as libc::c_int - ((*up).i < (*p).i) as libc::c_int
        };
        path[1 as libc::c_int as usize][plen[1 as libc::c_int as usize] as usize] = p;
        let fresh13 = plen[1 as libc::c_int as usize];
        plen[1 as libc::c_int as usize] = plen[1 as libc::c_int as usize] + 1;
        pcmp[1 as libc::c_int as usize][fresh13 as usize] = cmp;
        if cmp < 0 as libc::c_int {
            p = (*p).head.p[0 as libc::c_int as usize];
        } else {
            if !(cmp > 0 as libc::c_int) {
                break;
            }
            p = (*p).head.p[1 as libc::c_int as usize];
        }
    }
    i = 0 as libc::c_int;
    while i < plen[0 as libc::c_int as usize] && i < plen[1 as libc::c_int as usize] {
        if path[0 as libc::c_int as usize][i as usize]
            == path[1 as libc::c_int as usize][i as usize]
            && pcmp[0 as libc::c_int as usize][i as usize] <= 0 as libc::c_int
            && pcmp[1 as libc::c_int as usize][i as usize] >= 0 as libc::c_int
        {
            break;
        }
        i += 1;
        i;
    }
    if i == plen[0 as libc::c_int as usize] || i == plen[1 as libc::c_int as usize] {
        return 0 as *mut lc_elem_t;
    }
    lca = i;
    min = path[0 as libc::c_int as usize][lca as usize];
    i = lca + 1 as libc::c_int;
    while i < plen[0 as libc::c_int as usize] {
        if pcmp[0 as libc::c_int as usize][i as usize] <= 0 as libc::c_int {
            if (*path[0 as libc::c_int as usize][i as usize]).pri < (*min).pri {
                min = path[0 as libc::c_int as usize][i as usize];
            }
            if !((*path[0 as libc::c_int as usize][i as usize])
                .head
                .p[1 as libc::c_int as usize])
                .is_null()
                && (*(*(*path[0 as libc::c_int as usize][i as usize])
                    .head
                    .p[1 as libc::c_int as usize])
                    .head
                    .s)
                    .pri < (*min).pri
            {
                min = (*(*path[0 as libc::c_int as usize][i as usize])
                    .head
                    .p[1 as libc::c_int as usize])
                    .head
                    .s;
            }
        }
        i += 1;
        i;
    }
    i = lca + 1 as libc::c_int;
    while i < plen[1 as libc::c_int as usize] {
        if pcmp[1 as libc::c_int as usize][i as usize] >= 0 as libc::c_int {
            if (*path[1 as libc::c_int as usize][i as usize]).pri < (*min).pri {
                min = path[1 as libc::c_int as usize][i as usize];
            }
            if !((*path[1 as libc::c_int as usize][i as usize])
                .head
                .p[0 as libc::c_int as usize])
                .is_null()
                && (*(*(*path[1 as libc::c_int as usize][i as usize])
                    .head
                    .p[0 as libc::c_int as usize])
                    .head
                    .s)
                    .pri < (*min).pri
            {
                min = (*(*path[1 as libc::c_int as usize][i as usize])
                    .head
                    .p[0 as libc::c_int as usize])
                    .head
                    .s;
            }
        }
        i += 1;
        i;
    }
    return min as *mut lc_elem_t;
}
pub unsafe extern "C" fn krmq_interval_lc_elem(
    mut root: *const lc_elem_t,
    mut x: *const lc_elem_t,
    mut lower: *mut *mut lc_elem_t,
    mut upper: *mut *mut lc_elem_t,
) -> *mut lc_elem_t {
    let mut p: *const lc_elem_t = root;
    let mut l: *const lc_elem_t = 0 as *const lc_elem_t;
    let mut u: *const lc_elem_t = 0 as *const lc_elem_t;
    while !p.is_null() {
        let mut cmp: libc::c_int = 0;
        cmp = if (*x).y < (*p).y {
            -(1 as libc::c_int)
        } else if (*x).y > (*p).y {
            1 as libc::c_int
        } else {
            ((*x).i > (*p).i) as libc::c_int - ((*x).i < (*p).i) as libc::c_int
        };
        if cmp < 0 as libc::c_int {
            u = p;
            p = (*p).head.p[0 as libc::c_int as usize];
        } else if cmp > 0 as libc::c_int {
            l = p;
            p = (*p).head.p[1 as libc::c_int as usize];
        } else {
            u = p;
            l = u;
            break;
        }
    }
    if !lower.is_null() {
        *lower = l as *mut lc_elem_t;
    }
    if !upper.is_null() {
        *upper = u as *mut lc_elem_t;
    }
    return p as *mut lc_elem_t;
}
#[inline]
unsafe extern "C" fn kmp_init_rmq(mut km: *mut libc::c_void) -> *mut kmp_rmq_t {
    let mut mp: *mut kmp_rmq_t = 0 as *mut kmp_rmq_t;
    mp = kcalloc(
        km,
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<kmp_rmq_t>() as libc::c_ulong,
    ) as *mut kmp_rmq_t;
    (*mp).km = km;
    return mp;
}
#[inline]
unsafe extern "C" fn kmp_alloc_rmq(mut mp: *mut kmp_rmq_t) -> *mut lc_elem_t {
    (*mp).cnt = ((*mp).cnt).wrapping_add(1);
    (*mp).cnt;
    if (*mp).n == 0 as libc::c_int as libc::c_ulong {
        return kcalloc(
            (*mp).km,
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<lc_elem_t>() as libc::c_ulong,
        ) as *mut lc_elem_t;
    }
    (*mp).n = ((*mp).n).wrapping_sub(1);
    return *((*mp).buf).offset((*mp).n as isize);
}
#[inline]
unsafe extern "C" fn kmp_free_rmq(mut mp: *mut kmp_rmq_t, mut p: *mut lc_elem_t) {
    (*mp).cnt = ((*mp).cnt).wrapping_sub(1);
    (*mp).cnt;
    if (*mp).n == (*mp).max {
        (*mp)
            .max = if (*mp).max >= 4 as libc::c_int as libc::c_ulong {
            ((*mp).max).wrapping_add((*mp).max >> 1 as libc::c_int)
        } else {
            16 as libc::c_int as libc::c_ulong
        };
        (*mp)
            .buf = krealloc(
            (*mp).km,
            (*mp).buf as *mut libc::c_void,
            ((*mp).max)
                .wrapping_mul(::std::mem::size_of::<*mut lc_elem_t>() as libc::c_ulong),
        ) as *mut *mut lc_elem_t;
    }
    let fresh14 = (*mp).n;
    (*mp).n = ((*mp).n).wrapping_add(1);
    let ref mut fresh15 = *((*mp).buf).offset(fresh14 as isize);
    *fresh15 = p;
}
#[inline]
unsafe extern "C" fn comput_sc_simple(
    mut ai: *const mm128_t,
    mut aj: *const mm128_t,
    mut chn_pen_gap: libc::c_float,
    mut chn_pen_skip: libc::c_float,
    mut exact: *mut int32_t,
    mut width: *mut int32_t,
) -> int32_t {
    let mut dq: int32_t = (*ai).y as int32_t - (*aj).y as int32_t;
    let mut dr: int32_t = 0;
    let mut dd: int32_t = 0;
    let mut dg: int32_t = 0;
    let mut q_span: int32_t = 0;
    let mut sc: int32_t = 0;
    dr = ((*ai).x).wrapping_sub((*aj).x) as int32_t;
    dd = if dr > dq { dr - dq } else { dq - dr };
    *width = dd;
    dg = if dr < dq { dr } else { dq };
    q_span = ((*aj).y >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as int32_t;
    sc = if q_span < dg { q_span } else { dg };
    if !exact.is_null() {
        *exact = (dd == 0 as libc::c_int && dg <= q_span) as libc::c_int;
    }
    if dd != 0 || dq > q_span {
        let mut lin_pen: libc::c_float = 0.;
        let mut log_pen: libc::c_float = 0.;
        lin_pen = chn_pen_gap * dd as libc::c_float + chn_pen_skip * dg as libc::c_float;
        log_pen = if dd >= 1 as libc::c_int {
            mg_log2((dd + 1 as libc::c_int) as libc::c_float)
        } else {
            0.0f32
        };
        sc -= (lin_pen + 0.5f32 * log_pen) as libc::c_int;
    }
    return sc;
}
pub unsafe extern "C" fn mg_lchain_rmq(
    mut max_dist: libc::c_int,
    mut max_dist_inner: libc::c_int,
    mut bw: libc::c_int,
    mut max_chn_skip: libc::c_int,
    mut cap_rmq_size: libc::c_int,
    mut min_cnt: libc::c_int,
    mut min_sc: libc::c_int,
    mut chn_pen_gap: libc::c_float,
    mut chn_pen_skip: libc::c_float,
    mut n: int64_t,
    mut a: *mut mm128_t,
    mut n_u_: *mut libc::c_int,
    mut _u: *mut *mut uint64_t,
    mut km: *mut libc::c_void,
) -> *mut mm128_t {
    let mut f: *mut int32_t = 0 as *mut int32_t;
    let mut t: *mut int32_t = 0 as *mut int32_t;
    let mut v: *mut int32_t = 0 as *mut int32_t;
    let mut n_u: int32_t = 0;
    let mut n_v: int32_t = 0;
    let mut mmax_f: int32_t = 0 as libc::c_int;
    let mut max_rmq_size: int32_t = 0 as libc::c_int;
    let mut max_drop: int32_t = bw;
    let mut p: *mut int64_t = 0 as *mut int64_t;
    let mut i: int64_t = 0;
    let mut i0: int64_t = 0;
    let mut st: int64_t = 0 as libc::c_int as int64_t;
    let mut st_inner: int64_t = 0 as libc::c_int as int64_t;
    let mut n_iter: int64_t = 0 as libc::c_int as int64_t;
    let mut u: *mut uint64_t = 0 as *mut uint64_t;
    let mut root: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut root_inner: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut mem_mp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut mp: *mut kmp_rmq_t = 0 as *mut kmp_rmq_t;
    if !_u.is_null() {
        *_u = 0 as *mut uint64_t;
        *n_u_ = 0 as libc::c_int;
    }
    if n == 0 as libc::c_int as libc::c_long || a.is_null() {
        kfree(km, a as *mut libc::c_void);
        return 0 as *mut mm128_t;
    }
    if max_dist < bw {
        max_dist = bw;
    }
    if max_dist_inner <= 0 as libc::c_int || max_dist_inner >= max_dist {
        max_dist_inner = 0 as libc::c_int;
    }
    p = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int64_t>() as libc::c_ulong),
    ) as *mut int64_t;
    f = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
    ) as *mut int32_t;
    t = kcalloc(km, n as size_t, ::std::mem::size_of::<int32_t>() as libc::c_ulong)
        as *mut int32_t;
    v = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
    ) as *mut int32_t;
    mem_mp = km_init2(km, 0x10000 as libc::c_int as size_t);
    mp = kmp_init_rmq(mem_mp);
    i0 = 0 as libc::c_int as int64_t;
    i = i0;
    while i < n {
        let mut max_j: int64_t = -(1 as libc::c_int) as int64_t;
        let mut q_span: int32_t = ((*a.offset(i as isize)).y >> 32 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as int32_t;
        let mut max_f: int32_t = q_span;
        let mut s: lc_elem_t = lc_elem_t {
            y: 0,
            i: 0,
            pri: 0.,
            head: C2RustUnnamed_0 {
                p: [0 as *mut lc_elem_s; 2],
                s: 0 as *mut lc_elem_s,
                balance: 0,
                size: 0,
            },
        };
        let mut q: *mut lc_elem_t = 0 as *mut lc_elem_t;
        let mut r: *mut lc_elem_t = 0 as *mut lc_elem_t;
        let mut lo: lc_elem_t = lc_elem_t {
            y: 0,
            i: 0,
            pri: 0.,
            head: C2RustUnnamed_0 {
                p: [0 as *mut lc_elem_s; 2],
                s: 0 as *mut lc_elem_s,
                balance: 0,
                size: 0,
            },
        };
        let mut hi: lc_elem_t = lc_elem_t {
            y: 0,
            i: 0,
            pri: 0.,
            head: C2RustUnnamed_0 {
                p: [0 as *mut lc_elem_s; 2],
                s: 0 as *mut lc_elem_s,
                balance: 0,
                size: 0,
            },
        };
        if i0 < i && (*a.offset(i0 as isize)).x != (*a.offset(i as isize)).x {
            let mut j: int64_t = 0;
            j = i0;
            while j < i {
                q = kmp_alloc_rmq(mp);
                (*q).y = (*a.offset(j as isize)).y as int32_t;
                (*q).i = j;
                (*q)
                    .pri = -(*f.offset(j as isize) as libc::c_double
                    + 0.5f64 * chn_pen_gap as libc::c_double
                        * ((*a.offset(j as isize)).x as int32_t
                            + (*a.offset(j as isize)).y as int32_t) as libc::c_double);
                krmq_insert_lc_elem(&mut root, q, 0 as *mut libc::c_uint);
                if max_dist_inner > 0 as libc::c_int {
                    r = kmp_alloc_rmq(mp);
                    *r = *q;
                    krmq_insert_lc_elem(&mut root_inner, r, 0 as *mut libc::c_uint);
                }
                j += 1;
                j;
            }
            i0 = i;
        }
        while st < i
            && ((*a.offset(i as isize)).x >> 32 as libc::c_int
                != (*a.offset(st as isize)).x >> 32 as libc::c_int
                || (*a.offset(i as isize)).x
                    > ((*a.offset(st as isize)).x)
                        .wrapping_add(max_dist as libc::c_ulong)
                || (if !root.is_null() {
                    (*root).head.size
                } else {
                    0 as libc::c_int as libc::c_uint
                }) > cap_rmq_size as libc::c_uint)
        {
            s.y = (*a.offset(st as isize)).y as int32_t;
            s.i = st;
            q = krmq_find_lc_elem(root, &mut s, 0 as *mut libc::c_uint);
            if !q.is_null() {
                q = krmq_erase_lc_elem(&mut root, q, 0 as *mut libc::c_uint);
                kmp_free_rmq(mp, q);
            }
            st += 1;
            st;
        }
        if max_dist_inner > 0 as libc::c_int {
            while st_inner < i
                && ((*a.offset(i as isize)).x >> 32 as libc::c_int
                    != (*a.offset(st_inner as isize)).x >> 32 as libc::c_int
                    || (*a.offset(i as isize)).x
                        > ((*a.offset(st_inner as isize)).x)
                            .wrapping_add(max_dist_inner as libc::c_ulong)
                    || (if !root_inner.is_null() {
                        (*root_inner).head.size
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }) > cap_rmq_size as libc::c_uint)
            {
                s.y = (*a.offset(st_inner as isize)).y as int32_t;
                s.i = st_inner;
                q = krmq_find_lc_elem(root_inner, &mut s, 0 as *mut libc::c_uint);
                if !q.is_null() {
                    q = krmq_erase_lc_elem(&mut root_inner, q, 0 as *mut libc::c_uint);
                    kmp_free_rmq(mp, q);
                }
                st_inner += 1;
                st_inner;
            }
        }
        lo.i = 2147483647 as libc::c_int as int64_t;
        lo.y = (*a.offset(i as isize)).y as int32_t - max_dist;
        hi.i = 0 as libc::c_int as int64_t;
        hi.y = (*a.offset(i as isize)).y as int32_t;
        q = krmq_rmq_lc_elem(root, &mut lo, &mut hi);
        if !q.is_null() {
            let mut sc: int32_t = 0;
            let mut exact: int32_t = 0;
            let mut width: int32_t = 0;
            let mut n_skip: int32_t = 0 as libc::c_int;
            let mut j_0: int64_t = (*q).i;
            if (*q).y >= lo.y && (*q).y <= hi.y {} else {
                __assert_fail(
                    b"q->y >= lo.y && q->y <= hi.y\0" as *const u8
                        as *const libc::c_char,
                    b"lchain.c\0" as *const u8 as *const libc::c_char,
                    319 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 120],
                        &[libc::c_char; 120],
                    >(
                        b"mm128_t *mg_lchain_rmq(int, int, int, int, int, int, int, float, float, int64_t, mm128_t *, int *, uint64_t **, void *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_7287: {
                if (*q).y >= lo.y && (*q).y <= hi.y {} else {
                    __assert_fail(
                        b"q->y >= lo.y && q->y <= hi.y\0" as *const u8
                            as *const libc::c_char,
                        b"lchain.c\0" as *const u8 as *const libc::c_char,
                        319 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 120],
                            &[libc::c_char; 120],
                        >(
                            b"mm128_t *mg_lchain_rmq(int, int, int, int, int, int, int, float, float, int64_t, mm128_t *, int *, uint64_t **, void *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            sc = *f.offset(j_0 as isize)
                + comput_sc_simple(
                    &mut *a.offset(i as isize),
                    &mut *a.offset(j_0 as isize),
                    chn_pen_gap,
                    chn_pen_skip,
                    &mut exact,
                    &mut width,
                );
            if width <= bw && sc > max_f {
                max_f = sc;
                max_j = j_0;
            }
            if exact == 0 && !root_inner.is_null()
                && (*a.offset(i as isize)).y as int32_t > 0 as libc::c_int
            {
                let mut lo_0: *mut lc_elem_t = 0 as *mut lc_elem_t;
                let mut hi_0: *mut lc_elem_t = 0 as *mut lc_elem_t;
                s.y = (*a.offset(i as isize)).y as int32_t - 1 as libc::c_int;
                s.i = n;
                krmq_interval_lc_elem(root_inner, &mut s, &mut lo_0, &mut hi_0);
                if !lo_0.is_null() {
                    let mut q_0: *const lc_elem_t = 0 as *const lc_elem_t;
                    let mut width_0: int32_t = 0;
                    let mut n_rmq_iter: int32_t = 0 as libc::c_int;
                    let mut itr: krmq_itr_lc_elem = krmq_itr_lc_elem {
                        stack: [0 as *const lc_elem_t; 64],
                        top: 0 as *mut *const lc_elem_t,
                    };
                    krmq_itr_find_lc_elem(root_inner, lo_0, &mut itr);
                    loop {
                        q_0 = if itr.top < (itr.stack).as_mut_ptr() {
                            0 as *const lc_elem_t
                        } else {
                            *itr.top
                        };
                        if q_0.is_null() {
                            break;
                        }
                        if (*q_0).y
                            < (*a.offset(i as isize)).y as int32_t - max_dist_inner
                        {
                            break;
                        }
                        n_rmq_iter += 1;
                        n_rmq_iter;
                        j_0 = (*q_0).i;
                        sc = *f.offset(j_0 as isize)
                            + comput_sc_simple(
                                &mut *a.offset(i as isize),
                                &mut *a.offset(j_0 as isize),
                                chn_pen_gap,
                                chn_pen_skip,
                                0 as *mut int32_t,
                                &mut width_0,
                            );
                        if width_0 <= bw {
                            if sc > max_f {
                                max_f = sc;
                                max_j = j_0;
                                if n_skip > 0 as libc::c_int {
                                    n_skip -= 1;
                                    n_skip;
                                }
                            } else if *t.offset(j_0 as isize) == i as int32_t {
                                n_skip += 1;
                                if n_skip > max_chn_skip {
                                    break;
                                }
                            }
                            if *p.offset(j_0 as isize)
                                >= 0 as libc::c_int as libc::c_long
                            {
                                *t.offset(*p.offset(j_0 as isize) as isize) = i as int32_t;
                            }
                        }
                        if krmq_itr_next_bidir_lc_elem(&mut itr, 0 as libc::c_int) == 0 {
                            break;
                        }
                    }
                    n_iter += n_rmq_iter as libc::c_long;
                }
            }
        }
        if max_j < 0 as libc::c_int as libc::c_long
            || (*a.offset(max_j as isize)).x < (*a.offset(i as isize)).x
                && ((*a.offset(max_j as isize)).y as int32_t)
                    < (*a.offset(i as isize)).y as int32_t
        {} else {
            __assert_fail(
                b"max_j < 0 || (a[max_j].x < a[i].x && (int32_t)a[max_j].y < (int32_t)a[i].y)\0"
                    as *const u8 as *const libc::c_char,
                b"lchain.c\0" as *const u8 as *const libc::c_char,
                353 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 120],
                    &[libc::c_char; 120],
                >(
                    b"mm128_t *mg_lchain_rmq(int, int, int, int, int, int, int, float, float, int64_t, mm128_t *, int *, uint64_t **, void *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_6209: {
            if max_j < 0 as libc::c_int as libc::c_long
                || (*a.offset(max_j as isize)).x < (*a.offset(i as isize)).x
                    && ((*a.offset(max_j as isize)).y as int32_t)
                        < (*a.offset(i as isize)).y as int32_t
            {} else {
                __assert_fail(
                    b"max_j < 0 || (a[max_j].x < a[i].x && (int32_t)a[max_j].y < (int32_t)a[i].y)\0"
                        as *const u8 as *const libc::c_char,
                    b"lchain.c\0" as *const u8 as *const libc::c_char,
                    353 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 120],
                        &[libc::c_char; 120],
                    >(
                        b"mm128_t *mg_lchain_rmq(int, int, int, int, int, int, int, float, float, int64_t, mm128_t *, int *, uint64_t **, void *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        *f.offset(i as isize) = max_f;
        *p.offset(i as isize) = max_j;
        *v
            .offset(
                i as isize,
            ) = if max_j >= 0 as libc::c_int as libc::c_long
            && *v.offset(max_j as isize) > max_f
        {
            *v.offset(max_j as isize)
        } else {
            max_f
        };
        if mmax_f < max_f {
            mmax_f = max_f;
        }
        if (max_rmq_size as libc::c_uint)
            < (if !root.is_null() {
                (*root).head.size
            } else {
                0 as libc::c_int as libc::c_uint
            })
        {
            max_rmq_size = (if !root.is_null() {
                (*root).head.size
            } else {
                0 as libc::c_int as libc::c_uint
            }) as int32_t;
        }
        i += 1;
        i;
    }
    km_destroy(mem_mp);
    u = mg_chain_backtrack(
        km,
        n,
        f,
        p,
        v,
        t,
        min_cnt,
        min_sc,
        max_drop,
        &mut n_u,
        &mut n_v,
    );
    *n_u_ = n_u;
    *_u = u;
    kfree(km, p as *mut libc::c_void);
    kfree(km, f as *mut libc::c_void);
    kfree(km, t as *mut libc::c_void);
    if n_u == 0 as libc::c_int {
        kfree(km, a as *mut libc::c_void);
        kfree(km, v as *mut libc::c_void);
        return 0 as *mut mm128_t;
    }
    return compact_a(km, n_u, u, n_v, v, a);
}
