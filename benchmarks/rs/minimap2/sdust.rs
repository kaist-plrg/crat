use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn kmalloc(km: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn krealloc(
        km: *mut libc::c_void,
        ptr: *mut libc::c_void,
        size: size_t,
    ) -> *mut libc::c_void;
    fn kcalloc(km: *mut libc::c_void, count: size_t, size: size_t) -> *mut libc::c_void;
    fn kfree(km: *mut libc::c_void, ptr: *mut libc::c_void);
    static mut seq_nt4_table: [libc::c_uchar; 256];
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdust_buf_s {
    pub w: *mut kdq_int_t,
    pub P: perf_intv_v,
    pub res: uint64_v,
    pub km: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint64_v {
    pub n: size_t,
    pub m: size_t,
    pub a: *mut uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct perf_intv_v {
    pub n: size_t,
    pub m: size_t,
    pub a: *mut perf_intv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct perf_intv_t {
    pub start: libc::c_int,
    pub finish: libc::c_int,
    pub r: libc::c_int,
    pub l: libc::c_int,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct kdq_int_t {
    #[bitfield(name = "front", ty = "uint64_t", bits = "0..=57")]
    #[bitfield(name = "bits", ty = "uint64_t", bits = "58..=63")]
    pub front_bits: [u8; 8],
    pub count: uint64_t,
    pub mask: uint64_t,
    pub a: *mut libc::c_int,
    pub km: *mut libc::c_void,
}
pub type sdust_buf_t = sdust_buf_s;
#[inline]
unsafe extern "C" fn kdq_push_int(mut q: *mut kdq_int_t, mut v: libc::c_int) {
    if (*q).count as libc::c_ulonglong
        == (1 as libc::c_ulonglong) << (*q).bits() as libc::c_int
    {
        kdq_resize_int(q, (*q).bits() as libc::c_int + 1 as libc::c_int);
    }
    let fresh0 = (*q).count;
    (*q).count = ((*q).count).wrapping_add(1);
    *((*q).a).offset((fresh0.wrapping_add((*q).front()) & (*q).mask) as isize) = v;
}
#[inline]
unsafe extern "C" fn kdq_resize_int(
    mut q: *mut kdq_int_t,
    mut new_bits: libc::c_int,
) -> libc::c_int {
    let mut new_size: size_t = ((1 as libc::c_ulonglong) << new_bits) as size_t;
    let mut old_size: size_t = ((1 as libc::c_ulonglong) << (*q).bits() as libc::c_int)
        as size_t;
    if new_size < (*q).count {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 64 as libc::c_int {
            if (1 as libc::c_ulonglong) << i > (*q).count as libc::c_ulonglong {
                break;
            }
            i += 1;
            i;
        }
        new_bits = i;
        new_size = ((1 as libc::c_ulonglong) << new_bits) as size_t;
    }
    if new_bits == (*q).bits() as libc::c_int {
        return (*q).bits() as libc::c_int;
    }
    if new_bits > (*q).bits() as libc::c_int {
        (*q)
            .a = krealloc(
            (*q).km,
            (*q).a as *mut libc::c_void,
            ((1 as libc::c_ulonglong) << new_bits)
                .wrapping_mul(
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                        as libc::c_ulonglong,
                ) as size_t,
        ) as *mut libc::c_int;
    }
    if (*q).front().wrapping_add((*q).count) <= old_size {
        if (*q).front().wrapping_add((*q).count) > new_size {
            memmove(
                (*q).a as *mut libc::c_void,
                ((*q).a).offset(new_size as isize) as *const libc::c_void,
                (*q)
                    .front()
                    .wrapping_add((*q).count)
                    .wrapping_sub(new_size)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
        }
    } else {
        memmove(
            ((*q).a)
                .offset(
                    new_size.wrapping_sub(old_size.wrapping_sub((*q).front())) as isize,
                ) as *mut libc::c_void,
            ((*q).a).offset((*q).front() as isize) as *const libc::c_void,
            old_size
                .wrapping_sub((*q).front())
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        (*q).set_front(new_size.wrapping_sub(old_size.wrapping_sub((*q).front())));
    }
    (*q).set_bits(new_bits as uint64_t);
    (*q)
        .mask = ((1 as libc::c_ulonglong) << (*q).bits() as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as uint64_t;
    if new_bits < (*q).bits() as libc::c_int {
        (*q)
            .a = krealloc(
            (*q).km,
            (*q).a as *mut libc::c_void,
            ((1 as libc::c_ulonglong) << new_bits)
                .wrapping_mul(
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                        as libc::c_ulonglong,
                ) as size_t,
        ) as *mut libc::c_int;
    }
    return (*q).bits() as libc::c_int;
}
#[inline]
unsafe extern "C" fn kdq_shift_int(mut q: *mut kdq_int_t) -> *mut libc::c_int {
    let mut d: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*q).count == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_int;
    }
    let ref mut fresh1 = (*q).front();
    let fresh2 = *fresh1;
    *fresh1 = (*fresh1).wrapping_add(1);
    d = &mut *((*q).a).offset(fresh2 as isize) as *mut libc::c_int;
    (*q).set_front((*q).front() & (*q).mask as uint64_t);
    (*q).count = ((*q).count).wrapping_sub(1);
    (*q).count;
    return d;
}
#[inline]
unsafe extern "C" fn kdq_init_int(mut km: *mut libc::c_void) -> *mut kdq_int_t {
    let mut q: *mut kdq_int_t = 0 as *mut kdq_int_t;
    q = kcalloc(
        km,
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<kdq_int_t>() as libc::c_ulong,
    ) as *mut kdq_int_t;
    (*q).set_bits(2 as libc::c_int as uint64_t);
    (*q)
        .mask = ((1 as libc::c_ulonglong) << (*q).bits() as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as uint64_t;
    (*q)
        .a = kmalloc(
        km,
        (((1 as libc::c_int) << (*q).bits() as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*q).km = km;
    return q;
}
#[inline]
unsafe extern "C" fn kdq_destroy_int(mut q: *mut kdq_int_t) {
    if q.is_null() {
        return;
    }
    kfree((*q).km, (*q).a as *mut libc::c_void);
    kfree((*q).km, q as *mut libc::c_void);
}
pub unsafe extern "C" fn sdust_buf_init(mut km: *mut libc::c_void) -> *mut sdust_buf_t {
    let mut buf: *mut sdust_buf_t = 0 as *mut sdust_buf_t;
    buf = kcalloc(
        km,
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<sdust_buf_t>() as libc::c_ulong,
    ) as *mut sdust_buf_t;
    (*buf).km = km;
    (*buf).w = kdq_init_int((*buf).km);
    kdq_resize_int((*buf).w, 8 as libc::c_int);
    return buf;
}
pub unsafe extern "C" fn sdust_buf_destroy(mut buf: *mut sdust_buf_t) {
    if buf.is_null() {
        return;
    }
    kdq_destroy_int((*buf).w);
    kfree((*buf).km, (*buf).P.a as *mut libc::c_void);
    kfree((*buf).km, (*buf).res.a as *mut libc::c_void);
    kfree((*buf).km, buf as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn shift_window(
    mut t: libc::c_int,
    mut w: *mut kdq_int_t,
    mut T: libc::c_int,
    mut W: libc::c_int,
    mut L: *mut libc::c_int,
    mut rw: *mut libc::c_int,
    mut rv: *mut libc::c_int,
    mut cw: *mut libc::c_int,
    mut cv: *mut libc::c_int,
) {
    let mut s: libc::c_int = 0;
    if (*w).count as libc::c_int >= W - 3 as libc::c_int + 1 as libc::c_int {
        s = *kdq_shift_int(w);
        let ref mut fresh3 = *cw.offset(s as isize);
        *fresh3 -= 1;
        *rw -= *fresh3;
        if *L > (*w).count as libc::c_int {
            *L -= 1;
            *L;
            let ref mut fresh4 = *cv.offset(s as isize);
            *fresh4 -= 1;
            *rv -= *fresh4;
        }
    }
    kdq_push_int(w, t);
    *L += 1;
    *L;
    let ref mut fresh5 = *cw.offset(t as isize);
    let fresh6 = *fresh5;
    *fresh5 = *fresh5 + 1;
    *rw += fresh6;
    let ref mut fresh7 = *cv.offset(t as isize);
    let fresh8 = *fresh7;
    *fresh7 = *fresh7 + 1;
    *rv += fresh8;
    if *cv.offset(t as isize) * 10 as libc::c_int > T << 1 as libc::c_int {
        loop {
            s = *((*w).a)
                .offset(
                    ((*w)
                        .front()
                        .wrapping_add(((*w).count).wrapping_sub(*L as libc::c_ulong))
                        & (*w).mask) as isize,
                );
            let ref mut fresh9 = *cv.offset(s as isize);
            *fresh9 -= 1;
            *rv -= *fresh9;
            *L -= 1;
            *L;
            if !(s != t) {
                break;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn save_masked_regions(
    mut km: *mut libc::c_void,
    mut res: *mut uint64_v,
    mut P: *mut perf_intv_v,
    mut start: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut saved: libc::c_int = 0 as libc::c_int;
    let mut p: *mut perf_intv_t = 0 as *mut perf_intv_t;
    if (*P).n == 0 as libc::c_int as libc::c_ulong
        || (*((*P).a)
            .offset(((*P).n).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .start >= start
    {
        return;
    }
    p = &mut *((*P).a)
        .offset(((*P).n).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as *mut perf_intv_t;
    if (*res).n != 0 {
        let mut s: libc::c_int = (*((*res).a)
            .offset(((*res).n).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            >> 32 as libc::c_int) as libc::c_int;
        let mut f: libc::c_int = *((*res).a)
            .offset(((*res).n).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as uint32_t as libc::c_int;
        if (*p).start <= f {
            saved = 1 as libc::c_int;
            *((*res).a)
                .offset(
                    ((*res).n).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = (s as uint64_t) << 32 as libc::c_int
                | (if f > (*p).finish { f } else { (*p).finish }) as libc::c_ulong;
        }
    }
    if saved == 0 {
        if (*res).n == (*res).m {
            (*res)
                .m = if (*res).m != 0 {
                (*res).m << 1 as libc::c_int
            } else {
                2 as libc::c_int as libc::c_ulong
            };
            (*res)
                .a = krealloc(
                km,
                (*res).a as *mut libc::c_void,
                (::std::mem::size_of::<uint64_t>() as libc::c_ulong)
                    .wrapping_mul((*res).m),
            ) as *mut uint64_t;
        }
        let fresh10 = (*res).n;
        (*res).n = ((*res).n).wrapping_add(1);
        *((*res).a)
            .offset(
                fresh10 as isize,
            ) = ((*p).start as uint64_t) << 32 as libc::c_int
            | (*p).finish as libc::c_ulong;
    }
    i = ((*P).n).wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    while i >= 0 as libc::c_int && (*((*P).a).offset(i as isize)).start < start {
        i -= 1;
        i;
    }
    (*P).n = (i + 1 as libc::c_int) as size_t;
}
unsafe extern "C" fn find_perfect(
    mut km: *mut libc::c_void,
    mut P: *mut perf_intv_v,
    mut w: *const kdq_int_t,
    mut T: libc::c_int,
    mut start: libc::c_int,
    mut L: libc::c_int,
    mut rv: libc::c_int,
    mut cv: *const libc::c_int,
) {
    let mut c: [libc::c_int; 64] = [0; 64];
    let mut r: libc::c_int = rv;
    let mut i: libc::c_int = 0;
    let mut max_r: libc::c_int = 0 as libc::c_int;
    let mut max_l: libc::c_int = 0 as libc::c_int;
    memcpy(
        c.as_mut_ptr() as *mut libc::c_void,
        cv as *const libc::c_void,
        (((1 as libc::c_int) << ((3 as libc::c_int) << 1 as libc::c_int))
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    i = ((*w).count as libc::c_long - L as libc::c_long
        - 1 as libc::c_int as libc::c_long) as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut j: libc::c_int = 0;
        let mut t: libc::c_int = *((*w).a)
            .offset(
                ((*w).front().wrapping_add(i as libc::c_ulong) & (*w).mask) as isize,
            );
        let mut new_r: libc::c_int = 0;
        let mut new_l: libc::c_int = 0;
        let fresh11 = c[t as usize];
        c[t as usize] = c[t as usize] + 1;
        r += fresh11;
        new_r = r;
        new_l = ((*w).count)
            .wrapping_sub(i as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        if new_r * 10 as libc::c_int > T * new_l {
            j = 0 as libc::c_int;
            while j < (*P).n as libc::c_int
                && (*((*P).a).offset(j as isize)).start >= i + start
            {
                let mut p: *mut perf_intv_t = &mut *((*P).a).offset(j as isize)
                    as *mut perf_intv_t;
                if max_r == 0 as libc::c_int || (*p).r * max_l > max_r * (*p).l {
                    max_r = (*p).r;
                    max_l = (*p).l;
                }
                j += 1;
                j;
            }
            if max_r == 0 as libc::c_int || new_r * max_l >= max_r * new_l {
                max_r = new_r;
                max_l = new_l;
                if (*P).n == (*P).m {
                    if (*P).m < ((*P).n).wrapping_add(1 as libc::c_int as libc::c_ulong)
                    {
                        (*P)
                            .m = ((*P).n)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong);
                        (*P).m = ((*P).m).wrapping_sub(1);
                        (*P).m;
                        (*P).m |= (*P).m >> 1 as libc::c_int;
                        (*P).m |= (*P).m >> 2 as libc::c_int;
                        (*P).m |= (*P).m >> 4 as libc::c_int;
                        (*P).m |= (*P).m >> 8 as libc::c_int;
                        (*P).m |= (*P).m >> 16 as libc::c_int;
                        (*P).m = ((*P).m).wrapping_add(1);
                        (*P).m;
                        (*P)
                            .a = krealloc(
                            km,
                            (*P).a as *mut libc::c_void,
                            (::std::mem::size_of::<perf_intv_t>() as libc::c_ulong)
                                .wrapping_mul((*P).m),
                        ) as *mut perf_intv_t;
                    }
                }
                memmove(
                    &mut *((*P).a).offset((j + 1 as libc::c_int) as isize)
                        as *mut perf_intv_t as *mut libc::c_void,
                    &mut *((*P).a).offset(j as isize) as *mut perf_intv_t
                        as *const libc::c_void,
                    ((*P).n)
                        .wrapping_sub(j as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<perf_intv_t>() as libc::c_ulong,
                        ),
                );
                (*P).n = ((*P).n).wrapping_add(1);
                (*P).n;
                (*((*P).a).offset(j as isize)).start = i + start;
                (*((*P).a).offset(j as isize))
                    .finish = ((*w).count)
                    .wrapping_add((3 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_add(start as libc::c_ulong) as libc::c_int;
                (*((*P).a).offset(j as isize)).r = new_r;
                (*((*P).a).offset(j as isize)).l = new_l;
            }
        }
        i -= 1;
        i;
    }
}
pub unsafe extern "C" fn sdust_core(
    mut seq: *const uint8_t,
    mut l_seq: libc::c_int,
    mut T: libc::c_int,
    mut W: libc::c_int,
    mut n: *mut libc::c_int,
    mut buf: *mut sdust_buf_t,
) -> *const uint64_t {
    let mut rv: libc::c_int = 0 as libc::c_int;
    let mut rw: libc::c_int = 0 as libc::c_int;
    let mut L: libc::c_int = 0 as libc::c_int;
    let mut cv: [libc::c_int; 64] = [0; 64];
    let mut cw: [libc::c_int; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    (*buf).res.n = 0 as libc::c_int as size_t;
    (*buf).P.n = (*buf).res.n;
    (*(*buf).w).count = 0 as libc::c_int as uint64_t;
    (*(*buf).w).set_front((*(*buf).w).count);
    memset(
        cv.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (((1 as libc::c_int) << ((3 as libc::c_int) << 1 as libc::c_int))
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    memset(
        cw.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (((1 as libc::c_int) << ((3 as libc::c_int) << 1 as libc::c_int))
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if l_seq < 0 as libc::c_int {
        l_seq = strlen(seq as *const libc::c_char) as libc::c_int;
    }
    t = 0 as libc::c_int as libc::c_uint;
    l = t as libc::c_int;
    i = l;
    while i <= l_seq {
        let mut b: libc::c_int = if i < l_seq {
            seq_nt4_table[*seq.offset(i as isize) as usize] as libc::c_int
        } else {
            4 as libc::c_int
        };
        if b < 4 as libc::c_int {
            l += 1;
            l;
            t = (t << 2 as libc::c_int | b as libc::c_uint)
                & (((1 as libc::c_int) << ((3 as libc::c_int) << 1 as libc::c_int))
                    - 1 as libc::c_int) as libc::c_uint;
            if l >= 3 as libc::c_int {
                start = (if l - W > 0 as libc::c_int { l - W } else { 0 as libc::c_int })
                    + (i + 1 as libc::c_int - l);
                save_masked_regions((*buf).km, &mut (*buf).res, &mut (*buf).P, start);
                shift_window(
                    t as libc::c_int,
                    (*buf).w,
                    T,
                    W,
                    &mut L,
                    &mut rw,
                    &mut rv,
                    cw.as_mut_ptr(),
                    cv.as_mut_ptr(),
                );
                if rw * 10 as libc::c_int > L * T {
                    find_perfect(
                        (*buf).km,
                        &mut (*buf).P,
                        (*buf).w,
                        T,
                        start,
                        L,
                        rv,
                        cv.as_mut_ptr(),
                    );
                }
            }
        } else {
            start = (if l - W + 1 as libc::c_int > 0 as libc::c_int {
                l - W + 1 as libc::c_int
            } else {
                0 as libc::c_int
            }) + (i + 1 as libc::c_int - l);
            while (*buf).P.n != 0 {
                let fresh12 = start;
                start = start + 1;
                save_masked_regions((*buf).km, &mut (*buf).res, &mut (*buf).P, fresh12);
            }
            t = 0 as libc::c_int as libc::c_uint;
            l = t as libc::c_int;
        }
        i += 1;
        i;
    }
    *n = (*buf).res.n as libc::c_int;
    return (*buf).res.a;
}
pub unsafe extern "C" fn sdust(
    mut km: *mut libc::c_void,
    mut seq: *const uint8_t,
    mut l_seq: libc::c_int,
    mut T: libc::c_int,
    mut W: libc::c_int,
    mut n: *mut libc::c_int,
) -> *mut uint64_t {
    let mut ret: *mut uint64_t = 0 as *mut uint64_t;
    let mut buf: *mut sdust_buf_t = 0 as *mut sdust_buf_t;
    buf = sdust_buf_init(km);
    ret = sdust_core(seq, l_seq, T, W, n, buf) as *mut uint64_t;
    (*buf).res.a = 0 as *mut uint64_t;
    sdust_buf_destroy(buf);
    return ret;
}
