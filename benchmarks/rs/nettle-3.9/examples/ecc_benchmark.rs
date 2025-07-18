use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn time_init();
    static mut time_start: Option::<unsafe extern "C" fn() -> ()>;
    static mut time_end: Option::<unsafe extern "C" fn() -> libc::c_double>;
    fn __gmpn_gcdext(
        _: mp_ptr,
        _: mp_ptr,
        _: *mut mp_size_t,
        _: mp_ptr,
        _: mp_size_t,
        _: mp_ptr,
        _: mp_size_t,
    ) -> mp_size_t;
    fn __gmpn_random(_: mp_ptr, _: mp_size_t);
    fn __gmpn_sub_n(_: mp_ptr, _: mp_srcptr, _: mp_srcptr, _: mp_size_t) -> mp_limb_t;
    fn __gmpn_copyi(_: mp_ptr, _: mp_srcptr, _: mp_size_t);
    fn __gmpn_zero(_: mp_ptr, _: mp_size_t);
    fn __gmpn_sec_powm_itch(_: mp_size_t, _: mp_bitcnt_t, _: mp_size_t) -> mp_size_t;
    fn __gmpn_sec_powm(
        _: mp_ptr,
        _: mp_srcptr,
        _: mp_size_t,
        _: mp_srcptr,
        _: mp_bitcnt_t,
        _: mp_srcptr,
        _: mp_size_t,
        _: mp_ptr,
    );
    static _nettle_secp_192r1: ecc_curve;
    static _nettle_secp_224r1: ecc_curve;
    static _nettle_secp_256r1: ecc_curve;
    static _nettle_secp_384r1: ecc_curve;
    static _nettle_secp_521r1: ecc_curve;
    static _nettle_curve25519: ecc_curve;
    static _nettle_curve448: ecc_curve;
    static _nettle_gost_gc256b: ecc_curve;
    static _nettle_gost_gc512a: ecc_curve;
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
pub type mp_limb_t = libc::c_ulong;
pub type mp_bitcnt_t = libc::c_ulong;
pub type mp_ptr = *mut mp_limb_t;
pub type mp_srcptr = *const mp_limb_t;
pub type mp_size_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecc_curve {
    pub p: ecc_modulo,
    pub q: ecc_modulo,
    pub use_redc: libc::c_ushort,
    pub pippenger_k: libc::c_ushort,
    pub pippenger_c: libc::c_ushort,
    pub add_hh_itch: libc::c_ushort,
    pub add_hhh_itch: libc::c_ushort,
    pub dup_itch: libc::c_ushort,
    pub mul_itch: libc::c_ushort,
    pub mul_g_itch: libc::c_ushort,
    pub h_to_a_itch: libc::c_ushort,
    pub add_hh: Option::<ecc_add_func>,
    pub add_hhh: Option::<ecc_add_func>,
    pub dup: Option::<ecc_dup_func>,
    pub mul: Option::<ecc_mul_func>,
    pub mul_g: Option::<ecc_mul_g_func>,
    pub h_to_a: Option::<ecc_h_to_a_func>,
    pub b: *const mp_limb_t,
    pub unit: *const mp_limb_t,
    pub pippenger_table: *const mp_limb_t,
}
pub type ecc_h_to_a_func = unsafe extern "C" fn(
    *const ecc_curve,
    libc::c_int,
    *mut mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> ();
pub type ecc_mul_g_func = unsafe extern "C" fn(
    *const ecc_curve,
    *mut mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> ();
pub type ecc_mul_func = unsafe extern "C" fn(
    *const ecc_curve,
    *mut mp_limb_t,
    *const mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> ();
pub type ecc_dup_func = unsafe extern "C" fn(
    *const ecc_curve,
    *mut mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> ();
pub type ecc_add_func = unsafe extern "C" fn(
    *const ecc_curve,
    *mut mp_limb_t,
    *const mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecc_modulo {
    pub bit_size: libc::c_ushort,
    pub size: libc::c_ushort,
    pub B_size: libc::c_ushort,
    pub redc_size: libc::c_ushort,
    pub invert_itch: libc::c_ushort,
    pub sqrt_itch: libc::c_ushort,
    pub sqrt_ratio_itch: libc::c_ushort,
    pub m: *const mp_limb_t,
    pub B: *const mp_limb_t,
    pub B_shifted: *const mp_limb_t,
    pub Bm2m: *const mp_limb_t,
    pub redc_mpm1: *const mp_limb_t,
    pub mp1h: *const mp_limb_t,
    pub mod_0: Option::<ecc_mod_func>,
    pub reduce: Option::<ecc_mod_func>,
    pub invert: Option::<ecc_mod_inv_func>,
    pub sqrt: Option::<ecc_mod_sqrt_func>,
    pub sqrt_ratio: Option::<ecc_mod_sqrt_ratio_func>,
}
pub type ecc_mod_sqrt_ratio_func = unsafe extern "C" fn(
    *const ecc_modulo,
    *mut mp_limb_t,
    *const mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> libc::c_int;
pub type ecc_mod_sqrt_func = unsafe extern "C" fn(
    *const ecc_modulo,
    *mut mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> libc::c_int;
pub type ecc_mod_inv_func = unsafe extern "C" fn(
    *const ecc_modulo,
    *mut mp_limb_t,
    *const mp_limb_t,
    *mut mp_limb_t,
) -> ();
pub type ecc_mod_func = unsafe extern "C" fn(
    *const ecc_modulo,
    *mut mp_limb_t,
    *mut mp_limb_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecc_ctx {
    pub ecc: *const ecc_curve,
    pub rp: *mut mp_limb_t,
    pub ap: *mut mp_limb_t,
    pub bp: *mut mp_limb_t,
    pub tp: *mut mp_limb_t,
}
#[inline]
unsafe extern "C" fn __gmpn_sub(
    mut __gmp_wp: mp_ptr,
    mut __gmp_xp: mp_srcptr,
    mut __gmp_xsize: mp_size_t,
    mut __gmp_yp: mp_srcptr,
    mut __gmp_ysize: mp_size_t,
) -> mp_limb_t {
    let mut __gmp_c: mp_limb_t = 0;
    let mut current_block_11: u64;
    let mut __gmp_i: mp_size_t = 0;
    let mut __gmp_x: mp_limb_t = 0;
    __gmp_i = __gmp_ysize;
    if __gmp_i != 0 as libc::c_int as libc::c_long {
        if __gmpn_sub_n(__gmp_wp, __gmp_xp, __gmp_yp, __gmp_i) != 0 {
            loop {
                if __gmp_i >= __gmp_xsize {
                    __gmp_c = 1 as libc::c_int as mp_limb_t;
                    current_block_11 = 6009453772311597924;
                    break;
                } else {
                    __gmp_x = *__gmp_xp.offset(__gmp_i as isize);
                    let fresh0 = __gmp_i;
                    __gmp_i = __gmp_i + 1;
                    *__gmp_wp
                        .offset(
                            fresh0 as isize,
                        ) = __gmp_x.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        & !(0 as libc::c_int as mp_limb_t) >> 0 as libc::c_int;
                    if !(__gmp_x == 0 as libc::c_int as libc::c_ulong) {
                        current_block_11 = 13183875560443969876;
                        break;
                    }
                }
            }
        } else {
            current_block_11 = 13183875560443969876;
        }
    } else {
        current_block_11 = 13183875560443969876;
    }
    match current_block_11 {
        13183875560443969876 => {
            if __gmp_wp != __gmp_xp as mp_ptr {
                let mut __gmp_j: mp_size_t = 0;
                __gmp_j = __gmp_i;
                while __gmp_j < __gmp_xsize {
                    *__gmp_wp
                        .offset(__gmp_j as isize) = *__gmp_xp.offset(__gmp_j as isize);
                    __gmp_j += 1;
                    __gmp_j;
                }
            }
            __gmp_c = 0 as libc::c_int as mp_limb_t;
        }
        _ => {}
    }
    return __gmp_c;
}
#[inline]
unsafe extern "C" fn __gmpn_sub_1(
    mut __gmp_dst: mp_ptr,
    mut __gmp_src: mp_srcptr,
    mut __gmp_size: mp_size_t,
    mut __gmp_n: mp_limb_t,
) -> mp_limb_t {
    let mut __gmp_c: mp_limb_t = 0;
    let mut __gmp_i: mp_size_t = 0;
    let mut __gmp_x: mp_limb_t = 0;
    let mut __gmp_r: mp_limb_t = 0;
    __gmp_x = *__gmp_src.offset(0 as libc::c_int as isize);
    __gmp_r = __gmp_x.wrapping_sub(__gmp_n);
    *__gmp_dst.offset(0 as libc::c_int as isize) = __gmp_r;
    if __gmp_x < __gmp_n {
        __gmp_c = 1 as libc::c_int as mp_limb_t;
        __gmp_i = 1 as libc::c_int as mp_size_t;
        while __gmp_i < __gmp_size {
            __gmp_x = *__gmp_src.offset(__gmp_i as isize);
            __gmp_r = __gmp_x.wrapping_sub(1 as libc::c_int as libc::c_ulong);
            *__gmp_dst.offset(__gmp_i as isize) = __gmp_r;
            __gmp_i += 1;
            __gmp_i;
            if __gmp_x < 1 as libc::c_int as libc::c_ulong {
                continue;
            }
            if __gmp_src != __gmp_dst as mp_srcptr {
                let mut __gmp_j: mp_size_t = 0;
                __gmp_j = __gmp_i;
                while __gmp_j < __gmp_size {
                    *__gmp_dst
                        .offset(__gmp_j as isize) = *__gmp_src.offset(__gmp_j as isize);
                    __gmp_j += 1;
                    __gmp_j;
                }
            }
            __gmp_c = 0 as libc::c_int as mp_limb_t;
            break;
        }
    } else {
        if __gmp_src != __gmp_dst as mp_srcptr {
            let mut __gmp_j_0: mp_size_t = 0;
            __gmp_j_0 = 1 as libc::c_int as mp_size_t;
            while __gmp_j_0 < __gmp_size {
                *__gmp_dst
                    .offset(__gmp_j_0 as isize) = *__gmp_src.offset(__gmp_j_0 as isize);
                __gmp_j_0 += 1;
                __gmp_j_0;
            }
        }
        __gmp_c = 0 as libc::c_int as mp_limb_t;
    }
    return __gmp_c;
}
unsafe extern "C" fn xalloc(mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = malloc(size);
    if p.is_null() {
        fprintf(
            stderr,
            b"Virtual memory exhausted\n\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return p;
}
unsafe extern "C" fn xalloc_limbs(mut size: mp_size_t) -> *mut mp_limb_t {
    return xalloc(
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mp_limb_t>() as libc::c_ulong),
    ) as *mut mp_limb_t;
}
unsafe extern "C" fn time_function(
    mut f: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut arg: *mut libc::c_void,
) -> libc::c_double {
    let mut ncalls: libc::c_uint = 0;
    let mut elapsed: libc::c_double = 0.;
    f.unwrap()(arg);
    ncalls = 10 as libc::c_int as libc::c_uint;
    loop {
        let mut i: libc::c_uint = 0;
        time_start.unwrap()();
        i = 0 as libc::c_int as libc::c_uint;
        while i < ncalls {
            f.unwrap()(arg);
            i = i.wrapping_add(1);
            i;
        }
        elapsed = time_end.unwrap()();
        if elapsed > 0.1f64 {
            break;
        }
        if elapsed < 0.1f64 / 10 as libc::c_int as libc::c_double {
            ncalls = ncalls.wrapping_mul(10 as libc::c_int as libc::c_uint);
        } else {
            ncalls = ncalls.wrapping_mul(2 as libc::c_int as libc::c_uint);
        }
    }
    return elapsed / ncalls as libc::c_double;
}
unsafe extern "C" fn modinv_gcd(
    mut ecc: *const ecc_curve,
    mut rp: *mut mp_limb_t,
    mut ap: *mut mp_limb_t,
    mut tp: *mut mp_limb_t,
) -> libc::c_int {
    let mut size: mp_size_t = (*ecc).p.size as mp_size_t;
    let mut up: *mut mp_limb_t = tp;
    let mut vp: *mut mp_limb_t = tp
        .offset(size as isize)
        .offset(1 as libc::c_int as isize);
    let mut gp: *mut mp_limb_t = tp
        .offset(
            (2 as libc::c_int as libc::c_long
                * (size + 1 as libc::c_int as libc::c_long)) as isize,
        );
    let mut sp: *mut mp_limb_t = tp
        .offset(
            (3 as libc::c_int as libc::c_long
                * (size + 1 as libc::c_int as libc::c_long)) as isize,
        );
    let mut gn: mp_size_t = 0;
    let mut sn: mp_size_t = 0;
    __gmpn_copyi(up, ap as mp_srcptr, size);
    __gmpn_copyi(vp, (*ecc).p.m, size);
    gn = __gmpn_gcdext(gp, sp, &mut sn, up, size, vp, size);
    if gn != 1 as libc::c_int as libc::c_long
        || *gp.offset(0 as libc::c_int as isize) != 1 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if sn < 0 as libc::c_int as libc::c_long {
        __gmpn_sub(sp, (*ecc).p.m, size, sp as mp_srcptr, -sn);
    } else if sn < size {
        __gmpn_zero(sp.offset(sn as isize), size - sn);
    }
    __gmpn_copyi(rp, sp as mp_srcptr, size);
    return 1 as libc::c_int;
}
unsafe extern "C" fn bench_modp(mut p: *mut libc::c_void) {
    let mut ctx: *mut ecc_ctx = p as *mut ecc_ctx;
    __gmpn_copyi(
        (*ctx).rp,
        (*ctx).ap as mp_srcptr,
        (2 as libc::c_int * (*(*ctx).ecc).p.size as libc::c_int) as mp_size_t,
    );
    ((*(*ctx).ecc).p.mod_0).unwrap()(&(*(*ctx).ecc).p, (*ctx).rp, (*ctx).rp);
}
unsafe extern "C" fn bench_reduce(mut p: *mut libc::c_void) {
    let mut ctx: *mut ecc_ctx = p as *mut ecc_ctx;
    __gmpn_copyi(
        (*ctx).rp,
        (*ctx).ap as mp_srcptr,
        (2 as libc::c_int * (*(*ctx).ecc).p.size as libc::c_int) as mp_size_t,
    );
    ((*(*ctx).ecc).p.reduce).unwrap()(&(*(*ctx).ecc).p, (*ctx).rp, (*ctx).rp);
}
unsafe extern "C" fn bench_modq(mut p: *mut libc::c_void) {
    let mut ctx: *mut ecc_ctx = p as *mut ecc_ctx;
    __gmpn_copyi(
        (*ctx).rp,
        (*ctx).ap as mp_srcptr,
        (2 as libc::c_int * (*(*ctx).ecc).p.size as libc::c_int) as mp_size_t,
    );
    ((*(*ctx).ecc).q.mod_0).unwrap()(&(*(*ctx).ecc).q, (*ctx).rp, (*ctx).rp);
}
unsafe extern "C" fn bench_pinv(mut p: *mut libc::c_void) {
    let mut ctx: *mut ecc_ctx = p as *mut ecc_ctx;
    ((*(*ctx).ecc).p.invert).unwrap()(&(*(*ctx).ecc).p, (*ctx).rp, (*ctx).ap, (*ctx).tp);
}
unsafe extern "C" fn bench_qinv(mut p: *mut libc::c_void) {
    let mut ctx: *mut ecc_ctx = p as *mut ecc_ctx;
    ((*(*ctx).ecc).q.invert).unwrap()(&(*(*ctx).ecc).p, (*ctx).rp, (*ctx).ap, (*ctx).tp);
}
unsafe extern "C" fn bench_modinv_gcd(mut p: *mut libc::c_void) {
    let mut ctx: *mut ecc_ctx = p as *mut ecc_ctx;
    __gmpn_copyi(
        ((*ctx).rp).offset((*(*ctx).ecc).p.size as libc::c_int as isize),
        (*ctx).ap as mp_srcptr,
        (*(*ctx).ecc).p.size as mp_size_t,
    );
    modinv_gcd(
        (*ctx).ecc,
        (*ctx).rp,
        ((*ctx).rp).offset((*(*ctx).ecc).p.size as libc::c_int as isize),
        (*ctx).tp,
    );
}
unsafe extern "C" fn bench_modinv_powm(mut p: *mut libc::c_void) {
    let mut ctx: *mut ecc_ctx = p as *mut ecc_ctx;
    let mut ecc: *const ecc_curve = (*ctx).ecc;
    let mut size: mp_size_t = (*ecc).p.size as mp_size_t;
    __gmpn_sub_1(
        ((*ctx).rp).offset(size as isize),
        (*ecc).p.m,
        size,
        2 as libc::c_int as mp_limb_t,
    );
    __gmpn_sec_powm(
        (*ctx).rp,
        (*ctx).ap as mp_srcptr,
        size,
        ((*ctx).rp).offset(size as isize) as mp_srcptr,
        (*ecc).p.bit_size as mp_bitcnt_t,
        (*ecc).p.m,
        size,
        (*ctx).tp,
    );
}
unsafe extern "C" fn bench_dup_hh(mut p: *mut libc::c_void) {
    let mut ctx: *mut ecc_ctx = p as *mut ecc_ctx;
    ((*(*ctx).ecc).dup).unwrap()((*ctx).ecc, (*ctx).rp, (*ctx).ap, (*ctx).tp);
}
unsafe extern "C" fn bench_add_hh(mut p: *mut libc::c_void) {
    let mut ctx: *mut ecc_ctx = p as *mut ecc_ctx;
    ((*(*ctx).ecc).add_hh)
        .unwrap()((*ctx).ecc, (*ctx).rp, (*ctx).ap, (*ctx).bp, (*ctx).tp);
}
unsafe extern "C" fn bench_add_hhh(mut p: *mut libc::c_void) {
    let mut ctx: *mut ecc_ctx = p as *mut ecc_ctx;
    ((*(*ctx).ecc).add_hhh)
        .unwrap()((*ctx).ecc, (*ctx).rp, (*ctx).ap, (*ctx).bp, (*ctx).tp);
}
unsafe extern "C" fn bench_mul_g(mut p: *mut libc::c_void) {
    let mut ctx: *mut ecc_ctx = p as *mut ecc_ctx;
    ((*(*ctx).ecc).mul_g).unwrap()((*ctx).ecc, (*ctx).rp, (*ctx).ap, (*ctx).tp);
}
unsafe extern "C" fn bench_mul_a(mut p: *mut libc::c_void) {
    let mut ctx: *mut ecc_ctx = p as *mut ecc_ctx;
    ((*(*ctx).ecc).mul).unwrap()((*ctx).ecc, (*ctx).rp, (*ctx).ap, (*ctx).bp, (*ctx).tp);
}
unsafe extern "C" fn bench_curve(mut ecc: *const ecc_curve) {
    let mut ctx: ecc_ctx = ecc_ctx {
        ecc: 0 as *const ecc_curve,
        rp: 0 as *mut mp_limb_t,
        ap: 0 as *mut mp_limb_t,
        bp: 0 as *mut mp_limb_t,
        tp: 0 as *mut mp_limb_t,
    };
    let mut modp: libc::c_double = 0.;
    let mut reduce: libc::c_double = 0.;
    let mut modq: libc::c_double = 0.;
    let mut pinv: libc::c_double = 0.;
    let mut qinv: libc::c_double = 0.;
    let mut modinv_gcd_0: libc::c_double = 0.;
    let mut modinv_powm: libc::c_double = 0.;
    let mut dup_hh: libc::c_double = 0.;
    let mut add_hh: libc::c_double = 0.;
    let mut add_hhh: libc::c_double = 0.;
    let mut mul_g: libc::c_double = 0.;
    let mut mul_a: libc::c_double = 0.;
    let mut mask: mp_limb_t = 0;
    let mut itch: mp_size_t = 0;
    ctx.ecc = ecc;
    ctx
        .rp = xalloc_limbs(
        (3 as libc::c_int * (*ecc).p.size as libc::c_int) as mp_size_t,
    );
    ctx
        .ap = xalloc_limbs(
        (3 as libc::c_int * (*ecc).p.size as libc::c_int) as mp_size_t,
    );
    ctx
        .bp = xalloc_limbs(
        (3 as libc::c_int * (*ecc).p.size as libc::c_int) as mp_size_t,
    );
    itch = (*ecc).mul_itch as mp_size_t;
    let mut powm_itch: mp_size_t = __gmpn_sec_powm_itch(
        (*ecc).p.size as mp_size_t,
        (*ecc).p.bit_size as mp_bitcnt_t,
        (*ecc).p.size as mp_size_t,
    );
    if powm_itch > itch {
        itch = powm_itch;
    }
    ctx.tp = xalloc_limbs(itch);
    __gmpn_random(
        ctx.ap,
        (3 as libc::c_int * (*ecc).p.size as libc::c_int) as mp_size_t,
    );
    __gmpn_random(
        ctx.bp,
        (3 as libc::c_int * (*ecc).p.size as libc::c_int) as mp_size_t,
    );
    mask = !(0 as libc::c_int as mp_limb_t)
        >> (*ecc).p.size as libc::c_int * (64 as libc::c_int - 0 as libc::c_int)
            - (*ecc).p.bit_size as libc::c_int;
    let ref mut fresh1 = *(ctx.ap)
        .offset(((*ecc).p.size as libc::c_int - 1 as libc::c_int) as isize);
    *fresh1 &= mask;
    let ref mut fresh2 = *(ctx.ap)
        .offset(
            (2 as libc::c_int * (*ecc).p.size as libc::c_int - 1 as libc::c_int) as isize,
        );
    *fresh2 &= mask;
    let ref mut fresh3 = *(ctx.ap)
        .offset(
            (3 as libc::c_int * (*ecc).p.size as libc::c_int - 1 as libc::c_int) as isize,
        );
    *fresh3 &= mask;
    let ref mut fresh4 = *(ctx.bp)
        .offset(((*ecc).p.size as libc::c_int - 1 as libc::c_int) as isize);
    *fresh4 &= mask;
    let ref mut fresh5 = *(ctx.bp)
        .offset(
            (2 as libc::c_int * (*ecc).p.size as libc::c_int - 1 as libc::c_int) as isize,
        );
    *fresh5 &= mask;
    let ref mut fresh6 = *(ctx.bp)
        .offset(
            (3 as libc::c_int * (*ecc).p.size as libc::c_int - 1 as libc::c_int) as isize,
        );
    *fresh6 &= mask;
    modp = time_function(
        Some(bench_modp as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        &mut ctx as *mut ecc_ctx as *mut libc::c_void,
    );
    reduce = time_function(
        Some(bench_reduce as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        &mut ctx as *mut ecc_ctx as *mut libc::c_void,
    );
    modq = time_function(
        Some(bench_modq as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        &mut ctx as *mut ecc_ctx as *mut libc::c_void,
    );
    pinv = time_function(
        Some(bench_pinv as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        &mut ctx as *mut ecc_ctx as *mut libc::c_void,
    );
    qinv = time_function(
        Some(bench_qinv as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        &mut ctx as *mut ecc_ctx as *mut libc::c_void,
    );
    modinv_gcd_0 = time_function(
        Some(bench_modinv_gcd as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        &mut ctx as *mut ecc_ctx as *mut libc::c_void,
    );
    modinv_powm = time_function(
        Some(bench_modinv_powm as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        &mut ctx as *mut ecc_ctx as *mut libc::c_void,
    );
    dup_hh = time_function(
        Some(bench_dup_hh as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        &mut ctx as *mut ecc_ctx as *mut libc::c_void,
    );
    add_hh = time_function(
        Some(bench_add_hh as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        &mut ctx as *mut ecc_ctx as *mut libc::c_void,
    );
    add_hhh = time_function(
        Some(bench_add_hhh as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        &mut ctx as *mut ecc_ctx as *mut libc::c_void,
    );
    mul_g = time_function(
        Some(bench_mul_g as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        &mut ctx as *mut ecc_ctx as *mut libc::c_void,
    );
    mul_a = time_function(
        Some(bench_mul_a as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        &mut ctx as *mut ecc_ctx as *mut libc::c_void,
    );
    free(ctx.rp as *mut libc::c_void);
    free(ctx.ap as *mut libc::c_void);
    free(ctx.bp as *mut libc::c_void);
    free(ctx.tp as *mut libc::c_void);
    printf(
        b"%4d %6.4f %6.4f %6.4f %6.2f %6.2f %6.3f %6.2f %6.3f %6.3f %6.3f %6.1f %6.1f\n\0"
            as *const u8 as *const libc::c_char,
        (*ecc).p.bit_size as libc::c_int,
        1e6f64 * modp,
        1e6f64 * reduce,
        1e6f64 * modq,
        1e6f64 * pinv,
        1e6f64 * qinv,
        1e6f64 * modinv_gcd_0,
        1e6f64 * modinv_powm,
        1e6f64 * dup_hh,
        1e6f64 * add_hh,
        1e6f64 * add_hhh,
        1e6f64 * mul_g,
        1e6f64 * mul_a,
    );
}
pub static mut curves: [*const ecc_curve; 9] = unsafe {
    [
        &_nettle_secp_192r1 as *const ecc_curve,
        &_nettle_secp_224r1 as *const ecc_curve,
        &_nettle_curve25519 as *const ecc_curve,
        &_nettle_secp_256r1 as *const ecc_curve,
        &_nettle_secp_384r1 as *const ecc_curve,
        &_nettle_curve448 as *const ecc_curve,
        &_nettle_secp_521r1 as *const ecc_curve,
        &_nettle_gost_gc256b as *const ecc_curve,
        &_nettle_gost_gc512a as *const ecc_curve,
    ]
};
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    time_init();
    printf(
        b"%4s %6s %6s %6s %6s %6s %6s %6s %6s %6s %6s %6s %6s (us)\n\0" as *const u8
            as *const libc::c_char,
        b"size\0" as *const u8 as *const libc::c_char,
        b"modp\0" as *const u8 as *const libc::c_char,
        b"reduce\0" as *const u8 as *const libc::c_char,
        b"modq\0" as *const u8 as *const libc::c_char,
        b"pinv\0" as *const u8 as *const libc::c_char,
        b"qinv\0" as *const u8 as *const libc::c_char,
        b"mi_gcd\0" as *const u8 as *const libc::c_char,
        b"mi_pow\0" as *const u8 as *const libc::c_char,
        b"dup_hh\0" as *const u8 as *const libc::c_char,
        b"add_hh\0" as *const u8 as *const libc::c_char,
        b"ad_hhh\0" as *const u8 as *const libc::c_char,
        b"mul_g\0" as *const u8 as *const libc::c_char,
        b"mul_a\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*const ecc_curve; 9]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const ecc_curve>() as libc::c_ulong)
    {
        bench_curve(curves[i as usize]);
        i = i.wrapping_add(1);
        i;
    }
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
