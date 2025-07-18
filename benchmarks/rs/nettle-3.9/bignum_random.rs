use ::libc;
extern "C" {
    fn __gmpz_fdiv_r(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_fdiv_r_2exp(_: mpz_ptr, _: mpz_srcptr, _: mp_bitcnt_t);
    fn __gmpz_sizeinbase(_: mpz_srcptr, _: libc::c_int) -> size_t;
    fn nettle_mpz_set_str_256_u(x: *mut __mpz_struct, length: size_t, s: *const uint8_t);
    fn _nettle_gmp_alloc(n: size_t) -> *mut libc::c_void;
    fn _nettle_gmp_free(p: *mut libc::c_void, n: size_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type nettle_random_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
pub type mp_limb_t = libc::c_ulong;
pub type mp_bitcnt_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_srcptr = *const __mpz_struct;
pub type mpz_ptr = *mut __mpz_struct;
pub unsafe extern "C" fn nettle_mpz_random_size(
    mut x: *mut __mpz_struct,
    mut ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
    mut bits: libc::c_uint,
) {
    let mut length: libc::c_uint = bits
        .wrapping_add(7 as libc::c_int as libc::c_uint)
        .wrapping_div(8 as libc::c_int as libc::c_uint);
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp_data_size: size_t = 0;
    tmp_data_size = length as size_t;
    data = _nettle_gmp_alloc(
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul(length as libc::c_ulong),
    ) as *mut uint8_t;
    random.unwrap()(ctx, length as size_t, data);
    nettle_mpz_set_str_256_u(x, length as size_t, data);
    if bits.wrapping_rem(8 as libc::c_int as libc::c_uint) != 0 {
        __gmpz_fdiv_r_2exp(x, x as mpz_srcptr, bits as mp_bitcnt_t);
    }
    _nettle_gmp_free(data as *mut libc::c_void, tmp_data_size);
}
pub unsafe extern "C" fn nettle_mpz_random(
    mut x: *mut __mpz_struct,
    mut ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
    mut n: *const __mpz_struct,
) {
    nettle_mpz_random_size(
        x,
        ctx,
        random,
        (__gmpz_sizeinbase(n, 2 as libc::c_int))
            .wrapping_add(64 as libc::c_int as libc::c_ulong) as libc::c_uint,
    );
    __gmpz_fdiv_r(x, x as mpz_srcptr, n);
}
