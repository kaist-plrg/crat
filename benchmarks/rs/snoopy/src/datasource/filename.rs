use ::libc;
extern "C" {
    fn snoopy_inputdatastorage_get() -> *mut snoopy_inputdatastorage_t;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snoopy_inputdatastorage_t {
    pub initialized: libc::c_int,
    pub filename: *const libc::c_char,
    pub argv: *const *mut libc::c_char,
    pub envp: *const *mut libc::c_char,
}
pub unsafe extern "C" fn snoopy_datasource_filename(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut snoopy_inputdatastorage: *const snoopy_inputdatastorage_t = 0
        as *const snoopy_inputdatastorage_t;
    snoopy_inputdatastorage = snoopy_inputdatastorage_get();
    return snprintf(
        result,
        2048 as libc::c_int as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        (*snoopy_inputdatastorage).filename,
    );
}
