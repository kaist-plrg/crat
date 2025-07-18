use ::libc;
extern "C" {
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_cmp(_: mpz_srcptr, _: mpz_srcptr) -> libc::c_int;
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_mul(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn nettle_rsa_compute_root(
        key: *const rsa_private_key,
        x: *mut __mpz_struct,
        m: *const __mpz_struct,
    );
    fn nettle_pkcs1_decrypt(
        key_size: size_t,
        m: *const __mpz_struct,
        length: *mut size_t,
        message: *mut uint8_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
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
pub struct rsa_private_key {
    pub size: size_t,
    pub d: mpz_t,
    pub p: mpz_t,
    pub q: mpz_t,
    pub a: mpz_t,
    pub b: mpz_t,
    pub c: mpz_t,
}
pub unsafe extern "C" fn nettle_rsa_decrypt(
    mut key: *const rsa_private_key,
    mut length: *mut size_t,
    mut message: *mut uint8_t,
    mut gibberish: *const __mpz_struct,
) -> libc::c_int {
    let mut m: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut res: libc::c_int = 0;
    __gmpz_init(m.as_mut_ptr());
    __gmpz_mul(m.as_mut_ptr(), ((*key).p).as_ptr(), ((*key).q).as_ptr());
    if (if (*gibberish)._mp_size < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        ((*gibberish)._mp_size > 0 as libc::c_int) as libc::c_int
    }) < 0 as libc::c_int
        || __gmpz_cmp(gibberish, m.as_mut_ptr() as mpz_srcptr) >= 0 as libc::c_int
    {
        __gmpz_clear(m.as_mut_ptr());
        return 0 as libc::c_int;
    }
    nettle_rsa_compute_root(key, m.as_mut_ptr(), gibberish);
    res = nettle_pkcs1_decrypt(
        (*key).size,
        m.as_mut_ptr() as *const __mpz_struct,
        length,
        message,
    );
    __gmpz_clear(m.as_mut_ptr());
    return res;
}
