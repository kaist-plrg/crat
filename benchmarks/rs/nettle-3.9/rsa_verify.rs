use ::libc;
extern "C" {
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_cmp(_: mpz_srcptr, _: mpz_srcptr) -> libc::c_int;
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_powm(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr, _: mpz_srcptr);
}
pub type size_t = libc::c_ulong;
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
pub unsafe extern "C" fn _nettle_rsa_verify(
    mut key: *const rsa_public_key,
    mut m: *const __mpz_struct,
    mut s: *const __mpz_struct,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut m1: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    if (if (*s)._mp_size < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        ((*s)._mp_size > 0 as libc::c_int) as libc::c_int
    }) <= 0 as libc::c_int || __gmpz_cmp(s, ((*key).n).as_ptr()) >= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    __gmpz_init(m1.as_mut_ptr());
    __gmpz_powm(m1.as_mut_ptr(), s, ((*key).e).as_ptr(), ((*key).n).as_ptr());
    res = (__gmpz_cmp(m, m1.as_mut_ptr() as mpz_srcptr) == 0) as libc::c_int;
    __gmpz_clear(m1.as_mut_ptr());
    return res;
}
pub unsafe extern "C" fn _nettle_rsa_verify_recover(
    mut key: *const rsa_public_key,
    mut m: *mut __mpz_struct,
    mut s: *const __mpz_struct,
) -> libc::c_int {
    if (if (*s)._mp_size < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        ((*s)._mp_size > 0 as libc::c_int) as libc::c_int
    }) <= 0 as libc::c_int || __gmpz_cmp(s, ((*key).n).as_ptr()) >= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    __gmpz_powm(m, s, ((*key).e).as_ptr(), ((*key).n).as_ptr());
    return 1 as libc::c_int;
}
