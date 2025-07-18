use ::libc;
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn priv_set_remove_linkdir() -> libc::c_int {
    return -(1 as libc::c_int);
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn priv_set_restore_linkdir() -> libc::c_int {
    return -(1 as libc::c_int);
}
