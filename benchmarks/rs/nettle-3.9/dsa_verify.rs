use ::libc;
extern "C" {
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_cmp(_: mpz_srcptr, _: mpz_srcptr) -> libc::c_int;
    fn __gmpz_fdiv_r(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_invert(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr) -> libc::c_int;
    fn __gmpz_mul(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_powm(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_sizeinbase(_: mpz_srcptr, _: libc::c_int) -> size_t;
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
pub unsafe extern "C" fn nettle_dsa_verify(
    mut params: *const dsa_params,
    mut y: *const __mpz_struct,
    mut digest_size: size_t,
    mut digest: *const uint8_t,
    mut signature: *const dsa_signature,
) -> libc::c_int {
    let mut w: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut tmp: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut v: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut res: libc::c_int = 0;
    if (if (*((*signature).r).as_ptr())._mp_size < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        ((*((*signature).r).as_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
    }) <= 0 as libc::c_int
        || __gmpz_cmp(((*signature).r).as_ptr(), ((*params).q).as_ptr())
            >= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (if (*((*signature).s).as_ptr())._mp_size < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        ((*((*signature).s).as_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
    }) <= 0 as libc::c_int
        || __gmpz_cmp(((*signature).s).as_ptr(), ((*params).q).as_ptr())
            >= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    __gmpz_init(w.as_mut_ptr());
    if __gmpz_invert(w.as_mut_ptr(), ((*signature).s).as_ptr(), ((*params).q).as_ptr())
        == 0
    {
        __gmpz_clear(w.as_mut_ptr());
        return 0 as libc::c_int;
    }
    __gmpz_init(tmp.as_mut_ptr());
    __gmpz_init(v.as_mut_ptr());
    _nettle_dsa_hash(
        tmp.as_mut_ptr(),
        __gmpz_sizeinbase(((*params).q).as_ptr(), 2 as libc::c_int) as libc::c_uint,
        digest_size,
        digest,
    );
    __gmpz_mul(
        tmp.as_mut_ptr(),
        tmp.as_mut_ptr() as mpz_srcptr,
        w.as_mut_ptr() as mpz_srcptr,
    );
    __gmpz_fdiv_r(
        tmp.as_mut_ptr(),
        tmp.as_mut_ptr() as mpz_srcptr,
        ((*params).q).as_ptr(),
    );
    __gmpz_powm(
        v.as_mut_ptr(),
        ((*params).g).as_ptr(),
        tmp.as_mut_ptr() as mpz_srcptr,
        ((*params).p).as_ptr(),
    );
    __gmpz_mul(
        tmp.as_mut_ptr(),
        ((*signature).r).as_ptr(),
        w.as_mut_ptr() as mpz_srcptr,
    );
    __gmpz_fdiv_r(
        tmp.as_mut_ptr(),
        tmp.as_mut_ptr() as mpz_srcptr,
        ((*params).q).as_ptr(),
    );
    __gmpz_powm(
        tmp.as_mut_ptr(),
        y,
        tmp.as_mut_ptr() as mpz_srcptr,
        ((*params).p).as_ptr(),
    );
    __gmpz_mul(
        v.as_mut_ptr(),
        v.as_mut_ptr() as mpz_srcptr,
        tmp.as_mut_ptr() as mpz_srcptr,
    );
    __gmpz_fdiv_r(v.as_mut_ptr(), v.as_mut_ptr() as mpz_srcptr, ((*params).p).as_ptr());
    __gmpz_fdiv_r(v.as_mut_ptr(), v.as_mut_ptr() as mpz_srcptr, ((*params).q).as_ptr());
    res = (__gmpz_cmp(v.as_mut_ptr() as mpz_srcptr, ((*signature).r).as_ptr()) == 0)
        as libc::c_int;
    __gmpz_clear(w.as_mut_ptr());
    __gmpz_clear(tmp.as_mut_ptr());
    __gmpz_clear(v.as_mut_ptr());
    return res;
}
