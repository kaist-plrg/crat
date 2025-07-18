use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmpn_zero(_: mp_ptr, _: mp_size_t);
    fn __gmpn_sec_tabselect(
        _: *mut mp_limb_t,
        _: *const mp_limb_t,
        _: mp_size_t,
        _: mp_size_t,
        _: mp_size_t,
    );
    fn _nettle_ecc_a_to_j(ecc: *const ecc_curve, r: *mut mp_limb_t, p: *const mp_limb_t);
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
    fn _nettle_ecc_add_jjj(
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
unsafe extern "C" fn table_init(
    mut ecc: *const ecc_curve,
    mut table_0: *mut mp_limb_t,
    mut bits: libc::c_uint,
    mut p: *const mp_limb_t,
    mut scratch: *mut mp_limb_t,
) {
    let mut size: libc::c_uint = ((1 as libc::c_int) << bits) as libc::c_uint;
    let mut j: libc::c_uint = 0;
    __gmpn_zero(
        table_0
            .offset(
                (0 as libc::c_int * 3 as libc::c_int * (*ecc).p.size as libc::c_int)
                    as isize,
            ),
        (3 as libc::c_int * (*ecc).p.size as libc::c_int) as mp_size_t,
    );
    _nettle_ecc_a_to_j(
        ecc,
        table_0
            .offset(
                (1 as libc::c_int * 3 as libc::c_int * (*ecc).p.size as libc::c_int)
                    as isize,
            ),
        p,
    );
    j = 2 as libc::c_int as libc::c_uint;
    while j < size {
        _nettle_ecc_dup_jj(
            ecc,
            table_0
                .offset(
                    j
                        .wrapping_mul(3 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*ecc).p.size as libc::c_uint) as isize,
                ),
            table_0
                .offset(
                    j
                        .wrapping_div(2 as libc::c_int as libc::c_uint)
                        .wrapping_mul(3 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*ecc).p.size as libc::c_uint) as isize,
                ),
            scratch,
        );
        _nettle_ecc_add_jja(
            ecc,
            table_0
                .offset(
                    j
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                        .wrapping_mul(3 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*ecc).p.size as libc::c_uint) as isize,
                ),
            table_0
                .offset(
                    j
                        .wrapping_mul(3 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*ecc).p.size as libc::c_uint) as isize,
                ),
            table_0
                .offset(
                    (1 as libc::c_int * 3 as libc::c_int * (*ecc).p.size as libc::c_int)
                        as isize,
                ),
            scratch,
        );
        j = j.wrapping_add(2 as libc::c_int as libc::c_uint);
    }
}
pub unsafe extern "C" fn _nettle_ecc_mul_a(
    mut ecc: *const ecc_curve,
    mut r: *mut mp_limb_t,
    mut np: *const mp_limb_t,
    mut p: *const mp_limb_t,
    mut scratch: *mut mp_limb_t,
) {
    let mut scratch_out: *mut mp_limb_t = scratch
        .offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize)
        .offset(
            ((3 as libc::c_int * (*ecc).p.size as libc::c_int) << 4 as libc::c_int)
                as isize,
        );
    let mut is_zero: libc::c_int = 0 as libc::c_int;
    let mut blocks: libc::c_uint = (((*ecc).p.bit_size as libc::c_int + 4 as libc::c_int
        - 1 as libc::c_int) / 4 as libc::c_int) as libc::c_uint;
    let mut bit_index: libc::c_uint = blocks
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_mul(4 as libc::c_int as libc::c_uint);
    let mut limb_index: mp_size_t = bit_index
        .wrapping_div((64 as libc::c_int - 0 as libc::c_int) as libc::c_uint)
        as mp_size_t;
    let mut shift: libc::c_uint = bit_index
        .wrapping_rem((64 as libc::c_int - 0 as libc::c_int) as libc::c_uint);
    let mut w: mp_limb_t = 0;
    let mut bits: mp_limb_t = 0;
    table_init(
        ecc,
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        4 as libc::c_int as libc::c_uint,
        p,
        scratch_out,
    );
    w = *np.offset(limb_index as isize);
    bits = w >> shift;
    if limb_index < ((*ecc).p.size as libc::c_int - 1 as libc::c_int) as libc::c_long {
        bits
            |= *np.offset((limb_index + 1 as libc::c_int as libc::c_long) as isize)
                << ((64 as libc::c_int - 0 as libc::c_int) as libc::c_uint)
                    .wrapping_sub(shift);
    }
    if bits < ((1 as libc::c_uint) << 4 as libc::c_int) as libc::c_ulong {} else {
        __assert_fail(
            b"bits < TABLE_SIZE\0" as *const u8 as *const libc::c_char,
            b"ecc-mul-a.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"void _nettle_ecc_mul_a(const struct ecc_curve *, mp_limb_t *, const mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4171: {
        if bits < ((1 as libc::c_uint) << 4 as libc::c_int) as libc::c_ulong {} else {
            __assert_fail(
                b"bits < TABLE_SIZE\0" as *const u8 as *const libc::c_char,
                b"ecc-mul-a.c\0" as *const u8 as *const libc::c_char,
                145 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 113],
                    &[libc::c_char; 113],
                >(
                    b"void _nettle_ecc_mul_a(const struct ecc_curve *, mp_limb_t *, const mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    __gmpn_sec_tabselect(
        r as *mut mp_limb_t,
        scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
        (3 as libc::c_int * (*ecc).p.size as libc::c_int) as mp_size_t,
        ((1 as libc::c_uint) << 4 as libc::c_int) as mp_size_t,
        bits as mp_size_t,
    );
    is_zero = (bits == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
    loop {
        let mut j: libc::c_uint = 0;
        if shift >= 4 as libc::c_int as libc::c_uint {
            shift = shift.wrapping_sub(4 as libc::c_int as libc::c_uint);
            bits = w >> shift;
        } else if limb_index == 0 as libc::c_int as libc::c_long {
            if shift == 0 as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"shift == 0\0" as *const u8 as *const libc::c_char,
                    b"ecc-mul-a.c\0" as *const u8 as *const libc::c_char,
                    162 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 113],
                        &[libc::c_char; 113],
                    >(
                        b"void _nettle_ecc_mul_a(const struct ecc_curve *, mp_limb_t *, const mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_4063: {
                if shift == 0 as libc::c_int as libc::c_uint {} else {
                    __assert_fail(
                        b"shift == 0\0" as *const u8 as *const libc::c_char,
                        b"ecc-mul-a.c\0" as *const u8 as *const libc::c_char,
                        162 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 113],
                            &[libc::c_char; 113],
                        >(
                            b"void _nettle_ecc_mul_a(const struct ecc_curve *, mp_limb_t *, const mp_limb_t *, const mp_limb_t *, mp_limb_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            break;
        } else {
            bits = w << (4 as libc::c_int as libc::c_uint).wrapping_sub(shift);
            limb_index -= 1;
            w = *np.offset(limb_index as isize);
            shift = shift
                .wrapping_add((64 as libc::c_int - 0 as libc::c_int) as libc::c_uint)
                .wrapping_sub(4 as libc::c_int as libc::c_uint);
            bits |= w >> shift;
        }
        j = 0 as libc::c_int as libc::c_uint;
        while j < 4 as libc::c_int as libc::c_uint {
            _nettle_ecc_dup_jj(ecc, r, r, scratch_out);
            j = j.wrapping_add(1);
            j;
        }
        bits
            &= ((1 as libc::c_uint) << 4 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong;
        __gmpn_sec_tabselect(
            scratch as *mut mp_limb_t,
            scratch.offset((3 as libc::c_int * (*ecc).p.size as libc::c_int) as isize),
            (3 as libc::c_int * (*ecc).p.size as libc::c_int) as mp_size_t,
            ((1 as libc::c_uint) << 4 as libc::c_int) as mp_size_t,
            bits as mp_size_t,
        );
        _nettle_cnd_copy(
            is_zero,
            r,
            scratch,
            (3 as libc::c_int * (*ecc).p.size as libc::c_int) as mp_size_t,
        );
        _nettle_ecc_add_jjj(ecc, scratch, scratch, r, scratch_out);
        _nettle_cnd_copy(
            (bits & (is_zero - 1 as libc::c_int) as libc::c_ulong) as libc::c_int,
            r,
            scratch,
            (3 as libc::c_int * (*ecc).p.size as libc::c_int) as mp_size_t,
        );
        is_zero &= (bits == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
    };
}
