use ::libc;
extern "C" {
    fn __gmpz_add(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_add_ui(_: mpz_ptr, _: mpz_srcptr, _: libc::c_ulong);
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_cmp(_: mpz_srcptr, _: mpz_srcptr) -> libc::c_int;
    fn __gmpz_congruent_p(_: mpz_srcptr, _: mpz_srcptr, _: mpz_srcptr) -> libc::c_int;
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_init_set_ui(_: mpz_ptr, _: libc::c_ulong);
    fn __gmpz_mul(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_mul_ui(_: mpz_ptr, _: mpz_srcptr, _: libc::c_ulong);
    fn __gmpz_set_ui(_: mpz_ptr, _: libc::c_ulong);
    fn __gmpz_sub(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_sub_ui(_: mpz_ptr, _: mpz_srcptr, _: libc::c_ulong);
    fn __gmpz_submul(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_roinit_n(_: mpz_ptr, _: mp_srcptr, _: mp_size_t) -> mpz_srcptr;
    fn _nettle_gmp_free_limbs(p: *mut mp_limb_t, n: mp_size_t);
    fn _nettle_gmp_alloc_limbs(n: mp_size_t) -> *mut mp_limb_t;
    fn _nettle_mpz_limbs_copy(xp: *mut mp_limb_t, x: mpz_srcptr, n: mp_size_t);
    fn _nettle_mpz_set_n(r: *mut __mpz_struct, xp: *const mp_limb_t, xn: mp_size_t);
}
pub type mp_limb_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = [__mpz_struct; 1];
pub type mp_srcptr = *const mp_limb_t;
pub type mp_size_t = libc::c_long;
pub type mpz_srcptr = *const __mpz_struct;
pub type mpz_ptr = *mut __mpz_struct;
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
pub struct ecc_point {
    pub ecc: *const ecc_curve,
    pub p: *mut mp_limb_t,
}
pub unsafe extern "C" fn nettle_ecc_point_init(
    mut p: *mut ecc_point,
    mut ecc: *const ecc_curve,
) {
    (*p).ecc = ecc;
    (*p)
        .p = _nettle_gmp_alloc_limbs(
        (2 as libc::c_int * (*ecc).p.size as libc::c_int) as mp_size_t,
    );
}
pub unsafe extern "C" fn nettle_ecc_point_clear(mut p: *mut ecc_point) {
    _nettle_gmp_free_limbs(
        (*p).p,
        (2 as libc::c_int * (*(*p).ecc).p.size as libc::c_int) as mp_size_t,
    );
}
pub unsafe extern "C" fn nettle_ecc_point_set(
    mut p: *mut ecc_point,
    mut x: *const __mpz_struct,
    mut y: *const __mpz_struct,
) -> libc::c_int {
    let mut size: mp_size_t = 0;
    let mut m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut lhs: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut rhs: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut res: libc::c_int = 0;
    size = (*(*p).ecc).p.size as mp_size_t;
    __gmpz_roinit_n(m.as_mut_ptr(), (*(*p).ecc).p.m, size);
    if (if (*x)._mp_size < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        ((*x)._mp_size > 0 as libc::c_int) as libc::c_int
    }) < 0 as libc::c_int
        || __gmpz_cmp(x, m.as_mut_ptr() as mpz_srcptr) >= 0 as libc::c_int
        || (if (*y)._mp_size < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            ((*y)._mp_size > 0 as libc::c_int) as libc::c_int
        }) < 0 as libc::c_int
        || __gmpz_cmp(y, m.as_mut_ptr() as mpz_srcptr) >= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    __gmpz_init(lhs.as_mut_ptr());
    __gmpz_init(rhs.as_mut_ptr());
    __gmpz_mul(lhs.as_mut_ptr(), y, y);
    if (*(*p).ecc).p.bit_size as libc::c_int == 255 as libc::c_int {
        let mut x2: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        __gmpz_init(x2.as_mut_ptr());
        __gmpz_mul(x2.as_mut_ptr(), x, x);
        __gmpz_mul(
            rhs.as_mut_ptr(),
            x2.as_mut_ptr() as mpz_srcptr,
            lhs.as_mut_ptr() as mpz_srcptr,
        );
        __gmpz_sub(
            lhs.as_mut_ptr(),
            x2.as_mut_ptr() as mpz_srcptr,
            lhs.as_mut_ptr() as mpz_srcptr,
        );
        __gmpz_add_ui(
            lhs.as_mut_ptr(),
            lhs.as_mut_ptr() as mpz_srcptr,
            1 as libc::c_int as libc::c_ulong,
        );
        __gmpz_mul_ui(
            lhs.as_mut_ptr(),
            lhs.as_mut_ptr() as mpz_srcptr,
            121666 as libc::c_int as libc::c_ulong,
        );
        __gmpz_mul_ui(
            rhs.as_mut_ptr(),
            rhs.as_mut_ptr() as mpz_srcptr,
            121665 as libc::c_int as libc::c_ulong,
        );
        __gmpz_clear(x2.as_mut_ptr());
    } else if (*(*p).ecc).p.bit_size as libc::c_int == 448 as libc::c_int {
        let mut x2_0: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        let mut d: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        __gmpz_init(x2_0.as_mut_ptr());
        __gmpz_init_set_ui(d.as_mut_ptr(), 39081 as libc::c_int as libc::c_ulong);
        __gmpz_mul(x2_0.as_mut_ptr(), x, x);
        __gmpz_mul(
            d.as_mut_ptr(),
            d.as_mut_ptr() as mpz_srcptr,
            x2_0.as_mut_ptr() as mpz_srcptr,
        );
        __gmpz_set_ui(rhs.as_mut_ptr(), 1 as libc::c_int as libc::c_ulong);
        __gmpz_submul(
            rhs.as_mut_ptr(),
            d.as_mut_ptr() as mpz_srcptr,
            lhs.as_mut_ptr() as mpz_srcptr,
        );
        __gmpz_add(
            lhs.as_mut_ptr(),
            x2_0.as_mut_ptr() as mpz_srcptr,
            lhs.as_mut_ptr() as mpz_srcptr,
        );
        __gmpz_clear(d.as_mut_ptr());
        __gmpz_clear(x2_0.as_mut_ptr());
    } else {
        __gmpz_mul(rhs.as_mut_ptr(), x, x);
        __gmpz_sub_ui(
            rhs.as_mut_ptr(),
            rhs.as_mut_ptr() as mpz_srcptr,
            3 as libc::c_int as libc::c_ulong,
        );
        __gmpz_mul(rhs.as_mut_ptr(), rhs.as_mut_ptr() as mpz_srcptr, x);
        __gmpz_add(
            rhs.as_mut_ptr(),
            rhs.as_mut_ptr() as mpz_srcptr,
            __gmpz_roinit_n(t.as_mut_ptr(), (*(*p).ecc).b, size),
        );
    }
    res = __gmpz_congruent_p(
        lhs.as_mut_ptr() as mpz_srcptr,
        rhs.as_mut_ptr() as mpz_srcptr,
        __gmpz_roinit_n(t.as_mut_ptr(), (*(*p).ecc).p.m, size),
    );
    __gmpz_clear(lhs.as_mut_ptr());
    __gmpz_clear(rhs.as_mut_ptr());
    if res == 0 {
        return 0 as libc::c_int;
    }
    _nettle_mpz_limbs_copy((*p).p, x, size);
    _nettle_mpz_limbs_copy(((*p).p).offset(size as isize), y, size);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn nettle_ecc_point_get(
    mut p: *const ecc_point,
    mut x: *mut __mpz_struct,
    mut y: *mut __mpz_struct,
) {
    let mut size: mp_size_t = (*(*p).ecc).p.size as mp_size_t;
    if !x.is_null() {
        _nettle_mpz_set_n(x, (*p).p, size);
    }
    if !y.is_null() {
        _nettle_mpz_set_n(y, ((*p).p).offset(size as isize), size);
    }
}
