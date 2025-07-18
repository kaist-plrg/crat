use ::libc;
extern "C" {
    fn __gmpz_cmp(_: mpz_srcptr, _: mpz_srcptr) -> libc::c_int;
    fn _nettle_pkcs1_sec_decrypt(
        length: size_t,
        message: *mut uint8_t,
        padded_message_length: size_t,
        padded_message: *const uint8_t,
    ) -> libc::c_int;
    fn _nettle_rsa_sec_compute_root_tr(
        pub_0: *const rsa_public_key,
        key: *const rsa_private_key,
        random_ctx: *mut libc::c_void,
        random: Option::<nettle_random_func>,
        x: *mut mp_limb_t,
        m: *const mp_limb_t,
    ) -> libc::c_int;
    fn _nettle_mpz_limbs_copy(xp: *mut mp_limb_t, x: mpz_srcptr, n: mp_size_t);
    fn _nettle_mpn_get_base256(
        rp: *mut uint8_t,
        rn: size_t,
        xp: *const mp_limb_t,
        xn: mp_size_t,
    );
    fn _nettle_gmp_alloc(n: size_t) -> *mut libc::c_void;
    fn _nettle_gmp_free(p: *mut libc::c_void, n: size_t);
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
pub type mp_size_t = libc::c_long;
pub type mpz_srcptr = *const __mpz_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_public_key {
    pub size: size_t,
    pub n: mpz_t,
    pub e: mpz_t,
}
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
pub unsafe extern "C" fn nettle_rsa_sec_decrypt(
    mut pub_0: *const rsa_public_key,
    mut key: *const rsa_private_key,
    mut random_ctx: *mut libc::c_void,
    mut random: Option::<nettle_random_func>,
    mut length: size_t,
    mut message: *mut uint8_t,
    mut gibberish: *const __mpz_struct,
) -> libc::c_int {
    let mut m: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut tmp_m_size: size_t = 0;
    let mut em: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp_em_size: size_t = 0;
    let mut res: libc::c_int = 0;
    if (if (*gibberish)._mp_size < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        ((*gibberish)._mp_size > 0 as libc::c_int) as libc::c_int
    }) < 0 as libc::c_int
        || __gmpz_cmp(gibberish, ((*pub_0).n).as_ptr()) >= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    tmp_m_size = __gmpz_size(((*pub_0).n).as_ptr());
    m = _nettle_gmp_alloc(
        (::std::mem::size_of::<mp_limb_t>() as libc::c_ulong)
            .wrapping_mul(__gmpz_size(((*pub_0).n).as_ptr())),
    ) as *mut mp_limb_t;
    tmp_em_size = (*key).size;
    em = _nettle_gmp_alloc(
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong).wrapping_mul((*key).size),
    ) as *mut uint8_t;
    _nettle_mpz_limbs_copy(
        m,
        gibberish,
        __gmpz_size(((*pub_0).n).as_ptr()) as mp_size_t,
    );
    res = _nettle_rsa_sec_compute_root_tr(pub_0, key, random_ctx, random, m, m);
    _nettle_mpn_get_base256(
        em,
        (*key).size,
        m,
        __gmpz_size(((*pub_0).n).as_ptr()) as mp_size_t,
    );
    res &= _nettle_pkcs1_sec_decrypt(length, message, (*key).size, em);
    _nettle_gmp_free(em as *mut libc::c_void, tmp_em_size);
    _nettle_gmp_free(m as *mut libc::c_void, tmp_m_size);
    return res;
}
