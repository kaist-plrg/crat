use ::libc;
extern "C" {
    fn snoopy_tsrm_get_inputdatastorage() -> *mut snoopy_inputdatastorage_t;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snoopy_inputdatastorage_t {
    pub initialized: libc::c_int,
    pub filename: *const libc::c_char,
    pub argv: *const *mut libc::c_char,
    pub envp: *const *mut libc::c_char,
}
pub unsafe extern "C" fn snoopy_inputdatastorage_ctor() {
    let mut IDS: *mut snoopy_inputdatastorage_t = snoopy_inputdatastorage_get();
    snoopy_inputdatastorage_setDefaults(IDS);
}
pub unsafe extern "C" fn snoopy_inputdatastorage_dtor() {
    let mut IDS: *mut snoopy_inputdatastorage_t = snoopy_inputdatastorage_get();
    snoopy_inputdatastorage_setDefaults(IDS);
}
pub unsafe extern "C" fn snoopy_inputdatastorage_setUninitialized(
    mut IDS: *mut snoopy_inputdatastorage_t,
) {
    (*IDS).initialized = 0 as libc::c_int;
}
pub unsafe extern "C" fn snoopy_inputdatastorage_setDefaults(
    mut IDS: *mut snoopy_inputdatastorage_t,
) {
    static mut empty_string: *const libc::c_char = b"\0" as *const u8
        as *const libc::c_char;
    static mut empty_string_array: [*mut libc::c_char; 1] = [
        0 as *const libc::c_char as *mut libc::c_char,
    ];
    (*IDS).initialized = 1 as libc::c_int;
    (*IDS).filename = empty_string;
    (*IDS).argv = empty_string_array.as_mut_ptr();
    (*IDS).envp = empty_string_array.as_mut_ptr();
}
pub unsafe extern "C" fn snoopy_inputdatastorage_store_filename(
    mut filename: *const libc::c_char,
) {
    let mut IDS: *mut snoopy_inputdatastorage_t = snoopy_inputdatastorage_get();
    (*IDS).filename = filename;
}
pub unsafe extern "C" fn snoopy_inputdatastorage_store_argv(
    mut argv: *const *mut libc::c_char,
) {
    let mut IDS: *mut snoopy_inputdatastorage_t = snoopy_inputdatastorage_get();
    (*IDS).argv = argv;
}
pub unsafe extern "C" fn snoopy_inputdatastorage_store_envp(
    mut envp: *const *mut libc::c_char,
) {
    let mut IDS: *mut snoopy_inputdatastorage_t = snoopy_inputdatastorage_get();
    (*IDS).envp = envp;
}
pub unsafe extern "C" fn snoopy_inputdatastorage_get() -> *mut snoopy_inputdatastorage_t {
    let mut IDS: *mut snoopy_inputdatastorage_t = 0 as *mut snoopy_inputdatastorage_t;
    IDS = snoopy_tsrm_get_inputdatastorage();
    if 1 as libc::c_int != (*IDS).initialized {
        snoopy_inputdatastorage_setDefaults(IDS);
    }
    return IDS;
}
