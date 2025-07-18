use ::libc;
extern "C" {
    pub type ecc_curve;
    fn nettle_ecc_ecdsa_verify(
        ecc: *const ecc_curve,
        pp: *const mp_limb_t,
        length: size_t,
        digest: *const uint8_t,
        rp_0: *const mp_limb_t,
        sp_0: *const mp_limb_t,
        scratch: *mut mp_limb_t,
    ) -> libc::c_int;
    fn nettle_ecc_ecdsa_verify_itch(ecc: *const ecc_curve) -> mp_size_t;
    fn nettle_ecc_size(ecc: *const ecc_curve) -> mp_size_t;
    fn _nettle_mpz_limbs_copy(xp: *mut mp_limb_t, x: mpz_srcptr, n: mp_size_t);
    fn _nettle_gmp_alloc_limbs(n: mp_size_t) -> *mut mp_limb_t;
    fn _nettle_gmp_free_limbs(p: *mut mp_limb_t, n: mp_size_t);
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
pub type mp_size_t = libc::c_long;
pub type mpz_srcptr = *const __mpz_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ecc_point {
    pub ecc: *const ecc_curve,
    pub p: *mut mp_limb_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_signature {
    pub r: mpz_t,
    pub s: mpz_t,
}
#[inline]
unsafe extern "C" fn __gmpz_size(mut __gmp_z: mpz_srcptr) -> size_t {
    return (if (*__gmp_z)._mp_size >= 0 as libc::c_int {
        (*__gmp_z)._mp_size
    } else {
        -(*__gmp_z)._mp_size
    }) as size_t;
}
pub unsafe extern "C" fn nettle_ecdsa_verify(
    mut pub_0: *const ecc_point,
    mut length: size_t,
    mut digest: *const uint8_t,
    mut signature: *const dsa_signature,
) -> libc::c_int {
    let mut size: mp_limb_t = nettle_ecc_size((*pub_0).ecc) as mp_limb_t;
    let mut itch: mp_size_t = (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(size)
        .wrapping_add(nettle_ecc_ecdsa_verify_itch((*pub_0).ecc) as libc::c_ulong)
        as mp_size_t;
    let mut scratch: *mut mp_limb_t = 0 as *mut mp_limb_t;
    let mut res: libc::c_int = 0;
    if (if (*((*signature).r).as_ptr())._mp_size < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        ((*((*signature).r).as_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
    }) <= 0 as libc::c_int || __gmpz_size(((*signature).r).as_ptr()) > size
        || (if (*((*signature).s).as_ptr())._mp_size < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            ((*((*signature).s).as_ptr())._mp_size > 0 as libc::c_int) as libc::c_int
        }) <= 0 as libc::c_int || __gmpz_size(((*signature).s).as_ptr()) > size
    {
        return 0 as libc::c_int;
    }
    scratch = _nettle_gmp_alloc_limbs(itch);
    _nettle_mpz_limbs_copy(scratch, ((*signature).r).as_ptr(), size as mp_size_t);
    _nettle_mpz_limbs_copy(
        scratch.offset(size as isize),
        ((*signature).s).as_ptr(),
        size as mp_size_t,
    );
    res = nettle_ecc_ecdsa_verify(
        (*pub_0).ecc,
        (*pub_0).p,
        length,
        digest,
        scratch,
        scratch.offset(size as isize),
        scratch.offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(size) as isize),
    );
    _nettle_gmp_free_limbs(scratch, itch);
    return res;
}
