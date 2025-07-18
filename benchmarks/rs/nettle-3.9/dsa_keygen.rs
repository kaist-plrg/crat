use ::libc;
extern "C" {
    fn __gmpz_add_ui(_: mpz_ptr, _: mpz_srcptr, _: libc::c_ulong);
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_init_set(_: mpz_ptr, _: mpz_srcptr);
    fn __gmpz_powm(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_sub_ui(_: mpz_ptr, _: mpz_srcptr, _: libc::c_ulong);
    fn nettle_mpz_random(
        x: *mut __mpz_struct,
        ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        n: *const __mpz_struct,
    );
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
pub struct dsa_params {
    pub p: mpz_t,
    pub q: mpz_t,
    pub g: mpz_t,
}
pub unsafe extern "C" fn nettle_dsa_generate_keypair(
    mut params: *const dsa_params,
    mut pub_0: *mut __mpz_struct,
    mut key: *mut __mpz_struct,
    mut random_ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
) {
    let mut r: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    __gmpz_init_set(r.as_mut_ptr(), ((*params).q).as_ptr());
    __gmpz_sub_ui(
        r.as_mut_ptr(),
        r.as_mut_ptr() as mpz_srcptr,
        2 as libc::c_int as libc::c_ulong,
    );
    nettle_mpz_random(key, random_ctx, random, r.as_mut_ptr() as *const __mpz_struct);
    __gmpz_add_ui(key, key as mpz_srcptr, 1 as libc::c_int as libc::c_ulong);
    __gmpz_powm(
        pub_0,
        ((*params).g).as_ptr(),
        key as mpz_srcptr,
        ((*params).p).as_ptr(),
    );
    __gmpz_clear(r.as_mut_ptr());
}
