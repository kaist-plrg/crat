use ::libc;
extern "C" {
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_sizeinbase(_: mpz_srcptr, _: libc::c_int) -> size_t;
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
pub unsafe extern "C" fn nettle_rsa_public_key_init(mut key: *mut rsa_public_key) {
    __gmpz_init(((*key).n).as_mut_ptr());
    __gmpz_init(((*key).e).as_mut_ptr());
    (*key).size = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn nettle_rsa_public_key_clear(mut key: *mut rsa_public_key) {
    __gmpz_clear(((*key).n).as_mut_ptr());
    __gmpz_clear(((*key).e).as_mut_ptr());
}
pub unsafe extern "C" fn _nettle_rsa_check_size(mut n: *mut __mpz_struct) -> size_t {
    let mut size: size_t = 0;
    if ((*n)._mp_size != 0 as libc::c_int) as libc::c_int
        & *((*n)._mp_d).offset(0 as libc::c_int as isize) as libc::c_int == 0
    {
        return 0 as libc::c_int as size_t;
    }
    size = (__gmpz_sizeinbase(n as mpz_srcptr, 2 as libc::c_int))
        .wrapping_add(7 as libc::c_int as libc::c_ulong)
        .wrapping_div(8 as libc::c_int as libc::c_ulong);
    if size < 12 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    return size;
}
pub unsafe extern "C" fn nettle_rsa_public_key_prepare(
    mut key: *mut rsa_public_key,
) -> libc::c_int {
    (*key).size = _nettle_rsa_check_size(((*key).n).as_mut_ptr());
    return ((*key).size > 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
