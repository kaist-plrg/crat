use ::libc;
extern "C" {
    fn __gmpz_add(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_add_ui(_: mpz_ptr, _: mpz_srcptr, _: libc::c_ulong);
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_fdiv_r(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_init_set(_: mpz_ptr, _: mpz_srcptr);
    fn __gmpz_invert(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr) -> libc::c_int;
    fn __gmpz_mul(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_powm_sec(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_sizeinbase(_: mpz_srcptr, _: libc::c_int) -> size_t;
    fn __gmpz_sub_ui(_: mpz_ptr, _: mpz_srcptr, _: libc::c_ulong);
    fn nettle_mpz_random(
        x: *mut __mpz_struct,
        ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        n: *const __mpz_struct,
    );
    fn _nettle_dsa_hash(
        h: *mut __mpz_struct,
        bit_size: libc::c_uint,
        length: size_t,
        digest: *const uint8_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_signature {
    pub r: mpz_t,
    pub s: mpz_t,
}
pub unsafe extern "C" fn nettle_dsa_sign(
    mut params: *const dsa_params,
    mut x: *const __mpz_struct,
    mut random_ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
    mut digest_size: size_t,
    mut digest: *const uint8_t,
    mut signature: *mut dsa_signature,
) -> libc::c_int {
    let mut k: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut h: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut tmp: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut res: libc::c_int = 0;
    if ((*((*params).p).as_ptr())._mp_size != 0 as libc::c_int) as libc::c_int
        & *((*((*params).p).as_ptr())._mp_d).offset(0 as libc::c_int as isize)
            as libc::c_int == 0
    {
        return 0 as libc::c_int;
    }
    __gmpz_init_set(tmp.as_mut_ptr(), ((*params).q).as_ptr());
    __gmpz_sub_ui(
        tmp.as_mut_ptr(),
        tmp.as_mut_ptr() as mpz_srcptr,
        1 as libc::c_int as libc::c_ulong,
    );
    __gmpz_init(k.as_mut_ptr());
    nettle_mpz_random(
        k.as_mut_ptr(),
        random_ctx,
        random,
        tmp.as_mut_ptr() as *const __mpz_struct,
    );
    __gmpz_add_ui(
        k.as_mut_ptr(),
        k.as_mut_ptr() as mpz_srcptr,
        1 as libc::c_int as libc::c_ulong,
    );
    __gmpz_powm_sec(
        tmp.as_mut_ptr(),
        ((*params).g).as_ptr(),
        k.as_mut_ptr() as mpz_srcptr,
        ((*params).p).as_ptr(),
    );
    __gmpz_fdiv_r(
        ((*signature).r).as_mut_ptr(),
        tmp.as_mut_ptr() as mpz_srcptr,
        ((*params).q).as_ptr(),
    );
    __gmpz_init(h.as_mut_ptr());
    _nettle_dsa_hash(
        h.as_mut_ptr(),
        __gmpz_sizeinbase(((*params).q).as_ptr(), 2 as libc::c_int) as libc::c_uint,
        digest_size,
        digest,
    );
    if __gmpz_invert(
        k.as_mut_ptr(),
        k.as_mut_ptr() as mpz_srcptr,
        ((*params).q).as_ptr(),
    ) != 0
    {
        __gmpz_mul(tmp.as_mut_ptr(), ((*signature).r).as_mut_ptr() as mpz_srcptr, x);
        __gmpz_fdiv_r(
            tmp.as_mut_ptr(),
            tmp.as_mut_ptr() as mpz_srcptr,
            ((*params).q).as_ptr(),
        );
        __gmpz_add(
            tmp.as_mut_ptr(),
            tmp.as_mut_ptr() as mpz_srcptr,
            h.as_mut_ptr() as mpz_srcptr,
        );
        __gmpz_mul(
            tmp.as_mut_ptr(),
            tmp.as_mut_ptr() as mpz_srcptr,
            k.as_mut_ptr() as mpz_srcptr,
        );
        __gmpz_fdiv_r(
            ((*signature).s).as_mut_ptr(),
            tmp.as_mut_ptr() as mpz_srcptr,
            ((*params).q).as_ptr(),
        );
        res = 1 as libc::c_int;
    } else {
        res = 0 as libc::c_int;
    }
    __gmpz_clear(k.as_mut_ptr());
    __gmpz_clear(h.as_mut_ptr());
    __gmpz_clear(tmp.as_mut_ptr());
    return res;
}
