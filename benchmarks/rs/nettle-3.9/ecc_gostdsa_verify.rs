use ::libc;
extern "C" {
    fn __gmpn_sub_n(_: mp_ptr, _: mp_srcptr, _: mp_srcptr, _: mp_size_t) -> mp_limb_t;
    fn _nettle_ecc_mod_mul_canonical(
        m: *const ecc_modulo,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        bp: *const mp_limb_t,
        tp: *mut mp_limb_t,
    );
    fn _nettle_gost_hash(
        m: *const ecc_modulo,
        hp_0: *mut mp_limb_t,
        length: size_t,
        digest: *const uint8_t,
    );
    fn _nettle_ecc_j_to_a(
        ecc: *const ecc_curve,
        op: libc::c_int,
        r: *mut mp_limb_t,
        p: *const mp_limb_t,
        scratch: *mut mp_limb_t,
    );
    fn _nettle_ecc_nonsec_add_jjj(
        ecc: *const ecc_curve,
        r: *mut mp_limb_t,
        p: *const mp_limb_t,
        q: *const mp_limb_t,
        scratch: *mut mp_limb_t,
    ) -> libc::c_int;
    fn _nettle_ecc_mul_a(
        ecc: *const ecc_curve,
        r: *mut mp_limb_t,
        np: *const mp_limb_t,
        p: *const mp_limb_t,
        scratch: *mut mp_limb_t,
    );
    fn _nettle_ecc_mul_g(
        ecc: *const ecc_curve,
        r: *mut mp_limb_t,
        np: *const mp_limb_t,
        scratch: *mut mp_limb_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type mp_limb_t = libc::c_ulong;
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
#[inline]
unsafe extern "C" fn __gmpn_add_1(
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
    __gmp_r = __gmp_x.wrapping_add(__gmp_n);
    *__gmp_dst.offset(0 as libc::c_int as isize) = __gmp_r;
    if __gmp_r < __gmp_n {
        __gmp_c = 1 as libc::c_int as mp_limb_t;
        __gmp_i = 1 as libc::c_int as mp_size_t;
        while __gmp_i < __gmp_size {
            __gmp_x = *__gmp_src.offset(__gmp_i as isize);
            __gmp_r = __gmp_x.wrapping_add(1 as libc::c_int as libc::c_ulong);
            *__gmp_dst.offset(__gmp_i as isize) = __gmp_r;
            __gmp_i += 1;
            __gmp_i;
            if __gmp_r < 1 as libc::c_int as libc::c_ulong {
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
#[inline]
unsafe extern "C" fn __gmpn_cmp(
    mut __gmp_xp: mp_srcptr,
    mut __gmp_yp: mp_srcptr,
    mut __gmp_size: mp_size_t,
) -> libc::c_int {
    let mut __gmp_result: libc::c_int = 0;
    let mut __gmp_i: mp_size_t = 0;
    let mut __gmp_x: mp_limb_t = 0;
    let mut __gmp_y: mp_limb_t = 0;
    __gmp_result = 0 as libc::c_int;
    __gmp_i = __gmp_size;
    loop {
        __gmp_i -= 1;
        if !(__gmp_i >= 0 as libc::c_int as libc::c_long) {
            break;
        }
        __gmp_x = *__gmp_xp.offset(__gmp_i as isize);
        __gmp_y = *__gmp_yp.offset(__gmp_i as isize);
        if !(__gmp_x != __gmp_y) {
            continue;
        }
        __gmp_result = if __gmp_x > __gmp_y {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
        break;
    }
    return __gmp_result;
}
#[inline]
unsafe extern "C" fn __gmpn_zero_p(
    mut __gmp_p: mp_srcptr,
    mut __gmp_n: mp_size_t,
) -> libc::c_int {
    loop {
        __gmp_n -= 1;
        if *__gmp_p.offset(__gmp_n as isize) != 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
        if !(__gmp_n != 0 as libc::c_int as libc::c_long) {
            break;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn ecdsa_in_range(
    mut ecc: *const ecc_curve,
    mut xp: *const mp_limb_t,
) -> libc::c_int {
    return (__gmpn_zero_p(xp, (*ecc).p.size as mp_size_t) == 0
        && __gmpn_cmp(xp, (*ecc).q.m, (*ecc).p.size as mp_size_t) < 0 as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn nettle_ecc_gostdsa_verify_itch(
    mut ecc: *const ecc_curve,
) -> mp_size_t {
    return (5 as libc::c_int * (*ecc).p.size as libc::c_int
        + (((3 as libc::c_int) << 4 as libc::c_int) + 8 as libc::c_int)
            * (*ecc).p.size as libc::c_int) as mp_size_t;
}
pub unsafe extern "C" fn nettle_ecc_gostdsa_verify(
    mut ecc: *const ecc_curve,
    mut pp: *const mp_limb_t,
    mut length: size_t,
    mut digest: *const uint8_t,
    mut rp: *const mp_limb_t,
    mut sp: *const mp_limb_t,
    mut scratch: *mut mp_limb_t,
) -> libc::c_int {
    if !(ecdsa_in_range(ecc, rp) != 0 && ecdsa_in_range(ecc, sp) != 0) {
        return 0 as libc::c_int;
    }
    _nettle_gost_hash(&(*ecc).q, scratch, length, digest);
    if __gmpn_zero_p(scratch as mp_srcptr, (*ecc).p.size as mp_size_t) != 0 {
        __gmpn_add_1(
            scratch,
            scratch as mp_srcptr,
            (*ecc).p.size as mp_size_t,
            1 as libc::c_int as mp_limb_t,
        );
    }
    ((*ecc).q.invert)
        .unwrap()(
        &(*ecc).q,
        scratch.offset((*ecc).p.size as libc::c_int as isize),
        scratch,
        scratch
            .offset((*ecc).p.size as libc::c_int as isize)
            .offset((*ecc).p.size as libc::c_int as isize),
    );
    _nettle_ecc_mod_mul_canonical(
        &(*ecc).q,
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        sp,
        scratch.offset((*ecc).p.size as libc::c_int as isize),
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    __gmpn_sub_n(scratch, (*ecc).q.m, rp, (*ecc).p.size as mp_size_t);
    _nettle_ecc_mod_mul_canonical(
        &(*ecc).q,
        scratch.offset((4 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch,
        scratch.offset((*ecc).p.size as libc::c_int as isize),
        scratch.offset((4 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    _nettle_ecc_mul_a(
        ecc,
        scratch,
        scratch.offset((4 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        pp,
        scratch
            .offset((4 as libc::c_int * (*ecc).p.size as libc::c_int) as isize)
            .offset((*ecc).p.size as libc::c_int as isize),
    );
    _nettle_ecc_mul_g(
        ecc,
        scratch.offset((4 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch
            .offset((4 as libc::c_int * (*ecc).p.size as libc::c_int) as isize)
            .offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    if _nettle_ecc_nonsec_add_jjj(
        ecc,
        scratch.offset((4 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch.offset((4 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch,
        scratch
            .offset((4 as libc::c_int * (*ecc).p.size as libc::c_int) as isize)
            .offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    ) == 0
    {
        return 0 as libc::c_int;
    }
    _nettle_ecc_j_to_a(
        ecc,
        2 as libc::c_int,
        scratch,
        scratch.offset((4 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        scratch
            .offset((4 as libc::c_int * (*ecc).p.size as libc::c_int) as isize)
            .offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
    );
    return (__gmpn_cmp(rp, scratch as mp_srcptr, (*ecc).p.size as mp_size_t)
        == 0 as libc::c_int) as libc::c_int;
}
