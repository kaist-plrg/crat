use ::libc;
extern "C" {
    fn ixsysdep_process_time(pimicros: *mut libc::c_long) -> libc::c_long;
}
pub unsafe extern "C" fn ixsysdep_time(mut pimicros: *mut libc::c_long) -> libc::c_long {
    return ixsysdep_process_time(pimicros);
}
