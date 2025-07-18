use ::libc;
extern "C" {
    fn __gmpn_zero(_: mp_ptr, _: mp_size_t);
    fn __gmpn_sec_tabselect(
        _: *mut mp_limb_t,
        _: *const mp_limb_t,
        _: mp_size_t,
        _: mp_size_t,
        _: mp_size_t,
    );
    fn _nettle_ecc_dup_jj(
        ecc: *const ecc_curve,
        r: *mut mp_limb_t,
        p: *const mp_limb_t,
        scratch: *mut mp_limb_t,
    );
    fn _nettle_ecc_add_jja(
        ecc: *const ecc_curve,
        r: *mut mp_limb_t,
        p: *const mp_limb_t,
        q: *const mp_limb_t,
        scratch: *mut mp_limb_t,
    );
    fn _nettle_cnd_copy(
        cnd: libc::c_int,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        n: mp_size_t,
    );
}
pub type mp_limb_t = libc::c_ulong;
pub type mp_ptr = *mut mp_limb_t;
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
pub unsafe extern "C" fn _nettle_ecc_mul_g(
    mut ecc: *const ecc_curve,
    mut r: *mut mp_limb_t,
    mut np: *const mp_limb_t,
    mut scratch: *mut mp_limb_t,
) {
    let mut k: libc::c_uint = 0;
    let mut c: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut bit_rows: libc::c_uint = 0;
    let mut is_zero: libc::c_int = 0;
    k = (*ecc).pippenger_k as libc::c_uint;
    c = (*ecc).pippenger_c as libc::c_uint;
    bit_rows = ((*ecc).p.bit_size as libc::c_uint)
        .wrapping_add(k)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_div(k);
    __gmpn_zero(r, (3 as libc::c_int * (*ecc).p.size as libc::c_int) as mp_size_t);
    i = k;
    is_zero = 1 as libc::c_int;
    loop {
        let fresh0 = i;
        i = i.wrapping_sub(1);
        if !(fresh0 > 0 as libc::c_int as libc::c_uint) {
            break;
        }
        _nettle_ecc_dup_jj(ecc, r, r, scratch);
        j = 0 as libc::c_int as libc::c_uint;
        while j.wrapping_mul(c) < bit_rows {
            let mut bits: libc::c_uint = 0;
            let mut bit_index: libc::c_uint = 0;
            bits = 0 as libc::c_int as libc::c_uint;
            bit_index = i
                .wrapping_add(k.wrapping_mul(c.wrapping_mul(j).wrapping_add(c)));
            while bit_index > i.wrapping_add(k.wrapping_mul(c).wrapping_mul(j)) {
                let mut limb_index: mp_size_t = 0;
                let mut shift: libc::c_uint = 0;
                bit_index = bit_index.wrapping_sub(k);
                limb_index = bit_index
                    .wrapping_div((64 as libc::c_int - 0 as libc::c_int) as libc::c_uint)
                    as mp_size_t;
                if limb_index >= (*ecc).p.size as libc::c_long {
                    continue;
                }
                shift = bit_index
                    .wrapping_rem(
                        (64 as libc::c_int - 0 as libc::c_int) as libc::c_uint,
                    );
                bits = ((bits << 1 as libc::c_int) as libc::c_ulong
                    | *np.offset(limb_index as isize) >> shift
                        & 1 as libc::c_int as libc::c_ulong) as libc::c_uint;
            }
            __gmpn_sec_tabselect(
                scratch as *mut mp_limb_t,
                ((*ecc).pippenger_table)
                    .offset(
                        (((2 as libc::c_int * (*ecc).p.size as libc::c_int)
                            as libc::c_long * j as mp_size_t) << c) as isize,
                    ),
                (2 as libc::c_int * (*ecc).p.size as libc::c_int) as mp_size_t,
                ((1 as libc::c_int) << c) as mp_size_t,
                bits as mp_size_t,
            );
            _nettle_cnd_copy(
                is_zero,
                r,
                scratch,
                (2 as libc::c_int * (*ecc).p.size as libc::c_int) as mp_size_t,
            );
            _nettle_cnd_copy(
                is_zero,
                r.offset((2 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
                (*ecc).unit,
                (*ecc).p.size as mp_size_t,
            );
            _nettle_ecc_add_jja(
                ecc,
                scratch,
                r,
                scratch,
                scratch
                    .offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
            );
            _nettle_cnd_copy(
                (bits & (is_zero - 1 as libc::c_int) as libc::c_uint) as libc::c_int,
                r,
                scratch,
                (3 as libc::c_int * (*ecc).p.size as libc::c_int) as mp_size_t,
            );
            is_zero &= (bits == 0 as libc::c_int as libc::c_uint) as libc::c_int;
            j = j.wrapping_add(1);
            j;
        }
    };
}
