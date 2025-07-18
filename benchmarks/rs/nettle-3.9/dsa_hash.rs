use ::libc;
extern "C" {
    fn __gmpz_tdiv_q_2exp(_: mpz_ptr, _: mpz_srcptr, _: mp_bitcnt_t);
    fn nettle_mpz_set_str_256_u(x: *mut __mpz_struct, length: size_t, s: *const uint8_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
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
pub unsafe extern "C" fn _nettle_dsa_hash(
    mut h: *mut __mpz_struct,
    mut bit_size: libc::c_uint,
    mut length: size_t,
    mut digest: *const uint8_t,
) {
    if length
        > bit_size
            .wrapping_add(7 as libc::c_int as libc::c_uint)
            .wrapping_div(8 as libc::c_int as libc::c_uint) as libc::c_ulong
    {
        length = bit_size
            .wrapping_add(7 as libc::c_int as libc::c_uint)
            .wrapping_div(8 as libc::c_int as libc::c_uint) as size_t;
    }
    nettle_mpz_set_str_256_u(h, length, digest);
    if (8 as libc::c_int as libc::c_ulong).wrapping_mul(length)
        > bit_size as libc::c_ulong
    {
        __gmpz_tdiv_q_2exp(
            h,
            h as mpz_srcptr,
            (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(length)
                .wrapping_sub(bit_size as libc::c_ulong),
        );
    }
}
