use ::libc;
extern "C" {
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_fdiv_r(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_invert(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr) -> libc::c_int;
    fn __gmpz_mul(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_powm_sec(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr, _: mpz_srcptr);
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
pub struct rsa_public_key {
    pub size: size_t,
    pub n: mpz_t,
    pub e: mpz_t,
}
pub unsafe extern "C" fn _nettle_rsa_blind(
    mut pub_0: *const rsa_public_key,
    mut random_ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
    mut c: *mut __mpz_struct,
    mut ri: *mut __mpz_struct,
) {
    let mut r: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    __gmpz_init(r.as_mut_ptr());
    loop {
        nettle_mpz_random(r.as_mut_ptr(), random_ctx, random, ((*pub_0).n).as_ptr());
        if !(__gmpz_invert(ri, r.as_mut_ptr() as mpz_srcptr, ((*pub_0).n).as_ptr()) == 0)
        {
            break;
        }
    }
    __gmpz_powm_sec(
        r.as_mut_ptr(),
        r.as_mut_ptr() as mpz_srcptr,
        ((*pub_0).e).as_ptr(),
        ((*pub_0).n).as_ptr(),
    );
    __gmpz_mul(c, c as mpz_srcptr, r.as_mut_ptr() as mpz_srcptr);
    __gmpz_fdiv_r(c, c as mpz_srcptr, ((*pub_0).n).as_ptr());
    __gmpz_clear(r.as_mut_ptr());
}
pub unsafe extern "C" fn _nettle_rsa_unblind(
    mut pub_0: *const rsa_public_key,
    mut c: *mut __mpz_struct,
    mut ri: *const __mpz_struct,
) {
    __gmpz_mul(c, c as mpz_srcptr, ri);
    __gmpz_fdiv_r(c, c as mpz_srcptr, ((*pub_0).n).as_ptr());
}
