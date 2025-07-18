use ::libc;
extern "C" {
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_init(_: mpz_ptr);
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
pub struct dsa_signature {
    pub r: mpz_t,
    pub s: mpz_t,
}
pub unsafe extern "C" fn nettle_dsa_params_init(mut params: *mut dsa_params) {
    __gmpz_init(((*params).p).as_mut_ptr());
    __gmpz_init(((*params).q).as_mut_ptr());
    __gmpz_init(((*params).g).as_mut_ptr());
}
pub unsafe extern "C" fn nettle_dsa_params_clear(mut params: *mut dsa_params) {
    __gmpz_clear(((*params).p).as_mut_ptr());
    __gmpz_clear(((*params).q).as_mut_ptr());
    __gmpz_clear(((*params).g).as_mut_ptr());
}
pub unsafe extern "C" fn nettle_dsa_signature_init(mut signature: *mut dsa_signature) {
    __gmpz_init(((*signature).r).as_mut_ptr());
    __gmpz_init(((*signature).s).as_mut_ptr());
}
pub unsafe extern "C" fn nettle_dsa_signature_clear(mut signature: *mut dsa_signature) {
    __gmpz_clear(((*signature).r).as_mut_ptr());
    __gmpz_clear(((*signature).s).as_mut_ptr());
}
