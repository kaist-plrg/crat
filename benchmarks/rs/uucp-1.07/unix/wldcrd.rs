use ::libc;
extern "C" {
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn glob(
        __pattern: *const libc::c_char,
        __flags: libc::c_int,
        __errfunc: Option::<
            unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
        >,
        __pglob: *mut glob_t,
    ) -> libc::c_int;
    fn globfree(__pglob: *mut glob_t);
}
pub type boolean = libc::c_int;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub type __size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut libc::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: libc::c_int,
    pub gl_closedir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub gl_readdir: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
    >,
    pub gl_opendir: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
    >,
    pub gl_lstat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> libc::c_int,
    >,
    pub gl_stat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> libc::c_int,
    >,
}
static mut sSglob: glob_t = glob_t {
    gl_pathc: 0,
    gl_pathv: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    gl_offs: 0,
    gl_flags: 0,
    gl_closedir: None,
    gl_readdir: None,
    gl_opendir: None,
    gl_lstat: None,
    gl_stat: None,
};
static mut iSglob: libc::c_uint = 0;
pub unsafe extern "C" fn fsysdep_wildcard_start(
    mut zfile: *const libc::c_char,
) -> boolean {
    if *zfile as libc::c_int != '/' as i32 {
        ulog(
            LOG_FATAL,
            b"fsysdep_wildcard: %s: Can't happen\0" as *const u8 as *const libc::c_char,
            zfile,
        );
    }
    if glob(
        zfile,
        0 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            Option::<
                unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
            >,
        >(
            ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn() -> libc::c_int>,
            >(0 as *mut libc::c_void),
        ),
        &mut sSglob,
    ) != 0 as libc::c_int
    {
        sSglob.gl_pathc = 0 as libc::c_int as __size_t;
    }
    iSglob = 0 as libc::c_int as libc::c_uint;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn zsysdep_wildcard(
    mut zfile: *const libc::c_char,
) -> *mut libc::c_char {
    let mut zret: *mut libc::c_char = 0 as *mut libc::c_char;
    if iSglob as libc::c_ulong >= sSglob.gl_pathc {
        return 0 as *mut libc::c_char;
    }
    zret = zbufcpy(*(sSglob.gl_pathv).offset(iSglob as isize));
    iSglob = iSglob.wrapping_add(1);
    iSglob;
    return zret;
}
pub unsafe extern "C" fn fsysdep_wildcard_end() -> boolean {
    globfree(&mut sSglob);
    return 1 as libc::c_int;
}
