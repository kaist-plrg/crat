use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __gmpz_clear(_: mpz_ptr);
    fn __gmpz_init(_: mpz_ptr);
    fn __gmpz_mul(_: mpz_ptr, _: mpz_srcptr, _: mpz_srcptr);
    fn __gmpz_limbs_write(_: mpz_ptr, _: mp_size_t) -> mp_ptr;
    fn __gmpz_limbs_finish(_: mpz_ptr, _: mp_size_t);
    fn _nettle_rsa_check_size(n: *mut __mpz_struct) -> size_t;
    fn _nettle_rsa_sec_compute_root_itch(key: *const rsa_private_key) -> mp_size_t;
    fn _nettle_rsa_sec_compute_root(
        key: *const rsa_private_key,
        rp: *mut mp_limb_t,
        mp: *const mp_limb_t,
        scratch: *mut mp_limb_t,
    );
    fn _nettle_mpz_limbs_copy(xp: *mut mp_limb_t, x: mpz_srcptr, n: mp_size_t);
    fn _nettle_gmp_alloc(n: size_t) -> *mut libc::c_void;
    fn _nettle_gmp_free(p: *mut libc::c_void, n: size_t);
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
pub type mp_ptr = *mut mp_limb_t;
pub type mp_size_t = libc::c_long;
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
#[inline]
unsafe extern "C" fn __gmpz_size(mut __gmp_z: mpz_srcptr) -> size_t {
    return (if (*__gmp_z)._mp_size >= 0 as libc::c_int {
        (*__gmp_z)._mp_size
    } else {
        -(*__gmp_z)._mp_size
    }) as size_t;
}
pub unsafe extern "C" fn nettle_rsa_private_key_init(mut key: *mut rsa_private_key) {
    __gmpz_init(((*key).d).as_mut_ptr());
    __gmpz_init(((*key).p).as_mut_ptr());
    __gmpz_init(((*key).q).as_mut_ptr());
    __gmpz_init(((*key).a).as_mut_ptr());
    __gmpz_init(((*key).b).as_mut_ptr());
    __gmpz_init(((*key).c).as_mut_ptr());
    (*key).size = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn nettle_rsa_private_key_clear(mut key: *mut rsa_private_key) {
    __gmpz_clear(((*key).d).as_mut_ptr());
    __gmpz_clear(((*key).p).as_mut_ptr());
    __gmpz_clear(((*key).q).as_mut_ptr());
    __gmpz_clear(((*key).a).as_mut_ptr());
    __gmpz_clear(((*key).b).as_mut_ptr());
    __gmpz_clear(((*key).c).as_mut_ptr());
}
pub unsafe extern "C" fn nettle_rsa_private_key_prepare(
    mut key: *mut rsa_private_key,
) -> libc::c_int {
    let mut n: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    if (__gmpz_size(((*key).q).as_mut_ptr() as mpz_srcptr))
        .wrapping_add(__gmpz_size(((*key).c).as_mut_ptr() as mpz_srcptr))
        < __gmpz_size(((*key).p).as_mut_ptr() as mpz_srcptr)
    {
        return 0 as libc::c_int;
    }
    __gmpz_init(n.as_mut_ptr());
    __gmpz_mul(
        n.as_mut_ptr(),
        ((*key).p).as_mut_ptr() as mpz_srcptr,
        ((*key).q).as_mut_ptr() as mpz_srcptr,
    );
    (*key).size = _nettle_rsa_check_size(n.as_mut_ptr());
    __gmpz_clear(n.as_mut_ptr());
    return ((*key).size > 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
pub unsafe extern "C" fn nettle_rsa_compute_root(
    mut key: *const rsa_private_key,
    mut x: *mut __mpz_struct,
    mut m: *const __mpz_struct,
) {
    let mut scratch: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut tmp_scratch_size: size_t = 0;
    let mut ml: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut tmp_ml_size: size_t = 0;
    let mut xl: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut key_size: size_t = 0;
    key_size = ((*key).size)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_add((64 as libc::c_int - 0 as libc::c_int) as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div((64 as libc::c_int - 0 as libc::c_int) as libc::c_ulong);
    if __gmpz_size(m) <= key_size {} else {
        __assert_fail(
            b"mpz_size (m) <= key_size\0" as *const u8 as *const libc::c_char,
            b"rsa-sign.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 99],
                &[libc::c_char; 99],
            >(
                b"void nettle_rsa_compute_root(const struct rsa_private_key *, __mpz_struct *, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4106: {
        if __gmpz_size(m) <= key_size {} else {
            __assert_fail(
                b"mpz_size (m) <= key_size\0" as *const u8 as *const libc::c_char,
                b"rsa-sign.c\0" as *const u8 as *const libc::c_char,
                169 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 99],
                    &[libc::c_char; 99],
                >(
                    b"void nettle_rsa_compute_root(const struct rsa_private_key *, __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    tmp_ml_size = key_size;
    ml = _nettle_gmp_alloc(
        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong).wrapping_mul(key_size),
    ) as *mut mp_limb_t;
    _nettle_mpz_limbs_copy(ml, m, key_size as mp_size_t);
    tmp_scratch_size = _nettle_rsa_sec_compute_root_itch(key) as size_t;
    scratch = _nettle_gmp_alloc(
        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(_nettle_rsa_sec_compute_root_itch(key) as libc::c_ulong),
    ) as *mut mp_limb_t;
    xl = __gmpz_limbs_write(x, key_size as mp_size_t);
    _nettle_rsa_sec_compute_root(key, xl, ml, scratch);
    __gmpz_limbs_finish(x, key_size as mp_size_t);
    _nettle_gmp_free(ml as *mut libc::c_void, tmp_ml_size);
    _nettle_gmp_free(scratch as *mut libc::c_void, tmp_scratch_size);
}
