use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmpn_copyi(_: mp_ptr, _: mp_srcptr, _: mp_size_t);
    fn __gmpn_zero(_: mp_ptr, _: mp_size_t);
    fn __gmpn_cnd_swap(_: mp_limb_t, _: *mut mp_limb_t, _: *mut mp_limb_t, _: mp_size_t);
    fn _nettle_ecc_mod_add(
        m: *const ecc_modulo,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        bp: *const mp_limb_t,
    );
    fn _nettle_ecc_mod_sub(
        m: *const ecc_modulo,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        bp: *const mp_limb_t,
    );
    fn _nettle_ecc_mod_addmul_1(
        m: *const ecc_modulo,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        b: mp_limb_t,
    );
    fn _nettle_ecc_mod_mul(
        m: *const ecc_modulo,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        bp: *const mp_limb_t,
        tp_0: *mut mp_limb_t,
    );
    fn _nettle_ecc_mod_sqr(
        m: *const ecc_modulo,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        tp_0: *mut mp_limb_t,
    );
    fn _nettle_ecc_mod_mul_canonical(
        m: *const ecc_modulo,
        rp: *mut mp_limb_t,
        ap: *const mp_limb_t,
        bp: *const mp_limb_t,
        tp_0: *mut mp_limb_t,
    );
}
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type mp_limb_t = libc::c_ulong;
pub type mp_ptr = *mut mp_limb_t;
pub type mp_srcptr = *const mp_limb_t;
pub type mp_size_t = libc::c_long;
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
pub unsafe extern "C" fn _nettle_ecc_mul_m(
    mut m: *const ecc_modulo,
    mut a24: mp_limb_t,
    mut bit_low: libc::c_uint,
    mut bit_high: libc::c_uint,
    mut qx: *mut mp_limb_t,
    mut n: *const uint8_t,
    mut px: *const mp_limb_t,
    mut scratch: *mut mp_limb_t,
) {
    let mut i: libc::c_uint = 0;
    let mut swap: mp_limb_t = 0;
    __gmpn_copyi(scratch, px, (*m).size as mp_size_t);
    *scratch
        .offset((*m).size as libc::c_int as isize)
        .offset(0 as libc::c_int as isize) = 1 as libc::c_int as mp_limb_t;
    __gmpn_zero(
        scratch
            .offset((*m).size as libc::c_int as isize)
            .offset(1 as libc::c_int as isize),
        ((*m).size as libc::c_int - 1 as libc::c_int) as mp_size_t,
    );
    _nettle_ecc_mod_add(
        m,
        scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
        scratch,
        scratch.offset((*m).size as libc::c_int as isize),
    );
    _nettle_ecc_mod_sub(
        m,
        scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
        scratch,
        scratch.offset((*m).size as libc::c_int as isize),
    );
    _nettle_ecc_mod_sqr(
        m,
        scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
        scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
        scratch.offset((6 as libc::c_int * (*m).size as libc::c_int) as isize),
    );
    _nettle_ecc_mod_sqr(
        m,
        scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
        scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
        scratch.offset((6 as libc::c_int * (*m).size as libc::c_int) as isize),
    );
    _nettle_ecc_mod_mul(
        m,
        scratch.offset((2 as libc::c_int * (*m).size as libc::c_int) as isize),
        scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
        scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
        scratch.offset((6 as libc::c_int * (*m).size as libc::c_int) as isize),
    );
    _nettle_ecc_mod_sub(
        m,
        scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
        scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
        scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
    );
    _nettle_ecc_mod_addmul_1(
        m,
        scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
        scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
        a24,
    );
    _nettle_ecc_mod_mul(
        m,
        scratch.offset((3 as libc::c_int * (*m).size as libc::c_int) as isize),
        scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
        scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
        scratch.offset((6 as libc::c_int * (*m).size as libc::c_int) as isize),
    );
    i = bit_high;
    swap = 0 as libc::c_int as mp_limb_t;
    while i >= bit_low {
        let mut bit: mp_limb_t = (*n
            .offset(i.wrapping_div(8 as libc::c_int as libc::c_uint) as isize)
            as libc::c_int >> (i & 7 as libc::c_int as libc::c_uint) & 1 as libc::c_int)
            as mp_limb_t;
        __gmpn_cnd_swap(
            swap ^ bit,
            scratch as *mut mp_limb_t,
            scratch.offset((2 as libc::c_int * (*m).size as libc::c_int) as isize)
                as *mut mp_limb_t,
            (2 as libc::c_int * (*m).size as libc::c_int) as mp_size_t,
        );
        swap = bit;
        _nettle_ecc_mod_add(
            m,
            scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch,
            scratch.offset((*m).size as libc::c_int as isize),
        );
        _nettle_ecc_mod_sub(
            m,
            scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((2 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((3 as libc::c_int * (*m).size as libc::c_int) as isize),
        );
        _nettle_ecc_mod_mul(
            m,
            scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((6 as libc::c_int * (*m).size as libc::c_int) as isize),
        );
        _nettle_ecc_mod_sqr(
            m,
            scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((6 as libc::c_int * (*m).size as libc::c_int) as isize),
        );
        _nettle_ecc_mod_sub(
            m,
            scratch.offset((*m).size as libc::c_int as isize),
            scratch,
            scratch.offset((*m).size as libc::c_int as isize),
        );
        _nettle_ecc_mod_add(
            m,
            scratch.offset((3 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((2 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((3 as libc::c_int * (*m).size as libc::c_int) as isize),
        );
        _nettle_ecc_mod_mul(
            m,
            scratch.offset((3 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((3 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((*m).size as libc::c_int as isize),
            scratch.offset((6 as libc::c_int * (*m).size as libc::c_int) as isize),
        );
        _nettle_ecc_mod_sqr(
            m,
            scratch.offset((*m).size as libc::c_int as isize),
            scratch.offset((*m).size as libc::c_int as isize),
            scratch.offset((6 as libc::c_int * (*m).size as libc::c_int) as isize),
        );
        _nettle_ecc_mod_mul(
            m,
            scratch,
            scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((*m).size as libc::c_int as isize),
            scratch.offset((6 as libc::c_int * (*m).size as libc::c_int) as isize),
        );
        _nettle_ecc_mod_sub(
            m,
            scratch.offset((*m).size as libc::c_int as isize),
            scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((*m).size as libc::c_int as isize),
        );
        _nettle_ecc_mod_addmul_1(
            m,
            scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((*m).size as libc::c_int as isize),
            a24,
        );
        _nettle_ecc_mod_mul(
            m,
            scratch.offset((*m).size as libc::c_int as isize),
            scratch.offset((*m).size as libc::c_int as isize),
            scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((6 as libc::c_int * (*m).size as libc::c_int) as isize),
        );
        _nettle_ecc_mod_add(
            m,
            scratch.offset((2 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((3 as libc::c_int * (*m).size as libc::c_int) as isize),
        );
        _nettle_ecc_mod_sqr(
            m,
            scratch.offset((2 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((2 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((6 as libc::c_int * (*m).size as libc::c_int) as isize),
        );
        _nettle_ecc_mod_sub(
            m,
            scratch.offset((3 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((3 as libc::c_int * (*m).size as libc::c_int) as isize),
        );
        _nettle_ecc_mod_sqr(
            m,
            scratch.offset((3 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((3 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((6 as libc::c_int * (*m).size as libc::c_int) as isize),
        );
        _nettle_ecc_mod_mul(
            m,
            scratch.offset((3 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((3 as libc::c_int * (*m).size as libc::c_int) as isize),
            px,
            scratch.offset((6 as libc::c_int * (*m).size as libc::c_int) as isize),
        );
        i = i.wrapping_sub(1);
        i;
    }
    __gmpn_cnd_swap(
        swap,
        scratch as *mut mp_limb_t,
        scratch.offset((2 as libc::c_int * (*m).size as libc::c_int) as isize)
            as *mut mp_limb_t,
        (2 as libc::c_int * (*m).size as libc::c_int) as mp_size_t,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < bit_low {
        _nettle_ecc_mod_add(
            m,
            scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch,
            scratch.offset((*m).size as libc::c_int as isize),
        );
        _nettle_ecc_mod_sub(
            m,
            scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch,
            scratch.offset((*m).size as libc::c_int as isize),
        );
        _nettle_ecc_mod_sqr(
            m,
            scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((6 as libc::c_int * (*m).size as libc::c_int) as isize),
        );
        _nettle_ecc_mod_sqr(
            m,
            scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((6 as libc::c_int * (*m).size as libc::c_int) as isize),
        );
        _nettle_ecc_mod_mul(
            m,
            scratch,
            scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((6 as libc::c_int * (*m).size as libc::c_int) as isize),
        );
        _nettle_ecc_mod_sub(
            m,
            scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
        );
        _nettle_ecc_mod_addmul_1(
            m,
            scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
            a24,
        );
        _nettle_ecc_mod_mul(
            m,
            scratch.offset((*m).size as libc::c_int as isize),
            scratch.offset((5 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((4 as libc::c_int * (*m).size as libc::c_int) as isize),
            scratch.offset((6 as libc::c_int * (*m).size as libc::c_int) as isize),
        );
        i = i.wrapping_add(1);
        i;
    }
    if (*m).invert_itch as libc::c_int <= 7 as libc::c_int * (*m).size as libc::c_int
    {} else {
        __assert_fail(
            b"m->invert_itch <= 7 * m->size\0" as *const u8 as *const libc::c_char,
            b"ecc-mul-m.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 151],
                &[libc::c_char; 151],
            >(
                b"void _nettle_ecc_mul_m(const struct ecc_modulo *, mp_limb_t, unsigned int, unsigned int, mp_limb_t *, const uint8_t *, const mp_limb_t *, mp_limb_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3877: {
        if (*m).invert_itch as libc::c_int <= 7 as libc::c_int * (*m).size as libc::c_int
        {} else {
            __assert_fail(
                b"m->invert_itch <= 7 * m->size\0" as *const u8 as *const libc::c_char,
                b"ecc-mul-m.c\0" as *const u8 as *const libc::c_char,
                163 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 151],
                    &[libc::c_char; 151],
                >(
                    b"void _nettle_ecc_mul_m(const struct ecc_modulo *, mp_limb_t, unsigned int, unsigned int, mp_limb_t *, const uint8_t *, const mp_limb_t *, mp_limb_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    ((*m).invert)
        .unwrap()(
        m,
        scratch.offset((2 as libc::c_int * (*m).size as libc::c_int) as isize),
        scratch.offset((*m).size as libc::c_int as isize),
        scratch
            .offset((3 as libc::c_int * (*m).size as libc::c_int) as isize)
            .offset((*m).size as libc::c_int as isize),
    );
    _nettle_ecc_mod_mul_canonical(
        m,
        qx,
        scratch,
        scratch.offset((2 as libc::c_int * (*m).size as libc::c_int) as isize),
        scratch.offset((3 as libc::c_int * (*m).size as libc::c_int) as isize),
    );
}
