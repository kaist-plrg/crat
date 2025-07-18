use ::libc;
pub type mp_limb_t = libc::c_ulong;
pub type mp_size_t = libc::c_long;
pub unsafe extern "C" fn _nettle_cnd_copy(
    mut cnd: libc::c_int,
    mut rp: *mut mp_limb_t,
    mut ap: *const mp_limb_t,
    mut n: mp_size_t,
) {
    let mut mask: mp_limb_t = 0;
    let mut keep: mp_limb_t = 0;
    let mut i: mp_size_t = 0;
    mask = ((cnd != 0 as libc::c_int) as libc::c_int as mp_limb_t).wrapping_neg();
    keep = !mask;
    i = 0 as libc::c_int as mp_size_t;
    while i < n {
        *rp
            .offset(
                i as isize,
            ) = (*rp.offset(i as isize) & keep)
            .wrapping_add(*ap.offset(i as isize) & mask);
        i += 1;
        i;
    }
}
