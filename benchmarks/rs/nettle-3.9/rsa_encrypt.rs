use ::libc;
extern "C" {
    fn __gmpz_powm(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr, _: mpz_srcptr);
    fn nettle_pkcs1_encrypt(
        key_size: size_t,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        length: size_t,
        message: *const uint8_t,
        m: *mut __mpz_struct,
    ) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: libc::c_int,
    pub _mp_size: libc::c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = [__mpz_struct; 1];
pub type mpz_srcptr = *const __mpz_struct;
pub type mpz_ptr = *mut __mpz_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_public_key {
    pub size: size_t,
    pub n: mpz_t,
    pub e: mpz_t,
}
pub unsafe extern "C" fn nettle_rsa_encrypt(
    mut key: *const rsa_public_key,
    mut random_ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
    mut length: size_t,
    mut message: *const uint8_t,
    mut gibberish: *mut __mpz_struct,
) -> libc::c_int {
    if nettle_pkcs1_encrypt((*key).size, random_ctx, random, length, message, gibberish)
        != 0
    {
        __gmpz_powm(
            gibberish,
            gibberish as mpz_srcptr,
            ((*key).e).as_ptr(),
            ((*key).n).as_ptr(),
        );
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
