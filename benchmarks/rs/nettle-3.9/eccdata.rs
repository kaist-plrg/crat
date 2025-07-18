use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type ptrdiff_t = libc::c_long;
pub type mp_limb_t = libc::c_ulong;
pub type mp_size_t = libc::c_long;
pub type mp_bitcnt_t = libc::c_ulong;
pub type mp_ptr = *mut mp_limb_t;
pub type mp_srcptr = *const mp_limb_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = [__mpz_struct; 1];
pub type mpz_ptr = *mut __mpz_struct;
pub type mpz_srcptr = *const __mpz_struct;
pub type mpz_div_round_mode = libc::c_uint;
pub const GMP_DIV_TRUNC: mpz_div_round_mode = 2;
pub const GMP_DIV_CEIL: mpz_div_round_mode = 1;
pub const GMP_DIV_FLOOR: mpz_div_round_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gmp_div_inverse {
    pub shift: libc::c_uint,
    pub d1: mp_limb_t,
    pub d0: mp_limb_t,
    pub di: mp_limb_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpn_base_info {
    pub exp: libc::c_uint,
    pub bb: mp_limb_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecc_point {
    pub is_zero: libc::c_int,
    pub x: mpz_t,
    pub y: mpz_t,
}
pub type ecc_type = libc::c_uint;
pub const ECC_TYPE_TWISTED_EDWARDS: ecc_type = 2;
pub const ECC_TYPE_EDWARDS: ecc_type = 1;
pub const ECC_TYPE_WEIERSTRASS: ecc_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecc_curve {
    pub bit_size: libc::c_uint,
    pub pippenger_k: libc::c_uint,
    pub pippenger_c: libc::c_uint,
    pub type_0: ecc_type,
    pub p: mpz_t,
    pub b: mpz_t,
    pub q: mpz_t,
    pub g: ecc_point,
    pub table_size: mp_size_t,
    pub table: *mut ecc_point,
    pub ref_0: *mut ecc_point,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub static mut mp_bits_per_limb: libc::c_int = 0;
unsafe extern "C" fn gmp_die(mut msg: *const libc::c_char) {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, msg);
    abort();
}
unsafe extern "C" fn gmp_default_alloc(mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    if size > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"size > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            290 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void *gmp_default_alloc(size_t)\0"))
                .as_ptr(),
        );
    }
    'c_2520: {
        if size > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"size > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                290 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void *gmp_default_alloc(size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    p = malloc(size);
    if p.is_null() {
        gmp_die(
            b"gmp_default_alloc: Virtual memory exhausted.\0" as *const u8
                as *const libc::c_char,
        );
    }
    return p;
}
unsafe extern "C" fn gmp_default_realloc(
    mut old: *mut libc::c_void,
    mut unused_old_size: size_t,
    mut new_size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = realloc(old, new_size);
    if p.is_null() {
        gmp_die(
            b"gmp_default_realloc: Virtual memory exhausted.\0" as *const u8
                as *const libc::c_char,
        );
    }
    return p;
}
unsafe extern "C" fn gmp_default_free(
    mut p: *mut libc::c_void,
    mut unused_size: size_t,
) {
    free(p);
}
static mut gmp_allocate_func: Option::<
    unsafe extern "C" fn(size_t) -> *mut libc::c_void,
> = unsafe {
    Some(gmp_default_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void)
};
static mut gmp_reallocate_func: Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> *mut libc::c_void,
> = unsafe {
    Some(
        gmp_default_realloc
            as unsafe extern "C" fn(
                *mut libc::c_void,
                size_t,
                size_t,
            ) -> *mut libc::c_void,
    )
};
static mut gmp_free_func: Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> (),
> = unsafe {
    Some(gmp_default_free as unsafe extern "C" fn(*mut libc::c_void, size_t) -> ())
};
pub unsafe extern "C" fn mp_get_memory_functions(
    mut alloc_func: *mut Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    mut realloc_func: *mut Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> *mut libc::c_void,
    >,
    mut free_func: *mut Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> ()>,
) {
    if !alloc_func.is_null() {
        *alloc_func = gmp_allocate_func;
    }
    if !realloc_func.is_null() {
        *realloc_func = gmp_reallocate_func;
    }
    if !free_func.is_null() {
        *free_func = gmp_free_func;
    }
}
pub unsafe extern "C" fn mp_set_memory_functions(
    mut alloc_func: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    mut realloc_func: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> *mut libc::c_void,
    >,
    mut free_func: Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> ()>,
) {
    if alloc_func.is_none() {
        alloc_func = Some(
            gmp_default_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void,
        );
    }
    if realloc_func.is_none() {
        realloc_func = Some(
            gmp_default_realloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    size_t,
                    size_t,
                ) -> *mut libc::c_void,
        );
    }
    if free_func.is_none() {
        free_func = Some(
            gmp_default_free as unsafe extern "C" fn(*mut libc::c_void, size_t) -> (),
        );
    }
    gmp_allocate_func = alloc_func;
    gmp_reallocate_func = realloc_func;
    gmp_free_func = free_func;
}
unsafe extern "C" fn gmp_xalloc_limbs(mut size: mp_size_t) -> mp_ptr {
    return (Some(gmp_allocate_func.unwrap()))
        .unwrap()(
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mp_limb_t>() as libc::c_ulong),
    ) as mp_ptr;
}
unsafe extern "C" fn gmp_xrealloc_limbs(mut old: mp_ptr, mut size: mp_size_t) -> mp_ptr {
    if size > 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"size > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            366 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"mp_ptr gmp_xrealloc_limbs(mp_ptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_5700: {
        if size > 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"size > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                366 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"mp_ptr gmp_xrealloc_limbs(mp_ptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    return (Some(gmp_reallocate_func.unwrap()))
        .unwrap()(
        old as *mut libc::c_void,
        0 as libc::c_int as size_t,
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mp_limb_t>() as libc::c_ulong),
    ) as mp_ptr;
}
pub unsafe extern "C" fn mpn_copyi(mut d: mp_ptr, mut s: mp_srcptr, mut n: mp_size_t) {
    let mut i: mp_size_t = 0;
    i = 0 as libc::c_int as mp_size_t;
    while i < n {
        *d.offset(i as isize) = *s.offset(i as isize);
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn mpn_copyd(mut d: mp_ptr, mut s: mp_srcptr, mut n: mp_size_t) {
    loop {
        n -= 1;
        if !(n >= 0 as libc::c_int as libc::c_long) {
            break;
        }
        *d.offset(n as isize) = *s.offset(n as isize);
    };
}
pub unsafe extern "C" fn mpn_cmp(
    mut ap: mp_srcptr,
    mut bp: mp_srcptr,
    mut n: mp_size_t,
) -> libc::c_int {
    loop {
        n -= 1;
        if !(n >= 0 as libc::c_int as libc::c_long) {
            break;
        }
        if *ap.offset(n as isize) != *bp.offset(n as isize) {
            return if *ap.offset(n as isize) > *bp.offset(n as isize) {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            };
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mpn_cmp4(
    mut ap: mp_srcptr,
    mut an: mp_size_t,
    mut bp: mp_srcptr,
    mut bn: mp_size_t,
) -> libc::c_int {
    if an != bn {
        return if an < bn { -(1 as libc::c_int) } else { 1 as libc::c_int }
    } else {
        return mpn_cmp(ap, bp, an)
    };
}
unsafe extern "C" fn mpn_normalized_size(
    mut xp: mp_srcptr,
    mut n: mp_size_t,
) -> mp_size_t {
    while n > 0 as libc::c_int as libc::c_long
        && *xp.offset((n - 1 as libc::c_int as libc::c_long) as isize)
            == 0 as libc::c_int as libc::c_ulong
    {
        n -= 1;
        n;
    }
    return n;
}
pub unsafe extern "C" fn mpn_zero_p(mut rp: mp_srcptr, mut n: mp_size_t) -> libc::c_int {
    return (mpn_normalized_size(rp, n) == 0 as libc::c_int as libc::c_long)
        as libc::c_int;
}
pub unsafe extern "C" fn mpn_zero(mut rp: mp_ptr, mut n: mp_size_t) {
    loop {
        n -= 1;
        if !(n >= 0 as libc::c_int as libc::c_long) {
            break;
        }
        *rp.offset(n as isize) = 0 as libc::c_int as mp_limb_t;
    };
}
pub unsafe extern "C" fn mpn_add_1(
    mut rp: mp_ptr,
    mut ap: mp_srcptr,
    mut n: mp_size_t,
    mut b: mp_limb_t,
) -> mp_limb_t {
    let mut i: mp_size_t = 0;
    if n > 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"n > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            434 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"mp_limb_t mpn_add_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_2888: {
        if n > 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                434 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"mp_limb_t mpn_add_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int as mp_size_t;
    loop {
        let mut r: mp_limb_t = (*ap.offset(i as isize)).wrapping_add(b);
        b = (r < b) as libc::c_int as mp_limb_t;
        *rp.offset(i as isize) = r;
        i += 1;
        if !(i < n) {
            break;
        }
    }
    return b;
}
pub unsafe extern "C" fn mpn_add_n(
    mut rp: mp_ptr,
    mut ap: mp_srcptr,
    mut bp: mp_srcptr,
    mut n: mp_size_t,
) -> mp_limb_t {
    let mut i: mp_size_t = 0;
    let mut cy: mp_limb_t = 0;
    i = 0 as libc::c_int as mp_size_t;
    cy = 0 as libc::c_int as mp_limb_t;
    while i < n {
        let mut a: mp_limb_t = 0;
        let mut b: mp_limb_t = 0;
        let mut r: mp_limb_t = 0;
        a = *ap.offset(i as isize);
        b = *bp.offset(i as isize);
        r = a.wrapping_add(cy);
        cy = (r < cy) as libc::c_int as mp_limb_t;
        r = (r as libc::c_ulong).wrapping_add(b) as mp_limb_t as mp_limb_t;
        cy = (cy as libc::c_ulong).wrapping_add((r < b) as libc::c_int as libc::c_ulong)
            as mp_limb_t as mp_limb_t;
        *rp.offset(i as isize) = r;
        i += 1;
        i;
    }
    return cy;
}
pub unsafe extern "C" fn mpn_add(
    mut rp: mp_ptr,
    mut ap: mp_srcptr,
    mut an: mp_size_t,
    mut bp: mp_srcptr,
    mut bn: mp_size_t,
) -> mp_limb_t {
    let mut cy: mp_limb_t = 0;
    if an >= bn {} else {
        __assert_fail(
            b"an >= bn\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            472 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"mp_limb_t mpn_add(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3084: {
        if an >= bn {} else {
            __assert_fail(
                b"an >= bn\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                472 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 70],
                    &[libc::c_char; 70],
                >(
                    b"mp_limb_t mpn_add(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    cy = mpn_add_n(rp, ap, bp, bn);
    if an > bn {
        cy = mpn_add_1(rp.offset(bn as isize), ap.offset(bn as isize), an - bn, cy);
    }
    return cy;
}
pub unsafe extern "C" fn mpn_sub_1(
    mut rp: mp_ptr,
    mut ap: mp_srcptr,
    mut n: mp_size_t,
    mut b: mp_limb_t,
) -> mp_limb_t {
    let mut i: mp_size_t = 0;
    if n > 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"n > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            485 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"mp_limb_t mpn_sub_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_3180: {
        if n > 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                485 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"mp_limb_t mpn_sub_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int as mp_size_t;
    loop {
        let mut a: mp_limb_t = *ap.offset(i as isize);
        let mut cy: mp_limb_t = (a < b) as libc::c_int as mp_limb_t;
        *rp.offset(i as isize) = a.wrapping_sub(b);
        b = cy;
        i += 1;
        if !(i < n) {
            break;
        }
    }
    return b;
}
pub unsafe extern "C" fn mpn_sub_n(
    mut rp: mp_ptr,
    mut ap: mp_srcptr,
    mut bp: mp_srcptr,
    mut n: mp_size_t,
) -> mp_limb_t {
    let mut i: mp_size_t = 0;
    let mut cy: mp_limb_t = 0;
    i = 0 as libc::c_int as mp_size_t;
    cy = 0 as libc::c_int as mp_limb_t;
    while i < n {
        let mut a: mp_limb_t = 0;
        let mut b: mp_limb_t = 0;
        a = *ap.offset(i as isize);
        b = *bp.offset(i as isize);
        b = (b as libc::c_ulong).wrapping_add(cy) as mp_limb_t as mp_limb_t;
        cy = (b < cy) as libc::c_int as mp_limb_t;
        cy = (cy as libc::c_ulong).wrapping_add((a < b) as libc::c_int as libc::c_ulong)
            as mp_limb_t as mp_limb_t;
        *rp.offset(i as isize) = a.wrapping_sub(b);
        i += 1;
        i;
    }
    return cy;
}
pub unsafe extern "C" fn mpn_sub(
    mut rp: mp_ptr,
    mut ap: mp_srcptr,
    mut an: mp_size_t,
    mut bp: mp_srcptr,
    mut bn: mp_size_t,
) -> mp_limb_t {
    let mut cy: mp_limb_t = 0;
    if an >= bn {} else {
        __assert_fail(
            b"an >= bn\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            524 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"mp_limb_t mpn_sub(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3364: {
        if an >= bn {} else {
            __assert_fail(
                b"an >= bn\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                524 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 70],
                    &[libc::c_char; 70],
                >(
                    b"mp_limb_t mpn_sub(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    cy = mpn_sub_n(rp, ap, bp, bn);
    if an > bn {
        cy = mpn_sub_1(rp.offset(bn as isize), ap.offset(bn as isize), an - bn, cy);
    }
    return cy;
}
pub unsafe extern "C" fn mpn_mul_1(
    mut rp: mp_ptr,
    mut up: mp_srcptr,
    mut n: mp_size_t,
    mut vl: mp_limb_t,
) -> mp_limb_t {
    let mut ul: mp_limb_t = 0;
    let mut cl: mp_limb_t = 0;
    let mut hpl: mp_limb_t = 0;
    let mut lpl: mp_limb_t = 0;
    if n >= 1 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"n >= 1\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            537 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"mp_limb_t mpn_mul_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_3801: {
        if n >= 1 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"n >= 1\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                537 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"mp_limb_t mpn_mul_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    cl = 0 as libc::c_int as mp_limb_t;
    loop {
        let fresh0 = up;
        up = up.offset(1);
        ul = *fresh0;
        let mut LOCAL_GMP_LIMB_BITS: libc::c_int = (::std::mem::size_of::<mp_limb_t>()
            as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
        if (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            >= (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
        {
            let mut __ww: libc::c_uint = (ul as libc::c_uint as libc::c_ulong)
                .wrapping_mul(vl) as libc::c_uint;
            lpl = __ww as mp_limb_t;
            hpl = (__ww >> LOCAL_GMP_LIMB_BITS) as mp_limb_t;
        } else if (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            >= (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
        {
            let mut __ww_0: libc::c_ulong = ul.wrapping_mul(vl);
            lpl = __ww_0;
            hpl = __ww_0 >> LOCAL_GMP_LIMB_BITS;
        } else {
            let mut __x0: mp_limb_t = 0;
            let mut __x1: mp_limb_t = 0;
            let mut __x2: mp_limb_t = 0;
            let mut __x3: mp_limb_t = 0;
            let mut __ul: libc::c_uint = 0;
            let mut __vl: libc::c_uint = 0;
            let mut __uh: libc::c_uint = 0;
            let mut __vh: libc::c_uint = 0;
            let mut __u: mp_limb_t = ul;
            let mut __v: mp_limb_t = vl;
            __ul = (__u
                & ((1 as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __uh = (__u
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __vl = (__v
                & ((1 as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __vh = (__v
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __x0 = (__ul as mp_limb_t).wrapping_mul(__vl as libc::c_ulong);
            __x1 = (__ul as mp_limb_t).wrapping_mul(__vh as libc::c_ulong);
            __x2 = (__uh as mp_limb_t).wrapping_mul(__vl as libc::c_ulong);
            __x3 = (__uh as mp_limb_t).wrapping_mul(__vh as libc::c_ulong);
            __x1 = (__x1 as libc::c_ulong)
                .wrapping_add(
                    __x0
                        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong),
                ) as mp_limb_t as mp_limb_t;
            __x1 = (__x1 as libc::c_ulong).wrapping_add(__x2) as mp_limb_t as mp_limb_t;
            if __x1 < __x2 {
                __x3 = (__x3 as libc::c_ulong)
                    .wrapping_add(
                        (1 as libc::c_int as mp_limb_t)
                            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong),
                    ) as mp_limb_t as mp_limb_t;
            }
            hpl = __x3
                .wrapping_add(
                    __x1
                        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong),
                );
            lpl = (__x1
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong))
                .wrapping_add(
                    __x0
                        & ((1 as libc::c_int as mp_limb_t)
                            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
        }
        lpl = (lpl as libc::c_ulong).wrapping_add(cl) as mp_limb_t as mp_limb_t;
        cl = ((lpl < cl) as libc::c_int as libc::c_ulong).wrapping_add(hpl);
        let fresh1 = rp;
        rp = rp.offset(1);
        *fresh1 = lpl;
        n -= 1;
        if !(n != 0 as libc::c_int as libc::c_long) {
            break;
        }
    }
    return cl;
}
pub unsafe extern "C" fn mpn_addmul_1(
    mut rp: mp_ptr,
    mut up: mp_srcptr,
    mut n: mp_size_t,
    mut vl: mp_limb_t,
) -> mp_limb_t {
    let mut ul: mp_limb_t = 0;
    let mut cl: mp_limb_t = 0;
    let mut hpl: mp_limb_t = 0;
    let mut lpl: mp_limb_t = 0;
    let mut rl: mp_limb_t = 0;
    if n >= 1 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"n >= 1\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            560 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"mp_limb_t mpn_addmul_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_4261: {
        if n >= 1 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"n >= 1\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                560 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"mp_limb_t mpn_addmul_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    cl = 0 as libc::c_int as mp_limb_t;
    loop {
        let fresh2 = up;
        up = up.offset(1);
        ul = *fresh2;
        let mut LOCAL_GMP_LIMB_BITS: libc::c_int = (::std::mem::size_of::<mp_limb_t>()
            as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
        if (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            >= (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
        {
            let mut __ww: libc::c_uint = (ul as libc::c_uint as libc::c_ulong)
                .wrapping_mul(vl) as libc::c_uint;
            lpl = __ww as mp_limb_t;
            hpl = (__ww >> LOCAL_GMP_LIMB_BITS) as mp_limb_t;
        } else if (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            >= (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
        {
            let mut __ww_0: libc::c_ulong = ul.wrapping_mul(vl);
            lpl = __ww_0;
            hpl = __ww_0 >> LOCAL_GMP_LIMB_BITS;
        } else {
            let mut __x0: mp_limb_t = 0;
            let mut __x1: mp_limb_t = 0;
            let mut __x2: mp_limb_t = 0;
            let mut __x3: mp_limb_t = 0;
            let mut __ul: libc::c_uint = 0;
            let mut __vl: libc::c_uint = 0;
            let mut __uh: libc::c_uint = 0;
            let mut __vh: libc::c_uint = 0;
            let mut __u: mp_limb_t = ul;
            let mut __v: mp_limb_t = vl;
            __ul = (__u
                & ((1 as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __uh = (__u
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __vl = (__v
                & ((1 as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __vh = (__v
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __x0 = (__ul as mp_limb_t).wrapping_mul(__vl as libc::c_ulong);
            __x1 = (__ul as mp_limb_t).wrapping_mul(__vh as libc::c_ulong);
            __x2 = (__uh as mp_limb_t).wrapping_mul(__vl as libc::c_ulong);
            __x3 = (__uh as mp_limb_t).wrapping_mul(__vh as libc::c_ulong);
            __x1 = (__x1 as libc::c_ulong)
                .wrapping_add(
                    __x0
                        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong),
                ) as mp_limb_t as mp_limb_t;
            __x1 = (__x1 as libc::c_ulong).wrapping_add(__x2) as mp_limb_t as mp_limb_t;
            if __x1 < __x2 {
                __x3 = (__x3 as libc::c_ulong)
                    .wrapping_add(
                        (1 as libc::c_int as mp_limb_t)
                            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong),
                    ) as mp_limb_t as mp_limb_t;
            }
            hpl = __x3
                .wrapping_add(
                    __x1
                        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong),
                );
            lpl = (__x1
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong))
                .wrapping_add(
                    __x0
                        & ((1 as libc::c_int as mp_limb_t)
                            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
        }
        lpl = (lpl as libc::c_ulong).wrapping_add(cl) as mp_limb_t as mp_limb_t;
        cl = ((lpl < cl) as libc::c_int as libc::c_ulong).wrapping_add(hpl);
        rl = *rp;
        lpl = rl.wrapping_add(lpl);
        cl = (cl as libc::c_ulong)
            .wrapping_add((lpl < rl) as libc::c_int as libc::c_ulong) as mp_limb_t
            as mp_limb_t;
        let fresh3 = rp;
        rp = rp.offset(1);
        *fresh3 = lpl;
        n -= 1;
        if !(n != 0 as libc::c_int as libc::c_long) {
            break;
        }
    }
    return cl;
}
pub unsafe extern "C" fn mpn_submul_1(
    mut rp: mp_ptr,
    mut up: mp_srcptr,
    mut n: mp_size_t,
    mut vl: mp_limb_t,
) -> mp_limb_t {
    let mut ul: mp_limb_t = 0;
    let mut cl: mp_limb_t = 0;
    let mut hpl: mp_limb_t = 0;
    let mut lpl: mp_limb_t = 0;
    let mut rl: mp_limb_t = 0;
    if n >= 1 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"n >= 1\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            586 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"mp_limb_t mpn_submul_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_4721: {
        if n >= 1 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"n >= 1\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                586 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"mp_limb_t mpn_submul_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    cl = 0 as libc::c_int as mp_limb_t;
    loop {
        let fresh4 = up;
        up = up.offset(1);
        ul = *fresh4;
        let mut LOCAL_GMP_LIMB_BITS: libc::c_int = (::std::mem::size_of::<mp_limb_t>()
            as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
        if (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            >= (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
        {
            let mut __ww: libc::c_uint = (ul as libc::c_uint as libc::c_ulong)
                .wrapping_mul(vl) as libc::c_uint;
            lpl = __ww as mp_limb_t;
            hpl = (__ww >> LOCAL_GMP_LIMB_BITS) as mp_limb_t;
        } else if (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            >= (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
        {
            let mut __ww_0: libc::c_ulong = ul.wrapping_mul(vl);
            lpl = __ww_0;
            hpl = __ww_0 >> LOCAL_GMP_LIMB_BITS;
        } else {
            let mut __x0: mp_limb_t = 0;
            let mut __x1: mp_limb_t = 0;
            let mut __x2: mp_limb_t = 0;
            let mut __x3: mp_limb_t = 0;
            let mut __ul: libc::c_uint = 0;
            let mut __vl: libc::c_uint = 0;
            let mut __uh: libc::c_uint = 0;
            let mut __vh: libc::c_uint = 0;
            let mut __u: mp_limb_t = ul;
            let mut __v: mp_limb_t = vl;
            __ul = (__u
                & ((1 as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __uh = (__u
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __vl = (__v
                & ((1 as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __vh = (__v
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __x0 = (__ul as mp_limb_t).wrapping_mul(__vl as libc::c_ulong);
            __x1 = (__ul as mp_limb_t).wrapping_mul(__vh as libc::c_ulong);
            __x2 = (__uh as mp_limb_t).wrapping_mul(__vl as libc::c_ulong);
            __x3 = (__uh as mp_limb_t).wrapping_mul(__vh as libc::c_ulong);
            __x1 = (__x1 as libc::c_ulong)
                .wrapping_add(
                    __x0
                        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong),
                ) as mp_limb_t as mp_limb_t;
            __x1 = (__x1 as libc::c_ulong).wrapping_add(__x2) as mp_limb_t as mp_limb_t;
            if __x1 < __x2 {
                __x3 = (__x3 as libc::c_ulong)
                    .wrapping_add(
                        (1 as libc::c_int as mp_limb_t)
                            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong),
                    ) as mp_limb_t as mp_limb_t;
            }
            hpl = __x3
                .wrapping_add(
                    __x1
                        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong),
                );
            lpl = (__x1
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong))
                .wrapping_add(
                    __x0
                        & ((1 as libc::c_int as mp_limb_t)
                            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
        }
        lpl = (lpl as libc::c_ulong).wrapping_add(cl) as mp_limb_t as mp_limb_t;
        cl = ((lpl < cl) as libc::c_int as libc::c_ulong).wrapping_add(hpl);
        rl = *rp;
        lpl = rl.wrapping_sub(lpl);
        cl = (cl as libc::c_ulong)
            .wrapping_add((lpl > rl) as libc::c_int as libc::c_ulong) as mp_limb_t
            as mp_limb_t;
        let fresh5 = rp;
        rp = rp.offset(1);
        *fresh5 = lpl;
        n -= 1;
        if !(n != 0 as libc::c_int as libc::c_long) {
            break;
        }
    }
    return cl;
}
pub unsafe extern "C" fn mpn_mul(
    mut rp: mp_ptr,
    mut up: mp_srcptr,
    mut un: mp_size_t,
    mut vp: mp_srcptr,
    mut vn: mp_size_t,
) -> mp_limb_t {
    if un >= vn {} else {
        __assert_fail(
            b"un >= vn\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            610 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"mp_limb_t mpn_mul(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5050: {
        if un >= vn {} else {
            __assert_fail(
                b"un >= vn\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                610 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 70],
                    &[libc::c_char; 70],
                >(
                    b"mp_limb_t mpn_mul(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if vn >= 1 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"vn >= 1\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            611 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"mp_limb_t mpn_mul(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5011: {
        if vn >= 1 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"vn >= 1\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                611 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 70],
                    &[libc::c_char; 70],
                >(
                    b"mp_limb_t mpn_mul(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !(rp.offset((un + vn) as isize) > up as mp_ptr
        && up.offset(un as isize) > rp as mp_srcptr)
    {} else {
        __assert_fail(
            b"!GMP_MPN_OVERLAP_P(rp, un + vn, up, un)\0" as *const u8
                as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            612 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"mp_limb_t mpn_mul(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4923: {
        if !(rp.offset((un + vn) as isize) > up as mp_ptr
            && up.offset(un as isize) > rp as mp_srcptr)
        {} else {
            __assert_fail(
                b"!GMP_MPN_OVERLAP_P(rp, un + vn, up, un)\0" as *const u8
                    as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                612 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 70],
                    &[libc::c_char; 70],
                >(
                    b"mp_limb_t mpn_mul(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !(rp.offset((un + vn) as isize) > vp as mp_ptr
        && vp.offset(vn as isize) > rp as mp_srcptr)
    {} else {
        __assert_fail(
            b"!GMP_MPN_OVERLAP_P(rp, un + vn, vp, vn)\0" as *const u8
                as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            613 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"mp_limb_t mpn_mul(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4834: {
        if !(rp.offset((un + vn) as isize) > vp as mp_ptr
            && vp.offset(vn as isize) > rp as mp_srcptr)
        {} else {
            __assert_fail(
                b"!GMP_MPN_OVERLAP_P(rp, un + vn, vp, vn)\0" as *const u8
                    as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                613 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 70],
                    &[libc::c_char; 70],
                >(
                    b"mp_limb_t mpn_mul(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    *rp
        .offset(
            un as isize,
        ) = mpn_mul_1(rp, up, un, *vp.offset(0 as libc::c_int as isize));
    loop {
        vn -= 1;
        if !(vn >= 1 as libc::c_int as libc::c_long) {
            break;
        }
        rp = rp.offset(1 as libc::c_int as isize);
        vp = vp.offset(1 as libc::c_int as isize);
        *rp
            .offset(
                un as isize,
            ) = mpn_addmul_1(rp, up, un, *vp.offset(0 as libc::c_int as isize));
    }
    return *rp.offset(un as isize);
}
pub unsafe extern "C" fn mpn_mul_n(
    mut rp: mp_ptr,
    mut ap: mp_srcptr,
    mut bp: mp_srcptr,
    mut n: mp_size_t,
) {
    mpn_mul(rp, ap, n, bp, n);
}
pub unsafe extern "C" fn mpn_sqr(mut rp: mp_ptr, mut ap: mp_srcptr, mut n: mp_size_t) {
    mpn_mul(rp, ap, n, ap, n);
}
pub unsafe extern "C" fn mpn_lshift(
    mut rp: mp_ptr,
    mut up: mp_srcptr,
    mut n: mp_size_t,
    mut cnt: libc::c_uint,
) -> mp_limb_t {
    let mut high_limb: mp_limb_t = 0;
    let mut low_limb: mp_limb_t = 0;
    let mut tnc: libc::c_uint = 0;
    let mut retval: mp_limb_t = 0;
    if n >= 1 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"n >= 1\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            651 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"mp_limb_t mpn_lshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_9650: {
        if n >= 1 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"n >= 1\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                651 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"mp_limb_t mpn_lshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    if cnt >= 1 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"cnt >= 1\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            652 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"mp_limb_t mpn_lshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_9612: {
        if cnt >= 1 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"cnt >= 1\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                652 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"mp_limb_t mpn_lshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    if (cnt as libc::c_ulong)
        < (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {} else {
        __assert_fail(
            b"cnt < GMP_LIMB_BITS\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            653 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"mp_limb_t mpn_lshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_9566: {
        if (cnt as libc::c_ulong)
            < (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {} else {
            __assert_fail(
                b"cnt < GMP_LIMB_BITS\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                653 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"mp_limb_t mpn_lshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    up = up.offset(n as isize);
    rp = rp.offset(n as isize);
    tnc = (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(cnt as libc::c_ulong) as libc::c_uint;
    up = up.offset(-1);
    low_limb = *up;
    retval = low_limb >> tnc;
    high_limb = low_limb << cnt;
    loop {
        n -= 1;
        if !(n != 0 as libc::c_int as libc::c_long) {
            break;
        }
        up = up.offset(-1);
        low_limb = *up;
        rp = rp.offset(-1);
        *rp = high_limb | low_limb >> tnc;
        high_limb = low_limb << cnt;
    }
    rp = rp.offset(-1);
    *rp = high_limb;
    return retval;
}
pub unsafe extern "C" fn mpn_rshift(
    mut rp: mp_ptr,
    mut up: mp_srcptr,
    mut n: mp_size_t,
    mut cnt: libc::c_uint,
) -> mp_limb_t {
    let mut high_limb: mp_limb_t = 0;
    let mut low_limb: mp_limb_t = 0;
    let mut tnc: libc::c_uint = 0;
    let mut retval: mp_limb_t = 0;
    if n >= 1 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"n >= 1\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            681 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"mp_limb_t mpn_rshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_7927: {
        if n >= 1 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"n >= 1\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                681 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"mp_limb_t mpn_rshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    if cnt >= 1 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"cnt >= 1\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            682 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"mp_limb_t mpn_rshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_7889: {
        if cnt >= 1 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"cnt >= 1\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                682 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"mp_limb_t mpn_rshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    if (cnt as libc::c_ulong)
        < (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {} else {
        __assert_fail(
            b"cnt < GMP_LIMB_BITS\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            683 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"mp_limb_t mpn_rshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_7841: {
        if (cnt as libc::c_ulong)
            < (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {} else {
            __assert_fail(
                b"cnt < GMP_LIMB_BITS\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                683 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"mp_limb_t mpn_rshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    tnc = (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(cnt as libc::c_ulong) as libc::c_uint;
    let fresh6 = up;
    up = up.offset(1);
    high_limb = *fresh6;
    retval = high_limb << tnc;
    low_limb = high_limb >> cnt;
    loop {
        n -= 1;
        if !(n != 0 as libc::c_int as libc::c_long) {
            break;
        }
        let fresh7 = up;
        up = up.offset(1);
        high_limb = *fresh7;
        let fresh8 = rp;
        rp = rp.offset(1);
        *fresh8 = low_limb | high_limb << tnc;
        low_limb = high_limb >> cnt;
    }
    *rp = low_limb;
    return retval;
}
unsafe extern "C" fn mpn_common_scan(
    mut limb: mp_limb_t,
    mut i: mp_size_t,
    mut up: mp_srcptr,
    mut un: mp_size_t,
    mut ux: mp_limb_t,
) -> mp_bitcnt_t {
    let mut cnt: libc::c_uint = 0;
    if ux == 0 as libc::c_int as libc::c_ulong || ux == !(0 as libc::c_int as mp_limb_t)
    {} else {
        __assert_fail(
            b"ux == 0 || ux == GMP_LIMB_MAX\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            707 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 83],
                &[libc::c_char; 83],
            >(
                b"mp_bitcnt_t mpn_common_scan(mp_limb_t, mp_size_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_16441: {
        if ux == 0 as libc::c_int as libc::c_ulong
            || ux == !(0 as libc::c_int as mp_limb_t)
        {} else {
            __assert_fail(
                b"ux == 0 || ux == GMP_LIMB_MAX\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                707 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"mp_bitcnt_t mpn_common_scan(mp_limb_t, mp_size_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if 0 as libc::c_int as libc::c_long <= i && i <= un {} else {
        __assert_fail(
            b"0 <= i && i <= un\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            708 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 83],
                &[libc::c_char; 83],
            >(
                b"mp_bitcnt_t mpn_common_scan(mp_limb_t, mp_size_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_16389: {
        if 0 as libc::c_int as libc::c_long <= i && i <= un {} else {
            __assert_fail(
                b"0 <= i && i <= un\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                708 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"mp_bitcnt_t mpn_common_scan(mp_limb_t, mp_size_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while limb == 0 as libc::c_int as libc::c_ulong {
        i += 1;
        i;
        if i == un {
            return if ux == 0 as libc::c_int as libc::c_ulong {
                !(0 as libc::c_int as mp_bitcnt_t)
            } else {
                (un as libc::c_ulong)
                    .wrapping_mul(
                        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    )
            };
        }
        limb = ux ^ *up.offset(i as isize);
    }
    let mut __ctz_x: mp_limb_t = limb;
    let mut __ctz_c: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut __clz_x: mp_limb_t = __ctz_x & __ctz_x.wrapping_neg();
    let mut __clz_c: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut LOCAL_SHIFT_BITS: libc::c_int = 8 as libc::c_int;
    if (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        > LOCAL_SHIFT_BITS as libc::c_ulong
    {
        while __clz_x
            & (0xff as libc::c_int as mp_limb_t)
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(8 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        {
            __clz_x <<= LOCAL_SHIFT_BITS;
            __clz_c = __clz_c.wrapping_add(8 as libc::c_int as libc::c_uint);
        }
    }
    while __clz_x
        & (1 as libc::c_int as mp_limb_t)
            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        __clz_x <<= 1 as libc::c_int;
        __clz_c = __clz_c.wrapping_add(1);
        __clz_c;
    }
    __ctz_c = __clz_c;
    cnt = (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_sub(__ctz_c as libc::c_ulong) as libc::c_uint;
    return (i as mp_bitcnt_t)
        .wrapping_mul(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(cnt as libc::c_ulong);
}
pub unsafe extern "C" fn mpn_scan1(
    mut ptr: mp_srcptr,
    mut bit: mp_bitcnt_t,
) -> mp_bitcnt_t {
    let mut i: mp_size_t = 0;
    i = bit
        .wrapping_div(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as mp_size_t;
    return mpn_common_scan(
        *ptr.offset(i as isize)
            & !(0 as libc::c_int as mp_limb_t)
                << bit
                    .wrapping_rem(
                        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ),
        i,
        ptr,
        i,
        0 as libc::c_int as mp_limb_t,
    );
}
pub unsafe extern "C" fn mpn_scan0(
    mut ptr: mp_srcptr,
    mut bit: mp_bitcnt_t,
) -> mp_bitcnt_t {
    let mut i: mp_size_t = 0;
    i = bit
        .wrapping_div(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as mp_size_t;
    return mpn_common_scan(
        !*ptr.offset(i as isize)
            & !(0 as libc::c_int as mp_limb_t)
                << bit
                    .wrapping_rem(
                        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ),
        i,
        ptr,
        i,
        !(0 as libc::c_int as mp_limb_t),
    );
}
pub unsafe extern "C" fn mpn_com(mut rp: mp_ptr, mut up: mp_srcptr, mut n: mp_size_t) {
    loop {
        n -= 1;
        if !(n >= 0 as libc::c_int as libc::c_long) {
            break;
        }
        let fresh9 = up;
        up = up.offset(1);
        let fresh10 = rp;
        rp = rp.offset(1);
        *fresh10 = !*fresh9;
    };
}
pub unsafe extern "C" fn mpn_neg(
    mut rp: mp_ptr,
    mut up: mp_srcptr,
    mut n: mp_size_t,
) -> mp_limb_t {
    while *up == 0 as libc::c_int as libc::c_ulong {
        *rp = 0 as libc::c_int as mp_limb_t;
        n -= 1;
        if n == 0 {
            return 0 as libc::c_int as mp_limb_t;
        }
        up = up.offset(1);
        up;
        rp = rp.offset(1);
        rp;
    }
    *rp = (*up).wrapping_neg();
    rp = rp.offset(1);
    up = up.offset(1);
    n -= 1;
    mpn_com(rp, up, n);
    return 1 as libc::c_int as mp_limb_t;
}
pub unsafe extern "C" fn mpn_invert_3by2(
    mut u1: mp_limb_t,
    mut u0: mp_limb_t,
) -> mp_limb_t {
    let mut r: mp_limb_t = 0;
    let mut m: mp_limb_t = 0;
    let mut p: mp_limb_t = 0;
    let mut ql: mp_limb_t = 0;
    let mut ul: libc::c_uint = 0;
    let mut uh: libc::c_uint = 0;
    let mut qh: libc::c_uint = 0;
    ul = (u1
        & ((1 as libc::c_int as mp_limb_t)
            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_uint;
    uh = (u1
        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)) as libc::c_uint;
    qh = (u1 ^ !(0 as libc::c_int as mp_limb_t)).wrapping_div(uh as libc::c_ulong)
        as libc::c_uint;
    r = (!u1).wrapping_sub((qh as mp_limb_t).wrapping_mul(uh as libc::c_ulong))
        << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
        | ((1 as libc::c_int as mp_limb_t)
            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    p = (qh as mp_limb_t).wrapping_mul(ul as libc::c_ulong);
    if r < p {
        qh = qh.wrapping_sub(1);
        qh;
        r = (r as libc::c_ulong).wrapping_add(u1) as mp_limb_t as mp_limb_t;
        if r >= u1 {
            if r < p {
                qh = qh.wrapping_sub(1);
                qh;
                r = (r as libc::c_ulong).wrapping_add(u1) as mp_limb_t as mp_limb_t;
            }
        }
    }
    r = (r as libc::c_ulong).wrapping_sub(p) as mp_limb_t as mp_limb_t;
    p = (r
        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong))
        .wrapping_mul(qh as libc::c_ulong)
        .wrapping_add(r);
    ql = (p
        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    r = (r
        << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong))
        .wrapping_add(
            ((1 as libc::c_int as mp_limb_t)
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
        .wrapping_sub(ql.wrapping_mul(u1));
    if r
        >= !(0 as libc::c_int as mp_limb_t)
            & p
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        ql = ql.wrapping_sub(1);
        ql;
        r = (r as libc::c_ulong).wrapping_add(u1) as mp_limb_t as mp_limb_t;
    }
    m = ((qh as mp_limb_t)
        << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong))
        .wrapping_add(ql);
    if r >= u1 {
        m = m.wrapping_add(1);
        m;
        r = (r as libc::c_ulong).wrapping_sub(u1) as mp_limb_t as mp_limb_t;
    }
    if u0 > 0 as libc::c_int as libc::c_ulong {
        let mut th: mp_limb_t = 0;
        let mut tl: mp_limb_t = 0;
        r = !r;
        r = (r as libc::c_ulong).wrapping_add(u0) as mp_limb_t as mp_limb_t;
        if r < u0 {
            m = m.wrapping_sub(1);
            m;
            if r >= u1 {
                m = m.wrapping_sub(1);
                m;
                r = (r as libc::c_ulong).wrapping_sub(u1) as mp_limb_t as mp_limb_t;
            }
            r = (r as libc::c_ulong).wrapping_sub(u1) as mp_limb_t as mp_limb_t;
        }
        let mut LOCAL_GMP_LIMB_BITS: libc::c_int = (::std::mem::size_of::<mp_limb_t>()
            as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
        if (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            >= (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
        {
            let mut __ww: libc::c_uint = (u0 as libc::c_uint as libc::c_ulong)
                .wrapping_mul(m) as libc::c_uint;
            tl = __ww as mp_limb_t;
            th = (__ww >> LOCAL_GMP_LIMB_BITS) as mp_limb_t;
        } else if (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            >= (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
        {
            let mut __ww_0: libc::c_ulong = u0.wrapping_mul(m);
            tl = __ww_0;
            th = __ww_0 >> LOCAL_GMP_LIMB_BITS;
        } else {
            let mut __x0: mp_limb_t = 0;
            let mut __x1: mp_limb_t = 0;
            let mut __x2: mp_limb_t = 0;
            let mut __x3: mp_limb_t = 0;
            let mut __ul: libc::c_uint = 0;
            let mut __vl: libc::c_uint = 0;
            let mut __uh: libc::c_uint = 0;
            let mut __vh: libc::c_uint = 0;
            let mut __u: mp_limb_t = u0;
            let mut __v: mp_limb_t = m;
            __ul = (__u
                & ((1 as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __uh = (__u
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __vl = (__v
                & ((1 as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __vh = (__v
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __x0 = (__ul as mp_limb_t).wrapping_mul(__vl as libc::c_ulong);
            __x1 = (__ul as mp_limb_t).wrapping_mul(__vh as libc::c_ulong);
            __x2 = (__uh as mp_limb_t).wrapping_mul(__vl as libc::c_ulong);
            __x3 = (__uh as mp_limb_t).wrapping_mul(__vh as libc::c_ulong);
            __x1 = (__x1 as libc::c_ulong)
                .wrapping_add(
                    __x0
                        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong),
                ) as mp_limb_t as mp_limb_t;
            __x1 = (__x1 as libc::c_ulong).wrapping_add(__x2) as mp_limb_t as mp_limb_t;
            if __x1 < __x2 {
                __x3 = (__x3 as libc::c_ulong)
                    .wrapping_add(
                        (1 as libc::c_int as mp_limb_t)
                            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong),
                    ) as mp_limb_t as mp_limb_t;
            }
            th = __x3
                .wrapping_add(
                    __x1
                        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong),
                );
            tl = (__x1
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong))
                .wrapping_add(
                    __x0
                        & ((1 as libc::c_int as mp_limb_t)
                            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
        }
        r = (r as libc::c_ulong).wrapping_add(th) as mp_limb_t as mp_limb_t;
        if r < th {
            m = m.wrapping_sub(1);
            m;
            m = (m as libc::c_ulong)
                .wrapping_sub(
                    ((r > u1) as libc::c_int
                        | (r == u1) as libc::c_int & (tl > u0) as libc::c_int)
                        as libc::c_ulong,
                ) as mp_limb_t as mp_limb_t;
        }
    }
    return m;
}
unsafe extern "C" fn mpn_div_qr_1_invert(
    mut inv: *mut gmp_div_inverse,
    mut d: mp_limb_t,
) {
    let mut shift: libc::c_uint = 0;
    if d > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"d > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            893 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"void mpn_div_qr_1_invert(struct gmp_div_inverse *, mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_13391: {
        if d > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"d > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                893 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"void mpn_div_qr_1_invert(struct gmp_div_inverse *, mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut __clz_x: mp_limb_t = d;
    let mut __clz_c: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut LOCAL_SHIFT_BITS: libc::c_int = 8 as libc::c_int;
    if (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        > LOCAL_SHIFT_BITS as libc::c_ulong
    {
        while __clz_x
            & (0xff as libc::c_int as mp_limb_t)
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(8 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        {
            __clz_x <<= LOCAL_SHIFT_BITS;
            __clz_c = __clz_c.wrapping_add(8 as libc::c_int as libc::c_uint);
        }
    }
    while __clz_x
        & (1 as libc::c_int as mp_limb_t)
            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        __clz_x <<= 1 as libc::c_int;
        __clz_c = __clz_c.wrapping_add(1);
        __clz_c;
    }
    shift = __clz_c;
    (*inv).shift = shift;
    (*inv).d1 = d << shift;
    (*inv).di = mpn_invert_3by2((*inv).d1, 0 as libc::c_int as mp_limb_t);
}
unsafe extern "C" fn mpn_div_qr_2_invert(
    mut inv: *mut gmp_div_inverse,
    mut d1: mp_limb_t,
    mut d0: mp_limb_t,
) {
    let mut shift: libc::c_uint = 0;
    if d1 > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"d1 > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            906 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"void mpn_div_qr_2_invert(struct gmp_div_inverse *, mp_limb_t, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_13207: {
        if d1 > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"d1 > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                906 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"void mpn_div_qr_2_invert(struct gmp_div_inverse *, mp_limb_t, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut __clz_x: mp_limb_t = d1;
    let mut __clz_c: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut LOCAL_SHIFT_BITS: libc::c_int = 8 as libc::c_int;
    if (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        > LOCAL_SHIFT_BITS as libc::c_ulong
    {
        while __clz_x
            & (0xff as libc::c_int as mp_limb_t)
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(8 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        {
            __clz_x <<= LOCAL_SHIFT_BITS;
            __clz_c = __clz_c.wrapping_add(8 as libc::c_int as libc::c_uint);
        }
    }
    while __clz_x
        & (1 as libc::c_int as mp_limb_t)
            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        __clz_x <<= 1 as libc::c_int;
        __clz_c = __clz_c.wrapping_add(1);
        __clz_c;
    }
    shift = __clz_c;
    (*inv).shift = shift;
    if shift > 0 as libc::c_int as libc::c_uint {
        d1 = d1 << shift
            | d0
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(shift as libc::c_ulong);
        d0 <<= shift;
    }
    (*inv).d1 = d1;
    (*inv).d0 = d0;
    (*inv).di = mpn_invert_3by2(d1, d0);
}
unsafe extern "C" fn mpn_div_qr_invert(
    mut inv: *mut gmp_div_inverse,
    mut dp: mp_srcptr,
    mut dn: mp_size_t,
) {
    if dn > 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"dn > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            923 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"void mpn_div_qr_invert(struct gmp_div_inverse *, mp_srcptr, mp_size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_13434: {
        if dn > 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"dn > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                923 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 71],
                    &[libc::c_char; 71],
                >(
                    b"void mpn_div_qr_invert(struct gmp_div_inverse *, mp_srcptr, mp_size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if dn == 1 as libc::c_int as libc::c_long {
        mpn_div_qr_1_invert(inv, *dp.offset(0 as libc::c_int as isize));
    } else if dn == 2 as libc::c_int as libc::c_long {
        mpn_div_qr_2_invert(
            inv,
            *dp.offset(1 as libc::c_int as isize),
            *dp.offset(0 as libc::c_int as isize),
        );
    } else {
        let mut shift: libc::c_uint = 0;
        let mut d1: mp_limb_t = 0;
        let mut d0: mp_limb_t = 0;
        d1 = *dp.offset((dn - 1 as libc::c_int as libc::c_long) as isize);
        d0 = *dp.offset((dn - 2 as libc::c_int as libc::c_long) as isize);
        if d1 > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"d1 > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                936 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 71],
                    &[libc::c_char; 71],
                >(
                    b"void mpn_div_qr_invert(struct gmp_div_inverse *, mp_srcptr, mp_size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_12963: {
            if d1 > 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"d1 > 0\0" as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    936 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 71],
                        &[libc::c_char; 71],
                    >(
                        b"void mpn_div_qr_invert(struct gmp_div_inverse *, mp_srcptr, mp_size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let mut __clz_x: mp_limb_t = d1;
        let mut __clz_c: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut LOCAL_SHIFT_BITS: libc::c_int = 8 as libc::c_int;
        if (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            > LOCAL_SHIFT_BITS as libc::c_ulong
        {
            while __clz_x
                & (0xff as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(8 as libc::c_int as libc::c_ulong)
                == 0 as libc::c_int as libc::c_ulong
            {
                __clz_x <<= LOCAL_SHIFT_BITS;
                __clz_c = __clz_c.wrapping_add(8 as libc::c_int as libc::c_uint);
            }
        }
        while __clz_x
            & (1 as libc::c_int as mp_limb_t)
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        {
            __clz_x <<= 1 as libc::c_int;
            __clz_c = __clz_c.wrapping_add(1);
            __clz_c;
        }
        shift = __clz_c;
        (*inv).shift = shift;
        if shift > 0 as libc::c_int as libc::c_uint {
            d1 = d1 << shift
                | d0
                    >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(shift as libc::c_ulong);
            d0 = d0 << shift
                | *dp.offset((dn - 3 as libc::c_int as libc::c_long) as isize)
                    >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(shift as libc::c_ulong);
        }
        (*inv).d1 = d1;
        (*inv).d0 = d0;
        (*inv).di = mpn_invert_3by2(d1, d0);
    };
}
unsafe extern "C" fn mpn_div_qr_1_preinv(
    mut qp: mp_ptr,
    mut np: mp_srcptr,
    mut nn: mp_size_t,
    mut inv: *const gmp_div_inverse,
) -> mp_limb_t {
    let mut d: mp_limb_t = 0;
    let mut di: mp_limb_t = 0;
    let mut r: mp_limb_t = 0;
    let mut tp: mp_ptr = 0 as mp_ptr;
    if (*inv).shift > 0 as libc::c_int as libc::c_uint {
        tp = if !qp.is_null() { qp } else { gmp_xalloc_limbs(nn) };
        r = mpn_lshift(tp, np, nn, (*inv).shift);
        np = tp as mp_srcptr;
    } else {
        r = 0 as libc::c_int as mp_limb_t;
    }
    d = (*inv).d1;
    di = (*inv).di;
    loop {
        nn -= 1;
        if !(nn >= 0 as libc::c_int as libc::c_long) {
            break;
        }
        let mut q: mp_limb_t = 0;
        let mut _qh: mp_limb_t = 0;
        let mut _ql: mp_limb_t = 0;
        let mut _r: mp_limb_t = 0;
        let mut _mask: mp_limb_t = 0;
        let mut LOCAL_GMP_LIMB_BITS: libc::c_int = (::std::mem::size_of::<mp_limb_t>()
            as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
        if (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            >= (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
        {
            let mut __ww: libc::c_uint = (r as libc::c_uint as libc::c_ulong)
                .wrapping_mul(di) as libc::c_uint;
            _ql = __ww as mp_limb_t;
            _qh = (__ww >> LOCAL_GMP_LIMB_BITS) as mp_limb_t;
        } else if (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            >= (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
        {
            let mut __ww_0: libc::c_ulong = r.wrapping_mul(di);
            _ql = __ww_0;
            _qh = __ww_0 >> LOCAL_GMP_LIMB_BITS;
        } else {
            let mut __x0: mp_limb_t = 0;
            let mut __x1: mp_limb_t = 0;
            let mut __x2: mp_limb_t = 0;
            let mut __x3: mp_limb_t = 0;
            let mut __ul: libc::c_uint = 0;
            let mut __vl: libc::c_uint = 0;
            let mut __uh: libc::c_uint = 0;
            let mut __vh: libc::c_uint = 0;
            let mut __u: mp_limb_t = r;
            let mut __v: mp_limb_t = di;
            __ul = (__u
                & ((1 as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __uh = (__u
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __vl = (__v
                & ((1 as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __vh = (__v
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __x0 = (__ul as mp_limb_t).wrapping_mul(__vl as libc::c_ulong);
            __x1 = (__ul as mp_limb_t).wrapping_mul(__vh as libc::c_ulong);
            __x2 = (__uh as mp_limb_t).wrapping_mul(__vl as libc::c_ulong);
            __x3 = (__uh as mp_limb_t).wrapping_mul(__vh as libc::c_ulong);
            __x1 = (__x1 as libc::c_ulong)
                .wrapping_add(
                    __x0
                        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong),
                ) as mp_limb_t as mp_limb_t;
            __x1 = (__x1 as libc::c_ulong).wrapping_add(__x2) as mp_limb_t as mp_limb_t;
            if __x1 < __x2 {
                __x3 = (__x3 as libc::c_ulong)
                    .wrapping_add(
                        (1 as libc::c_int as mp_limb_t)
                            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong),
                    ) as mp_limb_t as mp_limb_t;
            }
            _qh = __x3
                .wrapping_add(
                    __x1
                        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong),
                );
            _ql = (__x1
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong))
                .wrapping_add(
                    __x0
                        & ((1 as libc::c_int as mp_limb_t)
                            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
        }
        let mut __x: mp_limb_t = 0;
        __x = _ql.wrapping_add(*np.offset(nn as isize));
        _qh = _qh
            .wrapping_add(r.wrapping_add(1 as libc::c_int as libc::c_ulong))
            .wrapping_add((__x < _ql) as libc::c_int as libc::c_ulong);
        _ql = __x;
        _r = (*np.offset(nn as isize)).wrapping_sub(_qh.wrapping_mul(d));
        _mask = ((_r > _ql) as libc::c_int as mp_limb_t).wrapping_neg();
        _qh = (_qh as libc::c_ulong).wrapping_add(_mask) as mp_limb_t as mp_limb_t;
        _r = (_r as libc::c_ulong).wrapping_add(_mask & d) as mp_limb_t as mp_limb_t;
        if _r >= d {
            _r = (_r as libc::c_ulong).wrapping_sub(d) as mp_limb_t as mp_limb_t;
            _qh = _qh.wrapping_add(1);
            _qh;
        }
        r = _r;
        q = _qh;
        if !qp.is_null() {
            *qp.offset(nn as isize) = q;
        }
    }
    if (*inv).shift > 0 as libc::c_int as libc::c_uint && tp != qp {
        (Some(gmp_free_func.unwrap()))
            .unwrap()(tp as *mut libc::c_void, 0 as libc::c_int as size_t);
    }
    return r >> (*inv).shift;
}
unsafe extern "C" fn mpn_div_qr_2_preinv(
    mut qp: mp_ptr,
    mut np: mp_ptr,
    mut nn: mp_size_t,
    mut inv: *const gmp_div_inverse,
) {
    let mut shift: libc::c_uint = 0;
    let mut i: mp_size_t = 0;
    let mut d1: mp_limb_t = 0;
    let mut d0: mp_limb_t = 0;
    let mut di: mp_limb_t = 0;
    let mut r1: mp_limb_t = 0;
    let mut r0: mp_limb_t = 0;
    if nn >= 2 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"nn >= 2\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            994 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"void mpn_div_qr_2_preinv(mp_ptr, mp_ptr, mp_size_t, const struct gmp_div_inverse *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_11159: {
        if nn >= 2 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"nn >= 2\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                994 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 84],
                    &[libc::c_char; 84],
                >(
                    b"void mpn_div_qr_2_preinv(mp_ptr, mp_ptr, mp_size_t, const struct gmp_div_inverse *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    shift = (*inv).shift;
    d1 = (*inv).d1;
    d0 = (*inv).d0;
    di = (*inv).di;
    if shift > 0 as libc::c_int as libc::c_uint {
        r1 = mpn_lshift(np, np as mp_srcptr, nn, shift);
    } else {
        r1 = 0 as libc::c_int as mp_limb_t;
    }
    r0 = *np.offset((nn - 1 as libc::c_int as libc::c_long) as isize);
    i = nn - 2 as libc::c_int as libc::c_long;
    loop {
        let mut n0: mp_limb_t = 0;
        let mut q: mp_limb_t = 0;
        n0 = *np.offset(i as isize);
        let mut _q0: mp_limb_t = 0;
        let mut _t1: mp_limb_t = 0;
        let mut _t0: mp_limb_t = 0;
        let mut _mask: mp_limb_t = 0;
        let mut LOCAL_GMP_LIMB_BITS: libc::c_int = (::std::mem::size_of::<mp_limb_t>()
            as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
        if (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            >= (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
        {
            let mut __ww: libc::c_uint = (r1 as libc::c_uint as libc::c_ulong)
                .wrapping_mul(di) as libc::c_uint;
            _q0 = __ww as mp_limb_t;
            q = (__ww >> LOCAL_GMP_LIMB_BITS) as mp_limb_t;
        } else if (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            >= (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
        {
            let mut __ww_0: libc::c_ulong = r1.wrapping_mul(di);
            _q0 = __ww_0;
            q = __ww_0 >> LOCAL_GMP_LIMB_BITS;
        } else {
            let mut __x0: mp_limb_t = 0;
            let mut __x1: mp_limb_t = 0;
            let mut __x2: mp_limb_t = 0;
            let mut __x3: mp_limb_t = 0;
            let mut __ul: libc::c_uint = 0;
            let mut __vl: libc::c_uint = 0;
            let mut __uh: libc::c_uint = 0;
            let mut __vh: libc::c_uint = 0;
            let mut __u: mp_limb_t = r1;
            let mut __v: mp_limb_t = di;
            __ul = (__u
                & ((1 as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __uh = (__u
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __vl = (__v
                & ((1 as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __vh = (__v
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __x0 = (__ul as mp_limb_t).wrapping_mul(__vl as libc::c_ulong);
            __x1 = (__ul as mp_limb_t).wrapping_mul(__vh as libc::c_ulong);
            __x2 = (__uh as mp_limb_t).wrapping_mul(__vl as libc::c_ulong);
            __x3 = (__uh as mp_limb_t).wrapping_mul(__vh as libc::c_ulong);
            __x1 = (__x1 as libc::c_ulong)
                .wrapping_add(
                    __x0
                        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong),
                ) as mp_limb_t as mp_limb_t;
            __x1 = (__x1 as libc::c_ulong).wrapping_add(__x2) as mp_limb_t as mp_limb_t;
            if __x1 < __x2 {
                __x3 = (__x3 as libc::c_ulong)
                    .wrapping_add(
                        (1 as libc::c_int as mp_limb_t)
                            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong),
                    ) as mp_limb_t as mp_limb_t;
            }
            q = __x3
                .wrapping_add(
                    __x1
                        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong),
                );
            _q0 = (__x1
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong))
                .wrapping_add(
                    __x0
                        & ((1 as libc::c_int as mp_limb_t)
                            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
        }
        let mut __x: mp_limb_t = 0;
        __x = _q0.wrapping_add(r0);
        q = q.wrapping_add(r1).wrapping_add((__x < _q0) as libc::c_int as libc::c_ulong);
        _q0 = __x;
        r1 = r0.wrapping_sub(d1.wrapping_mul(q));
        let mut __x_0: mp_limb_t = 0;
        __x_0 = n0.wrapping_sub(d0);
        r1 = r1.wrapping_sub(d1).wrapping_sub((n0 < d0) as libc::c_int as libc::c_ulong);
        r0 = __x_0;
        let mut LOCAL_GMP_LIMB_BITS_0: libc::c_int = (::std::mem::size_of::<mp_limb_t>()
            as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
        if (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            >= (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
        {
            let mut __ww_1: libc::c_uint = (d0 as libc::c_uint as libc::c_ulong)
                .wrapping_mul(q) as libc::c_uint;
            _t0 = __ww_1 as mp_limb_t;
            _t1 = (__ww_1 >> LOCAL_GMP_LIMB_BITS_0) as mp_limb_t;
        } else if (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            >= (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
        {
            let mut __ww_2: libc::c_ulong = d0.wrapping_mul(q);
            _t0 = __ww_2;
            _t1 = __ww_2 >> LOCAL_GMP_LIMB_BITS_0;
        } else {
            let mut __x0_0: mp_limb_t = 0;
            let mut __x1_0: mp_limb_t = 0;
            let mut __x2_0: mp_limb_t = 0;
            let mut __x3_0: mp_limb_t = 0;
            let mut __ul_0: libc::c_uint = 0;
            let mut __vl_0: libc::c_uint = 0;
            let mut __uh_0: libc::c_uint = 0;
            let mut __vh_0: libc::c_uint = 0;
            let mut __u_0: mp_limb_t = d0;
            let mut __v_0: mp_limb_t = q;
            __ul_0 = (__u_0
                & ((1 as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __uh_0 = (__u_0
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __vl_0 = (__v_0
                & ((1 as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __vh_0 = (__v_0
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __x0_0 = (__ul_0 as mp_limb_t).wrapping_mul(__vl_0 as libc::c_ulong);
            __x1_0 = (__ul_0 as mp_limb_t).wrapping_mul(__vh_0 as libc::c_ulong);
            __x2_0 = (__uh_0 as mp_limb_t).wrapping_mul(__vl_0 as libc::c_ulong);
            __x3_0 = (__uh_0 as mp_limb_t).wrapping_mul(__vh_0 as libc::c_ulong);
            __x1_0 = (__x1_0 as libc::c_ulong)
                .wrapping_add(
                    __x0_0
                        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong),
                ) as mp_limb_t as mp_limb_t;
            __x1_0 = (__x1_0 as libc::c_ulong).wrapping_add(__x2_0) as mp_limb_t
                as mp_limb_t;
            if __x1_0 < __x2_0 {
                __x3_0 = (__x3_0 as libc::c_ulong)
                    .wrapping_add(
                        (1 as libc::c_int as mp_limb_t)
                            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong),
                    ) as mp_limb_t as mp_limb_t;
            }
            _t1 = __x3_0
                .wrapping_add(
                    __x1_0
                        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong),
                );
            _t0 = (__x1_0
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong))
                .wrapping_add(
                    __x0_0
                        & ((1 as libc::c_int as mp_limb_t)
                            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
        }
        let mut __x_1: mp_limb_t = 0;
        __x_1 = r0.wrapping_sub(_t0);
        r1 = r1
            .wrapping_sub(_t1)
            .wrapping_sub((r0 < _t0) as libc::c_int as libc::c_ulong);
        r0 = __x_1;
        q = q.wrapping_add(1);
        q;
        _mask = ((r1 >= _q0) as libc::c_int as mp_limb_t).wrapping_neg();
        q = (q as libc::c_ulong).wrapping_add(_mask) as mp_limb_t as mp_limb_t;
        let mut __x_2: mp_limb_t = 0;
        __x_2 = r0.wrapping_add(_mask & d0);
        r1 = r1
            .wrapping_add(_mask & d1)
            .wrapping_add((__x_2 < r0) as libc::c_int as libc::c_ulong);
        r0 = __x_2;
        if r1 >= d1 {
            if r1 > d1 || r0 >= d0 {
                q = q.wrapping_add(1);
                q;
                let mut __x_3: mp_limb_t = 0;
                __x_3 = r0.wrapping_sub(d0);
                r1 = r1
                    .wrapping_sub(d1)
                    .wrapping_sub((r0 < d0) as libc::c_int as libc::c_ulong);
                r0 = __x_3;
            }
        }
        if !qp.is_null() {
            *qp.offset(i as isize) = q;
        }
        i -= 1;
        if !(i >= 0 as libc::c_int as libc::c_long) {
            break;
        }
    }
    if shift > 0 as libc::c_int as libc::c_uint {
        if r0
            & !(0 as libc::c_int as mp_limb_t)
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(shift as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"(r0 & (GMP_LIMB_MAX >> (GMP_LIMB_BITS - shift))) == 0\0" as *const u8
                    as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1021 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 84],
                    &[libc::c_char; 84],
                >(
                    b"void mpn_div_qr_2_preinv(mp_ptr, mp_ptr, mp_size_t, const struct gmp_div_inverse *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_9966: {
            if r0
                & !(0 as libc::c_int as mp_limb_t)
                    >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(shift as libc::c_ulong)
                == 0 as libc::c_int as libc::c_ulong
            {} else {
                __assert_fail(
                    b"(r0 & (GMP_LIMB_MAX >> (GMP_LIMB_BITS - shift))) == 0\0"
                        as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    1021 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 84],
                        &[libc::c_char; 84],
                    >(
                        b"void mpn_div_qr_2_preinv(mp_ptr, mp_ptr, mp_size_t, const struct gmp_div_inverse *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        r0 = r0 >> shift
            | r1
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(shift as libc::c_ulong);
        r1 >>= shift;
    }
    *np.offset(1 as libc::c_int as isize) = r1;
    *np.offset(0 as libc::c_int as isize) = r0;
}
unsafe extern "C" fn mpn_div_qr_pi1(
    mut qp: mp_ptr,
    mut np: mp_ptr,
    mut nn: mp_size_t,
    mut n1: mp_limb_t,
    mut dp: mp_srcptr,
    mut dn: mp_size_t,
    mut dinv: mp_limb_t,
) {
    let mut i: mp_size_t = 0;
    let mut d1: mp_limb_t = 0;
    let mut d0: mp_limb_t = 0;
    let mut cy: mp_limb_t = 0;
    let mut cy1: mp_limb_t = 0;
    let mut q: mp_limb_t = 0;
    if dn > 2 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"dn > 2\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            1042 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 91],
                &[libc::c_char; 91],
            >(
                b"void mpn_div_qr_pi1(mp_ptr, mp_ptr, mp_size_t, mp_limb_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_9397: {
        if dn > 2 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"dn > 2\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1042 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 91],
                    &[libc::c_char; 91],
                >(
                    b"void mpn_div_qr_pi1(mp_ptr, mp_ptr, mp_size_t, mp_limb_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if nn >= dn {} else {
        __assert_fail(
            b"nn >= dn\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            1043 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 91],
                &[libc::c_char; 91],
            >(
                b"void mpn_div_qr_pi1(mp_ptr, mp_ptr, mp_size_t, mp_limb_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_9359: {
        if nn >= dn {} else {
            __assert_fail(
                b"nn >= dn\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1043 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 91],
                    &[libc::c_char; 91],
                >(
                    b"void mpn_div_qr_pi1(mp_ptr, mp_ptr, mp_size_t, mp_limb_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    d1 = *dp.offset((dn - 1 as libc::c_int as libc::c_long) as isize);
    d0 = *dp.offset((dn - 2 as libc::c_int as libc::c_long) as isize);
    if d1
        & (1 as libc::c_int as mp_limb_t)
            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        != 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"(d1 & GMP_LIMB_HIGHBIT) != 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            1048 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 91],
                &[libc::c_char; 91],
            >(
                b"void mpn_div_qr_pi1(mp_ptr, mp_ptr, mp_size_t, mp_limb_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_9267: {
        if d1
            & (1 as libc::c_int as mp_limb_t)
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            != 0 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"(d1 & GMP_LIMB_HIGHBIT) != 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1048 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 91],
                    &[libc::c_char; 91],
                >(
                    b"void mpn_div_qr_pi1(mp_ptr, mp_ptr, mp_size_t, mp_limb_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    i = nn - dn;
    loop {
        let mut n0: mp_limb_t = *np
            .offset((dn - 1 as libc::c_int as libc::c_long + i) as isize);
        if n1 == d1 && n0 == d0 {
            q = !(0 as libc::c_int as mp_limb_t);
            mpn_submul_1(np.offset(i as isize), dp, dn, q);
            n1 = *np.offset((dn - 1 as libc::c_int as libc::c_long + i) as isize);
        } else {
            let mut _q0: mp_limb_t = 0;
            let mut _t1: mp_limb_t = 0;
            let mut _t0: mp_limb_t = 0;
            let mut _mask: mp_limb_t = 0;
            let mut LOCAL_GMP_LIMB_BITS: libc::c_int = (::std::mem::size_of::<
                mp_limb_t,
            >() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
            if (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                >= (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    )
            {
                let mut __ww: libc::c_uint = (n1 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(dinv) as libc::c_uint;
                _q0 = __ww as mp_limb_t;
                q = (__ww >> LOCAL_GMP_LIMB_BITS) as mp_limb_t;
            } else if (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                >= (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    )
            {
                let mut __ww_0: libc::c_ulong = n1.wrapping_mul(dinv);
                _q0 = __ww_0;
                q = __ww_0 >> LOCAL_GMP_LIMB_BITS;
            } else {
                let mut __x0: mp_limb_t = 0;
                let mut __x1: mp_limb_t = 0;
                let mut __x2: mp_limb_t = 0;
                let mut __x3: mp_limb_t = 0;
                let mut __ul: libc::c_uint = 0;
                let mut __vl: libc::c_uint = 0;
                let mut __uh: libc::c_uint = 0;
                let mut __vh: libc::c_uint = 0;
                let mut __u: mp_limb_t = n1;
                let mut __v: mp_limb_t = dinv;
                __ul = (__u
                    & ((1 as libc::c_int as mp_limb_t)
                        << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                    as libc::c_uint;
                __uh = (__u
                    >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    as libc::c_uint;
                __vl = (__v
                    & ((1 as libc::c_int as mp_limb_t)
                        << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                    as libc::c_uint;
                __vh = (__v
                    >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    as libc::c_uint;
                __x0 = (__ul as mp_limb_t).wrapping_mul(__vl as libc::c_ulong);
                __x1 = (__ul as mp_limb_t).wrapping_mul(__vh as libc::c_ulong);
                __x2 = (__uh as mp_limb_t).wrapping_mul(__vl as libc::c_ulong);
                __x3 = (__uh as mp_limb_t).wrapping_mul(__vh as libc::c_ulong);
                __x1 = (__x1 as libc::c_ulong)
                    .wrapping_add(
                        __x0
                            >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong),
                    ) as mp_limb_t as mp_limb_t;
                __x1 = (__x1 as libc::c_ulong).wrapping_add(__x2) as mp_limb_t
                    as mp_limb_t;
                if __x1 < __x2 {
                    __x3 = (__x3 as libc::c_ulong)
                        .wrapping_add(
                            (1 as libc::c_int as mp_limb_t)
                                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_div(2 as libc::c_int as libc::c_ulong),
                        ) as mp_limb_t as mp_limb_t;
                }
                q = __x3
                    .wrapping_add(
                        __x1
                            >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong),
                    );
                _q0 = (__x1
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_add(
                        __x0
                            & ((1 as libc::c_int as mp_limb_t)
                                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_div(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
            }
            let mut __x: mp_limb_t = 0;
            __x = _q0.wrapping_add(n0);
            q = q
                .wrapping_add(n1)
                .wrapping_add((__x < _q0) as libc::c_int as libc::c_ulong);
            _q0 = __x;
            n1 = n0.wrapping_sub(d1.wrapping_mul(q));
            let mut __x_0: mp_limb_t = 0;
            __x_0 = (*np.offset((dn - 2 as libc::c_int as libc::c_long + i) as isize))
                .wrapping_sub(d0);
            n1 = n1
                .wrapping_sub(d1)
                .wrapping_sub(
                    (*np.offset((dn - 2 as libc::c_int as libc::c_long + i) as isize)
                        < d0) as libc::c_int as libc::c_ulong,
                );
            n0 = __x_0;
            let mut LOCAL_GMP_LIMB_BITS_0: libc::c_int = (::std::mem::size_of::<
                mp_limb_t,
            >() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
            if (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                >= (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    )
            {
                let mut __ww_1: libc::c_uint = (d0 as libc::c_uint as libc::c_ulong)
                    .wrapping_mul(q) as libc::c_uint;
                _t0 = __ww_1 as mp_limb_t;
                _t1 = (__ww_1 >> LOCAL_GMP_LIMB_BITS_0) as mp_limb_t;
            } else if (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                >= (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    )
            {
                let mut __ww_2: libc::c_ulong = d0.wrapping_mul(q);
                _t0 = __ww_2;
                _t1 = __ww_2 >> LOCAL_GMP_LIMB_BITS_0;
            } else {
                let mut __x0_0: mp_limb_t = 0;
                let mut __x1_0: mp_limb_t = 0;
                let mut __x2_0: mp_limb_t = 0;
                let mut __x3_0: mp_limb_t = 0;
                let mut __ul_0: libc::c_uint = 0;
                let mut __vl_0: libc::c_uint = 0;
                let mut __uh_0: libc::c_uint = 0;
                let mut __vh_0: libc::c_uint = 0;
                let mut __u_0: mp_limb_t = d0;
                let mut __v_0: mp_limb_t = q;
                __ul_0 = (__u_0
                    & ((1 as libc::c_int as mp_limb_t)
                        << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                    as libc::c_uint;
                __uh_0 = (__u_0
                    >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    as libc::c_uint;
                __vl_0 = (__v_0
                    & ((1 as libc::c_int as mp_limb_t)
                        << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                    as libc::c_uint;
                __vh_0 = (__v_0
                    >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    as libc::c_uint;
                __x0_0 = (__ul_0 as mp_limb_t).wrapping_mul(__vl_0 as libc::c_ulong);
                __x1_0 = (__ul_0 as mp_limb_t).wrapping_mul(__vh_0 as libc::c_ulong);
                __x2_0 = (__uh_0 as mp_limb_t).wrapping_mul(__vl_0 as libc::c_ulong);
                __x3_0 = (__uh_0 as mp_limb_t).wrapping_mul(__vh_0 as libc::c_ulong);
                __x1_0 = (__x1_0 as libc::c_ulong)
                    .wrapping_add(
                        __x0_0
                            >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong),
                    ) as mp_limb_t as mp_limb_t;
                __x1_0 = (__x1_0 as libc::c_ulong).wrapping_add(__x2_0) as mp_limb_t
                    as mp_limb_t;
                if __x1_0 < __x2_0 {
                    __x3_0 = (__x3_0 as libc::c_ulong)
                        .wrapping_add(
                            (1 as libc::c_int as mp_limb_t)
                                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_div(2 as libc::c_int as libc::c_ulong),
                        ) as mp_limb_t as mp_limb_t;
                }
                _t1 = __x3_0
                    .wrapping_add(
                        __x1_0
                            >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong),
                    );
                _t0 = (__x1_0
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_add(
                        __x0_0
                            & ((1 as libc::c_int as mp_limb_t)
                                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_div(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
            }
            let mut __x_1: mp_limb_t = 0;
            __x_1 = n0.wrapping_sub(_t0);
            n1 = n1
                .wrapping_sub(_t1)
                .wrapping_sub((n0 < _t0) as libc::c_int as libc::c_ulong);
            n0 = __x_1;
            q = q.wrapping_add(1);
            q;
            _mask = ((n1 >= _q0) as libc::c_int as mp_limb_t).wrapping_neg();
            q = (q as libc::c_ulong).wrapping_add(_mask) as mp_limb_t as mp_limb_t;
            let mut __x_2: mp_limb_t = 0;
            __x_2 = n0.wrapping_add(_mask & d0);
            n1 = n1
                .wrapping_add(_mask & d1)
                .wrapping_add((__x_2 < n0) as libc::c_int as libc::c_ulong);
            n0 = __x_2;
            if n1 >= d1 {
                if n1 > d1 || n0 >= d0 {
                    q = q.wrapping_add(1);
                    q;
                    let mut __x_3: mp_limb_t = 0;
                    __x_3 = n0.wrapping_sub(d0);
                    n1 = n1
                        .wrapping_sub(d1)
                        .wrapping_sub((n0 < d0) as libc::c_int as libc::c_ulong);
                    n0 = __x_3;
                }
            }
            cy = mpn_submul_1(
                np.offset(i as isize),
                dp,
                dn - 2 as libc::c_int as libc::c_long,
                q,
            );
            cy1 = (n0 < cy) as libc::c_int as mp_limb_t;
            n0 = n0.wrapping_sub(cy);
            cy = (n1 < cy1) as libc::c_int as mp_limb_t;
            n1 = n1.wrapping_sub(cy1);
            *np.offset((dn - 2 as libc::c_int as libc::c_long + i) as isize) = n0;
            if cy != 0 as libc::c_int as libc::c_ulong {
                n1 = (n1 as libc::c_ulong)
                    .wrapping_add(
                        d1
                            .wrapping_add(
                                mpn_add_n(
                                    np.offset(i as isize),
                                    np.offset(i as isize) as mp_srcptr,
                                    dp,
                                    dn - 1 as libc::c_int as libc::c_long,
                                ),
                            ),
                    ) as mp_limb_t as mp_limb_t;
                q = q.wrapping_sub(1);
                q;
            }
        }
        if !qp.is_null() {
            *qp.offset(i as isize) = q;
        }
        i -= 1;
        if !(i >= 0 as libc::c_int as libc::c_long) {
            break;
        }
    }
    *np.offset((dn - 1 as libc::c_int as libc::c_long) as isize) = n1;
}
unsafe extern "C" fn mpn_div_qr_preinv(
    mut qp: mp_ptr,
    mut np: mp_ptr,
    mut nn: mp_size_t,
    mut dp: mp_srcptr,
    mut dn: mp_size_t,
    mut inv: *const gmp_div_inverse,
) {
    if dn > 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"dn > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            1098 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_11873: {
        if dn > 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"dn > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1098 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 104],
                    &[libc::c_char; 104],
                >(
                    b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if nn >= dn {} else {
        __assert_fail(
            b"nn >= dn\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            1099 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 104],
                &[libc::c_char; 104],
            >(
                b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_11835: {
        if nn >= dn {} else {
            __assert_fail(
                b"nn >= dn\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1099 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 104],
                    &[libc::c_char; 104],
                >(
                    b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if dn == 1 as libc::c_int as libc::c_long {
        *np
            .offset(
                0 as libc::c_int as isize,
            ) = mpn_div_qr_1_preinv(qp, np as mp_srcptr, nn, inv);
    } else if dn == 2 as libc::c_int as libc::c_long {
        mpn_div_qr_2_preinv(qp, np, nn, inv);
    } else {
        let mut nh: mp_limb_t = 0;
        let mut shift: libc::c_uint = 0;
        if (*inv).d1 == *dp.offset((dn - 1 as libc::c_int as libc::c_long) as isize)
        {} else {
            __assert_fail(
                b"inv->d1 == dp[dn-1]\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1110 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 104],
                    &[libc::c_char; 104],
                >(
                    b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_9827: {
            if (*inv).d1 == *dp.offset((dn - 1 as libc::c_int as libc::c_long) as isize)
            {} else {
                __assert_fail(
                    b"inv->d1 == dp[dn-1]\0" as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    1110 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 104],
                        &[libc::c_char; 104],
                    >(
                        b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if (*inv).d0 == *dp.offset((dn - 2 as libc::c_int as libc::c_long) as isize)
        {} else {
            __assert_fail(
                b"inv->d0 == dp[dn-2]\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1111 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 104],
                    &[libc::c_char; 104],
                >(
                    b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_9771: {
            if (*inv).d0 == *dp.offset((dn - 2 as libc::c_int as libc::c_long) as isize)
            {} else {
                __assert_fail(
                    b"inv->d0 == dp[dn-2]\0" as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    1111 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 104],
                        &[libc::c_char; 104],
                    >(
                        b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if (*inv).d1
            & (1 as libc::c_int as mp_limb_t)
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            != 0 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"(inv->d1 & GMP_LIMB_HIGHBIT) != 0\0" as *const u8
                    as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1112 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 104],
                    &[libc::c_char; 104],
                >(
                    b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_9698: {
            if (*inv).d1
                & (1 as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                != 0 as libc::c_int as libc::c_ulong
            {} else {
                __assert_fail(
                    b"(inv->d1 & GMP_LIMB_HIGHBIT) != 0\0" as *const u8
                        as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    1112 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 104],
                        &[libc::c_char; 104],
                    >(
                        b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        shift = (*inv).shift;
        if shift > 0 as libc::c_int as libc::c_uint {
            nh = mpn_lshift(np, np as mp_srcptr, nn, shift);
        } else {
            nh = 0 as libc::c_int as mp_limb_t;
        }
        mpn_div_qr_pi1(qp, np, nn, nh, dp, dn, (*inv).di);
        if shift > 0 as libc::c_int as libc::c_uint {
            let mut __cy: mp_limb_t = mpn_rshift(np, np as mp_srcptr, dn, shift);
            if __cy == 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"__cy == 0\0" as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    1123 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 104],
                        &[libc::c_char; 104],
                    >(
                        b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_7697: {
                if __cy == 0 as libc::c_int as libc::c_ulong {} else {
                    __assert_fail(
                        b"__cy == 0\0" as *const u8 as *const libc::c_char,
                        b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                        1123 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 104],
                            &[libc::c_char; 104],
                        >(
                            b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
        }
    };
}
unsafe extern "C" fn mpn_div_qr(
    mut qp: mp_ptr,
    mut np: mp_ptr,
    mut nn: mp_size_t,
    mut dp: mp_srcptr,
    mut dn: mp_size_t,
) {
    let mut inv: gmp_div_inverse = gmp_div_inverse {
        shift: 0,
        d1: 0,
        d0: 0,
        di: 0,
    };
    let mut tp: mp_ptr = 0 as mp_ptr;
    if dn > 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"dn > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            1133 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"void mpn_div_qr(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_13510: {
        if dn > 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"dn > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1133 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"void mpn_div_qr(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if nn >= dn {} else {
        __assert_fail(
            b"nn >= dn\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            1134 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"void mpn_div_qr(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_13472: {
        if nn >= dn {} else {
            __assert_fail(
                b"nn >= dn\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1134 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"void mpn_div_qr(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    mpn_div_qr_invert(&mut inv, dp, dn);
    if dn > 2 as libc::c_int as libc::c_long
        && inv.shift > 0 as libc::c_int as libc::c_uint
    {
        tp = gmp_xalloc_limbs(dn);
        let mut __cy: mp_limb_t = mpn_lshift(tp, dp, dn, inv.shift);
        if __cy == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"__cy == 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1140 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"void mpn_div_qr(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
        'c_11925: {
            if __cy == 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"__cy == 0\0" as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    1140 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 65],
                        &[libc::c_char; 65],
                    >(
                        b"void mpn_div_qr(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        dp = tp as mp_srcptr;
    }
    mpn_div_qr_preinv(qp, np, nn, dp, dn, &mut inv);
    if !tp.is_null() {
        (Some(gmp_free_func.unwrap()))
            .unwrap()(tp as *mut libc::c_void, 0 as libc::c_int as size_t);
    }
}
unsafe extern "C" fn mpn_base_power_of_two_p(mut b: libc::c_uint) -> libc::c_uint {
    match b {
        2 => return 1 as libc::c_int as libc::c_uint,
        4 => return 2 as libc::c_int as libc::c_uint,
        8 => return 3 as libc::c_int as libc::c_uint,
        16 => return 4 as libc::c_int as libc::c_uint,
        32 => return 5 as libc::c_int as libc::c_uint,
        64 => return 6 as libc::c_int as libc::c_uint,
        128 => return 7 as libc::c_int as libc::c_uint,
        256 => return 8 as libc::c_int as libc::c_uint,
        _ => return 0 as libc::c_int as libc::c_uint,
    };
}
unsafe extern "C" fn mpn_get_base_info(mut info: *mut mpn_base_info, mut b: mp_limb_t) {
    let mut m: mp_limb_t = 0;
    let mut p: mp_limb_t = 0;
    let mut exp: libc::c_uint = 0;
    m = (!(0 as libc::c_int as mp_limb_t)).wrapping_div(b);
    exp = 1 as libc::c_int as libc::c_uint;
    p = b;
    while p <= m {
        p = (p as libc::c_ulong).wrapping_mul(b) as mp_limb_t as mp_limb_t;
        exp = exp.wrapping_add(1);
        exp;
    }
    (*info).exp = exp;
    (*info).bb = p;
}
unsafe extern "C" fn mpn_limb_size_in_base_2(mut u: mp_limb_t) -> mp_bitcnt_t {
    let mut shift: libc::c_uint = 0;
    if u > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"u > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            1195 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"mp_bitcnt_t mpn_limb_size_in_base_2(mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_14737: {
        if u > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"u > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1195 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"mp_bitcnt_t mpn_limb_size_in_base_2(mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut __clz_x: mp_limb_t = u;
    let mut __clz_c: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut LOCAL_SHIFT_BITS: libc::c_int = 8 as libc::c_int;
    if (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        > LOCAL_SHIFT_BITS as libc::c_ulong
    {
        while __clz_x
            & (0xff as libc::c_int as mp_limb_t)
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(8 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        {
            __clz_x <<= LOCAL_SHIFT_BITS;
            __clz_c = __clz_c.wrapping_add(8 as libc::c_int as libc::c_uint);
        }
    }
    while __clz_x
        & (1 as libc::c_int as mp_limb_t)
            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        __clz_x <<= 1 as libc::c_int;
        __clz_c = __clz_c.wrapping_add(1);
        __clz_c;
    }
    shift = __clz_c;
    return (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(shift as libc::c_ulong);
}
unsafe extern "C" fn mpn_get_str_bits(
    mut sp: *mut libc::c_uchar,
    mut bits: libc::c_uint,
    mut up: mp_srcptr,
    mut un: mp_size_t,
) -> size_t {
    let mut mask: libc::c_uchar = 0;
    let mut sn: size_t = 0;
    let mut j: size_t = 0;
    let mut i: mp_size_t = 0;
    let mut shift: libc::c_uint = 0;
    sn = ((un - 1 as libc::c_int as libc::c_long) as libc::c_ulong)
        .wrapping_mul(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(
            mpn_limb_size_in_base_2(
                *up.offset((un - 1 as libc::c_int as libc::c_long) as isize),
            ),
        )
        .wrapping_add(bits as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(bits as libc::c_ulong);
    mask = ((1 as libc::c_uint) << bits).wrapping_sub(1 as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    i = 0 as libc::c_int as mp_size_t;
    j = sn;
    shift = 0 as libc::c_int as libc::c_uint;
    loop {
        let fresh11 = j;
        j = j.wrapping_sub(1);
        if !(fresh11 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        let mut digit: libc::c_uchar = (*up.offset(i as isize) >> shift)
            as libc::c_uchar;
        shift = shift.wrapping_add(bits);
        if shift as libc::c_ulong
            >= (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            && {
                i += 1;
                i < un
            }
        {
            shift = (shift as libc::c_ulong)
                .wrapping_sub(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as libc::c_uint as libc::c_uint;
            digit = (digit as libc::c_ulong
                | *up.offset(i as isize) << bits.wrapping_sub(shift)) as libc::c_uchar;
        }
        *sp
            .offset(
                j as isize,
            ) = (digit as libc::c_int & mask as libc::c_int) as libc::c_uchar;
    }
    return sn;
}
unsafe extern "C" fn mpn_limb_get_str(
    mut sp: *mut libc::c_uchar,
    mut w: mp_limb_t,
    mut binv: *const gmp_div_inverse,
) -> size_t {
    let mut i: mp_size_t = 0;
    i = 0 as libc::c_int as mp_size_t;
    while w > 0 as libc::c_int as libc::c_ulong {
        let mut h: mp_limb_t = 0;
        let mut l: mp_limb_t = 0;
        let mut r: mp_limb_t = 0;
        h = w
            >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub((*binv).shift as libc::c_ulong);
        l = w << (*binv).shift;
        let mut _qh: mp_limb_t = 0;
        let mut _ql: mp_limb_t = 0;
        let mut _r: mp_limb_t = 0;
        let mut _mask: mp_limb_t = 0;
        let mut LOCAL_GMP_LIMB_BITS: libc::c_int = (::std::mem::size_of::<mp_limb_t>()
            as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
        if (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            >= (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
        {
            let mut __ww: libc::c_uint = (h as libc::c_uint as libc::c_ulong)
                .wrapping_mul((*binv).di) as libc::c_uint;
            _ql = __ww as mp_limb_t;
            _qh = (__ww >> LOCAL_GMP_LIMB_BITS) as mp_limb_t;
        } else if (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            >= (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                )
        {
            let mut __ww_0: libc::c_ulong = h.wrapping_mul((*binv).di);
            _ql = __ww_0;
            _qh = __ww_0 >> LOCAL_GMP_LIMB_BITS;
        } else {
            let mut __x0: mp_limb_t = 0;
            let mut __x1: mp_limb_t = 0;
            let mut __x2: mp_limb_t = 0;
            let mut __x3: mp_limb_t = 0;
            let mut __ul: libc::c_uint = 0;
            let mut __vl: libc::c_uint = 0;
            let mut __uh: libc::c_uint = 0;
            let mut __vh: libc::c_uint = 0;
            let mut __u: mp_limb_t = h;
            let mut __v: mp_limb_t = (*binv).di;
            __ul = (__u
                & ((1 as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __uh = (__u
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __vl = (__v
                & ((1 as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __vh = (__v
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)) as libc::c_uint;
            __x0 = (__ul as mp_limb_t).wrapping_mul(__vl as libc::c_ulong);
            __x1 = (__ul as mp_limb_t).wrapping_mul(__vh as libc::c_ulong);
            __x2 = (__uh as mp_limb_t).wrapping_mul(__vl as libc::c_ulong);
            __x3 = (__uh as mp_limb_t).wrapping_mul(__vh as libc::c_ulong);
            __x1 = (__x1 as libc::c_ulong)
                .wrapping_add(
                    __x0
                        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong),
                ) as mp_limb_t as mp_limb_t;
            __x1 = (__x1 as libc::c_ulong).wrapping_add(__x2) as mp_limb_t as mp_limb_t;
            if __x1 < __x2 {
                __x3 = (__x3 as libc::c_ulong)
                    .wrapping_add(
                        (1 as libc::c_int as mp_limb_t)
                            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong),
                    ) as mp_limb_t as mp_limb_t;
            }
            _qh = __x3
                .wrapping_add(
                    __x1
                        >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong),
                );
            _ql = (__x1
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong))
                .wrapping_add(
                    __x0
                        & ((1 as libc::c_int as mp_limb_t)
                            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
        }
        let mut __x: mp_limb_t = 0;
        __x = _ql.wrapping_add(l);
        _qh = _qh
            .wrapping_add(h.wrapping_add(1 as libc::c_int as libc::c_ulong))
            .wrapping_add((__x < _ql) as libc::c_int as libc::c_ulong);
        _ql = __x;
        _r = l.wrapping_sub(_qh.wrapping_mul((*binv).d1));
        _mask = ((_r > _ql) as libc::c_int as mp_limb_t).wrapping_neg();
        _qh = (_qh as libc::c_ulong).wrapping_add(_mask) as mp_limb_t as mp_limb_t;
        _r = (_r as libc::c_ulong).wrapping_add(_mask & (*binv).d1) as mp_limb_t
            as mp_limb_t;
        if _r >= (*binv).d1 {
            _r = (_r as libc::c_ulong).wrapping_sub((*binv).d1) as mp_limb_t
                as mp_limb_t;
            _qh = _qh.wrapping_add(1);
            _qh;
        }
        r = _r;
        w = _qh;
        if r
            & !(0 as libc::c_int as mp_limb_t)
                >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub((*binv).shift as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"(r & (GMP_LIMB_MAX >> (GMP_LIMB_BITS - binv->shift))) == 0\0"
                    as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1244 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 84],
                    &[libc::c_char; 84],
                >(
                    b"size_t mpn_limb_get_str(unsigned char *, mp_limb_t, const struct gmp_div_inverse *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_16981: {
            if r
                & !(0 as libc::c_int as mp_limb_t)
                    >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub((*binv).shift as libc::c_ulong)
                == 0 as libc::c_int as libc::c_ulong
            {} else {
                __assert_fail(
                    b"(r & (GMP_LIMB_MAX >> (GMP_LIMB_BITS - binv->shift))) == 0\0"
                        as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    1244 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 84],
                        &[libc::c_char; 84],
                    >(
                        b"size_t mpn_limb_get_str(unsigned char *, mp_limb_t, const struct gmp_div_inverse *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        r >>= (*binv).shift;
        *sp.offset(i as isize) = r as libc::c_uchar;
        i += 1;
        i;
    }
    return i as size_t;
}
unsafe extern "C" fn mpn_get_str_other(
    mut sp: *mut libc::c_uchar,
    mut base: libc::c_int,
    mut info: *const mpn_base_info,
    mut up: mp_ptr,
    mut un: mp_size_t,
) -> size_t {
    let mut binv: gmp_div_inverse = gmp_div_inverse {
        shift: 0,
        d1: 0,
        d0: 0,
        di: 0,
    };
    let mut sn: size_t = 0;
    let mut i: size_t = 0;
    mpn_div_qr_1_invert(&mut binv, base as mp_limb_t);
    sn = 0 as libc::c_int as size_t;
    if un > 1 as libc::c_int as libc::c_long {
        let mut bbinv: gmp_div_inverse = gmp_div_inverse {
            shift: 0,
            d1: 0,
            d0: 0,
            di: 0,
        };
        mpn_div_qr_1_invert(&mut bbinv, (*info).bb);
        loop {
            let mut w: mp_limb_t = 0;
            let mut done: size_t = 0;
            w = mpn_div_qr_1_preinv(up, up as mp_srcptr, un, &mut bbinv);
            un
                -= (*up.offset((un - 1 as libc::c_int as libc::c_long) as isize)
                    == 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long;
            done = mpn_limb_get_str(sp.offset(sn as isize), w, &mut binv);
            sn = (sn as libc::c_ulong).wrapping_add(done) as size_t as size_t;
            while done < (*info).exp as libc::c_ulong {
                let fresh12 = sn;
                sn = sn.wrapping_add(1);
                *sp.offset(fresh12 as isize) = 0 as libc::c_int as libc::c_uchar;
                done = done.wrapping_add(1);
                done;
            }
            if !(un > 1 as libc::c_int as libc::c_long) {
                break;
            }
        }
    }
    sn = (sn as libc::c_ulong)
        .wrapping_add(
            mpn_limb_get_str(
                sp.offset(sn as isize),
                *up.offset(0 as libc::c_int as isize),
                &mut binv,
            ),
        ) as size_t as size_t;
    i = 0 as libc::c_int as size_t;
    while (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(i)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) < sn
    {
        let mut t: libc::c_uchar = *sp.offset(i as isize);
        *sp
            .offset(
                i as isize,
            ) = *sp
            .offset(
                sn.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            );
        *sp
            .offset(
                sn.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            ) = t;
        i = i.wrapping_add(1);
        i;
    }
    return sn;
}
pub unsafe extern "C" fn mpn_get_str(
    mut sp: *mut libc::c_uchar,
    mut base: libc::c_int,
    mut up: mp_ptr,
    mut un: mp_size_t,
) -> size_t {
    let mut bits: libc::c_uint = 0;
    if un > 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"un > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            1301 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"size_t mpn_get_str(unsigned char *, int, mp_ptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_18070: {
        if un > 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"un > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1301 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"size_t mpn_get_str(unsigned char *, int, mp_ptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if *up.offset((un - 1 as libc::c_int as libc::c_long) as isize)
        > 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"up[un-1] > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            1302 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"size_t mpn_get_str(unsigned char *, int, mp_ptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_18018: {
        if *up.offset((un - 1 as libc::c_int as libc::c_long) as isize)
            > 0 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"up[un-1] > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1302 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"size_t mpn_get_str(unsigned char *, int, mp_ptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    bits = mpn_base_power_of_two_p(base as libc::c_uint);
    if bits != 0 {
        return mpn_get_str_bits(sp, bits, up as mp_srcptr, un)
    } else {
        let mut info: mpn_base_info = mpn_base_info { exp: 0, bb: 0 };
        mpn_get_base_info(&mut info, base as mp_limb_t);
        return mpn_get_str_other(sp, base, &mut info, up, un);
    };
}
unsafe extern "C" fn mpn_set_str_bits(
    mut rp: mp_ptr,
    mut sp: *const libc::c_uchar,
    mut sn: size_t,
    mut bits: libc::c_uint,
) -> mp_size_t {
    let mut rn: mp_size_t = 0;
    let mut j: size_t = 0;
    let mut shift: libc::c_uint = 0;
    j = sn;
    rn = 0 as libc::c_int as mp_size_t;
    shift = 0 as libc::c_int as libc::c_uint;
    loop {
        let fresh13 = j;
        j = j.wrapping_sub(1);
        if !(fresh13 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        if shift == 0 as libc::c_int as libc::c_uint {
            let fresh14 = rn;
            rn = rn + 1;
            *rp.offset(fresh14 as isize) = *sp.offset(j as isize) as mp_limb_t;
            shift = shift.wrapping_add(bits);
        } else {
            let ref mut fresh15 = *rp
                .offset((rn - 1 as libc::c_int as libc::c_long) as isize);
            *fresh15 |= (*sp.offset(j as isize) as mp_limb_t) << shift;
            shift = shift.wrapping_add(bits);
            if shift as libc::c_ulong
                >= (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            {
                shift = (shift as libc::c_ulong)
                    .wrapping_sub(
                        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as libc::c_uint as libc::c_uint;
                if shift > 0 as libc::c_int as libc::c_uint {
                    let fresh16 = rn;
                    rn = rn + 1;
                    *rp
                        .offset(
                            fresh16 as isize,
                        ) = *sp.offset(j as isize) as mp_limb_t
                        >> bits.wrapping_sub(shift);
                }
            }
        }
    }
    rn = mpn_normalized_size(rp as mp_srcptr, rn);
    return rn;
}
unsafe extern "C" fn mpn_set_str_other(
    mut rp: mp_ptr,
    mut sp: *const libc::c_uchar,
    mut sn: size_t,
    mut b: mp_limb_t,
    mut info: *const mpn_base_info,
) -> mp_size_t {
    let mut rn: mp_size_t = 0;
    let mut w: mp_limb_t = 0;
    let mut k: libc::c_uint = 0;
    let mut j: size_t = 0;
    if sn > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"sn > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            1358 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 108],
                &[libc::c_char; 108],
            >(
                b"mp_size_t mpn_set_str_other(mp_ptr, const unsigned char *, size_t, mp_limb_t, const struct mpn_base_info *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_18369: {
        if sn > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"sn > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1358 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 108],
                    &[libc::c_char; 108],
                >(
                    b"mp_size_t mpn_set_str_other(mp_ptr, const unsigned char *, size_t, mp_limb_t, const struct mpn_base_info *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    k = (1 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            sn
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_rem((*info).exp as libc::c_ulong),
        ) as libc::c_uint;
    j = 0 as libc::c_int as size_t;
    let fresh17 = j;
    j = j.wrapping_add(1);
    w = *sp.offset(fresh17 as isize) as mp_limb_t;
    loop {
        k = k.wrapping_sub(1);
        if !(k != 0 as libc::c_int as libc::c_uint) {
            break;
        }
        let fresh18 = j;
        j = j.wrapping_add(1);
        w = w
            .wrapping_mul(b)
            .wrapping_add(*sp.offset(fresh18 as isize) as libc::c_ulong);
    }
    *rp.offset(0 as libc::c_int as isize) = w;
    rn = 1 as libc::c_int as mp_size_t;
    while j < sn {
        let mut cy: mp_limb_t = 0;
        let fresh19 = j;
        j = j.wrapping_add(1);
        w = *sp.offset(fresh19 as isize) as mp_limb_t;
        k = 1 as libc::c_int as libc::c_uint;
        while k < (*info).exp {
            let fresh20 = j;
            j = j.wrapping_add(1);
            w = w
                .wrapping_mul(b)
                .wrapping_add(*sp.offset(fresh20 as isize) as libc::c_ulong);
            k = k.wrapping_add(1);
            k;
        }
        cy = mpn_mul_1(rp, rp as mp_srcptr, rn, (*info).bb);
        cy = (cy as libc::c_ulong).wrapping_add(mpn_add_1(rp, rp as mp_srcptr, rn, w))
            as mp_limb_t as mp_limb_t;
        if cy > 0 as libc::c_int as libc::c_ulong {
            let fresh21 = rn;
            rn = rn + 1;
            *rp.offset(fresh21 as isize) = cy;
        }
    }
    if j == sn {} else {
        __assert_fail(
            b"j == sn\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            1382 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 108],
                &[libc::c_char; 108],
            >(
                b"mp_size_t mpn_set_str_other(mp_ptr, const unsigned char *, size_t, mp_limb_t, const struct mpn_base_info *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_18167: {
        if j == sn {} else {
            __assert_fail(
                b"j == sn\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1382 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 108],
                    &[libc::c_char; 108],
                >(
                    b"mp_size_t mpn_set_str_other(mp_ptr, const unsigned char *, size_t, mp_limb_t, const struct mpn_base_info *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return rn;
}
pub unsafe extern "C" fn mpn_set_str(
    mut rp: mp_ptr,
    mut sp: *const libc::c_uchar,
    mut sn: size_t,
    mut base: libc::c_int,
) -> mp_size_t {
    let mut bits: libc::c_uint = 0;
    if sn == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as mp_size_t;
    }
    bits = mpn_base_power_of_two_p(base as libc::c_uint);
    if bits != 0 {
        return mpn_set_str_bits(rp, sp, sn, bits)
    } else {
        let mut info: mpn_base_info = mpn_base_info { exp: 0, bb: 0 };
        mpn_get_base_info(&mut info, base as mp_limb_t);
        return mpn_set_str_other(rp, sp, sn, base as mp_limb_t, &mut info);
    };
}
pub unsafe extern "C" fn mpz_init(mut r: *mut __mpz_struct) {
    static mut dummy_limb: mp_limb_t = !(0 as libc::c_int as mp_limb_t)
        & 0xc1a0 as libc::c_int as libc::c_ulong;
    (*r)._mp_alloc = 0 as libc::c_int;
    (*r)._mp_size = 0 as libc::c_int;
    (*r)._mp_d = &dummy_limb as *const mp_limb_t as mp_ptr;
}
pub unsafe extern "C" fn mpz_init2(mut r: *mut __mpz_struct, mut bits: mp_bitcnt_t) {
    let mut rn: mp_size_t = 0;
    bits = (bits as libc::c_ulong)
        .wrapping_sub(
            (bits != 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_ulong,
        ) as mp_bitcnt_t as mp_bitcnt_t;
    rn = (1 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            bits
                .wrapping_div(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ),
        ) as mp_size_t;
    (*r)._mp_alloc = rn as libc::c_int;
    (*r)._mp_size = 0 as libc::c_int;
    (*r)._mp_d = gmp_xalloc_limbs(rn);
}
pub unsafe extern "C" fn mpz_clear(mut r: *mut __mpz_struct) {
    if (*r)._mp_alloc != 0 {
        (Some(gmp_free_func.unwrap()))
            .unwrap()((*r)._mp_d as *mut libc::c_void, 0 as libc::c_int as size_t);
    }
}
unsafe extern "C" fn mpz_realloc(
    mut r: *mut __mpz_struct,
    mut size: mp_size_t,
) -> mp_ptr {
    size = if size > 1 as libc::c_int as libc::c_long {
        size
    } else {
        1 as libc::c_int as libc::c_long
    };
    if (*r)._mp_alloc != 0 {
        (*r)._mp_d = gmp_xrealloc_limbs((*r)._mp_d, size);
    } else {
        (*r)._mp_d = gmp_xalloc_limbs(size);
    }
    (*r)._mp_alloc = size as libc::c_int;
    if (if (*r)._mp_size >= 0 as libc::c_int { (*r)._mp_size } else { -(*r)._mp_size })
        as libc::c_long > size
    {
        (*r)._mp_size = 0 as libc::c_int;
    }
    return (*r)._mp_d;
}
pub unsafe extern "C" fn mpz_set_si(mut r: *mut __mpz_struct, mut x: libc::c_long) {
    if x >= 0 as libc::c_int as libc::c_long {
        mpz_set_ui(r, x as libc::c_ulong);
    } else if (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        < (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        mpz_set_ui(
            r,
            ((x + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_neg(),
        );
        mpz_neg(r, r as *const __mpz_struct);
    } else {
        (*r)._mp_size = -(1 as libc::c_int);
        *if 1 as libc::c_int > (*r)._mp_alloc {
            mpz_realloc(r, 1 as libc::c_int as mp_size_t)
        } else {
            (*r)._mp_d
        }
            .offset(
                0 as libc::c_int as isize,
            ) = ((x + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_neg();
    };
}
pub unsafe extern "C" fn mpz_set_ui(mut r: *mut __mpz_struct, mut x: libc::c_ulong) {
    if x > 0 as libc::c_int as libc::c_ulong {
        (*r)._mp_size = 1 as libc::c_int;
        *if 1 as libc::c_int > (*r)._mp_alloc {
            mpz_realloc(r, 1 as libc::c_int as mp_size_t)
        } else {
            (*r)._mp_d
        }
            .offset(0 as libc::c_int as isize) = x;
        if (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            < (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            let mut LOCAL_GMP_LIMB_BITS: libc::c_int = (::std::mem::size_of::<
                mp_limb_t,
            >() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
            loop {
                x >>= LOCAL_GMP_LIMB_BITS;
                if !(x != 0) {
                    break;
                }
                (*r)._mp_size += 1;
                (*r)._mp_size;
                *if (*r)._mp_size > (*r)._mp_alloc {
                    mpz_realloc(r, (*r)._mp_size as mp_size_t)
                } else {
                    (*r)._mp_d
                }
                    .offset(((*r)._mp_size - 1 as libc::c_int) as isize) = x;
            }
        }
    } else {
        (*r)._mp_size = 0 as libc::c_int;
    };
}
pub unsafe extern "C" fn mpz_set(mut r: *mut __mpz_struct, mut x: *const __mpz_struct) {
    if r != x as *mut __mpz_struct {
        let mut n: mp_size_t = 0;
        let mut rp: mp_ptr = 0 as *mut mp_limb_t;
        n = (if (*x)._mp_size >= 0 as libc::c_int {
            (*x)._mp_size
        } else {
            -(*x)._mp_size
        }) as mp_size_t;
        rp = if n > (*r)._mp_alloc as libc::c_long {
            mpz_realloc(r, n)
        } else {
            (*r)._mp_d
        };
        mpn_copyi(rp, (*x)._mp_d as mp_srcptr, n);
        (*r)._mp_size = (*x)._mp_size;
    }
}
pub unsafe extern "C" fn mpz_init_set_si(mut r: *mut __mpz_struct, mut x: libc::c_long) {
    mpz_init(r);
    mpz_set_si(r, x);
}
pub unsafe extern "C" fn mpz_init_set_ui(
    mut r: *mut __mpz_struct,
    mut x: libc::c_ulong,
) {
    mpz_init(r);
    mpz_set_ui(r, x);
}
pub unsafe extern "C" fn mpz_init_set(
    mut r: *mut __mpz_struct,
    mut x: *const __mpz_struct,
) {
    mpz_init(r);
    mpz_set(r, x);
}
pub unsafe extern "C" fn mpz_fits_slong_p(mut u: *const __mpz_struct) -> libc::c_int {
    return (mpz_cmp_si(u, 9223372036854775807 as libc::c_long) <= 0 as libc::c_int
        && mpz_cmp_si(u, -(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
            >= 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn mpn_absfits_ulong_p(
    mut up: mp_srcptr,
    mut un: mp_size_t,
) -> libc::c_int {
    let mut ulongsize: libc::c_int = (::std::mem::size_of::<libc::c_ulong>()
        as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as libc::c_int;
    let mut ulongrem: mp_limb_t = 0 as libc::c_int as mp_limb_t;
    if (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_rem(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) != 0 as libc::c_int as libc::c_ulong
    {
        ulongrem = ((9223372036854775807 as libc::c_long as libc::c_ulong)
            .wrapping_mul(2 as libc::c_ulong)
            .wrapping_add(1 as libc::c_ulong)
            >> (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(ulongsize as libc::c_ulong))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    return (un <= ulongsize as libc::c_long
        || *up.offset(ulongsize as isize) < ulongrem
            && un == (ulongsize + 1 as libc::c_int) as libc::c_long) as libc::c_int;
}
pub unsafe extern "C" fn mpz_fits_ulong_p(mut u: *const __mpz_struct) -> libc::c_int {
    let mut us: mp_size_t = (*u)._mp_size as mp_size_t;
    return (us >= 0 as libc::c_int as libc::c_long
        && mpn_absfits_ulong_p((*u)._mp_d as mp_srcptr, us) != 0) as libc::c_int;
}
pub unsafe extern "C" fn mpz_fits_sint_p(mut u: *const __mpz_struct) -> libc::c_int {
    return (mpz_cmp_si(u, 2147483647 as libc::c_int as libc::c_long) <= 0 as libc::c_int
        && mpz_cmp_si(
            u,
            (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long,
        ) >= 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn mpz_fits_uint_p(mut u: *const __mpz_struct) -> libc::c_int {
    return ((*u)._mp_size >= 0 as libc::c_int
        && mpz_cmpabs_ui(
            u,
            (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint) as libc::c_ulong,
        ) <= 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn mpz_fits_sshort_p(mut u: *const __mpz_struct) -> libc::c_int {
    return (mpz_cmp_si(u, 32767 as libc::c_int as libc::c_long) <= 0 as libc::c_int
        && mpz_cmp_si(u, (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long)
            >= 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn mpz_fits_ushort_p(mut u: *const __mpz_struct) -> libc::c_int {
    return ((*u)._mp_size >= 0 as libc::c_int
        && mpz_cmpabs_ui(
            u,
            (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        ) <= 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn mpz_get_si(mut u: *const __mpz_struct) -> libc::c_long {
    let mut r: libc::c_ulong = mpz_get_ui(u);
    let mut c: libc::c_ulong = (-(9223372036854775807 as libc::c_long)
        - (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)) as libc::c_ulong;
    if (*u)._mp_size < 0 as libc::c_int {
        return -(c as libc::c_long)
            - (r.wrapping_sub(c) & 9223372036854775807 as libc::c_long as libc::c_ulong)
                as libc::c_long
    } else {
        return (r & 9223372036854775807 as libc::c_long as libc::c_ulong) as libc::c_long
    };
}
pub unsafe extern "C" fn mpz_get_ui(mut u: *const __mpz_struct) -> libc::c_ulong {
    if (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        < (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        let mut LOCAL_GMP_LIMB_BITS: libc::c_int = (::std::mem::size_of::<mp_limb_t>()
            as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
        let mut r: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
        let mut n: mp_size_t = (if (*u)._mp_size >= 0 as libc::c_int {
            (*u)._mp_size
        } else {
            -(*u)._mp_size
        }) as mp_size_t;
        n = (if (n as libc::c_ulong)
            < (1 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    ((::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as mp_size_t
                        as libc::c_ulong)
                        .wrapping_div(
                            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ),
                )
        {
            n as libc::c_ulong
        } else {
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    ((::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as mp_size_t
                        as libc::c_ulong)
                        .wrapping_div(
                            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ),
                )
        }) as mp_size_t;
        loop {
            n -= 1;
            if !(n >= 0 as libc::c_int as libc::c_long) {
                break;
            }
            r = (r << LOCAL_GMP_LIMB_BITS)
                .wrapping_add(*((*u)._mp_d).offset(n as isize));
        }
        return r;
    }
    return if (*u)._mp_size == 0 as libc::c_int {
        0 as libc::c_int as libc::c_ulong
    } else {
        *((*u)._mp_d).offset(0 as libc::c_int as isize)
    };
}
pub unsafe extern "C" fn mpz_size(mut u: *const __mpz_struct) -> size_t {
    return (if (*u)._mp_size >= 0 as libc::c_int {
        (*u)._mp_size
    } else {
        -(*u)._mp_size
    }) as size_t;
}
pub unsafe extern "C" fn mpz_getlimbn(
    mut u: *const __mpz_struct,
    mut n: mp_size_t,
) -> mp_limb_t {
    if n >= 0 as libc::c_int as libc::c_long
        && n
            < (if (*u)._mp_size >= 0 as libc::c_int {
                (*u)._mp_size
            } else {
                -(*u)._mp_size
            }) as libc::c_long
    {
        return *((*u)._mp_d).offset(n as isize)
    } else {
        return 0 as libc::c_int as mp_limb_t
    };
}
pub unsafe extern "C" fn mpz_realloc2(mut x: *mut __mpz_struct, mut n: mp_bitcnt_t) {
    mpz_realloc(
        x,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                n
                    .wrapping_sub(
                        (n != 0 as libc::c_int as libc::c_ulong) as libc::c_int
                            as libc::c_ulong,
                    )
                    .wrapping_div(
                        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ),
            ) as mp_size_t,
    );
}
pub unsafe extern "C" fn mpz_limbs_read(mut x: mpz_srcptr) -> mp_srcptr {
    return (*x)._mp_d as mp_srcptr;
}
pub unsafe extern "C" fn mpz_limbs_modify(
    mut x: *mut __mpz_struct,
    mut n: mp_size_t,
) -> mp_ptr {
    if n > 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"n > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            1651 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"mp_ptr mpz_limbs_modify(__mpz_struct *, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_28395: {
        if n > 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1651 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"mp_ptr mpz_limbs_modify(__mpz_struct *, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    return if n > (*x)._mp_alloc as libc::c_long {
        mpz_realloc(x, n)
    } else {
        (*x)._mp_d
    };
}
pub unsafe extern "C" fn mpz_limbs_write(
    mut x: *mut __mpz_struct,
    mut n: mp_size_t,
) -> mp_ptr {
    return mpz_limbs_modify(x, n);
}
pub unsafe extern "C" fn mpz_limbs_finish(mut x: *mut __mpz_struct, mut xs: mp_size_t) {
    let mut xn: mp_size_t = 0;
    xn = mpn_normalized_size(
        (*x)._mp_d as mp_srcptr,
        if xs >= 0 as libc::c_int as libc::c_long { xs } else { -xs },
    );
    (*x)
        ._mp_size = (if xs < 0 as libc::c_int as libc::c_long { -xn } else { xn })
        as libc::c_int;
}
unsafe extern "C" fn mpz_roinit_normal_n(
    mut x: *mut __mpz_struct,
    mut xp: mp_srcptr,
    mut xs: mp_size_t,
) -> mpz_srcptr {
    (*x)._mp_alloc = 0 as libc::c_int;
    (*x)._mp_d = xp as mp_ptr;
    (*x)._mp_size = xs as libc::c_int;
    return x as mpz_srcptr;
}
pub unsafe extern "C" fn mpz_roinit_n(
    mut x: *mut __mpz_struct,
    mut xp: mp_srcptr,
    mut xs: mp_size_t,
) -> mpz_srcptr {
    mpz_roinit_normal_n(x, xp, xs);
    mpz_limbs_finish(x, xs);
    return x as mpz_srcptr;
}
pub unsafe extern "C" fn mpz_set_d(mut r: *mut __mpz_struct, mut x: libc::c_double) {
    let mut sign: libc::c_int = 0;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    let mut rn: mp_size_t = 0;
    let mut i: mp_size_t = 0;
    let mut B: libc::c_double = 0.;
    let mut Bi: libc::c_double = 0.;
    let mut f: mp_limb_t = 0;
    if x != x || x == x * 0.5f64 {
        (*r)._mp_size = 0 as libc::c_int;
        return;
    }
    sign = (x < 0.0f64) as libc::c_int;
    if sign != 0 {
        x = -x;
    }
    if x < 1.0f64 {
        (*r)._mp_size = 0 as libc::c_int;
        return;
    }
    B = 4.0f64
        * ((1 as libc::c_int as mp_limb_t)
            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) >> 1 as libc::c_int)
            as libc::c_double;
    Bi = 1.0f64 / B;
    rn = 1 as libc::c_int as mp_size_t;
    while x >= B {
        x *= Bi;
        rn += 1;
        rn;
    }
    rp = if rn > (*r)._mp_alloc as libc::c_long {
        mpz_realloc(r, rn)
    } else {
        (*r)._mp_d
    };
    f = x as mp_limb_t;
    x -= f as libc::c_double;
    if x < 1.0f64 {} else {
        __assert_fail(
            b"x < 1.0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            1724 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void mpz_set_d(__mpz_struct *, double)\0"))
                .as_ptr(),
        );
    }
    'c_28657: {
        if x < 1.0f64 {} else {
            __assert_fail(
                b"x < 1.0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1724 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void mpz_set_d(__mpz_struct *, double)\0"))
                    .as_ptr(),
            );
        }
    };
    i = rn - 1 as libc::c_int as libc::c_long;
    *rp.offset(i as isize) = f;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int as libc::c_long) {
            break;
        }
        x = B * x;
        f = x as mp_limb_t;
        x -= f as libc::c_double;
        if x < 1.0f64 {} else {
            __assert_fail(
                b"x < 1.0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1732 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void mpz_set_d(__mpz_struct *, double)\0"))
                    .as_ptr(),
            );
        }
        'c_28583: {
            if x < 1.0f64 {} else {
                __assert_fail(
                    b"x < 1.0\0" as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    1732 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 39],
                        &[libc::c_char; 39],
                    >(b"void mpz_set_d(__mpz_struct *, double)\0"))
                        .as_ptr(),
                );
            }
        };
        *rp.offset(i as isize) = f;
    }
    (*r)._mp_size = (if sign != 0 { -rn } else { rn }) as libc::c_int;
}
pub unsafe extern "C" fn mpz_init_set_d(
    mut r: *mut __mpz_struct,
    mut x: libc::c_double,
) {
    mpz_init(r);
    mpz_set_d(r, x);
}
pub unsafe extern "C" fn mpz_get_d(mut u: *const __mpz_struct) -> libc::c_double {
    let mut m: libc::c_int = 0;
    let mut l: mp_limb_t = 0;
    let mut un: mp_size_t = 0;
    let mut x: libc::c_double = 0.;
    let mut B: libc::c_double = 4.0f64
        * ((1 as libc::c_int as mp_limb_t)
            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) >> 1 as libc::c_int)
            as libc::c_double;
    un = (if (*u)._mp_size >= 0 as libc::c_int { (*u)._mp_size } else { -(*u)._mp_size })
        as mp_size_t;
    if un == 0 as libc::c_int as libc::c_long {
        return 0.0f64;
    }
    un -= 1;
    l = *((*u)._mp_d).offset(un as isize);
    let mut __clz_x: mp_limb_t = l;
    let mut __clz_c: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut LOCAL_SHIFT_BITS: libc::c_int = 8 as libc::c_int;
    if (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        > LOCAL_SHIFT_BITS as libc::c_ulong
    {
        while __clz_x
            & (0xff as libc::c_int as mp_limb_t)
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(8 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        {
            __clz_x <<= LOCAL_SHIFT_BITS;
            __clz_c = __clz_c.wrapping_add(8 as libc::c_int as libc::c_uint);
        }
    }
    while __clz_x
        & (1 as libc::c_int as mp_limb_t)
            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        __clz_x <<= 1 as libc::c_int;
        __clz_c = __clz_c.wrapping_add(1);
        __clz_c;
    }
    m = __clz_c as libc::c_int;
    m = ((m + 53 as libc::c_int) as libc::c_ulong)
        .wrapping_sub(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as libc::c_int;
    if m < 0 as libc::c_int {
        l &= !(0 as libc::c_int as mp_limb_t) << -m;
    }
    x = l as libc::c_double;
    loop {
        un -= 1;
        if !(un >= 0 as libc::c_int as libc::c_long) {
            break;
        }
        x = B * x;
        if m > 0 as libc::c_int {
            l = *((*u)._mp_d).offset(un as isize);
            m = (m as libc::c_ulong)
                .wrapping_sub(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as libc::c_int as libc::c_int;
            if m < 0 as libc::c_int {
                l &= !(0 as libc::c_int as mp_limb_t) << -m;
            }
            x += l as libc::c_double;
        }
    }
    if (*u)._mp_size < 0 as libc::c_int {
        x = -x;
    }
    return x;
}
pub unsafe extern "C" fn mpz_cmpabs_d(
    mut x: *const __mpz_struct,
    mut d: libc::c_double,
) -> libc::c_int {
    let mut xn: mp_size_t = 0;
    let mut B: libc::c_double = 0.;
    let mut Bi: libc::c_double = 0.;
    let mut i: mp_size_t = 0;
    xn = (*x)._mp_size as mp_size_t;
    d = if d >= 0 as libc::c_int as libc::c_double { d } else { -d };
    if xn != 0 as libc::c_int as libc::c_long {
        xn = if xn >= 0 as libc::c_int as libc::c_long { xn } else { -xn };
        B = 4.0f64
            * ((1 as libc::c_int as mp_limb_t)
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) >> 1 as libc::c_int)
                as libc::c_double;
        Bi = 1.0f64 / B;
        i = 1 as libc::c_int as mp_size_t;
        while i < xn {
            d *= Bi;
            i += 1;
            i;
        }
        if d >= B {
            return -(1 as libc::c_int);
        }
        i = xn;
        loop {
            let fresh22 = i;
            i = i - 1;
            if !(fresh22 > 0 as libc::c_int as libc::c_long) {
                break;
            }
            let mut f: mp_limb_t = 0;
            let mut xl: mp_limb_t = 0;
            f = d as mp_limb_t;
            xl = *((*x)._mp_d).offset(i as isize);
            if xl > f {
                return 1 as libc::c_int
            } else if xl < f {
                return -(1 as libc::c_int)
            }
            d = B * (d - f as libc::c_double);
        }
    }
    return -((d > 0.0f64) as libc::c_int);
}
pub unsafe extern "C" fn mpz_cmp_d(
    mut x: *const __mpz_struct,
    mut d: libc::c_double,
) -> libc::c_int {
    if (*x)._mp_size < 0 as libc::c_int {
        if d >= 0.0f64 { return -(1 as libc::c_int) } else { return -mpz_cmpabs_d(x, d) }
    } else if d < 0.0f64 {
        return 1 as libc::c_int
    } else {
        return mpz_cmpabs_d(x, d)
    };
}
pub unsafe extern "C" fn mpz_sgn(mut u: *const __mpz_struct) -> libc::c_int {
    return ((*u)._mp_size > 0 as libc::c_int) as libc::c_int
        - ((*u)._mp_size < 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn mpz_cmp_si(
    mut u: *const __mpz_struct,
    mut v: libc::c_long,
) -> libc::c_int {
    let mut usize: mp_size_t = (*u)._mp_size as mp_size_t;
    if v >= 0 as libc::c_int as libc::c_long {
        return mpz_cmp_ui(u, v as libc::c_ulong)
    } else if usize >= 0 as libc::c_int as libc::c_long {
        return 1 as libc::c_int
    } else {
        return -mpz_cmpabs_ui(
            u,
            ((v + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_neg(),
        )
    };
}
pub unsafe extern "C" fn mpz_cmp_ui(
    mut u: *const __mpz_struct,
    mut v: libc::c_ulong,
) -> libc::c_int {
    let mut usize: mp_size_t = (*u)._mp_size as mp_size_t;
    if usize < 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int)
    } else {
        return mpz_cmpabs_ui(u, v)
    };
}
pub unsafe extern "C" fn mpz_cmp(
    mut a: *const __mpz_struct,
    mut b: *const __mpz_struct,
) -> libc::c_int {
    let mut asize: mp_size_t = (*a)._mp_size as mp_size_t;
    let mut bsize: mp_size_t = (*b)._mp_size as mp_size_t;
    if asize != bsize {
        return if asize < bsize { -(1 as libc::c_int) } else { 1 as libc::c_int }
    } else if asize >= 0 as libc::c_int as libc::c_long {
        return mpn_cmp((*a)._mp_d as mp_srcptr, (*b)._mp_d as mp_srcptr, asize)
    } else {
        return mpn_cmp((*b)._mp_d as mp_srcptr, (*a)._mp_d as mp_srcptr, -asize)
    };
}
pub unsafe extern "C" fn mpz_cmpabs_ui(
    mut u: *const __mpz_struct,
    mut v: libc::c_ulong,
) -> libc::c_int {
    let mut un: mp_size_t = (if (*u)._mp_size >= 0 as libc::c_int {
        (*u)._mp_size
    } else {
        -(*u)._mp_size
    }) as mp_size_t;
    if mpn_absfits_ulong_p((*u)._mp_d as mp_srcptr, un) == 0 {
        return 1 as libc::c_int
    } else {
        let mut uu: libc::c_ulong = mpz_get_ui(u);
        return (uu > v) as libc::c_int - (uu < v) as libc::c_int;
    };
}
pub unsafe extern "C" fn mpz_cmpabs(
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) -> libc::c_int {
    return mpn_cmp4(
        (*u)._mp_d as mp_srcptr,
        (if (*u)._mp_size >= 0 as libc::c_int { (*u)._mp_size } else { -(*u)._mp_size })
            as mp_size_t,
        (*v)._mp_d as mp_srcptr,
        (if (*v)._mp_size >= 0 as libc::c_int { (*v)._mp_size } else { -(*v)._mp_size })
            as mp_size_t,
    );
}
pub unsafe extern "C" fn mpz_abs(mut r: *mut __mpz_struct, mut u: *const __mpz_struct) {
    mpz_set(r, u);
    (*r)
        ._mp_size = if (*r)._mp_size >= 0 as libc::c_int {
        (*r)._mp_size
    } else {
        -(*r)._mp_size
    };
}
pub unsafe extern "C" fn mpz_neg(mut r: *mut __mpz_struct, mut u: *const __mpz_struct) {
    mpz_set(r, u);
    (*r)._mp_size = -(*r)._mp_size;
}
pub unsafe extern "C" fn mpz_swap(mut u: *mut __mpz_struct, mut v: *mut __mpz_struct) {
    let mut __mp_size_t_swap__tmp: mp_size_t = (*u)._mp_size as mp_size_t;
    (*u)._mp_size = (*v)._mp_size;
    (*v)._mp_size = __mp_size_t_swap__tmp as libc::c_int;
    let mut __mp_size_t_swap__tmp_0: mp_size_t = (*u)._mp_alloc as mp_size_t;
    (*u)._mp_alloc = (*v)._mp_alloc;
    (*v)._mp_alloc = __mp_size_t_swap__tmp_0 as libc::c_int;
    let mut __mp_ptr_swap__tmp: mp_ptr = (*u)._mp_d;
    (*u)._mp_d = (*v)._mp_d;
    (*v)._mp_d = __mp_ptr_swap__tmp;
}
pub unsafe extern "C" fn mpz_add_ui(
    mut r: *mut __mpz_struct,
    mut a: *const __mpz_struct,
    mut b: libc::c_ulong,
) {
    let mut bb: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init_set_ui(bb.as_mut_ptr(), b);
    mpz_add(r, a, bb.as_mut_ptr() as *const __mpz_struct);
    mpz_clear(bb.as_mut_ptr());
}
pub unsafe extern "C" fn mpz_sub_ui(
    mut r: *mut __mpz_struct,
    mut a: *const __mpz_struct,
    mut b: libc::c_ulong,
) {
    mpz_ui_sub(r, b, a);
    mpz_neg(r, r as *const __mpz_struct);
}
pub unsafe extern "C" fn mpz_ui_sub(
    mut r: *mut __mpz_struct,
    mut a: libc::c_ulong,
    mut b: *const __mpz_struct,
) {
    mpz_neg(r, b);
    mpz_add_ui(r, r as *const __mpz_struct, a);
}
unsafe extern "C" fn mpz_abs_add(
    mut r: *mut __mpz_struct,
    mut a: *const __mpz_struct,
    mut b: *const __mpz_struct,
) -> mp_size_t {
    let mut an: mp_size_t = (if (*a)._mp_size >= 0 as libc::c_int {
        (*a)._mp_size
    } else {
        -(*a)._mp_size
    }) as mp_size_t;
    let mut bn: mp_size_t = (if (*b)._mp_size >= 0 as libc::c_int {
        (*b)._mp_size
    } else {
        -(*b)._mp_size
    }) as mp_size_t;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    let mut cy: mp_limb_t = 0;
    if an < bn {
        let mut __mpz_srcptr_swap__tmp: mpz_srcptr = a;
        a = b;
        b = __mpz_srcptr_swap__tmp;
        let mut __mp_size_t_swap__tmp: mp_size_t = an;
        an = bn;
        bn = __mp_size_t_swap__tmp;
    }
    rp = if an + 1 as libc::c_int as libc::c_long > (*r)._mp_alloc as libc::c_long {
        mpz_realloc(r, an + 1 as libc::c_int as libc::c_long)
    } else {
        (*r)._mp_d
    };
    cy = mpn_add(rp, (*a)._mp_d as mp_srcptr, an, (*b)._mp_d as mp_srcptr, bn);
    *rp.offset(an as isize) = cy;
    return (an as libc::c_ulong).wrapping_add(cy) as mp_size_t;
}
unsafe extern "C" fn mpz_abs_sub(
    mut r: *mut __mpz_struct,
    mut a: *const __mpz_struct,
    mut b: *const __mpz_struct,
) -> mp_size_t {
    let mut an: mp_size_t = (if (*a)._mp_size >= 0 as libc::c_int {
        (*a)._mp_size
    } else {
        -(*a)._mp_size
    }) as mp_size_t;
    let mut bn: mp_size_t = (if (*b)._mp_size >= 0 as libc::c_int {
        (*b)._mp_size
    } else {
        -(*b)._mp_size
    }) as mp_size_t;
    let mut cmp: libc::c_int = 0;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    cmp = mpn_cmp4((*a)._mp_d as mp_srcptr, an, (*b)._mp_d as mp_srcptr, bn);
    if cmp > 0 as libc::c_int {
        rp = if an > (*r)._mp_alloc as libc::c_long {
            mpz_realloc(r, an)
        } else {
            (*r)._mp_d
        };
        let mut __cy: mp_limb_t = mpn_sub(
            rp,
            (*a)._mp_d as mp_srcptr,
            an,
            (*b)._mp_d as mp_srcptr,
            bn,
        );
        if __cy == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"__cy == 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                1994 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 82],
                    &[libc::c_char; 82],
                >(
                    b"mp_size_t mpz_abs_sub(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_6018: {
            if __cy == 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"__cy == 0\0" as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    1994 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 82],
                        &[libc::c_char; 82],
                    >(
                        b"mp_size_t mpz_abs_sub(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        return mpn_normalized_size(rp as mp_srcptr, an);
    } else if cmp < 0 as libc::c_int {
        rp = if bn > (*r)._mp_alloc as libc::c_long {
            mpz_realloc(r, bn)
        } else {
            (*r)._mp_d
        };
        let mut __cy_0: mp_limb_t = mpn_sub(
            rp,
            (*b)._mp_d as mp_srcptr,
            bn,
            (*a)._mp_d as mp_srcptr,
            an,
        );
        if __cy_0 == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"__cy == 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                2000 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 82],
                    &[libc::c_char; 82],
                >(
                    b"mp_size_t mpz_abs_sub(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5890: {
            if __cy_0 == 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"__cy == 0\0" as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    2000 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 82],
                        &[libc::c_char; 82],
                    >(
                        b"mp_size_t mpz_abs_sub(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        return -mpn_normalized_size(rp as mp_srcptr, bn);
    } else {
        return 0 as libc::c_int as mp_size_t
    };
}
pub unsafe extern "C" fn mpz_add(
    mut r: *mut __mpz_struct,
    mut a: *const __mpz_struct,
    mut b: *const __mpz_struct,
) {
    let mut rn: mp_size_t = 0;
    if (*a)._mp_size ^ (*b)._mp_size >= 0 as libc::c_int {
        rn = mpz_abs_add(r, a, b);
    } else {
        rn = mpz_abs_sub(r, a, b);
    }
    (*r)
        ._mp_size = (if (*a)._mp_size >= 0 as libc::c_int { rn } else { -rn })
        as libc::c_int;
}
pub unsafe extern "C" fn mpz_sub(
    mut r: *mut __mpz_struct,
    mut a: *const __mpz_struct,
    mut b: *const __mpz_struct,
) {
    let mut rn: mp_size_t = 0;
    if (*a)._mp_size ^ (*b)._mp_size >= 0 as libc::c_int {
        rn = mpz_abs_sub(r, a, b);
    } else {
        rn = mpz_abs_add(r, a, b);
    }
    (*r)
        ._mp_size = (if (*a)._mp_size >= 0 as libc::c_int { rn } else { -rn })
        as libc::c_int;
}
pub unsafe extern "C" fn mpz_mul_si(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: libc::c_long,
) {
    if v < 0 as libc::c_int as libc::c_long {
        mpz_mul_ui(
            r,
            u,
            ((v + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_neg(),
        );
        mpz_neg(r, r as *const __mpz_struct);
    } else {
        mpz_mul_ui(r, u, v as libc::c_ulong);
    };
}
pub unsafe extern "C" fn mpz_mul_ui(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: libc::c_ulong,
) {
    let mut vv: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init_set_ui(vv.as_mut_ptr(), v);
    mpz_mul(r, u, vv.as_mut_ptr() as *const __mpz_struct);
    mpz_clear(vv.as_mut_ptr());
}
pub unsafe extern "C" fn mpz_mul(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) {
    let mut sign: libc::c_int = 0;
    let mut un: mp_size_t = 0;
    let mut vn: mp_size_t = 0;
    let mut rn: mp_size_t = 0;
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut tp: mp_ptr = 0 as *mut mp_limb_t;
    un = (*u)._mp_size as mp_size_t;
    vn = (*v)._mp_size as mp_size_t;
    if un == 0 as libc::c_int as libc::c_long || vn == 0 as libc::c_int as libc::c_long {
        (*r)._mp_size = 0 as libc::c_int;
        return;
    }
    sign = (un ^ vn < 0 as libc::c_int as libc::c_long) as libc::c_int;
    un = if un >= 0 as libc::c_int as libc::c_long { un } else { -un };
    vn = if vn >= 0 as libc::c_int as libc::c_long { vn } else { -vn };
    mpz_init2(
        t.as_mut_ptr(),
        ((un + vn) as libc::c_ulong)
            .wrapping_mul(
                (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ),
    );
    tp = (*t.as_mut_ptr())._mp_d;
    if un >= vn {
        mpn_mul(tp, (*u)._mp_d as mp_srcptr, un, (*v)._mp_d as mp_srcptr, vn);
    } else {
        mpn_mul(tp, (*v)._mp_d as mp_srcptr, vn, (*u)._mp_d as mp_srcptr, un);
    }
    rn = un + vn;
    rn
        -= (*tp.offset((rn - 1 as libc::c_int as libc::c_long) as isize)
            == 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long;
    (*t.as_mut_ptr())._mp_size = (if sign != 0 { -rn } else { rn }) as libc::c_int;
    mpz_swap(r, t.as_mut_ptr());
    mpz_clear(t.as_mut_ptr());
}
pub unsafe extern "C" fn mpz_mul_2exp(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut bits: mp_bitcnt_t,
) {
    let mut un: mp_size_t = 0;
    let mut rn: mp_size_t = 0;
    let mut limbs: mp_size_t = 0;
    let mut shift: libc::c_uint = 0;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    un = (if (*u)._mp_size >= 0 as libc::c_int { (*u)._mp_size } else { -(*u)._mp_size })
        as mp_size_t;
    if un == 0 as libc::c_int as libc::c_long {
        (*r)._mp_size = 0 as libc::c_int;
        return;
    }
    limbs = bits
        .wrapping_div(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as mp_size_t;
    shift = bits
        .wrapping_rem(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as libc::c_uint;
    rn = un + limbs
        + (shift > 0 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_long;
    rp = if rn > (*r)._mp_alloc as libc::c_long {
        mpz_realloc(r, rn)
    } else {
        (*r)._mp_d
    };
    if shift > 0 as libc::c_int as libc::c_uint {
        let mut cy: mp_limb_t = mpn_lshift(
            rp.offset(limbs as isize),
            (*u)._mp_d as mp_srcptr,
            un,
            shift,
        );
        *rp.offset((rn - 1 as libc::c_int as libc::c_long) as isize) = cy;
        rn -= (cy == 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long;
    } else {
        mpn_copyd(rp.offset(limbs as isize), (*u)._mp_d as mp_srcptr, un);
    }
    mpn_zero(rp, limbs);
    (*r)
        ._mp_size = (if (*u)._mp_size < 0 as libc::c_int { -rn } else { rn })
        as libc::c_int;
}
pub unsafe extern "C" fn mpz_addmul_ui(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: libc::c_ulong,
) {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init_set_ui(t.as_mut_ptr(), v);
    mpz_mul(t.as_mut_ptr(), u, t.as_mut_ptr() as *const __mpz_struct);
    mpz_add(r, r as *const __mpz_struct, t.as_mut_ptr() as *const __mpz_struct);
    mpz_clear(t.as_mut_ptr());
}
pub unsafe extern "C" fn mpz_submul_ui(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: libc::c_ulong,
) {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init_set_ui(t.as_mut_ptr(), v);
    mpz_mul(t.as_mut_ptr(), u, t.as_mut_ptr() as *const __mpz_struct);
    mpz_sub(r, r as *const __mpz_struct, t.as_mut_ptr() as *const __mpz_struct);
    mpz_clear(t.as_mut_ptr());
}
pub unsafe extern "C" fn mpz_addmul(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init(t.as_mut_ptr());
    mpz_mul(t.as_mut_ptr(), u, v);
    mpz_add(r, r as *const __mpz_struct, t.as_mut_ptr() as *const __mpz_struct);
    mpz_clear(t.as_mut_ptr());
}
pub unsafe extern "C" fn mpz_submul(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init(t.as_mut_ptr());
    mpz_mul(t.as_mut_ptr(), u, v);
    mpz_sub(r, r as *const __mpz_struct, t.as_mut_ptr() as *const __mpz_struct);
    mpz_clear(t.as_mut_ptr());
}
unsafe extern "C" fn mpz_div_qr(
    mut q: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
    mut mode: mpz_div_round_mode,
) -> libc::c_int {
    let mut ns: mp_size_t = 0;
    let mut ds: mp_size_t = 0;
    let mut nn: mp_size_t = 0;
    let mut dn: mp_size_t = 0;
    let mut qs: mp_size_t = 0;
    ns = (*n)._mp_size as mp_size_t;
    ds = (*d)._mp_size as mp_size_t;
    if ds == 0 as libc::c_int as libc::c_long {
        gmp_die(b"mpz_div_qr: Divide by zero.\0" as *const u8 as *const libc::c_char);
    }
    if ns == 0 as libc::c_int as libc::c_long {
        if !q.is_null() {
            (*q)._mp_size = 0 as libc::c_int;
        }
        if !r.is_null() {
            (*r)._mp_size = 0 as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    nn = if ns >= 0 as libc::c_int as libc::c_long { ns } else { -ns };
    dn = if ds >= 0 as libc::c_int as libc::c_long { ds } else { -ds };
    qs = ds ^ ns;
    if nn < dn {
        if mode as libc::c_uint == GMP_DIV_CEIL as libc::c_int as libc::c_uint
            && qs >= 0 as libc::c_int as libc::c_long
        {
            if !r.is_null() {
                mpz_sub(r, n, d);
            }
            if !q.is_null() {
                mpz_set_ui(q, 1 as libc::c_int as libc::c_ulong);
            }
        } else if mode as libc::c_uint == GMP_DIV_FLOOR as libc::c_int as libc::c_uint
            && qs < 0 as libc::c_int as libc::c_long
        {
            if !r.is_null() {
                mpz_add(r, n, d);
            }
            if !q.is_null() {
                mpz_set_si(q, -(1 as libc::c_int) as libc::c_long);
            }
        } else {
            if !r.is_null() {
                mpz_set(r, n);
            }
            if !q.is_null() {
                (*q)._mp_size = 0 as libc::c_int;
            }
        }
        return 1 as libc::c_int;
    } else {
        let mut np: mp_ptr = 0 as *mut mp_limb_t;
        let mut qp: mp_ptr = 0 as *mut mp_limb_t;
        let mut qn: mp_size_t = 0;
        let mut rn: mp_size_t = 0;
        let mut tq: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        let mut tr: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        mpz_init_set(tr.as_mut_ptr(), n);
        np = (*tr.as_mut_ptr())._mp_d;
        qn = nn - dn + 1 as libc::c_int as libc::c_long;
        if !q.is_null() {
            mpz_init2(
                tq.as_mut_ptr(),
                (qn as libc::c_ulong)
                    .wrapping_mul(
                        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ),
            );
            qp = (*tq.as_mut_ptr())._mp_d;
        } else {
            qp = 0 as mp_ptr;
        }
        mpn_div_qr(qp, np, nn, (*d)._mp_d as mp_srcptr, dn);
        if !qp.is_null() {
            qn
                -= (*qp.offset((qn - 1 as libc::c_int as libc::c_long) as isize)
                    == 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long;
            (*tq.as_mut_ptr())
                ._mp_size = (if qs < 0 as libc::c_int as libc::c_long {
                -qn
            } else {
                qn
            }) as libc::c_int;
        }
        rn = mpn_normalized_size(np as mp_srcptr, dn);
        (*tr.as_mut_ptr())
            ._mp_size = (if ns < 0 as libc::c_int as libc::c_long { -rn } else { rn })
            as libc::c_int;
        if mode as libc::c_uint == GMP_DIV_FLOOR as libc::c_int as libc::c_uint
            && qs < 0 as libc::c_int as libc::c_long
            && rn != 0 as libc::c_int as libc::c_long
        {
            if !q.is_null() {
                mpz_sub_ui(
                    tq.as_mut_ptr(),
                    tq.as_mut_ptr() as *const __mpz_struct,
                    1 as libc::c_int as libc::c_ulong,
                );
            }
            if !r.is_null() {
                mpz_add(tr.as_mut_ptr(), tr.as_mut_ptr() as *const __mpz_struct, d);
            }
        } else if mode as libc::c_uint == GMP_DIV_CEIL as libc::c_int as libc::c_uint
            && qs >= 0 as libc::c_int as libc::c_long
            && rn != 0 as libc::c_int as libc::c_long
        {
            if !q.is_null() {
                mpz_add_ui(
                    tq.as_mut_ptr(),
                    tq.as_mut_ptr() as *const __mpz_struct,
                    1 as libc::c_int as libc::c_ulong,
                );
            }
            if !r.is_null() {
                mpz_sub(tr.as_mut_ptr(), tr.as_mut_ptr() as *const __mpz_struct, d);
            }
        }
        if !q.is_null() {
            mpz_swap(tq.as_mut_ptr(), q);
            mpz_clear(tq.as_mut_ptr());
        }
        if !r.is_null() {
            mpz_swap(tr.as_mut_ptr(), r);
        }
        mpz_clear(tr.as_mut_ptr());
        return (rn != 0 as libc::c_int as libc::c_long) as libc::c_int;
    };
}
pub unsafe extern "C" fn mpz_cdiv_qr(
    mut q: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(q, r, n, d, GMP_DIV_CEIL);
}
pub unsafe extern "C" fn mpz_fdiv_qr(
    mut q: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(q, r, n, d, GMP_DIV_FLOOR);
}
pub unsafe extern "C" fn mpz_tdiv_qr(
    mut q: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(q, r, n, d, GMP_DIV_TRUNC);
}
pub unsafe extern "C" fn mpz_cdiv_q(
    mut q: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(q, 0 as *mut __mpz_struct, n, d, GMP_DIV_CEIL);
}
pub unsafe extern "C" fn mpz_fdiv_q(
    mut q: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(q, 0 as *mut __mpz_struct, n, d, GMP_DIV_FLOOR);
}
pub unsafe extern "C" fn mpz_tdiv_q(
    mut q: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(q, 0 as *mut __mpz_struct, n, d, GMP_DIV_TRUNC);
}
pub unsafe extern "C" fn mpz_cdiv_r(
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(0 as *mut __mpz_struct, r, n, d, GMP_DIV_CEIL);
}
pub unsafe extern "C" fn mpz_fdiv_r(
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(0 as *mut __mpz_struct, r, n, d, GMP_DIV_FLOOR);
}
pub unsafe extern "C" fn mpz_tdiv_r(
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(0 as *mut __mpz_struct, r, n, d, GMP_DIV_TRUNC);
}
pub unsafe extern "C" fn mpz_mod(
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(
        0 as *mut __mpz_struct,
        r,
        n,
        d,
        (if (*d)._mp_size >= 0 as libc::c_int {
            GMP_DIV_FLOOR as libc::c_int
        } else {
            GMP_DIV_CEIL as libc::c_int
        }) as mpz_div_round_mode,
    );
}
unsafe extern "C" fn mpz_div_q_2exp(
    mut q: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut bit_index: mp_bitcnt_t,
    mut mode: mpz_div_round_mode,
) {
    let mut un: mp_size_t = 0;
    let mut qn: mp_size_t = 0;
    let mut limb_cnt: mp_size_t = 0;
    let mut qp: mp_ptr = 0 as *mut mp_limb_t;
    let mut adjust: libc::c_int = 0;
    un = (*u)._mp_size as mp_size_t;
    if un == 0 as libc::c_int as libc::c_long {
        (*q)._mp_size = 0 as libc::c_int;
        return;
    }
    limb_cnt = bit_index
        .wrapping_div(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as mp_size_t;
    qn = (if un >= 0 as libc::c_int as libc::c_long { un } else { -un }) - limb_cnt;
    bit_index = (bit_index as libc::c_ulong)
        .wrapping_rem(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as mp_bitcnt_t as mp_bitcnt_t;
    if mode as libc::c_uint
        == (if un > 0 as libc::c_int as libc::c_long {
            GMP_DIV_CEIL as libc::c_int
        } else {
            GMP_DIV_FLOOR as libc::c_int
        }) as libc::c_uint
    {
        adjust = (qn <= 0 as libc::c_int as libc::c_long
            || mpn_zero_p((*u)._mp_d as mp_srcptr, limb_cnt) == 0
            || *((*u)._mp_d).offset(limb_cnt as isize)
                & ((1 as libc::c_int as mp_limb_t) << bit_index)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) != 0)
            as libc::c_int;
    } else {
        adjust = 0 as libc::c_int;
    }
    if qn <= 0 as libc::c_int as libc::c_long {
        qn = 0 as libc::c_int as mp_size_t;
    } else {
        qp = if qn > (*q)._mp_alloc as libc::c_long {
            mpz_realloc(q, qn)
        } else {
            (*q)._mp_d
        };
        if bit_index != 0 as libc::c_int as libc::c_ulong {
            mpn_rshift(
                qp,
                ((*u)._mp_d).offset(limb_cnt as isize) as mp_srcptr,
                qn,
                bit_index as libc::c_uint,
            );
            qn
                -= (*qp.offset((qn - 1 as libc::c_int as libc::c_long) as isize)
                    == 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long;
        } else {
            mpn_copyi(qp, ((*u)._mp_d).offset(limb_cnt as isize) as mp_srcptr, qn);
        }
    }
    (*q)._mp_size = qn as libc::c_int;
    if adjust != 0 {
        mpz_add_ui(q, q as *const __mpz_struct, 1 as libc::c_int as libc::c_ulong);
    }
    if un < 0 as libc::c_int as libc::c_long {
        mpz_neg(q, q as *const __mpz_struct);
    }
}
unsafe extern "C" fn mpz_div_r_2exp(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut bit_index: mp_bitcnt_t,
    mut mode: mpz_div_round_mode,
) {
    let mut us: mp_size_t = 0;
    let mut un: mp_size_t = 0;
    let mut rn: mp_size_t = 0;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    let mut mask: mp_limb_t = 0;
    us = (*u)._mp_size as mp_size_t;
    if us == 0 as libc::c_int as libc::c_long
        || bit_index == 0 as libc::c_int as libc::c_ulong
    {
        (*r)._mp_size = 0 as libc::c_int;
        return;
    }
    rn = bit_index
        .wrapping_add(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        )
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as mp_size_t;
    if rn > 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"rn > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            2415 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"void mpz_div_r_2exp(__mpz_struct *, const __mpz_struct *, mp_bitcnt_t, enum mpz_div_round_mode)\0",
            ))
                .as_ptr(),
        );
    }
    'c_20065: {
        if rn > 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"rn > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                2415 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"void mpz_div_r_2exp(__mpz_struct *, const __mpz_struct *, mp_bitcnt_t, enum mpz_div_round_mode)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    rp = if rn > (*r)._mp_alloc as libc::c_long {
        mpz_realloc(r, rn)
    } else {
        (*r)._mp_d
    };
    un = if us >= 0 as libc::c_int as libc::c_long { us } else { -us };
    mask = !(0 as libc::c_int as mp_limb_t)
        >> (rn as libc::c_ulong)
            .wrapping_mul(
                (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
            .wrapping_sub(bit_index);
    if rn > un {
        if mode as libc::c_uint
            == (if us > 0 as libc::c_int as libc::c_long {
                GMP_DIV_CEIL as libc::c_int
            } else {
                GMP_DIV_FLOOR as libc::c_int
            }) as libc::c_uint
        {
            let mut i: mp_size_t = 0;
            let mut __cy: mp_limb_t = (mpn_neg(rp, (*u)._mp_d as mp_srcptr, un) == 0)
                as libc::c_int as mp_limb_t;
            if __cy == 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"__cy == 0\0" as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    2431 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 96],
                        &[libc::c_char; 96],
                    >(
                        b"void mpz_div_r_2exp(__mpz_struct *, const __mpz_struct *, mp_bitcnt_t, enum mpz_div_round_mode)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_19928: {
                if __cy == 0 as libc::c_int as libc::c_ulong {} else {
                    __assert_fail(
                        b"__cy == 0\0" as *const u8 as *const libc::c_char,
                        b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                        2431 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 96],
                            &[libc::c_char; 96],
                        >(
                            b"void mpz_div_r_2exp(__mpz_struct *, const __mpz_struct *, mp_bitcnt_t, enum mpz_div_round_mode)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            i = un;
            while i < rn - 1 as libc::c_int as libc::c_long {
                *rp.offset(i as isize) = !(0 as libc::c_int as mp_limb_t);
                i += 1;
                i;
            }
            *rp.offset((rn - 1 as libc::c_int as libc::c_long) as isize) = mask;
            us = -us;
        } else {
            if r != u as *mut __mpz_struct {
                mpn_copyi(rp, (*u)._mp_d as mp_srcptr, un);
            }
            rn = un;
        }
    } else {
        if r != u as *mut __mpz_struct {
            mpn_copyi(
                rp,
                (*u)._mp_d as mp_srcptr,
                rn - 1 as libc::c_int as libc::c_long,
            );
        }
        *rp
            .offset(
                (rn - 1 as libc::c_int as libc::c_long) as isize,
            ) = *((*u)._mp_d).offset((rn - 1 as libc::c_int as libc::c_long) as isize)
            & mask;
        if mode as libc::c_uint
            == (if us > 0 as libc::c_int as libc::c_long {
                GMP_DIV_CEIL as libc::c_int
            } else {
                GMP_DIV_FLOOR as libc::c_int
            }) as libc::c_uint
        {
            mpn_neg(rp, rp as mp_srcptr, rn);
            let ref mut fresh23 = *rp
                .offset((rn - 1 as libc::c_int as libc::c_long) as isize);
            *fresh23 &= mask;
            us = -us;
        }
    }
    rn = mpn_normalized_size(rp as mp_srcptr, rn);
    (*r)
        ._mp_size = (if us < 0 as libc::c_int as libc::c_long { -rn } else { rn })
        as libc::c_int;
}
pub unsafe extern "C" fn mpz_cdiv_q_2exp(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut cnt: mp_bitcnt_t,
) {
    mpz_div_q_2exp(r, u, cnt, GMP_DIV_CEIL);
}
pub unsafe extern "C" fn mpz_fdiv_q_2exp(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut cnt: mp_bitcnt_t,
) {
    mpz_div_q_2exp(r, u, cnt, GMP_DIV_FLOOR);
}
pub unsafe extern "C" fn mpz_tdiv_q_2exp(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut cnt: mp_bitcnt_t,
) {
    mpz_div_q_2exp(r, u, cnt, GMP_DIV_TRUNC);
}
pub unsafe extern "C" fn mpz_cdiv_r_2exp(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut cnt: mp_bitcnt_t,
) {
    mpz_div_r_2exp(r, u, cnt, GMP_DIV_CEIL);
}
pub unsafe extern "C" fn mpz_fdiv_r_2exp(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut cnt: mp_bitcnt_t,
) {
    mpz_div_r_2exp(r, u, cnt, GMP_DIV_FLOOR);
}
pub unsafe extern "C" fn mpz_tdiv_r_2exp(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut cnt: mp_bitcnt_t,
) {
    mpz_div_r_2exp(r, u, cnt, GMP_DIV_TRUNC);
}
pub unsafe extern "C" fn mpz_divexact(
    mut q: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    let mut __cy: mp_limb_t = mpz_div_qr(q, 0 as *mut __mpz_struct, n, d, GMP_DIV_TRUNC)
        as mp_limb_t;
    if __cy == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"__cy == 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            2509 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 78],
                &[libc::c_char; 78],
            >(
                b"void mpz_divexact(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_20223: {
        if __cy == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"__cy == 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                2509 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"void mpz_divexact(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
pub unsafe extern "C" fn mpz_divisible_p(
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) -> libc::c_int {
    return (mpz_div_qr(
        0 as *mut __mpz_struct,
        0 as *mut __mpz_struct,
        n,
        d,
        GMP_DIV_TRUNC,
    ) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn mpz_div_qr_ui(
    mut q: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: libc::c_ulong,
    mut mode: mpz_div_round_mode,
) -> libc::c_ulong {
    let mut ret: libc::c_ulong = 0;
    let mut rr: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut dd: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init(rr.as_mut_ptr());
    mpz_init_set_ui(dd.as_mut_ptr(), d);
    mpz_div_qr(q, rr.as_mut_ptr(), n, dd.as_mut_ptr() as *const __mpz_struct, mode);
    mpz_clear(dd.as_mut_ptr());
    ret = mpz_get_ui(rr.as_mut_ptr() as *const __mpz_struct);
    if !r.is_null() {
        mpz_swap(r, rr.as_mut_ptr());
    }
    mpz_clear(rr.as_mut_ptr());
    return ret;
}
pub unsafe extern "C" fn mpz_setbit(
    mut d: *mut __mpz_struct,
    mut bit_index: mp_bitcnt_t,
) {
    if mpz_tstbit(d as *const __mpz_struct, bit_index) == 0 {
        if (*d)._mp_size >= 0 as libc::c_int {
            mpz_abs_add_bit(d, bit_index);
        } else {
            mpz_abs_sub_bit(d, bit_index);
        }
    }
}
unsafe extern "C" fn mpz_abs_sub_bit(
    mut d: *mut __mpz_struct,
    mut bit_index: mp_bitcnt_t,
) {
    let mut dn: mp_size_t = 0;
    let mut limb_index: mp_size_t = 0;
    let mut dp: mp_ptr = 0 as *mut mp_limb_t;
    let mut bit: mp_limb_t = 0;
    dn = (if (*d)._mp_size >= 0 as libc::c_int { (*d)._mp_size } else { -(*d)._mp_size })
        as mp_size_t;
    dp = (*d)._mp_d;
    limb_index = bit_index
        .wrapping_div(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as mp_size_t;
    bit = (1 as libc::c_int as mp_limb_t)
        << bit_index
            .wrapping_rem(
                (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            );
    if limb_index < dn {} else {
        __assert_fail(
            b"limb_index < dn\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3738 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void mpz_abs_sub_bit(__mpz_struct *, mp_bitcnt_t)\0"))
                .as_ptr(),
        );
    }
    'c_15032: {
        if limb_index < dn {} else {
            __assert_fail(
                b"limb_index < dn\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3738 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"void mpz_abs_sub_bit(__mpz_struct *, mp_bitcnt_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut __cy: mp_limb_t = mpn_sub_1(
        dp.offset(limb_index as isize),
        dp.offset(limb_index as isize) as mp_srcptr,
        dn - limb_index,
        bit,
    );
    if __cy == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"__cy == 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3741 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void mpz_abs_sub_bit(__mpz_struct *, mp_bitcnt_t)\0"))
                .as_ptr(),
        );
    }
    'c_14968: {
        if __cy == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"__cy == 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3741 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"void mpz_abs_sub_bit(__mpz_struct *, mp_bitcnt_t)\0"))
                    .as_ptr(),
            );
        }
    };
    dn = mpn_normalized_size(dp as mp_srcptr, dn);
    (*d)
        ._mp_size = (if (*d)._mp_size < 0 as libc::c_int { -dn } else { dn })
        as libc::c_int;
}
unsafe extern "C" fn mpz_abs_add_bit(
    mut d: *mut __mpz_struct,
    mut bit_index: mp_bitcnt_t,
) {
    let mut dn: mp_size_t = 0;
    let mut limb_index: mp_size_t = 0;
    let mut bit: mp_limb_t = 0;
    let mut dp: mp_ptr = 0 as *mut mp_limb_t;
    dn = (if (*d)._mp_size >= 0 as libc::c_int { (*d)._mp_size } else { -(*d)._mp_size })
        as mp_size_t;
    limb_index = bit_index
        .wrapping_div(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as mp_size_t;
    bit = (1 as libc::c_int as mp_limb_t)
        << bit_index
            .wrapping_rem(
                (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            );
    if limb_index >= dn {
        let mut i: mp_size_t = 0;
        dp = if limb_index + 1 as libc::c_int as libc::c_long
            > (*d)._mp_alloc as libc::c_long
        {
            mpz_realloc(d, limb_index + 1 as libc::c_int as libc::c_long)
        } else {
            (*d)._mp_d
        };
        *dp.offset(limb_index as isize) = bit;
        i = dn;
        while i < limb_index {
            *dp.offset(i as isize) = 0 as libc::c_int as mp_limb_t;
            i += 1;
            i;
        }
        dn = limb_index + 1 as libc::c_int as libc::c_long;
    } else {
        let mut cy: mp_limb_t = 0;
        dp = (*d)._mp_d;
        cy = mpn_add_1(
            dp.offset(limb_index as isize),
            dp.offset(limb_index as isize) as mp_srcptr,
            dn - limb_index,
            bit,
        );
        if cy > 0 as libc::c_int as libc::c_ulong {
            dp = if dn + 1 as libc::c_int as libc::c_long
                > (*d)._mp_alloc as libc::c_long
            {
                mpz_realloc(d, dn + 1 as libc::c_int as libc::c_long)
            } else {
                (*d)._mp_d
            };
            let fresh24 = dn;
            dn = dn + 1;
            *dp.offset(fresh24 as isize) = cy;
        }
    }
    (*d)
        ._mp_size = (if (*d)._mp_size < 0 as libc::c_int { -dn } else { dn })
        as libc::c_int;
}
pub unsafe extern "C" fn mpz_tstbit(
    mut d: *const __mpz_struct,
    mut bit_index: mp_bitcnt_t,
) -> libc::c_int {
    let mut limb_index: mp_size_t = 0;
    let mut shift: libc::c_uint = 0;
    let mut ds: mp_size_t = 0;
    let mut dn: mp_size_t = 0;
    let mut w: mp_limb_t = 0;
    let mut bit: libc::c_int = 0;
    ds = (*d)._mp_size as mp_size_t;
    dn = if ds >= 0 as libc::c_int as libc::c_long { ds } else { -ds };
    limb_index = bit_index
        .wrapping_div(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as mp_size_t;
    if limb_index >= dn {
        return (ds < 0 as libc::c_int as libc::c_long) as libc::c_int;
    }
    shift = bit_index
        .wrapping_rem(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as libc::c_uint;
    w = *((*d)._mp_d).offset(limb_index as isize);
    bit = (w >> shift & 1 as libc::c_int as libc::c_ulong) as libc::c_int;
    if ds < 0 as libc::c_int as libc::c_long {
        if shift > 0 as libc::c_int as libc::c_uint
            && w
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(shift as libc::c_ulong)
                > 0 as libc::c_int as libc::c_ulong
        {
            return bit ^ 1 as libc::c_int;
        }
        loop {
            limb_index -= 1;
            if !(limb_index >= 0 as libc::c_int as libc::c_long) {
                break;
            }
            if *((*d)._mp_d).offset(limb_index as isize)
                > 0 as libc::c_int as libc::c_ulong
            {
                return bit ^ 1 as libc::c_int;
            }
        }
    }
    return bit;
}
pub unsafe extern "C" fn mpz_tdiv_q_ui(
    mut q: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: libc::c_ulong,
) -> libc::c_ulong {
    return mpz_div_qr_ui(q, 0 as *mut __mpz_struct, n, d, GMP_DIV_TRUNC);
}
pub unsafe extern "C" fn mpz_pow_ui(
    mut r: *mut __mpz_struct,
    mut b: *const __mpz_struct,
    mut e: libc::c_ulong,
) {
    let mut bit: libc::c_ulong = 0;
    let mut tr: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init_set_ui(tr.as_mut_ptr(), 1 as libc::c_int as libc::c_ulong);
    bit = (1 as libc::c_int as libc::c_ulong)
        << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    loop {
        mpz_mul(
            tr.as_mut_ptr(),
            tr.as_mut_ptr() as *const __mpz_struct,
            tr.as_mut_ptr() as *const __mpz_struct,
        );
        if e & bit != 0 {
            mpz_mul(tr.as_mut_ptr(), tr.as_mut_ptr() as *const __mpz_struct, b);
        }
        bit >>= 1 as libc::c_int;
        if !(bit > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    mpz_swap(r, tr.as_mut_ptr());
    mpz_clear(tr.as_mut_ptr());
}
pub unsafe extern "C" fn mpn_sqrtrem(
    mut sp: mp_ptr,
    mut rp: mp_ptr,
    mut p: mp_srcptr,
    mut n: mp_size_t,
) -> mp_size_t {
    let mut s: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut r: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut u: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut res: mp_size_t = 0;
    if n > 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"n > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3295 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"mp_size_t mpn_sqrtrem(mp_ptr, mp_ptr, mp_srcptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_16107: {
        if n > 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3295 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"mp_size_t mpn_sqrtrem(mp_ptr, mp_ptr, mp_srcptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if *p.offset((n - 1 as libc::c_int as libc::c_long) as isize)
        != 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"p [n-1] != 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3296 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"mp_size_t mpn_sqrtrem(mp_ptr, mp_ptr, mp_srcptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_16055: {
        if *p.offset((n - 1 as libc::c_int as libc::c_long) as isize)
            != 0 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"p [n-1] != 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3296 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"mp_size_t mpn_sqrtrem(mp_ptr, mp_ptr, mp_srcptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    mpz_init(r.as_mut_ptr());
    mpz_init(s.as_mut_ptr());
    mpz_rootrem(
        s.as_mut_ptr(),
        r.as_mut_ptr(),
        mpz_roinit_normal_n(u.as_mut_ptr(), p, n),
        2 as libc::c_int as libc::c_ulong,
    );
    if (*s.as_mut_ptr())._mp_size as libc::c_long
        == (n + 1 as libc::c_int as libc::c_long) / 2 as libc::c_int as libc::c_long
    {} else {
        __assert_fail(
            b"s->_mp_size == (n+1)/2\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3302 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"mp_size_t mpn_sqrtrem(mp_ptr, mp_ptr, mp_srcptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_15966: {
        if (*s.as_mut_ptr())._mp_size as libc::c_long
            == (n + 1 as libc::c_int as libc::c_long) / 2 as libc::c_int as libc::c_long
        {} else {
            __assert_fail(
                b"s->_mp_size == (n+1)/2\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3302 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"mp_size_t mpn_sqrtrem(mp_ptr, mp_ptr, mp_srcptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    mpn_copyd(
        sp,
        (*s.as_mut_ptr())._mp_d as mp_srcptr,
        (*s.as_mut_ptr())._mp_size as mp_size_t,
    );
    mpz_clear(s.as_mut_ptr());
    res = (*r.as_mut_ptr())._mp_size as mp_size_t;
    if !rp.is_null() {
        mpn_copyd(rp, (*r.as_mut_ptr())._mp_d as mp_srcptr, res);
    }
    mpz_clear(r.as_mut_ptr());
    return res;
}
pub unsafe extern "C" fn mpz_rootrem(
    mut x: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut y: *const __mpz_struct,
    mut z: libc::c_ulong,
) {
    let mut sgn: libc::c_int = 0;
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut u: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    sgn = ((*y)._mp_size < 0 as libc::c_int) as libc::c_int;
    if !z & sgn as libc::c_ulong != 0 as libc::c_int as libc::c_ulong {
        gmp_die(
            b"mpz_rootrem: Negative argument, with even root.\0" as *const u8
                as *const libc::c_char,
        );
    }
    if z == 0 as libc::c_int as libc::c_ulong {
        gmp_die(b"mpz_rootrem: Zeroth root.\0" as *const u8 as *const libc::c_char);
    }
    if mpz_cmpabs_ui(y, 1 as libc::c_int as libc::c_ulong) <= 0 as libc::c_int {
        if !x.is_null() {
            mpz_set(x, y);
        }
        if !r.is_null() {
            (*r)._mp_size = 0 as libc::c_int;
        }
        return;
    }
    mpz_init(u.as_mut_ptr());
    mpz_init(t.as_mut_ptr());
    mpz_setbit(
        t.as_mut_ptr(),
        (mpz_sizeinbase(y, 2 as libc::c_int))
            .wrapping_div(z)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    if z == 2 as libc::c_int as libc::c_ulong {
        loop {
            mpz_swap(u.as_mut_ptr(), t.as_mut_ptr());
            mpz_tdiv_q(t.as_mut_ptr(), y, u.as_mut_ptr() as *const __mpz_struct);
            mpz_add(
                t.as_mut_ptr(),
                t.as_mut_ptr() as *const __mpz_struct,
                u.as_mut_ptr() as *const __mpz_struct,
            );
            mpz_tdiv_q_2exp(
                t.as_mut_ptr(),
                t.as_mut_ptr() as *const __mpz_struct,
                1 as libc::c_int as mp_bitcnt_t,
            );
            if !(mpz_cmpabs(
                t.as_mut_ptr() as *const __mpz_struct,
                u.as_mut_ptr() as *const __mpz_struct,
            ) < 0 as libc::c_int)
            {
                break;
            }
        }
    } else {
        let mut v: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        mpz_init(v.as_mut_ptr());
        if sgn != 0 {
            mpz_neg(t.as_mut_ptr(), t.as_mut_ptr() as *const __mpz_struct);
        }
        loop {
            mpz_swap(u.as_mut_ptr(), t.as_mut_ptr());
            mpz_pow_ui(
                t.as_mut_ptr(),
                u.as_mut_ptr() as *const __mpz_struct,
                z.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            mpz_tdiv_q(t.as_mut_ptr(), y, t.as_mut_ptr() as *const __mpz_struct);
            mpz_mul_ui(
                v.as_mut_ptr(),
                u.as_mut_ptr() as *const __mpz_struct,
                z.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            mpz_add(
                t.as_mut_ptr(),
                t.as_mut_ptr() as *const __mpz_struct,
                v.as_mut_ptr() as *const __mpz_struct,
            );
            mpz_tdiv_q_ui(t.as_mut_ptr(), t.as_mut_ptr() as *const __mpz_struct, z);
            if !(mpz_cmpabs(
                t.as_mut_ptr() as *const __mpz_struct,
                u.as_mut_ptr() as *const __mpz_struct,
            ) < 0 as libc::c_int)
            {
                break;
            }
        }
        mpz_clear(v.as_mut_ptr());
    }
    if !r.is_null() {
        mpz_pow_ui(t.as_mut_ptr(), u.as_mut_ptr() as *const __mpz_struct, z);
        mpz_sub(r, y, t.as_mut_ptr() as *const __mpz_struct);
    }
    if !x.is_null() {
        mpz_swap(x, u.as_mut_ptr());
    }
    mpz_clear(u.as_mut_ptr());
    mpz_clear(t.as_mut_ptr());
}
pub unsafe extern "C" fn mpz_root(
    mut x: *mut __mpz_struct,
    mut y: *const __mpz_struct,
    mut z: libc::c_ulong,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut r: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init(r.as_mut_ptr());
    mpz_rootrem(x, r.as_mut_ptr(), y, z);
    res = ((*r.as_mut_ptr())._mp_size == 0 as libc::c_int) as libc::c_int;
    mpz_clear(r.as_mut_ptr());
    return res;
}
pub unsafe extern "C" fn mpn_perfect_square_p(
    mut p: mp_srcptr,
    mut n: mp_size_t,
) -> libc::c_int {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    if n > 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"n > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3284 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"int mpn_perfect_square_p(mp_srcptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_15855: {
        if n > 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3284 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"int mpn_perfect_square_p(mp_srcptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if *p.offset((n - 1 as libc::c_int as libc::c_long) as isize)
        != 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"p [n-1] != 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3285 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"int mpn_perfect_square_p(mp_srcptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_15803: {
        if *p.offset((n - 1 as libc::c_int as libc::c_long) as isize)
            != 0 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"p [n-1] != 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3285 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"int mpn_perfect_square_p(mp_srcptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    return mpz_root(
        0 as *mut __mpz_struct,
        mpz_roinit_normal_n(t.as_mut_ptr(), p, n),
        2 as libc::c_int as libc::c_ulong,
    );
}
pub unsafe extern "C" fn mpn_popcount(
    mut p: mp_srcptr,
    mut n: mp_size_t,
) -> mp_bitcnt_t {
    let mut i: mp_size_t = 0;
    let mut c: mp_bitcnt_t = 0;
    c = 0 as libc::c_int as mp_bitcnt_t;
    i = 0 as libc::c_int as mp_size_t;
    while i < n {
        c = (c as libc::c_ulong)
            .wrapping_add(gmp_popcount_limb(*p.offset(i as isize)) as libc::c_ulong)
            as mp_bitcnt_t as mp_bitcnt_t;
        i += 1;
        i;
    }
    return c;
}
unsafe extern "C" fn gmp_popcount_limb(mut x: mp_limb_t) -> libc::c_uint {
    let mut c: libc::c_uint = 0;
    let mut LOCAL_SHIFT_BITS: libc::c_int = 16 as libc::c_int;
    c = 0 as libc::c_int as libc::c_uint;
    while x > 0 as libc::c_int as libc::c_ulong {
        let mut w: libc::c_uint = x
            .wrapping_sub(x >> 1 as libc::c_int & 0x5555 as libc::c_int as libc::c_ulong)
            as libc::c_uint;
        w = (w >> 2 as libc::c_int & 0x3333 as libc::c_int as libc::c_uint)
            .wrapping_add(w & 0x3333 as libc::c_int as libc::c_uint);
        w = (w >> 4 as libc::c_int).wrapping_add(w);
        w = (w >> 8 as libc::c_int & 0xf as libc::c_int as libc::c_uint)
            .wrapping_add(w & 0xf as libc::c_int as libc::c_uint);
        c = c.wrapping_add(w);
        if (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            > LOCAL_SHIFT_BITS as libc::c_ulong
        {
            x >>= LOCAL_SHIFT_BITS;
        } else {
            x = 0 as libc::c_int as mp_limb_t;
        }
    }
    return c;
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub unsafe extern "C" fn mpz_congruent_p(
    mut a: *const __mpz_struct,
    mut b: *const __mpz_struct,
    mut m: *const __mpz_struct,
) -> libc::c_int {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut res: libc::c_int = 0;
    if mpz_sgn(m) == 0 as libc::c_int {
        return (mpz_cmp(a, b) == 0 as libc::c_int) as libc::c_int;
    }
    mpz_init(t.as_mut_ptr());
    mpz_sub(t.as_mut_ptr(), a, b);
    res = mpz_divisible_p(t.as_mut_ptr() as *const __mpz_struct, m);
    mpz_clear(t.as_mut_ptr());
    return res;
}
pub unsafe extern "C" fn mpz_cdiv_qr_ui(
    mut q: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: libc::c_ulong,
) -> libc::c_ulong {
    return mpz_div_qr_ui(q, r, n, d, GMP_DIV_CEIL);
}
pub unsafe extern "C" fn mpz_fdiv_qr_ui(
    mut q: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: libc::c_ulong,
) -> libc::c_ulong {
    return mpz_div_qr_ui(q, r, n, d, GMP_DIV_FLOOR);
}
pub unsafe extern "C" fn mpz_tdiv_qr_ui(
    mut q: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: libc::c_ulong,
) -> libc::c_ulong {
    return mpz_div_qr_ui(q, r, n, d, GMP_DIV_TRUNC);
}
pub unsafe extern "C" fn mpz_cdiv_q_ui(
    mut q: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: libc::c_ulong,
) -> libc::c_ulong {
    return mpz_div_qr_ui(q, 0 as *mut __mpz_struct, n, d, GMP_DIV_CEIL);
}
pub unsafe extern "C" fn mpz_fdiv_q_ui(
    mut q: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: libc::c_ulong,
) -> libc::c_ulong {
    return mpz_div_qr_ui(q, 0 as *mut __mpz_struct, n, d, GMP_DIV_FLOOR);
}
pub unsafe extern "C" fn mpz_cdiv_r_ui(
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: libc::c_ulong,
) -> libc::c_ulong {
    return mpz_div_qr_ui(0 as *mut __mpz_struct, r, n, d, GMP_DIV_CEIL);
}
pub unsafe extern "C" fn mpz_fdiv_r_ui(
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: libc::c_ulong,
) -> libc::c_ulong {
    return mpz_div_qr_ui(0 as *mut __mpz_struct, r, n, d, GMP_DIV_FLOOR);
}
pub unsafe extern "C" fn mpz_tdiv_r_ui(
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: libc::c_ulong,
) -> libc::c_ulong {
    return mpz_div_qr_ui(0 as *mut __mpz_struct, r, n, d, GMP_DIV_TRUNC);
}
pub unsafe extern "C" fn mpz_cdiv_ui(
    mut n: *const __mpz_struct,
    mut d: libc::c_ulong,
) -> libc::c_ulong {
    return mpz_div_qr_ui(
        0 as *mut __mpz_struct,
        0 as *mut __mpz_struct,
        n,
        d,
        GMP_DIV_CEIL,
    );
}
pub unsafe extern "C" fn mpz_fdiv_ui(
    mut n: *const __mpz_struct,
    mut d: libc::c_ulong,
) -> libc::c_ulong {
    return mpz_div_qr_ui(
        0 as *mut __mpz_struct,
        0 as *mut __mpz_struct,
        n,
        d,
        GMP_DIV_FLOOR,
    );
}
pub unsafe extern "C" fn mpz_tdiv_ui(
    mut n: *const __mpz_struct,
    mut d: libc::c_ulong,
) -> libc::c_ulong {
    return mpz_div_qr_ui(
        0 as *mut __mpz_struct,
        0 as *mut __mpz_struct,
        n,
        d,
        GMP_DIV_TRUNC,
    );
}
pub unsafe extern "C" fn mpz_mod_ui(
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: libc::c_ulong,
) -> libc::c_ulong {
    return mpz_div_qr_ui(0 as *mut __mpz_struct, r, n, d, GMP_DIV_FLOOR);
}
pub unsafe extern "C" fn mpz_divexact_ui(
    mut q: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: libc::c_ulong,
) {
    let mut __cy: mp_limb_t = mpz_div_qr_ui(
        q,
        0 as *mut __mpz_struct,
        n,
        d,
        GMP_DIV_TRUNC,
    );
    if __cy == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"__cy == 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            2635 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"void mpz_divexact_ui(__mpz_struct *, const __mpz_struct *, unsigned long)\0",
            ))
                .as_ptr(),
        );
    }
    'c_20634: {
        if __cy == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"__cy == 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                2635 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"void mpz_divexact_ui(__mpz_struct *, const __mpz_struct *, unsigned long)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
pub unsafe extern "C" fn mpz_divisible_ui_p(
    mut n: *const __mpz_struct,
    mut d: libc::c_ulong,
) -> libc::c_int {
    return (mpz_div_qr_ui(
        0 as *mut __mpz_struct,
        0 as *mut __mpz_struct,
        n,
        d,
        GMP_DIV_TRUNC,
    ) == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
pub unsafe extern "C" fn mpz_gcd_ui(
    mut g: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: libc::c_ulong,
) -> libc::c_ulong {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init_set_ui(t.as_mut_ptr(), v);
    mpz_gcd(t.as_mut_ptr(), u, t.as_mut_ptr() as *const __mpz_struct);
    if v > 0 as libc::c_int as libc::c_ulong {
        v = mpz_get_ui(t.as_mut_ptr() as *const __mpz_struct);
    }
    if !g.is_null() {
        mpz_swap(t.as_mut_ptr(), g);
    }
    mpz_clear(t.as_mut_ptr());
    return v;
}
pub unsafe extern "C" fn mpz_gcd(
    mut g: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) {
    let mut tu: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut tv: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut uz: mp_bitcnt_t = 0;
    let mut vz: mp_bitcnt_t = 0;
    let mut gz: mp_bitcnt_t = 0;
    if (*u)._mp_size == 0 as libc::c_int {
        mpz_abs(g, v);
        return;
    }
    if (*v)._mp_size == 0 as libc::c_int {
        mpz_abs(g, u);
        return;
    }
    mpz_init(tu.as_mut_ptr());
    mpz_init(tv.as_mut_ptr());
    mpz_abs(tu.as_mut_ptr(), u);
    uz = mpz_make_odd(tu.as_mut_ptr());
    mpz_abs(tv.as_mut_ptr(), v);
    vz = mpz_make_odd(tv.as_mut_ptr());
    gz = if uz < vz { uz } else { vz };
    if (*tu.as_mut_ptr())._mp_size < (*tv.as_mut_ptr())._mp_size {
        mpz_swap(tu.as_mut_ptr(), tv.as_mut_ptr());
    }
    mpz_tdiv_r(
        tu.as_mut_ptr(),
        tu.as_mut_ptr() as *const __mpz_struct,
        tv.as_mut_ptr() as *const __mpz_struct,
    );
    if (*tu.as_mut_ptr())._mp_size == 0 as libc::c_int {
        mpz_swap(g, tv.as_mut_ptr());
    } else {
        loop {
            let mut c: libc::c_int = 0;
            mpz_make_odd(tu.as_mut_ptr());
            c = mpz_cmp(
                tu.as_mut_ptr() as *const __mpz_struct,
                tv.as_mut_ptr() as *const __mpz_struct,
            );
            if c == 0 as libc::c_int {
                mpz_swap(g, tu.as_mut_ptr());
                break;
            } else {
                if c < 0 as libc::c_int {
                    mpz_swap(tu.as_mut_ptr(), tv.as_mut_ptr());
                }
                if (*tv.as_mut_ptr())._mp_size == 1 as libc::c_int {
                    let mut vl: mp_limb_t = *((*tv.as_mut_ptr())._mp_d)
                        .offset(0 as libc::c_int as isize);
                    let mut ul: mp_limb_t = mpz_tdiv_ui(
                        tu.as_mut_ptr() as *const __mpz_struct,
                        vl,
                    );
                    mpz_set_ui(g, mpn_gcd_11(ul, vl));
                    break;
                } else {
                    mpz_sub(
                        tu.as_mut_ptr(),
                        tu.as_mut_ptr() as *const __mpz_struct,
                        tv.as_mut_ptr() as *const __mpz_struct,
                    );
                }
            }
        }
    }
    mpz_clear(tu.as_mut_ptr());
    mpz_clear(tv.as_mut_ptr());
    mpz_mul_2exp(g, g as *const __mpz_struct, gz);
}
unsafe extern "C" fn mpn_gcd_11(mut u: mp_limb_t, mut v: mp_limb_t) -> mp_limb_t {
    let mut shift: libc::c_uint = 0;
    if u | v > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"(u | v) > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            2651 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"mp_limb_t mpn_gcd_11(mp_limb_t, mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_21134: {
        if u | v > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"(u | v) > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                2651 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"mp_limb_t mpn_gcd_11(mp_limb_t, mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if u == 0 as libc::c_int as libc::c_ulong {
        return v
    } else if v == 0 as libc::c_int as libc::c_ulong {
        return u
    }
    let mut __ctz_x: mp_limb_t = u | v;
    let mut __ctz_c: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut __clz_x: mp_limb_t = __ctz_x & __ctz_x.wrapping_neg();
    let mut __clz_c: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut LOCAL_SHIFT_BITS: libc::c_int = 8 as libc::c_int;
    if (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        > LOCAL_SHIFT_BITS as libc::c_ulong
    {
        while __clz_x
            & (0xff as libc::c_int as mp_limb_t)
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(8 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        {
            __clz_x <<= LOCAL_SHIFT_BITS;
            __clz_c = __clz_c.wrapping_add(8 as libc::c_int as libc::c_uint);
        }
    }
    while __clz_x
        & (1 as libc::c_int as mp_limb_t)
            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        __clz_x <<= 1 as libc::c_int;
        __clz_c = __clz_c.wrapping_add(1);
        __clz_c;
    }
    __ctz_c = __clz_c;
    shift = (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_sub(__ctz_c as libc::c_ulong) as libc::c_uint;
    u >>= shift;
    v >>= shift;
    if u & 1 as libc::c_int as libc::c_ulong == 0 as libc::c_int as libc::c_ulong {
        let mut __mp_limb_t_swap__tmp: mp_limb_t = u;
        u = v;
        v = __mp_limb_t_swap__tmp;
    }
    while v & 1 as libc::c_int as libc::c_ulong == 0 as libc::c_int as libc::c_ulong {
        v >>= 1 as libc::c_int;
    }
    while u != v {
        if u > v {
            u = (u as libc::c_ulong).wrapping_sub(v) as mp_limb_t as mp_limb_t;
            loop {
                u >>= 1 as libc::c_int;
                if !(u & 1 as libc::c_int as libc::c_ulong
                    == 0 as libc::c_int as libc::c_ulong)
                {
                    break;
                }
            }
        } else {
            v = (v as libc::c_ulong).wrapping_sub(u) as mp_limb_t as mp_limb_t;
            loop {
                v >>= 1 as libc::c_int;
                if !(v & 1 as libc::c_int as libc::c_ulong
                    == 0 as libc::c_int as libc::c_ulong)
                {
                    break;
                }
            }
        }
    }
    return u << shift;
}
unsafe extern "C" fn mpz_make_odd(mut r: *mut __mpz_struct) -> mp_bitcnt_t {
    let mut shift: mp_bitcnt_t = 0;
    if (*r)._mp_size > 0 as libc::c_int {} else {
        __assert_fail(
            b"r->_mp_size > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            2711 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"mp_bitcnt_t mpz_make_odd(__mpz_struct *)\0"))
                .as_ptr(),
        );
    }
    'c_21278: {
        if (*r)._mp_size > 0 as libc::c_int {} else {
            __assert_fail(
                b"r->_mp_size > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                2711 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"mp_bitcnt_t mpz_make_odd(__mpz_struct *)\0"))
                    .as_ptr(),
            );
        }
    };
    shift = mpn_common_scan(
        *((*r)._mp_d).offset(0 as libc::c_int as isize),
        0 as libc::c_int as mp_size_t,
        (*r)._mp_d as mp_srcptr,
        0 as libc::c_int as mp_size_t,
        0 as libc::c_int as mp_limb_t,
    );
    mpz_tdiv_q_2exp(r, r as *const __mpz_struct, shift);
    return shift;
}
pub unsafe extern "C" fn mpz_gcdext(
    mut g: *mut __mpz_struct,
    mut s: *mut __mpz_struct,
    mut t: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) {
    let mut tu: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut tv: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut s0: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut s1: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut t0: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut t1: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut uz: mp_bitcnt_t = 0;
    let mut vz: mp_bitcnt_t = 0;
    let mut gz: mp_bitcnt_t = 0;
    let mut power: mp_bitcnt_t = 0;
    if (*u)._mp_size == 0 as libc::c_int {
        let mut sign: libc::c_long = mpz_sgn(v) as libc::c_long;
        mpz_abs(g, v);
        if !s.is_null() {
            (*s)._mp_size = 0 as libc::c_int;
        }
        if !t.is_null() {
            mpz_set_si(t, sign);
        }
        return;
    }
    if (*v)._mp_size == 0 as libc::c_int {
        let mut sign_0: libc::c_long = mpz_sgn(u) as libc::c_long;
        mpz_abs(g, u);
        if !s.is_null() {
            mpz_set_si(s, sign_0);
        }
        if !t.is_null() {
            (*t)._mp_size = 0 as libc::c_int;
        }
        return;
    }
    mpz_init(tu.as_mut_ptr());
    mpz_init(tv.as_mut_ptr());
    mpz_init(s0.as_mut_ptr());
    mpz_init(s1.as_mut_ptr());
    mpz_init(t0.as_mut_ptr());
    mpz_init(t1.as_mut_ptr());
    mpz_abs(tu.as_mut_ptr(), u);
    uz = mpz_make_odd(tu.as_mut_ptr());
    mpz_abs(tv.as_mut_ptr(), v);
    vz = mpz_make_odd(tv.as_mut_ptr());
    gz = if uz < vz { uz } else { vz };
    uz = (uz as libc::c_ulong).wrapping_sub(gz) as mp_bitcnt_t as mp_bitcnt_t;
    vz = (vz as libc::c_ulong).wrapping_sub(gz) as mp_bitcnt_t as mp_bitcnt_t;
    if (*tu.as_mut_ptr())._mp_size < (*tv.as_mut_ptr())._mp_size {
        mpz_swap(tu.as_mut_ptr(), tv.as_mut_ptr());
        let mut __mpz_srcptr_swap__tmp: mpz_srcptr = u;
        u = v;
        v = __mpz_srcptr_swap__tmp;
        let mut __mpz_ptr_swap__tmp: mpz_ptr = s;
        s = t;
        t = __mpz_ptr_swap__tmp;
        let mut __mp_bitcnt_t_swap__tmp: mp_bitcnt_t = uz;
        uz = vz;
        vz = __mp_bitcnt_t_swap__tmp;
    }
    mpz_setbit(t0.as_mut_ptr(), uz);
    mpz_tdiv_qr(
        t1.as_mut_ptr(),
        tu.as_mut_ptr(),
        tu.as_mut_ptr() as *const __mpz_struct,
        tv.as_mut_ptr() as *const __mpz_struct,
    );
    mpz_mul_2exp(t1.as_mut_ptr(), t1.as_mut_ptr() as *const __mpz_struct, uz);
    mpz_setbit(s1.as_mut_ptr(), vz);
    power = uz.wrapping_add(vz);
    if (*tu.as_mut_ptr())._mp_size > 0 as libc::c_int {
        let mut shift: mp_bitcnt_t = 0;
        shift = mpz_make_odd(tu.as_mut_ptr());
        mpz_mul_2exp(t0.as_mut_ptr(), t0.as_mut_ptr() as *const __mpz_struct, shift);
        mpz_mul_2exp(s0.as_mut_ptr(), s0.as_mut_ptr() as *const __mpz_struct, shift);
        power = (power as libc::c_ulong).wrapping_add(shift) as mp_bitcnt_t
            as mp_bitcnt_t;
        loop {
            let mut c: libc::c_int = 0;
            c = mpz_cmp(
                tu.as_mut_ptr() as *const __mpz_struct,
                tv.as_mut_ptr() as *const __mpz_struct,
            );
            if c == 0 as libc::c_int {
                break;
            }
            if c < 0 as libc::c_int {
                mpz_sub(
                    tv.as_mut_ptr(),
                    tv.as_mut_ptr() as *const __mpz_struct,
                    tu.as_mut_ptr() as *const __mpz_struct,
                );
                mpz_add(
                    t0.as_mut_ptr(),
                    t0.as_mut_ptr() as *const __mpz_struct,
                    t1.as_mut_ptr() as *const __mpz_struct,
                );
                mpz_add(
                    s0.as_mut_ptr(),
                    s0.as_mut_ptr() as *const __mpz_struct,
                    s1.as_mut_ptr() as *const __mpz_struct,
                );
                shift = mpz_make_odd(tv.as_mut_ptr());
                mpz_mul_2exp(
                    t1.as_mut_ptr(),
                    t1.as_mut_ptr() as *const __mpz_struct,
                    shift,
                );
                mpz_mul_2exp(
                    s1.as_mut_ptr(),
                    s1.as_mut_ptr() as *const __mpz_struct,
                    shift,
                );
            } else {
                mpz_sub(
                    tu.as_mut_ptr(),
                    tu.as_mut_ptr() as *const __mpz_struct,
                    tv.as_mut_ptr() as *const __mpz_struct,
                );
                mpz_add(
                    t1.as_mut_ptr(),
                    t0.as_mut_ptr() as *const __mpz_struct,
                    t1.as_mut_ptr() as *const __mpz_struct,
                );
                mpz_add(
                    s1.as_mut_ptr(),
                    s0.as_mut_ptr() as *const __mpz_struct,
                    s1.as_mut_ptr() as *const __mpz_struct,
                );
                shift = mpz_make_odd(tu.as_mut_ptr());
                mpz_mul_2exp(
                    t0.as_mut_ptr(),
                    t0.as_mut_ptr() as *const __mpz_struct,
                    shift,
                );
                mpz_mul_2exp(
                    s0.as_mut_ptr(),
                    s0.as_mut_ptr() as *const __mpz_struct,
                    shift,
                );
            }
            power = (power as libc::c_ulong).wrapping_add(shift) as mp_bitcnt_t
                as mp_bitcnt_t;
        }
    }
    mpz_mul_2exp(tv.as_mut_ptr(), tv.as_mut_ptr() as *const __mpz_struct, gz);
    mpz_neg(s0.as_mut_ptr(), s0.as_mut_ptr() as *const __mpz_struct);
    mpz_divexact(s1.as_mut_ptr(), v, tv.as_mut_ptr() as *const __mpz_struct);
    mpz_abs(s1.as_mut_ptr(), s1.as_mut_ptr() as *const __mpz_struct);
    mpz_divexact(t1.as_mut_ptr(), u, tv.as_mut_ptr() as *const __mpz_struct);
    mpz_abs(t1.as_mut_ptr(), t1.as_mut_ptr() as *const __mpz_struct);
    loop {
        let fresh25 = power;
        power = power.wrapping_sub(1);
        if !(fresh25 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        if ((*s0.as_mut_ptr())._mp_size != 0 as libc::c_int) as libc::c_int
            & *((*s0.as_mut_ptr())._mp_d).offset(0 as libc::c_int as isize)
                as libc::c_int != 0
            || ((*t0.as_mut_ptr())._mp_size != 0 as libc::c_int) as libc::c_int
                & *((*t0.as_mut_ptr())._mp_d).offset(0 as libc::c_int as isize)
                    as libc::c_int != 0
        {
            mpz_sub(
                s0.as_mut_ptr(),
                s0.as_mut_ptr() as *const __mpz_struct,
                s1.as_mut_ptr() as *const __mpz_struct,
            );
            mpz_add(
                t0.as_mut_ptr(),
                t0.as_mut_ptr() as *const __mpz_struct,
                t1.as_mut_ptr() as *const __mpz_struct,
            );
        }
        if ((*t0.as_mut_ptr())._mp_size != 0 as libc::c_int) as libc::c_int
            & *((*t0.as_mut_ptr())._mp_d).offset(0 as libc::c_int as isize)
                as libc::c_int == 0
            && ((*s0.as_mut_ptr())._mp_size != 0 as libc::c_int) as libc::c_int
                & *((*s0.as_mut_ptr())._mp_d).offset(0 as libc::c_int as isize)
                    as libc::c_int == 0
        {} else {
            __assert_fail(
                b"mpz_even_p (t0) && mpz_even_p (s0)\0" as *const u8
                    as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                2934 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 108],
                    &[libc::c_char; 108],
                >(
                    b"void mpz_gcdext(__mpz_struct *, __mpz_struct *, __mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_21671: {
            if ((*t0.as_mut_ptr())._mp_size != 0 as libc::c_int) as libc::c_int
                & *((*t0.as_mut_ptr())._mp_d).offset(0 as libc::c_int as isize)
                    as libc::c_int == 0
                && ((*s0.as_mut_ptr())._mp_size != 0 as libc::c_int) as libc::c_int
                    & *((*s0.as_mut_ptr())._mp_d).offset(0 as libc::c_int as isize)
                        as libc::c_int == 0
            {} else {
                __assert_fail(
                    b"mpz_even_p (t0) && mpz_even_p (s0)\0" as *const u8
                        as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    2934 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 108],
                        &[libc::c_char; 108],
                    >(
                        b"void mpz_gcdext(__mpz_struct *, __mpz_struct *, __mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        mpz_tdiv_q_2exp(
            s0.as_mut_ptr(),
            s0.as_mut_ptr() as *const __mpz_struct,
            1 as libc::c_int as mp_bitcnt_t,
        );
        mpz_tdiv_q_2exp(
            t0.as_mut_ptr(),
            t0.as_mut_ptr() as *const __mpz_struct,
            1 as libc::c_int as mp_bitcnt_t,
        );
    }
    mpz_add(
        s1.as_mut_ptr(),
        s0.as_mut_ptr() as *const __mpz_struct,
        s1.as_mut_ptr() as *const __mpz_struct,
    );
    if mpz_cmpabs(
        s0.as_mut_ptr() as *const __mpz_struct,
        s1.as_mut_ptr() as *const __mpz_struct,
    ) > 0 as libc::c_int
    {
        mpz_swap(s0.as_mut_ptr(), s1.as_mut_ptr());
        mpz_sub(
            t0.as_mut_ptr(),
            t0.as_mut_ptr() as *const __mpz_struct,
            t1.as_mut_ptr() as *const __mpz_struct,
        );
    }
    if (*u)._mp_size < 0 as libc::c_int {
        mpz_neg(s0.as_mut_ptr(), s0.as_mut_ptr() as *const __mpz_struct);
    }
    if (*v)._mp_size < 0 as libc::c_int {
        mpz_neg(t0.as_mut_ptr(), t0.as_mut_ptr() as *const __mpz_struct);
    }
    mpz_swap(g, tv.as_mut_ptr());
    if !s.is_null() {
        mpz_swap(s, s0.as_mut_ptr());
    }
    if !t.is_null() {
        mpz_swap(t, t0.as_mut_ptr());
    }
    mpz_clear(tu.as_mut_ptr());
    mpz_clear(tv.as_mut_ptr());
    mpz_clear(s0.as_mut_ptr());
    mpz_clear(s1.as_mut_ptr());
    mpz_clear(t0.as_mut_ptr());
    mpz_clear(t1.as_mut_ptr());
}
pub unsafe extern "C" fn mpz_lcm_ui(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: libc::c_ulong,
) {
    if v == 0 as libc::c_int as libc::c_ulong || (*u)._mp_size == 0 as libc::c_int {
        (*r)._mp_size = 0 as libc::c_int;
        return;
    }
    v = v.wrapping_div(mpz_gcd_ui(0 as *mut __mpz_struct, u, v));
    mpz_mul_ui(r, u, v);
    mpz_abs(r, r as *const __mpz_struct);
}
pub unsafe extern "C" fn mpz_lcm(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) {
    let mut g: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    if (*u)._mp_size == 0 as libc::c_int || (*v)._mp_size == 0 as libc::c_int {
        (*r)._mp_size = 0 as libc::c_int;
        return;
    }
    mpz_init(g.as_mut_ptr());
    mpz_gcd(g.as_mut_ptr(), u, v);
    mpz_divexact(g.as_mut_ptr(), u, g.as_mut_ptr() as *const __mpz_struct);
    mpz_mul(r, g.as_mut_ptr() as *const __mpz_struct, v);
    mpz_clear(g.as_mut_ptr());
    mpz_abs(r, r as *const __mpz_struct);
}
pub unsafe extern "C" fn mpz_invert(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut m: *const __mpz_struct,
) -> libc::c_int {
    let mut g: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut tr: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut invertible: libc::c_int = 0;
    if (*u)._mp_size == 0 as libc::c_int
        || mpz_cmpabs_ui(m, 1 as libc::c_int as libc::c_ulong) <= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    mpz_init(g.as_mut_ptr());
    mpz_init(tr.as_mut_ptr());
    mpz_gcdext(g.as_mut_ptr(), tr.as_mut_ptr(), 0 as *mut __mpz_struct, u, m);
    invertible = (mpz_cmp_ui(
        g.as_mut_ptr() as *const __mpz_struct,
        1 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int) as libc::c_int;
    if invertible != 0 {
        if (*tr.as_mut_ptr())._mp_size < 0 as libc::c_int {
            if (*m)._mp_size >= 0 as libc::c_int {
                mpz_add(tr.as_mut_ptr(), tr.as_mut_ptr() as *const __mpz_struct, m);
            } else {
                mpz_sub(tr.as_mut_ptr(), tr.as_mut_ptr() as *const __mpz_struct, m);
            }
        }
        mpz_swap(r, tr.as_mut_ptr());
    }
    mpz_clear(g.as_mut_ptr());
    mpz_clear(tr.as_mut_ptr());
    return invertible;
}
pub unsafe extern "C" fn mpz_sqrtrem(
    mut s: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
) {
    mpz_rootrem(s, r, u, 2 as libc::c_int as libc::c_ulong);
}
pub unsafe extern "C" fn mpz_sqrt(mut s: *mut __mpz_struct, mut u: *const __mpz_struct) {
    mpz_rootrem(s, 0 as *mut __mpz_struct, u, 2 as libc::c_int as libc::c_ulong);
}
pub unsafe extern "C" fn mpz_perfect_square_p(
    mut u: *const __mpz_struct,
) -> libc::c_int {
    if (*u)._mp_size <= 0 as libc::c_int {
        return ((*u)._mp_size == 0 as libc::c_int) as libc::c_int
    } else {
        return mpz_root(0 as *mut __mpz_struct, u, 2 as libc::c_int as libc::c_ulong)
    };
}
pub unsafe extern "C" fn mpz_ui_pow_ui(
    mut r: *mut __mpz_struct,
    mut blimb: libc::c_ulong,
    mut e: libc::c_ulong,
) {
    let mut b: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init_set_ui(b.as_mut_ptr(), blimb);
    mpz_pow_ui(r, b.as_mut_ptr() as *const __mpz_struct, e);
    mpz_clear(b.as_mut_ptr());
}
pub unsafe extern "C" fn mpz_powm(
    mut r: *mut __mpz_struct,
    mut b: *const __mpz_struct,
    mut e: *const __mpz_struct,
    mut m: *const __mpz_struct,
) {
    let mut tr: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut base: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut en: mp_size_t = 0;
    let mut mn: mp_size_t = 0;
    let mut mp: mp_srcptr = 0 as *const mp_limb_t;
    let mut minv: gmp_div_inverse = gmp_div_inverse {
        shift: 0,
        d1: 0,
        d0: 0,
        di: 0,
    };
    let mut shift: libc::c_uint = 0;
    let mut tp: mp_ptr = 0 as mp_ptr;
    en = (if (*e)._mp_size >= 0 as libc::c_int { (*e)._mp_size } else { -(*e)._mp_size })
        as mp_size_t;
    mn = (if (*m)._mp_size >= 0 as libc::c_int { (*m)._mp_size } else { -(*m)._mp_size })
        as mp_size_t;
    if mn == 0 as libc::c_int as libc::c_long {
        gmp_die(b"mpz_powm: Zero modulo.\0" as *const u8 as *const libc::c_char);
    }
    if en == 0 as libc::c_int as libc::c_long {
        mpz_set_ui(r, 1 as libc::c_int as libc::c_ulong);
        return;
    }
    mp = (*m)._mp_d as mp_srcptr;
    mpn_div_qr_invert(&mut minv, mp, mn);
    shift = minv.shift;
    if shift > 0 as libc::c_int as libc::c_uint {
        minv.shift = 0 as libc::c_int as libc::c_uint;
        tp = gmp_xalloc_limbs(mn);
        let mut __cy: mp_limb_t = mpn_lshift(tp, mp, mn, shift);
        if __cy == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"__cy == 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3100 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"void mpz_powm(__mpz_struct *, const __mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_23276: {
            if __cy == 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"__cy == 0\0" as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    3100 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 96],
                        &[libc::c_char; 96],
                    >(
                        b"void mpz_powm(__mpz_struct *, const __mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        mp = tp as mp_srcptr;
    }
    mpz_init(base.as_mut_ptr());
    if (*e)._mp_size < 0 as libc::c_int {
        if mpz_invert(base.as_mut_ptr(), b, m) == 0 {
            gmp_die(
                b"mpz_powm: Negative exponent and non-invertible base.\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        let mut bn: mp_size_t = 0;
        mpz_abs(base.as_mut_ptr(), b);
        bn = (*base.as_mut_ptr())._mp_size as mp_size_t;
        if bn >= mn {
            mpn_div_qr_preinv(
                0 as mp_ptr,
                (*base.as_mut_ptr())._mp_d,
                (*base.as_mut_ptr())._mp_size as mp_size_t,
                mp,
                mn,
                &mut minv,
            );
            bn = mn;
        }
        if (*b)._mp_size < 0 as libc::c_int {
            let mut bp: mp_ptr = if mn > (*base.as_mut_ptr())._mp_alloc as libc::c_long {
                mpz_realloc(base.as_mut_ptr(), mn)
            } else {
                (*base.as_mut_ptr())._mp_d
            };
            let mut __cy_0: mp_limb_t = mpn_sub(bp, mp, mn, bp as mp_srcptr, bn);
            if __cy_0 == 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"__cy == 0\0" as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    3129 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 96],
                        &[libc::c_char; 96],
                    >(
                        b"void mpz_powm(__mpz_struct *, const __mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_23099: {
                if __cy_0 == 0 as libc::c_int as libc::c_ulong {} else {
                    __assert_fail(
                        b"__cy == 0\0" as *const u8 as *const libc::c_char,
                        b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                        3129 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 96],
                            &[libc::c_char; 96],
                        >(
                            b"void mpz_powm(__mpz_struct *, const __mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            bn = mn;
        }
        (*base.as_mut_ptr())
            ._mp_size = mpn_normalized_size((*base.as_mut_ptr())._mp_d as mp_srcptr, bn)
            as libc::c_int;
    }
    mpz_init_set_ui(tr.as_mut_ptr(), 1 as libc::c_int as libc::c_ulong);
    loop {
        en -= 1;
        if !(en >= 0 as libc::c_int as libc::c_long) {
            break;
        }
        let mut w: mp_limb_t = *((*e)._mp_d).offset(en as isize);
        let mut bit: mp_limb_t = 0;
        bit = (1 as libc::c_int as mp_limb_t)
            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        loop {
            mpz_mul(
                tr.as_mut_ptr(),
                tr.as_mut_ptr() as *const __mpz_struct,
                tr.as_mut_ptr() as *const __mpz_struct,
            );
            if w & bit != 0 {
                mpz_mul(
                    tr.as_mut_ptr(),
                    tr.as_mut_ptr() as *const __mpz_struct,
                    base.as_mut_ptr() as *const __mpz_struct,
                );
            }
            if (*tr.as_mut_ptr())._mp_size as libc::c_long > mn {
                mpn_div_qr_preinv(
                    0 as mp_ptr,
                    (*tr.as_mut_ptr())._mp_d,
                    (*tr.as_mut_ptr())._mp_size as mp_size_t,
                    mp,
                    mn,
                    &mut minv,
                );
                (*tr.as_mut_ptr())
                    ._mp_size = mpn_normalized_size(
                    (*tr.as_mut_ptr())._mp_d as mp_srcptr,
                    mn,
                ) as libc::c_int;
            }
            bit >>= 1 as libc::c_int;
            if !(bit > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
        }
    }
    if (*tr.as_mut_ptr())._mp_size as libc::c_long >= mn {
        minv.shift = shift;
        mpn_div_qr_preinv(
            0 as mp_ptr,
            (*tr.as_mut_ptr())._mp_d,
            (*tr.as_mut_ptr())._mp_size as mp_size_t,
            mp,
            mn,
            &mut minv,
        );
        (*tr.as_mut_ptr())
            ._mp_size = mpn_normalized_size((*tr.as_mut_ptr())._mp_d as mp_srcptr, mn)
            as libc::c_int;
    }
    if !tp.is_null() {
        (Some(gmp_free_func.unwrap()))
            .unwrap()(tp as *mut libc::c_void, 0 as libc::c_int as size_t);
    }
    mpz_swap(r, tr.as_mut_ptr());
    mpz_clear(tr.as_mut_ptr());
    mpz_clear(base.as_mut_ptr());
}
pub unsafe extern "C" fn mpz_powm_ui(
    mut r: *mut __mpz_struct,
    mut b: *const __mpz_struct,
    mut elimb: libc::c_ulong,
    mut m: *const __mpz_struct,
) {
    let mut e: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init_set_ui(e.as_mut_ptr(), elimb);
    mpz_powm(r, b, e.as_mut_ptr() as *const __mpz_struct, m);
    mpz_clear(e.as_mut_ptr());
}
pub unsafe extern "C" fn mpz_fac_ui(mut x: *mut __mpz_struct, mut n: libc::c_ulong) {
    mpz_mfac_uiui(x, n, 1 as libc::c_int as libc::c_ulong);
}
pub unsafe extern "C" fn mpz_mfac_uiui(
    mut x: *mut __mpz_struct,
    mut n: libc::c_ulong,
    mut m: libc::c_ulong,
) {
    mpz_set_ui(
        x,
        n
            .wrapping_add(
                (n == 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_ulong,
            ),
    );
    if m.wrapping_add(1 as libc::c_int as libc::c_ulong)
        < 2 as libc::c_int as libc::c_ulong
    {
        return;
    }
    while n > m.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        n = n.wrapping_sub(m);
        mpz_mul_ui(x, x as *const __mpz_struct, n);
    }
}
pub unsafe extern "C" fn mpz_2fac_ui(mut x: *mut __mpz_struct, mut n: libc::c_ulong) {
    mpz_mfac_uiui(x, n, 2 as libc::c_int as libc::c_ulong);
}
pub unsafe extern "C" fn mpz_bin_uiui(
    mut r: *mut __mpz_struct,
    mut n: libc::c_ulong,
    mut k: libc::c_ulong,
) {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_set_ui(r, (k <= n) as libc::c_int as libc::c_ulong);
    if k > n >> 1 as libc::c_int {
        k = if k <= n { n.wrapping_sub(k) } else { 0 as libc::c_int as libc::c_ulong };
    }
    mpz_init(t.as_mut_ptr());
    mpz_fac_ui(t.as_mut_ptr(), k);
    while k > 0 as libc::c_int as libc::c_ulong {
        let fresh26 = n;
        n = n.wrapping_sub(1);
        mpz_mul_ui(r, r as *const __mpz_struct, fresh26);
        k = k.wrapping_sub(1);
        k;
    }
    mpz_divexact(r, r as *const __mpz_struct, t.as_mut_ptr() as *const __mpz_struct);
    mpz_clear(t.as_mut_ptr());
}
pub unsafe extern "C" fn mpz_probab_prime_p(
    mut n: *const __mpz_struct,
    mut reps: libc::c_int,
) -> libc::c_int {
    let mut nm1: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut q: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut y: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut k: mp_bitcnt_t = 0;
    let mut is_prime: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if ((*n)._mp_size != 0 as libc::c_int) as libc::c_int
        & *((*n)._mp_d).offset(0 as libc::c_int as isize) as libc::c_int == 0
    {
        return if mpz_cmpabs_ui(n, 2 as libc::c_int as libc::c_ulong) == 0 as libc::c_int
        {
            2 as libc::c_int
        } else {
            0 as libc::c_int
        };
    }
    if (*n)._mp_size != 0 as libc::c_int {} else {
        __assert_fail(
            b"n->_mp_size != 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3576 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"int mpz_probab_prime_p(const __mpz_struct *, int)\0"))
                .as_ptr(),
        );
    }
    'c_25740: {
        if (*n)._mp_size != 0 as libc::c_int {} else {
            __assert_fail(
                b"n->_mp_size != 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3576 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"int mpz_probab_prime_p(const __mpz_struct *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if mpz_cmpabs_ui(n, 64 as libc::c_int as libc::c_ulong) < 0 as libc::c_int {
        return (0xc96996dc as libc::c_ulong
            >> (*((*n)._mp_d).offset(0 as libc::c_int as isize) >> 1 as libc::c_int)
            & 2 as libc::c_int as libc::c_ulong) as libc::c_int;
    }
    if mpz_gcd_ui(
        0 as *mut __mpz_struct,
        n,
        (3 as libc::c_ulong)
            .wrapping_mul(5 as libc::c_ulong)
            .wrapping_mul(7 as libc::c_ulong)
            .wrapping_mul(11 as libc::c_ulong)
            .wrapping_mul(13 as libc::c_ulong)
            .wrapping_mul(17 as libc::c_ulong)
            .wrapping_mul(19 as libc::c_ulong)
            .wrapping_mul(23 as libc::c_ulong)
            .wrapping_mul(29 as libc::c_ulong),
    ) != 1 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if mpz_cmpabs_ui(n, (31 as libc::c_int * 31 as libc::c_int) as libc::c_ulong)
        < 0 as libc::c_int
    {
        return 2 as libc::c_int;
    }
    mpz_init(nm1.as_mut_ptr());
    mpz_init(q.as_mut_ptr());
    mpz_abs(nm1.as_mut_ptr(), n);
    let ref mut fresh27 = *((*nm1.as_mut_ptr())._mp_d).offset(0 as libc::c_int as isize);
    *fresh27 = (*fresh27 as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as mp_limb_t as mp_limb_t;
    k = mpz_scan1(
        nm1.as_mut_ptr() as *const __mpz_struct,
        0 as libc::c_int as mp_bitcnt_t,
    );
    mpz_tdiv_q_2exp(q.as_mut_ptr(), nm1.as_mut_ptr() as *const __mpz_struct, k);
    mpz_init_set_ui(y.as_mut_ptr(), 2 as libc::c_int as libc::c_ulong);
    is_prime = (gmp_millerrabin(
        n,
        nm1.as_mut_ptr() as *const __mpz_struct,
        y.as_mut_ptr(),
        q.as_mut_ptr() as *const __mpz_struct,
        k,
    ) != 0 && gmp_stronglucas(n, y.as_mut_ptr()) != 0) as libc::c_int;
    reps -= 24 as libc::c_int;
    j = 0 as libc::c_int;
    while is_prime & (j < reps) as libc::c_int != 0 {
        mpz_set_ui(
            y.as_mut_ptr(),
            (j as libc::c_ulong)
                .wrapping_mul(j as libc::c_ulong)
                .wrapping_add(j as libc::c_ulong)
                .wrapping_add(41 as libc::c_int as libc::c_ulong),
        );
        if mpz_cmp(
            y.as_mut_ptr() as *const __mpz_struct,
            nm1.as_mut_ptr() as *const __mpz_struct,
        ) >= 0 as libc::c_int
        {
            if j >= 30 as libc::c_int {} else {
                __assert_fail(
                    b"j >= 30\0" as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    3614 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 50],
                        &[libc::c_char; 50],
                    >(b"int mpz_probab_prime_p(const __mpz_struct *, int)\0"))
                        .as_ptr(),
                );
            }
            'c_23866: {
                if j >= 30 as libc::c_int {} else {
                    __assert_fail(
                        b"j >= 30\0" as *const u8 as *const libc::c_char,
                        b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                        3614 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 50],
                            &[libc::c_char; 50],
                        >(b"int mpz_probab_prime_p(const __mpz_struct *, int)\0"))
                            .as_ptr(),
                    );
                }
            };
            break;
        } else {
            is_prime = gmp_millerrabin(
                n,
                nm1.as_mut_ptr() as *const __mpz_struct,
                y.as_mut_ptr(),
                q.as_mut_ptr() as *const __mpz_struct,
                k,
            );
            j += 1;
            j;
        }
    }
    mpz_clear(nm1.as_mut_ptr());
    mpz_clear(q.as_mut_ptr());
    mpz_clear(y.as_mut_ptr());
    return is_prime;
}
unsafe extern "C" fn gmp_millerrabin(
    mut n: *const __mpz_struct,
    mut nm1: *const __mpz_struct,
    mut y: *mut __mpz_struct,
    mut q: *const __mpz_struct,
    mut k: mp_bitcnt_t,
) -> libc::c_int {
    if k > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"k > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3531 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 115],
                &[libc::c_char; 115],
            >(
                b"int gmp_millerrabin(const __mpz_struct *, const __mpz_struct *, __mpz_struct *, const __mpz_struct *, mp_bitcnt_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_23823: {
        if k > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"k > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3531 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 115],
                    &[libc::c_char; 115],
                >(
                    b"int gmp_millerrabin(const __mpz_struct *, const __mpz_struct *, __mpz_struct *, const __mpz_struct *, mp_bitcnt_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    mpz_powm(y, y as *const __mpz_struct, q, n);
    if mpz_cmp_ui(y as *const __mpz_struct, 1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int || mpz_cmp(y as *const __mpz_struct, nm1) == 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    loop {
        k = k.wrapping_sub(1);
        if !(k > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        mpz_powm_ui(y, y as *const __mpz_struct, 2 as libc::c_int as libc::c_ulong, n);
        if mpz_cmp(y as *const __mpz_struct, nm1) == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        if mpz_cmp_ui(y as *const __mpz_struct, 1 as libc::c_int as libc::c_ulong)
            <= 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn gmp_stronglucas(
    mut x: *const __mpz_struct,
    mut Qk: *mut __mpz_struct,
) -> libc::c_int {
    let mut b0: mp_bitcnt_t = 0;
    let mut V: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut n: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut maxD: mp_limb_t = 0;
    let mut D: mp_limb_t = 0;
    let mut Q: libc::c_long = 0;
    let mut tl: mp_limb_t = 0;
    mpz_roinit_normal_n(
        n.as_mut_ptr(),
        (*x)._mp_d as mp_srcptr,
        (if (*x)._mp_size >= 0 as libc::c_int { (*x)._mp_size } else { -(*x)._mp_size })
            as mp_size_t,
    );
    if ((*n.as_mut_ptr())._mp_size != 0 as libc::c_int) as libc::c_int
        & *((*n.as_mut_ptr())._mp_d).offset(0 as libc::c_int as isize) as libc::c_int
        != 0
    {} else {
        __assert_fail(
            b"mpz_odd_p (n)\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3486 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"int gmp_stronglucas(const __mpz_struct *, __mpz_struct *)\0"))
                .as_ptr(),
        );
    }
    'c_25331: {
        if ((*n.as_mut_ptr())._mp_size != 0 as libc::c_int) as libc::c_int
            & *((*n.as_mut_ptr())._mp_d).offset(0 as libc::c_int as isize) as libc::c_int
            != 0
        {} else {
            __assert_fail(
                b"mpz_odd_p (n)\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3486 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 58],
                    &[libc::c_char; 58],
                >(b"int gmp_stronglucas(const __mpz_struct *, __mpz_struct *)\0"))
                    .as_ptr(),
            );
        }
    };
    if mpz_root(
        Qk,
        n.as_mut_ptr() as *const __mpz_struct,
        2 as libc::c_int as libc::c_ulong,
    ) != 0
    {
        return 0 as libc::c_int;
    }
    maxD = if (*Qk)._mp_size == 1 as libc::c_int {
        (*((*Qk)._mp_d).offset(0 as libc::c_int as isize))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    } else {
        !(0 as libc::c_int as mp_limb_t)
    };
    D = 3 as libc::c_int as mp_limb_t;
    loop {
        if D >= maxD {
            return 1 as libc::c_int
                + (D != !(0 as libc::c_int as mp_limb_t)) as libc::c_int;
        }
        D = (D as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as mp_limb_t as mp_limb_t;
        tl = mpz_tdiv_ui(n.as_mut_ptr() as *const __mpz_struct, D);
        if tl == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
        if !(gmp_jacobi_coprime(tl, D) == 1 as libc::c_int) {
            break;
        }
    }
    mpz_init(V.as_mut_ptr());
    b0 = mpz_scan0(
        n.as_mut_ptr() as *const __mpz_struct,
        0 as libc::c_int as mp_bitcnt_t,
    );
    Q = if D & 2 as libc::c_int as libc::c_ulong != 0 {
        (D >> 2 as libc::c_int) as libc::c_long + 1 as libc::c_int as libc::c_long
    } else {
        -((D >> 2 as libc::c_int) as libc::c_long)
    };
    if gmp_lucas_mod(V.as_mut_ptr(), Qk, Q, b0, n.as_mut_ptr() as *const __mpz_struct)
        == 0
    {
        while (*V.as_mut_ptr())._mp_size != 0 as libc::c_int
            && {
                b0 = b0.wrapping_sub(1);
                b0 != 0 as libc::c_int as libc::c_ulong
            }
        {
            gmp_lucas_step_k_2k(
                V.as_mut_ptr(),
                Qk,
                n.as_mut_ptr() as *const __mpz_struct,
            );
        }
    }
    mpz_clear(V.as_mut_ptr());
    return (b0 != 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
unsafe extern "C" fn gmp_lucas_step_k_2k(
    mut V: *mut __mpz_struct,
    mut Qk: *mut __mpz_struct,
    mut n: *const __mpz_struct,
) {
    mpz_mod(Qk, Qk as *const __mpz_struct, n);
    mpz_mul(V, V as *const __mpz_struct, V as *const __mpz_struct);
    mpz_submul_ui(V, Qk as *const __mpz_struct, 2 as libc::c_int as libc::c_ulong);
    mpz_tdiv_r(V, V as *const __mpz_struct, n);
    mpz_mul(Qk, Qk as *const __mpz_struct, Qk as *const __mpz_struct);
}
unsafe extern "C" fn gmp_lucas_mod(
    mut V: *mut __mpz_struct,
    mut Qk: *mut __mpz_struct,
    mut Q: libc::c_long,
    mut b0: mp_bitcnt_t,
    mut n: *const __mpz_struct,
) -> libc::c_int {
    let mut bs: mp_bitcnt_t = 0;
    let mut U: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut res: libc::c_int = 0;
    if b0 > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"b0 > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3424 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 91],
                &[libc::c_char; 91],
            >(
                b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_24553: {
        if b0 > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"b0 > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3424 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 91],
                    &[libc::c_char; 91],
                >(
                    b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if Q
        <= -((-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
            / 2 as libc::c_int as libc::c_long)
    {} else {
        __assert_fail(
            b"Q <= - (LONG_MIN / 2)\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3425 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 91],
                &[libc::c_char; 91],
            >(
                b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_24499: {
        if Q
            <= -((-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                / 2 as libc::c_int as libc::c_long)
        {} else {
            __assert_fail(
                b"Q <= - (LONG_MIN / 2)\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3425 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 91],
                    &[libc::c_char; 91],
                >(
                    b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if Q >= -(9223372036854775807 as libc::c_long / 2 as libc::c_int as libc::c_long)
    {} else {
        __assert_fail(
            b"Q >= - (LONG_MAX / 2)\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3426 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 91],
                &[libc::c_char; 91],
            >(
                b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_24453: {
        if Q >= -(9223372036854775807 as libc::c_long / 2 as libc::c_int as libc::c_long)
        {} else {
            __assert_fail(
                b"Q >= - (LONG_MAX / 2)\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3426 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 91],
                    &[libc::c_char; 91],
                >(
                    b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if mpz_cmp_ui(n, 4 as libc::c_int as libc::c_ulong) > 0 as libc::c_int {} else {
        __assert_fail(
            b"mpz_cmp_ui (n, 4) > 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3427 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 91],
                &[libc::c_char; 91],
            >(
                b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_24406: {
        if mpz_cmp_ui(n, 4 as libc::c_int as libc::c_ulong) > 0 as libc::c_int {} else {
            __assert_fail(
                b"mpz_cmp_ui (n, 4) > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3427 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 91],
                    &[libc::c_char; 91],
                >(
                    b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if ((*n)._mp_size != 0 as libc::c_int) as libc::c_int
        & *((*n)._mp_d).offset(0 as libc::c_int as isize) as libc::c_int != 0
    {} else {
        __assert_fail(
            b"mpz_odd_p (n)\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3428 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 91],
                &[libc::c_char; 91],
            >(
                b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_24339: {
        if ((*n)._mp_size != 0 as libc::c_int) as libc::c_int
            & *((*n)._mp_d).offset(0 as libc::c_int as isize) as libc::c_int != 0
        {} else {
            __assert_fail(
                b"mpz_odd_p (n)\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3428 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 91],
                    &[libc::c_char; 91],
                >(
                    b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    mpz_init_set_ui(U.as_mut_ptr(), 1 as libc::c_int as libc::c_ulong);
    mpz_set_ui(V, 1 as libc::c_int as libc::c_ulong);
    mpz_set_si(Qk, Q);
    bs = (mpz_sizeinbase(n, 2 as libc::c_int))
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    loop {
        bs = bs.wrapping_sub(1);
        if !(bs >= b0) {
            break;
        }
        mpz_mul(
            U.as_mut_ptr(),
            U.as_mut_ptr() as *const __mpz_struct,
            V as *const __mpz_struct,
        );
        gmp_lucas_step_k_2k(V, Qk, n);
        if b0 == bs || mpz_tstbit(n, bs) != 0 {
            mpz_mul_si(Qk, Qk as *const __mpz_struct, Q);
            mpz_swap(U.as_mut_ptr(), V);
            mpz_add(
                U.as_mut_ptr(),
                U.as_mut_ptr() as *const __mpz_struct,
                V as *const __mpz_struct,
            );
            if ((*U.as_mut_ptr())._mp_size != 0 as libc::c_int) as libc::c_int
                & *((*U.as_mut_ptr())._mp_d).offset(0 as libc::c_int as isize)
                    as libc::c_int != 0
            {
                mpz_add(U.as_mut_ptr(), U.as_mut_ptr() as *const __mpz_struct, n);
            }
            mpz_tdiv_q_2exp(
                U.as_mut_ptr(),
                U.as_mut_ptr() as *const __mpz_struct,
                1 as libc::c_int as mp_bitcnt_t,
            );
            mpz_mul_si(
                V,
                V as *const __mpz_struct,
                -(2 as libc::c_int) as libc::c_long * Q,
            );
            mpz_add(V, U.as_mut_ptr() as *const __mpz_struct, V as *const __mpz_struct);
            mpz_tdiv_r(V, V as *const __mpz_struct, n);
        }
        mpz_tdiv_r(U.as_mut_ptr(), U.as_mut_ptr() as *const __mpz_struct, n);
    }
    res = ((*U.as_mut_ptr())._mp_size == 0 as libc::c_int) as libc::c_int;
    mpz_clear(U.as_mut_ptr());
    return res;
}
pub unsafe extern "C" fn mpz_scan0(
    mut u: *const __mpz_struct,
    mut starting_bit: mp_bitcnt_t,
) -> mp_bitcnt_t {
    let mut up: mp_ptr = 0 as *mut mp_limb_t;
    let mut us: mp_size_t = 0;
    let mut un: mp_size_t = 0;
    let mut i: mp_size_t = 0;
    let mut limb: mp_limb_t = 0;
    let mut ux: mp_limb_t = 0;
    us = (*u)._mp_size as mp_size_t;
    ux = ((us >= 0 as libc::c_int as libc::c_long) as libc::c_int as mp_limb_t)
        .wrapping_neg();
    un = if us >= 0 as libc::c_int as libc::c_long { us } else { -us };
    i = starting_bit
        .wrapping_div(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as mp_size_t;
    if i >= un {
        return if ux != 0 { starting_bit } else { !(0 as libc::c_int as mp_bitcnt_t) };
    }
    up = (*u)._mp_d;
    limb = *up.offset(i as isize) ^ ux;
    if ux == 0 as libc::c_int as libc::c_ulong {
        limb = (limb as libc::c_ulong)
            .wrapping_sub(mpn_zero_p(up as mp_srcptr, i) as libc::c_ulong) as mp_limb_t
            as mp_limb_t;
    }
    limb
        &= !(0 as libc::c_int as mp_limb_t)
            << starting_bit
                .wrapping_rem(
                    (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                );
    return mpn_common_scan(limb, i, up as mp_srcptr, un, ux);
}
unsafe extern "C" fn gmp_jacobi_coprime(
    mut a: mp_limb_t,
    mut b: mp_limb_t,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut bit: libc::c_int = 0 as libc::c_int;
    if b & 1 as libc::c_int as libc::c_ulong != 0 {} else {
        __assert_fail(
            b"b & 1\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3365 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"int gmp_jacobi_coprime(mp_limb_t, mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_25209: {
        if b & 1 as libc::c_int as libc::c_ulong != 0 {} else {
            __assert_fail(
                b"b & 1\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3365 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"int gmp_jacobi_coprime(mp_limb_t, mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if a != 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"a != 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3366 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"int gmp_jacobi_coprime(mp_limb_t, mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_25171: {
        if a != 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"a != 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3366 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"int gmp_jacobi_coprime(mp_limb_t, mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    b >>= 1 as libc::c_int;
    let mut __ctz_x: mp_limb_t = a;
    let mut __ctz_c: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut __clz_x: mp_limb_t = __ctz_x & __ctz_x.wrapping_neg();
    let mut __clz_c: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut LOCAL_SHIFT_BITS: libc::c_int = 8 as libc::c_int;
    if (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        > LOCAL_SHIFT_BITS as libc::c_ulong
    {
        while __clz_x
            & (0xff as libc::c_int as mp_limb_t)
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(8 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        {
            __clz_x <<= LOCAL_SHIFT_BITS;
            __clz_c = __clz_c.wrapping_add(8 as libc::c_int as libc::c_uint);
        }
    }
    while __clz_x
        & (1 as libc::c_int as mp_limb_t)
            << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        __clz_x <<= 1 as libc::c_int;
        __clz_c = __clz_c.wrapping_add(1);
        __clz_c;
    }
    __ctz_c = __clz_c;
    c = (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_sub(__ctz_c as libc::c_ulong) as libc::c_int;
    a >>= 1 as libc::c_int;
    loop {
        a >>= c;
        bit = (bit as libc::c_ulong ^ c as libc::c_ulong & (b ^ b >> 1 as libc::c_int))
            as libc::c_int;
        if a < b {
            if a == 0 as libc::c_int as libc::c_ulong {
                return if bit & 1 as libc::c_int != 0 {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                };
            }
            bit = (bit as libc::c_ulong ^ a & b) as libc::c_int;
            a = b.wrapping_sub(a);
            b = (b as libc::c_ulong).wrapping_sub(a) as mp_limb_t as mp_limb_t;
        } else {
            a = (a as libc::c_ulong).wrapping_sub(b) as mp_limb_t as mp_limb_t;
            if a != 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"a != 0\0" as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    3392 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 45],
                        &[libc::c_char; 45],
                    >(b"int gmp_jacobi_coprime(mp_limb_t, mp_limb_t)\0"))
                        .as_ptr(),
                );
            }
            'c_24941: {
                if a != 0 as libc::c_int as libc::c_ulong {} else {
                    __assert_fail(
                        b"a != 0\0" as *const u8 as *const libc::c_char,
                        b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                        3392 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 45],
                            &[libc::c_char; 45],
                        >(b"int gmp_jacobi_coprime(mp_limb_t, mp_limb_t)\0"))
                            .as_ptr(),
                    );
                }
            };
        }
        let mut __ctz_x_0: mp_limb_t = a;
        let mut __ctz_c_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut __clz_x_0: mp_limb_t = __ctz_x_0 & __ctz_x_0.wrapping_neg();
        let mut __clz_c_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut LOCAL_SHIFT_BITS_0: libc::c_int = 8 as libc::c_int;
        if (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            > LOCAL_SHIFT_BITS_0 as libc::c_ulong
        {
            while __clz_x_0
                & (0xff as libc::c_int as mp_limb_t)
                    << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(8 as libc::c_int as libc::c_ulong)
                == 0 as libc::c_int as libc::c_ulong
            {
                __clz_x_0 <<= LOCAL_SHIFT_BITS_0;
                __clz_c_0 = __clz_c_0.wrapping_add(8 as libc::c_int as libc::c_uint);
            }
        }
        while __clz_x_0
            & (1 as libc::c_int as mp_limb_t)
                << (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        {
            __clz_x_0 <<= 1 as libc::c_int;
            __clz_c_0 = __clz_c_0.wrapping_add(1);
            __clz_c_0;
        }
        __ctz_c_0 = __clz_c_0;
        c = (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_sub(__ctz_c_0 as libc::c_ulong) as libc::c_int;
        c += 1;
        c;
    };
}
pub unsafe extern "C" fn mpz_scan1(
    mut u: *const __mpz_struct,
    mut starting_bit: mp_bitcnt_t,
) -> mp_bitcnt_t {
    let mut up: mp_ptr = 0 as *mut mp_limb_t;
    let mut us: mp_size_t = 0;
    let mut un: mp_size_t = 0;
    let mut i: mp_size_t = 0;
    let mut limb: mp_limb_t = 0;
    let mut ux: mp_limb_t = 0;
    us = (*u)._mp_size as mp_size_t;
    un = if us >= 0 as libc::c_int as libc::c_long { us } else { -us };
    i = starting_bit
        .wrapping_div(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as mp_size_t;
    if i >= un {
        return if us >= 0 as libc::c_int as libc::c_long {
            !(0 as libc::c_int as mp_bitcnt_t)
        } else {
            starting_bit
        };
    }
    up = (*u)._mp_d;
    ux = 0 as libc::c_int as mp_limb_t;
    limb = *up.offset(i as isize);
    if starting_bit != 0 as libc::c_int as libc::c_ulong {
        if us < 0 as libc::c_int as libc::c_long {
            ux = mpn_zero_p(up as mp_srcptr, i) as mp_limb_t;
            limb = (!limb).wrapping_add(ux);
            ux = ((limb >= ux) as libc::c_int as mp_limb_t).wrapping_neg();
        }
        limb
            &= !(0 as libc::c_int as mp_limb_t)
                << starting_bit
                    .wrapping_rem(
                        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    );
    }
    return mpn_common_scan(limb, i, up as mp_srcptr, un, ux);
}
pub unsafe extern "C" fn mpz_clrbit(
    mut d: *mut __mpz_struct,
    mut bit_index: mp_bitcnt_t,
) {
    if mpz_tstbit(d as *const __mpz_struct, bit_index) != 0 {
        if (*d)._mp_size >= 0 as libc::c_int {
            mpz_abs_sub_bit(d, bit_index);
        } else {
            mpz_abs_add_bit(d, bit_index);
        }
    }
}
pub unsafe extern "C" fn mpz_combit(
    mut d: *mut __mpz_struct,
    mut bit_index: mp_bitcnt_t,
) {
    if mpz_tstbit(d as *const __mpz_struct, bit_index)
        ^ ((*d)._mp_size < 0 as libc::c_int) as libc::c_int != 0
    {
        mpz_abs_sub_bit(d, bit_index);
    } else {
        mpz_abs_add_bit(d, bit_index);
    };
}
pub unsafe extern "C" fn mpz_com(mut r: *mut __mpz_struct, mut u: *const __mpz_struct) {
    mpz_add_ui(r, u, 1 as libc::c_int as libc::c_ulong);
    mpz_neg(r, r as *const __mpz_struct);
}
pub unsafe extern "C" fn mpz_and(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) {
    let mut un: mp_size_t = 0;
    let mut vn: mp_size_t = 0;
    let mut rn: mp_size_t = 0;
    let mut i: mp_size_t = 0;
    let mut up: mp_ptr = 0 as *mut mp_limb_t;
    let mut vp: mp_ptr = 0 as *mut mp_limb_t;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    let mut ux: mp_limb_t = 0;
    let mut vx: mp_limb_t = 0;
    let mut rx: mp_limb_t = 0;
    let mut uc: mp_limb_t = 0;
    let mut vc: mp_limb_t = 0;
    let mut rc: mp_limb_t = 0;
    let mut ul: mp_limb_t = 0;
    let mut vl: mp_limb_t = 0;
    let mut rl: mp_limb_t = 0;
    un = (if (*u)._mp_size >= 0 as libc::c_int { (*u)._mp_size } else { -(*u)._mp_size })
        as mp_size_t;
    vn = (if (*v)._mp_size >= 0 as libc::c_int { (*v)._mp_size } else { -(*v)._mp_size })
        as mp_size_t;
    if un < vn {
        let mut __mpz_srcptr_swap__tmp: mpz_srcptr = u;
        u = v;
        v = __mpz_srcptr_swap__tmp;
        let mut __mp_size_t_swap__tmp: mp_size_t = un;
        un = vn;
        vn = __mp_size_t_swap__tmp;
    }
    if vn == 0 as libc::c_int as libc::c_long {
        (*r)._mp_size = 0 as libc::c_int;
        return;
    }
    uc = ((*u)._mp_size < 0 as libc::c_int) as libc::c_int as mp_limb_t;
    vc = ((*v)._mp_size < 0 as libc::c_int) as libc::c_int as mp_limb_t;
    rc = uc & vc;
    ux = uc.wrapping_neg();
    vx = vc.wrapping_neg();
    rx = rc.wrapping_neg();
    rn = if vx != 0 { un } else { vn };
    rp = if rn + rc as mp_size_t > (*r)._mp_alloc as libc::c_long {
        mpz_realloc(r, rn + rc as mp_size_t)
    } else {
        (*r)._mp_d
    };
    up = (*u)._mp_d;
    vp = (*v)._mp_d;
    i = 0 as libc::c_int as mp_size_t;
    loop {
        ul = (*up.offset(i as isize) ^ ux).wrapping_add(uc);
        uc = (ul < uc) as libc::c_int as mp_limb_t;
        vl = (*vp.offset(i as isize) ^ vx).wrapping_add(vc);
        vc = (vl < vc) as libc::c_int as mp_limb_t;
        rl = (ul & vl ^ rx).wrapping_add(rc);
        rc = (rl < rc) as libc::c_int as mp_limb_t;
        *rp.offset(i as isize) = rl;
        i += 1;
        if !(i < vn) {
            break;
        }
    }
    if vc == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"vc == 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3839 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"void mpz_and(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_26057: {
        if vc == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"vc == 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3839 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"void mpz_and(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while i < rn {
        ul = (*up.offset(i as isize) ^ ux).wrapping_add(uc);
        uc = (ul < uc) as libc::c_int as mp_limb_t;
        rl = (ul & vx ^ rx).wrapping_add(rc);
        rc = (rl < rc) as libc::c_int as mp_limb_t;
        *rp.offset(i as isize) = rl;
        i += 1;
        i;
    }
    if rc != 0 {
        let fresh28 = rn;
        rn = rn + 1;
        *rp.offset(fresh28 as isize) = rc;
    } else {
        rn = mpn_normalized_size(rp as mp_srcptr, rn);
    }
    (*r)._mp_size = (if rx != 0 { -rn } else { rn }) as libc::c_int;
}
pub unsafe extern "C" fn mpz_ior(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) {
    let mut un: mp_size_t = 0;
    let mut vn: mp_size_t = 0;
    let mut rn: mp_size_t = 0;
    let mut i: mp_size_t = 0;
    let mut up: mp_ptr = 0 as *mut mp_limb_t;
    let mut vp: mp_ptr = 0 as *mut mp_limb_t;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    let mut ux: mp_limb_t = 0;
    let mut vx: mp_limb_t = 0;
    let mut rx: mp_limb_t = 0;
    let mut uc: mp_limb_t = 0;
    let mut vc: mp_limb_t = 0;
    let mut rc: mp_limb_t = 0;
    let mut ul: mp_limb_t = 0;
    let mut vl: mp_limb_t = 0;
    let mut rl: mp_limb_t = 0;
    un = (if (*u)._mp_size >= 0 as libc::c_int { (*u)._mp_size } else { -(*u)._mp_size })
        as mp_size_t;
    vn = (if (*v)._mp_size >= 0 as libc::c_int { (*v)._mp_size } else { -(*v)._mp_size })
        as mp_size_t;
    if un < vn {
        let mut __mpz_srcptr_swap__tmp: mpz_srcptr = u;
        u = v;
        v = __mpz_srcptr_swap__tmp;
        let mut __mp_size_t_swap__tmp: mp_size_t = un;
        un = vn;
        vn = __mp_size_t_swap__tmp;
    }
    if vn == 0 as libc::c_int as libc::c_long {
        mpz_set(r, u);
        return;
    }
    uc = ((*u)._mp_size < 0 as libc::c_int) as libc::c_int as mp_limb_t;
    vc = ((*v)._mp_size < 0 as libc::c_int) as libc::c_int as mp_limb_t;
    rc = uc | vc;
    ux = uc.wrapping_neg();
    vx = vc.wrapping_neg();
    rx = rc.wrapping_neg();
    rn = if vx != 0 { vn } else { un };
    rp = if rn + rc as mp_size_t > (*r)._mp_alloc as libc::c_long {
        mpz_realloc(r, rn + rc as mp_size_t)
    } else {
        (*r)._mp_d
    };
    up = (*u)._mp_d;
    vp = (*v)._mp_d;
    i = 0 as libc::c_int as mp_size_t;
    loop {
        ul = (*up.offset(i as isize) ^ ux).wrapping_add(uc);
        uc = (ul < uc) as libc::c_int as mp_limb_t;
        vl = (*vp.offset(i as isize) ^ vx).wrapping_add(vc);
        vc = (vl < vc) as libc::c_int as mp_limb_t;
        rl = ((ul | vl) ^ rx).wrapping_add(rc);
        rc = (rl < rc) as libc::c_int as mp_limb_t;
        *rp.offset(i as isize) = rl;
        i += 1;
        if !(i < vn) {
            break;
        }
    }
    if vc == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"vc == 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3912 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"void mpz_ior(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_26539: {
        if vc == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"vc == 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3912 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"void mpz_ior(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while i < rn {
        ul = (*up.offset(i as isize) ^ ux).wrapping_add(uc);
        uc = (ul < uc) as libc::c_int as mp_limb_t;
        rl = ((ul | vx) ^ rx).wrapping_add(rc);
        rc = (rl < rc) as libc::c_int as mp_limb_t;
        *rp.offset(i as isize) = rl;
        i += 1;
        i;
    }
    if rc != 0 {
        let fresh29 = rn;
        rn = rn + 1;
        *rp.offset(fresh29 as isize) = rc;
    } else {
        rn = mpn_normalized_size(rp as mp_srcptr, rn);
    }
    (*r)._mp_size = (if rx != 0 { -rn } else { rn }) as libc::c_int;
}
pub unsafe extern "C" fn mpz_xor(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) {
    let mut un: mp_size_t = 0;
    let mut vn: mp_size_t = 0;
    let mut i: mp_size_t = 0;
    let mut up: mp_ptr = 0 as *mut mp_limb_t;
    let mut vp: mp_ptr = 0 as *mut mp_limb_t;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    let mut ux: mp_limb_t = 0;
    let mut vx: mp_limb_t = 0;
    let mut rx: mp_limb_t = 0;
    let mut uc: mp_limb_t = 0;
    let mut vc: mp_limb_t = 0;
    let mut rc: mp_limb_t = 0;
    let mut ul: mp_limb_t = 0;
    let mut vl: mp_limb_t = 0;
    let mut rl: mp_limb_t = 0;
    un = (if (*u)._mp_size >= 0 as libc::c_int { (*u)._mp_size } else { -(*u)._mp_size })
        as mp_size_t;
    vn = (if (*v)._mp_size >= 0 as libc::c_int { (*v)._mp_size } else { -(*v)._mp_size })
        as mp_size_t;
    if un < vn {
        let mut __mpz_srcptr_swap__tmp: mpz_srcptr = u;
        u = v;
        v = __mpz_srcptr_swap__tmp;
        let mut __mp_size_t_swap__tmp: mp_size_t = un;
        un = vn;
        vn = __mp_size_t_swap__tmp;
    }
    if vn == 0 as libc::c_int as libc::c_long {
        mpz_set(r, u);
        return;
    }
    uc = ((*u)._mp_size < 0 as libc::c_int) as libc::c_int as mp_limb_t;
    vc = ((*v)._mp_size < 0 as libc::c_int) as libc::c_int as mp_limb_t;
    rc = uc ^ vc;
    ux = uc.wrapping_neg();
    vx = vc.wrapping_neg();
    rx = rc.wrapping_neg();
    rp = if un + rc as mp_size_t > (*r)._mp_alloc as libc::c_long {
        mpz_realloc(r, un + rc as mp_size_t)
    } else {
        (*r)._mp_d
    };
    up = (*u)._mp_d;
    vp = (*v)._mp_d;
    i = 0 as libc::c_int as mp_size_t;
    loop {
        ul = (*up.offset(i as isize) ^ ux).wrapping_add(uc);
        uc = (ul < uc) as libc::c_int as mp_limb_t;
        vl = (*vp.offset(i as isize) ^ vx).wrapping_add(vc);
        vc = (vl < vc) as libc::c_int as mp_limb_t;
        rl = (ul ^ vl ^ rx).wrapping_add(rc);
        rc = (rl < rc) as libc::c_int as mp_limb_t;
        *rp.offset(i as isize) = rl;
        i += 1;
        if !(i < vn) {
            break;
        }
    }
    if vc == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"vc == 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            3981 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"void mpz_xor(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_27017: {
        if vc == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"vc == 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                3981 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"void mpz_xor(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while i < un {
        ul = (*up.offset(i as isize) ^ ux).wrapping_add(uc);
        uc = (ul < uc) as libc::c_int as mp_limb_t;
        rl = (ul ^ ux).wrapping_add(rc);
        rc = (rl < rc) as libc::c_int as mp_limb_t;
        *rp.offset(i as isize) = rl;
        i += 1;
        i;
    }
    if rc != 0 {
        let fresh30 = un;
        un = un + 1;
        *rp.offset(fresh30 as isize) = rc;
    } else {
        un = mpn_normalized_size(rp as mp_srcptr, un);
    }
    (*r)._mp_size = (if rx != 0 { -un } else { un }) as libc::c_int;
}
pub unsafe extern "C" fn mpz_popcount(mut u: *const __mpz_struct) -> mp_bitcnt_t {
    let mut un: mp_size_t = 0;
    un = (*u)._mp_size as mp_size_t;
    if un < 0 as libc::c_int as libc::c_long {
        return !(0 as libc::c_int as mp_bitcnt_t);
    }
    return mpn_popcount((*u)._mp_d as mp_srcptr, un);
}
pub unsafe extern "C" fn mpz_hamdist(
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) -> mp_bitcnt_t {
    let mut un: mp_size_t = 0;
    let mut vn: mp_size_t = 0;
    let mut i: mp_size_t = 0;
    let mut uc: mp_limb_t = 0;
    let mut vc: mp_limb_t = 0;
    let mut ul: mp_limb_t = 0;
    let mut vl: mp_limb_t = 0;
    let mut comp: mp_limb_t = 0;
    let mut up: mp_srcptr = 0 as *const mp_limb_t;
    let mut vp: mp_srcptr = 0 as *const mp_limb_t;
    let mut c: mp_bitcnt_t = 0;
    un = (*u)._mp_size as mp_size_t;
    vn = (*v)._mp_size as mp_size_t;
    if un ^ vn < 0 as libc::c_int as libc::c_long {
        return !(0 as libc::c_int as mp_bitcnt_t);
    }
    vc = (un < 0 as libc::c_int as libc::c_long) as libc::c_int as mp_limb_t;
    uc = vc;
    comp = uc.wrapping_neg();
    if uc != 0 {
        if vn < 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"vn < 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                4064 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"mp_bitcnt_t mpz_hamdist(const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_27661: {
            if vn < 0 as libc::c_int as libc::c_long {} else {
                __assert_fail(
                    b"vn < 0\0" as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    4064 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 68],
                        &[libc::c_char; 68],
                    >(
                        b"mp_bitcnt_t mpz_hamdist(const __mpz_struct *, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        un = -un;
        vn = -vn;
    }
    up = (*u)._mp_d as mp_srcptr;
    vp = (*v)._mp_d as mp_srcptr;
    if un < vn {
        let mut __mp_srcptr_swap__tmp: mp_srcptr = up;
        up = vp;
        vp = __mp_srcptr_swap__tmp;
        let mut __mp_size_t_swap__tmp: mp_size_t = un;
        un = vn;
        vn = __mp_size_t_swap__tmp;
    }
    i = 0 as libc::c_int as mp_size_t;
    c = 0 as libc::c_int as mp_bitcnt_t;
    while i < vn {
        ul = (*up.offset(i as isize) ^ comp).wrapping_add(uc);
        uc = (ul < uc) as libc::c_int as mp_limb_t;
        vl = (*vp.offset(i as isize) ^ comp).wrapping_add(vc);
        vc = (vl < vc) as libc::c_int as mp_limb_t;
        c = (c as libc::c_ulong)
            .wrapping_add(gmp_popcount_limb(ul ^ vl) as libc::c_ulong) as mp_bitcnt_t
            as mp_bitcnt_t;
        i += 1;
        i;
    }
    if vc == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"vc == 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            4085 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"mp_bitcnt_t mpz_hamdist(const __mpz_struct *, const __mpz_struct *)\0"))
                .as_ptr(),
        );
    }
    'c_27459: {
        if vc == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"vc == 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                4085 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"mp_bitcnt_t mpz_hamdist(const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while i < un {
        ul = (*up.offset(i as isize) ^ comp).wrapping_add(uc);
        uc = (ul < uc) as libc::c_int as mp_limb_t;
        c = (c as libc::c_ulong)
            .wrapping_add(gmp_popcount_limb(ul ^ comp) as libc::c_ulong) as mp_bitcnt_t
            as mp_bitcnt_t;
        i += 1;
        i;
    }
    return c;
}
pub unsafe extern "C" fn mpz_get_str(
    mut sp: *mut libc::c_char,
    mut base: libc::c_int,
    mut u: *const __mpz_struct,
) -> *mut libc::c_char {
    let mut bits: libc::c_uint = 0;
    let mut digits: *const libc::c_char = 0 as *const libc::c_char;
    let mut un: mp_size_t = 0;
    let mut i: size_t = 0;
    let mut sn: size_t = 0;
    digits = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz\0"
        as *const u8 as *const libc::c_char;
    if base > 1 as libc::c_int {
        if base <= 36 as libc::c_int {
            digits = b"0123456789abcdefghijklmnopqrstuvwxyz\0" as *const u8
                as *const libc::c_char;
        } else if base > 62 as libc::c_int {
            return 0 as *mut libc::c_char
        }
    } else if base >= -(1 as libc::c_int) {
        base = 10 as libc::c_int;
    } else {
        base = -base;
        if base > 36 as libc::c_int {
            return 0 as *mut libc::c_char;
        }
    }
    sn = (1 as libc::c_int as libc::c_ulong).wrapping_add(mpz_sizeinbase(u, base));
    if sp.is_null() {
        sp = (Some(gmp_allocate_func.unwrap()))
            .unwrap()((1 as libc::c_int as libc::c_ulong).wrapping_add(sn))
            as *mut libc::c_char;
    }
    un = (if (*u)._mp_size >= 0 as libc::c_int { (*u)._mp_size } else { -(*u)._mp_size })
        as mp_size_t;
    if un == 0 as libc::c_int as libc::c_long {
        *sp.offset(0 as libc::c_int as isize) = '0' as i32 as libc::c_char;
        *sp.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        return sp;
    }
    i = 0 as libc::c_int as size_t;
    if (*u)._mp_size < 0 as libc::c_int {
        let fresh31 = i;
        i = i.wrapping_add(1);
        *sp.offset(fresh31 as isize) = '-' as i32 as libc::c_char;
    }
    bits = mpn_base_power_of_two_p(base as libc::c_uint);
    if bits != 0 {
        sn = i
            .wrapping_add(
                mpn_get_str_bits(
                    (sp as *mut libc::c_uchar).offset(i as isize),
                    bits,
                    (*u)._mp_d as mp_srcptr,
                    un,
                ),
            );
    } else {
        let mut info: mpn_base_info = mpn_base_info { exp: 0, bb: 0 };
        let mut tp: mp_ptr = 0 as *mut mp_limb_t;
        mpn_get_base_info(&mut info, base as mp_limb_t);
        tp = gmp_xalloc_limbs(un);
        mpn_copyi(tp, (*u)._mp_d as mp_srcptr, un);
        sn = i
            .wrapping_add(
                mpn_get_str_other(
                    (sp as *mut libc::c_uchar).offset(i as isize),
                    base,
                    &mut info,
                    tp,
                    un,
                ),
            );
        (Some(gmp_free_func.unwrap()))
            .unwrap()(tp as *mut libc::c_void, 0 as libc::c_int as size_t);
    }
    while i < sn {
        *sp
            .offset(
                i as isize,
            ) = *digits.offset(*sp.offset(i as isize) as libc::c_uchar as isize);
        i = i.wrapping_add(1);
        i;
    }
    *sp.offset(sn as isize) = '\0' as i32 as libc::c_char;
    return sp;
}
pub unsafe extern "C" fn mpz_set_str(
    mut r: *mut __mpz_struct,
    mut sp: *const libc::c_char,
    mut base: libc::c_int,
) -> libc::c_int {
    let mut bits: libc::c_uint = 0;
    let mut value_of_a: libc::c_uint = 0;
    let mut rn: mp_size_t = 0;
    let mut alloc: mp_size_t = 0;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    let mut dn: size_t = 0;
    let mut sign: libc::c_int = 0;
    let mut dp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if base == 0 as libc::c_int || base >= 2 as libc::c_int && base <= 62 as libc::c_int
    {} else {
        __assert_fail(
            b"base == 0 || (base >= 2 && base <= 62)\0" as *const u8
                as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            4297 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"int mpz_set_str(__mpz_struct *, const char *, int)\0"))
                .as_ptr(),
        );
    }
    'c_29861: {
        if base == 0 as libc::c_int
            || base >= 2 as libc::c_int && base <= 62 as libc::c_int
        {} else {
            __assert_fail(
                b"base == 0 || (base >= 2 && base <= 62)\0" as *const u8
                    as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                4297 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"int mpz_set_str(__mpz_struct *, const char *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    while *(*__ctype_b_loc()).offset(*sp as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        sp = sp.offset(1);
        sp;
    }
    sign = (*sp as libc::c_int == '-' as i32) as libc::c_int;
    sp = sp.offset(sign as isize);
    if base == 0 as libc::c_int {
        if *sp.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32 {
            if *sp.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
                || *sp.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32
            {
                base = 16 as libc::c_int;
                sp = sp.offset(2 as libc::c_int as isize);
            } else if *sp.offset(1 as libc::c_int as isize) as libc::c_int == 'b' as i32
                || *sp.offset(1 as libc::c_int as isize) as libc::c_int == 'B' as i32
            {
                base = 2 as libc::c_int;
                sp = sp.offset(2 as libc::c_int as isize);
            } else {
                base = 8 as libc::c_int;
            }
        } else {
            base = 10 as libc::c_int;
        }
    }
    if *sp == 0 {
        (*r)._mp_size = 0 as libc::c_int;
        return -(1 as libc::c_int);
    }
    dp = (Some(gmp_allocate_func.unwrap())).unwrap()(strlen(sp)) as *mut libc::c_uchar;
    value_of_a = (if base > 36 as libc::c_int {
        36 as libc::c_int
    } else {
        10 as libc::c_int
    }) as libc::c_uint;
    dn = 0 as libc::c_int as size_t;
    while *sp != 0 {
        let mut digit: libc::c_uint = 0;
        if !(*(*__ctype_b_loc()).offset(*sp as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0)
        {
            if *sp as libc::c_int >= '0' as i32 && *sp as libc::c_int <= '9' as i32 {
                digit = (*sp as libc::c_int - '0' as i32) as libc::c_uint;
            } else if *sp as libc::c_int >= 'a' as i32
                && *sp as libc::c_int <= 'z' as i32
            {
                digit = ((*sp as libc::c_int - 'a' as i32) as libc::c_uint)
                    .wrapping_add(value_of_a);
            } else if *sp as libc::c_int >= 'A' as i32
                && *sp as libc::c_int <= 'Z' as i32
            {
                digit = (*sp as libc::c_int - 'A' as i32 + 10 as libc::c_int)
                    as libc::c_uint;
            } else {
                digit = base as libc::c_uint;
            }
            if digit >= base as libc::c_uint {
                (Some(gmp_free_func.unwrap()))
                    .unwrap()(dp as *mut libc::c_void, 0 as libc::c_int as size_t);
                (*r)._mp_size = 0 as libc::c_int;
                return -(1 as libc::c_int);
            }
            let fresh32 = dn;
            dn = dn.wrapping_add(1);
            *dp.offset(fresh32 as isize) = digit as libc::c_uchar;
        }
        sp = sp.offset(1);
        sp;
    }
    if dn == 0 {
        (Some(gmp_free_func.unwrap()))
            .unwrap()(dp as *mut libc::c_void, 0 as libc::c_int as size_t);
        (*r)._mp_size = 0 as libc::c_int;
        return -(1 as libc::c_int);
    }
    bits = mpn_base_power_of_two_p(base as libc::c_uint);
    if bits > 0 as libc::c_int as libc::c_uint {
        alloc = dn
            .wrapping_mul(bits as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as mp_size_t;
        rp = if alloc > (*r)._mp_alloc as libc::c_long {
            mpz_realloc(r, alloc)
        } else {
            (*r)._mp_d
        };
        rn = mpn_set_str_bits(rp, dp, dn, bits);
    } else {
        let mut info: mpn_base_info = mpn_base_info { exp: 0, bb: 0 };
        mpn_get_base_info(&mut info, base as mp_limb_t);
        alloc = dn
            .wrapping_add(info.exp as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(info.exp as libc::c_ulong) as mp_size_t;
        rp = if alloc > (*r)._mp_alloc as libc::c_long {
            mpz_realloc(r, alloc)
        } else {
            (*r)._mp_d
        };
        rn = mpn_set_str_other(rp, dp, dn, base as mp_limb_t, &mut info);
        if rn > 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"rn > 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                4381 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"int mpz_set_str(__mpz_struct *, const char *, int)\0"))
                    .as_ptr(),
            );
        }
        'c_29308: {
            if rn > 0 as libc::c_int as libc::c_long {} else {
                __assert_fail(
                    b"rn > 0\0" as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    4381 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 51],
                        &[libc::c_char; 51],
                    >(b"int mpz_set_str(__mpz_struct *, const char *, int)\0"))
                        .as_ptr(),
                );
            }
        };
        rn
            -= (*rp.offset((rn - 1 as libc::c_int as libc::c_long) as isize)
                == 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long;
    }
    if rn <= alloc {} else {
        __assert_fail(
            b"rn <= alloc\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            4384 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"int mpz_set_str(__mpz_struct *, const char *, int)\0"))
                .as_ptr(),
        );
    }
    'c_29243: {
        if rn <= alloc {} else {
            __assert_fail(
                b"rn <= alloc\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                4384 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"int mpz_set_str(__mpz_struct *, const char *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    (Some(gmp_free_func.unwrap()))
        .unwrap()(dp as *mut libc::c_void, 0 as libc::c_int as size_t);
    (*r)._mp_size = (if sign != 0 { -rn } else { rn }) as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mpz_init_set_str(
    mut r: *mut __mpz_struct,
    mut sp: *const libc::c_char,
    mut base: libc::c_int,
) -> libc::c_int {
    mpz_init(r);
    return mpz_set_str(r, sp, base);
}
pub unsafe extern "C" fn mpz_out_str(
    mut stream: *mut FILE,
    mut base: libc::c_int,
    mut x: *const __mpz_struct,
) -> size_t {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    str = mpz_get_str(0 as *mut libc::c_char, base, x);
    len = strlen(str);
    len = fwrite(
        str as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        len,
        stream,
    );
    (Some(gmp_free_func.unwrap()))
        .unwrap()(str as *mut libc::c_void, 0 as libc::c_int as size_t);
    return len;
}
pub unsafe extern "C" fn mpz_import(
    mut r: *mut __mpz_struct,
    mut count: size_t,
    mut order: libc::c_int,
    mut size: size_t,
    mut endian: libc::c_int,
    mut nails: size_t,
    mut src: *const libc::c_void,
) {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut word_step: ptrdiff_t = 0;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    let mut rn: mp_size_t = 0;
    let mut limb: mp_limb_t = 0;
    let mut bytes: size_t = 0;
    let mut i: mp_size_t = 0;
    if nails != 0 as libc::c_int as libc::c_ulong {
        gmp_die(
            b"mpz_import: Nails not supported.\0" as *const u8 as *const libc::c_char,
        );
    }
    if order == 1 as libc::c_int || order == -(1 as libc::c_int) {} else {
        __assert_fail(
            b"order == 1 || order == -1\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            4442 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"void mpz_import(__mpz_struct *, size_t, int, size_t, int, size_t, const void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_30402: {
        if order == 1 as libc::c_int || order == -(1 as libc::c_int) {} else {
            __assert_fail(
                b"order == 1 || order == -1\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                4442 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"void mpz_import(__mpz_struct *, size_t, int, size_t, int, size_t, const void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if endian >= -(1 as libc::c_int) && endian <= 1 as libc::c_int {} else {
        __assert_fail(
            b"endian >= -1 && endian <= 1\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            4443 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"void mpz_import(__mpz_struct *, size_t, int, size_t, int, size_t, const void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_30354: {
        if endian >= -(1 as libc::c_int) && endian <= 1 as libc::c_int {} else {
            __assert_fail(
                b"endian >= -1 && endian <= 1\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                4443 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"void mpz_import(__mpz_struct *, size_t, int, size_t, int, size_t, const void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if endian == 0 as libc::c_int {
        endian = gmp_detect_endian();
    }
    p = src as *mut libc::c_uchar;
    word_step = (if order != endian {
        (2 as libc::c_int as libc::c_ulong).wrapping_mul(size)
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as ptrdiff_t;
    if order == 1 as libc::c_int {
        p = p
            .offset(
                size.wrapping_mul(count.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                    as isize,
            );
        word_step = -word_step;
    }
    if endian == 1 as libc::c_int {
        p = p.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    }
    rn = size
        .wrapping_mul(count)
        .wrapping_add(::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<mp_limb_t>() as libc::c_ulong) as mp_size_t;
    rp = if rn > (*r)._mp_alloc as libc::c_long {
        mpz_realloc(r, rn)
    } else {
        (*r)._mp_d
    };
    limb = 0 as libc::c_int as mp_limb_t;
    bytes = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as mp_size_t;
    while count > 0 as libc::c_int as libc::c_ulong {
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < size {
            let fresh33 = bytes;
            bytes = bytes.wrapping_add(1);
            limb
                |= (*p as mp_limb_t)
                    << fresh33.wrapping_mul(8 as libc::c_int as libc::c_ulong);
            if bytes == ::std::mem::size_of::<mp_limb_t>() as libc::c_ulong {
                let fresh34 = i;
                i = i + 1;
                *rp.offset(fresh34 as isize) = limb;
                bytes = 0 as libc::c_int as size_t;
                limb = 0 as libc::c_int as mp_limb_t;
            }
            j = j.wrapping_add(1);
            j;
            p = p.offset(-(endian as ptrdiff_t as isize));
        }
        count = count.wrapping_sub(1);
        count;
        p = p.offset(word_step as isize);
    }
    if i + (bytes > 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long
        == rn
    {} else {
        __assert_fail(
            b"i + (bytes > 0) == rn\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            4481 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"void mpz_import(__mpz_struct *, size_t, int, size_t, int, size_t, const void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_30074: {
        if i + (bytes > 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long
            == rn
        {} else {
            __assert_fail(
                b"i + (bytes > 0) == rn\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                4481 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"void mpz_import(__mpz_struct *, size_t, int, size_t, int, size_t, const void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if limb != 0 as libc::c_int as libc::c_ulong {
        let fresh35 = i;
        i = i + 1;
        *rp.offset(fresh35 as isize) = limb;
    } else {
        i = mpn_normalized_size(rp as mp_srcptr, i);
    }
    (*r)._mp_size = i as libc::c_int;
}
unsafe extern "C" fn gmp_detect_endian() -> libc::c_int {
    static mut i: libc::c_int = 2 as libc::c_int;
    let mut p: *const libc::c_uchar = &i as *const libc::c_int as *const libc::c_uchar;
    return 1 as libc::c_int - *p as libc::c_int;
}
pub unsafe extern "C" fn mpz_export(
    mut r: *mut libc::c_void,
    mut countp: *mut size_t,
    mut order: libc::c_int,
    mut size: size_t,
    mut endian: libc::c_int,
    mut nails: size_t,
    mut u: *const __mpz_struct,
) -> *mut libc::c_void {
    let mut count: size_t = 0;
    let mut un: mp_size_t = 0;
    if nails != 0 as libc::c_int as libc::c_ulong {
        gmp_die(
            b"mpz_import: Nails not supported.\0" as *const u8 as *const libc::c_char,
        );
    }
    if order == 1 as libc::c_int || order == -(1 as libc::c_int) {} else {
        __assert_fail(
            b"order == 1 || order == -1\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            4500 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 83],
                &[libc::c_char; 83],
            >(
                b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_31060: {
        if order == 1 as libc::c_int || order == -(1 as libc::c_int) {} else {
            __assert_fail(
                b"order == 1 || order == -1\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                4500 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if endian >= -(1 as libc::c_int) && endian <= 1 as libc::c_int {} else {
        __assert_fail(
            b"endian >= -1 && endian <= 1\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            4501 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 83],
                &[libc::c_char; 83],
            >(
                b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_31012: {
        if endian >= -(1 as libc::c_int) && endian <= 1 as libc::c_int {} else {
            __assert_fail(
                b"endian >= -1 && endian <= 1\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                4501 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if size > 0 as libc::c_int as libc::c_ulong || (*u)._mp_size == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"size > 0 || u->_mp_size == 0\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            4502 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 83],
                &[libc::c_char; 83],
            >(
                b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_30960: {
        if size > 0 as libc::c_int as libc::c_ulong || (*u)._mp_size == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"size > 0 || u->_mp_size == 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                4502 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    un = (*u)._mp_size as mp_size_t;
    count = 0 as libc::c_int as size_t;
    if un != 0 as libc::c_int as libc::c_long {
        let mut k: size_t = 0;
        let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut word_step: ptrdiff_t = 0;
        let mut limb: mp_limb_t = 0;
        let mut bytes: size_t = 0;
        let mut i: mp_size_t = 0;
        un = if un >= 0 as libc::c_int as libc::c_long { un } else { -un };
        limb = *((*u)._mp_d).offset((un - 1 as libc::c_int as libc::c_long) as isize);
        if limb != 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"limb != 0\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                4522 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_30877: {
            if limb != 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"limb != 0\0" as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    4522 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 83],
                        &[libc::c_char; 83],
                    >(
                        b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        k = ((::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            <= 8 as libc::c_int as libc::c_ulong) as libc::c_int as size_t;
        if k == 0 {
            loop {
                let mut LOCAL_CHAR_BIT: libc::c_int = 8 as libc::c_int;
                k = k.wrapping_add(1);
                k;
                limb >>= LOCAL_CHAR_BIT;
                if !(limb != 0 as libc::c_int as libc::c_ulong) {
                    break;
                }
            }
        }
        count = k
            .wrapping_add(
                ((un - 1 as libc::c_int as libc::c_long) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<mp_limb_t>() as libc::c_ulong),
            )
            .wrapping_add(size)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(size);
        if r.is_null() {
            r = (Some(gmp_allocate_func.unwrap())).unwrap()(count.wrapping_mul(size));
        }
        if endian == 0 as libc::c_int {
            endian = gmp_detect_endian();
        }
        p = r as *mut libc::c_uchar;
        word_step = (if order != endian {
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(size)
        } else {
            0 as libc::c_int as libc::c_ulong
        }) as ptrdiff_t;
        if order == 1 as libc::c_int {
            p = p
                .offset(
                    size
                        .wrapping_mul(
                            count.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ) as isize,
                );
            word_step = -word_step;
        }
        if endian == 1 as libc::c_int {
            p = p.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        }
        bytes = 0 as libc::c_int as size_t;
        i = 0 as libc::c_int as mp_size_t;
        k = 0 as libc::c_int as size_t;
        while k < count {
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < size {
                if ::std::mem::size_of::<mp_limb_t>() as libc::c_ulong
                    == 1 as libc::c_int as libc::c_ulong
                {
                    if i < un {
                        let fresh36 = i;
                        i = i + 1;
                        *p = *((*u)._mp_d).offset(fresh36 as isize) as libc::c_uchar;
                    } else {
                        *p = 0 as libc::c_int as libc::c_uchar;
                    }
                } else {
                    let mut LOCAL_CHAR_BIT_0: libc::c_int = 8 as libc::c_int;
                    if bytes == 0 as libc::c_int as libc::c_ulong {
                        if i < un {
                            let fresh37 = i;
                            i = i + 1;
                            limb = *((*u)._mp_d).offset(fresh37 as isize);
                        }
                        bytes = ::std::mem::size_of::<mp_limb_t>() as libc::c_ulong;
                    }
                    *p = limb as libc::c_uchar;
                    limb >>= LOCAL_CHAR_BIT_0;
                    bytes = bytes.wrapping_sub(1);
                    bytes;
                }
                j = j.wrapping_add(1);
                j;
                p = p.offset(-(endian as ptrdiff_t as isize));
            }
            k = k.wrapping_add(1);
            k;
            p = p.offset(word_step as isize);
        }
        if i == un {} else {
            __assert_fail(
                b"i == un\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                4585 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_30557: {
            if i == un {} else {
                __assert_fail(
                    b"i == un\0" as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    4585 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 83],
                        &[libc::c_char; 83],
                    >(
                        b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if k == count {} else {
            __assert_fail(
                b"k == count\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                4586 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_30518: {
            if k == count {} else {
                __assert_fail(
                    b"k == count\0" as *const u8 as *const libc::c_char,
                    b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                    4586 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 83],
                        &[libc::c_char; 83],
                    >(
                        b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
    }
    if !countp.is_null() {
        *countp = count;
    }
    return r;
}
pub unsafe extern "C" fn mpz_sizeinbase(
    mut u: *const __mpz_struct,
    mut base: libc::c_int,
) -> size_t {
    let mut un: mp_size_t = 0;
    let mut up: mp_srcptr = 0 as *const mp_limb_t;
    let mut tp: mp_ptr = 0 as *mut mp_limb_t;
    let mut bits: mp_bitcnt_t = 0;
    let mut bi: gmp_div_inverse = gmp_div_inverse {
        shift: 0,
        d1: 0,
        d0: 0,
        di: 0,
    };
    let mut ndigits: size_t = 0;
    if base >= 2 as libc::c_int {} else {
        __assert_fail(
            b"base >= 2\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            4176 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 49],
                &[libc::c_char; 49],
            >(b"size_t mpz_sizeinbase(const __mpz_struct *, int)\0"))
                .as_ptr(),
        );
    }
    'c_14863: {
        if base >= 2 as libc::c_int {} else {
            __assert_fail(
                b"base >= 2\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                4176 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 49],
                    &[libc::c_char; 49],
                >(b"size_t mpz_sizeinbase(const __mpz_struct *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if base <= 62 as libc::c_int {} else {
        __assert_fail(
            b"base <= 62\0" as *const u8 as *const libc::c_char,
            b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
            4177 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 49],
                &[libc::c_char; 49],
            >(b"size_t mpz_sizeinbase(const __mpz_struct *, int)\0"))
                .as_ptr(),
        );
    }
    'c_14825: {
        if base <= 62 as libc::c_int {} else {
            __assert_fail(
                b"base <= 62\0" as *const u8 as *const libc::c_char,
                b"./mini-gmp.c\0" as *const u8 as *const libc::c_char,
                4177 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 49],
                    &[libc::c_char; 49],
                >(b"size_t mpz_sizeinbase(const __mpz_struct *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    un = (if (*u)._mp_size >= 0 as libc::c_int { (*u)._mp_size } else { -(*u)._mp_size })
        as mp_size_t;
    if un == 0 as libc::c_int as libc::c_long {
        return 1 as libc::c_int as size_t;
    }
    up = (*u)._mp_d as mp_srcptr;
    bits = ((un - 1 as libc::c_int as libc::c_long) as libc::c_ulong)
        .wrapping_mul(
            (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(
            mpn_limb_size_in_base_2(
                *up.offset((un - 1 as libc::c_int as libc::c_long) as isize),
            ),
        );
    match base {
        2 => return bits,
        4 => {
            return bits
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong);
        }
        8 => {
            return bits
                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                .wrapping_div(3 as libc::c_int as libc::c_ulong);
        }
        16 => {
            return bits
                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                .wrapping_div(4 as libc::c_int as libc::c_ulong);
        }
        32 => {
            return bits
                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                .wrapping_div(5 as libc::c_int as libc::c_ulong);
        }
        _ => {}
    }
    tp = gmp_xalloc_limbs(un);
    mpn_copyi(tp, up, un);
    mpn_div_qr_1_invert(&mut bi, base as mp_limb_t);
    ndigits = 0 as libc::c_int as size_t;
    loop {
        ndigits = ndigits.wrapping_add(1);
        ndigits;
        mpn_div_qr_1_preinv(tp, tp as mp_srcptr, un, &mut bi);
        un
            -= (*tp.offset((un - 1 as libc::c_int as libc::c_long) as isize)
                == 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long;
        if !(un > 0 as libc::c_int as libc::c_long) {
            break;
        }
    }
    (Some(gmp_free_func.unwrap()))
        .unwrap()(tp as *mut libc::c_void, 0 as libc::c_int as size_t);
    return ndigits;
}
unsafe extern "C" fn ecc_init(mut p: *mut ecc_point) {
    mpz_init(((*p).x).as_mut_ptr());
    mpz_init(((*p).y).as_mut_ptr());
}
unsafe extern "C" fn ecc_clear(mut p: *mut ecc_point) {
    mpz_clear(((*p).x).as_mut_ptr());
    mpz_clear(((*p).y).as_mut_ptr());
}
unsafe extern "C" fn ecc_zero_p(mut p: *const ecc_point) -> libc::c_int {
    return (*p).is_zero;
}
unsafe extern "C" fn ecc_equal_p(
    mut p: *const ecc_point,
    mut q: *const ecc_point,
) -> libc::c_int {
    return if (*p).is_zero != 0 {
        (*q).is_zero
    } else {
        ((*q).is_zero == 0
            && mpz_cmp(((*p).x).as_ptr(), ((*q).x).as_ptr()) == 0 as libc::c_int
            && mpz_cmp(((*p).y).as_ptr(), ((*q).y).as_ptr()) == 0 as libc::c_int)
            as libc::c_int
    };
}
unsafe extern "C" fn ecc_set_zero(mut ecc: *const ecc_curve, mut r: *mut ecc_point) {
    (*r).is_zero = 1 as libc::c_int;
    mpz_set_ui(((*r).x).as_mut_ptr(), 0 as libc::c_int as libc::c_ulong);
    mpz_set_ui(
        ((*r).y).as_mut_ptr(),
        ((*ecc).type_0 as libc::c_uint
            != ECC_TYPE_WEIERSTRASS as libc::c_int as libc::c_uint) as libc::c_int
            as libc::c_ulong,
    );
}
unsafe extern "C" fn ecc_set(mut r: *mut ecc_point, mut p: *const ecc_point) {
    (*r).is_zero = (*p).is_zero;
    mpz_set(((*r).x).as_mut_ptr(), ((*p).x).as_ptr());
    mpz_set(((*r).y).as_mut_ptr(), ((*p).y).as_ptr());
}
unsafe extern "C" fn ecc_dup(
    mut ecc: *const ecc_curve,
    mut r: *mut ecc_point,
    mut p: *const ecc_point,
) {
    if (*ecc).type_0 as libc::c_uint
        != ECC_TYPE_WEIERSTRASS as libc::c_int as libc::c_uint
    {
        ecc_add(ecc, r, p, p);
        return;
    }
    if ecc_zero_p(p) != 0 {
        ecc_set_zero(ecc, r);
    } else {
        let mut m: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        let mut t: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        let mut x: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        let mut y: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        mpz_init(m.as_mut_ptr());
        mpz_init(t.as_mut_ptr());
        mpz_init(x.as_mut_ptr());
        mpz_init(y.as_mut_ptr());
        mpz_mul_ui(m.as_mut_ptr(), ((*p).y).as_ptr(), 2 as libc::c_int as libc::c_ulong);
        mpz_invert(
            m.as_mut_ptr(),
            m.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_mul(t.as_mut_ptr(), ((*p).x).as_ptr(), ((*p).x).as_ptr());
        mpz_mod(
            t.as_mut_ptr(),
            t.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_sub_ui(
            t.as_mut_ptr(),
            t.as_mut_ptr() as *const __mpz_struct,
            1 as libc::c_int as libc::c_ulong,
        );
        mpz_mul_ui(
            t.as_mut_ptr(),
            t.as_mut_ptr() as *const __mpz_struct,
            3 as libc::c_int as libc::c_ulong,
        );
        mpz_mul(
            t.as_mut_ptr(),
            t.as_mut_ptr() as *const __mpz_struct,
            m.as_mut_ptr() as *const __mpz_struct,
        );
        mpz_mod(
            t.as_mut_ptr(),
            t.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_mul(
            x.as_mut_ptr(),
            t.as_mut_ptr() as *const __mpz_struct,
            t.as_mut_ptr() as *const __mpz_struct,
        );
        mpz_submul_ui(
            x.as_mut_ptr(),
            ((*p).x).as_ptr(),
            2 as libc::c_int as libc::c_ulong,
        );
        mpz_mod(
            x.as_mut_ptr(),
            x.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_sub(
            y.as_mut_ptr(),
            ((*p).x).as_ptr(),
            x.as_mut_ptr() as *const __mpz_struct,
        );
        mpz_mul(
            y.as_mut_ptr(),
            y.as_mut_ptr() as *const __mpz_struct,
            t.as_mut_ptr() as *const __mpz_struct,
        );
        mpz_sub(
            y.as_mut_ptr(),
            y.as_mut_ptr() as *const __mpz_struct,
            ((*p).y).as_ptr(),
        );
        mpz_mod(
            y.as_mut_ptr(),
            y.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        (*r).is_zero = 0 as libc::c_int;
        mpz_swap(x.as_mut_ptr(), ((*r).x).as_mut_ptr());
        mpz_swap(y.as_mut_ptr(), ((*r).y).as_mut_ptr());
        mpz_clear(m.as_mut_ptr());
        mpz_clear(t.as_mut_ptr());
        mpz_clear(x.as_mut_ptr());
        mpz_clear(y.as_mut_ptr());
    };
}
unsafe extern "C" fn ecc_add(
    mut ecc: *const ecc_curve,
    mut r: *mut ecc_point,
    mut p: *const ecc_point,
    mut q: *const ecc_point,
) {
    if (*ecc).type_0 as libc::c_uint
        == ECC_TYPE_WEIERSTRASS as libc::c_int as libc::c_uint
    {
        if ecc_zero_p(p) != 0 {
            ecc_set(r, q);
        } else if ecc_zero_p(q) != 0 {
            ecc_set(r, p);
        } else if mpz_cmp(((*p).x).as_ptr(), ((*q).x).as_ptr()) == 0 as libc::c_int {
            if mpz_cmp(((*p).y).as_ptr(), ((*q).y).as_ptr()) == 0 as libc::c_int {
                ecc_dup(ecc, r, p);
            } else {
                ecc_set_zero(ecc, r);
            }
        } else {
            let mut s: mpz_t = [__mpz_struct {
                _mp_alloc: 0,
                _mp_size: 0,
                _mp_d: 0 as *mut mp_limb_t,
            }; 1];
            let mut t: mpz_t = [__mpz_struct {
                _mp_alloc: 0,
                _mp_size: 0,
                _mp_d: 0 as *mut mp_limb_t,
            }; 1];
            let mut x: mpz_t = [__mpz_struct {
                _mp_alloc: 0,
                _mp_size: 0,
                _mp_d: 0 as *mut mp_limb_t,
            }; 1];
            let mut y: mpz_t = [__mpz_struct {
                _mp_alloc: 0,
                _mp_size: 0,
                _mp_d: 0 as *mut mp_limb_t,
            }; 1];
            mpz_init(s.as_mut_ptr());
            mpz_init(t.as_mut_ptr());
            mpz_init(x.as_mut_ptr());
            mpz_init(y.as_mut_ptr());
            mpz_sub(t.as_mut_ptr(), ((*q).x).as_ptr(), ((*p).x).as_ptr());
            mpz_invert(
                t.as_mut_ptr(),
                t.as_mut_ptr() as *const __mpz_struct,
                ((*ecc).p).as_ptr(),
            );
            mpz_sub(s.as_mut_ptr(), ((*q).y).as_ptr(), ((*p).y).as_ptr());
            mpz_mul(
                t.as_mut_ptr(),
                t.as_mut_ptr() as *const __mpz_struct,
                s.as_mut_ptr() as *const __mpz_struct,
            );
            mpz_mod(
                t.as_mut_ptr(),
                t.as_mut_ptr() as *const __mpz_struct,
                ((*ecc).p).as_ptr(),
            );
            mpz_mul(
                x.as_mut_ptr(),
                t.as_mut_ptr() as *const __mpz_struct,
                t.as_mut_ptr() as *const __mpz_struct,
            );
            mpz_sub(
                x.as_mut_ptr(),
                x.as_mut_ptr() as *const __mpz_struct,
                ((*p).x).as_ptr(),
            );
            mpz_sub(
                x.as_mut_ptr(),
                x.as_mut_ptr() as *const __mpz_struct,
                ((*q).x).as_ptr(),
            );
            mpz_mod(
                x.as_mut_ptr(),
                x.as_mut_ptr() as *const __mpz_struct,
                ((*ecc).p).as_ptr(),
            );
            mpz_sub(
                y.as_mut_ptr(),
                ((*p).x).as_ptr(),
                x.as_mut_ptr() as *const __mpz_struct,
            );
            mpz_mul(
                y.as_mut_ptr(),
                y.as_mut_ptr() as *const __mpz_struct,
                t.as_mut_ptr() as *const __mpz_struct,
            );
            mpz_sub(
                y.as_mut_ptr(),
                y.as_mut_ptr() as *const __mpz_struct,
                ((*p).y).as_ptr(),
            );
            mpz_mod(
                y.as_mut_ptr(),
                y.as_mut_ptr() as *const __mpz_struct,
                ((*ecc).p).as_ptr(),
            );
            (*r).is_zero = 0 as libc::c_int;
            mpz_swap(x.as_mut_ptr(), ((*r).x).as_mut_ptr());
            mpz_swap(y.as_mut_ptr(), ((*r).y).as_mut_ptr());
            mpz_clear(s.as_mut_ptr());
            mpz_clear(t.as_mut_ptr());
            mpz_clear(x.as_mut_ptr());
            mpz_clear(y.as_mut_ptr());
        }
    } else if (*ecc).type_0 as libc::c_uint
        == ECC_TYPE_EDWARDS as libc::c_int as libc::c_uint
    {
        let mut s_0: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        let mut t_0: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        let mut x_0: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        let mut y_0: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        mpz_init(s_0.as_mut_ptr());
        mpz_init(t_0.as_mut_ptr());
        mpz_init(x_0.as_mut_ptr());
        mpz_init(y_0.as_mut_ptr());
        mpz_mul(t_0.as_mut_ptr(), ((*ecc).b).as_ptr(), ((*p).x).as_ptr());
        mpz_mod(
            t_0.as_mut_ptr(),
            t_0.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_mul(
            t_0.as_mut_ptr(),
            t_0.as_mut_ptr() as *const __mpz_struct,
            ((*p).y).as_ptr(),
        );
        mpz_mod(
            t_0.as_mut_ptr(),
            t_0.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_mul(
            t_0.as_mut_ptr(),
            t_0.as_mut_ptr() as *const __mpz_struct,
            ((*q).x).as_ptr(),
        );
        mpz_mod(
            t_0.as_mut_ptr(),
            t_0.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_mul(
            t_0.as_mut_ptr(),
            t_0.as_mut_ptr() as *const __mpz_struct,
            ((*q).y).as_ptr(),
        );
        mpz_mod(
            t_0.as_mut_ptr(),
            t_0.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_mul(x_0.as_mut_ptr(), ((*p).x).as_ptr(), ((*q).y).as_ptr());
        mpz_mod(
            x_0.as_mut_ptr(),
            x_0.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_addmul(x_0.as_mut_ptr(), ((*q).x).as_ptr(), ((*p).y).as_ptr());
        mpz_mod(
            x_0.as_mut_ptr(),
            x_0.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_add_ui(
            s_0.as_mut_ptr(),
            t_0.as_mut_ptr() as *const __mpz_struct,
            1 as libc::c_int as libc::c_ulong,
        );
        mpz_invert(
            s_0.as_mut_ptr(),
            s_0.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_mul(
            x_0.as_mut_ptr(),
            x_0.as_mut_ptr() as *const __mpz_struct,
            s_0.as_mut_ptr() as *const __mpz_struct,
        );
        mpz_mod(
            x_0.as_mut_ptr(),
            x_0.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_mul(y_0.as_mut_ptr(), ((*p).y).as_ptr(), ((*q).y).as_ptr());
        mpz_mod(
            y_0.as_mut_ptr(),
            y_0.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_submul(y_0.as_mut_ptr(), ((*p).x).as_ptr(), ((*q).x).as_ptr());
        mpz_mod(
            y_0.as_mut_ptr(),
            y_0.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_set_ui(s_0.as_mut_ptr(), 1 as libc::c_int as libc::c_ulong);
        mpz_sub(
            s_0.as_mut_ptr(),
            s_0.as_mut_ptr() as *const __mpz_struct,
            t_0.as_mut_ptr() as *const __mpz_struct,
        );
        mpz_invert(
            s_0.as_mut_ptr(),
            s_0.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_mul(
            y_0.as_mut_ptr(),
            y_0.as_mut_ptr() as *const __mpz_struct,
            s_0.as_mut_ptr() as *const __mpz_struct,
        );
        mpz_mod(
            y_0.as_mut_ptr(),
            y_0.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_swap(x_0.as_mut_ptr(), ((*r).x).as_mut_ptr());
        mpz_swap(y_0.as_mut_ptr(), ((*r).y).as_mut_ptr());
        (*r)
            .is_zero = (mpz_cmp_ui(
            ((*r).x).as_mut_ptr() as *const __mpz_struct,
            0 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            && mpz_cmp_ui(
                ((*r).y).as_mut_ptr() as *const __mpz_struct,
                1 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int) as libc::c_int;
        mpz_clear(s_0.as_mut_ptr());
        mpz_clear(t_0.as_mut_ptr());
        mpz_clear(x_0.as_mut_ptr());
        mpz_clear(y_0.as_mut_ptr());
    } else {
        let mut s_1: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        let mut t_1: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        let mut x_1: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        let mut y_1: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        mpz_init(s_1.as_mut_ptr());
        mpz_init(t_1.as_mut_ptr());
        mpz_init(x_1.as_mut_ptr());
        mpz_init(y_1.as_mut_ptr());
        mpz_mul(t_1.as_mut_ptr(), ((*ecc).b).as_ptr(), ((*p).x).as_ptr());
        mpz_mod(
            t_1.as_mut_ptr(),
            t_1.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_mul(
            t_1.as_mut_ptr(),
            t_1.as_mut_ptr() as *const __mpz_struct,
            ((*p).y).as_ptr(),
        );
        mpz_mod(
            t_1.as_mut_ptr(),
            t_1.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_mul(
            t_1.as_mut_ptr(),
            t_1.as_mut_ptr() as *const __mpz_struct,
            ((*q).x).as_ptr(),
        );
        mpz_mod(
            t_1.as_mut_ptr(),
            t_1.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_mul(
            t_1.as_mut_ptr(),
            t_1.as_mut_ptr() as *const __mpz_struct,
            ((*q).y).as_ptr(),
        );
        mpz_mod(
            t_1.as_mut_ptr(),
            t_1.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_mul(x_1.as_mut_ptr(), ((*p).x).as_ptr(), ((*q).y).as_ptr());
        mpz_mod(
            x_1.as_mut_ptr(),
            x_1.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_addmul(x_1.as_mut_ptr(), ((*q).x).as_ptr(), ((*p).y).as_ptr());
        mpz_mod(
            x_1.as_mut_ptr(),
            x_1.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_ui_sub(
            s_1.as_mut_ptr(),
            1 as libc::c_int as libc::c_ulong,
            t_1.as_mut_ptr() as *const __mpz_struct,
        );
        mpz_invert(
            s_1.as_mut_ptr(),
            s_1.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_mul(
            x_1.as_mut_ptr(),
            x_1.as_mut_ptr() as *const __mpz_struct,
            s_1.as_mut_ptr() as *const __mpz_struct,
        );
        mpz_mod(
            x_1.as_mut_ptr(),
            x_1.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_mul(y_1.as_mut_ptr(), ((*p).y).as_ptr(), ((*q).y).as_ptr());
        mpz_mod(
            y_1.as_mut_ptr(),
            y_1.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_addmul(y_1.as_mut_ptr(), ((*p).x).as_ptr(), ((*q).x).as_ptr());
        mpz_mod(
            y_1.as_mut_ptr(),
            y_1.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_add_ui(
            s_1.as_mut_ptr(),
            t_1.as_mut_ptr() as *const __mpz_struct,
            1 as libc::c_int as libc::c_ulong,
        );
        mpz_invert(
            s_1.as_mut_ptr(),
            s_1.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_mul(
            y_1.as_mut_ptr(),
            y_1.as_mut_ptr() as *const __mpz_struct,
            s_1.as_mut_ptr() as *const __mpz_struct,
        );
        mpz_mod(
            y_1.as_mut_ptr(),
            y_1.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_swap(x_1.as_mut_ptr(), ((*r).x).as_mut_ptr());
        mpz_swap(y_1.as_mut_ptr(), ((*r).y).as_mut_ptr());
        (*r)
            .is_zero = (mpz_cmp_ui(
            ((*r).x).as_mut_ptr() as *const __mpz_struct,
            0 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            && mpz_cmp_ui(
                ((*r).y).as_mut_ptr() as *const __mpz_struct,
                1 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int) as libc::c_int;
        mpz_clear(s_1.as_mut_ptr());
        mpz_clear(t_1.as_mut_ptr());
        mpz_clear(x_1.as_mut_ptr());
        mpz_clear(y_1.as_mut_ptr());
    };
}
unsafe extern "C" fn ecc_mul_binary(
    mut ecc: *const ecc_curve,
    mut r: *mut ecc_point,
    mut n: *const __mpz_struct,
    mut p: *const ecc_point,
) {
    let mut k: libc::c_uint = 0;
    if r != p as *mut ecc_point {} else {
        __assert_fail(
            b"r != p\0" as *const u8 as *const libc::c_char,
            b"eccdata.c\0" as *const u8 as *const libc::c_char,
            375 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"void ecc_mul_binary(const struct ecc_curve *, struct ecc_point *, const __mpz_struct *, const struct ecc_point *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_33096: {
        if r != p as *mut ecc_point {} else {
            __assert_fail(
                b"r != p\0" as *const u8 as *const libc::c_char,
                b"eccdata.c\0" as *const u8 as *const libc::c_char,
                375 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 114],
                    &[libc::c_char; 114],
                >(
                    b"void ecc_mul_binary(const struct ecc_curve *, struct ecc_point *, const __mpz_struct *, const struct ecc_point *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if mpz_sgn(n) > 0 as libc::c_int {} else {
        __assert_fail(
            b"mpz_sgn (n) > 0\0" as *const u8 as *const libc::c_char,
            b"eccdata.c\0" as *const u8 as *const libc::c_char,
            376 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"void ecc_mul_binary(const struct ecc_curve *, struct ecc_point *, const __mpz_struct *, const struct ecc_point *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_33053: {
        if mpz_sgn(n) > 0 as libc::c_int {} else {
            __assert_fail(
                b"mpz_sgn (n) > 0\0" as *const u8 as *const libc::c_char,
                b"eccdata.c\0" as *const u8 as *const libc::c_char,
                376 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 114],
                    &[libc::c_char; 114],
                >(
                    b"void ecc_mul_binary(const struct ecc_curve *, struct ecc_point *, const __mpz_struct *, const struct ecc_point *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    ecc_set(r, p);
    k = (mpz_sizeinbase(n, 2 as libc::c_int))
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint;
    loop {
        let fresh38 = k;
        k = k.wrapping_sub(1);
        if !(fresh38 > 0 as libc::c_int as libc::c_uint) {
            break;
        }
        ecc_dup(ecc, r, r);
        if mpz_tstbit(n, k as mp_bitcnt_t) != 0 {
            ecc_add(ecc, r, r, p);
        }
    };
}
unsafe extern "C" fn ecc_alloc(mut n: size_t) -> *mut ecc_point {
    let mut p: *mut ecc_point = malloc(
        n.wrapping_mul(::std::mem::size_of::<ecc_point>() as libc::c_ulong),
    ) as *mut ecc_point;
    let mut i: size_t = 0;
    if p.is_null() {
        fprintf(
            stderr,
            b"Virtual memory exhausted.\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        ecc_init(&mut *p.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    return p;
}
unsafe extern "C" fn ecc_set_str(
    mut p: *mut ecc_point,
    mut x: *const libc::c_char,
    mut y: *const libc::c_char,
) {
    (*p).is_zero = 0 as libc::c_int;
    mpz_set_str(((*p).x).as_mut_ptr(), x, 16 as libc::c_int);
    mpz_set_str(((*p).y).as_mut_ptr(), y, 16 as libc::c_int);
}
unsafe extern "C" fn ecc_curve_init_str(
    mut ecc: *mut ecc_curve,
    mut type_0: ecc_type,
    mut p: *const libc::c_char,
    mut b: *const libc::c_char,
    mut q: *const libc::c_char,
    mut gx: *const libc::c_char,
    mut gy: *const libc::c_char,
) {
    (*ecc).type_0 = type_0;
    mpz_init_set_str(((*ecc).p).as_mut_ptr(), p, 16 as libc::c_int);
    mpz_init_set_str(((*ecc).b).as_mut_ptr(), b, 16 as libc::c_int);
    mpz_init_set_str(((*ecc).q).as_mut_ptr(), q, 16 as libc::c_int);
    ecc_init(&mut (*ecc).g);
    ecc_set_str(&mut (*ecc).g, gx, gy);
    (*ecc).pippenger_k = 0 as libc::c_int as libc::c_uint;
    (*ecc).pippenger_c = 0 as libc::c_int as libc::c_uint;
    (*ecc).table = 0 as *mut ecc_point;
    (*ecc).ref_0 = 0 as *mut ecc_point;
}
unsafe extern "C" fn ecc_curve_init(
    mut ecc: *mut ecc_curve,
    mut curve: *const libc::c_char,
) {
    if strcmp(curve, b"secp192r1\0" as *const u8 as *const libc::c_char) == 0 {
        ecc_curve_init_str(
            ecc,
            ECC_TYPE_WEIERSTRASS,
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFFFFFFFFFFFF\0" as *const u8
                as *const libc::c_char,
            b"64210519e59c80e70fa7e9ab72243049feb8deecc146b9b1\0" as *const u8
                as *const libc::c_char,
            b"ffffffffffffffffffffffff99def836146bc9b1b4d22831\0" as *const u8
                as *const libc::c_char,
            b"188da80eb03090f67cbf20eb43a18800f4ff0afd82ff1012\0" as *const u8
                as *const libc::c_char,
            b"07192b95ffc8da78631011ed6b24cdd573f977a11e794811\0" as *const u8
                as *const libc::c_char,
        );
        (*ecc).ref_0 = ecc_alloc(3 as libc::c_int as size_t);
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(0 as libc::c_int as isize),
            b"dafebf5828783f2ad35534631588a3f629a70fb16982a888\0" as *const u8
                as *const libc::c_char,
            b"dd6bda0d993da0fa46b27bbc141b868f59331afa5c7e93ab\0" as *const u8
                as *const libc::c_char,
        );
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(1 as libc::c_int as isize),
            b"76e32a2557599e6edcd283201fb2b9aadfd0d359cbb263da\0" as *const u8
                as *const libc::c_char,
            b"782c37e372ba4520aa62e0fed121d49ef3b543660cfd05fd\0" as *const u8
                as *const libc::c_char,
        );
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(2 as libc::c_int as isize),
            b"35433907297cc378b0015703374729d7a4fe46647084e4ba\0" as *const u8
                as *const libc::c_char,
            b"a2649984f2135c301ea3acb0776cd4f125389b311db3be32\0" as *const u8
                as *const libc::c_char,
        );
    } else if strcmp(curve, b"secp224r1\0" as *const u8 as *const libc::c_char) == 0 {
        ecc_curve_init_str(
            ecc,
            ECC_TYPE_WEIERSTRASS,
            b"ffffffffffffffffffffffffffffffff000000000000000000000001\0" as *const u8
                as *const libc::c_char,
            b"b4050a850c04b3abf54132565044b0b7d7bfd8ba270b39432355ffb4\0" as *const u8
                as *const libc::c_char,
            b"ffffffffffffffffffffffffffff16a2e0b8f03e13dd29455c5c2a3d\0" as *const u8
                as *const libc::c_char,
            b"b70e0cbd6bb4bf7f321390b94a03c1d356c21122343280d6115c1d21\0" as *const u8
                as *const libc::c_char,
            b"bd376388b5f723fb4c22dfe6cd4375a05a07476444d5819985007e34\0" as *const u8
                as *const libc::c_char,
        );
        (*ecc).ref_0 = ecc_alloc(3 as libc::c_int as size_t);
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(0 as libc::c_int as isize),
            b"706a46dc76dcb76798e60e6d89474788d16dc18032d268fd1a704fa6\0" as *const u8
                as *const libc::c_char,
            b"1c2b76a7bc25e7702a704fa986892849fca629487acf3709d2e4e8bb\0" as *const u8
                as *const libc::c_char,
        );
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(1 as libc::c_int as isize),
            b"df1b1d66a551d0d31eff822558b9d2cc75c2180279fe0d08fd896d04\0" as *const u8
                as *const libc::c_char,
            b"a3f7f03cadd0be444c0aa56830130ddf77d317344e1af3591981a925\0" as *const u8
                as *const libc::c_char,
        );
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(2 as libc::c_int as isize),
            b"ae99feebb5d26945b54892092a8aee02912930fa41cd114e40447301\0" as *const u8
                as *const libc::c_char,
            b"482580a0ec5bc47e88bc8c378632cd196cb3fa058a7114eb03054c9\0" as *const u8
                as *const libc::c_char,
        );
    } else if strcmp(curve, b"secp256r1\0" as *const u8 as *const libc::c_char) == 0 {
        ecc_curve_init_str(
            ecc,
            ECC_TYPE_WEIERSTRASS,
            b"FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF\0"
                as *const u8 as *const libc::c_char,
            b"5AC635D8AA3A93E7B3EBBD55769886BC651D06B0CC53B0F63BCE3C3E27D2604B\0"
                as *const u8 as *const libc::c_char,
            b"FFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551\0"
                as *const u8 as *const libc::c_char,
            b"6B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C296\0"
                as *const u8 as *const libc::c_char,
            b"4FE342E2FE1A7F9B8EE7EB4A7C0F9E162BCE33576B315ECECBB6406837BF51F5\0"
                as *const u8 as *const libc::c_char,
        );
        (*ecc).ref_0 = ecc_alloc(3 as libc::c_int as size_t);
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(0 as libc::c_int as isize),
            b"7cf27b188d034f7e8a52380304b51ac3c08969e277f21b35a60b48fc47669978\0"
                as *const u8 as *const libc::c_char,
            b"7775510db8ed040293d9ac69f7430dbba7dade63ce982299e04b79d227873d1\0"
                as *const u8 as *const libc::c_char,
        );
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(1 as libc::c_int as isize),
            b"5ecbe4d1a6330a44c8f7ef951d4bf165e6c6b721efada985fb41661bc6e7fd6c\0"
                as *const u8 as *const libc::c_char,
            b"8734640c4998ff7e374b06ce1a64a2ecd82ab036384fb83d9a79b127a27d5032\0"
                as *const u8 as *const libc::c_char,
        );
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(2 as libc::c_int as isize),
            b"e2534a3532d08fbba02dde659ee62bd0031fe2db785596ef509302446b030852\0"
                as *const u8 as *const libc::c_char,
            b"e0f1575a4c633cc719dfee5fda862d764efc96c3f30ee0055c42c23f184ed8c6\0"
                as *const u8 as *const libc::c_char,
        );
    } else if strcmp(curve, b"secp384r1\0" as *const u8 as *const libc::c_char) == 0 {
        ecc_curve_init_str(
            ecc,
            ECC_TYPE_WEIERSTRASS,
            b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000ffffffff\0"
                as *const u8 as *const libc::c_char,
            b"b3312fa7e23ee7e4988e056be3f82d19181d9c6efe8141120314088f5013875ac656398d8a2ed19d2a85c8edd3ec2aef\0"
                as *const u8 as *const libc::c_char,
            b"ffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52973\0"
                as *const u8 as *const libc::c_char,
            b"aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a385502f25dbf55296c3a545e3872760ab7\0"
                as *const u8 as *const libc::c_char,
            b"3617de4a96262c6f5d9e98bf9292dc29f8f41dbd289a147ce9da3113b5f0b8c00a60b1ce1d7e819d7a431d7c90ea0e5f\0"
                as *const u8 as *const libc::c_char,
        );
        (*ecc).ref_0 = ecc_alloc(3 as libc::c_int as size_t);
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(0 as libc::c_int as isize),
            b"8d999057ba3d2d969260045c55b97f089025959a6f434d651d207d19fb96e9e4fe0e86ebe0e64f85b96a9c75295df61\0"
                as *const u8 as *const libc::c_char,
            b"8e80f1fa5b1b3cedb7bfe8dffd6dba74b275d875bc6cc43e904e505f256ab4255ffd43e94d39e22d61501e700a940e80\0"
                as *const u8 as *const libc::c_char,
        );
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(1 as libc::c_int as isize),
            b"77a41d4606ffa1464793c7e5fdc7d98cb9d3910202dcd06bea4f240d3566da6b408bbae5026580d02d7e5c70500c831\0"
                as *const u8 as *const libc::c_char,
            b"c995f7ca0b0c42837d0bbe9602a9fc998520b41c85115aa5f7684c0edc111eacc24abd6be4b5d298b65f28600a2f1df1\0"
                as *const u8 as *const libc::c_char,
        );
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(2 as libc::c_int as isize),
            b"138251cd52ac9298c1c8aad977321deb97e709bd0b4ca0aca55dc8ad51dcfc9d1589a1597e3a5120e1efd631c63e1835\0"
                as *const u8 as *const libc::c_char,
            b"cacae29869a62e1631e8a28181ab56616dc45d918abc09f3ab0e63cf792aa4dced7387be37bba569549f1c02b270ed67\0"
                as *const u8 as *const libc::c_char,
        );
    } else if strcmp(curve, b"secp521r1\0" as *const u8 as *const libc::c_char) == 0 {
        ecc_curve_init_str(
            ecc,
            ECC_TYPE_WEIERSTRASS,
            b"1ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\0"
                as *const u8 as *const libc::c_char,
            b"051953eb9618e1c9a1f929a21a0b68540eea2da725b99b315f3b8b489918ef109e156193951ec7e937b1652c0bd3bb1bf073573df883d2c34f1ef451fd46b503f00\0"
                as *const u8 as *const libc::c_char,
            b"1fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e91386409\0"
                as *const u8 as *const libc::c_char,
            b"c6858e06b70404e9cd9e3ecb662395b4429c648139053fb521f828af606b4d3dbaa14b5e77efe75928fe1dc127a2ffa8de3348b3c1856a429bf97e7e31c2e5bd66\0"
                as *const u8 as *const libc::c_char,
            b"11839296a789a3bc0045c8a5fb42c7d1bd998f54449579b446817afbd17273e662c97ee72995ef42640c550b9013fad0761353c7086a272c24088be94769fd16650\0"
                as *const u8 as *const libc::c_char,
        );
        (*ecc).ref_0 = ecc_alloc(3 as libc::c_int as size_t);
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(0 as libc::c_int as isize),
            b"433c219024277e7e682fcb288148c282747403279b1ccc06352c6e5505d769be97b3b204da6ef55507aa104a3a35c5af41cf2fa364d60fd967f43e3933ba6d783d\0"
                as *const u8 as *const libc::c_char,
            b"f4bb8cc7f86db26700a7f3eceeeed3f0b5c6b5107c4da97740ab21a29906c42dbbb3e377de9f251f6b93937fa99a3248f4eafcbe95edc0f4f71be356d661f41b02\0"
                as *const u8 as *const libc::c_char,
        );
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(1 as libc::c_int as isize),
            b"1a73d352443de29195dd91d6a64b5959479b52a6e5b123d9ab9e5ad7a112d7a8dd1ad3f164a3a4832051da6bd16b59fe21baeb490862c32ea05a5919d2ede37ad7d\0"
                as *const u8 as *const libc::c_char,
            b"13e9b03b97dfa62ddd9979f86c6cab814f2f1557fa82a9d0317d2f8ab1fa355ceec2e2dd4cf8dc575b02d5aced1dec3c70cf105c9bc93a590425f588ca1ee86c0e5\0"
                as *const u8 as *const libc::c_char,
        );
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(2 as libc::c_int as isize),
            b"35b5df64ae2ac204c354b483487c9070cdc61c891c5ff39afc06c5d55541d3ceac8659e24afe3d0750e8b88e9f078af066a1d5025b08e5a5e2fbc87412871902f3\0"
                as *const u8 as *const libc::c_char,
            b"82096f84261279d2b673e0178eb0b4abb65521aef6e6e32e1b5ae63fe2f19907f279f283e54ba385405224f750a95b85eebb7faef04699d1d9e21f47fc346e4d0d\0"
                as *const u8 as *const libc::c_char,
        );
    } else if strcmp(curve, b"curve25519\0" as *const u8 as *const libc::c_char) == 0 {
        ecc_curve_init_str(
            ecc,
            ECC_TYPE_TWISTED_EDWARDS,
            b"7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed\0"
                as *const u8 as *const libc::c_char,
            b"2dfc9311d490018c7338bf8688861767ff8ff5b2bebe27548a14b235eca6874a\0"
                as *const u8 as *const libc::c_char,
            b"1000000000000000000000000000000014def9dea2f79cd65812631a5cf5d3ed\0"
                as *const u8 as *const libc::c_char,
            b"216936d3cd6e53fec0a4e231fdd6dc5c692cc7609525a7b2c9562d608f25d51a\0"
                as *const u8 as *const libc::c_char,
            b"6666666666666666666666666666666666666666666666666666666666666658\0"
                as *const u8 as *const libc::c_char,
        );
        (*ecc).ref_0 = ecc_alloc(3 as libc::c_int as size_t);
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(0 as libc::c_int as isize),
            b"36ab384c9f5a046c3d043b7d1833e7ac080d8e4515d7a45f83c5a14e2843ce0e\0"
                as *const u8 as *const libc::c_char,
            b"2260cdf3092329c21da25ee8c9a21f5697390f51643851560e5f46ae6af8a3c9\0"
                as *const u8 as *const libc::c_char,
        );
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(1 as libc::c_int as isize),
            b"67ae9c4a22928f491ff4ae743edac83a6343981981624886ac62485fd3f8e25c\0"
                as *const u8 as *const libc::c_char,
            b"1267b1d177ee69aba126a18e60269ef79f16ec176724030402c3684878f5b4d4\0"
                as *const u8 as *const libc::c_char,
        );
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(2 as libc::c_int as isize),
            b"203da8db56cff1468325d4b87a3520f91a739ec193ce1547493aa657c4c9f870\0"
                as *const u8 as *const libc::c_char,
            b"47d0e827cb1595e1470eb88580d5716c4cf22832ea2f0ff0df38ab61ca32112f\0"
                as *const u8 as *const libc::c_char,
        );
    } else if strcmp(curve, b"gost_gc256b\0" as *const u8 as *const libc::c_char) == 0 {
        ecc_curve_init_str(
            ecc,
            ECC_TYPE_WEIERSTRASS,
            b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd97\0"
                as *const u8 as *const libc::c_char,
            b"00000000000000000000000000000000000000000000000000000000000000a6\0"
                as *const u8 as *const libc::c_char,
            b"ffffffffffffffffffffffffffffffff6c611070995ad10045841b09b761b893\0"
                as *const u8 as *const libc::c_char,
            b"0000000000000000000000000000000000000000000000000000000000000001\0"
                as *const u8 as *const libc::c_char,
            b"8d91e471e0989cda27df505a453f2b7635294f2ddf23e3b122acc99c9e9f1e14\0"
                as *const u8 as *const libc::c_char,
        );
        (*ecc).ref_0 = ecc_alloc(3 as libc::c_int as size_t);
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(0 as libc::c_int as isize),
            b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd95\0"
                as *const u8 as *const libc::c_char,
            b"726e1b8e1f676325d820afa5bac0d489cad6b0d220dc1c4edd5336636160df83\0"
                as *const u8 as *const libc::c_char,
        );
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(1 as libc::c_int as isize),
            b"8e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38d2c\0"
                as *const u8 as *const libc::c_char,
            b"76bcd1ca9a23b041d4d9baf507a6cd821267a94c838768e8486117796b788a51\0"
                as *const u8 as *const libc::c_char,
        );
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(2 as libc::c_int as isize),
            b"f7063e7063e7063e7063e7063e7063e7063e7063e7063e7063e7063e7063e4b7\0"
                as *const u8 as *const libc::c_char,
            b"83ccf17ba6706d73625cc3534c7a2b9d6ec1ee6a9a7e07c10d84b388de59f741\0"
                as *const u8 as *const libc::c_char,
        );
    } else if strcmp(curve, b"gost_gc512a\0" as *const u8 as *const libc::c_char) == 0 {
        ecc_curve_init_str(
            ecc,
            ECC_TYPE_WEIERSTRASS,
            b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffdc7\0"
                as *const u8 as *const libc::c_char,
            b"e8c2505dedfc86ddc1bd0b2b6667f1da34b82574761cb0e879bd081cfd0b6265ee3cb090f30d27614cb4574010da90dd862ef9d4ebee4761503190785a71c760\0"
                as *const u8 as *const libc::c_char,
            b"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff27e69532f48d89116ff22b8d4e0560609b4b38abfad2b85dcacdb1411f10b275\0"
                as *const u8 as *const libc::c_char,
            b"00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003\0"
                as *const u8 as *const libc::c_char,
            b"7503cfe87a836ae3a61b8816e25450e6ce5e1c93acf1abc1778064fdcbefa921df1626be4fd036e93d75e6a50e3a41e98028fe5fc235f5b889a589cb5215f2a4\0"
                as *const u8 as *const libc::c_char,
        );
        (*ecc).ref_0 = ecc_alloc(3 as libc::c_int as size_t);
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(0 as libc::c_int as isize),
            b"3b89dcfc622996ab97a5869dbff15cf51db00954f43a58a5e5f6b0470a132b2f4434bbcd405d2a9516151d2a6a04f2e4375bf48de1fdb21fb982afd9d2ea137c\0"
                as *const u8 as *const libc::c_char,
            b"c813c4e2e2e0a8a391774c7903da7a6f14686e98e183e670ee6fb784809a3e92ca209dc631d85b1c7534ed3b37fddf64d854d7e01f91f18bb3fd307591afc051\0"
                as *const u8 as *const libc::c_char,
        );
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(1 as libc::c_int as isize),
            b"a1ff1ab2712a267eb53935ddb5a567f84db156cc096168a1174291d5f488fba543d2840b4d2dd35d764b2f57b308907aec55cfba10544e8416e134687ccb87c3\0"
                as *const u8 as *const libc::c_char,
            b"3cb5c4417ec4637f30374f189bb5b984c41e3a48d7f84fbfa3819e3f333f7eb311d3af7e67c4c16eeacfac2fe94c6dd4c6366f711a4fb6c7125cd7ec518d90d6\0"
                as *const u8 as *const libc::c_char,
        );
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(2 as libc::c_int as isize),
            b"b7bfb80956c8670031ba191929f64e301d681634236d47a60e571a4bedc0ef257452ef78b5b98dbb3d9f3129d9349433ce2a3a35cb519c91e2d633d7b373ae16\0"
                as *const u8 as *const libc::c_char,
            b"3bee95e29eecc5d5ad2beba941abcbf9f1cad478df0fecf614f63aeebef77850da7efdb93de8f3df80bc25eac09239c14175f5c29704ce9a3e383f1b3ec0e929\0"
                as *const u8 as *const libc::c_char,
        );
    } else if strcmp(curve, b"curve448\0" as *const u8 as *const libc::c_char) == 0 {
        ecc_curve_init_str(
            ecc,
            ECC_TYPE_EDWARDS,
            b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffffffffffffffffffffffffffffffffffffffffffffffffffff\0"
                as *const u8 as *const libc::c_char,
            b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffffffffffffffffffffffffffffffffffffffffffffffff6756\0"
                as *const u8 as *const libc::c_char,
            b"3fffffffffffffffffffffffffffffffffffffffffffffffffffffff7cca23e9c44edb49aed63690216cc2728dc58f552378c292ab5844f3\0"
                as *const u8 as *const libc::c_char,
            b"4f1970c66bed0ded221d15a622bf36da9e146570470f1767ea6de324a3d3a46412ae1af72ab66511433b80e18b00938e2626a82bc70cc05e\0"
                as *const u8 as *const libc::c_char,
            b"693f46716eb6bc248876203756c9c7624bea73736ca3984087789c1e05a0c2d73ad3ff1ce67c39c4fdbd132c4ed7c8ad9808795bf230fa14\0"
                as *const u8 as *const libc::c_char,
        );
        (*ecc).ref_0 = ecc_alloc(3 as libc::c_int as size_t);
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(0 as libc::c_int as isize),
            b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa955555555555555555555555555555555555555555555555555555555\0"
                as *const u8 as *const libc::c_char,
            b"ae05e9634ad7048db359d6205086c2b0036ed7a035884dd7b7e36d728ad8c4b80d6565833a2a3098bbbcb2bed1cda06bdaeafbcdea9386ed\0"
                as *const u8 as *const libc::c_char,
        );
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(1 as libc::c_int as isize),
            b"865886b9108af6455bd64316cb6943332241b8b8cda82c7e2ba077a4a3fcfe8daa9cbf7f6271fd6e862b769465da8575728173286ff2f8f\0"
                as *const u8 as *const libc::c_char,
            b"e005a8dbd5125cf706cbda7ad43aa6449a4a8d952356c3b9fce43c82ec4e1d58bb3a331bdb6767f0bffa9a68fed02dafb822ac13588ed6fc\0"
                as *const u8 as *const libc::c_char,
        );
        ecc_set_str(
            &mut *((*ecc).ref_0).offset(2 as libc::c_int as isize),
            b"49dcbc5c6c0cce2c1419a17226f929ea255a09cf4e0891c693fda4be70c74cc301b7bdf1515dd8ba21aee1798949e120e2ce42ac48ba7f30\0"
                as *const u8 as *const libc::c_char,
            b"d49077e4accde527164b33a5de021b979cb7c02f0457d845c90dc3227b8a5bc1c0d8f97ea1ca9472b5d444285d0d4f5b32e236f86de51839\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        fprintf(
            stderr,
            b"No known curve with name %s\n\0" as *const u8 as *const libc::c_char,
            curve,
        );
        exit(1 as libc::c_int);
    }
    (*ecc)
        .bit_size = mpz_sizeinbase(
        ((*ecc).p).as_mut_ptr() as *const __mpz_struct,
        2 as libc::c_int,
    ) as libc::c_uint;
}
unsafe extern "C" fn ecc_curve_clear(mut ecc: *mut ecc_curve) {
    mpz_clear(((*ecc).p).as_mut_ptr());
    mpz_clear(((*ecc).b).as_mut_ptr());
    mpz_clear(((*ecc).q).as_mut_ptr());
    ecc_clear(&mut (*ecc).g);
    if !((*ecc).table).is_null() {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*ecc).table_size as libc::c_ulong {
            ecc_clear(&mut *((*ecc).table).offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
        free((*ecc).table as *mut libc::c_void);
    }
    if !((*ecc).ref_0).is_null() {
        let mut i_0: size_t = 0;
        i_0 = 0 as libc::c_int as size_t;
        while i_0 < 3 as libc::c_int as libc::c_ulong {
            ecc_clear(&mut *((*ecc).ref_0).offset(i_0 as isize));
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
        free((*ecc).ref_0 as *mut libc::c_void);
    }
}
unsafe extern "C" fn ecc_table_size(
    mut bits: libc::c_uint,
    mut k: libc::c_uint,
    mut c: libc::c_uint,
) -> libc::c_uint {
    let mut p: libc::c_uint = bits
        .wrapping_add(k)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_div(k);
    let mut M: libc::c_uint = p
        .wrapping_add(c)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_div(c);
    return M;
}
unsafe extern "C" fn ecc_pippenger_precompute(
    mut ecc: *mut ecc_curve,
    mut k: libc::c_uint,
    mut c: libc::c_uint,
) {
    let mut M: libc::c_uint = ecc_table_size((*ecc).bit_size, k, c);
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    if M < 2 as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"Invalid parameters, implies M = %u\n\0" as *const u8
                as *const libc::c_char,
            M,
        );
        exit(1 as libc::c_int);
    }
    if M
        == ecc_table_size(
            (*ecc).bit_size,
            k.wrapping_sub(1 as libc::c_int as libc::c_uint),
            c,
        )
    {
        fprintf(
            stderr,
            b"warn: Parameters k = %u, c = %d are suboptimal, could use smaller k\n\0"
                as *const u8 as *const libc::c_char,
            k,
            c,
        );
    }
    (*ecc).pippenger_k = k;
    (*ecc).pippenger_c = c;
    (*ecc).table_size = (M << c) as mp_size_t;
    if (*ecc).table_size >= 2 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"ecc->table_size >= 2\0" as *const u8 as *const libc::c_char,
            b"eccdata.c\0" as *const u8 as *const libc::c_char,
            888 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 78],
                &[libc::c_char; 78],
            >(
                b"void ecc_pippenger_precompute(struct ecc_curve *, unsigned int, unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_34675: {
        if (*ecc).table_size >= 2 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"ecc->table_size >= 2\0" as *const u8 as *const libc::c_char,
                b"eccdata.c\0" as *const u8 as *const libc::c_char,
                888 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"void ecc_pippenger_precompute(struct ecc_curve *, unsigned int, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*ecc).table = ecc_alloc((*ecc).table_size as size_t);
    ecc_set_zero(ecc, &mut *((*ecc).table).offset(0 as libc::c_int as isize));
    ecc_set(&mut *((*ecc).table).offset(1 as libc::c_int as isize), &mut (*ecc).g);
    j = 2 as libc::c_int as libc::c_uint;
    while j < (1 as libc::c_uint) << c {
        if (j as libc::c_long) < (*ecc).table_size {} else {
            __assert_fail(
                b"j < ecc->table_size\0" as *const u8 as *const libc::c_char,
                b"eccdata.c\0" as *const u8 as *const libc::c_char,
                898 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"void ecc_pippenger_precompute(struct ecc_curve *, unsigned int, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_34578: {
            if (j as libc::c_long) < (*ecc).table_size {} else {
                __assert_fail(
                    b"j < ecc->table_size\0" as *const u8 as *const libc::c_char,
                    b"eccdata.c\0" as *const u8 as *const libc::c_char,
                    898 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 78],
                        &[libc::c_char; 78],
                    >(
                        b"void ecc_pippenger_precompute(struct ecc_curve *, unsigned int, unsigned int)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        ecc_dup(
            ecc,
            &mut *((*ecc).table).offset(j as isize),
            &mut *((*ecc).table)
                .offset(j.wrapping_div(2 as libc::c_int as libc::c_uint) as isize),
        );
        i = 1 as libc::c_int as libc::c_uint;
        while i < k {
            ecc_dup(
                ecc,
                &mut *((*ecc).table).offset(j as isize),
                &mut *((*ecc).table).offset(j as isize),
            );
            i = i.wrapping_add(1);
            i;
        }
        i = 1 as libc::c_int as libc::c_uint;
        while i < j {
            if (j.wrapping_add(i) as libc::c_long) < (*ecc).table_size {} else {
                __assert_fail(
                    b"j + i < ecc->table_size\0" as *const u8 as *const libc::c_char,
                    b"eccdata.c\0" as *const u8 as *const libc::c_char,
                    905 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 78],
                        &[libc::c_char; 78],
                    >(
                        b"void ecc_pippenger_precompute(struct ecc_curve *, unsigned int, unsigned int)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_34456: {
                if (j.wrapping_add(i) as libc::c_long) < (*ecc).table_size {} else {
                    __assert_fail(
                        b"j + i < ecc->table_size\0" as *const u8 as *const libc::c_char,
                        b"eccdata.c\0" as *const u8 as *const libc::c_char,
                        905 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 78],
                            &[libc::c_char; 78],
                        >(
                            b"void ecc_pippenger_precompute(struct ecc_curve *, unsigned int, unsigned int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            ecc_add(
                ecc,
                &mut *((*ecc).table).offset(j.wrapping_add(i) as isize),
                &mut *((*ecc).table).offset(j as isize),
                &mut *((*ecc).table).offset(i as isize),
            );
            i = i.wrapping_add(1);
            i;
        }
        j <<= 1 as libc::c_int;
    }
    j = ((1 as libc::c_int) << c) as libc::c_uint;
    while (j as libc::c_long) < (*ecc).table_size {
        ecc_dup(
            ecc,
            &mut *((*ecc).table).offset(j as isize),
            &mut *((*ecc).table)
                .offset(
                    j.wrapping_sub(((1 as libc::c_int) << c) as libc::c_uint) as isize,
                ),
        );
        i = 1 as libc::c_int as libc::c_uint;
        while i < k.wrapping_mul(c) {
            ecc_dup(
                ecc,
                &mut *((*ecc).table).offset(j as isize),
                &mut *((*ecc).table).offset(j as isize),
            );
            i = i.wrapping_add(1);
            i;
        }
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn ecc_mul_pippenger(
    mut ecc: *const ecc_curve,
    mut r: *mut ecc_point,
    mut n_input: *const __mpz_struct,
) {
    let mut n: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut k: libc::c_uint = 0;
    let mut c: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut bit_rows: libc::c_uint = 0;
    mpz_init(n.as_mut_ptr());
    mpz_mod(n.as_mut_ptr(), n_input, ((*ecc).q).as_ptr());
    ecc_set_zero(ecc, r);
    k = (*ecc).pippenger_k;
    c = (*ecc).pippenger_c;
    bit_rows = ((*ecc).bit_size)
        .wrapping_add(k)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_div(k);
    i = k;
    loop {
        let fresh39 = i;
        i = i.wrapping_sub(1);
        if !(fresh39 > 0 as libc::c_int as libc::c_uint) {
            break;
        }
        ecc_dup(ecc, r, r);
        j = 0 as libc::c_int as libc::c_uint;
        while j.wrapping_mul(c) < bit_rows {
            let mut bits: libc::c_uint = 0;
            let mut bit_index: mp_size_t = 0;
            bits = 0 as libc::c_int as libc::c_uint;
            bit_index = i.wrapping_add(k.wrapping_mul(c.wrapping_mul(j).wrapping_add(c)))
                as mp_size_t;
            while bit_index
                > i.wrapping_add(k.wrapping_mul(c).wrapping_mul(j)) as libc::c_long
            {
                bit_index -= k as libc::c_long;
                bits = bits << 1 as libc::c_int
                    | mpz_tstbit(
                        n.as_mut_ptr() as *const __mpz_struct,
                        bit_index as mp_bitcnt_t,
                    ) as libc::c_uint;
            }
            ecc_add(ecc, r, r, &mut *((*ecc).table).offset((j << c | bits) as isize));
            j = j.wrapping_add(1);
            j;
        }
    }
    mpz_clear(n.as_mut_ptr());
}
unsafe extern "C" fn ecc_point_out(mut f: *mut FILE, mut p: *const ecc_point) {
    if (*p).is_zero != 0 {
        fprintf(f, b"zero\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(f, b"(\0" as *const u8 as *const libc::c_char);
        mpz_out_str(f, 16 as libc::c_int, ((*p).x).as_ptr());
        fprintf(f, b",\n     \0" as *const u8 as *const libc::c_char);
        mpz_out_str(f, 16 as libc::c_int, ((*p).y).as_ptr());
        fprintf(f, b")\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn ecc_curve_check(mut ecc: *const ecc_curve) {
    let mut p: ecc_point = ecc_point {
        is_zero: 0,
        x: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        y: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
    };
    let mut q: ecc_point = ecc_point {
        is_zero: 0,
        x: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        y: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
    };
    let mut n: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    ecc_init(&mut p);
    ecc_init(&mut q);
    mpz_init(n.as_mut_ptr());
    ecc_dup(ecc, &mut p, &(*ecc).g);
    if !((*ecc).ref_0).is_null() {
        if ecc_equal_p(&mut p, &mut *((*ecc).ref_0).offset(0 as libc::c_int as isize))
            == 0
        {
            fprintf(
                stderr,
                b"%s:%d: ASSERT_EQUAL (%s, %s) failed.\n\0" as *const u8
                    as *const libc::c_char,
                b"eccdata.c\0" as *const u8 as *const libc::c_char,
                1011 as libc::c_int,
                b"&p\0" as *const u8 as *const libc::c_char,
                b"&ecc->ref[0]\0" as *const u8 as *const libc::c_char,
            );
            fprintf(stderr, b"p = \0" as *const u8 as *const libc::c_char);
            ecc_point_out(stderr, &mut p);
            fprintf(stderr, b"\nq = \0" as *const u8 as *const libc::c_char);
            ecc_point_out(
                stderr,
                &mut *((*ecc).ref_0).offset(0 as libc::c_int as isize),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            abort();
        }
    } else {
        fprintf(stderr, b"g2 = \0" as *const u8 as *const libc::c_char);
        mpz_out_str(
            stderr,
            16 as libc::c_int,
            (p.x).as_mut_ptr() as *const __mpz_struct,
        );
        fprintf(stderr, b"\n     \0" as *const u8 as *const libc::c_char);
        mpz_out_str(
            stderr,
            16 as libc::c_int,
            (p.y).as_mut_ptr() as *const __mpz_struct,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    ecc_add(ecc, &mut q, &mut p, &(*ecc).g);
    if !((*ecc).ref_0).is_null() {
        if ecc_equal_p(&mut q, &mut *((*ecc).ref_0).offset(1 as libc::c_int as isize))
            == 0
        {
            fprintf(
                stderr,
                b"%s:%d: ASSERT_EQUAL (%s, %s) failed.\n\0" as *const u8
                    as *const libc::c_char,
                b"eccdata.c\0" as *const u8 as *const libc::c_char,
                1022 as libc::c_int,
                b"&q\0" as *const u8 as *const libc::c_char,
                b"&ecc->ref[1]\0" as *const u8 as *const libc::c_char,
            );
            fprintf(stderr, b"p = \0" as *const u8 as *const libc::c_char);
            ecc_point_out(stderr, &mut q);
            fprintf(stderr, b"\nq = \0" as *const u8 as *const libc::c_char);
            ecc_point_out(
                stderr,
                &mut *((*ecc).ref_0).offset(1 as libc::c_int as isize),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            abort();
        }
    } else {
        fprintf(stderr, b"g3 = \0" as *const u8 as *const libc::c_char);
        mpz_out_str(
            stderr,
            16 as libc::c_int,
            (q.x).as_mut_ptr() as *const __mpz_struct,
        );
        fprintf(stderr, b"\n     \0" as *const u8 as *const libc::c_char);
        mpz_out_str(
            stderr,
            16 as libc::c_int,
            (q.y).as_mut_ptr() as *const __mpz_struct,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    ecc_add(ecc, &mut q, &mut q, &(*ecc).g);
    if !((*ecc).ref_0).is_null() {
        if ecc_equal_p(&mut q, &mut *((*ecc).ref_0).offset(2 as libc::c_int as isize))
            == 0
        {
            fprintf(
                stderr,
                b"%s:%d: ASSERT_EQUAL (%s, %s) failed.\n\0" as *const u8
                    as *const libc::c_char,
                b"eccdata.c\0" as *const u8 as *const libc::c_char,
                1034 as libc::c_int,
                b"&q\0" as *const u8 as *const libc::c_char,
                b"&ecc->ref[2]\0" as *const u8 as *const libc::c_char,
            );
            fprintf(stderr, b"p = \0" as *const u8 as *const libc::c_char);
            ecc_point_out(stderr, &mut q);
            fprintf(stderr, b"\nq = \0" as *const u8 as *const libc::c_char);
            ecc_point_out(
                stderr,
                &mut *((*ecc).ref_0).offset(2 as libc::c_int as isize),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            abort();
        }
    } else {
        fprintf(stderr, b"g4 = \0" as *const u8 as *const libc::c_char);
        mpz_out_str(
            stderr,
            16 as libc::c_int,
            (q.x).as_mut_ptr() as *const __mpz_struct,
        );
        fprintf(stderr, b"\n     \0" as *const u8 as *const libc::c_char);
        mpz_out_str(
            stderr,
            16 as libc::c_int,
            (q.y).as_mut_ptr() as *const __mpz_struct,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    ecc_dup(ecc, &mut q, &mut p);
    if !((*ecc).ref_0).is_null() {
        if ecc_equal_p(&mut q, &mut *((*ecc).ref_0).offset(2 as libc::c_int as isize))
            == 0
        {
            fprintf(
                stderr,
                b"%s:%d: ASSERT_EQUAL (%s, %s) failed.\n\0" as *const u8
                    as *const libc::c_char,
                b"eccdata.c\0" as *const u8 as *const libc::c_char,
                1046 as libc::c_int,
                b"&q\0" as *const u8 as *const libc::c_char,
                b"&ecc->ref[2]\0" as *const u8 as *const libc::c_char,
            );
            fprintf(stderr, b"p = \0" as *const u8 as *const libc::c_char);
            ecc_point_out(stderr, &mut q);
            fprintf(stderr, b"\nq = \0" as *const u8 as *const libc::c_char);
            ecc_point_out(
                stderr,
                &mut *((*ecc).ref_0).offset(2 as libc::c_int as isize),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            abort();
        }
    } else {
        fprintf(stderr, b"g4 = \0" as *const u8 as *const libc::c_char);
        mpz_out_str(
            stderr,
            16 as libc::c_int,
            (q.x).as_mut_ptr() as *const __mpz_struct,
        );
        fprintf(stderr, b"\n     \0" as *const u8 as *const libc::c_char);
        mpz_out_str(
            stderr,
            16 as libc::c_int,
            (q.y).as_mut_ptr() as *const __mpz_struct,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    ecc_mul_binary(ecc, &mut p, ((*ecc).q).as_ptr(), &(*ecc).g);
    if ecc_zero_p(&mut p) == 0 {
        fprintf(
            stderr,
            b"%s:%d: ASSERT_ZERO (%s) failed.\n\0" as *const u8 as *const libc::c_char,
            b"eccdata.c\0" as *const u8 as *const libc::c_char,
            1057 as libc::c_int,
            b"&p\0" as *const u8 as *const libc::c_char,
        );
        fprintf(stderr, b"p = \0" as *const u8 as *const libc::c_char);
        ecc_point_out(stderr, &mut p);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    ecc_mul_pippenger(ecc, &mut q, ((*ecc).q).as_ptr());
    if ecc_zero_p(&mut q) == 0 {
        fprintf(
            stderr,
            b"%s:%d: ASSERT_ZERO (%s) failed.\n\0" as *const u8 as *const libc::c_char,
            b"eccdata.c\0" as *const u8 as *const libc::c_char,
            1060 as libc::c_int,
            b"&q\0" as *const u8 as *const libc::c_char,
        );
        fprintf(stderr, b"p = \0" as *const u8 as *const libc::c_char);
        ecc_point_out(stderr, &mut q);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    ecc_clear(&mut p);
    ecc_clear(&mut q);
    mpz_clear(n.as_mut_ptr());
}
unsafe extern "C" fn output_digits(
    mut x: *const __mpz_struct,
    mut size: libc::c_uint,
    mut bits_per_limb: libc::c_uint,
) {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut mask: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut limb: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut i: libc::c_uint = 0;
    let mut suffix: *const libc::c_char = 0 as *const libc::c_char;
    mpz_init(t.as_mut_ptr());
    mpz_init(mask.as_mut_ptr());
    mpz_init(limb.as_mut_ptr());
    mpz_setbit(mask.as_mut_ptr(), bits_per_limb as mp_bitcnt_t);
    mpz_sub_ui(
        mask.as_mut_ptr(),
        mask.as_mut_ptr() as *const __mpz_struct,
        1 as libc::c_int as libc::c_ulong,
    );
    suffix = if bits_per_limb > 32 as libc::c_int as libc::c_uint {
        b"ULL\0" as *const u8 as *const libc::c_char
    } else {
        b"UL\0" as *const u8 as *const libc::c_char
    };
    mpz_init_set(t.as_mut_ptr(), x);
    i = 0 as libc::c_int as libc::c_uint;
    while i < size {
        if i.wrapping_rem(8 as libc::c_int as libc::c_uint)
            == 0 as libc::c_int as libc::c_uint
        {
            printf(b"\n \0" as *const u8 as *const libc::c_char);
        }
        mpz_and(
            limb.as_mut_ptr(),
            mask.as_mut_ptr() as *const __mpz_struct,
            t.as_mut_ptr() as *const __mpz_struct,
        );
        printf(b" 0x\0" as *const u8 as *const libc::c_char);
        mpz_out_str(stdout, 16 as libc::c_int, limb.as_mut_ptr() as *const __mpz_struct);
        printf(b"%s,\0" as *const u8 as *const libc::c_char, suffix);
        mpz_tdiv_q_2exp(
            t.as_mut_ptr(),
            t.as_mut_ptr() as *const __mpz_struct,
            bits_per_limb as mp_bitcnt_t,
        );
        i = i.wrapping_add(1);
        i;
    }
    mpz_clear(t.as_mut_ptr());
    mpz_clear(mask.as_mut_ptr());
    mpz_clear(limb.as_mut_ptr());
}
unsafe extern "C" fn output_bignum(
    mut name: *const libc::c_char,
    mut x: *const __mpz_struct,
    mut size: libc::c_uint,
    mut bits_per_limb: libc::c_uint,
) {
    printf(
        b"static const mp_limb_t %s[%d] = {\0" as *const u8 as *const libc::c_char,
        name,
        size,
    );
    output_digits(x, size, bits_per_limb);
    printf(b"\n};\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn output_bignum_redc(
    mut name: *const libc::c_char,
    mut x: *const __mpz_struct,
    mut p: *const __mpz_struct,
    mut size: libc::c_uint,
    mut bits_per_limb: libc::c_uint,
) {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init(t.as_mut_ptr());
    mpz_mul_2exp(t.as_mut_ptr(), x, size.wrapping_mul(bits_per_limb) as mp_bitcnt_t);
    mpz_mod(t.as_mut_ptr(), t.as_mut_ptr() as *const __mpz_struct, p);
    output_bignum(name, t.as_mut_ptr() as *const __mpz_struct, size, bits_per_limb);
}
unsafe extern "C" fn output_point(
    mut ecc: *const ecc_curve,
    mut p: *const ecc_point,
    mut use_redc: libc::c_int,
    mut size: libc::c_uint,
    mut bits_per_limb: libc::c_uint,
) {
    let mut x: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut y: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init(x.as_mut_ptr());
    mpz_init(y.as_mut_ptr());
    mpz_init(t.as_mut_ptr());
    mpz_set(x.as_mut_ptr(), ((*p).x).as_ptr());
    mpz_set(y.as_mut_ptr(), ((*p).y).as_ptr());
    if use_redc != 0 {
        mpz_mul_2exp(
            x.as_mut_ptr(),
            x.as_mut_ptr() as *const __mpz_struct,
            size.wrapping_mul(bits_per_limb) as mp_bitcnt_t,
        );
        mpz_mod(
            x.as_mut_ptr(),
            x.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
        mpz_mul_2exp(
            y.as_mut_ptr(),
            y.as_mut_ptr() as *const __mpz_struct,
            size.wrapping_mul(bits_per_limb) as mp_bitcnt_t,
        );
        mpz_mod(
            y.as_mut_ptr(),
            y.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
        );
    }
    output_digits(x.as_mut_ptr() as *const __mpz_struct, size, bits_per_limb);
    output_digits(y.as_mut_ptr() as *const __mpz_struct, size, bits_per_limb);
    mpz_clear(x.as_mut_ptr());
    mpz_clear(y.as_mut_ptr());
    mpz_clear(t.as_mut_ptr());
}
unsafe extern "C" fn string_toupper(
    mut buf: *mut libc::c_char,
    mut size: size_t,
    mut s: *const libc::c_char,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        *buf
            .offset(
                i as isize,
            ) = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *s.offset(i as isize) as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    };
                } else {
                    __res = toupper(*s.offset(i as isize) as libc::c_int);
                }
            } else {
                __res = *(*__ctype_toupper_loc())
                    .offset(*s.offset(i as isize) as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
        if *buf.offset(i as isize) == 0 {
            return;
        }
        i = i.wrapping_add(1);
        i;
    }
    fprintf(
        stderr,
        b"string '%s' too large for buffer of size %u.\n\0" as *const u8
            as *const libc::c_char,
        s,
        size as libc::c_uint,
    );
    abort();
}
unsafe extern "C" fn output_modulo(
    mut name: *const libc::c_char,
    mut x: *const __mpz_struct,
    mut size: libc::c_uint,
    mut bits_per_limb: libc::c_uint,
) {
    let mut bit_size: libc::c_uint = 0;
    let mut shift: libc::c_int = 0;
    let mut buf: [libc::c_char; 20] = [0; 20];
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
        b"ecc_%s\0" as *const u8 as *const libc::c_char,
        name,
    );
    output_bignum(buf.as_mut_ptr(), x, size, bits_per_limb);
    mpz_init(t.as_mut_ptr());
    mpz_setbit(t.as_mut_ptr(), bits_per_limb.wrapping_mul(size) as mp_bitcnt_t);
    mpz_mod(t.as_mut_ptr(), t.as_mut_ptr() as *const __mpz_struct, x);
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
        b"ecc_Bmod%s\0" as *const u8 as *const libc::c_char,
        name,
    );
    output_bignum(
        buf.as_mut_ptr(),
        t.as_mut_ptr() as *const __mpz_struct,
        size,
        bits_per_limb,
    );
    string_toupper(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
        name,
    );
    printf(
        b"#define ECC_BMOD%s_SIZE %u\n\0" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        (mpz_sizeinbase(t.as_mut_ptr() as *const __mpz_struct, 2 as libc::c_int))
            .wrapping_add(bits_per_limb as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(bits_per_limb as libc::c_ulong) as libc::c_uint,
    );
    bit_size = mpz_sizeinbase(x, 2 as libc::c_int) as libc::c_uint;
    shift = size.wrapping_mul(bits_per_limb).wrapping_sub(bit_size) as libc::c_int;
    if shift >= 0 as libc::c_int {} else {
        __assert_fail(
            b"shift >= 0\0" as *const u8 as *const libc::c_char,
            b"eccdata.c\0" as *const u8 as *const libc::c_char,
            1198 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 83],
                &[libc::c_char; 83],
            >(
                b"void output_modulo(const char *, const __mpz_struct *, unsigned int, unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_36834: {
        if shift >= 0 as libc::c_int {} else {
            __assert_fail(
                b"shift >= 0\0" as *const u8 as *const libc::c_char,
                b"eccdata.c\0" as *const u8 as *const libc::c_char,
                1198 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"void output_modulo(const char *, const __mpz_struct *, unsigned int, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if shift > 0 as libc::c_int {
        mpz_set_ui(t.as_mut_ptr(), 0 as libc::c_int as libc::c_ulong);
        mpz_setbit(t.as_mut_ptr(), size.wrapping_mul(bits_per_limb) as mp_bitcnt_t);
        mpz_submul_ui(t.as_mut_ptr(), x, 2 as libc::c_int as libc::c_ulong);
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
            b"ecc_Bm2%s\0" as *const u8 as *const libc::c_char,
            name,
        );
        output_bignum(
            buf.as_mut_ptr(),
            t.as_mut_ptr() as *const __mpz_struct,
            size,
            bits_per_limb,
        );
        if bit_size == 253 as libc::c_int as libc::c_uint {
            let mut mbits: libc::c_uint = 0;
            let mut shift_0: libc::c_uint = 0;
            shift_0 = bits_per_limb
                .wrapping_mul(size)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_sub(bit_size);
            mpz_set(t.as_mut_ptr(), x);
            mpz_clrbit(
                t.as_mut_ptr(),
                bit_size.wrapping_sub(1 as libc::c_int as libc::c_uint) as mp_bitcnt_t,
            );
            mbits = mpz_sizeinbase(
                t.as_mut_ptr() as *const __mpz_struct,
                2 as libc::c_int,
            ) as libc::c_uint;
            if mbits.wrapping_add(shift_0).wrapping_add(bits_per_limb) <= bit_size
            {} else {
                __assert_fail(
                    b"mbits + shift + bits_per_limb <= bit_size\0" as *const u8
                        as *const libc::c_char,
                    b"eccdata.c\0" as *const u8 as *const libc::c_char,
                    1222 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 83],
                        &[libc::c_char; 83],
                    >(
                        b"void output_modulo(const char *, const __mpz_struct *, unsigned int, unsigned int)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_36682: {
                if mbits.wrapping_add(shift_0).wrapping_add(bits_per_limb) <= bit_size
                {} else {
                    __assert_fail(
                        b"mbits + shift + bits_per_limb <= bit_size\0" as *const u8
                            as *const libc::c_char,
                        b"eccdata.c\0" as *const u8 as *const libc::c_char,
                        1222 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 83],
                            &[libc::c_char; 83],
                        >(
                            b"void output_modulo(const char *, const __mpz_struct *, unsigned int, unsigned int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            mpz_mul_2exp(
                t.as_mut_ptr(),
                t.as_mut_ptr() as *const __mpz_struct,
                shift_0 as mp_bitcnt_t,
            );
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
                b"ecc_mBmod%s_shifted\0" as *const u8 as *const libc::c_char,
                name,
            );
            output_bignum(
                buf.as_mut_ptr(),
                t.as_mut_ptr() as *const __mpz_struct,
                size,
                bits_per_limb,
            );
        } else {
            mpz_set_ui(t.as_mut_ptr(), 0 as libc::c_int as libc::c_ulong);
            mpz_setbit(t.as_mut_ptr(), bit_size as mp_bitcnt_t);
            mpz_sub(t.as_mut_ptr(), t.as_mut_ptr() as *const __mpz_struct, x);
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
                b"ecc_Bmod%s_shifted\0" as *const u8 as *const libc::c_char,
                name,
            );
            output_bignum(
                buf.as_mut_ptr(),
                t.as_mut_ptr() as *const __mpz_struct,
                size,
                bits_per_limb,
            );
            mpz_mul_2exp(
                t.as_mut_ptr(),
                t.as_mut_ptr() as *const __mpz_struct,
                (shift + 1 as libc::c_int) as mp_bitcnt_t,
            );
            if mpz_cmp(t.as_mut_ptr() as *const __mpz_struct, x) > 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"Reduction condition failed for %u-bit %s.\n\0" as *const u8
                        as *const libc::c_char,
                    bit_size,
                    name,
                );
                exit(1 as libc::c_int);
            }
        }
    } else {
        printf(
            b"#define ecc_Bm2%s ecc_Bmod%s\n\0" as *const u8 as *const libc::c_char,
            name,
            name,
        );
        printf(
            b"#define ecc_Bmod%s_shifted ecc_Bmod%s\n\0" as *const u8
                as *const libc::c_char,
            name,
            name,
        );
    }
    mpz_clear(t.as_mut_ptr());
}
unsafe extern "C" fn output_curve(
    mut ecc: *const ecc_curve,
    mut bits_per_limb: libc::c_uint,
) {
    let mut limb_size: libc::c_uint = ((*ecc).bit_size)
        .wrapping_add(bits_per_limb)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_div(bits_per_limb);
    let mut i: libc::c_uint = 0;
    let mut redc_limbs: libc::c_int = 0;
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut z: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init(t.as_mut_ptr());
    mpz_init(z.as_mut_ptr());
    printf(
        b"/* For NULL. */\n#include <stddef.h>\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"#define ECC_LIMB_SIZE %u\n\0" as *const u8 as *const libc::c_char,
        limb_size,
    );
    printf(
        b"#define ECC_PIPPENGER_K %u\n\0" as *const u8 as *const libc::c_char,
        (*ecc).pippenger_k,
    );
    printf(
        b"#define ECC_PIPPENGER_C %u\n\0" as *const u8 as *const libc::c_char,
        (*ecc).pippenger_c,
    );
    output_modulo(
        b"p\0" as *const u8 as *const libc::c_char,
        ((*ecc).p).as_ptr(),
        limb_size,
        bits_per_limb,
    );
    output_modulo(
        b"q\0" as *const u8 as *const libc::c_char,
        ((*ecc).q).as_ptr(),
        limb_size,
        bits_per_limb,
    );
    output_bignum(
        b"ecc_b\0" as *const u8 as *const libc::c_char,
        ((*ecc).b).as_ptr(),
        limb_size,
        bits_per_limb,
    );
    mpz_add_ui(t.as_mut_ptr(), ((*ecc).p).as_ptr(), 1 as libc::c_int as libc::c_ulong);
    mpz_fdiv_q_2exp(
        t.as_mut_ptr(),
        t.as_mut_ptr() as *const __mpz_struct,
        1 as libc::c_int as mp_bitcnt_t,
    );
    output_bignum(
        b"ecc_pp1h\0" as *const u8 as *const libc::c_char,
        t.as_mut_ptr() as *const __mpz_struct,
        limb_size,
        bits_per_limb,
    );
    mpz_add_ui(t.as_mut_ptr(), ((*ecc).q).as_ptr(), 1 as libc::c_int as libc::c_ulong);
    mpz_fdiv_q_2exp(
        t.as_mut_ptr(),
        t.as_mut_ptr() as *const __mpz_struct,
        1 as libc::c_int as mp_bitcnt_t,
    );
    output_bignum(
        b"ecc_qp1h\0" as *const u8 as *const libc::c_char,
        t.as_mut_ptr() as *const __mpz_struct,
        limb_size,
        bits_per_limb,
    );
    redc_limbs = (mpz_scan0(((*ecc).p).as_ptr(), 0 as libc::c_int as mp_bitcnt_t))
        .wrapping_div(bits_per_limb as libc::c_ulong) as libc::c_int;
    if redc_limbs > 0 as libc::c_int {
        mpz_add_ui(
            t.as_mut_ptr(),
            ((*ecc).p).as_ptr(),
            1 as libc::c_int as libc::c_ulong,
        );
        mpz_fdiv_q_2exp(
            t.as_mut_ptr(),
            t.as_mut_ptr() as *const __mpz_struct,
            (redc_limbs as libc::c_uint).wrapping_mul(bits_per_limb) as mp_bitcnt_t,
        );
        output_bignum(
            b"ecc_redc_ppm1\0" as *const u8 as *const libc::c_char,
            t.as_mut_ptr() as *const __mpz_struct,
            limb_size.wrapping_sub(redc_limbs as libc::c_uint),
            bits_per_limb,
        );
    } else {
        redc_limbs = (mpz_scan1(((*ecc).p).as_ptr(), 1 as libc::c_int as mp_bitcnt_t))
            .wrapping_div(bits_per_limb as libc::c_ulong) as libc::c_int;
        if redc_limbs > 0 as libc::c_int {
            printf(
                b"#define ecc_redc_ppm1 (ecc_p + %d)\n\0" as *const u8
                    as *const libc::c_char,
                redc_limbs,
            );
            redc_limbs = -redc_limbs;
        } else {
            printf(
                b"#define ecc_redc_ppm1 NULL\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    printf(
        b"#define ECC_REDC_SIZE %d\n\0" as *const u8 as *const libc::c_char,
        redc_limbs,
    );
    if !(mpz_fdiv_ui(((*ecc).p).as_ptr(), 4 as libc::c_int as libc::c_ulong)
        == 3 as libc::c_int as libc::c_ulong)
    {
        let mut g: libc::c_uint = 0;
        let mut i_0: libc::c_uint = 0;
        let mut e: libc::c_uint = 0;
        let mut s: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        mpz_init(s.as_mut_ptr());
        mpz_sub_ui(
            s.as_mut_ptr(),
            ((*ecc).p).as_ptr(),
            1 as libc::c_int as libc::c_ulong,
        );
        e = mpz_scan1(
            s.as_mut_ptr() as *const __mpz_struct,
            0 as libc::c_int as mp_bitcnt_t,
        ) as libc::c_uint;
        if e > 1 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"e > 1\0" as *const u8 as *const libc::c_char,
                b"eccdata.c\0" as *const u8 as *const libc::c_char,
                1344 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 58],
                    &[libc::c_char; 58],
                >(b"void output_curve(const struct ecc_curve *, unsigned int)\0"))
                    .as_ptr(),
            );
        }
        'c_37505: {
            if e > 1 as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"e > 1\0" as *const u8 as *const libc::c_char,
                    b"eccdata.c\0" as *const u8 as *const libc::c_char,
                    1344 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 58],
                        &[libc::c_char; 58],
                    >(b"void output_curve(const struct ecc_curve *, unsigned int)\0"))
                        .as_ptr(),
                );
            }
        };
        mpz_fdiv_q_2exp(
            s.as_mut_ptr(),
            s.as_mut_ptr() as *const __mpz_struct,
            e as mp_bitcnt_t,
        );
        g = 2 as libc::c_int as libc::c_uint;
        loop {
            mpz_set_ui(z.as_mut_ptr(), g as libc::c_ulong);
            mpz_powm(
                z.as_mut_ptr(),
                z.as_mut_ptr() as *const __mpz_struct,
                s.as_mut_ptr() as *const __mpz_struct,
                ((*ecc).p).as_ptr(),
            );
            mpz_mul(
                t.as_mut_ptr(),
                z.as_mut_ptr() as *const __mpz_struct,
                z.as_mut_ptr() as *const __mpz_struct,
            );
            mpz_mod(
                t.as_mut_ptr(),
                t.as_mut_ptr() as *const __mpz_struct,
                ((*ecc).p).as_ptr(),
            );
            i_0 = 2 as libc::c_int as libc::c_uint;
            while i_0 < e {
                mpz_mul(
                    t.as_mut_ptr(),
                    t.as_mut_ptr() as *const __mpz_struct,
                    t.as_mut_ptr() as *const __mpz_struct,
                );
                mpz_mod(
                    t.as_mut_ptr(),
                    t.as_mut_ptr() as *const __mpz_struct,
                    ((*ecc).p).as_ptr(),
                );
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
            if mpz_cmp_ui(
                t.as_mut_ptr() as *const __mpz_struct,
                1 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int
            {
                break;
            }
            g = g.wrapping_add(1);
            g;
        }
        mpz_add_ui(
            t.as_mut_ptr(),
            t.as_mut_ptr() as *const __mpz_struct,
            1 as libc::c_int as libc::c_ulong,
        );
        if mpz_cmp(t.as_mut_ptr() as *const __mpz_struct, ((*ecc).p).as_ptr())
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"mpz_cmp (t, ecc->p) == 0\0" as *const u8 as *const libc::c_char,
                b"eccdata.c\0" as *const u8 as *const libc::c_char,
                1366 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 58],
                    &[libc::c_char; 58],
                >(b"void output_curve(const struct ecc_curve *, unsigned int)\0"))
                    .as_ptr(),
            );
        }
        'c_37320: {
            if mpz_cmp(t.as_mut_ptr() as *const __mpz_struct, ((*ecc).p).as_ptr())
                == 0 as libc::c_int
            {} else {
                __assert_fail(
                    b"mpz_cmp (t, ecc->p) == 0\0" as *const u8 as *const libc::c_char,
                    b"eccdata.c\0" as *const u8 as *const libc::c_char,
                    1366 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 58],
                        &[libc::c_char; 58],
                    >(b"void output_curve(const struct ecc_curve *, unsigned int)\0"))
                        .as_ptr(),
                );
            }
        };
        mpz_fdiv_q_2exp(
            t.as_mut_ptr(),
            s.as_mut_ptr() as *const __mpz_struct,
            1 as libc::c_int as mp_bitcnt_t,
        );
        mpz_clear(s.as_mut_ptr());
        printf(b"#define ECC_SQRT_E %u\n\0" as *const u8 as *const libc::c_char, e);
    }
    printf(b"#if USE_REDC\n\0" as *const u8 as *const libc::c_char);
    printf(b"#define ecc_unit ecc_Bmodp\n\0" as *const u8 as *const libc::c_char);
    if mpz_sgn(z.as_mut_ptr() as *const __mpz_struct) > 0 as libc::c_int {
        output_bignum_redc(
            b"ecc_sqrt_z\0" as *const u8 as *const libc::c_char,
            z.as_mut_ptr() as *const __mpz_struct,
            ((*ecc).p).as_ptr(),
            limb_size,
            bits_per_limb,
        );
    }
    printf(
        b"static const mp_limb_t ecc_table[%lu] = {\0" as *const u8
            as *const libc::c_char,
        (2 as libc::c_int as libc::c_long * (*ecc).table_size
            * limb_size as libc::c_long) as libc::c_ulong,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_long) < (*ecc).table_size {
        output_point(
            ecc,
            &mut *((*ecc).table).offset(i as isize),
            1 as libc::c_int,
            limb_size,
            bits_per_limb,
        );
        i = i.wrapping_add(1);
        i;
    }
    printf(b"\n};\n\0" as *const u8 as *const libc::c_char);
    printf(b"#else\n\0" as *const u8 as *const libc::c_char);
    mpz_set_ui(t.as_mut_ptr(), 1 as libc::c_int as libc::c_ulong);
    output_bignum(
        b"ecc_unit\0" as *const u8 as *const libc::c_char,
        t.as_mut_ptr() as *const __mpz_struct,
        limb_size,
        bits_per_limb,
    );
    if mpz_sgn(z.as_mut_ptr() as *const __mpz_struct) > 0 as libc::c_int {
        output_bignum(
            b"ecc_sqrt_z\0" as *const u8 as *const libc::c_char,
            z.as_mut_ptr() as *const __mpz_struct,
            limb_size,
            bits_per_limb,
        );
    }
    printf(
        b"static const mp_limb_t ecc_table[%lu] = {\0" as *const u8
            as *const libc::c_char,
        (2 as libc::c_int as libc::c_long * (*ecc).table_size
            * limb_size as libc::c_long) as libc::c_ulong,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_long) < (*ecc).table_size {
        output_point(
            ecc,
            &mut *((*ecc).table).offset(i as isize),
            0 as libc::c_int,
            limb_size,
            bits_per_limb,
        );
        i = i.wrapping_add(1);
        i;
    }
    printf(b"\n};\n\0" as *const u8 as *const libc::c_char);
    printf(b"#endif\n\0" as *const u8 as *const libc::c_char);
    mpz_clear(t.as_mut_ptr());
    mpz_clear(z.as_mut_ptr());
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ecc: ecc_curve = ecc_curve {
        bit_size: 0,
        pippenger_k: 0,
        pippenger_c: 0,
        type_0: ECC_TYPE_WEIERSTRASS,
        p: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        b: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        q: [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1],
        g: ecc_point {
            is_zero: 0,
            x: [__mpz_struct {
                _mp_alloc: 0,
                _mp_size: 0,
                _mp_d: 0 as *mut mp_limb_t,
            }; 1],
            y: [__mpz_struct {
                _mp_alloc: 0,
                _mp_size: 0,
                _mp_d: 0 as *mut mp_limb_t,
            }; 1],
        },
        table_size: 0,
        table: 0 as *mut ecc_point,
        ref_0: 0 as *mut ecc_point,
    };
    if argc < 4 as libc::c_int {
        fprintf(
            stderr,
            b"Usage: %s CURVE K C [BITS-PER-LIMB]\n\0" as *const u8
                as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        return 1 as libc::c_int;
    }
    ecc_curve_init(&mut ecc, *argv.offset(1 as libc::c_int as isize));
    ecc_pippenger_precompute(
        &mut ecc,
        atoi(*argv.offset(2 as libc::c_int as isize)) as libc::c_uint,
        atoi(*argv.offset(3 as libc::c_int as isize)) as libc::c_uint,
    );
    fprintf(
        stderr,
        b"Table size: %lu entries\n\0" as *const u8 as *const libc::c_char,
        ecc.table_size as libc::c_ulong,
    );
    ecc_curve_check(&mut ecc);
    if argc > 4 as libc::c_int {
        output_curve(
            &mut ecc,
            atoi(*argv.offset(4 as libc::c_int as isize)) as libc::c_uint,
        );
    }
    ecc_curve_clear(&mut ecc);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
unsafe extern "C" fn run_static_initializers() {
    mp_bits_per_limb = (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
