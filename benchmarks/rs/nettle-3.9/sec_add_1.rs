use ::libc;
pub type mp_limb_t = libc::c_ulong;
pub type mp_size_t = libc::c_long;
pub unsafe extern "C" fn _nettle_sec_add_1(
    mut rp: *mut mp_limb_t,
    mut ap: *mut mp_limb_t,
    mut n: mp_size_t,
    mut b: mp_limb_t,
) -> mp_limb_t {
    let mut i: mp_size_t = 0;
    i = 0 as libc::c_int as mp_size_t;
    while i < n {
        let mut r: mp_limb_t = (*ap.offset(i as isize)).wrapping_add(b);
        b = (r < b) as libc::c_int as mp_limb_t;
        *rp.offset(i as isize) = r;
        i += 1;
        i;
    }
    return b;
}
