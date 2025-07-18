use ::libc;
extern "C" {
    fn usysdep_pause();
}
pub unsafe extern "C" fn usysdep_sleep(mut c: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 2 as libc::c_int * c;
    while i > 0 as libc::c_int {
        usysdep_pause();
        i -= 1;
        i;
    }
}
