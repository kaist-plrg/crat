use ::libc;
extern "C" {
    fn qcopy_acl(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: mode_t,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type __mode_t = libc::c_uint;
pub type mode_t = __mode_t;
pub unsafe extern "C" fn copy_acl(
    mut src_name: *const libc::c_char,
    mut source_desc: libc::c_int,
    mut dst_name: *const libc::c_char,
    mut dest_desc: libc::c_int,
    mut mode: mode_t,
) -> libc::c_int {
    let mut ret: libc::c_int = qcopy_acl(
        src_name,
        source_desc,
        dst_name,
        dest_desc,
        mode,
    );
    match ret {
        -2 => {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quote(src_name),
            );
        }
        -1 => {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"preserving permissions for %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(dst_name),
            );
        }
        _ => {}
    }
    return ret;
}
