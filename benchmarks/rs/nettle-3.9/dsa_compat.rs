use ::libc;
extern "C" {
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_init(_: mpz_ptr);
    fn nettle_dsa_params_init(params: *mut dsa_params);
    fn nettle_dsa_params_clear(params: *mut dsa_params);
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
pub type mpz_ptr = *mut __mpz_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_params {
    pub p: mpz_t,
    pub q: mpz_t,
    pub g: mpz_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_public_key {
    pub p: mpz_t,
    pub q: mpz_t,
    pub g: mpz_t,
    pub y: mpz_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_private_key {
    pub x: mpz_t,
}
pub unsafe extern "C" fn nettle_dsa_public_key_init(mut key: *mut dsa_public_key) {
    nettle_dsa_params_init(key as *mut dsa_params);
    __gmpz_init(((*key).y).as_mut_ptr());
}
pub unsafe extern "C" fn nettle_dsa_public_key_clear(mut key: *mut dsa_public_key) {
    nettle_dsa_params_clear(key as *mut dsa_params);
    __gmpz_clear(((*key).y).as_mut_ptr());
}
pub unsafe extern "C" fn nettle_dsa_private_key_init(mut key: *mut dsa_private_key) {
    __gmpz_init(((*key).x).as_mut_ptr());
}
pub unsafe extern "C" fn nettle_dsa_private_key_clear(mut key: *mut dsa_private_key) {
    __gmpz_clear(((*key).x).as_mut_ptr());
}
