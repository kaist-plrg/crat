use ::libc;
extern "C" {
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
}
pub type __useconds_t = libc::c_uint;
pub unsafe extern "C" fn usysdep_pause() {
    usleep(
        (500 as libc::c_int as libc::c_long * 1000 as libc::c_int as libc::c_long)
            as __useconds_t,
    );
}
