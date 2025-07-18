use ::libc;
pub type mp_limb_t = libc::c_ulong;
pub type mp_size_t = libc::c_long;
pub unsafe extern "C" fn _nettle_sec_sub_1(
    mut rp: *mut mp_limb_t,
    mut ap: *mut mp_limb_t,
    mut n: mp_size_t,
    mut b: mp_limb_t,
) -> mp_limb_t {
    let mut i: mp_size_t = 0;
    i = 0 as libc::c_int as mp_size_t;
    while i < n {
        let mut a: mp_limb_t = 0;
        a = *ap.offset(i as isize);
        *rp.offset(i as isize) = a.wrapping_sub(b);
        b = (a < b) as libc::c_int as mp_limb_t;
        i += 1;
        i;
    }
    return b;
}
